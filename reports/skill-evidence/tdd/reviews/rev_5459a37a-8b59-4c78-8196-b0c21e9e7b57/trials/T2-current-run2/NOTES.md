# NOTES — duration formatter under the `tdd` skill

Deliverables: `duration.py` (implementation), `test_duration.py` (tests). Final state: 6 tests, all passing.

## Decisions made without a user

No user was available, so the skill's "ask the user" branches were resolved by me and are recorded here:

- **Seam and its authority.** The skill requires every seam to be pre-agreed before a test is written.
  `TASK.md` names the public module (`duration.py`), the public function (`format_duration(ms) -> str`),
  and six acceptance behaviors with exact expected strings. I treated `TASK.md` as the ratifying
  authority and the imported `format_duration` as the single agreed seam. No test touches anything else;
  there are no private helpers to bypass it.
- **Ingress-path enumeration** (`tests.md`, "Public invariant coverage"). `duration.py` exports exactly one
  entry point that can admit a duration value — `format_duration`. There is no separate parser,
  constructor, factory, or deserializer that could bypass the negative-input check, so the single refusal
  test at `format_duration(-1)` covers every ingress path. Nothing to add.
- **Refusal non-observation** (`tests.md`). The refusal path returns no value and the raised `ValueError`
  carries only the caller's own input; there is no protected payload to leak, so no non-observation
  assertion was warranted.
- **Scope.** I implemented exactly the six specified behaviors and wrote no speculative tests
  ("don't anticipate future tests"). Behaviors the spec does not fix are listed under Open questions.

## Command key

- `K-focused <selector>` = `python3 -m pytest "test_duration.py::<selector>" -q`
- `K-suite` = `python3 -m pytest -q` (run from the working directory; `python3 -m pytest test_duration.py -q`
  is the identical run for cycles 1–5, before any second test file could exist)

## TDD evidence rows

Every row: seam authority = `TASK.md` acceptance behaviors (the ratifying document for this task);
observed public entry point = `format_duration` imported from module `duration` in `test_duration.py`;
test file = `test_duration.py`.

### Row 1 — zero renders as bare `0s`
- **Authority**: `TASK.md` behavior 1.
- **Selector**: `test_zero_formats_as_bare_zero_seconds`
- **Red**: `K-focused test_zero_formats_as_bare_zero_seconds`. Intended failure: no public entry point
  exists yet. Observed: `ModuleNotFoundError: No module named 'duration'` (collection error, 1 error).
  Intended == observed.
- **Green**: created `duration.py` with `format_duration` returning `"0s"`. `K-focused ...` → 1 passed.

### Row 2 — sub-second renders with two decimals
- **Authority**: `TASK.md` behavior 2 (`450` → `"0.45s"`).
- **Selector**: `test_sub_second_formats_with_two_decimal_places`
- **Red**: `K-focused test_sub_second_formats_with_two_decimal_places`. Intended failure: the constant
  `"0s"` cannot express sub-second precision. Observed: `AssertionError`, expected `0.45s`, got `0s`.
  Intended == observed.
- **Green**: added the `0 < ms < 1000` branch returning `f"{ms / 1000:.2f}s"`.
  `K-focused ...` → 1 passed; `K-suite` → 2 passed.
- **Discriminating golden**: the observed red output was literally `"0s"` — i.e. this golden was observed
  to reject the truncate-to-whole-seconds implementation. It also rejects `"450ms"` and any
  round-to-nearest-second rendering.

### Row 3 — whole seconds under a minute render without decimals
- **Authority**: `TASK.md` behavior 3 (`9000` → `"9s"`).
- **Selector**: `test_whole_seconds_under_a_minute_format_without_decimals`
- **Red**: `K-focused test_whole_seconds_under_a_minute_format_without_decimals`. Intended failure: the
  fallback still returns the `"0s"` constant for every input at or above a second. Observed:
  `AssertionError`, expected `9s`, got `0s`. Intended == observed.
- **Green**: fallback became `f"{ms // 1000}s"`. `K-focused ...` → 1 passed; `K-suite` → 3 passed.
- **Discriminating golden**: rejects a `"9.00s"` always-two-decimals rendering — the required string has
  no decimal part while row 2's does, so the two goldens together pin the switch point.

### Row 4 — a minute or more splits into minutes and seconds
- **Authority**: `TASK.md` behavior 4 (`90000` → `"1m 30s"`).
- **Selector**: `test_at_least_a_minute_splits_into_minutes_and_seconds`
- **Red**: `K-focused test_at_least_a_minute_splits_into_minutes_and_seconds`. Intended failure: the flat
  seconds formatter never emits a minutes component. Observed: `AssertionError`, expected `1m 30s`,
  got `90s`. Intended == observed.
- **Green**: `minutes, seconds = divmod(ms // 1000, 60)`, emitting `f"{minutes}m {seconds}s"` when
  `minutes` is non-zero. `K-focused ...` → 1 passed; `K-suite` → 4 passed.
- **Discriminating golden**: the observed red output was literally `"90s"` — this golden was observed to
  reject the flat-total-seconds alternative.

### Row 5 — an hour or more splits into hours, minutes and seconds
- **Authority**: `TASK.md` behavior 5 (`3723000` → `"1h 2m 3s"`).
- **Selector**: `test_at_least_an_hour_splits_into_hours_minutes_and_seconds`
- **Red**: `K-focused test_at_least_an_hour_splits_into_hours_minutes_and_seconds`. Intended failure: the
  minutes/seconds formatter never emits an hours component and lets minutes overflow past 59. Observed:
  `AssertionError`, expected `1h 2m 3s`, got `62m 3s`. Intended == observed.
- **Green**: added `hours, minutes = divmod(minutes, 60)` and the `f"{hours}h {minutes}m {seconds}s"`
  branch. `K-focused ...` → 1 passed; `K-suite` → 5 passed.
- **Discriminating golden**: the observed red output was literally `"62m 3s"` — this golden was observed
  to reject the unbounded-minutes alternative. `3723000` also separates each component (1/2/3), so a
  wrong component order or a wrong divisor cannot produce the required string by coincidence.

### Row 6 — negative input is refused
- **Authority**: `TASK.md` behavior 6.
- **Selector**: `test_negative_input_is_rejected`
- **Red**: `K-focused test_negative_input_is_rejected`. Intended failure: nothing validates the input, so
  no `ValueError` is raised. Observed: `Failed: DID NOT RAISE <class 'ValueError'>`. Intended == observed.
- **Green**: added the leading `if ms < 0: raise ValueError(...)` guard.
  `K-focused ...` → 1 passed; `K-suite` → 6 passed.
- **Note**: `pytest` was imported into the test module in this cycle, for `pytest.raises`.

## Pre-review reconciliation (tests → seams)

The working directory is not a git repository (`git status` → `fatal: not a git repository`) and every
file in it is new, so the scoped change is the whole of `test_duration.py`. Inventory derived from the
repository's native test structure via `python3 -m pytest --collect-only -q` (6 selectors collected).

| # | Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
|---|---|---|---|---|---|
| 1 | `test_duration.py` | `test_zero_formats_as_bare_zero_seconds` | `duration.format_duration` | `TASK.md` behavior 1 | Row 1 |
| 2 | `test_duration.py` | `test_sub_second_formats_with_two_decimal_places` | `duration.format_duration` | `TASK.md` behavior 2 | Row 2 |
| 3 | `test_duration.py` | `test_whole_seconds_under_a_minute_format_without_decimals` | `duration.format_duration` | `TASK.md` behavior 3 | Row 3 |
| 4 | `test_duration.py` | `test_at_least_a_minute_splits_into_minutes_and_seconds` | `duration.format_duration` | `TASK.md` behavior 4 | Row 4 |
| 5 | `test_duration.py` | `test_at_least_an_hour_splits_into_hours_minutes_and_seconds` | `duration.format_duration` | `TASK.md` behavior 5 | Row 5 |
| 6 | `test_duration.py` | `test_negative_input_is_rejected` | `duration.format_duration` | `TASK.md` behavior 6 | Row 6 |

**Both-directions comparison.** Collected inventory identities = the 6 selectors listed by
`--collect-only`. Reconciliation identities = the 6 rows above. Inventory minus reconciliation: empty.
Reconciliation minus inventory: empty. Gate passes.

**Coverage check.** Every row claims public behavior and proves it through the ratified public entry
point; no row substitutes a private-invariant test for a public-behavior claim. No test reaches through a
module-private helper, mutable global, or side channel — the tests import one name and assert on its
return value or its raised exception.

**Anti-pattern sweep.** No mocks (nothing here crosses a system boundary — no clock, no I/O, no
randomness, so `mocking.md` has nothing to apply). No expected value is recomputed the way the code
computes it; all six are literals taken from `TASK.md`. Work was vertical: one test → one implementation,
six times, never bulk-written.

## Open questions (deliberately untested — the spec does not fix them)

Recorded rather than guessed at, since inventing an answer would mean testing imagined behavior:

- Rounding at the sub-second boundary: `999` currently renders as `"1.00s"` (a second's worth of text for
  a sub-second value). `TASK.md` does not fix this.
- Whether a non-zero sub-second value that rounds to `0.00`, e.g. `1`, should render `"0.00s"` (current
  behavior) or something else.
- Zero-valued leading components inside a larger duration, e.g. whether `3600000` should be `"1h 0m 0s"`
  (current behavior) or `"1h"`.
- Non-integer or non-numeric input: a `float` works, a `str` raises `TypeError` from the comparison.
  `TASK.md` specifies rejection only for negatives.

## Chronological command log

Every command, in order, with its outcome. All run from the working directory
`.../scratchpad/runs/T2-current-run2`.

| # | Command | Outcome |
|---|---|---|
| 1 | `python3 --version && python3 -m pytest --version && ls -la` | Passed — Python 3.14.4, pytest 9.0.2, only `TASK.md` and `skill/` present |
| 2 | *(wrote `test_duration.py` with test 1 only)* | — |
| 3 | `python3 -m pytest "test_duration.py::test_zero_formats_as_bare_zero_seconds" -q` | **FAILED (intended red, cycle 1)** — 1 error, `ModuleNotFoundError: No module named 'duration'` |
| 4 | *(wrote `duration.py` returning `"0s"`)* | — |
| 5 | `python3 -m pytest "test_duration.py::test_zero_formats_as_bare_zero_seconds" -q` | **PASSED** — 1 passed (green, cycle 1) |
| 6 | *(added test 2 to `test_duration.py`)* | — |
| 7 | `python3 -m pytest "test_duration.py::test_sub_second_formats_with_two_decimal_places" -q` | **FAILED (intended red, cycle 2)** — expected `0.45s`, got `0s` |
| 8 | *(added the sub-second branch to `duration.py`)* | — |
| 9 | `python3 -m pytest "test_duration.py::test_sub_second_formats_with_two_decimal_places" -q` | **PASSED** — 1 passed (green, cycle 2) |
| 10 | `python3 -m pytest test_duration.py -q` | **PASSED** — 2 passed (regression check after cycle 2) |
| 11 | *(added test 3)* | — |
| 12 | `python3 -m pytest "test_duration.py::test_whole_seconds_under_a_minute_format_without_decimals" -q` | **FAILED (intended red, cycle 3)** — expected `9s`, got `0s` |
| 13 | *(fallback became `f"{ms // 1000}s"`)* | — |
| 14 | `python3 -m pytest "test_duration.py::test_whole_seconds_under_a_minute_format_without_decimals" -q` | **PASSED** — 1 passed (green, cycle 3) |
| 15 | `python3 -m pytest test_duration.py -q` | **PASSED** — 3 passed (regression check after cycle 3) |
| 16 | *(added test 4)* | — |
| 17 | `python3 -m pytest "test_duration.py::test_at_least_a_minute_splits_into_minutes_and_seconds" -q` | **FAILED (intended red, cycle 4)** — expected `1m 30s`, got `90s` |
| 18 | *(added the minutes split)* | — |
| 19 | `python3 -m pytest "test_duration.py::test_at_least_a_minute_splits_into_minutes_and_seconds" -q` | **PASSED** — 1 passed (green, cycle 4) |
| 20 | `python3 -m pytest test_duration.py -q` | **PASSED** — 4 passed (regression check after cycle 4) |
| 21 | *(added test 5)* | — |
| 22 | `python3 -m pytest "test_duration.py::test_at_least_an_hour_splits_into_hours_minutes_and_seconds" -q` | **FAILED (intended red, cycle 5)** — expected `1h 2m 3s`, got `62m 3s` |
| 23 | *(added the hours split)* | — |
| 24 | `python3 -m pytest "test_duration.py::test_at_least_an_hour_splits_into_hours_minutes_and_seconds" -q` | **PASSED** — 1 passed (green, cycle 5) |
| 25 | `python3 -m pytest test_duration.py -q` | **PASSED** — 5 passed (regression check after cycle 5) |
| 26 | *(added `import pytest` and test 6)* | — |
| 27 | `python3 -m pytest "test_duration.py::test_negative_input_is_rejected" -q` | **FAILED (intended red, cycle 6)** — `Failed: DID NOT RAISE <class 'ValueError'>` |
| 28 | *(added the negative guard)* | — |
| 29 | `python3 -m pytest "test_duration.py::test_negative_input_is_rejected" -q` | **PASSED** — 1 passed (green, cycle 6) |
| 30 | `python3 -m pytest -q` | **PASSED** — 6 passed (full suite) |
| 31 | `python3 -m pytest --collect-only -q` | **PASSED** — 6 tests collected (reconciliation inventory) |
| 32 | `git status --porcelain` | **FAILED** — `fatal: not a git repository`; expected, the working directory is untracked scratch space, so the scoped change is "every file here is new" |

Summary of failures: every failing run (#3, #7, #12, #17, #22, #27) was a deliberate red step whose
observed failure matched the intended failure for that cycle, plus #32, the `git status` probe that
established there is no repository to diff against. No unintended failure occurred at any point, and no
red step had to be retried.
