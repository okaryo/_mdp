mod block;
mod inline;
mod renderer;

/// Converts Markdown input into HTML output.
///
/// Plain text is currently rendered as a single HTML paragraph.
pub fn parse(markdown: &str) -> String {
    if markdown.is_empty() {
        return String::new();
    }

    renderer::render_blocks(&block::parse_blocks(markdown))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_document() {
        assert_eq!(parse(""), "");
    }

    #[test]
    fn renders_plain_text_as_a_paragraph() {
        assert_eq!(parse("Hello, Markdown!"), "<p>Hello, Markdown!</p>");
    }

    #[test]
    fn escapes_html_sensitive_characters() {
        assert_eq!(
            parse("Hello <Rust> & Markdown"),
            "<p>Hello &lt;Rust&gt; &amp; Markdown</p>"
        );
    }

    #[test]
    fn renders_level_1_heading() {
        assert_eq!(parse("# Hello, Markdown!"), "<h1>Hello, Markdown!</h1>");
    }

    #[test]
    fn renders_level_2_heading() {
        assert_eq!(parse("## Hello, Markdown!"), "<h2>Hello, Markdown!</h2>");
    }

    #[test]
    fn renders_level_3_heading() {
        assert_eq!(parse("### Hello, Markdown!"), "<h3>Hello, Markdown!</h3>");
    }

    #[test]
    fn renders_multiple_lines_as_separate_blocks() {
        assert_eq!(
            parse("# Title\nHello, Markdown!"),
            "<h1>Title</h1>\n<p>Hello, Markdown!</p>"
        );
    }

    #[test]
    fn renders_unordered_list() {
        assert_eq!(
            parse("- Apples\n- Oranges"),
            "<ul><li>Apples</li><li>Oranges</li></ul>"
        );
    }

    #[test]
    fn renders_unordered_list_with_surrounding_blocks() {
        assert_eq!(
            parse("# Groceries\n- Apples\n- Oranges\nDone"),
            "<h1>Groceries</h1>\n<ul><li>Apples</li><li>Oranges</li></ul>\n<p>Done</p>"
        );
    }

    #[test]
    fn renders_ordered_list() {
        assert_eq!(
            parse("1. First\n2. Second"),
            "<ol><li>First</li><li>Second</li></ol>"
        );
    }

    #[test]
    fn renders_ordered_list_with_surrounding_blocks() {
        assert_eq!(
            parse("# Steps\n1. Read\n2. Write\nDone"),
            "<h1>Steps</h1>\n<ol><li>Read</li><li>Write</li></ol>\n<p>Done</p>"
        );
    }

    #[test]
    fn renders_block_quote() {
        assert_eq!(
            parse("> Stay focused\n> Keep learning"),
            "<blockquote><p>Stay focused\nKeep learning</p></blockquote>"
        );
    }

    #[test]
    fn renders_block_quote_with_surrounding_blocks() {
        assert_eq!(
            parse("# Note\n> Stay focused\n> Keep learning\nDone"),
            "<h1>Note</h1>\n<blockquote><p>Stay focused\nKeep learning</p></blockquote>\n<p>Done</p>"
        );
    }

    #[test]
    fn renders_fenced_code_block() {
        let input = r#"```
fn main() {
    println!("Hello");
}
```"#;

        let expected = r#"<pre><code>fn main() {
    println!("Hello");
}</code></pre>"#;

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn does_not_parse_markdown_inside_fenced_code_block() {
        let input = r#"```
# Not a heading
- Not a list item
```"#;

        let expected = "<pre><code># Not a heading\n- Not a list item</code></pre>";

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn treats_unterminated_fenced_code_block_as_code_until_end_of_input() {
        let input = r#"```
let value = 1 < 2;
# Not a heading"#;

        let expected = "<pre><code>let value = 1 &lt; 2;\n# Not a heading</code></pre>";

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn renders_inline_code_in_paragraph() {
        assert_eq!(
            parse("Use `cargo test` often"),
            "<p>Use <code>cargo test</code> often</p>"
        );
    }

    #[test]
    fn renders_inline_code_in_heading() {
        assert_eq!(
            parse("# Use `cargo test`"),
            "<h1>Use <code>cargo test</code></h1>"
        );
    }

    #[test]
    fn escapes_html_inside_inline_code() {
        assert_eq!(
            parse("Use `<tag>` as text"),
            "<p>Use <code>&lt;tag&gt;</code> as text</p>"
        );
    }

    #[test]
    fn treats_inline_markdown_delimiters_as_text_inside_inline_code() {
        assert_eq!(
            parse("Use `*not emphasis* [link](url)`"),
            "<p>Use <code>*not emphasis* [link](url)</code></p>"
        );
    }

    #[test]
    fn renders_emphasis_in_paragraph() {
        assert_eq!(
            parse("Keep *learning* Rust"),
            "<p>Keep <em>learning</em> Rust</p>"
        );
    }

    #[test]
    fn leaves_unclosed_emphasis_as_text() {
        assert_eq!(parse("Keep *learning Rust"), "<p>Keep *learning Rust</p>");
    }

    #[test]
    fn renders_inline_code_before_emphasis_when_it_appears_first() {
        assert_eq!(
            parse("Use `cargo test` and *learn*"),
            "<p>Use <code>cargo test</code> and <em>learn</em></p>"
        );
    }

    #[test]
    fn renders_strong_emphasis_in_paragraph() {
        assert_eq!(
            parse("Keep **learning** Rust"),
            "<p>Keep <strong>learning</strong> Rust</p>"
        );
    }

    #[test]
    fn leaves_unclosed_strong_emphasis_as_text() {
        assert_eq!(parse("Keep **learning Rust"), "<p>Keep **learning Rust</p>");
    }

    #[test]
    fn renders_strong_emphasis_before_emphasis_when_it_appears_first() {
        assert_eq!(
            parse("Keep **learning** and *practicing*"),
            "<p>Keep <strong>learning</strong> and <em>practicing</em></p>"
        );
    }

    #[test]
    fn renders_link_in_paragraph() {
        assert_eq!(
            parse("Visit [Rust](https://www.rust-lang.org/)"),
            "<p>Visit <a href=\"https://www.rust-lang.org/\">Rust</a></p>"
        );
    }

    #[test]
    fn escapes_link_label_and_url_attribute() {
        assert_eq!(
            parse(r#"Visit [<Rust>](https://example.com/?q="rust"&page=1)"#),
            "<p>Visit <a href=\"https://example.com/?q=&quot;rust&quot;&amp;page=1\">&lt;Rust&gt;</a></p>"
        );
    }

    #[test]
    fn leaves_incomplete_link_as_text() {
        assert_eq!(
            parse("Visit [Rust](https://www.rust-lang.org/"),
            "<p>Visit [Rust](https://www.rust-lang.org/</p>"
        );
    }
}
