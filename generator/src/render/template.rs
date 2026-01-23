use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;
use tera::Tera;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Load templates from the theme directory
    pub fn new<P: AsRef<Path>>(theme_dir: P) -> Result<Self> {
        let templates_dir = theme_dir.as_ref().join("templates");
        let pattern = templates_dir.join("**").join("*.html");
        let pattern_str = pattern.to_string_lossy();

        let tera = Tera::new(&pattern_str)
            .with_context(|| format!("Failed to load templates from {}", templates_dir.display()))?;

        Ok(Self { tera })
    }

    /// Render a template with the given context
    pub fn render<T: Serialize>(&self, template: &str, context: &T) -> Result<String> {
        let tera_context = tera::Context::from_serialize(context)
            .context("Failed to serialize template context")?;

        self.tera
            .render(template, &tera_context)
            .with_context(|| format!("Failed to render template: {}", template))
    }
}

/// Context for rendering a post page
#[derive(Debug, Serialize)]
pub struct PostContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub post: &'a crate::content::Post,
    pub nav: &'a [crate::config::NavItem],
}

/// Context for rendering a page
#[derive(Debug, Serialize)]
pub struct PageContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub page: &'a crate::content::Page,
    pub nav: &'a [crate::config::NavItem],
}

/// Context for rendering index/archive pages
#[derive(Debug, Serialize)]
pub struct ListContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub posts: &'a [crate::content::Post],
    pub nav: &'a [crate::config::NavItem],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// Context for rendering tag/category pages
#[derive(Debug, Serialize)]
pub struct TagContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub posts: &'a [crate::content::Post],
    pub nav: &'a [crate::config::NavItem],
    pub tag: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// Context for rendering category pages
#[derive(Debug, Serialize)]
pub struct CategoryContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub posts: &'a [crate::content::Post],
    pub nav: &'a [crate::config::NavItem],
    pub category: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// Context for rendering the tools list page
#[derive(Debug, Serialize)]
pub struct ToolsListContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub tools: &'a [crate::content::Tool],
    pub nav: &'a [crate::config::NavItem],
}

/// Context for rendering the astrology index page
#[derive(Debug, Serialize)]
pub struct AstrologyIndexContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub categories: &'a [crate::content::AstrologyCategory],
    pub nav: &'a [crate::config::NavItem],
}

/// Context for rendering an astrology category page
#[derive(Debug, Serialize)]
pub struct AstrologyCategoryContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub category: &'a crate::content::AstrologyCategory,
    pub items: &'a [crate::content::AstrologyItem],
    pub nav: &'a [crate::config::NavItem],
}

/// Context for rendering an individual astrology item page
#[derive(Debug, Serialize)]
pub struct AstrologyItemContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub item: &'a crate::content::AstrologyItem,
    pub category: &'a crate::content::AstrologyCategory,
    pub nav: &'a [crate::config::NavItem],
}

/// Subset of config exposed to templates
#[derive(Debug, Serialize)]
pub struct ConfigContext<'a> {
    pub markdown: &'a crate::config::MarkdownConfig,
}

/// Pagination information for list pages
#[derive(Debug, Clone, Serialize)]
pub struct Pagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub total_items: usize,
    pub per_page: usize,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}

impl Pagination {
    /// Create pagination info for a list of items
    pub fn new(
        total_items: usize,
        per_page: usize,
        current_page: usize,
        base_url: &str,
    ) -> Self {
        let total_pages = if total_items == 0 {
            1
        } else {
            (total_items + per_page - 1) / per_page
        };

        // Normalize base_url: remove leading slash if present
        let base_url = base_url.trim_start_matches('/');

        let prev_page = if current_page > 1 {
            if current_page == 2 {
                // Page 2's prev is the first page
                if base_url.is_empty() {
                    // Index root - prev is just empty (root)
                    Some(String::new())
                } else {
                    Some(format!("{}/", base_url))
                }
            } else {
                Some(format!("{}/page/{}/", base_url, current_page - 1))
            }
        } else {
            None
        };

        let next_page = if current_page < total_pages {
            Some(format!("{}/page/{}/", base_url, current_page + 1))
        } else {
            None
        };

        Self {
            current_page,
            total_pages,
            total_items,
            per_page,
            prev_page,
            next_page,
        }
    }

    /// Get a slice of items for the current page
    pub fn paginate<'a, T>(&self, items: &'a [T]) -> &'a [T] {
        let start = (self.current_page - 1) * self.per_page;
        let end = std::cmp::min(start + self.per_page, items.len());
        if start >= items.len() {
            &[]
        } else {
            &items[start..end]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_templates() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        fs::create_dir_all(&templates_dir).unwrap();

        fs::write(
            templates_dir.join("test.html"),
            "Hello {{ name }}!",
        ).unwrap();

        let engine = TemplateEngine::new(temp_dir.path()).unwrap();

        #[derive(Serialize)]
        struct Ctx {
            name: String,
        }

        let result = engine.render("test.html", &Ctx { name: "World".to_string() }).unwrap();
        assert_eq!(result, "Hello World!");
    }
}
