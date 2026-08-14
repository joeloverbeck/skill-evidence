# TDD evidence

## Cycle 1: a value below the minimum returns the minimum

- Seam authority: `TASK.md` lines 3-5 authorize public `clamp(value, minimum, maximum)` behavior where a value below the minimum returns the minimum.
- Test: `test_numbers.py` — `test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.
- Observed public entry point: imported public function `numbers.clamp(value, minimum, maximum)`.
- Red command: `python3 -m unittest test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.
- Intended failure: the public call has no below-minimum behavior and raises `NotImplementedError` instead of returning the minimum.
- Observed failure: exit 1; the named test errored at `clamp(-5, 0, 10)` with `NotImplementedError`; 1 test ran and the result was `FAILED (errors=1)`.
- Minimal implementation: return `minimum` when `value < minimum`; leave all unauthorized cases unspecified.
- Green command: `python3 -m unittest test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.
- Green result: exit 0; 1 test ran and the result was `OK`.

## Changed-test reconciliation

Changed-test inventory derived from the scoped diff:

- `test_numbers.py` — `test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.

Reconciliation:

- Test: `test_numbers.py` — `test_numbers.ClampTests.test_value_below_minimum_returns_minimum`.
- Observed public entry point: imported public function `numbers.clamp(value, minimum, maximum)`.
- Seam authority: `TASK.md` lines 3-5.
- TDD evidence row: Cycle 1.

Inventory identities absent from reconciliation: none.

Reconciliation identities absent from inventory: none.
