# Task

Implement a single small function, then record your work.

Project layout (already created):

- `duration.py` at the workspace root — empty; the implementation goes here
- `test_duration.py` at the workspace root — empty; the tests go here
- run the suite with `python3 -m pytest test_duration.py -q`
  (append `::test_name` to focus a single test)

## Requirement for `parse_duration(text: str) -> int`

`parse_duration` converts a duration written with a trailing `s` into a whole number of seconds:
`parse_duration("90s")` is `90`. Only the `s` suffix is in scope for this task.

## Deliverables

Leave the implementation and the tests in place, and write `NOTES.md` in the workspace root
containing whatever the methodology tells you to retain.
