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
    let url = format!("/posts/{}/", slug);
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
    let url = format!("/{}/", slug);
    let html_content = render_markdown(&markdown_content);

    Ok(Page {
        slug,
        url,
        title: meta.title,
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
}
