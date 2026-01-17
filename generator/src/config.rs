use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub site: SiteConfig,
    pub build: BuildConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub markdown: MarkdownConfig,
    #[serde(default)]
    pub pagination: PaginationConfig,
    #[serde(default)]
    pub nav: Vec<NavItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SiteConfig {
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    pub url: String,
    #[serde(default)]
    pub base_path: String,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_content_dir")]
    pub content_dir: String,
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_content_dir() -> String {
    "content".to_string()
}

fn default_output_dir() -> String {
    "output".to_string()
}

fn default_theme() -> String {
    "default".to_string()
}

#[derive(Debug, Deserialize, Default)]
pub struct ThemeConfig {
    #[serde(default = "default_theme")]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarkdownConfig {
    #[serde(default = "default_true")]
    pub syntax_highlight: bool,
    #[serde(default = "default_syntax_theme")]
    pub syntax_theme: String,
    #[serde(default)]
    pub mermaid: bool,
    #[serde(default)]
    pub math: bool,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            syntax_highlight: true,
            syntax_theme: default_syntax_theme(),
            mermaid: false,
            math: false,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_syntax_theme() -> String {
    "base16-ocean.dark".to_string()
}

#[derive(Debug, Deserialize, Default)]
pub struct PaginationConfig {
    #[serde(default = "default_posts_per_page")]
    pub posts_per_page: usize,
}

fn default_posts_per_page() -> usize {
    10
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NavItem {
    pub title: String,
    pub url: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Configuration file not found: {}", path.display()))?;

        let config: Config = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse configuration file: {}", path.display()))?;

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.site.title.is_empty() {
            anyhow::bail!("Configuration error: site.title is required");
        }
        if self.site.url.is_empty() {
            anyhow::bail!("Configuration error: site.url is required");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() {
        let yaml = r#"
site:
  title: "Test Blog"
  url: "https://example.com"
build:
  content_dir: "content"
  output_dir: "output"
"#;
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();

        let config = Config::load(file.path()).unwrap();
        assert_eq!(config.site.title, "Test Blog");
        assert_eq!(config.site.url, "https://example.com");
    }

    #[test]
    fn test_missing_config_file() {
        let result = Config::load("/nonexistent/config.yaml");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Configuration file not found"));
    }

    #[test]
    fn test_missing_required_field() {
        let yaml = r#"
site:
  title: ""
  url: "https://example.com"
build:
  content_dir: "content"
"#;
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(yaml.as_bytes()).unwrap();

        let result = Config::load(file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("site.title is required"));
    }
}
