# AGENTS.md

## Project Context

This is a learning project for building a small Markdown parser in Rust. The
main purpose is to learn Rust, parser design, tests, and HTML rendering by
implementing a practical subset of Markdown in small steps.

Completing a perfect Markdown parser is not the goal. Prefer learning value,
clear reasoning, and incremental progress over broad feature coverage.

## Collaboration Rules

- Move in small steps.
- Before implementing a new feature, explain the goal, the smallest useful
  change, and the Rust concept being practiced.
- After each step, summarize what changed and include an understanding check.
- Do not jump ahead to larger architecture unless the current code makes the
  need visible.
- Prefer simple code first, then refactor deliberately.
- When the user asks for investigation or research, investigate and summarize
  the findings without making code changes.
- Do not make unsolicited code changes when the user is asking a conceptual
  question.

## TODO.md Maintenance

- Treat `TODO.md` as a living roadmap, not a fixed plan.
- Update `TODO.md` whenever the project direction, supported scope, learning
  goals, or implementation decisions change.
- When a task is completed, mark the corresponding checklist item as checked.
- When a task is split, removed, deferred, or reworded, update `TODO.md` so it
  reflects the current plan.
- Keep `TODO.md` useful for both project progress and learning review.

## Implementation Guidelines

- Use Rust idioms, but keep explanations beginner-friendly.
- Prefer readable code over clever code.
- Start with direct parsing functions before introducing larger abstractions.
- Introduce AST nodes, tokenization, or parser modules only when they help the
  current learning step.
- Keep supported Markdown behavior explicit in tests and documentation.
- Treat unsupported Markdown syntax as acceptable if it is documented.

## Testing Guidelines

- Add focused tests for each supported Markdown behavior.
- Prefer small input and expected HTML pairs.
- Add regression tests when fixing parser behavior.
- Keep tests understandable enough to serve as learning examples.
