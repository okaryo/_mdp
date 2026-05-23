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

## Initial Markdown Scope

The first version will focus on common Markdown elements:

- Headings: `#`, `##`, `###`
- Paragraphs
- Unordered lists: `- item`
- Ordered lists: `1. item`
- Block quotes: `> quote`
- Fenced code blocks
- Inline code
- Emphasis: `*em*`
- Strong emphasis: `**strong**`
- Links: `[text](url)`

Unsupported syntax should be handled deliberately and documented as the project
evolves.

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
