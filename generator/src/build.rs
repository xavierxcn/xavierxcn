use crate::config::Config;
use crate::content::{load_pages, load_posts, Page, Post};
use crate::render::template::{
    CategoryContext, ConfigContext, ListContext, PageContext, PostContext, TagContext,
    TemplateEngine,
};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;

pub struct Builder {
    config: Config,
    template_engine: TemplateEngine,
    posts: Vec<Post>,
    pages: Vec<Page>,
}

impl Builder {
    /// Create a new builder with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let theme_dir = Path::new("themes").join(&config.build.theme);
        let template_engine = TemplateEngine::new(&theme_dir)?;
        let posts = load_posts(&config.build.content_dir)?;
        let pages = load_pages(&config.build.content_dir)?;

        info!("Loaded {} posts and {} pages", posts.len(), pages.len());

        Ok(Self {
            config,
            template_engine,
            posts,
            pages,
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
        self.generate_index(output_dir)?;
        self.generate_archive(output_dir)?;
        self.generate_tag_pages(output_dir)?;
        self.generate_category_pages(output_dir)?;

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

    /// Generate the index page
    fn generate_index(&self, output_dir: &Path) -> Result<()> {
        let context = ListContext {
            site: &self.config.site,
            config: ConfigContext {
                markdown: &self.config.markdown,
            },
            posts: &self.posts,
            nav: &self.config.nav,
        };

        let html = self.template_engine.render("index.html", &context)?;
        fs::write(output_dir.join("index.html"), html)?;

        info!("Generated index page");
        Ok(())
    }

    /// Generate the archive page
    fn generate_archive(&self, output_dir: &Path) -> Result<()> {
        let archive_dir = output_dir.join("archive");
        fs::create_dir_all(&archive_dir)?;

        let context = ListContext {
            site: &self.config.site,
            config: ConfigContext {
                markdown: &self.config.markdown,
            },
            posts: &self.posts,
            nav: &self.config.nav,
        };

        let html = self.template_engine.render("archive.html", &context)?;
        fs::write(archive_dir.join("index.html"), html)?;

        info!("Generated archive page");
        Ok(())
    }

    /// Generate tag pages
    fn generate_tag_pages(&self, output_dir: &Path) -> Result<()> {
        let mut tags_map: HashMap<String, Vec<&Post>> = HashMap::new();

        for post in &self.posts {
            for tag in &post.meta.tags {
                tags_map.entry(tag.clone()).or_default().push(post);
            }
        }

        let tags_dir = output_dir.join("tags");
        fs::create_dir_all(&tags_dir)?;

        for (tag, posts) in tags_map {
            let tag_dir = tags_dir.join(&tag);
            fs::create_dir_all(&tag_dir)?;

            let posts_vec: Vec<Post> = posts.into_iter().cloned().collect();
            let context = TagContext {
                site: &self.config.site,
                config: ConfigContext {
                    markdown: &self.config.markdown,
                },
                posts: &posts_vec,
                nav: &self.config.nav,
                tag: &tag,
            };

            let html = self.template_engine.render("archive.html", &context)?;
            fs::write(tag_dir.join("index.html"), html)?;

            info!("Generated tag page: /tags/{}/", tag);
        }

        Ok(())
    }

    /// Generate category pages
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

        for (category, posts) in categories_map {
            let category_dir = categories_dir.join(&category);
            fs::create_dir_all(&category_dir)?;

            let posts_vec: Vec<Post> = posts.into_iter().cloned().collect();
            let context = CategoryContext {
                site: &self.config.site,
                config: ConfigContext {
                    markdown: &self.config.markdown,
                },
                posts: &posts_vec,
                nav: &self.config.nav,
                category: &category,
            };

            let html = self.template_engine.render("archive.html", &context)?;
            fs::write(category_dir.join("index.html"), html)?;

            info!("Generated category page: /categories/{}/", category);
        }

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
