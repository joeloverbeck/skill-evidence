# TDD evidence

## Agreed seam

- Public seam: `numbers.clamp(value, minimum, maximum)`.
- Authority: `TASK.md`, which authorizes the below-minimum behavior: a value below
  the minimum returns the minimum.

## Cycle: below-minimum value

- Seam authority: `TASK.md`, lines 3-5.
- Test file and exact selector:
  `test_numbers.py::ExistingTests.test_clamp_returns_minimum_when_value_is_below_minimum`.
- Observed public entry point: `numbers.clamp(value, minimum, maximum)`.
- Working directory for both commands:
  `/tmp/tdd-evolution-rev-3e39-t3-core-r2/work`.
- Red command: `python3 -m unittest test_numbers.ExistingTests.test_clamp_returns_minimum_when_value_is_below_minimum -v`.
- Intended red: the public call `clamp(-2, 3, 9)` cannot return the authorized
  minimum because the supplied implementation raises `NotImplementedError`.
- Observed red: exit 1; the exact selector errored at the public call with
  `NotImplementedError`; 1 test ran and the command reported `FAILED (errors=1)`.
- Minimal production change: return `minimum` when `value < minimum`, retaining
  the existing refusal for all behavior not authorized by this slice.
- Green command: `python3 -m unittest test_numbers.ExistingTests.test_clamp_returns_minimum_when_value_is_below_minimum -v`.
- Green result: exit 0; the exact selector passed; 1 test ran and the command
  reported `OK`.

## Reconciliation

| Change kind | File and exact identity | Public entry point | Seam authority | Disposition |
| --- | --- | --- | --- | --- |
| Added | `test_numbers.py::ExistingTests.test_clamp_returns_minimum_when_value_is_below_minimum` | `numbers.clamp(value, minimum, maximum)` | `TASK.md`, lines 3-5 | TDD custody: cycle `below-minimum value` |

- Test inventory minus reconciliation: empty.
- Reconciliation minus test inventory: empty.
- TDD evidence identities minus TDD-custody reconciliation identities: empty.
- TDD-custody reconciliation identities minus TDD evidence identities: empty.
