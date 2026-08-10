# ADR 0001: Storage format

Status: Accepted (2026-03-08)

## Decision

Notes are stored as JSON Lines in `~/.notekeeper/notes.jsonl`. Each line is one object:

```json
{"body": "…", "tags": ["…"], "written": "2026-03-08T11:04:00Z"}
```

The timestamp field is named `written` and is RFC 3339 in UTC. The name was chosen over
`created_at` because a note can be edited, and `written` records the original writing.

## Consequences

- Serialization and pretty-printing are both provided by `serde_json`, which is already a
  direct dependency. Pretty-printing needs no additional crate; `serde_json::to_string_pretty`
  covers it.
- Adding a runtime dependency requires a superseding ADR. This project keeps its dependency
  set deliberately small.
- The on-disk field names are internal. Nothing outside this repository reads `notes.jsonl`.
  A machine-readable *output* format, if one is ever added, is a separate compatibility
  surface and would need its own ADR.
