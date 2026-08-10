# TDD record — duration formatter

Skill followed: `skill/SKILL.md` (tdd), with `skill/tests.md` and `skill/mocking.md`.

## Context scan

- No `CONTEXT.md` and no `docs/adr/` in this working directory, so no project domain
  vocabulary or ADR constrains the naming. `TASK.md` is the only authority available.
- Environment: Python 3.14.4, pytest 9.0.2. Working directory is a fresh git repo whose
  baseline commit `ee4b1a8` ("Baseline: task and skill only") contains only `TASK.md` and
  `skill/`. That commit is the fixed point for the scoped diff used by the pre-review
  reconciliation gate.

## Decisions I made without a user (no user was available to confirm)

1. **Seam ratification.** The skill requires seams be pre-agreed and says to ask the user
   when the authority is absent or ambiguous. No user is available. `TASK.md` is explicit —
   it names the public module (`duration.py`), the public function
   (`format_duration(ms) -> str`), and six required behaviors — so I treated `TASK.md` as
   the ratifying authority rather than asking. The seam is *not* ambiguous; the task text
   fixes it exactly, so this decision carried no discretion.
2. **`/code-review` was not invoked.** The skill's reconciliation gate is defined as a
   pre-`/code-review` gate, and its review-repair rule is keyed to `/code-review` findings.
   The task asks only for working code and tests, and no reviewer is available. I performed
   and retained the reconciliation gate (below) because this skill owns it, but did not run
   `/code-review`, so there are no review-reentry evidence rows and the
   `Findings fixed during review` comparison is vacuous (both sides empty).
3. **Rounding at the sub-second boundary.** `TASK.md` fixes `450 ms -> "0.45s"` but says
   nothing about, e.g., `999.5`-style ties. Inputs are milliseconds, and every acceptance
   case is an integer count of milliseconds, so two decimal places is exact for integer
   input below one second and no tie-breaking rule is needed. I did not invent behavior
   for non-integer input; nothing in the task calls for it, and speculative features are
   forbidden by the loop rules.

## Seams under test (written down before the first test)

| Seam | Authority | Notes |
| --- | --- | --- |
| `duration.format_duration(ms)`, imported as `from duration import format_duration` | `TASK.md` lines 6–16 (task specification supplied as the authoritative requirement) | The single public entry point. |

**Public-invariant ingress check** (`skill/tests.md`, "Public invariant coverage"): the
invariant "a negative duration is not formattable" can only be admitted through
`format_duration` itself. The module exposes no other parser, constructor, factory, or
deserializer that could construct or admit a duration value and bypass that validation, so
there is exactly one ingress path and it is probed directly by behavior 6. Confirmed against
the final module: `duration.py` defines exactly one public name, `format_duration`.

**Refusal-path non-observation** (`skill/tests.md`): behavior 6 asserts the refusal *and*
that no formatted string is produced — `pytest.raises` proves no return value escaped, and
the test additionally asserts the raised message does not leak a formatted duration.

## Golden discrimination

Each golden is a literal taken from `TASK.md`, never recomputed the way the implementation
computes it. Rejected alternatives each golden rules out:

| Input | Required (authority: `TASK.md`) | Rejected implementations it discriminates against |
| --- | --- | --- |
| `0` | `"0s"` | `""` (dropping zero-valued units entirely), `"0.00s"` (always two decimals) |
| `450` | `"0.45s"` | `"0s"` (integer truncation of sub-second), `"0.5s"` (one decimal), `"0.450s"` (three decimals), `"450ms"` (millisecond unit) |
| `9000` | `"9s"` | `"9.00s"` (always two decimals), `"0m 9s"` (padding zero units) |
| `90000` | `"1m 30s"` | `"90s"` (no minute rollover), `"1.5m"` (fractional minutes), `"1m 30.00s"` (decimals above one second) |
| `3723000` | `"1h 2m 3s"` | `"62m 3s"` (no hour rollover), `"1h 2m 3.00s"`, `"3723s"` |
| `-1` | `ValueError` | clamping to `"0s"`, formatting as `"-0.00s"` |

None of these goldens would still pass if the corresponding forbidden behavior were
substituted, which is the discrimination requirement in `skill/tests.md`.

## Mocking

No mocks. `skill/mocking.md` restricts mocking to system boundaries; `format_duration` is a
pure function with no external API, database, clock, randomness, or filesystem dependency,
so every test runs through the real public interface.

## TDD evidence (one row per behavior)

Command keys, so every row is replayable verbatim:

- **K-RED(sel)** = `python3 -m pytest "test_duration.py::<sel>" -q`
- **K-GREEN(sel)** = `python3 -m pytest "test_duration.py::<sel>" -q`
- **K-SUITE** = `python3 -m pytest -q`

All commands run with the working directory set to
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/b0fd637a-3121-48c1-b47e-a6e9d2129c01/scratchpad/runs/T2-current-run3`.

### Row 1 — zero renders as a bare `0s`

- Seam authority: `TASK.md` behavior 1.
- Test file / selector: `test_duration.py::test_zero_renders_as_bare_zero_seconds`.
- Observed public entry point: `format_duration(0)`.
- Red command: `python3 -m pytest "test_duration.py::test_zero_renders_as_bare_zero_seconds" -q`
  - Intended failure: the seam does not exist yet.
  - Observed failure: `ModuleNotFoundError: No module named 'duration'` (collection error, 1 error).
- Minimal implementation: `format_duration` returns the literal `"0s"`.
- Green command: same as red command above. Result: 1 passed.
- Sensitivity note: because this red was an import error rather than a wrong rendering, the
  golden's discrimination was verified separately by mutation (see "Golden sensitivity check").

### Row 2 — sub-second renders with two decimal places

- Seam authority: `TASK.md` behavior 2.
- Test file / selector: `test_duration.py::test_sub_second_renders_with_two_decimal_places`.
- Observed public entry point: `format_duration(450)`.
- Red command: `python3 -m pytest "test_duration.py::test_sub_second_renders_with_two_decimal_places" -q`
  - Intended failure: the hardcoded `"0s"` cannot render a sub-second value.
  - Observed failure: `AssertionError`, expected `0.45s`, got `0s` (1 failed).
- Minimal implementation: keep the `ms == 0` case; otherwise `f"{ms / 1000:.2f}s"`.
- Green command: same as red command above. Result: 1 passed; `K-SUITE` 2 passed.

### Row 3 — whole seconds under a minute render without decimals

- Seam authority: `TASK.md` behavior 3.
- Test file / selector: `test_duration.py::test_whole_seconds_under_a_minute_render_without_decimals`.
- Observed public entry point: `format_duration(9000)`.
- Red command: `python3 -m pytest "test_duration.py::test_whole_seconds_under_a_minute_render_without_decimals" -q`
  - Intended failure: the two-decimal path applies to every non-zero value.
  - Observed failure: `AssertionError`, expected `9s`, got `9.00s` (1 failed).
- Minimal implementation: restrict the two-decimal path to `ms < 1000`; otherwise `f"{ms // 1000}s"`.
- Green command: same as red command above. Result: 1 passed; `K-SUITE` 3 passed.

### Row 4 — ninety seconds rolls over into minutes and seconds

- Seam authority: `TASK.md` behavior 4.
- Test file / selector: `test_duration.py::test_ninety_seconds_rolls_over_into_minutes_and_seconds`.
- Observed public entry point: `format_duration(90000)`.
- Red command: `python3 -m pytest "test_duration.py::test_ninety_seconds_rolls_over_into_minutes_and_seconds" -q`
  - Intended failure: no minute rollover exists yet.
  - Observed failure: `AssertionError`, expected `1m 30s`, got `90s` (1 failed).
- Minimal implementation: `divmod(ms // 1000, 60)`; emit the minute segment only when non-zero.
- Green command: same as red command above. Result: 1 passed; `K-SUITE` 4 passed.

### Row 5 — multi-hour duration renders hours, minutes and seconds

- Seam authority: `TASK.md` behavior 5.
- Test file / selector: `test_duration.py::test_multi_hour_duration_renders_hours_minutes_and_seconds`.
- Observed public entry point: `format_duration(3723000)`.
- Red command: `python3 -m pytest "test_duration.py::test_multi_hour_duration_renders_hours_minutes_and_seconds" -q`
  - Intended failure: minutes are never carried into hours.
  - Observed failure: `AssertionError`, expected `1h 2m 3s`, got `62m 3s` (1 failed).
- Minimal implementation: second `divmod(minutes, 60)`; emit the hour segment only when non-zero.
- Green command: same as red command above. Result: 1 passed; `K-SUITE` 5 passed.

### Row 6 — negative input is refused, leaking no formatted duration

- Seam authority: `TASK.md` behavior 6, plus the refusal-path rule in `skill/tests.md`
  (assert both the refusal and non-observation of the protected output).
- Test file / selector: `test_duration.py::test_negative_duration_is_rejected_without_producing_a_formatted_duration`.
- Observed public entry point: `format_duration(-1)`.
- Red command: `python3 -m pytest "test_duration.py::test_negative_duration_is_rejected_without_producing_a_formatted_duration" -q`
  - Intended failure: nothing rejects negative input, so no `ValueError` is raised.
  - Observed failure: `Failed: DID NOT RAISE <class 'ValueError'>` (1 failed). Diagnostic
    `python3 -c "import duration; print(repr(duration.format_duration(-1)))"` confirmed the
    pre-fix behavior returned `'-0.00s'` — precisely the rejected rendering in the golden table.
- Minimal implementation: guard `if ms < 0: raise ValueError(...)` at the top of the function.
  The message names the offending input but embeds no formatted duration.
- Green command: same as red command above. Result: 1 passed; `K-SUITE` 6 passed.

## Golden sensitivity check (row 1)

Rows 2–6 each observed the rejected implementation's own output as their red, which *is* the
sensitivity evidence for those goldens. Row 1's red was an import error, so I checked it by
mutation in a throwaway copy outside the deliverable (`scratchpad/mutants/`, since deleted):

| Mutant | Forbidden behavior substituted | Result |
| --- | --- | --- |
| A | removed the `ms == 0` case, so zero renders `"0.00s"` (always two decimals) | test FAILED (mutant killed) |
| B | zero-valued unit dropped, so zero renders `""` | test FAILED (mutant killed) |

Both mutants are killed, so the `0 -> "0s"` golden is discriminating and was kept.

## Pre-review reconciliation (changed-test inventory vs. evidence)

Inventory derived from the scoped diff against baseline `ee4b1a8`, using pytest's own
collection as the native test structure — not hand-listed:

```
python3 -m pytest --collect-only -q $(git diff --cached --name-only ee4b1a8 -- '*test_*.py')
```

Changed test files in the scoped diff: `test_duration.py` (added). Non-test files in the same
diff: `duration.py`, `NOTES.md`, `.gitignore`.

| # | Test file | Exact selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `test_duration.py` | `test_zero_renders_as_bare_zero_seconds` | `format_duration(0)` | `TASK.md` behavior 1 | Row 1 |
| 2 | `test_duration.py` | `test_sub_second_renders_with_two_decimal_places` | `format_duration(450)` | `TASK.md` behavior 2 | Row 2 |
| 3 | `test_duration.py` | `test_whole_seconds_under_a_minute_render_without_decimals` | `format_duration(9000)` | `TASK.md` behavior 3 | Row 3 |
| 4 | `test_duration.py` | `test_ninety_seconds_rolls_over_into_minutes_and_seconds` | `format_duration(90000)` | `TASK.md` behavior 4 | Row 4 |
| 5 | `test_duration.py` | `test_multi_hour_duration_renders_hours_minutes_and_seconds` | `format_duration(3723000)` | `TASK.md` behavior 5 | Row 5 |
| 6 | `test_duration.py` | `test_negative_duration_is_rejected_without_producing_a_formatted_duration` | `format_duration(-1)` | `TASK.md` behavior 6 | Row 6 |

**Both-direction comparison** (run with `comm`, not asserted by eye): inventory selectors minus
reconciliation selectors = empty; reconciliation selectors minus inventory selectors = empty.
6 selectors on each side. **Gate passes.**

**Boundary check.** `grep -n duration test_duration.py` shows the tests' only route into
production code is `from duration import format_duration`; there is no module-internal import,
helper, mutable global, or side channel. `dir(duration)` reports exactly one public name,
`format_duration`. Every row therefore proves public behavior at the ratified seam, and no row
substitutes a private-invariant test for a public-behavior claim.

**Review-reentry rows:** none. `/code-review` was not run (decision 2 above), so the
`Findings fixed during review` comparison is empty on both sides.

## Final state

`python3 -m pytest -q` → **6 passed**. All six acceptance behaviors from `TASK.md` are covered
by one test each, and no test was written for behavior the task does not specify.

---

# Chronological command log

Every command in order. Working directory for all of them is the run directory
`.../scratchpad/runs/T2-current-run3` unless stated otherwise.

| # | Command | Outcome |
| --- | --- | --- |
| 1 | `python3 --version && python3 -m pytest --version && ls -a && git rev-parse --is-inside-work-tree && ls CONTEXT.md` | **Mixed, as expected.** Python 3.14.4 and pytest 9.0.2 reported; `git rev-parse` failed (`not a git repository`) and `ls CONTEXT.md` failed (no such file). Compound exit code 2. Both failures were informational probes, not errors in the work. |
| 2 | `git init -q && git add TASK.md skill && git -c user.name=tdd -c user.email=tdd@local commit -q -m "Baseline: task and skill only" && git log --oneline` | Passed. Baseline commit `ee4b1a8` created as the scoped-diff fixed point. |
| 3 | Wrote `NOTES.md` (seams, decisions, golden table) — no command. | n/a |
| 4 | Wrote `test_duration.py` with **only** `test_zero_renders_as_bare_zero_seconds`. | n/a |
| 5 | `python3 -m pytest "test_duration.py::test_zero_renders_as_bare_zero_seconds" -q` | **FAILED (intended red, cycle 1).** 1 error: `ModuleNotFoundError: No module named 'duration'`. |
| 6 | Wrote `duration.py` returning the literal `"0s"`. | n/a |
| 7 | `python3 -m pytest "test_duration.py::test_zero_renders_as_bare_zero_seconds" -q` | **PASSED** (green, cycle 1). 1 passed. |
| 8 | Added `test_sub_second_renders_with_two_decimal_places` to `test_duration.py`. | n/a |
| 9 | `python3 -m pytest "test_duration.py::test_sub_second_renders_with_two_decimal_places" -q` | **FAILED (intended red, cycle 2).** 1 failed: expected `0.45s`, got `0s`. |
| 10 | Edited `duration.py`: two-decimal rendering for non-zero values. | n/a |
| 11 | `python3 -m pytest "test_duration.py::test_sub_second_renders_with_two_decimal_places" -q` then `python3 -m pytest test_duration.py -q` | **Both PASSED** (green, cycle 2). 1 passed, then 2 passed. |
| 12 | Added `test_whole_seconds_under_a_minute_render_without_decimals`. | n/a |
| 13 | `python3 -m pytest "test_duration.py::test_whole_seconds_under_a_minute_render_without_decimals" -q` | **FAILED (intended red, cycle 3).** 1 failed: expected `9s`, got `9.00s`. |
| 14 | Edited `duration.py`: two-decimal path limited to `ms < 1000`, whole seconds above. | n/a |
| 15 | `python3 -m pytest "test_duration.py::test_whole_seconds_under_a_minute_render_without_decimals" -q` then `python3 -m pytest test_duration.py -q` | **Both PASSED** (green, cycle 3). 1 passed, then 3 passed. |
| 16 | Added `test_ninety_seconds_rolls_over_into_minutes_and_seconds`. | n/a |
| 17 | `python3 -m pytest "test_duration.py::test_ninety_seconds_rolls_over_into_minutes_and_seconds" -q` | **FAILED (intended red, cycle 4).** 1 failed: expected `1m 30s`, got `90s`. |
| 18 | Edited `duration.py`: minute rollover via `divmod(ms // 1000, 60)`. | n/a |
| 19 | `python3 -m pytest "test_duration.py::test_ninety_seconds_rolls_over_into_minutes_and_seconds" -q` then `python3 -m pytest test_duration.py -q` | **Both PASSED** (green, cycle 4). 1 passed, then 4 passed. |
| 20 | Added `test_multi_hour_duration_renders_hours_minutes_and_seconds`. | n/a |
| 21 | `python3 -m pytest "test_duration.py::test_multi_hour_duration_renders_hours_minutes_and_seconds" -q` | **FAILED (intended red, cycle 5).** 1 failed: expected `1h 2m 3s`, got `62m 3s`. |
| 22 | Edited `duration.py`: hour rollover via a second `divmod`. | n/a |
| 23 | `python3 -m pytest "test_duration.py::test_multi_hour_duration_renders_hours_minutes_and_seconds" -q` then `python3 -m pytest test_duration.py -q` | **Both PASSED** (green, cycle 5). 1 passed, then 5 passed. |
| 24 | Added `import pytest` and `test_negative_duration_is_rejected_without_producing_a_formatted_duration`. | n/a |
| 25 | `python3 -m pytest "test_duration.py::test_negative_duration_is_rejected_without_producing_a_formatted_duration" -q` | **FAILED (intended red, cycle 6).** 1 failed: `Failed: DID NOT RAISE <class 'ValueError'>`. |
| 26 | `python3 -c "import duration; print(repr(duration.format_duration(-1)))"` | Passed (diagnostic). Printed `'-0.00s'`, confirming the pre-fix rendering was the rejected alternative. |
| 27 | Edited `duration.py`: negative guard raising `ValueError`. | n/a |
| 28 | `python3 -m pytest "test_duration.py::test_negative_duration_is_rejected_without_producing_a_formatted_duration" -q` then `python3 -m pytest -q` | **Both PASSED** (green, cycle 6). 1 passed, then 6 passed. |
| 29 | `git add -A && git diff --cached --name-status ee4b1a8 && git diff --cached --name-only ee4b1a8 -- '*test_*.py' '*_test.py' && python3 -m pytest --collect-only -q <changed test files>` | Passed. Scoped diff listed; 6 selectors collected from `test_duration.py`. Revealed `__pycache__` had been staged. |
| 30 | Wrote `.gitignore`; `git rm -r --cached -q __pycache__`; `rm -rf __pycache__ .pytest_cache`; regenerated the inventory and compared it against the reconciliation list with `comm` in both directions. | Passed. Both differences empty, 6 selectors each side — reconciliation gate passes. |
| 31 | Created two mutant copies of the module in `scratchpad/mutants/{a,b}` and ran `python3 -m pytest test_duration.py::test_zero_renders_as_bare_zero_seconds -q` in each. | **Both FAILED — which is the desired result.** Each mutant was killed by the test, proving the `0 -> "0s"` golden discriminates. |
| 32 | `rm -rf .../scratchpad/mutants`; `grep -n duration test_duration.py`; `python3 -c "import duration; print([n for n in dir(duration) if not n.startswith('_')])"`; `python3 -m pytest -q` | Passed. Tests touch production code only via `from duration import format_duration`; module exports exactly `['format_duration']`; full suite **6 passed**. |

Failure summary: the only failing commands were the six intended reds (steps 5, 9, 13, 17, 21,
25), the two intended mutant kills (step 31), and the two informational probes inside step 1
(`git rev-parse`, `ls CONTEXT.md`). Every green and every full-suite run passed. The final state
of the deliverable is 6 passed, 0 failed.
