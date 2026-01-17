use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::sync::LazyLock;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

/// Render markdown content to HTML with syntax highlighting
pub fn render_markdown(content: &str) -> String {
    render_markdown_with_theme(content, "base16-ocean.dark")
}

/// Render markdown content to HTML with specified syntax theme
pub fn render_markdown_with_theme(content: &str, theme_name: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();

    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                code_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let highlighted = highlight_code(&code_content, &code_lang, theme_name);
                html_output.push_str(&highlighted);
                code_lang.clear();
                code_content.clear();
            }
            Event::Text(text) if in_code_block => {
                code_content.push_str(&text);
            }
            _ => {
                // For other events, use default HTML rendering
                let mut temp = String::new();
                pulldown_cmark::html::push_html(&mut temp, std::iter::once(event));
                html_output.push_str(&temp);
            }
        }
    }

    html_output
}

/// Highlight code with syntax highlighting or handle special cases like mermaid
fn highlight_code(code: &str, lang: &str, theme_name: &str) -> String {
    let lang = lang.trim();

    // Handle mermaid - preserve for client-side rendering
    if lang == "mermaid" {
        return format!(
            "<pre class=\"mermaid\">{}</pre>\n",
            html_escape::encode_text(code)
        );
    }

    // Empty language - plain code block
    if lang.is_empty() {
        return format!(
            "<pre><code>{}</code></pre>\n",
            html_escape::encode_text(code)
        );
    }

    // Try to find syntax for the language
    let syntax = SYNTAX_SET
        .find_syntax_by_token(lang)
        .or_else(|| SYNTAX_SET.find_syntax_by_extension(lang));

    match syntax {
        Some(syntax) => {
            let theme = THEME_SET
                .themes
                .get(theme_name)
                .or_else(|| THEME_SET.themes.get("base16-ocean.dark"))
                .expect("Default theme should exist");

            match highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme) {
                Ok(html) => html,
                Err(_) => format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>\n",
                    html_escape::encode_text(lang),
                    html_escape::encode_text(code)
                ),
            }
        }
        None => {
            // Unknown language - render as plain code with language class
            format!(
                "<pre><code class=\"language-{}\">{}</code></pre>\n",
                html_escape::encode_text(lang),
                html_escape::encode_text(code)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let md = "# Hello\n\nThis is **bold** and *italic*.";
        let html = render_markdown(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[test]
    fn test_table() {
        let md = r#"
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
"#;
        let html = render_markdown(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Header 1</th>"));
    }

    #[test]
    fn test_task_list() {
        let md = "- [x] Done\n- [ ] Todo";
        let html = render_markdown(md);
        assert!(html.contains("checked"));
        assert!(html.contains("type=\"checkbox\""));
    }

    #[test]
    fn test_strikethrough() {
        let md = "This is ~~deleted~~ text.";
        let html = render_markdown(md);
        assert!(html.contains("<del>deleted</del>"));
    }

    #[test]
    fn test_code_highlight_rust() {
        let md = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let html = render_markdown(md);
        // Should contain syntax highlighting (style attributes)
        assert!(html.contains("style="));
        assert!(html.contains("fn"));
    }

    #[test]
    fn test_code_highlight_javascript() {
        let md = "```javascript\nconst x = 42;\n```";
        let html = render_markdown(md);
        assert!(html.contains("style="));
        assert!(html.contains("const"));
    }

    #[test]
    fn test_mermaid_passthrough() {
        let md = "```mermaid\ngraph TD\n    A --> B\n```";
        let html = render_markdown(md);
        assert!(html.contains("class=\"mermaid\""));
        assert!(html.contains("graph TD"));
        // Should NOT have syntax highlighting
        assert!(!html.contains("style="));
    }

    #[test]
    fn test_unknown_language() {
        let md = "```unknownlang\nsome code\n```";
        let html = render_markdown(md);
        assert!(html.contains("language-unknownlang"));
        assert!(html.contains("some code"));
    }

    #[test]
    fn test_plain_code_block() {
        let md = "```\nplain code\n```";
        let html = render_markdown(md);
        assert!(html.contains("<pre><code>"));
        assert!(html.contains("plain code"));
    }
}
