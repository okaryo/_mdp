pub(crate) fn render_inline(text: &str) -> String {
    render_inline_nodes(&parse_inline_nodes(text))
}

#[derive(Debug, PartialEq, Eq)]
enum InlineNode<'a> {
    Text(&'a str),
    Code(String),
    Emphasis(Vec<InlineNode<'a>>),
    Strong(Vec<InlineNode<'a>>),
    Link {
        label: Vec<InlineNode<'a>>,
        url: &'a str,
    },
}

fn parse_inline_nodes(text: &str) -> Vec<InlineNode<'_>> {
    let mut nodes = Vec::new();
    let mut remaining = text;

    while let Some((start, delimiter)) = find_next_inline_delimiter(remaining) {
        if start > 0 {
            nodes.push(InlineNode::Text(&remaining[..start]));
        }

        match delimiter {
            '`' => {
                let inline_code_input = &remaining[start..];

                if let Some((inline_node, consumed_len)) =
                    render_inline_code_from_tokens(inline_code_input)
                {
                    nodes.push(inline_node);
                    remaining = &remaining[start + consumed_len..];
                } else {
                    nodes.push(InlineNode::Text("`"));
                    remaining = &remaining[start + 1..];
                    if !remaining.is_empty() {
                        nodes.push(InlineNode::Text(remaining));
                    }
                    return nodes;
                }
            }
            '*' => {
                if remaining[start..].starts_with("**") {
                    remaining = &remaining[start + 2..];

                    if let Some(end) = remaining.find("**") {
                        nodes.push(InlineNode::Strong(parse_inline_nodes(&remaining[..end])));
                        remaining = &remaining[end + 2..];
                    } else {
                        nodes.push(InlineNode::Text("**"));
                        if !remaining.is_empty() {
                            nodes.push(InlineNode::Text(remaining));
                        }
                        return nodes;
                    }
                } else {
                    remaining = &remaining[start + 1..];

                    if let Some(end) = remaining.find('*') {
                        nodes.push(InlineNode::Emphasis(parse_inline_nodes(&remaining[..end])));
                        remaining = &remaining[end + 1..];
                    } else {
                        nodes.push(InlineNode::Text("*"));
                        if !remaining.is_empty() {
                            nodes.push(InlineNode::Text(remaining));
                        }
                        return nodes;
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
                        nodes.push(InlineNode::Link {
                            label: parse_inline_nodes(label),
                            url,
                        });
                        remaining = &after_label[url_end + 1..];
                    } else {
                        nodes.push(InlineNode::Text("["));
                        if !remaining.is_empty() {
                            nodes.push(InlineNode::Text(remaining));
                        }
                        return nodes;
                    }
                } else {
                    nodes.push(InlineNode::Text("["));
                    if !remaining.is_empty() {
                        nodes.push(InlineNode::Text(remaining));
                    }
                    return nodes;
                }
            }
            _ => unreachable!("only configured inline delimiters are returned"),
        }
    }

    if !remaining.is_empty() {
        nodes.push(InlineNode::Text(remaining));
    }

    nodes
}

fn render_inline_nodes(nodes: &[InlineNode<'_>]) -> String {
    let mut rendered = String::new();

    for node in nodes {
        match node {
            InlineNode::Text(text) => rendered.push_str(&escape_html(text)),
            InlineNode::Code(code) => {
                rendered.push_str("<code>");
                rendered.push_str(&escape_html(code));
                rendered.push_str("</code>");
            }
            InlineNode::Emphasis(children) => {
                rendered.push_str("<em>");
                rendered.push_str(&render_inline_nodes(children));
                rendered.push_str("</em>");
            }
            InlineNode::Strong(children) => {
                rendered.push_str("<strong>");
                rendered.push_str(&render_inline_nodes(children));
                rendered.push_str("</strong>");
            }
            InlineNode::Link { label, url } => {
                rendered.push_str("<a href=\"");
                rendered.push_str(&escape_html_attribute(url));
                rendered.push_str("\">");
                rendered.push_str(&render_inline_nodes(label));
                rendered.push_str("</a>");
            }
        }
    }

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

fn render_inline_code_from_tokens(text: &str) -> Option<(InlineNode<'_>, usize)> {
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
                return Some((InlineNode::Code(code), consumed_len));
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

pub(crate) fn escape_html(text: &str) -> String {
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

    #[test]
    fn parses_inline_nodes() {
        assert_eq!(
            parse_inline_nodes("Use `cargo` and **Rust** at [site](https://example.com)"),
            vec![
                InlineNode::Text("Use "),
                InlineNode::Code("cargo".to_string()),
                InlineNode::Text(" and "),
                InlineNode::Strong(vec![InlineNode::Text("Rust")]),
                InlineNode::Text(" at "),
                InlineNode::Link {
                    label: vec![InlineNode::Text("site")],
                    url: "https://example.com",
                },
            ]
        );
    }
}
