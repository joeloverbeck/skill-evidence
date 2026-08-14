# TDD evidence

## Cycle: value below the minimum

- Seam authority: `../TASK.md`, which authorizes public `clamp(value, minimum, maximum)` behavior where a value below the minimum returns the minimum.
- Test: `test_numbers.py` — `test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.
- Observed public entry point: `numbers.clamp(value, minimum, maximum)`.
- Red command: `python3 -m unittest test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.
- Intended failure: the public `clamp(2, 5, 9)` call does not yet return the authorized lower bound.
- Observed failure: exit 1; the public call raised `NotImplementedError` from `numbers.clamp`.
- Minimal production change: return `minimum` when `value < minimum`.
- Green command: `python3 -m unittest test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.
- Green result: exit 0; 1 test ran and passed.

## Selector reconciliation

| Change kind | File and exact identity | Public entry point | Seam authority | Disposition |
| --- | --- | --- | --- | --- |
| Added | `test_numbers.py` — `test_numbers.ClampTests.test_value_below_minimum_returns_minimum` | `numbers.clamp(value, minimum, maximum)` | `../TASK.md` | TDD custody: cycle `value below the minimum` |

- Changed-selector inventory minus reconciliation: empty.
- Reconciliation minus changed-selector inventory: empty.
- TDD evidence identities minus TDD-custody reconciliation: empty.
- TDD-custody reconciliation minus TDD evidence identities: empty.
