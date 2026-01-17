use std::sync::LazyLock;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

pub struct SyntaxHighlighter {
    theme_name: String,
}

impl SyntaxHighlighter {
    pub fn new(theme_name: &str) -> Self {
        Self {
            theme_name: theme_name.to_string(),
        }
    }

    /// Highlight code with the specified language
    pub fn highlight(&self, code: &str, lang: &str) -> String {
        // Handle mermaid specially - preserve for client-side rendering
        if lang == "mermaid" {
            return format!(
                "<pre class=\"mermaid\">{}</pre>",
                html_escape::encode_text(code)
            );
        }

        let syntax = SYNTAX_SET
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

        let theme = THEME_SET
            .themes
            .get(&self.theme_name)
            .or_else(|| THEME_SET.themes.get("base16-ocean.dark"))
            .expect("Default theme should exist");

        match highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme) {
            Ok(html) => html,
            Err(_) => format!(
                "<pre><code>{}</code></pre>",
                html_escape::encode_text(code)
            ),
        }
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new("base16-ocean.dark")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_rust() {
        let highlighter = SyntaxHighlighter::default();
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let html = highlighter.highlight(code, "rust");
        assert!(html.contains("fn"));
        assert!(html.contains("style="));
    }

    #[test]
    fn test_highlight_unknown_lang() {
        let highlighter = SyntaxHighlighter::default();
        let code = "some code";
        let html = highlighter.highlight(code, "nonexistent");
        assert!(html.contains("some code"));
    }

    #[test]
    fn test_mermaid_passthrough() {
        let highlighter = SyntaxHighlighter::default();
        let code = "graph TD\n    A --> B";
        let html = highlighter.highlight(code, "mermaid");
        assert!(html.contains("class=\"mermaid\""));
        assert!(html.contains("graph TD"));
    }
}
