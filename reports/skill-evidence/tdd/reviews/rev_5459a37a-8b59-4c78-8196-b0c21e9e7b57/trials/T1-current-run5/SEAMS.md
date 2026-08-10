# Seams under test (written before the first test)

Authority for every seam below: `TASK.md` in this working directory — it is the
task-defining document for this work and names the public module and both
functions explicitly. No `CONTEXT.md` and no ADR directory exist in this working
directory, so there is no project domain vocabulary to conform to beyond
`TASK.md`'s own terms (event, severity, retrospective, source, escalation).

| Seam | Public entry point | Authority | Ratified by |
| --- | --- | --- | --- |
| S1 | `alerting.should_alert(event) -> bool` | `TASK.md` "Implement an alert router with a public module `alerting.py` exposing: `should_alert(event) -> bool`"; acceptance behaviors 1-3 | Task document (authoritative for this work) |
| S2 | `alerting.escalation_count(events) -> int` | `TASK.md` "... `escalation_count(events) -> int`"; acceptance behavior 4 | Task document (authoritative for this work) |

No other seam is tested. Nothing private in `alerting.py` is imported by the
tests; the tests import only `should_alert` and `escalation_count`.

## Decisions made without a user

The run constraints state no user is available. Where the skill would have me
confirm something, I decided and recorded it here.

- **D1 — the seams above are treated as already agreed.** The skill says a seam
  ratified in an authoritative document or explicit user decision needs no
  question. `TASK.md` names both function signatures and the module, so S1 and
  S2 are ratified and I did not ask.
- **D2 — event representation: both mapping and attribute forms are in scope.**
  `TASK.md` says "An `event` is a simple object/dict with fields". It names both
  forms, so both are ratified input shapes at S1 and S2 rather than speculative
  extras. Cycles 1-4 (the four numbered acceptance behaviors, in the task's
  order) use dict events; cycles 5-6 drive the attribute form, one function per
  cycle. Each is a real red before its green.
- **D3 — `severity="warning"` at S1 is left unspecified and untested.**
  `TASK.md` pins only `critical` (behavior 1) and `info` (behavior 2) for
  `should_alert`, and behavior 3 is about `retrospective`, which I probe with
  `critical` because that is the only severity the task pins as alerting and so
  the only one where suppression is observable. Writing a `warning` test would
  invent an acceptance criterion the authority does not state. The
  implementation that behaviors 1-3 force ("critical and not retrospective")
  happens to return `False` for `warning`; that is a consequence of the minimal
  implementation, not a tested claim.
- **D4 — the `source` field is carried on fixtures but never asserted on.**
  `TASK.md` lists it as a field of an event but attaches no behavior to it. It
  appears in test fixtures so the fixtures are well-formed events, and no test
  claims anything about it.
