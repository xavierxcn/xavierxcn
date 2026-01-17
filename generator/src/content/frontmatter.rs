use anyhow::{Context, Result};
use serde::de::DeserializeOwned;

const FRONTMATTER_DELIMITER: &str = "---";

/// Parse YAML front matter from markdown content
/// Returns the parsed metadata and the remaining content
pub fn parse_frontmatter<T: DeserializeOwned>(content: &str) -> Result<(T, String)> {
    let content = content.trim_start();

    if !content.starts_with(FRONTMATTER_DELIMITER) {
        anyhow::bail!("No front matter found: file must start with ---");
    }

    let after_first = &content[FRONTMATTER_DELIMITER.len()..];
    let end_pos = after_first
        .find(&format!("\n{}", FRONTMATTER_DELIMITER))
        .context("No closing --- found for front matter")?;

    let yaml_content = &after_first[..end_pos];
    let markdown_start = end_pos + 1 + FRONTMATTER_DELIMITER.len();
    let markdown_content = after_first[markdown_start..].trim_start().to_string();

    let meta: T = serde_yaml::from_str(yaml_content.trim())
        .context("Failed to parse front matter YAML")?;

    Ok((meta, markdown_content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TestMeta {
        title: String,
        #[serde(default)]
        tags: Vec<String>,
    }

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
title: Hello World
tags:
  - rust
  - blog
---
# Content here
"#;
        let (meta, body): (TestMeta, String) = parse_frontmatter(content).unwrap();
        assert_eq!(meta.title, "Hello World");
        assert_eq!(meta.tags, vec!["rust", "blog"]);
        assert!(body.starts_with("# Content here"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just markdown\n\nNo front matter here.";
        let result = parse_frontmatter::<TestMeta>(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_frontmatter() {
        let content = "---\ntitle: Test\n# No closing delimiter";
        let result = parse_frontmatter::<TestMeta>(content);
        assert!(result.is_err());
    }
}
