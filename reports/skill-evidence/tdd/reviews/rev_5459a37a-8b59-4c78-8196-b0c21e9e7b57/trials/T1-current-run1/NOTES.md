# TDD record — alert router

Skill followed: `skill/SKILL.md` (tdd) plus `skill/tests.md` and `skill/mocking.md`.

## Seams under test (written down before any test)

| Seam | Public entry point | Authority |
| --- | --- | --- |
| S1 | `alerting.should_alert(event) -> bool` | `TASK.md` acceptance behaviors 1-3 (module `alerting.py` and both function signatures are named there) |
| S2 | `alerting.escalation_count(events) -> int` | `TASK.md` acceptance behavior 4 |

Both seams are ratified by `TASK.md`, which is the authoritative spec for this
work, so no seam is ambiguous or absent and none needed to be raised with a
user. Nothing is tested below the module boundary: every test imports
`alerting` and calls only these two functions. No mocking is used — there is no
system boundary here (no I/O, clock, or network), so per `skill/mocking.md`
mocking would only couple tests to internals.

## Decisions I made without a user (no user was available to ask)

- **D1 — `severity="warning"` is unspecified.** `TASK.md` fixes the verdict for
  `"critical"` (alerts) and `"info"` (does not alert) but says nothing about
  `"warning"`, even though it is a legal severity. I implemented the minimal
  rule the stated behaviors force — only `"critical"` alerts — because the
  loop's "don't anticipate future tests or add speculative features" rule
  forbids inventing an escalation policy for `"warning"`. I deliberately did
  **not** write a test pinning `"warning"`: a test there would encode an
  unjustified decision as if it were spec. This is the one behavior a future
  authority may change without any test going red, and it is called out here
  for that reason.
- **D2 — "object/dict" means both.** `TASK.md` describes an event as "a simple
  object/dict with fields". I read that as: both a mapping and an attribute
  carrying object must work at both seams, and covered each with its own
  red → green slice (cycles 5 and 6) rather than assuming dict-only.
- **D3 — no `/code-review` run.** The task did not ask for a review and the
  review skill is out of scope here, but the loop's pre-review reconciliation
  gate is still executed below, since that gate is what proves the tests sit on
  the agreed seams.

## Command keys (replayable)

All paths absolute. `$BASE` =
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/b0fd637a-3121-48c1-b47e-a6e9d2129c01/scratchpad/runs/T1-current-run1`
— expand it literally when replaying; it is written out in full in the log at
the bottom of this file.

- **K-focus(SELECTOR)** = `python3 -m pytest "$BASE/test_alerting.py::SELECTOR" -q`
- **K-suite** = `python3 -m pytest $BASE/test_alerting.py -q`
- **K-collect** = `python3 -m pytest $BASE/test_alerting.py --collect-only -q`

## TDD evidence rows

One vertical slice per row: one seam, one test, one minimal implementation.
No row was rewritten to absorb a later cycle.

### Row 1 — a critical event alerts

- Seam authority: S1, `TASK.md` acceptance behavior 1.
- Test file / selector: `test_alerting.py::test_critical_event_alerts`.
- Observed public entry point: `alerting.should_alert`.
- Red: `K-focus(test_critical_event_alerts)`. Intended failure — no `alerting`
  module exists yet. Observed —
  `ModuleNotFoundError: No module named 'alerting'` (1 error). Match.
- Green: same command. Result — 1 passed. Minimal implementation:
  `should_alert` returned the constant `True`. Deliberately constant: nothing
  observed so far justifies looking at a field.

### Row 2 — an info event does not alert

- Seam authority: S1, `TASK.md` acceptance behavior 2.
- Test file / selector: `test_alerting.py::test_info_event_does_not_alert`.
- Observed public entry point: `alerting.should_alert`.
- Red: `K-focus(test_info_event_does_not_alert)`. Intended failure — the
  constant `True` from row 1 cannot distinguish severities. Observed —
  `AssertionError: assert True is False` (1 failed). Match.
- Green: same command. Result — 1 passed. Minimal implementation:
  `return event["severity"] == "critical"`. Regression: `K-suite` — 2 passed.

### Row 3 — a retrospective event never alerts

- Seam authority: S1, `TASK.md` acceptance behavior 3.
- Test file / selector:
  `test_alerting.py::test_retrospective_critical_event_does_not_alert`.
- Observed public entry point: `alerting.should_alert`.
- Red: `K-focus(test_retrospective_critical_event_does_not_alert)`. Intended
  failure — severity `critical` still alerts because nothing reads the
  retrospective flag. Observed — `AssertionError: assert True is False`
  (1 failed). Match. The input is a **critical** retrospective event on
  purpose: any lower severity would pass for the wrong reason, since row 2's
  rule already returns `False` for it, and the test would prove nothing.
- Green: same command. Result — 1 passed. Minimal implementation: early
  `return False` when `event["retrospective"]` is truthy.

### Row 4 — retrospective criticals still count as escalations

- Seam authority: S2, `TASK.md` acceptance behavior 4.
- Test file / selector:
  `test_alerting.py::test_escalation_count_counts_retrospective_criticals_too`.
- Observed public entry point: `alerting.escalation_count` (with
  `alerting.should_alert` used only to state the rejected alternative).
- Red: `K-focus(test_escalation_count_counts_retrospective_criticals_too)`.
  Intended failure — `escalation_count` does not exist. Observed —
  `ImportError: cannot import name 'escalation_count' from 'alerting'`
  (1 error; the import sits at module top so collection of the file failed,
  which is the intended absence showing up at collection time rather than in
  the assert). Match.
- Green: same command. Result — 1 passed. Minimal implementation:
  `sum(1 for event in events if event["severity"] == "critical")`. Regression:
  `K-suite` — 4 passed.
- Golden: the 6-event fixture with expected value **3**, taken from `TASK.md`
  behavior 4 ("how many of them are `severity="critical"`, whether or not they
  are retrospective") — an independent literal, not a recomputation of the
  implementation. It is discriminating: for that same list, "count only what
  alerts" gives 1, "count everything" 6, "retrospective only" 4,
  "non-retrospective only" 2, and "critical or warning" 4. This is the one
  behavior in the task where the obvious wrong implementation (reusing
  `should_alert` as the filter) is a plausible reading, so the golden was
  chosen to separate exactly those two rules.

### Row 5 — `should_alert` accepts an attribute-style event

- Seam authority: S1, `TASK.md` event definition ("a simple object/dict"), per
  decision D2.
- Test file / selector:
  `test_alerting.py::test_should_alert_reads_attribute_style_events`.
- Observed public entry point: `alerting.should_alert`.
- Red: `K-focus(test_should_alert_reads_attribute_style_events)`. Intended
  failure — the implementation subscripts the event, so an object shape cannot
  work. Observed — `TypeError: 'types.SimpleNamespace' object is not
  subscriptable` (1 failed). Match.
- Green: same command. Result — 1 passed. Minimal implementation: a `_field`
  accessor branching on `collections.abc.Mapping`, wired into `should_alert`
  **only**. Regression: `K-suite` — 5 passed.

### Row 6 — `escalation_count` accepts attribute-style events

- Seam authority: S2, `TASK.md` event definition, per decision D2.
- Test file / selector:
  `test_alerting.py::test_escalation_count_reads_attribute_style_events`.
- Observed public entry point: `alerting.escalation_count`.
- Red: `K-focus(test_escalation_count_reads_attribute_style_events)`. Intended
  failure — row 5 wired `_field` into `should_alert` only, so the counter still
  subscripts. Observed — `TypeError: 'types.SimpleNamespace' object is not
  subscriptable` at `alerting.py:22`, the counter line (1 failed). Match. Row 5
  was deliberately kept narrow so this cycle would have a real red instead of
  being green on arrival.
- Green: same command. Result — 1 passed. Minimal implementation: `_field` in
  `escalation_count` too. Regression: `K-suite` — 6 passed.

## Sensitivity check on the goldens

`skill/tests.md` requires that a golden fail once the forbidden behavior is
substituted. I verified this rather than asserting it, by copying the module
and suite into throwaway `mutation-check/m1` and `mutation-check/m2`
directories (both deleted afterwards; the working directory now holds only
`NOTES.md`, `TASK.md`, `alerting.py`, `test_alerting.py`, and `skill/`).

| Mutant | Forbidden behavior substituted | Result |
| --- | --- | --- |
| M1 | `escalation_count` filters with `should_alert` | 2 failed, 4 passed — killed by `test_escalation_count_counts_retrospective_criticals_too` and `test_escalation_count_reads_attribute_style_events` |
| M2 | `should_alert` ignores `retrospective` | 3 failed, 3 passed — killed by `test_retrospective_critical_event_does_not_alert`, `test_should_alert_reads_attribute_style_events`, and (via its rejected-alternative assertion) `test_escalation_count_counts_retrospective_criticals_too` |

Both mutants died, so neither golden passes by construction.

## Pre-review reconciliation (tests to seams)

Inventory derived mechanically from the native test structure via `K-collect`,
not from memory. Six collected node IDs, all in
`$BASE/test_alerting.py`.

| # | Selector | Observed public entry point | Seam | Authority | Evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `test_critical_event_alerts` | `alerting.should_alert` | S1 | `TASK.md` behavior 1 | Row 1 |
| 2 | `test_info_event_does_not_alert` | `alerting.should_alert` | S1 | `TASK.md` behavior 2 | Row 2 |
| 3 | `test_retrospective_critical_event_does_not_alert` | `alerting.should_alert` | S1 | `TASK.md` behavior 3 | Row 3 |
| 4 | `test_escalation_count_counts_retrospective_criticals_too` | `alerting.escalation_count` (+ `alerting.should_alert`) | S2 (+ S1) | `TASK.md` behavior 4 | Row 4 |
| 5 | `test_should_alert_reads_attribute_style_events` | `alerting.should_alert` | S1 | `TASK.md` event definition (D2) | Row 5 |
| 6 | `test_escalation_count_reads_attribute_style_events` | `alerting.escalation_count` | S2 | `TASK.md` event definition (D2) | Row 6 |

Both-directions comparison:

- Collected selectors not present in the table: **none**.
- Table rows not present in the collected inventory: **none**.

Both differences are empty, so the gate passes. No test enters through a
module-private helper, a mutable global, or a side channel: `_field` is private
and is exercised only through the two public functions, and no test asserts on
it directly. The `event(...)` builder in the test file is test-local data
construction, not a production seam.

Final full-suite state: `K-suite` — **6 passed**.

## What I would flag to a reviewer

- The `"warning"` severity has no pinned behavior (D1). It is the only legal
  input whose verdict rests on my judgment rather than on `TASK.md`.
- Row 4's test carries a second assertion
  (`sum(1 for e in events if should_alert(e)) == 1`) that touches the S1 seam
  from an S2 test. I kept it because it is what makes the golden's
  discrimination visible in the test rather than only in this document, but it
  is the one place where a reviewer could reasonably ask for a split.

---

# Chronological command log

`$BASE` =
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/b0fd637a-3121-48c1-b47e-a6e9d2129c01/scratchpad/runs/T1-current-run1`.
Every pytest invocation below is listed in the order it was run. File
reads/writes/edits are noted inline so the sequence is unambiguous, but only
the shell commands are numbered.

1. `python3 --version; python3 -m pytest --version` — **passed** (Python 3.14.4,
   pytest 9.0.2).
2. `ls -la $BASE $BASE/skill/` — **passed** (listed `TASK.md`, `skill/SKILL.md`,
   `skill/tests.md`, `skill/mocking.md`).

   *Then, no command: read `TASK.md`, `skill/SKILL.md`, `skill/tests.md`,
   `skill/mocking.md`; wrote `NOTES.md` with the seams and decisions; wrote
   `test_alerting.py` containing only `test_critical_event_alerts`.*

3. `python3 -m pytest "$BASE/test_alerting.py::test_critical_event_alerts" -q`
   — **FAILED (intended red, cycle 1)**: 1 error,
   `ModuleNotFoundError: No module named 'alerting'`.

   *No command: created `alerting.py` with `should_alert` returning `True`.*

4. `python3 -m pytest "$BASE/test_alerting.py::test_critical_event_alerts" -q`
   — **passed** (1 passed). Cycle 1 green.

   *No command: added `test_info_event_does_not_alert` to `test_alerting.py`.*

5. `python3 -m pytest "$BASE/test_alerting.py::test_info_event_does_not_alert" -q`
   — **FAILED (intended red, cycle 2)**: 1 failed,
   `AssertionError: assert True is False`.

   *No command: changed `should_alert` to `return event["severity"] == "critical"`.*

6. `python3 -m pytest "$BASE/test_alerting.py::test_info_event_does_not_alert" -q`
   — **passed** (1 passed). Cycle 2 green.
7. `python3 -m pytest $BASE/test_alerting.py -q` — **passed** (2 passed).
   Regression after cycle 2.

   *No command: added `test_retrospective_critical_event_does_not_alert`.*

8. `python3 -m pytest "$BASE/test_alerting.py::test_retrospective_critical_event_does_not_alert" -q`
   — **FAILED (intended red, cycle 3)**: 1 failed,
   `AssertionError: assert True is False`.

   *No command: added the `retrospective` early return to `should_alert`.*

9. `python3 -m pytest "$BASE/test_alerting.py::test_retrospective_critical_event_does_not_alert" -q`
   — **passed** (1 passed). Cycle 3 green.
   Run in the same shell invocation as command 10.
10. `python3 -m pytest $BASE/test_alerting.py -q` — **passed** (3 passed).
    Regression after cycle 3.

    *No command: added `test_escalation_count_counts_retrospective_criticals_too`
    and extended the import to `from alerting import escalation_count, should_alert`.*

11. `python3 -m pytest "$BASE/test_alerting.py::test_escalation_count_counts_retrospective_criticals_too" -q`
    — **FAILED (intended red, cycle 4)**: 1 error,
    `ImportError: cannot import name 'escalation_count' from 'alerting'`.

    *No command: added `escalation_count` to `alerting.py`.*

12. `python3 -m pytest "$BASE/test_alerting.py::test_escalation_count_counts_retrospective_criticals_too" -q`
    — **passed** (1 passed). Cycle 4 green. Same shell invocation as 13.
13. `python3 -m pytest $BASE/test_alerting.py -q` — **passed** (4 passed).
    Regression after cycle 4.

    *No command: added `test_should_alert_reads_attribute_style_events` and the
    `from types import SimpleNamespace` import.*

14. `python3 -m pytest "$BASE/test_alerting.py::test_should_alert_reads_attribute_style_events" -q`
    — **FAILED (intended red, cycle 5)**:
    1 failed, `TypeError: 'types.SimpleNamespace' object is not subscriptable`
    raised at `alerting.py:6` (inside `should_alert`).

    *No command: added the private `_field` accessor and used it in
    `should_alert` only.*

15. `python3 -m pytest "$BASE/test_alerting.py::test_should_alert_reads_attribute_style_events" -q`
    — **passed** (1 passed). Cycle 5 green. Same shell invocation as 16.
16. `python3 -m pytest $BASE/test_alerting.py -q` — **passed** (5 passed).
    Regression after cycle 5.

    *No command: added `test_escalation_count_reads_attribute_style_events`.*

17. `python3 -m pytest "$BASE/test_alerting.py::test_escalation_count_reads_attribute_style_events" -q`
    — **FAILED (intended red, cycle 6)**:
    1 failed, `TypeError: 'types.SimpleNamespace' object is not subscriptable`
    raised at `alerting.py:22` (inside `escalation_count`) — a different line
    from the cycle-5 red, confirming this was a genuine new red and not a
    repeat of the previous one.

    *No command: used `_field` in `escalation_count`.*

18. `python3 -m pytest "$BASE/test_alerting.py::test_escalation_count_reads_attribute_style_events" -q`
    — **passed** (1 passed). Cycle 6 green. Same shell invocation as 19.
19. `python3 -m pytest $BASE/test_alerting.py -q` — **passed** (6 passed).
    All six behaviors green.
20. `mkdir`/`cp`/`sed` block building `mutation-check/m1` (escalation_count
    filters with `should_alert`) and `mutation-check/m2` (retrospective guard
    deleted), plus two `grep` checks — **passed** (both mutations confirmed
    applied: `return sum(1 for event in events if should_alert(event))` in m1;
    no `retrospective` guard left in m2).
21. `python3 -m pytest $BASE/mutation-check/m1/test_alerting.py -q` —
    **FAILED, as required**: 2 failed, 4 passed. Killed by
    `test_escalation_count_counts_retrospective_criticals_too` and
    `test_escalation_count_reads_attribute_style_events`.
22. `python3 -m pytest $BASE/mutation-check/m2/test_alerting.py -q` —
    **FAILED, as required**: 3 failed, 3 passed. Killed by
    `test_retrospective_critical_event_does_not_alert`,
    `test_escalation_count_counts_retrospective_criticals_too`, and
    `test_should_alert_reads_attribute_style_events`.
    Commands 21 and 22 ran in one shell invocation.
23. `rm -rf $BASE/mutation-check` then
    `python3 -m pytest $BASE/test_alerting.py --collect-only -q` then
    `ls -1 $BASE` — **passed** (6 tests collected; mutants gone; a stray
    `__pycache__` was visible).
24. `rm -rf $BASE/__pycache__ $BASE/.pytest_cache` then
    `python3 -m pytest $BASE/test_alerting.py -q -p no:cacheprovider` then
    `ls -1a $BASE` — **passed** (6 passed; `__pycache__` regenerated by the run).
25. `rm -rf $BASE/__pycache__; ls -1a $BASE; cat $BASE/alerting.py` —
    **passed** (working directory left holding only `NOTES.md`, `TASK.md`,
    `alerting.py`, `skill/`, `test_alerting.py`).

    *Between commands 25 and 26, no test runs: that gap is where the evidence
    rows, the reconciliation table, and this log were written into `NOTES.md`.*

26. `python3 -m pytest $BASE/test_alerting.py -q -p no:cacheprovider` then
    `rm -rf $BASE/__pycache__; ls -1a $BASE` — **passed** (6 passed). Final
    confirmation after all writing was done, leaving the working directory at
    `NOTES.md`, `TASK.md`, `alerting.py`, `skill/`, `test_alerting.py`.

## Summary of failures

Every failing run above was a deliberate red or a mutation probe. In sequence:
commands 3, 5, 8, 11, 14, 17 are the six intended reds (one per cycle), and
commands 21 and 22 are the two mutants that were supposed to die. No unplanned
failure occurred at any point, and no green run ever regressed.
