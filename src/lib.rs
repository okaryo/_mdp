/// Converts Markdown input into HTML output.
///
/// Plain text is currently rendered as a single HTML paragraph.
pub fn parse(markdown: &str) -> String {
    if markdown.is_empty() {
        String::new()
    } else if let Some(heading) = markdown.strip_prefix("# ") {
        format!("<h1>{}</h1>", escape_html(heading))
    } else {
        format!("<p>{}</p>", escape_html(markdown))
    }
}

fn escape_html(text: &str) -> String {
    let mut escaped = String::new();

    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(character),
        }
    }

    escaped
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
}
