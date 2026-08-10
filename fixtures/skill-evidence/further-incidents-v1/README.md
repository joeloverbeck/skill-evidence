# Further-incident fixture

This fixture repository was written on 2026-08-10 by the compiled `skills evidence record`,
`skills evolution preflight`, `skills evolution claim`, and `skills evolution close` commands
for GitHub [#27](https://github.com/joeloverbeck/skill-evidence/issues/27), which let one run
record more than one incident. Every event here came out of the real commands; none was
hand-authored.

It holds the stream shape no earlier corpus could contain: **two use records sharing one run
group on one target hash**, written by declaring the second a further incident of the first.
Before #27 the write path refused exactly that, so `lifecycle-v1` and `status-reporters-v1`
have one record per run group and cannot exercise the rules below.

## What happened, in order

| # | Event | Run group | Session |
|---|---|---|---|
| 1 | `use_recorded` — `friction`, symptom `execution` | run A | `session-a` |
| 2 | `use_recorded` — `friction`, symptom `execution` | run B | `session-b` |
| 3 | `use_recorded` — `friction`, symptom `execution` | run C | `session-c` |
| 4 | `review_started` — `rev_further_incidents` | — | `session-review` |
| 5 | `use_recorded` — **a further incident of run A** | run A | `session-a` |
| 6 | `use_recorded` — **a third incident of run A** | run A | `session-a` |
| 7 | `review_disposition` — `closed_no_skill_defect` | — | `session-review` |

Three independent incidents reached `friction_recurrence:execution`, so the review was
authorized and claimed, freezing a trigger list of events 1–3. Run A then deviated twice more
while the review was still running, so events 5 and 6 sit **outside** the coverage the close
can account for. The close adjudicated its coverage and named event 1 — one sibling, not the
run — as untestable coverage.

## What the frozen projection pins

- `qualifying_uses_on_current_hash` is **3** against **5** use records. The denominator counts
  run groups, so the run that deviated three ways is one use. A reader that reverted to
  counting records would report 5 and fail the replay.
- `instrument_limited_incident_ids` holds **event 1 alone**. Naming a sibling retires that
  sibling, never the run group it belongs to.
- Event 1 stays in `open_incident_ids`: named coverage was never adjudicated, so it is still
  open in the ledger while out of the clusters.
- **The `execution` cluster holds events 5 and 6.** Their only run-mate, event 1, was named
  untestable and retired — named coverage is never adjudicated, so nothing about run A was
  adjudicated at all. Events 2 and 3, from runs B and C, were. Events 5 and 6 are therefore
  still open and still clustering, which is the rule whose loss motivated #27: when one event
  carried both a testable and an untestable mechanism, naming the event retired the testable
  one with it.
- That cluster reports **1** independent incident across its **2** open events. Siblings share
  a top-level session and a task fingerprint, so one run contributes one independent incident
  however many ways it deviated, and a `friction_recurrence` threshold cannot be reached by one
  run misbehaving repeatedly.

`lifecycle-v1` cannot stand in for any of this: its projection records zero qualifying uses, so
replaying it compares `0` against `0` and says nothing about how uses are counted.

## What this corpus is not

It attests nothing about whether #27 was implemented correctly, and no reading of it should
claim otherwise. `lifecycle-v1` is evidence in a stronger sense — a *different* implementation
wrote it, so replaying it shows the Rust reader reproducing what the JavaScript one derived.
This corpus was written by the same code it now guards, and its `gate-status.json` is that
code's own output, so at birth the byte-comparison can only agree with itself.

What it does is guard forward. The numbers above were fixed by the ratified requirement, not
read off a run, and the replay asserts them explicitly before it asserts byte-equality — so a
future reader that counts records rather than run groups, lets siblings count as independent,
or spreads a name across a run group fails here on the requirement and not merely on a diff.
Freezing a shape the writer has only just started producing is the only way that guard can
exist at all: no consumer stream contains a further incident yet, and one written later could
not be added retroactively to a corpus this change was validated against.
