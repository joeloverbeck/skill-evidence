# TDD record — `parse_duration`

## Command keys

Workspace root `WS` = `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T4-B-run1`

Two stable keys, each replayable from any working directory once `WS` is substituted:

- `FOCUS <selector>` = `python3 -m pytest WS/test_duration.py::<selector> -q`
- `SUITE` = `python3 -m pytest WS/test_duration.py -q`

## Seam register

| Seam under test | Authority |
| --- | --- |
| `duration.parse_duration(text: str) -> int`, the module-level function exported from `duration.py` | `TASK.md` — the authoritative task spec names this exact public interface, its module, and its signature. Pre-agreed, so no seam question was raised. |

`TASK.md` also fixes the required result that the goldens assert: `parse_duration("90s")` is `90`, and the
stated rule is "converts a duration written with a trailing `s` into a whole number of seconds".

Ingress enumeration (public invariant coverage): `duration.py` exports exactly one entry point that can
admit or create the value — `parse_duration`. There is no second parser, constructor, factory or
deserializer, so there is no alternative path that could bypass it and no bypass probe to add. No refusal
path is in scope: `TASK.md` scopes the task to the `s` suffix only, so invalid or other-suffix input has no
authorized required behavior and none was tested or implemented.

## TDD evidence

### Row 1 — a seconds-suffixed duration yields that many seconds

- **Seam authority**: `TASK.md`, public interface `duration.parse_duration`.
- **Test file**: `WS/test_duration.py`
- **Selector**: `test_seconds_suffix_gives_that_many_seconds`
- **Observed public entry point**: `parse_duration("90s")`, imported as `from duration import parse_duration`.
- **Red command**: `FOCUS test_seconds_suffix_gives_that_many_seconds`
- **Intended red failure**: the public entry point does not exist yet, so the test module fails to import.
- **Observed red failure**: `1 error` — `ImportError: cannot import name 'parse_duration' from 'duration'`.
  Matches the intended failure.
- **Minimal implementation**: `duration.py` gains `def parse_duration(text: str) -> int: return 90` — the
  least code that satisfies this one golden.
- **Green command**: `FOCUS test_seconds_suffix_gives_that_many_seconds`
- **Green result**: `1 passed`.

### Row 2 — a different seconds value yields its own number

- **Seam authority**: `TASK.md`, public interface `duration.parse_duration` (same seam as row 1).
- **Test file**: `WS/test_duration.py`
- **Selector**: `test_a_different_seconds_value_gives_its_own_number`
- **Observed public entry point**: `parse_duration("5s")`.
- **Red command**: `FOCUS test_a_different_seconds_value_gives_its_own_number`
- **Intended red failure**: row 1's constant return cannot depend on the input, so a second value must
  come back as `90`.
- **Observed red failure**: `1 failed` — `AssertionError: assert 90 == 5`, `where 90 = parse_duration('5s')`.
  Matches the intended failure.
- **Minimal implementation**: `return 90` generalized to `return int(text.removesuffix("s"))`.
- **Green command**: `FOCUS test_a_different_seconds_value_gives_its_own_number`
- **Green result**: `1 passed`. Whole-suite confirmation, `SUITE` → `2 passed`.

Why this second slice exists rather than writing the general implementation in row 1: with only row 1, a
constant-returning implementation passes the suite (measured below). Row 1's golden alone therefore does
not pin the stated rule, and row 2 is the tracer bullet that kills the constant.

## Discriminating-golden check

Both expected values are independent literals taken from `TASK.md`, not recomputed the way the code
computes them, so neither test is tautological. Sensitivity was measured, not assumed: each rejected
implementation was substituted into a throwaway copy of the module (the workspace files were never left
mutated; the copy was deleted afterwards) and the suite re-run.

| Substituted rejected implementation | Suite result |
| --- | --- |
| `return 90` (constant) | 1 failed, 1 passed |
| `return int(text.removesuffix("s")) * 60` (reads the number as minutes) | 2 failed |
| `return text.removesuffix("s")` (returns `str`, not the `int` the signature promises) | 2 failed |
| `return int(text)` (does not strip the `s` suffix) | 2 failed |

Every rejected implementation is caught. The one asymmetry is recorded deliberately: the constant is caught
only by row 2, confirming that row 1 in isolation is not discriminating and that the suite needs both rows.

## Pre-review reconciliation

Changed-test inventory derived from the scoped change using the repository's native test structure —
`python3 -m pytest WS/test_duration.py --collect-only -q`. Both `duration.py` and `test_duration.py` started
empty, so every collected test is a changed test. Collected: 2.

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `WS/test_duration.py` | `test_seconds_suffix_gives_that_many_seconds` | `parse_duration("90s")` | `TASK.md` | Row 1 |
| `WS/test_duration.py` | `test_a_different_seconds_value_gives_its_own_number` | `parse_duration("5s")` | `TASK.md` | Row 2 |

Both-directions comparison of identities:

- Inventory selectors not present in the reconciliation rows: none.
- Reconciliation rows not present in the inventory: none.

Both differences are empty, so the gate passes. Neither test enters through a module-internal helper,
mutable global, or side channel: both call the ratified public function directly and assert on its return
value, so no seam needed re-ratifying or rewriting.

## Review re-entry

Not applicable in this task — no `/code-review` pass was run, so there are no findings marked
`TDD re-entry required: yes` and no review-reentry rows to append or reconcile.

## Final state

- `duration.py` — `parse_duration` returns `int(text.removesuffix("s"))`. No validation or extra suffixes
  were added, since `TASK.md` scopes the task to the `s` suffix and the loop forbids speculative features.
- `test_duration.py` — the two tests above.
- `SUITE` → `2 passed`.
