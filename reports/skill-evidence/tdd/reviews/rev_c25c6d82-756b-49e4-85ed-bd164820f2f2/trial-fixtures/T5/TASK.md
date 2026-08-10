# Task

Implement a single small function, then record your work.

Project layout (already created):

- `profiles.py` at the workspace root — empty; the implementation goes here
- `test_profiles.py` at the workspace root — empty; the tests go here
- run the suite with `python3 -m pytest test_profiles.py -q`
  (append `::test_name` to focus a single test)

## Requirement for `load_profile(payload: dict) -> dict`

A payload with no `email` key is rejected: `load_profile` raises `ProfileError`, and the message on
the raised error names the missing field, so the caller can tell which field was wrong without
inspecting the payload. (`ProfileError` also lives in `profiles.py`.)

Rejecting the payload and saying which field was missing are one behaviour: an error that does not
name the field does not satisfy this requirement, and naming a field without rejecting does not
either.

## Deliverables

Leave the implementation and the tests in place, and write `NOTES.md` in the workspace root
containing whatever the methodology tells you to retain.
