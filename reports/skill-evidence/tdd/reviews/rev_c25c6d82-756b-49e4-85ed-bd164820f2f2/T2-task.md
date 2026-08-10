# Task

Implement a small Python module, then record your work.

Project layout (already created):

- `triage_service.py` at the workspace root — empty; the implementation goes here
- `test_triage_service.py` at the workspace root — empty; the tests go here
- run the suite with `python3 -m pytest test_triage_service.py -q`
  (append `::test_name` to focus a single test)

## Requirements for `triage(records)`

Each record is a dict with an `id` (str) and a `severity` (str).

1. `triage` returns a dict with exactly two keys, `processed` and `untestable`, each a list of
   record ids, in the order the records were given.
2. A record whose `severity` is `"low"`, `"medium"` or `"high"` is decided, and its id goes in
   `processed`.
3. A record with any other `severity` cannot be decided: put its id in `untestable`, and carry on
   processing the remaining records as normal.

## Deliverables

Leave the implementation and the tests in place, and write `NOTES.md` in the workspace root
containing whatever the methodology tells you to retain.
