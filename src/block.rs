#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BlockNode<'a> {
    Paragraph(&'a str),
    Heading { level: u8, text: &'a str },
    UnorderedList(Vec<&'a str>),
    OrderedList(Vec<&'a str>),
    BlockQuote(Vec<&'a str>),
    CodeBlock(Vec<&'a str>),
}

pub(crate) fn parse_blocks(markdown: &str) -> Vec<BlockNode<'_>> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
