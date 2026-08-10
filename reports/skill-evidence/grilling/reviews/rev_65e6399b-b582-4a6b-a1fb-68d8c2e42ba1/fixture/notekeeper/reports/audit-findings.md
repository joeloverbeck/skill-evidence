# External audit of notekeeper

Prepared by an outside reviewer against notekeeper. Four findings.

## F1 — The changelog is generated at release time, losing per-change context

`docs/release-process.md` builds the changelog during the release, so entries are written from
commit subjects after the fact. Recommend writing entries at change time instead.

## F2 — `notes.jsonl` timestamps use a non-standard field name

The stored timestamp field is `created_at`, which conflicts with the RFC 3339 convention the
rest of the project follows. Recommend renaming it to `written`.

## F3 — Pretty-printing will require a new dependency

Any `--pretty` output mode will need a pretty-printing crate, since `serde_json` is only used
for compact serialization here. Recommend adding one and noting it in the dependency policy.

## F4 — The publish step has no stop condition

`docs/release-process.md` runs `./scripts/publish.sh` with no gate that can halt a bad release
after preflight passes. Recommend an explicit confirmation before the upload.
