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
}

/// Context for rendering tag/category pages
#[derive(Debug, Serialize)]
pub struct TagContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub posts: &'a [crate::content::Post],
    pub nav: &'a [crate::config::NavItem],
    pub tag: &'a str,
}

/// Context for rendering category pages
#[derive(Debug, Serialize)]
pub struct CategoryContext<'a> {
    pub site: &'a crate::config::SiteConfig,
    pub config: ConfigContext<'a>,
    pub posts: &'a [crate::content::Post],
    pub nav: &'a [crate::config::NavItem],
    pub category: &'a str,
}

/// Subset of config exposed to templates
#[derive(Debug, Serialize)]
pub struct ConfigContext<'a> {
    pub markdown: &'a crate::config::MarkdownConfig,
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
