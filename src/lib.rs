/// Converts Markdown input into HTML output.
///
/// Phase 0 only establishes the public entry point, so non-empty input is
/// temporarily returned unchanged. Actual Markdown behavior will be added one
/// small step at a time.
pub fn parse(markdown: &str) -> String {
    markdown.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_document() {
        assert_eq!(parse(""), "");
    }
}
