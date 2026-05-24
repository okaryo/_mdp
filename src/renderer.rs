use crate::block::BlockNode;
use crate::inline::{InlineNode, parse_inline_nodes};

pub(crate) fn render_blocks(blocks: &[BlockNode<'_>]) -> String {
    blocks
        .iter()
        .map(render_block)
        .collect::<Vec<_>>()
        .join("\n")
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
    render_inline_nodes(&parse_inline_nodes(text))
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
    fn renders_block_nodes_to_html() {
        let blocks = vec![
            BlockNode::Heading {
                level: 1,
                text: "Title",
            },
            BlockNode::UnorderedList(vec!["Apples", "Oranges"]),
        ];

        assert_eq!(
            render_blocks(&blocks),
            "<h1>Title</h1>\n<ul><li>Apples</li><li>Oranges</li></ul>"
        );
    }

    #[test]
    fn renders_inline_nodes_to_html() {
        let nodes = vec![
            InlineNode::Text("Use "),
            InlineNode::Code("cargo test".to_string()),
            InlineNode::Text(" and "),
            InlineNode::Strong(vec![InlineNode::Text("learn")]),
        ];

        assert_eq!(
            render_inline_nodes(&nodes),
            "Use <code>cargo test</code> and <strong>learn</strong>"
        );
    }

    #[test]
    fn escapes_text_and_attributes() {
        let nodes = vec![InlineNode::Link {
            label: vec![InlineNode::Text("<Rust>")],
            url: "https://example.com/?q=\"rust\"&page=1",
        }];

        assert_eq!(
            render_inline_nodes(&nodes),
            "<a href=\"https://example.com/?q=&quot;rust&quot;&amp;page=1\">&lt;Rust&gt;</a>"
        );
    }
}
