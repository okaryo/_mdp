/// Converts Markdown input into HTML output.
///
/// Plain text is currently rendered as a single HTML paragraph.
pub fn parse(markdown: &str) -> String {
    if markdown.is_empty() {
        return String::new();
    }

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

            blocks.push(format!(
                "<pre><code>{}</code></pre>",
                escape_html(&code_lines.join("\n"))
            ));
        } else if line.strip_prefix("- ").is_some() {
            let mut items = Vec::new();

            while index < lines.len() {
                if let Some(item) = lines[index].strip_prefix("- ") {
                    items.push(format!("<li>{}</li>", escape_html(item)));
                    index += 1;
                } else {
                    break;
                }
            }

            blocks.push(format!("<ul>{}</ul>", items.join("")));
        } else if ordered_list_item_content(line).is_some() {
            let mut items = Vec::new();

            while index < lines.len() {
                if let Some(item) = ordered_list_item_content(lines[index]) {
                    items.push(format!("<li>{}</li>", escape_html(item)));
                    index += 1;
                } else {
                    break;
                }
            }

            blocks.push(format!("<ol>{}</ol>", items.join("")));
        } else if line.strip_prefix("> ").is_some() {
            let mut quoted_lines = Vec::new();

            while index < lines.len() {
                if let Some(quote) = lines[index].strip_prefix("> ") {
                    quoted_lines.push(escape_html(quote));
                    index += 1;
                } else {
                    break;
                }
            }

            blocks.push(format!(
                "<blockquote><p>{}</p></blockquote>",
                quoted_lines.join("\n")
            ));
        } else {
            blocks.push(parse_line(line));
            index += 1;
        }
    }

    blocks.join("\n")
}

fn ordered_list_item_content(line: &str) -> Option<&str> {
    let (number, item) = line.split_once(". ")?;

    if !number.is_empty() && number.chars().all(|character| character.is_ascii_digit()) {
        Some(item)
    } else {
        None
    }
}

fn parse_line(line: &str) -> String {
    if let Some(heading) = line.strip_prefix("### ") {
        format!("<h3>{}</h3>", escape_html(heading))
    } else if let Some(heading) = line.strip_prefix("## ") {
        format!("<h2>{}</h2>", escape_html(heading))
    } else if let Some(heading) = line.strip_prefix("# ") {
        format!("<h1>{}</h1>", escape_html(heading))
    } else {
        format!("<p>{}</p>", escape_html(line))
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
}
