# TDD record — alert router

Working directory: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/b0fd637a-3121-48c1-b47e-a6e9d2129c01/scratchpad/runs/T1-current-run2`

## Command keys (replayable)

All commands run from the working directory above.

- `K-ENV` = `python3 --version && python3 -m pytest --version`
- `K1` = `python3 -m pytest test_alerting.py::test_critical_event_alerts`
- `K2` = `python3 -m pytest test_alerting.py::test_info_event_does_not_alert`
- `K3` = `python3 -m pytest test_alerting.py::test_retrospective_critical_event_does_not_alert`
- `K4` = `python3 -m pytest test_alerting.py::test_escalation_count_counts_criticals_including_retrospective`
- `K-ALL` = `python3 -m pytest`
- `K-INV` = `python3 -m pytest --collect-only -q`

## Seams under test, and their authority

One seam: the public module `alerting.py` and its two exported functions
`should_alert(event)` and `escalation_count(events)`.

Authority: `TASK.md` in this working directory. It names the module and both
function signatures verbatim and lists the four acceptance behaviors in order,
so the seam is ratified by the task spec (the standing-in PRD/issue). No seam
was ambiguous, so no user question was required. Both functions are called
directly as the observed public entry point; nothing reaches into internals,
and nothing is mocked (there are no system boundaries in this module — no I/O,
clock, or network).

## Decisions made without a user (no user was available to ask)

- **D1 — Seam authority.** Treated `TASK.md` as the authoritative spec that
  ratifies the seam, rather than pausing to confirm.
- **D2 — Event representation.** `TASK.md` says an event is "a simple
  object/dict", which is ambiguous. Chose **dict (mapping) access**
  (`event["severity"]`). Tests construct dicts; the implementation reads keys.
  Deliberately did *not* add dual attribute/mapping support — that would be a
  speculative feature no test drove.
- **D3 — `severity="warning"` when not retrospective.** `TASK.md` fixes only
  critical (alerts) and info (does not). Warning's alerting behavior is
  unspecified, so **no test asserts it** — testing it would pin imagined
  behavior. The minimal implementation that satisfies behaviors 1–3 happens to
  leave non-retrospective warnings non-alerting; that is an implementation
  consequence, not a ratified claim.
- **D4 — Changed-test inventory source.** This working directory is not a git
  repository, so there is no literal scoped diff. Every file here other than
  `TASK.md` and `skill/` is new in this change, so the whole-suite collection
  (`K-INV`) *is* the changed-test inventory, exactly and without omission.
- **D5 — Sensitivity probe location.** The mutation/sensitivity check ran on a
  throwaway copy in a sibling temp directory so the deliverable files were never
  mutated; that copy was deleted afterwards. The deliverable was re-run
  afterwards to confirm it was untouched (4 passed).

## TDD evidence rows

### Row 1 — behavior 1: a critical event alerts

- Seam authority: `TASK.md` acceptance behavior 1.
- Test file / selector: `test_alerting.py::test_critical_event_alerts`.
- Observed public entry point: `alerting.should_alert(event)`.
- Red command: `K1`. Intended failure: the module does not exist yet, so import
  fails. Observed failure: collection ERROR —
  `ModuleNotFoundError: No module named 'alerting'` (1 error).
- Green: minimal implementation `should_alert` returning `True`
  (deliberate fake-it; triangulated away by Row 2). Command `K1` → 1 passed.

### Row 2 — behavior 2: an info event does not alert

- Seam authority: `TASK.md` acceptance behavior 2.
- Test file / selector: `test_alerting.py::test_info_event_does_not_alert`.
- Observed public entry point: `alerting.should_alert(event)`.
- Red command: `K2`. Intended failure: the fake-it `return True` from Row 1
  returns True for an info event. Observed failure:
  `AssertionError: assert True is False`, where
  `True = should_alert({'retrospective': False, 'severity': 'info', 'source': 'payments-api'})`
  (1 failed).
- Green: `should_alert` becomes `return event["severity"] == "critical"`.
  Command `K2` → 1 passed.

### Row 3 — behavior 3: a retrospective event never alerts

- Seam authority: `TASK.md` acceptance behavior 3.
- Test file / selector:
  `test_alerting.py::test_retrospective_critical_event_does_not_alert`.
- Observed public entry point: `alerting.should_alert(event)`.
- Red command: `K3`. Intended failure: the severity-only rule from Row 2 still
  alerts on a retrospective critical. Observed failure:
  `AssertionError: assert True is False`, where
  `True = should_alert({'retrospective': True, 'severity': 'critical', 'source': 'payments-api'})`
  (1 failed).
- Green: added the guard `if event["retrospective"]: return False` ahead of the
  severity check. Command `K3` → 1 passed.
- Note: the case chosen is a retrospective **critical**, which is the only
  discriminating one — a retrospective info would already pass under Row 2's
  implementation and so would prove nothing.

### Row 4 — behavior 4: retrospective events still count toward escalation_count

- Seam authority: `TASK.md` acceptance behavior 4.
- Test file / selector:
  `test_alerting.py::test_escalation_count_counts_criticals_including_retrospective`.
- Observed public entry point: `alerting.escalation_count(events)`.
- Red command: `K4`. Intended failure: `escalation_count` is not exported yet.
  Observed failure: collection ERROR —
  `ImportError: cannot import name 'escalation_count' from 'alerting'`
  (1 error).
- Green: added
  `escalation_count = sum(1 for event in events if event["severity"] == "critical")`.
  Command `K4` → 1 passed.
- Discriminating golden: the fixture is a hand-counted 5-event list whose
  required answer is **3**, chosen so every plausible rejected rule gives a
  different number — "criticals that would alert" → 1, "criticals excluding
  retrospective" → 1, "any retrospective event" → 4, "anything above info" → 4,
  "every event" → 5. The expected value is an independent literal counted from
  the fixture by hand, not recomputed the way the implementation computes it.

## Pre-review reconciliation (changed-test inventory vs. evidence rows)

Inventory derived from `K-INV` (see D4). Four selectors collected:

| # | Test file | Exact selector | Observed public entry point | Seam authority | TDD evidence row |
|---|-----------|----------------|-----------------------------|----------------|------------------|
| 1 | `test_alerting.py` | `test_critical_event_alerts` | `alerting.should_alert` | `TASK.md` behavior 1 | Row 1 |
| 2 | `test_alerting.py` | `test_info_event_does_not_alert` | `alerting.should_alert` | `TASK.md` behavior 2 | Row 2 |
| 3 | `test_alerting.py` | `test_retrospective_critical_event_does_not_alert` | `alerting.should_alert` | `TASK.md` behavior 3 | Row 3 |
| 4 | `test_alerting.py` | `test_escalation_count_counts_criticals_including_retrospective` | `alerting.escalation_count` | `TASK.md` behavior 4 | Row 4 |

Two-way comparison:

- Inventory selectors not present in the reconciliation rows: **none**.
- Reconciliation rows not present in the inventory: **none**.

Both differences are empty. Every test enters through the module's ratified
public functions; none tests a private invariant, a helper, a mutable global, or
any side channel.

Sensitivity check (each golden must fail if the forbidden behavior is
substituted) — run against a throwaway copy, three mutants:

- M1 `escalation_count` skips retrospective criticals → **Row 4's test failed**,
  the other three passed.
- M2 `should_alert` drops the retrospective guard → **Row 3's test failed**, the
  other three passed.
- M3 `should_alert` always returns True → **Rows 2 and 3's tests failed**, the
  other two passed.

Each mutant is caught by exactly the row that owns the behavior, so no golden
passes by construction.

## Deliverables

- `alerting.py` — `should_alert(event)`, `escalation_count(events)`.
- `test_alerting.py` — the four tests above.
- Final state: `K-ALL` → **4 passed**.

---

# Chronological command log

Every command run, in order. Commands are shown as run (from the working
directory unless an absolute path is given).

1. `python3 --version && python3 -m pytest --version && git rev-parse --is-inside-work-tree`
   — **passed** for the version probes: Python 3.14.4, pytest 9.0.2. The
   `git rev-parse` part printed `fatal: not a git repository (or any of the
   parent directories): .git` — expected and informational only (see D4); it did
   not block anything.
2. *(file write, not a command)* Created `test_alerting.py` with the Row 1 test
   only.
3. `python3 -m pytest test_alerting.py::test_critical_event_alerts` (`K1`)
   — **FAILED as intended (red)**: 1 error, collection ImportError,
   `ModuleNotFoundError: No module named 'alerting'`.
4. *(file write)* Created `alerting.py` with `should_alert` returning `True`.
5. `python3 -m pytest test_alerting.py::test_critical_event_alerts` (`K1`)
   — **passed**: 1 passed (`test_critical_event_alerts`).
6. *(file edit)* Appended `test_info_event_does_not_alert` to `test_alerting.py`.
7. `python3 -m pytest test_alerting.py::test_info_event_does_not_alert` (`K2`)
   — **FAILED as intended (red)**: 1 failed
   (`test_info_event_does_not_alert`), `AssertionError: assert True is False`.
8. *(file edit)* `should_alert` → `return event["severity"] == "critical"`.
9. `python3 -m pytest test_alerting.py::test_info_event_does_not_alert` (`K2`)
   — **passed**: 1 passed (`test_info_event_does_not_alert`).
10. *(file edit)* Appended `test_retrospective_critical_event_does_not_alert` to
    `test_alerting.py`.
11. `python3 -m pytest test_alerting.py::test_retrospective_critical_event_does_not_alert`
    (`K3`) — **FAILED as intended (red)**: 1 failed
    (`test_retrospective_critical_event_does_not_alert`),
    `AssertionError: assert True is False`.
12. *(file edit)* Added the `if event["retrospective"]: return False` guard to
    `should_alert`.
13. `python3 -m pytest test_alerting.py::test_retrospective_critical_event_does_not_alert`
    (`K3`) — **passed**: 1 passed
    (`test_retrospective_critical_event_does_not_alert`).
14. *(file edits, two)* Changed the import line to
    `from alerting import escalation_count, should_alert`, then appended
    `test_escalation_count_counts_criticals_including_retrospective`.
15. `python3 -m pytest test_alerting.py::test_escalation_count_counts_criticals_including_retrospective`
    (`K4`) — **FAILED as intended (red)**: 1 error, collection
    `ImportError: cannot import name 'escalation_count' from 'alerting'`.
    (This error also blocked the three previously green tests for the duration
    of the red step, since the failure is at module import.)
16. *(file edit)* Added `escalation_count` to `alerting.py`.
17. `python3 -m pytest test_alerting.py::test_escalation_count_counts_criticals_including_retrospective`
    (`K4`) — **passed**: 1 passed
    (`test_escalation_count_counts_criticals_including_retrospective`).
18. `python3 -m pytest` (`K-ALL`) — **passed**: 4 passed (all four selectors).
    In the same shell call, `python3 -m pytest --collect-only -q` (`K-INV`)
    — **passed**: 4 tests collected, listing the four selectors used in the
    reconciliation table.
19. Sensitivity check, one shell call over a throwaway copy in a sibling temp
    directory (`scratchpad/mutation-check`), copying `alerting.py` and
    `test_alerting.py` there and restoring the pristine `alerting.py` between
    mutants:
    - M1 (`escalation_count` skips retrospective criticals), `python3 -m pytest <mutant dir>`
      — **1 failed, 3 passed**; the failure was
      `test_escalation_count_counts_criticals_including_retrospective`.
    - M2 (`should_alert` retrospective guard deleted), `python3 -m pytest <mutant dir>`
      — **1 failed, 3 passed**; the failure was
      `test_retrospective_critical_event_does_not_alert`.
    - M3 (`should_alert` always returns True), `python3 -m pytest <mutant dir>`
      — **2 failed, 2 passed**; the failures were
      `test_info_event_does_not_alert` and
      `test_retrospective_critical_event_does_not_alert`.
    - Final step of the same call, `python3 -m pytest <working directory>`
      — **passed**: 4 passed, confirming the deliverable was never mutated.
20. `rm -rf <scratchpad>/mutation-check && ls <working directory>`
    — **passed**: temp copy removed; working directory contains `TASK.md`,
    `__pycache__`, `alerting.py`, `skill`, `test_alerting.py`.
21. *(file write)* Created this `NOTES.md`.

No command failed unexpectedly. The four failures at steps 3, 7, 11 and 15 were
the intended red steps of their cycles; every other run passed.
