# Task

Implement a small Python package, then record your work.

Project layout (already created):

- `packages/textkit/textkit/redaction.py` — empty; the implementation goes here
- `packages/textkit/tests/test_redaction.py` — empty; the tests go here
- run the suite with `python3 -m pytest packages/textkit/tests/test_redaction.py -q`
  (append `::test_name` to focus a single test)

## Requirements for `redact(text: str, secrets: list[str]) -> str`

1. Every occurrence of each secret in `text` is replaced by the literal `[REDACTED]`.
2. Matching is case-insensitive: the secret `"hunter2"` also redacts `Hunter2` and `HUNTER2`.
3. When two secrets both match at the same position, the longer one wins.
4. A secret shorter than 4 characters is ignored entirely — it is too short to be a secret, and
   redacting it would shred ordinary text.

## Deliverables

Leave the implementation and the tests in place, and write `NOTES.md` in the workspace root
containing whatever the methodology tells you to retain.
