#[test]
fn parses_larger_markdown_fixture() {
    let markdown = include_str!("fixtures/sample.md");
    let expected_html = include_str!("fixtures/sample.html");

    assert_eq!(mdp::parse(markdown), expected_html.trim_end());
}
