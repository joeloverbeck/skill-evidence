# TDD record — alert router

Working directory (all paths absolute):
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/b0fd637a-3121-48c1-b47e-a6e9d2129c01/scratchpad/runs/T1-current-run3`

Abbreviated below as `<WD>`. **Every command in this file was run with `<WD>` expanded to
exactly that absolute path** — expand it and the command replays as written.

Deliverables: `<WD>/alerting.py`, `<WD>/test_alerting.py`.

---

## 1. Seams under test, and their authority

Written down before any test was authored, as the skill requires.

| Seam | Public entry point | Authority |
| --- | --- | --- |
| S1 | `alerting.should_alert(event) -> bool` | `<WD>/TASK.md` — names the module and the signature, and lists acceptance behaviors 1–3 against it. |
| S2 | `alerting.escalation_count(events) -> int` | `<WD>/TASK.md` — names the signature and acceptance behavior 4. |

No other seam is under test. No test touches a module-private name, a helper, a mutable
global, or a side channel. `alerting._field` is private and is observed only indirectly,
through S1 and S2. The test module's entire import surface is
`from alerting import escalation_count, should_alert`, which is exactly the ratified
surface.

### Decisions I made without a user (no user was available to confirm)

1. **`TASK.md` is the ratifying authority for the seams.** The skill says to ask the user
   when a seam is absent or ambiguous. `TASK.md` names the module and both exported
   signatures explicitly, so nothing was ambiguous and no question was warranted. I treat
   `TASK.md` the way the skill treats an authoritative PRD.
2. **Event representation: both mappings and attribute-style objects are supported.**
   `TASK.md` says an event is "a simple object/dict with fields", naming two shapes, and it
   exports no constructor I could hand callers. Rather than silently pick one shape and
   break the other, I drove both from tests (cycles 5 and 6). This is spec text, not
   speculation: no field, option, or code path exists that `TASK.md` does not mention.
3. **`severity="warning"` is left unspecified and deliberately untested.** See section 5 —
   this one has a measured consequence, not just a note.
4. **`source` carries no behavior.** `TASK.md` lists the field but attaches no acceptance
   behavior to it. No code reads it; no test asserts on it. Events in the fixtures carry
   realistic `source` values so the shape is right, but nothing depends on them.

---

## 2. TDD evidence rows

One row per behavior, in the order `TASK.md` lists them. Cycles 1–4 are the four stated
acceptance behaviors; cycles 5–6 discharge the "object/dict" clause (decision 2).

Every command is written out in full. Command keys are stable and referenced from the
chronological log in section 7.

### Cycle 1 — a critical event alerts (`TASK.md` behavior 1)

- **Seam / authority:** S1 — `TASK.md` behavior 1.
- **Test file / selector:** `<WD>/test_alerting.py::test_critical_event_alerts`
- **Observed public entry point:** `alerting.should_alert`
- **Red command (`RED-1`):**
  `python3 -m pytest <WD>/test_alerting.py::test_critical_event_alerts -q`
- **Intended failure:** the test cannot even import, because `alerting` does not exist yet.
- **Observed failure:** `ModuleNotFoundError: No module named 'alerting'` — `1 error`.
  Matches the intent.
- **Minimal implementation:** created `alerting.py` with `should_alert` returning `True`.
  Deliberately degenerate: the skill says only enough code to pass this test, and *not* to
  anticipate future tests. The severity rule would have anticipated behavior 2, so it
  waited for cycle 2 to force it.
- **Green command (`GREEN-1`):**
  `python3 -m pytest <WD>/test_alerting.py::test_critical_event_alerts -q`
- **Green result:** `1 passed`.

### Cycle 2 — an info event does not alert (`TASK.md` behavior 2)

- **Seam / authority:** S1 — `TASK.md` behavior 2.
- **Test file / selector:** `<WD>/test_alerting.py::test_info_event_does_not_alert`
- **Observed public entry point:** `alerting.should_alert`
- **Red command (`RED-2`):**
  `python3 -m pytest <WD>/test_alerting.py::test_info_event_does_not_alert -q`
- **Intended failure:** the always-true stub from cycle 1 alerts on an info event.
- **Observed failure:** `AssertionError: assert True is False` at
  `should_alert({'retrospective': False, 'severity': 'info', 'source': 'checkout-api'})`
  — `1 failed`. Matches the intent, and it kills the cycle-1 stub exactly as expected.
- **Minimal implementation:** `return event["severity"] == "critical"`. Still no
  retrospective handling — that is behavior 3's job.
- **Green command (`GREEN-2`):**
  `python3 -m pytest <WD>/test_alerting.py::test_info_event_does_not_alert -q`
- **Green result:** `1 passed`.

### Cycle 3 — a retrospective event never alerts (`TASK.md` behavior 3)

- **Seam / authority:** S1 — `TASK.md` behavior 3.
- **Test file / selector:**
  `<WD>/test_alerting.py::test_retrospective_critical_event_does_not_alert`
- **Observed public entry point:** `alerting.should_alert`
- **Discriminating input:** `severity="critical"`, `retrospective=True`. Critical is the
  only severity that discriminates here — a retrospective *info* event already returns
  `False` under the cycle-2 severity-only rule, so it could not tell the two
  implementations apart and would have been a non-discriminating golden.
- **Red command (`RED-3`):**
  `python3 -m pytest <WD>/test_alerting.py::test_retrospective_critical_event_does_not_alert -q`
- **Intended failure:** the severity-only rule alerts on a retrospective critical.
- **Observed failure:** `AssertionError: assert True is False` at
  `should_alert({'retrospective': True, 'severity': 'critical', ...})` — `1 failed`.
  Matches the intent.
- **Minimal implementation:** early `return False` when `event["retrospective"]`.
- **Green command (`GREEN-3`):**
  `python3 -m pytest <WD>/test_alerting.py::test_retrospective_critical_event_does_not_alert -q`
- **Green result:** `1 passed`.

### Cycle 4 — retrospective criticals still count (`TASK.md` behavior 4)

- **Seam / authority:** S2 — `TASK.md` behavior 4.
- **Test file / selector:**
  `<WD>/test_alerting.py::test_escalation_count_counts_retrospective_criticals_too`
- **Observed public entry point:** `alerting.escalation_count`
- **Discriminating golden:** six events, expected `2`. The expected value is read off the
  fixture (exactly two events are critical), not recomputed the way the code computes it,
  so the test is not tautological. The mix is chosen so `2` is the answer for "count
  criticals" and for no other rule anyone might plausibly ship instead: all events → 6,
  non-retrospective → 3, retrospective → 3, events that alert → 1, non-retrospective
  criticals → 1, anything above info → 4. Section 4 verifies this by experiment rather
  than by assertion.
- **Red command (`RED-4`):**
  `python3 -m pytest <WD>/test_alerting.py::test_escalation_count_counts_retrospective_criticals_too -q`
- **Intended failure:** `escalation_count` does not exist yet.
- **Observed failure:** `ImportError: cannot import name 'escalation_count' from
  'alerting'` — `1 error`. Matches the intent.
- **Minimal implementation:**
  `return sum(1 for event in events if event["severity"] == "critical")`.
- **Green command (`GREEN-4`):**
  `python3 -m pytest <WD>/test_alerting.py::test_escalation_count_counts_retrospective_criticals_too -q`
- **Green result:** `1 passed`.

### Cycle 5 — `should_alert` accepts attribute-style events (`TASK.md` "object/dict")

- **Seam / authority:** S1 — `TASK.md`, "An `event` is a simple object/dict with fields".
  Recorded as decision 2 above.
- **Test file / selector:**
  `<WD>/test_alerting.py::test_should_alert_accepts_attribute_style_events`
- **Observed public entry point:** `alerting.should_alert`
- **Red command (`RED-5`):**
  `python3 -m pytest <WD>/test_alerting.py::test_should_alert_accepts_attribute_style_events -q`
- **Intended failure:** `should_alert` subscripts the event, which an attribute-style
  object does not support.
- **Observed failure:** `TypeError: 'types.SimpleNamespace' object is not subscriptable`
  at `alerting.py:12` — `1 failed`. Matches the intent.
- **Minimal implementation:** added the private `_field(event, name)` accessor
  (`Mapping` → subscript, otherwise `getattr`) and used it **in `should_alert` only**.
  `escalation_count` was left subscripting on purpose, so that cycle 6 would have a real
  red instead of a vacuous one.
- **Green command (`GREEN-5`):**
  `python3 -m pytest <WD>/test_alerting.py::test_should_alert_accepts_attribute_style_events -q`
- **Green result:** `1 passed`.

### Cycle 6 — `escalation_count` accepts attribute-style events (`TASK.md` "object/dict")

- **Seam / authority:** S2 — same clause as cycle 5.
- **Test file / selector:**
  `<WD>/test_alerting.py::test_escalation_count_accepts_attribute_style_events`
- **Observed public entry point:** `alerting.escalation_count`
- **Red command (`RED-6`):**
  `python3 -m pytest <WD>/test_alerting.py::test_escalation_count_accepts_attribute_style_events -q`
- **Intended failure:** `escalation_count` still subscripts, so it rejects attribute-style
  events even though `should_alert` now accepts them.
- **Observed failure:** `TypeError: 'types.SimpleNamespace' object is not subscriptable`
  at `alerting.py:32` — `1 failed`. Matches the intent, and confirms cycle 5's green did
  not silently cover this seam.
- **Minimal implementation:** `escalation_count` now reads through `_field` too.
- **Green command (`GREEN-6`):**
  `python3 -m pytest <WD>/test_alerting.py::test_escalation_count_accepts_attribute_style_events -q`
- **Green result:** `1 passed`.

### Full suite

- **Command (`SUITE`):** `python3 -m pytest <WD> -v`
- **Result:** `6 passed in 0.12s`.

---

## 3. Pre-review reconciliation (changed tests ↔ seams)

There is no git baseline in this directory, so every test in the inventory is new; the
inventory is derived mechanically from pytest's own collection, which is this project's
native test structure, rather than from memory.

- **Command (`INVENTORY`):**
  `python3 -m pytest <WD>/test_alerting.py --collect-only -q -p no:cacheprovider`
- **Result:** `6 tests collected`.

| # | Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `<WD>/test_alerting.py` | `test_critical_event_alerts` | `alerting.should_alert` | `TASK.md` behavior 1 | Cycle 1 |
| 2 | `<WD>/test_alerting.py` | `test_info_event_does_not_alert` | `alerting.should_alert` | `TASK.md` behavior 2 | Cycle 2 |
| 3 | `<WD>/test_alerting.py` | `test_retrospective_critical_event_does_not_alert` | `alerting.should_alert` | `TASK.md` behavior 3 | Cycle 3 |
| 4 | `<WD>/test_alerting.py` | `test_escalation_count_counts_retrospective_criticals_too` | `alerting.escalation_count` | `TASK.md` behavior 4 | Cycle 4 |
| 5 | `<WD>/test_alerting.py` | `test_should_alert_accepts_attribute_style_events` | `alerting.should_alert` | `TASK.md` "object/dict" (decision 2) | Cycle 5 |
| 6 | `<WD>/test_alerting.py` | `test_escalation_count_accepts_attribute_style_events` | `alerting.escalation_count` | `TASK.md` "object/dict" (decision 2) | Cycle 6 |

**Both-directions comparison.** Collected selectors = {1..6 above}. Reconciliation rows =
{1..6 above}. Collected − reconciled = ∅. Reconciled − collected = ∅. Gate passes.

Every row enters through S1 or S2. No row enters through a module, helper, mutable global,
or side channel, so nothing needs re-ratifying or rewriting. No row substitutes a
private-invariant test for a public-behavior claim: each row's assertion is the public
behavior its authority states.

**`/code-review` was not run.** It reviews a diff against a commit/branch/merge-base, and
this working directory is not a git repository, so it has nothing to scope against. The
task also did not ask for a review. The reconciliation gate above is complete and would be
what I hand it. Consequently the skill's "review repairs" clause never triggered: there are
no `TDD re-entry required: yes` findings, and therefore zero review-reentry rows — the
both-directions comparison against a `Findings fixed during review` ledger is vacuous
(∅ vs ∅).

---

## 4. Golden sensitivity check (evidence, not assertion)

`tests.md` says a golden that would still pass after the forbidden behavior is substituted
is not discriminating and must be replaced. I checked that by substituting each rejected
implementation and running the suite, rather than eyeballing it. The harness lived in
`<WD>/.mutation-check/run_check.py` and was **deleted afterwards** — it is not part of the
deliverable. It is reproducible from this description.

- **Command (`MUTANTS`):** `python3 <WD>/.mutation-check/run_check.py`
- **Result:** exit 0 — all 8 rejected rules caught, no survivors.

| Substituted (rejected) implementation | Caught by |
| --- | --- |
| `should_alert` ignores `retrospective` | `test_retrospective_critical_event_does_not_alert` |
| `should_alert` always alerts | `test_info_event_does_not_alert`, `test_retrospective_critical_event_does_not_alert` |
| `should_alert` alerts on anything above info (no retro guard) | `test_retrospective_critical_event_does_not_alert` |
| `escalation_count` drops retrospective criticals | both `escalation_count` tests |
| `escalation_count` counts every event | both `escalation_count` tests |
| `escalation_count` counts non-retrospective events | `test_escalation_count_counts_retrospective_criticals_too` |
| `escalation_count` counts events that alert | both `escalation_count` tests |
| `escalation_count` counts anything above info | both `escalation_count` tests |

Note the sixth row: the "counts non-retrospective events" mutant is caught **only** by the
six-event golden. In the smaller three-event fixture of cycle 6 that rule also yields 2, so
it would have survived. That is precisely why the cycle-4 fixture was built with six events
instead of the obvious four — a four-event version I first considered (2 retrospective, 2
not) would *not* have discriminated that rule.

---

## 5. Known coverage gap, measured rather than assumed

I checked the one claim I would otherwise have been tempted to state without evidence.

- **Command (`MUTANT-WARNING`):** inline `python3 - <<'PY' … PY` heredoc run from
  `<WD>/.mutation-check`, substituting a `should_alert` that keeps the retrospective guard
  but widens the severity rule to `!= "info"`.
- **Result:** `SURVIVED` — `6 passed`.

So the suite does **not** pin the behavior of a non-retrospective `warning` event. A mutant
that alerts on warnings passes all six tests. This is deliberate, not an oversight:

`TASK.md` lists `"warning"` as a legal severity but none of its four acceptance behaviors
says whether a non-retrospective warning alerts. The skill forbids writing a test at an
unconfirmed seam, and a test asserting `should_alert(warning) is False` would ratify a
decision no authority ever made — freezing a guess into the suite where a later reader
would mistake it for a requirement. The implementation returns `False` for warnings as a
consequence of the minimal `== "critical"` rule, not as a decided behavior.

**This is the open question a real user should settle:** should a non-retrospective
`warning` alert? One sentence of authority and it becomes cycle 7. Until then the gap is
documented rather than silently filled.

---

## 6. Anti-patterns actively avoided

- **Horizontal slicing** — not done. Six vertical slices, one test → one implementation
  each; the test file was appended to six times, never written in bulk up front. The
  cycle-1 stub (`return True`) is the visible proof that no future test was anticipated.
- **Tautological assertions** — every expected value is an independent literal (`True`,
  `False`, `2`), never a re-computation of the implementation's own logic.
- **Implementation coupling** — no mocks at all (there is no system boundary here to mock:
  no I/O, no time, no randomness), no private methods tested, no side-channel verification.
- **Non-discriminating goldens** — checked by experiment in section 4, not assumed.

---

## 7. Chronological command log

Every command, in order, with its outcome. File writes/edits are listed too so the red and
green states are unambiguous. "Red" failures are intended failures, not accidents.

| # | Command / action | Outcome |
| --- | --- | --- |
| 1 | `python3 --version; python3 -m pytest --version` | **passed** — Python 3.14.4, pytest 9.0.2 |
| 2 | *(write)* `NOTES.md` — seams + authority recorded before any test | n/a |
| 3 | *(write)* `test_alerting.py` with `test_critical_event_alerts` only | n/a |
| 4 | `RED-1` | **failed as intended** — `1 error`, `ModuleNotFoundError: No module named 'alerting'` |
| 5 | *(write)* `alerting.py`, `should_alert` returns `True` | n/a |
| 6 | `GREEN-1` | **passed** — `1 passed` |
| 7 | *(edit)* append `test_info_event_does_not_alert` | n/a |
| 8 | `RED-2` | **failed as intended** — `1 failed`, `AssertionError: assert True is False` |
| 9 | *(edit)* `should_alert` → `event["severity"] == "critical"` | n/a |
| 10 | `GREEN-2` | **passed** — `1 passed` |
| 11 | *(edit)* append `test_retrospective_critical_event_does_not_alert` | n/a |
| 12 | `RED-3` | **failed as intended** — `1 failed`, `AssertionError: assert True is False` |
| 13 | *(edit)* `should_alert` → early `return False` on `retrospective` | n/a |
| 14 | `GREEN-3` | **passed** — `1 passed` |
| 15 | *(edit)* import `escalation_count`; append `test_escalation_count_counts_retrospective_criticals_too` | n/a |
| 16 | `RED-4` | **failed as intended** — `1 error`, `ImportError: cannot import name 'escalation_count'` |
| 17 | *(edit)* add `escalation_count` counting criticals | n/a |
| 18 | `GREEN-4` | **passed** — `1 passed` |
| 19 | *(edit)* import `SimpleNamespace`; append `test_should_alert_accepts_attribute_style_events` | n/a |
| 20 | `RED-5` | **failed as intended** — `1 failed`, `TypeError: 'types.SimpleNamespace' object is not subscriptable` (`alerting.py:12`) |
| 21 | *(edit)* add private `_field`; use it in `should_alert` only | n/a |
| 22 | `GREEN-5` | **passed** — `1 passed` |
| 23 | *(edits ×3)* append `test_escalation_count_accepts_attribute_style_events`, then move it to end of file for readability (no test command run in between) | n/a |
| 24 | `RED-6` | **failed as intended** — `1 failed`, `TypeError: 'types.SimpleNamespace' object is not subscriptable` (`alerting.py:32`) |
| 25 | *(edit)* `escalation_count` reads through `_field` | n/a |
| 26 | `GREEN-6` | **passed** — `1 passed` |
| 27 | `SUITE` | **passed** — `6 passed in 0.12s`, all six selectors listed PASSED |
| 28 | `MUTANTS` | **passed** (exit 0) — all 8 rejected implementations caught, `survivors: none` |
| 29 | `MUTANT-WARNING` | **mutant SURVIVED** (`6 passed`) — expected and documented in section 5; not a test failure |
| 30 | `rm -rf <WD>/.mutation-check <WD>/.pytest_cache <WD>/__pycache__` | **passed** — deliverable left as `NOTES.md`, `TASK.md`, `alerting.py`, `test_alerting.py`, `skill/` |
| 31 | `INVENTORY` | **passed** — `6 tests collected`, matching the six reconciliation rows both ways |
| 32 | `grep -nE "_field\|import\|alerting\." <WD>/test_alerting.py` | **passed** — import surface is only `escalation_count`, `should_alert`, `SimpleNamespace`; no private access |

No command failed unexpectedly at any point. Every red in the table above was the intended
failure for its cycle, and each was verified against the intent before I wrote the
implementation that turned it green.

### Command key expansion

| Key | Complete runnable command |
| --- | --- |
| `RED-1` / `GREEN-1` | `python3 -m pytest <WD>/test_alerting.py::test_critical_event_alerts -q` |
| `RED-2` / `GREEN-2` | `python3 -m pytest <WD>/test_alerting.py::test_info_event_does_not_alert -q` |
| `RED-3` / `GREEN-3` | `python3 -m pytest <WD>/test_alerting.py::test_retrospective_critical_event_does_not_alert -q` |
| `RED-4` / `GREEN-4` | `python3 -m pytest <WD>/test_alerting.py::test_escalation_count_counts_retrospective_criticals_too -q` |
| `RED-5` / `GREEN-5` | `python3 -m pytest <WD>/test_alerting.py::test_should_alert_accepts_attribute_style_events -q` |
| `RED-6` / `GREEN-6` | `python3 -m pytest <WD>/test_alerting.py::test_escalation_count_accepts_attribute_style_events -q` |
| `SUITE` | `python3 -m pytest <WD> -v` |
| `INVENTORY` | `python3 -m pytest <WD>/test_alerting.py --collect-only -q -p no:cacheprovider` |
| `MUTANTS` | `python3 <WD>/.mutation-check/run_check.py` (harness since deleted; see section 4) |
| `MUTANT-WARNING` | inline heredoc, see section 5 |
