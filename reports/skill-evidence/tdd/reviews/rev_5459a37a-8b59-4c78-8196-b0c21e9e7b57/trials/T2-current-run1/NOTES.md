# NOTES — duration formatter (TDD skill)

Deliverables: `duration.py`, `test_duration.py`. Supporting record: `SEAMS.md` (seam agreement,
written before the first test), `.sensitivity/run_sensitivity.py` (discriminating-golden harness).

All commands below were run from the working directory
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/b0fd637a-3121-48c1-b47e-a6e9d2129c01/scratchpad/runs/T2-current-run1`.
Environment: Python 3.14.4, pytest 9.0.2. No `CONTEXT.md` and no `docs/adr/` exist in this working
directory, so there was no project domain language or ADR to conform to.

---

## 1. The record the skill asks for

### 1.1 Seam agreement

One ratified seam, **S1**: `duration.format_duration(ms) -> str`, reached as
`from duration import format_duration`. Authority: `TASK.md`, which fixes the module, the signature,
and six acceptance behaviors. Full agreement, ingress enumeration, and the rejected-alternative
table are in `SEAMS.md`.

### 1.2 Decisions I made without a user

No user was available to confirm anything, so I decided and recorded these (also in `SEAMS.md`):

1. **`TASK.md` is treated as the authoritative PRD** for seam ratification. SKILL.md permits a seam
   ratified in "an authoritative PRD, live issue, ADR, or explicit user decision"; TASK.md is the
   only specification in scope and it fixes both interface and goldens.
2. **Scope is exactly the six acceptance behaviors.** I deliberately did not invent unratified
   goldens — e.g. what `3600000` renders as (`"1h"`? `"1h 0m 0s"`?), whether `1500` is `"1.5s"` or
   `"1s"`, or how a non-integer or non-numeric `ms` behaves. Writing a test for any of those would
   freeze my guess as a spec at a seam no authority ratified. See §1.6 for the open questions.
3. **`/code-review` was not invoked** — the task asks for working code and tests, not a review pass.
   I still ran the pre-review reconciliation gate (§1.4) so the evidence is review-ready. There are
   consequently **no review-reentry rows**: no review ran, so no finding exists to key one to.

### 1.3 TDD evidence rows

Stable command keys (each expands to a complete runnable command, run from the working directory):

- `FOCUSED(<selector>)` = `python3 -m pytest "test_duration.py::<selector>"`
- `SUITE` = `python3 -m pytest`

Every row's seam authority is **S1 / `TASK.md`** and every row's observed public entry point is
**`format_duration`, imported as `from duration import format_duration` in `test_duration.py`** —
no test reaches a module internal, a helper, a global, or any side channel.

---

**Row 1 — zero renders as bare zero seconds** (TASK.md behavior 1)

- Test file / selector: `test_duration.py::test_zero_renders_as_bare_zero_seconds`
- Red command: `FOCUSED(test_zero_renders_as_bare_zero_seconds)` =
  `python3 -m pytest "test_duration.py::test_zero_renders_as_bare_zero_seconds"`
- Intended failure: the public module `duration.py` does not exist yet, so the import of the seam
  fails.
- Observed failure: `1 error` — `ModuleNotFoundError: No module named 'duration'` at
  `test_duration.py:1`. Matches the intended failure.
- Minimal implementation: `def format_duration(ms): return "0s"`.
- Green command: same `FOCUSED(...)`. Result: **1 passed**.
- Note: this red is at import level, not assertion level, so this row alone does not prove the
  assertion can fail. That gap is closed by the sensitivity check in §1.5 (mutant M1 substitutes
  `""` for zero and this selector kills it).

**Row 2 — sub-second renders with two decimal places** (TASK.md behavior 2)

- Test file / selector: `test_duration.py::test_sub_second_renders_with_two_decimal_places`
- Red command: `FOCUSED(test_sub_second_renders_with_two_decimal_places)` =
  `python3 -m pytest "test_duration.py::test_sub_second_renders_with_two_decimal_places"`
- Intended failure: the constant `"0s"` from row 1 is returned for `450`, so no sub-second form exists.
- Observed failure: `1 failed` — `AssertionError`, `- 0.45s` / `+ 0s`. Matches.
- Minimal implementation: keep `"0s"` for zero, otherwise `f"{ms / 1000:.2f}s"`.
- Green command: same `FOCUSED(...)`. Result: **1 passed**. `SUITE`: **2 passed**.

**Row 3 — whole seconds render without decimals** (TASK.md behavior 3)

- Test file / selector: `test_duration.py::test_whole_seconds_render_without_decimals`
- Red command: `FOCUSED(test_whole_seconds_render_without_decimals)` =
  `python3 -m pytest "test_duration.py::test_whole_seconds_render_without_decimals"`
- Intended failure: row 2's two-decimal form applies to every non-zero input, so `9000` renders as
  `"9.00s"` — exactly the rejected alternative recorded for this behavior in `SEAMS.md`.
- Observed failure: `1 failed` — `AssertionError`, `- 9s` / `+ 9.00s`. Matches.
- Minimal implementation: restrict the two-decimal form to `ms < 1000`; otherwise `f"{ms // 1000}s"`.
- Green command: same `FOCUSED(...)`. Result: **1 passed**. `SUITE`: **3 passed**.

**Row 4 — a minute and a half rolls over into minutes** (TASK.md behavior 4)

- Test file / selector: `test_duration.py::test_a_minute_and_a_half_rolls_over_into_minutes`
- Red command: `FOCUSED(test_a_minute_and_a_half_rolls_over_into_minutes)` =
  `python3 -m pytest "test_duration.py::test_a_minute_and_a_half_rolls_over_into_minutes"`
- Intended failure: no minute unit exists, so `90000` renders as flat `"90s"` — the recorded
  rejected alternative.
- Observed failure: `1 failed` — `AssertionError`, `- 1m 30s` / `+ 90s`. Matches.
- Minimal implementation: below 60 seconds keep `f"{seconds}s"`; otherwise
  `divmod(seconds, 60)` into `f"{minutes}m {seconds}s"`.
- Green command: same `FOCUSED(...)`. Result: **1 passed**. `SUITE`: **4 passed**.

**Row 5 — more than an hour rolls over into hours** (TASK.md behavior 5)

- Test file / selector: `test_duration.py::test_more_than_an_hour_rolls_over_into_hours`
- Red command: `FOCUSED(test_more_than_an_hour_rolls_over_into_hours)` =
  `python3 -m pytest "test_duration.py::test_more_than_an_hour_rolls_over_into_hours"`
- Intended failure: no hour unit exists, so minutes accumulate past 60 and `3723000` renders as
  `"62m 3s"` — the recorded rejected alternative.
- Observed failure: `1 failed` — `AssertionError`, `- 1h 2m 3s` / `+ 62m 3s`. Matches.
- Minimal implementation: below 60 minutes keep `f"{minutes}m {seconds}s"`; otherwise
  `divmod(minutes, 60)` into `f"{hours}h {minutes}m {seconds}s"`.
- Green command: same `FOCUSED(...)`. Result: **1 passed**. `SUITE`: **5 passed**.

**Row 6 — a negative duration is refused** (TASK.md behavior 6)

- Test file / selector: `test_duration.py::test_a_negative_duration_is_refused`
- Red command: `FOCUSED(test_a_negative_duration_is_refused)` =
  `python3 -m pytest "test_duration.py::test_a_negative_duration_is_refused"`
- Intended failure: no validation exists, so `-1` falls into the sub-second branch and is formatted
  instead of refused.
- Observed failure: `1 failed` — `Failed: DID NOT RAISE <class 'ValueError'>`. Matches. I confirmed
  the value it silently produced rather than asserting it from memory:
  `python3 -c "print(repr(f'{-1/1000:.2f}s'))"` prints `'-0.00s'`, so the unguarded code returned
  the plausible-but-wrong `"-0.00s"`.
- Minimal implementation: leading guard
  `if ms < 0: raise ValueError(f"duration must not be negative, got {ms}")`.
- Green command: same `FOCUSED(...)`. Result: **1 passed**. `SUITE`: **6 passed**.
- Refusal-path coverage (`tests.md`): the test asserts the refusal through `pytest.raises`, which by
  construction observes no returned string, so no formatted value can leak on the refusal path.
  There is no protected payload in this domain and the `ValueError` message contains only the
  caller's own input.

**Refactor (outside the red → green loop, no behavior change):** added a module docstring and a
`format_duration` docstring stating the contract. No logic touched. `SUITE`: **6 passed**.

### 1.4 Pre-review reconciliation gate

Changed-test inventory, derived from the change under review (both `duration.py` and
`test_duration.py` are new files, so every test in the scope is a changed test) using pytest's
native structure via `python3 -m pytest --collect-only -q`:

```
test_duration.py::test_zero_renders_as_bare_zero_seconds
test_duration.py::test_sub_second_renders_with_two_decimal_places
test_duration.py::test_whole_seconds_render_without_decimals
test_duration.py::test_a_minute_and_a_half_rolls_over_into_minutes
test_duration.py::test_more_than_an_hour_rolls_over_into_hours
test_duration.py::test_a_negative_duration_is_refused
```

Reconciliation rows (one per changed test selector; no file- or class-level summaries):

| Test file | Exact selector | Observed public entry point | Seam authority | Evidence row |
|---|---|---|---|---|
| `test_duration.py` | `test_zero_renders_as_bare_zero_seconds` | `format_duration(0)` | S1 / TASK.md behavior 1 | Row 1 |
| `test_duration.py` | `test_sub_second_renders_with_two_decimal_places` | `format_duration(450)` | S1 / TASK.md behavior 2 | Row 2 |
| `test_duration.py` | `test_whole_seconds_render_without_decimals` | `format_duration(9000)` | S1 / TASK.md behavior 3 | Row 3 |
| `test_duration.py` | `test_a_minute_and_a_half_rolls_over_into_minutes` | `format_duration(90000)` | S1 / TASK.md behavior 4 | Row 4 |
| `test_duration.py` | `test_more_than_an_hour_rolls_over_into_hours` | `format_duration(3723000)` | S1 / TASK.md behavior 5 | Row 5 |
| `test_duration.py` | `test_a_negative_duration_is_refused` | `format_duration(-1)` | S1 / TASK.md behavior 6 | Row 6 |

Bidirectional comparison:

- inventory identities \ reconciliation identities = **empty**
- reconciliation identities \ inventory identities = **empty**

Coverage check: every row claims public behavior and proves it at S1. No row substitutes a
private-invariant test for a public-behavior proof; there are no private helpers, no mocks (nothing
here crosses a system boundary — no I/O, no clock, no randomness), and no side-channel assertions.

### 1.5 Discriminating-golden sensitivity check

`tests.md` requires that a golden which would still pass after the forbidden behavior is substituted
be replaced. I checked this by substitution rather than by argument: `.sensitivity/run_sensitivity.py`
writes each forbidden implementation into a throwaway temp directory beside a copy of the real test
file and runs the suite there. The real `duration.py` is never modified.

Command: `python3 .sensitivity/run_sensitivity.py` — **survivors: 0**. All nine mutants killed, each
by the selector whose row claims to discriminate it:

| Mutant | Killed by |
|---|---|
| M1 zero renders as empty string | `test_zero_renders_as_bare_zero_seconds` |
| M2 sub-second truncates to whole seconds | `test_sub_second_renders_with_two_decimal_places` |
| M3 sub-second uses one decimal place | `test_sub_second_renders_with_two_decimal_places` |
| M4 every duration gets two decimals | `test_whole_seconds_render_without_decimals` |
| M5 no minute rollover | `test_a_minute_and_a_half_rolls_over_into_minutes`, `test_more_than_an_hour_rolls_over_into_hours` |
| M6 no hour rollover | `test_more_than_an_hour_rolls_over_into_hours` |
| M7 drops the trailing seconds component | `test_more_than_an_hour_rolls_over_into_hours` |
| M8 negative guard removed | `test_a_negative_duration_is_refused` |
| M9 negative raises `TypeError` instead of `ValueError` | `test_a_negative_duration_is_refused` |

No golden is tautological: every expected value is a literal copied from `TASK.md`, not recomputed
the way the implementation computes it.

### 1.6 Open, deliberately untested questions

Flagged rather than guessed at, because no authority fixes them and an invented golden would freeze
a guess as a spec:

- Exact hours/minutes with a zero tail: does `3600000` render `"1h"`, `"1h 0m 0s"`, or `"1h 0s"`?
  The current cascade produces `"1h 0m 0s"`, but that is an artifact of the minimal implementation,
  not a ratified decision.
- Non-integer input: a float `ms` does not raise, but it leaks a float repr into every whole-unit
  path, because `float // 1000` stays a float. Measured, not assumed:
  `format_duration(9000.0)` -> `"9.0s"`, `format_duration(90000.0)` -> `"1.0m 30.0s"`,
  `format_duration(3723000.0)` -> `"1.0h 2.0m 3.0s"`, `format_duration(1500.5)` -> `"1.0s"`. This
  is the one place where the current implementation looks actively wrong rather than merely
  unspecified, but TASK.md ratifies only integer goldens, so fixing it would mean writing a test at
  an unratified seam. **Flagging it as the first thing to ratify if this ships.**
- Non-numeric input: `format_duration("x")` raises `TypeError` from the `ms < 0` comparison, not
  `ValueError`. TASK.md constrains only *negative* input, so I did not extend the guard.
- Sub-second rounding: `format_duration(455)` -> `"0.46s"` (Python's format rounds); `999` ->
  `"1.00s"` rather than `"1s"`. That boundary is unratified — worth pinning if this ever ships.

---

## 2. Chronological command log

Every command in order. File writes/edits are listed where they fall in the sequence for context but
are marked as such; only the numbered `$` lines are commands.

| # | Command | Outcome |
|---|---|---|
| 1 | `ls -la <wd> <wd>/skill/` | passed — found `TASK.md` and `skill/{SKILL.md,mocking.md,tests.md}` |
| 2 | `ls -a && find . -name 'CONTEXT.md' -o -name 'adr' -type d; python3 --version; python3 -m pytest --version` | passed — no `CONTEXT.md`, no ADR dir; Python 3.14.4, pytest 9.0.2 |
| — | *wrote `SEAMS.md`* (seam agreement, before any test) | — |
| — | *wrote `test_duration.py`* with test 1 only | — |
| 3 | `python3 -m pytest "test_duration.py::test_zero_renders_as_bare_zero_seconds"` | **failed by design (RED 1)** — `1 error`, `ModuleNotFoundError: No module named 'duration'` |
| — | *wrote `duration.py`* (returns `"0s"`) | — |
| 4 | `python3 -m pytest "test_duration.py::test_zero_renders_as_bare_zero_seconds"` | **passed (GREEN 1)** — 1 passed |
| — | *edited `test_duration.py`* to add test 2 | — |
| 5 | `python3 -m pytest "test_duration.py::test_sub_second_renders_with_two_decimal_places"` | **failed by design (RED 2)** — 1 failed, `- 0.45s` / `+ 0s` |
| — | *rewrote `duration.py`* (two-decimal branch) | — |
| 6 | `python3 -m pytest "test_duration.py::test_sub_second_renders_with_two_decimal_places"` then `python3 -m pytest test_duration.py` | **passed (GREEN 2)** — 1 passed, then 2 passed |
| — | *edited `test_duration.py`* to add test 3 | — |
| 7 | `python3 -m pytest "test_duration.py::test_whole_seconds_render_without_decimals"` | **failed by design (RED 3)** — 1 failed, `- 9s` / `+ 9.00s` |
| — | *rewrote `duration.py`* (whole-seconds branch) | — |
| 8 | `python3 -m pytest "test_duration.py::test_whole_seconds_render_without_decimals"` then `python3 -m pytest test_duration.py` | **passed (GREEN 3)** — 1 passed, then 3 passed |
| — | *edited `test_duration.py`* to add test 4 | — |
| 9 | `python3 -m pytest "test_duration.py::test_a_minute_and_a_half_rolls_over_into_minutes"` | **failed by design (RED 4)** — 1 failed, `- 1m 30s` / `+ 90s` |
| — | *rewrote `duration.py`* (minute rollover) | — |
| 10 | `python3 -m pytest "test_duration.py::test_a_minute_and_a_half_rolls_over_into_minutes"` then `python3 -m pytest test_duration.py` | **passed (GREEN 4)** — 1 passed, then 4 passed |
| — | *edited `test_duration.py`* to add test 5 | — |
| 11 | `python3 -m pytest "test_duration.py::test_more_than_an_hour_rolls_over_into_hours"` | **failed by design (RED 5)** — 1 failed, `- 1h 2m 3s` / `+ 62m 3s` |
| — | *rewrote `duration.py`* (hour rollover) | — |
| 12 | `python3 -m pytest "test_duration.py::test_more_than_an_hour_rolls_over_into_hours"` then `python3 -m pytest test_duration.py` | **passed (GREEN 5)** — 1 passed, then 5 passed |
| — | *edited `test_duration.py`* twice: added `import pytest`, added test 6 | — |
| 13 | `python3 -m pytest "test_duration.py::test_a_negative_duration_is_refused"` | **failed by design (RED 6)** — 1 failed, `Failed: DID NOT RAISE <class 'ValueError'>` |
| — | *edited `duration.py`* to add the negative guard | — |
| 14 | `python3 -m pytest "test_duration.py::test_a_negative_duration_is_refused"` then `python3 -m pytest test_duration.py` | **passed (GREEN 6)** — 1 passed, then 6 passed |
| — | *edited `duration.py`*: refactor only — module + function docstrings, no logic change | — |
| 15 | `python3 -m pytest` | passed — 6 passed (refactor confirmed behavior-neutral) |
| — | *wrote `.sensitivity/run_sensitivity.py`* | — |
| 16 | `python3 .sensitivity/run_sensitivity.py` | **FAILED unexpectedly** — exit 1, `IndexError: list index out of range`. My harness bug: it parsed `split(" ")[0]` of a pytest `FAILED ...` summary line, which is the literal word `FAILED`, not the selector. Not a product defect; `duration.py` and `test_duration.py` untouched |
| — | *edited `.sensitivity/run_sensitivity.py`*: `split(" ")[0]` → `split(" ")[1]` | — |
| 17 | `python3 .sensitivity/run_sensitivity.py` | passed — all 9 mutants KILLED, `survivors: 0` |
| 18 | `python3 -m pytest --collect-only -q` | passed — exactly the 6 expected selectors collected; `.sensitivity/` correctly not collected |
| 19 | `python3 -c "print(repr(f'{-1/1000:.2f}s'))"` then `python3 -m pytest` | passed — printed `'-0.00s'` (confirming the RED-6 leak value), then 6 passed |
| 20 | `python3 -c "from duration import format_duration as f; print(repr(f(3600000)), repr(f(1500.5)), repr(f(455)), repr(f(999))); ..."` then `python3 -m pytest` | passed, **but falsified one of my own claims** — printed `'1h 0m 0s' '1.0s' '0.46s' '1.00s'` and `TypeError`. I had written in §1.6 that `format_duration(1500.5)` renders `"1s"`; it actually renders `"1.0s"`. Corrected §1.6. Then 6 passed |
| 21 | `python3 -c "from duration import format_duration as f; print(repr(f(1500.5)), repr(f(90000.0)), repr(f(3723000.0)), repr(f(9000.0)))"` | passed — `'1.0s' '1.0m 30.0s' '1.0h 2.0m 3.0s' '9.0s'`, establishing the float leak affects every whole-unit path, not just one |

**Summary of failures.** Seven commands reported failure. Six were the intended reds — commands 3,
5, 7, 9, 11, 13, one per acceptance behavior, each failing for the reason its evidence row predicted.
Exactly one was an unintended failure: command 16, a bug in my own sensitivity harness's output
parsing, fixed and re-run green as command 17. No product test ever failed unexpectedly, and no
test was ever changed to accommodate the implementation.

One written claim was falsified by checking it: at command 20 the float-input behavior I had
recorded in §1.6 turned out to be wrong (`"1.0s"`, not `"1s"`), and command 21 showed the same leak
runs through every whole-unit path. §1.6 now records the measured values.

Final state: `python3 -m pytest` → **6 passed**.
