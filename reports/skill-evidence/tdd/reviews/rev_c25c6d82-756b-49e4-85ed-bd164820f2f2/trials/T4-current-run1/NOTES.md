# TDD record — `parse_duration`

All commands are run from the workspace root (the directory holding `duration.py`).
Environment: Python 3.14.4, pytest 9.0.2.

## Seam agreement

| Item | Value |
| --- | --- |
| Seam under test | `parse_duration(text: str) -> int`, imported from the `duration` module |
| Authority | `TASK.md` § "Requirement for `parse_duration(text: str) -> int`" — fixes the signature, the module, the worked example `parse_duration("90s") == 90`, and the scope limit ("Only the `s` suffix is in scope") |
| Status | Pre-agreed. The authority names the public function and its invocation, so no seam question was outstanding and none was invented. |

No `CONTEXT.md` and no ADRs exist in this workspace, so domain vocabulary was taken from `TASK.md` itself ("duration", "seconds", "suffix").

`mocking.md` does not apply: the function touches no system boundary — no I/O, clock, randomness, or
network — so nothing is mocked and no dependency is injected.

## TDD evidence rows

### Cycle 1 — the worked example from the authority

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` requirement section (worked example `parse_duration("90s")` is `90`) |
| Test file | `test_duration.py` |
| Selector | `test_duration.py::test_parses_the_worked_example_90s_as_90_seconds` |
| Observed public entry point | `parse_duration("90s")` via `from duration import parse_duration` |
| Red command | `python3 -m pytest "test_duration.py::test_parses_the_worked_example_90s_as_90_seconds" -q` |
| Intended failure | The ratified public function does not exist yet, so the import of `parse_duration` fails. |
| Observed failure | `ImportError: cannot import name 'parse_duration' from 'duration'` — 1 error, collection failed. Matches the intended failure. |
| Minimal implementation | `def parse_duration(text: str) -> int: return 90` — a constant, deliberately the least code that passes this one test (see note below). |
| Green command | `python3 -m pytest "test_duration.py::test_parses_the_worked_example_90s_as_90_seconds" -q` |
| Green result | `1 passed` |

Note on the constant: the loop's rule is "only enough code to pass it. Don't anticipate future
tests." Generalising here would have made Cycle 2 green on arrival and there would have been no red
to observe. Triangulating in Cycle 2 is what forces the parse, and its red below is genuine.

### Cycle 2 — generalisation beyond the worked example

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` requirement section — the general rule "converts a duration written with a trailing `s` into a whole number of seconds" ratifies seconds values other than the worked example |
| Test file | `test_duration.py` |
| Selector | `test_duration.py::test_parses_a_different_seconds_duration_than_the_worked_example` |
| Observed public entry point | `parse_duration("5s")` via `from duration import parse_duration` |
| Red command | `python3 -m pytest "test_duration.py::test_parses_a_different_seconds_duration_than_the_worked_example" -q` |
| Intended failure | Cycle 1's constant returns `90` for every input, so a second, different duration must disagree. |
| Observed failure | `AssertionError: assert 90 == 5` / `where 90 = parse_duration('5s')` — 1 failed. Matches the intended failure. |
| Minimal implementation | `return int(text.removesuffix("s"))` |
| Green command | `python3 -m pytest "test_duration.py::test_parses_a_different_seconds_duration_than_the_worked_example" -q` |
| Green result | `1 passed` |

Full suite after Cycle 2 — `python3 -m pytest test_duration.py -q` → `2 passed`.

`removesuffix("s")` was chosen over `text[:-1]` deliberately: both pass both tests, but `text[:-1]`
would silently drop a significant digit on an input with no suffix, whereas `removesuffix` states
the intent the authority actually wrote. No behaviour beyond the two ratified tests was added.

## Coverage and sensitivity checks

**Public invariant coverage.** `tests.md` requires enumerating every exported ingress path that can
admit the value. Enumerated, not assumed:
`python3 -c "import duration; print([n for n in dir(duration) if not n.startswith('_')])"`
→ `['parse_duration']`. There is exactly one public ingress path — no second parser, constructor,
factory, or deserializer exists to bypass it — so the single-path tests cover the invariant and no
bypass probe is applicable.

**Discriminating goldens.** Both expected values are independent literals from the authority
(`90`, and `5` under the general rule); neither recomputes the implementation, so neither is
tautological. Sensitivity was observed by substituting rejected implementations into a throwaway
copy, not argued on paper:

| Rejected implementation | Observed result | Killed by |
| --- | --- | --- |
| `return 90` (constant) | `assert 90 == 5` | Cycle 2 (this is Cycle 2's red, observed above) |
| `return int(text[0])` (first digit / truncation) | 1 failed, 1 passed | Cycle 1 — `"90s"` is multi-digit, so it discriminates; note Cycle 2's `"5s"` alone would **not** have caught this |
| `return len(text)` | 2 failed | both cycles |

Replayable probe (self-contained; creates and removes its own scratch copy):

```
mkdir -p .sensitivity && cp test_duration.py .sensitivity/ \
  && printf 'def parse_duration(text: str) -> int:\n    return int(text[0])\n' > .sensitivity/duration.py \
  && python3 -m pytest .sensitivity/test_duration.py -q; rm -rf .sensitivity
```

The two goldens are load-bearing in different directions — that is why both were kept rather than
collapsed into one.

## Pre-review reconciliation

Changed-test inventory derived from the repository's native test structure via
`python3 -m pytest test_duration.py --collect-only -q`. Both `duration.py` and `test_duration.py`
started empty per `TASK.md`, so every collected test is a changed test.

Inventory:

1. `test_duration.py::test_parses_the_worked_example_90s_as_90_seconds`
2. `test_duration.py::test_parses_a_different_seconds_duration_than_the_worked_example`

Reconciliation rows, one per selector:

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `test_duration.py` | `test_parses_the_worked_example_90s_as_90_seconds` | `parse_duration("90s")` | `TASK.md` requirement — worked example | Cycle 1 |
| `test_duration.py` | `test_parses_a_different_seconds_duration_than_the_worked_example` | `parse_duration("5s")` | `TASK.md` requirement — general rule | Cycle 2 |

Both-direction comparison: inventory \ reconciliation = ∅; reconciliation \ inventory = ∅. Gate passes.

Coverage check before reconciliation: every row enters through the ratified public function
`parse_duration`. No test reaches a module internal, helper, mutable global, or side channel, and no
private-invariant test stands in for a public-behaviour claim. So no seam needed ratifying or
rewriting.

## Scope boundary — behaviours deliberately left untested

The authority fixes only the trailing-`s` case. These behaviours are **unratified**, so per the
seam rule no test was written at them and no implementation was speculatively added. They are the
questions to put to the user before any further slice:

- Other unit suffixes (`"5m"`, `"2h"`) — explicitly out of scope for this task.
- Missing suffix (`"90"`), empty string, whitespace (`" 90s "`), mixed case (`"90S"`).
- Non-numeric or malformed input (`"abcs"`) — whether to raise, and with what error type/message.
- Negative, zero, or fractional values (`"-5s"`, `"1.5s"`) and what "whole number of seconds" requires of them.

Current incidental behaviour on those inputs is whatever `int()` does; it is not tested and must
not be relied on as a contract.

## Review re-entry

No `/code-review` pass has run against this change, so there are no review-re-entry rows and no
`Findings fixed during review` ledger to reconcile. If a review runs, each finding marked
`TDD re-entry required: yes` gets its own appended row keyed by pass/axis/finding — the two rows
above are closed and must not be rewritten to absorb a later finding.
