use crate::config::Config;
use crate::content::{load_astrology, load_pages, load_posts, load_tools, AstrologyData, Page, Post, Tool};
use crate::render::template::{
    AstrologyCategoryContext, AstrologyIndexContext, AstrologyItemContext, CategoryContext, ConfigContext, ListContext,
    PageContext, Pagination, PostContext, TagContext, TemplateEngine, ToolPageContext, ToolsListContext,
};
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;

/// Search index item for client-side search
#[derive(Debug, Serialize)]
pub struct SearchIndexItem {
    pub slug: String,
    pub url: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

pub struct Builder {
    config: Config,
    template_engine: TemplateEngine,
    posts: Vec<Post>,
    pages: Vec<Page>,
    tools: Vec<Tool>,
    astrology: AstrologyData,
}

impl Builder {
    /// Create a new builder with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let theme_dir = Path::new("themes").join(&config.build.theme);
        let template_engine = TemplateEngine::new(&theme_dir)?;
        let posts = load_posts(&config.build.content_dir)?;
        let pages = load_pages(&config.build.content_dir)?;
        let tools = load_tools(&config.build.content_dir)?;
        let astrology = load_astrology(&config.build.content_dir)?;

        let astrology_count: usize = astrology.categories.iter().map(|c| c.items.len()).sum();
        info!(
            "Loaded {} posts, {} pages, {} tools, and {} astrology items",
            posts.len(),
            pages.len(),
            tools.len(),
            astrology_count
        );

        Ok(Self {
            config,
            template_engine,
            posts,
            pages,
            tools,
            astrology,
        })
    }

    /// Build the entire site
    pub fn build(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);

        // Clean output directory
        if output_dir.exists() {
            fs::remove_dir_all(output_dir).context("Failed to clean output directory")?;
        }
        fs::create_dir_all(output_dir).context("Failed to create output directory")?;

        // Generate all pages
        self.generate_posts(output_dir)?;
        self.generate_pages(output_dir)?;
        self.generate_tools(output_dir)?;
        self.generate_astrology(output_dir)?;
        self.generate_index(output_dir)?;
        self.generate_archive(output_dir)?;
        self.generate_tag_pages(output_dir)?;
        self.generate_category_pages(output_dir)?;
        self.generate_search_index(output_dir)?;

        // Copy static assets
        self.copy_static_assets(output_dir)?;

        info!("Build complete! Output: {}", output_dir.display());
        Ok(())
    }

    /// Generate individual post pages
    fn generate_posts(&self, output_dir: &Path) -> Result<()> {
        for post in &self.posts {
            let post_dir = output_dir.join("posts").join(&post.slug);
            fs::create_dir_all(&post_dir)?;

            let context = PostContext {
                site: &self.config.site,
                config: ConfigContext {
                    markdown: &self.config.markdown,
                },
                post,
                nav: &self.config.nav,
            };

            let html = self.template_engine.render("post.html", &context)?;
            fs::write(post_dir.join("index.html"), html)?;

            info!("Generated post: /posts/{}/", post.slug);
        }
        Ok(())
    }

    /// Generate standalone pages
    fn generate_pages(&self, output_dir: &Path) -> Result<()> {
        for page in &self.pages {
            let page_dir = output_dir.join(&page.slug);
            fs::create_dir_all(&page_dir)?;

            let context = PageContext {
                site: &self.config.site,
                config: ConfigContext {
                    markdown: &self.config.markdown,
                },
                page,
                nav: &self.config.nav,
            };

            let html = self.template_engine.render("page.html", &context)?;
            fs::write(page_dir.join("index.html"), html)?;

            info!("Generated page: /{}/", page.slug);
        }
        Ok(())
    }

    /// Generate tool pages (render templates or copy directories) and tools list
    fn generate_tools(&self, output_dir: &Path) -> Result<()> {
        if self.tools.is_empty() {
            return Ok(());
        }

        let tools_dir = output_dir.join("tools");
        fs::create_dir_all(&tools_dir)?;

        // Process each tool
        let content_tools_dir = Path::new(&self.config.build.content_dir).join("tools");
        for tool in &self.tools {
            let src_dir = content_tools_dir.join(&tool.slug);
            let dst_dir = tools_dir.join(&tool.slug);

            if src_dir.exists() {
                fs::create_dir_all(&dst_dir)?;

                let template_path = src_dir.join("template.html");
                if template_path.exists() {
                    // Render template.html with Tera
                    let context = ToolPageContext {
                        site: &self.config.site,
                        config: ConfigContext {
                            markdown: &self.config.markdown,
                        },
                        tool,
                        nav: &self.config.nav,
                    };

                    let template_name = format!("tools/{}/template.html", tool.slug);
                    let html = self.template_engine.render_tool_template(&template_path, &template_name, &context)?;
                    fs::write(dst_dir.join("index.html"), html)?;

                    // Copy non-template files (assets, etc.)
                    copy_tool_assets(&src_dir, &dst_dir)?;

                    info!("Generated tool (template): /tools/{}/", tool.slug);
                } else {
                    // Fallback: copy the entire directory as before
                    copy_dir_recursive(&src_dir, &dst_dir)?;
                    info!("Generated tool (copy): /tools/{}/", tool.slug);
                }
            } else {
                tracing::warn!("Tool directory not found: {}", src_dir.display());
            }
        }

        // Generate tools list page
        let context = ToolsListContext {
            site: &self.config.site,
            config: ConfigContext {
                markdown: &self.config.markdown,
            },
            tools: &self.tools,
            nav: &self.config.nav,
        };

        let html = self.template_engine.render("tools.html", &context)?;
        fs::write(tools_dir.join("index.html"), html)?;

        info!("Generated tools list page");

        Ok(())
    }

    /// Generate astrology pages (index, category pages, and individual item pages)
    fn generate_astrology(&self, output_dir: &Path) -> Result<()> {
        let astrology_dir = output_dir.join("astrology");
        fs::create_dir_all(&astrology_dir)?;

        // Generate astrology index page
        let context = AstrologyIndexContext {
            site: &self.config.site,
            config: ConfigContext {
                markdown: &self.config.markdown,
            },
            categories: &self.astrology.categories,
            nav: &self.config.nav,
        };

        let html = self.template_engine.render("astrology.html", &context)?;
        fs::write(astrology_dir.join("index.html"), html)?;
        info!("Generated astrology index page");

        // Generate category pages and individual item pages
        for category in &self.astrology.categories {
            let category_dir = astrology_dir.join(&category.id);
            fs::create_dir_all(&category_dir)?;

            let context = AstrologyCategoryContext {
                site: &self.config.site,
                config: ConfigContext {
                    markdown: &self.config.markdown,
                },
                category,
                items: &category.items,
                nav: &self.config.nav,
            };

            let html = self.template_engine.render("astrology-category.html", &context)?;
            fs::write(category_dir.join("index.html"), html)?;
            info!("Generated astrology category page: /astrology/{}/", category.id);

            // Generate individual item pages
            for item in &category.items {
                let item_dir = category_dir.join(&item.slug);
                fs::create_dir_all(&item_dir)?;

                let item_context = AstrologyItemContext {
                    site: &self.config.site,
                    config: ConfigContext {
                        markdown: &self.config.markdown,
                    },
                    item,
                    category,
                    nav: &self.config.nav,
                };

                let item_html = self.template_engine.render("astrology-item.html", &item_context)?;
                fs::write(item_dir.join("index.html"), item_html)?;
            }
            info!("Generated {} individual item pages in /astrology/{}/", category.items.len(), category.id);
        }

        Ok(())
    }

    /// Generate the index page with pagination
    fn generate_index(&self, output_dir: &Path) -> Result<()> {
        let per_page = self.config.pagination.posts_per_page;
        let total_posts = self.posts.len();
        let total_pages = if total_posts == 0 {
            1
        } else {
            (total_posts + per_page - 1) / per_page
        };

        for page_num in 1..=total_pages {
            let pagination = Pagination::new(total_posts, per_page, page_num, "");
            let page_posts: Vec<Post> = pagination.paginate(&self.posts).to_vec();

            let context = ListContext {
                site: &self.config.site,
                config: ConfigContext {
                    markdown: &self.config.markdown,
                },
                posts: &page_posts,
                nav: &self.config.nav,
                pagination: Some(pagination),
            };

            let html = self.template_engine.render("index.html", &context)?;

            if page_num == 1 {
                fs::write(output_dir.join("index.html"), &html)?;
                info!("Generated index page");
            } else {
                let page_dir = output_dir.join("page").join(page_num.to_string());
                fs::create_dir_all(&page_dir)?;
                fs::write(page_dir.join("index.html"), &html)?;
                info!("Generated index page {}", page_num);
            }
        }

        Ok(())
    }

    /// Generate the archive page with pagination
    fn generate_archive(&self, output_dir: &Path) -> Result<()> {
        let archive_dir = output_dir.join("archive");
        fs::create_dir_all(&archive_dir)?;

        let per_page = self.config.pagination.posts_per_page;
        let total_posts = self.posts.len();
        let total_pages = if total_posts == 0 {
            1
        } else {
            (total_posts + per_page - 1) / per_page
        };

        for page_num in 1..=total_pages {
            let pagination = Pagination::new(total_posts, per_page, page_num, "/archive");
            let page_posts: Vec<Post> = pagination.paginate(&self.posts).to_vec();

            let context = ListContext {
                site: &self.config.site,
                config: ConfigContext {
                    markdown: &self.config.markdown,
                },
                posts: &page_posts,
                nav: &self.config.nav,
                pagination: Some(pagination),
            };

            let html = self.template_engine.render("archive.html", &context)?;

            if page_num == 1 {
                fs::write(archive_dir.join("index.html"), &html)?;
                info!("Generated archive page");
            } else {
                let page_dir = archive_dir.join("page").join(page_num.to_string());
                fs::create_dir_all(&page_dir)?;
                fs::write(page_dir.join("index.html"), &html)?;
                info!("Generated archive page {}", page_num);
            }
        }

        Ok(())
    }

    /// Generate tag pages with pagination
    fn generate_tag_pages(&self, output_dir: &Path) -> Result<()> {
        let mut tags_map: HashMap<String, Vec<&Post>> = HashMap::new();

        for post in &self.posts {
            for tag in &post.meta.tags {
                tags_map.entry(tag.clone()).or_default().push(post);
            }
        }

        let tags_dir = output_dir.join("tags");
        fs::create_dir_all(&tags_dir)?;

        let per_page = self.config.pagination.posts_per_page;

        for (tag, posts) in tags_map {
            let tag_dir = tags_dir.join(&tag);
            fs::create_dir_all(&tag_dir)?;

            let posts_vec: Vec<Post> = posts.into_iter().cloned().collect();
            let total_posts = posts_vec.len();
            let total_pages = if total_posts == 0 {
                1
            } else {
                (total_posts + per_page - 1) / per_page
            };

            let base_url = format!("/tags/{}", tag);

            for page_num in 1..=total_pages {
                let pagination = Pagination::new(total_posts, per_page, page_num, &base_url);
                let page_posts: Vec<Post> = pagination.paginate(&posts_vec).to_vec();

                let context = TagContext {
                    site: &self.config.site,
                    config: ConfigContext {
                        markdown: &self.config.markdown,
                    },
                    posts: &page_posts,
                    nav: &self.config.nav,
                    tag: &tag,
                    pagination: Some(pagination),
                };

                let html = self.template_engine.render("archive.html", &context)?;

                if page_num == 1 {
                    fs::write(tag_dir.join("index.html"), &html)?;
                    info!("Generated tag page: /tags/{}/", tag);
                } else {
                    let page_dir = tag_dir.join("page").join(page_num.to_string());
                    fs::create_dir_all(&page_dir)?;
                    fs::write(page_dir.join("index.html"), &html)?;
                    info!("Generated tag page: /tags/{}/page/{}/", tag, page_num);
                }
            }
        }

        Ok(())
    }

    /// Generate category pages with pagination
    fn generate_category_pages(&self, output_dir: &Path) -> Result<()> {
        let mut categories_map: HashMap<String, Vec<&Post>> = HashMap::new();

        for post in &self.posts {
            for category in &post.meta.categories {
                categories_map
                    .entry(category.clone())
                    .or_default()
                    .push(post);
            }
        }

        let categories_dir = output_dir.join("categories");
        fs::create_dir_all(&categories_dir)?;

        let per_page = self.config.pagination.posts_per_page;

        for (category, posts) in categories_map {
            let category_dir = categories_dir.join(&category);
            fs::create_dir_all(&category_dir)?;

            let posts_vec: Vec<Post> = posts.into_iter().cloned().collect();
            let total_posts = posts_vec.len();
            let total_pages = if total_posts == 0 {
                1
            } else {
                (total_posts + per_page - 1) / per_page
            };

            let base_url = format!("/categories/{}", category);

            for page_num in 1..=total_pages {
                let pagination = Pagination::new(total_posts, per_page, page_num, &base_url);
                let page_posts: Vec<Post> = pagination.paginate(&posts_vec).to_vec();

                let context = CategoryContext {
                    site: &self.config.site,
                    config: ConfigContext {
                        markdown: &self.config.markdown,
                    },
                    posts: &page_posts,
                    nav: &self.config.nav,
                    category: &category,
                    pagination: Some(pagination),
                };

                let html = self.template_engine.render("archive.html", &context)?;

                if page_num == 1 {
                    fs::write(category_dir.join("index.html"), &html)?;
                    info!("Generated category page: /categories/{}/", category);
                } else {
                    let page_dir = category_dir.join("page").join(page_num.to_string());
                    fs::create_dir_all(&page_dir)?;
                    fs::write(page_dir.join("index.html"), &html)?;
                    info!("Generated category page: /categories/{}/page/{}/", category, page_num);
                }
            }
        }

        Ok(())
    }

    /// Generate search index JSON file
    fn generate_search_index(&self, output_dir: &Path) -> Result<()> {
        // Helper to strip HTML tags
        let strip_html = |html: &str| -> String {
            let mut result = String::new();
            let mut in_tag = false;
            for c in html.chars() {
                if c == '<' {
                    in_tag = true;
                } else if c == '>' {
                    in_tag = false;
                } else if !in_tag {
                    result.push(c);
                }
            }
            result
        };

        let mut search_index: Vec<SearchIndexItem> = Vec::new();

        // Add posts to search index
        for post in &self.posts {
            let plain_content = strip_html(&post.html_content);
            let summary = if let Some(ref s) = post.meta.summary {
                s.clone()
            } else {
                plain_content.chars().take(200).collect::<String>() + "..."
            };
            let content = plain_content.chars().take(2000).collect();

            search_index.push(SearchIndexItem {
                slug: post.slug.clone(),
                url: post.url.clone(),
                title: post.meta.title.clone(),
                summary,
                content,
                tags: post.meta.tags.clone(),
                date: post
                    .meta
                    .date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
                item_type: Some("post".to_string()),
                category: None,
            });
        }

        // Add astrology items to search index
        for cat in &self.astrology.categories {
            for item in &cat.items {
                let plain_content = strip_html(&item.html_content);
                let summary = item.meta.summary.clone().unwrap_or_else(|| {
                    plain_content.chars().take(200).collect::<String>() + "..."
                });
                let content = plain_content.chars().take(2000).collect();

                search_index.push(SearchIndexItem {
                    slug: item.slug.clone(),
                    url: item.url.clone(),
                    title: item.meta.title.clone(),
                    summary,
                    content,
                    tags: item.meta.keywords.clone(),
                    date: String::new(),
                    item_type: Some("astrology".to_string()),
                    category: Some(cat.name.clone()),
                });
            }
        }

        let json = serde_json::to_string_pretty(&search_index)
            .context("Failed to serialize search index")?;

        fs::write(output_dir.join("search-index.json"), json)?;
        info!("Generated search index with {} items", search_index.len());

        Ok(())
    }

    /// Copy static assets to output directory
    fn copy_static_assets(&self, output_dir: &Path) -> Result<()> {
        let theme_static = Path::new("themes")
            .join(&self.config.build.theme)
            .join("static");
        let global_static = Path::new("static");
        let output_static = output_dir.join("static");

        fs::create_dir_all(&output_static)?;

        // Copy theme static assets
        if theme_static.exists() {
            copy_dir_recursive(&theme_static, &output_static)?;
            info!("Copied theme static assets");
        }

        // Copy global static assets (overwrite theme assets if same name)
        if global_static.exists() {
            copy_dir_recursive(&global_static, &output_static)?;
            info!("Copied global static assets");
        }

        Ok(())
    }
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(src)?;
        let target = dst.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target)?;
        }
    }
    Ok(())
}

/// Copy tool assets (non-template files) from source to destination
fn copy_tool_assets(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(src)?;

        // Skip template.html as it's rendered separately
        if relative == Path::new("template.html") {
            continue;
        }

        let target = dst.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target)?;
        }
    }
    Ok(())
}
