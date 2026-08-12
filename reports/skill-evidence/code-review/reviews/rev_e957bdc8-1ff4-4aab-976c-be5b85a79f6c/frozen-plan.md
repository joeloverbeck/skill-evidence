# Frozen Validation Plan: code-review

Frozen before any current-arm output or candidate existed.

## Evidence and mechanism map

### M1 — long-session aggregation compliance

- Trigger events: `evt_7814cfa8-ad78-4511-b202-f78ad96902d3`,
  `evt_d629f005-ff0e-4bda-8136-9f82d13c3c04`, and
  `evt_3ea9e7a9-f39c-4c9b-8dff-5dbeab0860f9`.
- Candidate mechanism: the final output contract is present, but its placement after a long dispatch
  contract and its several independently stated artifacts can repeatedly lose compliance in a
  many-pass implementation review. This is a candidate target-compliance defect, not a conclusion.
- Why grouped: all three uses replaced the required side-by-side reports, fixed-findings ledger,
  aggregate-conformance result, and/or exact summary with prose after nine to twelve passes.
- Binding constraint: the operating caller has accumulated and repaired nine to twelve consecutive
  two-axis passes in one long implementation session, then must reconcile and present the complete
  cross-pass set at final handoff.
- Provenance: each trigger's `run_condition` and `observed` fields records nine to twelve passes and
  omission from the first pass through final handoff; two record 18 to 24 axis reports and one
  records 26 findings.
- Trial slot: unable to be expressed. A fresh, short-context executor cannot naturally acquire the
  accumulated long-session state. Preloading a finished pass census would vary the task itself and
  would test a summarization fixture rather than the recorded operating condition.
- Would-be witness: the naturally accumulated pass census and axis reports, followed by the final
  side-by-side reports, ledger, bidirectional identity reconciliation, and one-line summary.
- Unable-to-be-expressed reading: the executor begins with supplied historical artifacts rather than
  having accumulated them through the long implementation run, or lacks the nine-to-twelve-pass
  state entirely.
- Earlier same-shape unable ruling on these target bytes: none. The governing same-hash predecessor
  tested working-tree mutation, not aggregation. The other same-hash report used a different
  operating Skill Evolution hash and weighed one aggregation omission as outside-target evidence;
  it did not run or rule an unable reproduction.

### M2 — Spec report header placement

- Trigger event: `evt_c5666704-0654-4a6f-9626-2deb557e1302`.
- Candidate mechanism: the required opening lines are expressed as prose around the Spec prompt
  rather than as a copy-ready report scaffold, allowing an otherwise substantive report to place
  them elsewhere. This is a candidate target-compliance defect.
- Binding constraint: a fresh Spec-axis executor receives the current packet contract and returns a
  completed report for a concrete diff and spec.
- Provenance: the trigger's `observed` and `run_condition` fields record one completed R2 Spec report
  that placed both required lines at the end; its direct re-emission workaround suppressed the
  deviation.
- Natural witness: a finished report that analyzes the supplied `max_attempts` change and cites the
  supplied spec. Unexpressed means no finished substantive report, refusal, or failure to address
  the supplied change.
- Failure reading: the first two nonblank report lines are not exactly `Reviewed HEAD: <sha>` and
  `Review pass: R2` in that order. Passing also requires any actionable finding to carry the stable
  finding ID and both routing fields.
- Runs: three independent current-arm runs if run 1's witness is expressed. The packet supplies no
  reliable per-launch denominator, so the incidence-sized floor of three is used.

### M3 — documented-standard precedence over the smell baseline

- Trigger event: `evt_35c7290b-507a-48b4-8f31-4186fb262cee`.
- Candidate mechanism: the Standards packet names the source list and separately pastes a much more
  salient smell baseline, but does not require an explicit source-before-smell reconciliation for
  each proposed smell. This is a candidate target-compliance defect.
- Binding constraint: a fresh Standards-axis executor reviews a boundary that the selected
  repository standard expressly requires but that resembles Speculative Generality or Middle Man.
- Provenance: the trigger's `expected`, `observed`, and `workaround_taken` fields record that the
  first R2 Standards report criticized the required before-append seam and withdrew the finding
  after the exact criterion was resubmitted.
- Natural witness: a finished report that discusses the `PreparedReviewEvent`/append boundary and
  names the selected standards source or reports no actionable findings after checking it.
  Unexpressed means no finished substantive report or no analysis of that boundary.
- Failure reading: an actionable finding criticizes the mandated preparation/append boundary as
  Speculative Generality, Middle Man, or unnecessary indirection. Passing requires both coverage
  lines and must not suppress a separate genuine smell when one is present.
- Runs: three independent current-arm runs if run 1's witness is expressed. The trigger does not
  establish a reliable per-launch denominator, so the incidence-sized floor of three is used.

## Recorded-workaround direction

- M1: two triggers substituted informal `/tdd` reconciliation and prose handoff without suppressing
  the missing prescribed artifacts; the third records no workaround. This weighs against target
  ownership but does not override the independent recurrence.
- M2: discarding and directly requesting a conformant re-emission suppressed the deviation, which
  supports a compliance mechanism that target-carried structure could affect.
- M3: resubmitting the exact selected-source criterion suppressed the deviation, which supports a
  salience/competition mechanism that the target packet could affect.

The authorization cluster contains no open incident outside the five trigger IDs. The packet has
one open incident in a different symptom cluster; its payload is outside this review and was not
read.

## Candidate rule

Run M2 and M3 against an opaque held copy of the unchanged package before creating a candidate.
Build a candidate only for a mechanism that reproduces. Any candidate must be limited to
consolidation or clearer replacement inside the relevant reviewer-packet contract, with no incident
narrative and no change for M1. If neither runnable mechanism reproduces, build nothing.

## Risk tier and paired suite

Risk tier: **high**. A possible candidate can affect two reviewer behaviors and a broad packet
contract, so the frozen suite has five paired trials:

| Trial | Capability | Runs per arm | Protected behavior |
|---|---|---:|---|
| T1 | M2 Spec header reproduction | 3 | Exact header order plus substantive Spec review |
| T2 | M3 standards-precedence reproduction | 3 | Repository rule overrides smell baseline |
| T3 | Adjacent Spec case | 1 | A real missing requirement is still reported with identity and routing |
| T4 | Core Standards regression | 1 | A real unendorsed Speculative Generality smell is still reported |
| T5 | Fragile mixed Standards case | 1 | Mandated seam is suppressed while a separate genuine smell is retained |

T1 and T2 current-arm runs occur before any candidate. T3 through T5 run on both arms only if a
candidate is built. Every runnable candidate-arm run whose witness is unexpressed is replaced once
with a fresh executor; a second unexpressed reading fails that trial. Current-arm reproduction run 1
stops its mechanism immediately if unexpressed, without a recut prompt or fixture.

## Raw tasks, artifacts, and logistics

Each fixture directory contains `raw-task.md` and the raw input artifacts. Executors receive those
unchanged files plus only a logistics note naming an opaque held-package path and an output path.
They receive no evidence store, incident text, diagnosis, expected answer, arm name, or candidate
label. Held packages and executor-visible fixture copies live outside the repository under `/tmp`.
Raw outputs are copied back under `trials/` after the executor finishes.

## Rubrics and deterministic checks

- T1: parse the first two nonblank lines exactly; verify the reviewed SHA and `R2`; verify any
  actionable finding has `Finding ID`, `Repair class`, and `TDD re-entry required`.
- T2: require the two coverage lines, require analysis of the named boundary, and reject a finding
  against that mandated boundary.
- T3: require a finding for the missing empty-key rejection, with the Spec header, a stable Spec
  finding ID, and both routing fields.
- T4: require a judgement-call Speculative Generality finding for the unused `FutureFormatter`
  extension point, with Standards coverage lines and routing fields.
- T5: reject a finding against the required prepare/append boundary, require a finding against the
  unrelated unused `FutureHook`, and require complete Standards coverage and routing fields.

An independent evaluator receives randomized output labels, the raw fixture, and this frozen rubric
but not package paths or arm mapping. Deterministic header/field checks are applied before semantic
comparison. A candidate is acceptable only if it resolves every reproduced mechanism, is
noninferior on T3–T5, introduces no material regression, and is materially clearer or smaller when
behavior is tied.
