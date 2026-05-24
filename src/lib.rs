/// Converts Markdown input into HTML output.
///
/// Plain text is currently rendered as a single HTML paragraph.
pub fn parse(markdown: &str) -> String {
    if markdown.is_empty() {
        return String::new();
    }

    parse_blocks(markdown)
        .iter()
        .map(render_block)
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, PartialEq, Eq)]
enum BlockNode<'a> {
    Paragraph(&'a str),
    Heading { level: u8, text: &'a str },
    UnorderedList(Vec<&'a str>),
    OrderedList(Vec<&'a str>),
    BlockQuote(Vec<&'a str>),
    CodeBlock(Vec<&'a str>),
}

fn parse_blocks(markdown: &str) -> Vec<BlockNode<'_>> {
    let lines = markdown.lines().collect::<Vec<_>>();
    let mut blocks = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];

        if line == "```" {
            index += 1;
            let mut code_lines = Vec::new();

            while index < lines.len() && lines[index] != "```" {
                code_lines.push(lines[index]);
                index += 1;
            }

            if index < lines.len() {
                index += 1;
            }

            blocks.push(BlockNode::CodeBlock(code_lines));
        } else if line.strip_prefix("- ").is_some() {
            let mut items = Vec::new();

            while index < lines.len() {
                if let Some(item) = lines[index].strip_prefix("- ") {
                    items.push(item);
                    index += 1;
                } else {
                    break;
                }
            }

            blocks.push(BlockNode::UnorderedList(items));
        } else if ordered_list_item_content(line).is_some() {
            let mut items = Vec::new();

            while index < lines.len() {
                if let Some(item) = ordered_list_item_content(lines[index]) {
                    items.push(item);
                    index += 1;
                } else {
                    break;
                }
            }

            blocks.push(BlockNode::OrderedList(items));
        } else if line.strip_prefix("> ").is_some() {
            let mut quoted_lines = Vec::new();

            while index < lines.len() {
                if let Some(quote) = lines[index].strip_prefix("> ") {
                    quoted_lines.push(quote);
                    index += 1;
                } else {
                    break;
                }
            }

            blocks.push(BlockNode::BlockQuote(quoted_lines));
        } else {
            blocks.push(parse_line(line));
            index += 1;
        }
    }

    blocks
}

fn ordered_list_item_content(line: &str) -> Option<&str> {
    let (number, item) = line.split_once(". ")?;

    if !number.is_empty() && number.chars().all(|character| character.is_ascii_digit()) {
        Some(item)
    } else {
        None
    }
}

fn parse_line(line: &str) -> BlockNode<'_> {
    if let Some(heading) = line.strip_prefix("### ") {
        BlockNode::Heading {
            level: 3,
            text: heading,
        }
    } else if let Some(heading) = line.strip_prefix("## ") {
        BlockNode::Heading {
            level: 2,
            text: heading,
        }
    } else if let Some(heading) = line.strip_prefix("# ") {
        BlockNode::Heading {
            level: 1,
            text: heading,
        }
    } else {
        BlockNode::Paragraph(line)
    }
}

fn render_block(block: &BlockNode<'_>) -> String {
    match block {
        BlockNode::Paragraph(text) => format!("<p>{}</p>", render_inline(text)),
        BlockNode::Heading { level, text } => {
            format!("<h{level}>{}</h{level}>", render_inline(text))
        }
        BlockNode::UnorderedList(items) => {
            let rendered_items = items
                .iter()
                .map(|item| format!("<li>{}</li>", render_inline(item)))
                .collect::<Vec<_>>()
                .join("");
            format!("<ul>{rendered_items}</ul>")
        }
        BlockNode::OrderedList(items) => {
            let rendered_items = items
                .iter()
                .map(|item| format!("<li>{}</li>", render_inline(item)))
                .collect::<Vec<_>>()
                .join("");
            format!("<ol>{rendered_items}</ol>")
        }
        BlockNode::BlockQuote(lines) => {
            let rendered_lines = lines
                .iter()
                .map(|line| render_inline(line))
                .collect::<Vec<_>>()
                .join("\n");
            format!("<blockquote><p>{rendered_lines}</p></blockquote>")
        }
        BlockNode::CodeBlock(lines) => {
            format!("<pre><code>{}</code></pre>", escape_html(&lines.join("\n")))
        }
    }
}

fn render_inline(text: &str) -> String {
    let mut rendered = String::new();
    let mut remaining = text;

    while let Some((start, delimiter)) = find_next_inline_delimiter(remaining) {
        rendered.push_str(&escape_html(&remaining[..start]));

        match delimiter {
            '`' => {
                let inline_code_input = &remaining[start..];

                if let Some((html, consumed_len)) =
                    render_inline_code_from_tokens(inline_code_input)
                {
                    rendered.push_str(&html);
                    remaining = &remaining[start + consumed_len..];
                } else {
                    rendered.push_str("`");
                    remaining = &remaining[start + 1..];
                    rendered.push_str(&escape_html(remaining));
                    return rendered;
                }
            }
            '*' => {
                if remaining[start..].starts_with("**") {
                    remaining = &remaining[start + 2..];

                    if let Some(end) = remaining.find("**") {
                        rendered.push_str("<strong>");
                        rendered.push_str(&escape_html(&remaining[..end]));
                        rendered.push_str("</strong>");
                        remaining = &remaining[end + 2..];
                    } else {
                        rendered.push_str("**");
                        rendered.push_str(&escape_html(remaining));
                        return rendered;
                    }
                } else {
                    remaining = &remaining[start + 1..];

                    if let Some(end) = remaining.find('*') {
                        rendered.push_str("<em>");
                        rendered.push_str(&escape_html(&remaining[..end]));
                        rendered.push_str("</em>");
                        remaining = &remaining[end + 1..];
                    } else {
                        rendered.push_str("*");
                        rendered.push_str(&escape_html(remaining));
                        return rendered;
                    }
                }
            }
            '[' => {
                remaining = &remaining[start + 1..];

                if let Some(label_end) = remaining.find("](") {
                    let label = &remaining[..label_end];
                    let after_label = &remaining[label_end + 2..];

                    if let Some(url_end) = after_label.find(')') {
                        let url = &after_label[..url_end];
                        rendered.push_str("<a href=\"");
                        rendered.push_str(&escape_html_attribute(url));
                        rendered.push_str("\">");
                        rendered.push_str(&escape_html(label));
                        rendered.push_str("</a>");
                        remaining = &after_label[url_end + 1..];
                    } else {
                        rendered.push_str("[");
                        rendered.push_str(&escape_html(remaining));
                        return rendered;
                    }
                } else {
                    rendered.push_str("[");
                    rendered.push_str(&escape_html(remaining));
                    return rendered;
                }
            }
            _ => unreachable!("only configured inline delimiters are returned"),
        }
    }

    rendered.push_str(&escape_html(remaining));
    rendered
}

fn find_next_inline_delimiter(text: &str) -> Option<(usize, char)> {
    text.char_indices()
        .find(|(_, character)| matches!(character, '`' | '*' | '['))
}

#[derive(Debug, PartialEq, Eq)]
enum InlineToken<'a> {
    Text(&'a str),
    Backtick,
    Star,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    End,
}

fn tokenize_inline(text: &str) -> Vec<InlineToken<'_>> {
    let mut tokens = Vec::new();
    let mut text_start = 0;

    for (index, character) in text.char_indices() {
        let token = match character {
            '`' => InlineToken::Backtick,
            '*' => InlineToken::Star,
            '[' => InlineToken::OpenBracket,
            ']' => InlineToken::CloseBracket,
            '(' => InlineToken::OpenParen,
            ')' => InlineToken::CloseParen,
            _ => continue,
        };

        if text_start < index {
            tokens.push(InlineToken::Text(&text[text_start..index]));
        }

        tokens.push(token);
        text_start = index + character.len_utf8();
    }

    if text_start < text.len() {
        tokens.push(InlineToken::Text(&text[text_start..]));
    }

    tokens.push(InlineToken::End);
    tokens
}

fn render_inline_code_from_tokens(text: &str) -> Option<(String, usize)> {
    let tokens = tokenize_inline(text);

    if !matches!(tokens.first(), Some(InlineToken::Backtick)) {
        return None;
    }

    let mut code = String::new();
    let mut consumed_len = 1;

    for token in tokens.iter().skip(1) {
        match token {
            InlineToken::Backtick => {
                consumed_len += 1;
                return Some((format!("<code>{}</code>", escape_html(&code)), consumed_len));
            }
            InlineToken::End => return None,
            _ => {
                code.push_str(&inline_token_text(token));
                consumed_len += inline_token_len(token);
            }
        }
    }

    None
}

fn inline_token_text(token: &InlineToken<'_>) -> String {
    match token {
        InlineToken::Text(text) => text.to_string(),
        InlineToken::Backtick => "`".to_string(),
        InlineToken::Star => "*".to_string(),
        InlineToken::OpenBracket => "[".to_string(),
        InlineToken::CloseBracket => "]".to_string(),
        InlineToken::OpenParen => "(".to_string(),
        InlineToken::CloseParen => ")".to_string(),
        InlineToken::End => String::new(),
    }
}

fn inline_token_len(token: &InlineToken<'_>) -> usize {
    match token {
        InlineToken::Text(text) => text.len(),
        InlineToken::Backtick
        | InlineToken::Star
        | InlineToken::OpenBracket
        | InlineToken::CloseBracket
        | InlineToken::OpenParen
        | InlineToken::CloseParen => 1,
        InlineToken::End => 0,
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

fn escape_html_attribute(text: &str) -> String {
    let mut escaped = String::new();

    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
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
    fn parses_blocks_into_block_nodes() {
        assert_eq!(
            parse_blocks("# Title\n- Apples\n- Oranges\n``` \nnot a fence"),
            vec![
                BlockNode::Heading {
                    level: 1,
                    text: "Title"
                },
                BlockNode::UnorderedList(vec!["Apples", "Oranges"]),
                BlockNode::Paragraph("``` "),
                BlockNode::Paragraph("not a fence"),
            ]
        );
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

    #[test]
    fn tokenizes_inline_syntax_delimiters() {
        assert_eq!(
            tokenize_inline("Use `code` and [Rust](url)"),
            vec![
                InlineToken::Text("Use "),
                InlineToken::Backtick,
                InlineToken::Text("code"),
                InlineToken::Backtick,
                InlineToken::Text(" and "),
                InlineToken::OpenBracket,
                InlineToken::Text("Rust"),
                InlineToken::CloseBracket,
                InlineToken::OpenParen,
                InlineToken::Text("url"),
                InlineToken::CloseParen,
                InlineToken::End,
            ]
        );
    }

    #[test]
    fn tokenizes_plain_text_as_one_text_token() {
        assert_eq!(
            tokenize_inline("plain text"),
            vec![InlineToken::Text("plain text"), InlineToken::End]
        );
    }

    #[test]
    fn tokenizes_multibyte_text_without_splitting_characters() {
        assert_eq!(
            tokenize_inline("Rustは*楽しい*"),
            vec![
                InlineToken::Text("Rustは"),
                InlineToken::Star,
                InlineToken::Text("楽しい"),
                InlineToken::Star,
                InlineToken::End,
            ]
        );
    }
}
