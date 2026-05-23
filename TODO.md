# TODO

This roadmap is both a task list and a learning checklist. Keep each step small,
verify understanding before moving on, and prefer clear learning value over full
Markdown compatibility.

## Phase 0: Project Setup

- [x] Create a minimal Rust project structure.
- [x] Add a basic parser entry point.
- [x] Add a basic test structure.
- [x] Decide the first public API shape.
  - Decision: start with `parse(markdown: &str) -> String`.

## Phase 1: First HTML Output

- [x] Convert plain text into a paragraph.
  - Learning: basic function design, `&str`, `String`, and `format!`.
- [x] Escape HTML-sensitive characters in text.
  - Learning: character iteration and safe output.
- [x] Add tests for paragraph rendering and escaping.

## Phase 2: Simple Block Parsing

- [x] Parse level 1 headings with `# heading`.
  - Learning: `strip_prefix`, branching, and reusing escaping logic.
- [x] Parse level 2 and level 3 headings.
  - Learning: small branching logic and checking longer prefixes first.
- [x] Parse multiple lines into separate blocks.
  - Learning: line iteration with `lines()`.
- [x] Add tests for headings, paragraphs, and mixed input.

## Phase 3: Lists and Quotes

- [ ] Parse unordered list items beginning with `- `.
  - Learning: collecting related lines into one HTML element.
- [ ] Parse ordered list items beginning with `1. `, `2. `, etc.
  - Learning: simple prefix detection and parser limitations.
- [ ] Parse block quotes beginning with `> `.
  - Learning: block-level syntax with stripped prefixes.
- [ ] Add tests for lists, quotes, and surrounding paragraphs.

## Phase 4: Code Blocks

- [ ] Parse fenced code blocks using triple backticks.
  - Learning: parser state and multi-line blocks.
- [ ] Preserve code block content without inline parsing.
  - Learning: separating raw text handling from normal Markdown handling.
- [ ] Add tests for code blocks and unterminated fences.

## Phase 5: Inline Parsing

- [ ] Parse inline code using single backticks.
  - Learning: scanning within a line.
- [ ] Parse emphasis with `*text*`.
  - Learning: delimiter matching and edge cases.
- [ ] Parse strong emphasis with `**text**`.
  - Learning: precedence between similar delimiters.
- [ ] Parse links with `[text](url)`.
  - Learning: nested scanning and validation.
- [ ] Add focused tests for each inline feature.

## Phase 6: Tokenizer Basics

- [ ] Introduce a small tokenizer for inline syntax.
  - Learning: turning raw text into simple tokens.
- [ ] Define token types for plain text, backticks, stars, brackets, parentheses, and end of input.
  - Learning: enums and explicit parser input.
- [ ] Rewrite one inline feature to read from tokens instead of raw string scanning.
  - Learning: the relationship between tokenizer and parser.
- [ ] Add tests for tokenizer output.

## Phase 7: AST Basics

- [ ] Introduce simple block node types when string-only parsing becomes awkward.
  - Learning: enums and data modeling in Rust.
- [ ] Introduce simple inline node types for text, code, emphasis, strong emphasis, and links.
  - Learning: nested data structures.
- [ ] Separate block parsing from inline parsing.
  - Learning: module boundaries and parser responsibilities.
- [ ] Refactor tests around the new structure.

## Phase 8: HTML Renderer Basics

- [ ] Introduce a renderer that converts AST nodes into HTML.
  - Learning: separating parsing from output generation.
- [ ] Move HTML escaping into the renderer.
  - Learning: output safety and responsibility boundaries.
- [ ] Add renderer-specific tests.
  - Learning: testing parser and renderer behavior separately.
- [ ] Keep an integration test for Markdown input to HTML output.
  - Learning: end-to-end behavior checks.

## Phase 9: Review and Stretch Goals

- [ ] Document the supported Markdown subset.
- [ ] Add a small CLI if it helps practice input and output handling.
- [ ] Compare behavior against a few CommonMark examples without aiming for full compatibility.
- [ ] Review the implementation and simplify names, modules, and tests.

## Per-Step Template

Use this format before starting a new learning step:

```text
Step:
Goal:
Smallest feature:
Rust concept:
Test case:
Understanding check:
```
