# _mdp

`_mdp` is a learning project for building a small Markdown parser in Rust.

The goal is not to create a complete CommonMark-compatible implementation.
Instead, this project focuses on learning Rust string handling, parser design,
simple syntax trees, testing, and HTML rendering by implementing a practical
subset of Markdown step by step.

## Learning Goals

- Practice Rust string processing with `&str`, `String`, `char`, and iterators.
- Learn how block-level Markdown parsing works.
- Learn how inline Markdown parsing works.
- Introduce a simple AST or intermediate representation when it becomes useful.
- Build confidence with small tests that define the supported behavior.
- Convert Markdown input into HTML output safely and predictably.
- Practice refactoring after the design pressure becomes visible.

## Supported Markdown Subset

The current parser supports a deliberately small Markdown subset:

- Headings: `#`, `##`, `###`
- Paragraphs
- Unordered lists: `- item`
- Ordered lists: `1. item`
- Block quotes: `> quote`
- Fenced code blocks with triple backticks
- Inline code: `` `code` ``
- Emphasis: `*em*`
- Strong emphasis: `**strong**`
- Links: `[text](url)`

HTML-sensitive text is escaped during rendering. Link URLs are escaped for HTML
attribute context.

## Known Limitations

This parser is intentionally incomplete. Current limitations include:

- CommonMark compatibility is not a goal.
- Nested lists are not supported; lists are depth 1 only.
- Fenced code blocks must use a line containing exactly triple backticks.
- Language identifiers on code fences are not supported.
- Unterminated fenced code blocks are treated as code until the end of input.
- Blank-line paragraph grouping is not implemented yet.
- Inline parsing is intentionally simple and does not cover all delimiter edge
  cases.
- Links do not support nested brackets or URLs containing `)`.

## Architecture

The implementation is split into small learning-oriented modules:

- `src/lib.rs`: public `parse(markdown: &str) -> String` entry point
- `src/block.rs`: block-level parsing and `BlockNode`
- `src/inline.rs`: inline tokenization, inline parsing, and `InlineNode`
- `src/renderer.rs`: HTML rendering and HTML escaping

The high-level flow is:

```text
Markdown text
  -> block parser
  -> BlockNode AST
  -> inline parser where needed
  -> InlineNode AST
  -> HTML renderer
  -> HTML string
```

## CLI Usage

Parse Markdown from a file:

```bash
cargo run -- input.md
```

Parse Markdown from standard input:

```bash
echo '# Hello' | cargo run
```

## Learning Style

This project should move in very small steps. Each step should define:

- What we want to learn
- The smallest feature to implement
- The Rust concept being practiced
- One or more tests that describe the behavior
- A short understanding check before moving on

Early implementation can be simple, such as a `parse(markdown: &str) -> String`
function. More structured designs, such as block parsers, inline parsers, and
AST nodes, should be introduced only when they help explain or simplify the next
step.
