/// Converts Markdown input into HTML output.
///
/// Plain text is currently rendered as a single HTML paragraph.
pub fn parse(markdown: &str) -> String {
    if markdown.is_empty() {
        String::new()
    } else {
        format!("<p>{markdown}</p>")
    }
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
}
