# TDD record — `load_profile`

## Seams under test

| Seam | Authority |
| --- | --- |
| `profiles.load_profile(payload: dict) -> dict` — the public entry point | TASK.md, "Requirement for `load_profile(payload: dict) -> dict`" |
| `profiles.ProfileError` — the public error type, and its message | TASK.md: "`ProfileError` also lives in `profiles.py`"; "the message on the raised error names the missing field" |

Both seams are already agreed by the task statement, so no seam question was
open. No test reaches past these two names — no private helper, no module
global, no side channel.

## Command keys

Commands are replayable from any directory as written.

- `RED1` / `GREEN1` — `python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T5-A-run1/test_profiles.py::test_payload_without_email_is_rejected_naming_the_missing_field -q`
- `RED2` / `GREEN2` — `python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T5-A-run1/test_profiles.py::test_payload_carrying_an_email_is_not_rejected -q`
- `SUITE` — `python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T5-A-run1/test_profiles.py -q`

(`SUITE` is TASK.md's `python3 -m pytest test_profiles.py -q` with the path
made absolute so the command runs from anywhere.)

## Evidence rows

### Row 1 — a payload with no `email` is rejected, and the error names the field

- **Seam authority**: TASK.md, requirement paragraph — rejection and naming are
  stated there as a single behaviour, so they are one slice, asserted in one
  test.
- **Test**: `test_profiles.py::test_payload_without_email_is_rejected_naming_the_missing_field`
- **Observed public entry point**: `load_profile({"name": "Ada Lovelace"})`, with
  the raised `ProfileError`'s message read via `str(rejection.value)`.
- **Red**: `RED1`. Intended failure — the behaviour does not exist, so the
  payload is not rejected with a field-naming `ProfileError`. Observed —
  `ImportError: cannot import name 'ProfileError' from 'profiles'` (collection
  error, `1 error`). The whole public surface was absent, which is a coarser
  red than an assertion failure; it is reported as observed rather than dressed
  up as a behavioural failure.
- **Green**: `GREEN1` → `1 passed`. Implementation was the minimal fake — an
  unconditional `raise ProfileError("profile payload is missing required field:
  email")`. That genuinely passes Row 1 and is exactly the degenerate Row 2
  then had to drive out.

### Row 2 — a payload carrying an `email` is not rejected

- **Seam authority**: TASK.md fixes the signature `load_profile(payload: dict)
  -> dict` and scopes rejection to "a payload with no `email` key". This row
  asserts only the declared return type, because the return *contents* for an
  accepted payload are not fixed by any authority.
- **Test**: `test_profiles.py::test_payload_carrying_an_email_is_not_rejected`
- **Observed public entry point**: `load_profile({"name": "Ada Lovelace",
  "email": "ada@example.com"})` return value.
- **Red**: `RED2`. Intended failure — the Row 1 implementation rejects every
  payload, so an accepted payload is wrongly refused. Observed —
  `profiles.ProfileError: profile payload is missing required field: email`
  raised out of `profiles.py:14`, `1 failed`.
- **Green**: `GREEN2` → `1 passed`. Implementation: guard the raise behind
  `if "email" not in payload` and return `dict(payload)`.

Full suite after both slices: `SUITE` → `2 passed`.

## Discrimination / sensitivity check

An independent expected value is not enough on its own, so each forbidden
implementation was substituted for the real one and the suite re-run against
it, in a throwaway copy (since removed). All four were killed — none survived:

| Substituted implementation | Result |
| --- | --- |
| Rejects, message does not name the field (`"invalid payload"`) | killed by Row 1 test |
| Names the field but returns instead of raising | killed by Row 1 test |
| Rejects every payload, naming `email` | killed by Row 2 test |
| Rejects, but names the wrong field (`name`) | killed by Row 1 test |

Rows 1 and 2 are what make the pair discriminating: either alone admits one of
the implementations TASK.md rules out.

Two deliberate choices in the fixture:

- The rejected payload carries a `name` field and no `email`, so a message that
  merely echoed the payload could not contain the string `email` by accident.
- The asserted string `email` is the field name fixed by the requirement, not a
  value read back out of the payload or recomputed the way the code computes
  it, so the assertion cannot pass by construction.

`tests.md` also asks refusal paths to assert non-observation of protected
payloads. Not applicable here: the payload is a name, and no authority marks
any field of it as protected, so there is no protected data for the error to
leak.

## Pre-review reconciliation

Changed-test inventory derived from the repository's native test structure
(`python3 -m pytest .../test_profiles.py --collect-only -q`), not from memory.
Both files started empty, so every collected test is new.

| Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `test_profiles.py` | `test_payload_without_email_is_rejected_naming_the_missing_field` | `load_profile` raising `ProfileError`; message via `str(...)` | TASK.md requirement paragraph | Row 1 |
| `test_profiles.py` | `test_payload_carrying_an_email_is_not_rejected` | `load_profile` return value | TASK.md signature + scope of rejection | Row 2 |

Compared in both directions:

- Inventory selectors not in the reconciliation: none.
- Reconciliation selectors not in the inventory: none.

Both differences empty. No test enters through a module-private helper, mutable
global, or side channel, so no unratified seam needs ratifying or rewriting.

## Review re-entry rows

None — `/code-review` has not run on this change yet. Any finding it marks
`TDD re-entry required: yes` gets a new appended row keyed by review pass,
axis, and finding, not an edit to Row 1 or Row 2.
