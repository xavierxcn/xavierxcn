mod frontmatter;
mod markdown;

pub use frontmatter::parse_frontmatter;
pub use markdown::render_markdown;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::warn;
use walkdir::WalkDir;

/// Post metadata from front matter
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostMeta {
    pub title: String,
    #[serde(default)]
    pub date: Option<NaiveDate>,
    #[serde(default)]
    pub updated: Option<NaiveDate>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub draft: bool,
}

/// A blog post
#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub slug: String,
    pub url: String,
    #[serde(flatten)]
    pub meta: PostMeta,
    #[serde(rename = "content")]
    pub html_content: String,
}

/// Page metadata from front matter
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageMeta {
    pub title: String,
}

/// A standalone page
#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub slug: String,
    pub url: String,
    pub title: String,
    #[serde(rename = "content")]
    pub html_content: String,
}

/// Tool metadata from _meta.yaml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolMeta {
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

/// Tools metadata file (_meta.yaml)
#[derive(Debug, Clone, Deserialize)]
pub struct ToolsMetaFile {
    #[serde(default)]
    pub tools: Vec<ToolMeta>,
}

/// A tool page (for template rendering)
#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    pub slug: String,
    pub url: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

/// Astrology item metadata from front matter
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AstrologyMeta {
    pub title: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub element: Option<String>,
    #[serde(default)]
    pub modality: Option<String>,
    #[serde(default)]
    pub ruling_planet: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

/// An astrology item (sign, planet, house, or aspect)
#[derive(Debug, Clone, Serialize)]
pub struct AstrologyItem {
    pub slug: String,
    pub url: String,
    pub category: String,
    #[serde(flatten)]
    pub meta: AstrologyMeta,
    #[serde(rename = "content")]
    pub html_content: String,
}

/// Astrology category with its items
#[derive(Debug, Clone, Serialize)]
pub struct AstrologyCategory {
    pub id: String,
    pub name: String,
    pub items: Vec<AstrologyItem>,
}

/// All astrology data organized by category
#[derive(Debug, Clone, Serialize)]
pub struct AstrologyData {
    pub categories: Vec<AstrologyCategory>,
}

impl AstrologyData {
    /// Get all items across all categories
    pub fn all_items(&self) -> Vec<&AstrologyItem> {
        self.categories.iter().flat_map(|c| c.items.iter()).collect()
    }

    /// Get a specific category by id
    pub fn get_category(&self, id: &str) -> Option<&AstrologyCategory> {
        self.categories.iter().find(|c| c.id == id)
    }
}

/// Generate slug from filename
fn slug_from_filename(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled")
        .to_string()
}

/// Load all posts from the posts directory
pub fn load_posts<P: AsRef<Path>>(content_dir: P) -> Result<Vec<Post>> {
    let posts_dir = content_dir.as_ref().join("posts");
    if !posts_dir.exists() {
        return Ok(Vec::new());
    }

    let mut posts = Vec::new();

    for entry in WalkDir::new(&posts_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        match load_post(path) {
            Ok(Some(post)) => posts.push(post),
            Ok(None) => {} // Draft, skip
            Err(e) => {
                warn!("Failed to load post {}: {}", path.display(), e);
            }
        }
    }

    // Sort by date descending (newest first)
    posts.sort_by(|a, b| {
        let date_a = a.meta.date.unwrap_or(NaiveDate::MIN);
        let date_b = b.meta.date.unwrap_or(NaiveDate::MIN);
        date_b.cmp(&date_a)
    });

    Ok(posts)
}

/// Load a single post from a file
fn load_post(path: &Path) -> Result<Option<Post>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let (meta, markdown_content) = parse_frontmatter::<PostMeta>(&content)?;

    // Skip drafts
    if meta.draft {
        return Ok(None);
    }

    let slug = slug_from_filename(path);
    let url = format!("posts/{}/", slug);
    let html_content = render_markdown(&markdown_content);

    Ok(Some(Post {
        slug,
        url,
        meta,
        html_content,
    }))
}

/// Load all pages from the pages directory
pub fn load_pages<P: AsRef<Path>>(content_dir: P) -> Result<Vec<Page>> {
    let pages_dir = content_dir.as_ref().join("pages");
    if !pages_dir.exists() {
        return Ok(Vec::new());
    }

    let mut pages = Vec::new();

    for entry in WalkDir::new(&pages_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        match load_page(path) {
            Ok(page) => pages.push(page),
            Err(e) => {
                warn!("Failed to load page {}: {}", path.display(), e);
            }
        }
    }

    Ok(pages)
}

/// Load a single page from a file
fn load_page(path: &Path) -> Result<Page> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let (meta, markdown_content) = parse_frontmatter::<PageMeta>(&content)?;
    let slug = slug_from_filename(path);
    let url = format!("{}/", slug);
    let html_content = render_markdown(&markdown_content);

    Ok(Page {
        slug,
        url,
        title: meta.title,
        html_content,
    })
}

/// Load all tools from the _meta.yaml file
pub fn load_tools<P: AsRef<Path>>(content_dir: P) -> Result<Vec<Tool>> {
    let meta_path = content_dir.as_ref().join("tools/_meta.yaml");
    if !meta_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&meta_path)
        .with_context(|| format!("Failed to read tools metadata: {}", meta_path.display()))?;

    let meta_file: ToolsMetaFile = serde_yaml::from_str(&content)
        .with_context(|| "Failed to parse tools/_meta.yaml")?;

    let tools = meta_file
        .tools
        .into_iter()
        .map(|meta| Tool {
            url: format!("tools/{}/", meta.slug),
            slug: meta.slug,
            title: meta.title,
            description: meta.description,
            icon: meta.icon,
        })
        .collect();

    Ok(tools)
}

/// Astrology category definitions
const ASTROLOGY_CATEGORIES: &[(&str, &str)] = &[
    ("signs", "星座"),
    ("planets", "行星"),
    ("houses", "宫位"),
    ("aspects", "相位"),
];

/// Load all astrology content from the astrology directory
pub fn load_astrology<P: AsRef<Path>>(content_dir: P) -> Result<AstrologyData> {
    let astrology_dir = content_dir.as_ref().join("astrology");

    let mut categories = Vec::new();

    for (category_id, category_name) in ASTROLOGY_CATEGORIES {
        let category_dir = astrology_dir.join(category_id);
        let items = if category_dir.exists() {
            load_astrology_category(&category_dir, category_id)?
        } else {
            Vec::new()
        };

        categories.push(AstrologyCategory {
            id: category_id.to_string(),
            name: category_name.to_string(),
            items,
        });
    }

    Ok(AstrologyData { categories })
}

/// Load astrology items from a category directory
fn load_astrology_category(category_dir: &Path, category_id: &str) -> Result<Vec<AstrologyItem>> {
    let mut items = Vec::new();

    for entry in WalkDir::new(category_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        match load_astrology_item(path, category_id) {
            Ok(item) => items.push(item),
            Err(e) => {
                warn!("Failed to load astrology item {}: {}", path.display(), e);
            }
        }
    }

    // Sort by title
    items.sort_by(|a, b| a.meta.title.cmp(&b.meta.title));

    Ok(items)
}

/// Load a single astrology item from a file
fn load_astrology_item(path: &Path, category_id: &str) -> Result<AstrologyItem> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let (meta, markdown_content) = parse_frontmatter::<AstrologyMeta>(&content)?;
    let slug = slug_from_filename(path);
    let url = format!("astrology/{}/{}/", category_id, slug);
    let html_content = render_markdown(&markdown_content);

    Ok(AstrologyItem {
        slug,
        url,
        category: category_id.to_string(),
        meta,
        html_content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_slug_from_filename() {
        let path = Path::new("/content/posts/hello-world.md");
        assert_eq!(slug_from_filename(path), "hello-world");
    }

    #[test]
    fn test_load_posts() {
        let temp_dir = TempDir::new().unwrap();
        let posts_dir = temp_dir.path().join("posts");
        fs::create_dir_all(&posts_dir).unwrap();

        let post_content = r#"---
title: Test Post
date: 2024-01-15
tags:
  - test
---
# Hello World
"#;
        fs::write(posts_dir.join("test-post.md"), post_content).unwrap();

        let posts = load_posts(temp_dir.path()).unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].slug, "test-post");
        assert_eq!(posts[0].meta.title, "Test Post");
    }

    #[test]
    fn test_skip_draft_posts() {
        let temp_dir = TempDir::new().unwrap();
        let posts_dir = temp_dir.path().join("posts");
        fs::create_dir_all(&posts_dir).unwrap();

        let draft_content = r#"---
title: Draft Post
draft: true
---
# Draft
"#;
        fs::write(posts_dir.join("draft.md"), draft_content).unwrap();

        let posts = load_posts(temp_dir.path()).unwrap();
        assert_eq!(posts.len(), 0);
    }

    #[test]
    fn test_load_astrology() {
        let temp_dir = TempDir::new().unwrap();
        let signs_dir = temp_dir.path().join("astrology/signs");
        fs::create_dir_all(&signs_dir).unwrap();

        let aries_content = r#"---
title: 白羊座
symbol: "♈"
element: 火
modality: 本位
ruling_planet: 火星
keywords:
  - 开创
  - 勇敢
summary: 黄道第一宫，象征新的开始
---
# 白羊座

白羊座是黄道十二宫的第一宫。
"#;
        fs::write(signs_dir.join("aries.md"), aries_content).unwrap();

        let data = load_astrology(temp_dir.path()).unwrap();
        assert_eq!(data.categories.len(), 4);

        let signs = data.get_category("signs").unwrap();
        assert_eq!(signs.items.len(), 1);
        assert_eq!(signs.items[0].slug, "aries");
        assert_eq!(signs.items[0].meta.title, "白羊座");
        assert_eq!(signs.items[0].meta.symbol, Some("♈".to_string()));
        assert_eq!(signs.items[0].meta.element, Some("火".to_string()));
    }

    #[test]
    fn test_load_astrology_empty() {
        let temp_dir = TempDir::new().unwrap();
        let data = load_astrology(temp_dir.path()).unwrap();
        assert_eq!(data.categories.len(), 4);
        for cat in &data.categories {
            assert_eq!(cat.items.len(), 0);
        }
    }
}
