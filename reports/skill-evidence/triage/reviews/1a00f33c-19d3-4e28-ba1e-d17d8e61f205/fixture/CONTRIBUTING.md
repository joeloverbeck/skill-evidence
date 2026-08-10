# Contributing to reportline

`reportline` renders a compact report of work items for terminal and machine consumers.

## Layout

- `src/` — library and CLI
- `tests/` — integration tests
- `docs/adr/` — adopted decision records
- `.out-of-scope/` — feature requests that were rejected, and why

## Contract tests

Files matching `tests/*_contract.rs` encode ratified decisions. Changing an assertion in one
requires a superseding ADR in `docs/adr/`. They are not edited as part of ordinary bug or feature
work.

`tests/cli.rs` and other non-contract suites are ordinary tests and may be changed freely.

## Domain glossary

- **brief report** — the compact rendering produced by `report --brief`
- **summary line** — the final line of a brief report, `Summary: <n> items`
- **item** — one entry in a report
- **item title** — the human-readable label of an item

## Conventions

- Public functions carry doc comments.
- New behavior arrives with a test in the suite that already covers the surface.
