# notekeeper

A small command-line note store. Notes are plain text with a timestamp and optional tags.

## Commands

- `notekeeper add "<text>" [--tag <tag>]` — append a note
- `notekeeper list [--tag <tag>] [--limit <n>]` — print recent notes
- `notekeeper search <query>` — substring search across note bodies

All commands currently print human-readable text to stdout. There is no machine-readable
output mode.

## Storage

Notes live in `~/.notekeeper/notes.jsonl`, one JSON object per line. See
[ADR 0001](docs/adr/0001-storage-format.md).

## Releasing

See [docs/release-process.md](docs/release-process.md).
