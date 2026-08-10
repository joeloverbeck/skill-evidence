# TDD record — alert router

Working directory (`WD`):
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/b0fd637a-3121-48c1-b47e-a6e9d2129c01/scratchpad/runs/T1-current-run4`

## Seam agreement (written before any test)

| # | Seam under test | Authority |
|---|---|---|
| S1 | `alerting.should_alert(event) -> bool` | `TASK.md` — names the module and the exported function, and fixes acceptance behaviors 1–3 |
| S2 | `alerting.escalation_count(events) -> int` | `TASK.md` — names the module and the exported function, and fixes acceptance behavior 4 |

`TASK.md` is the authoritative task document for this work; it ratifies both seams
explicitly (module path + function signatures), so no seam is ambiguous and no seam
question needed to go to a user. No test is written at any other boundary: the tests
import only `alerting` and call only these two functions. Nothing private is touched,
nothing is mocked (there is no system boundary here — no I/O, clock, or network).

## Decisions taken without a user (no user was available to confirm)

1. **Event representation = dict.** `TASK.md` says an event is "a simple object/dict"
   and so permits either. I chose plain dicts with keys `severity`, `retrospective`,
   `source`, because it needs no extra exported constructor and therefore keeps the
   public surface exactly the two functions the task asked for. Recorded here rather
   than asked.
2. **`warning` severity alerting is left unspecified.** The task fixes only
   critical (alerts) and info (does not alert). I wrote no test for a
   non-retrospective `warning`, because testing unspecified behavior would pin
   behavior no authority ratified. Under the implementation it does not alert; that
   is a consequence of the minimal implementation, not a ratified requirement.
3. **`source` is carried in fixtures but never asserted on.** No acceptance behavior
   makes routing depend on it, so no test claims anything about it.
4. **Scoped diff for the pre-review reconciliation gate.** This working directory is
   not a git repository (`git rev-parse --is-inside-work-tree` → fatal: not a git
   repository), so there is no commit to diff against. The scoped diff is therefore
   the whole of the newly authored `test_alerting.py`, and the changed-test inventory
   is derived from pytest's own collection (`--collect-only -q`), which is the
   repository's native test structure here.
5. **No `/code-review` pass was requested by the task.** I ran the reconciliation gate
   that this skill owns and that must precede review; the review itself is out of scope
   for "deliver working code and tests", and there is no committed change to hand over.

## TDD evidence

Command keys (each is a complete, replayable command; `cd` avoided by using absolute paths):

- `CMD_S1_C1` = `python3 -m pytest "$WD/test_alerting.py::test_critical_event_alerts" -q`
- `CMD_S1_C2` = `python3 -m pytest "$WD/test_alerting.py::test_info_event_does_not_alert" -q`
- `CMD_S1_C3` = `python3 -m pytest "$WD/test_alerting.py::test_retrospective_critical_event_does_not_alert" -q`
- `CMD_S2_C4` = `python3 -m pytest "$WD/test_alerting.py::test_escalation_count_includes_retrospective_criticals" -q`
- `CMD_ALL` = `python3 -m pytest "$WD" -q`

(where `WD` is the working directory above; the runs below used it expanded in full)

### Row 1 — critical alerts (acceptance behavior 1)

- **Seam authority:** `TASK.md` behavior 1, seam S1
- **Test file / selector:** `test_alerting.py::test_critical_event_alerts`
- **Observed public entry point:** `alerting.should_alert(event)` — imported from the
  module, called with a dict; nothing else is touched
- **Red command:** `CMD_S1_C1`
- **Intended red:** no `alerting` module exists yet, so the import fails
- **Observed red:** `ModuleNotFoundError: No module named 'alerting'` → `1 error` —
  matches the intended failure
- **Minimal implementation:** created `alerting.py` with `should_alert` returning `True`
  (deliberately degenerate — nothing observed so far distinguishes more)
- **Green command:** `CMD_S1_C1`
- **Green result:** `1 passed`

### Row 2 — info does not alert (acceptance behavior 2)

- **Seam authority:** `TASK.md` behavior 2, seam S1
- **Test file / selector:** `test_alerting.py::test_info_event_does_not_alert`
- **Observed public entry point:** `alerting.should_alert(event)`
- **Red command:** `CMD_S1_C2`
- **Intended red:** the blanket-`True` router alerts on an `info` event
- **Observed red:** `AssertionError: assert True is False` where
  `True = should_alert({'retrospective': False, 'severity': 'info', 'source': 'checkout-api'})`
  → `1 failed` — matches the intended failure
- **Minimal implementation:** `return event["severity"] == "critical"`
- **Green command:** `CMD_S1_C2`
- **Green result:** `1 passed`

### Row 3 — retrospective never alerts (acceptance behavior 3)

- **Seam authority:** `TASK.md` behavior 3, seam S1
- **Test file / selector:** `test_alerting.py::test_retrospective_critical_event_does_not_alert`
- **Observed public entry point:** `alerting.should_alert(event)`
- **Red command:** `CMD_S1_C3`
- **Intended red:** the severity-only router still alerts on a *retrospective critical*
- **Observed red:** `AssertionError: assert True is False` where
  `True = should_alert({'retrospective': True, 'severity': 'critical', 'source': 'checkout-api'})`
  → `1 failed` — matches the intended failure
- **Minimal implementation:** early `if event["retrospective"]: return False`
- **Green command:** `CMD_S1_C3`
- **Green result:** `1 passed`
- **Golden discrimination:** `critical` is the only severity that discriminates here.
  A retrospective `info` event returns `False` under the rejected severity-only
  implementation as well, so such a test would pass by construction and prove nothing.

### Row 4 — retrospective criticals still count (acceptance behavior 4)

- **Seam authority:** `TASK.md` behavior 4, seam S2
- **Test file / selector:** `test_alerting.py::test_escalation_count_includes_retrospective_criticals`
- **Observed public entry point:** `alerting.escalation_count(events)`
- **Red command:** `CMD_S2_C4`
- **Intended red:** `escalation_count` is not exported yet
- **Observed red:** `ImportError: cannot import name 'escalation_count' from 'alerting'`
  → `1 error` — matches the intended failure
- **Minimal implementation:** `sum(1 for event in events if event["severity"] == "critical")`,
  written independently of `should_alert` because the counting rule is not the alerting rule
- **Green command:** `CMD_S2_C4`
- **Green result:** `1 passed`
- **Golden discrimination:** the fixture is 6 events — 4 critical (2 of them
  retrospective), 1 non-retrospective warning, 1 retrospective info. The required
  answer 4 was counted by hand from the list against the rule in `TASK.md`, and it
  differs from every plausible rejected rule: count all events → 6; count only
  alerting events → 2; count retrospective → 3; count non-retrospective → 3; count
  non-critical → 2.

### Sensitivity check (mutation probe)

Ran the whole suite against four throwaway mutant implementations in a temporary
`.sensitivity/` copy (removed afterwards; the real `alerting.py` was never edited for
this). Each forbidden behavior is caught by the test that claims it:

| Mutant | Result |
|---|---|
| `should_alert` ignores `retrospective` (severity-only) | `1 failed, 3 passed` — `test_retrospective_critical_event_does_not_alert` |
| `escalation_count` counts only events that alert | `1 failed, 3 passed` — `test_escalation_count_includes_retrospective_criticals` |
| `escalation_count` counts every event (`len`) | `1 failed, 3 passed` — `test_escalation_count_includes_retrospective_criticals` |
| `should_alert` returns `True` always | `2 failed, 2 passed` — the info and retrospective tests |

No golden survives substitution of the behavior it is supposed to pin.

## Pre-review reconciliation

Changed-test inventory derived from the scoped diff (see decision 4 — the scoped diff
is all of the new `test_alerting.py`), using pytest's own collection as the native test
structure. Command:
`python3 -m pytest "$WD" --collect-only -q`

Inventory (4 selectors, verbatim from collection):

1. `test_alerting.py::test_critical_event_alerts`
2. `test_alerting.py::test_info_event_does_not_alert`
3. `test_alerting.py::test_retrospective_critical_event_does_not_alert`
4. `test_alerting.py::test_escalation_count_includes_retrospective_criticals`

Reconciliation rows, one per changed selector:

| Test file | Selector | Observed public entry point | Seam authority | Evidence row |
|---|---|---|---|---|
| `test_alerting.py` | `test_critical_event_alerts` | `alerting.should_alert` | `TASK.md` behavior 1 (seam S1) | Row 1 |
| `test_alerting.py` | `test_info_event_does_not_alert` | `alerting.should_alert` | `TASK.md` behavior 2 (seam S1) | Row 2 |
| `test_alerting.py` | `test_retrospective_critical_event_does_not_alert` | `alerting.should_alert` | `TASK.md` behavior 3 (seam S1) | Row 3 |
| `test_alerting.py` | `test_escalation_count_includes_retrospective_criticals` | `alerting.escalation_count` | `TASK.md` behavior 4 (seam S2) | Row 4 |

**Both-direction difference check:**

- inventory identities not present in the reconciliation rows: *(empty)*
- reconciliation identities not present in the inventory: *(empty)*

Both differences are empty, so the gate passes. Every row enters through one of the two
functions `TASK.md` ratified — no test enters through a module-private helper, a mutable
global, a side channel, or any other unratified boundary. The only non-exported thing a
test touches is its own local `event(...)` fixture builder, which lives in the test file
and constructs an input; it is not a boundary into the code under test. No mocks are
used, and there is no system boundary here that would justify one.

Coverage check before reconciliation: each of the four acceptance behaviors in `TASK.md`
has exactly one row proving it at the public interface, and no row substitutes a private
invariant for the public behavior it claims.

## Chronological command log

Every command run, in order. `WD` is the working directory named at the top; the actual
runs used the fully expanded absolute path.

| # | Command | Outcome |
|---|---|---|
| 1 | `python3 --version` / `python3 -m pytest --version` / `git -C "$WD" rev-parse --is-inside-work-tree` | Python 3.14.4, pytest 9.0.2; git → **fatal: not a git repository** (expected — drove decision 4) |
| 2 | `python3 -m pytest "$WD/test_alerting.py::test_critical_event_alerts" -q` (`CMD_S1_C1`) | **FAILED — 1 error**, `ModuleNotFoundError: No module named 'alerting'` (intended red, cycle 1) |
| 3 | `python3 -m pytest "$WD/test_alerting.py::test_critical_event_alerts" -q` (`CMD_S1_C1`) | **1 passed** (green, cycle 1) |
| 4 | `python3 -m pytest "$WD/test_alerting.py::test_info_event_does_not_alert" -q` (`CMD_S1_C2`) | **FAILED — 1 failed**, `AssertionError: assert True is False` (intended red, cycle 2) |
| 5 | `python3 -m pytest "$WD/test_alerting.py::test_info_event_does_not_alert" -q` (`CMD_S1_C2`) | **1 passed** (green, cycle 2) |
| 6 | `python3 -m pytest "$WD/test_alerting.py::test_retrospective_critical_event_does_not_alert" -q` (`CMD_S1_C3`) | **FAILED — 1 failed**, `AssertionError: assert True is False` on a retrospective critical (intended red, cycle 3) |
| 7 | `python3 -m pytest "$WD/test_alerting.py::test_retrospective_critical_event_does_not_alert" -q` (`CMD_S1_C3`) | **1 passed** (green, cycle 3) |
| 8 | `python3 -m pytest "$WD/test_alerting.py::test_escalation_count_includes_retrospective_criticals" -q` (`CMD_S2_C4`) | **FAILED — 1 error**, `ImportError: cannot import name 'escalation_count' from 'alerting'` (intended red, cycle 4) |
| 9 | `python3 -m pytest "$WD/test_alerting.py::test_escalation_count_includes_retrospective_criticals" -q` (`CMD_S2_C4`) | **1 passed** (green, cycle 4) |
| 10 | `python3 -m pytest . -q` then `python3 -m pytest . --collect-only -q` (run from `WD`) | **4 passed**; collection listed the 4 selectors used as the reconciliation inventory |
| 11 | Sensitivity probe: 4 × `python3 -m pytest "$WD/.sensitivity" -q` against mutant `alerting.py` copies | mutant 1 **1 failed, 3 passed**; mutant 2 **1 failed, 3 passed**; mutant 3 **1 failed, 3 passed**; mutant 4 **2 failed, 2 passed** — all mutants caught, as intended; `.sensitivity/` deleted afterwards |
| 12 | `python3 -m pytest "$WD" -q` (`CMD_ALL`), plus `rm -rf` of `.pytest_cache`/`__pycache__` | **4 passed** (final state) |

Summary of failures: the only failing runs were the four intended reds (#2, #4, #6, #8)
and the four deliberate mutant probes (#11). Every green run passed on the first attempt;
no test needed rewriting after it was written, and no production code was written before
its failing test.

## Deliverables

- `alerting.py` — `should_alert(event)`, `escalation_count(events)`
- `test_alerting.py` — 4 behavior tests, all passing
