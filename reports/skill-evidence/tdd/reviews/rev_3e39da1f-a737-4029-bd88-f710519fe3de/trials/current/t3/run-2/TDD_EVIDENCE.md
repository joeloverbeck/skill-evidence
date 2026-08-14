# TDD evidence

## Agreed seam

- Seam: the public `numbers.clamp(value, minimum, maximum)` function.
- Authority: `TASK.md` explicitly authorizes one behavior at this public entry point: a value below the minimum returns the minimum.

## Cycle 1: value below minimum

| Field | Evidence |
| --- | --- |
| Seam authority | `TASK.md`: `clamp(value, minimum, maximum)` returns `minimum` when `value` is below it. |
| Test | `test_numbers.py`; selector `test_numbers.ClampTests.test_value_below_minimum_returns_minimum` |
| Observed public entry point | `numbers.clamp(-7, 3, 11)` |
| Golden and sensitivity | Required literal `3`; explicitly differs from rejected pass-through result `-7`. |
| Red command | `python3 -m unittest test_numbers.ClampTests.test_value_below_minimum_returns_minimum` (run from `work/`) |
| Intended failure | The unimplemented public function does not return the authorized minimum. |
| Observed failure | Exit 1; the focused test errored because `numbers.clamp` raised `NotImplementedError`. |
| Green command | `python3 -m unittest test_numbers.ClampTests.test_value_below_minimum_returns_minimum` (run from `work/`) |
| Green result | Exit 0; 1 focused test ran and passed (`OK`). |

## Changed-test reconciliation

| Changed test selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- |
| `test_numbers.ClampTests.test_value_below_minimum_returns_minimum` | `numbers.clamp(-7, 3, 11)` | `TASK.md`: below-minimum values return `minimum`. | Cycle 1: value below minimum |

- Changed-test inventory minus reconciliation identities: empty.
- Reconciliation identities minus changed-test inventory: empty.

## Final verification

- Command: `python3 -m unittest -v` (run from `work/`).
- Result: exit 0; 2 tests ran and passed (`OK`).
