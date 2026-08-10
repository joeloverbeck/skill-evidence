# Contributing

- Keep the CLI surface small. Every new flag needs a line in the README's command list.
- Public output formats are a compatibility surface: once a format ships, downstream scripts
  parse it. Changing a shipped format needs an ADR.
- Run `cargo fmt` and `cargo clippy` before opening a pull request.
- The changelog is maintained by hand. Every user-visible change gets an entry under
  `## Unreleased` at the time of the change, not at release time.
