# A frozen reading rests only on artifacts the retention contract carries

Status: accepted (2026-08-14, GitHub [#58](https://github.com/joeloverbeck/skill-evidence/issues/58))

A frozen mechanism clause may name a natural observable only when that observable is readable from
the artifacts step 6 already promises to retain — the run's own outputs and the frozen inputs. When a
plan names one that is not, the clause is a **defect in the plan**, of the same species as a
[frozen-input fault](0009-the-acceptance-gate-spends-only-arm-discriminating-evidence.md): it voids
that clause's reading and nothing else. It supports no unable-to-be-expressed marking, no
`--instrument-limited` naming, and no `blocked_no_valid_test`. At evaluation, a reading is
adjudicated only from the artifacts its plan named for it; an artifact that is absent makes the
reading **unread**, never proxied, and the terminal report restates a frozen observable verbatim or
not at all.

## Why

Review `rev_f03acab9-1491-4313-891a-9f3e38543752`, on the repository-local
`.claude/skills/grill-with-docs`, froze clause C2 — *work begins without a declared grilling shape* —
against the natural observable **"retained first substantive response and report process ledger."**
No such artifact class exists anywhere in `authorized-review.md`; step 4 invented it, and step 6 was
never told to retain it. Each retained run directory holds `raw-task.md`, `inputs/`, and
`outputs/decision-report.md`. There is no response stream.

The blind evaluator did not stop. It graded C2 from line ordering *inside each decision report* —
"the first verdict is at line 5; 'adjudication' first appears at line 19" — and recorded 3/3
recurrence for the current arm and 1/3 for the candidate. On that reading a candidate was built,
graded, and rejected, and the review closed `candidate_rejected_validation`.

The executor session streams say otherwise. In all six valid T1 runs the shape was named before the
first read of `inputs/review-card.md`; three of them open with a literal `Grilling shape:
adjudication` line. C2's recurrence was 0/3 and 0/3. The current arm never reproduced the mechanism,
so no candidate should have been built at all.

Three things make this a decision about what a plan may freeze, rather than a decision to start
retaining chronology.

**The target never owed the chronology to any artifact.** `grill-with-docs` says *"First classify the
`grilling` shape."* Classify — not state, declare, or write. A compliant run owed the output nothing,
so no retention policy could have made C2 readable from a finished run. The candidate's own change
hypothesis was to require *"one visible `Grilling shape: ...` line before premise verification"* —
which is to say the observable C2 needed exists only under the repair. Grading the current arm on it
is the failure mode
[ADR 0009](0009-the-acceptance-gate-spends-only-arm-discriminating-evidence.md) already names: *"a
criterion that first existed at step 7."*

**The instrument reads finished artifacts, and says so.** [`../../CONTEXT.md`](../../CONTEXT.md)
defines a witness as *"the observable in a finished run's own output."* Artifact identity relation,
frozen-input fault, and the deterministic checks all read what is on disk. C2's observable reached
outside that class, and nothing in the glossary, the principles, or the installed reference
authorizes it to.

**The cost fell on evidence, not just on a report.** [ADR
0003](0003-no-new-instrument-for-conformance-only-evidence.md) routes per trigger: *"A trigger whose
own mechanism read not reproduced with witnesses expressed was decided by its reproduction trial and
never by the acceptance gate... Those adjudicate normally."* Under the correct reading every one of
this review's six triggers reads that way — C3, C4, and C5 already did on the review's own results,
and C2 joins them at 0/3 — so the review never reaches the acceptance gate and nothing retires.
Instead these five retired instrument-limited:

- `evt_fd4e5936-9eef-43ed-8eeb-5e9b2b77ebc7`
- `evt_cb30fe77-2977-42c8-9e7c-79ef74f0580a`
- `evt_209c2aac-6e1f-4efe-80f0-e0a4fb4e9ae3`
- `evt_d43f3d30-9d50-4479-9192-25400ec5ed7a`
- `evt_a6e1da94-b9fb-4d2e-a96b-39ef50d74980`

That is the mislabel ADR 0003 warns of by name — *"the projection claim a mechanism was untestable
that the trials demonstrably tested"* — reached here through a wrong reproduction reading rather than
through the over-broad reach [#16](https://github.com/joeloverbeck/skill-evidence/issues/16) narrowed
or the untestable mechanism [#30](https://github.com/joeloverbeck/skill-evidence/issues/30) discloses.
This decision closes that route and does not correct the five. *"Nothing added here brings retired
evidence back, and no route back is being built,"* and [ADR
0002](0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md) declined correction and
supersession twice; its reopening clause asks for *"a residue with no door,"* which these are not,
because the gate reopens on the next incident. The harm is bounded and permanent at once: evidence
spent for nothing, with the door still open. This paragraph is the record of it, and no tracker
carries it.

The plan-defect consequence is chosen over the obvious alternative for exactly that reason. Marking
the clause *unable to be expressed* would retire its triggers from the gate, which is the harm #16
was filed about, arriving from the opposite direction. Voiding it costs the clause alone. It also
keeps ADR 0003's distinction intact: *"the limit that naming records is the acceptance gate, not the
reproduction trial, and the two must not be confused."*

## Considered options

**Retain the host-native session stream.** Copy each executor's rollout or transcript into
`reviews/<review-id>/`, making chronological observables admissible. Faithful, unprimed, and
technically available — both agent hosts persist durable JSONL, and this is the evidence #58 was
verified from. Rejected on three grounds in adopted text, none of them availability.
[`../principles/inherited-prohibitions.md`](../principles/inherited-prohibitions.md) bars *"no
feature requiring an operator to write prose into a record so the system can read it back as
truth"* — a response stream is narration *about* classifying, not the classifying — and *"no
transient conversation as the only record."* Step 6 additionally bars every executor from the
evidence store; retaining full reasoning traces walks that wall back for later reviewers.

**Require the run to emit a process ledger.** Make the chronology output-readable by construction,
portable across hosts, with no host coupling. Rejected on a verified collision: step 6 forbids
executor logistics that *"change what a compliant run would do or output,"* and
[#41](https://github.com/joeloverbeck/skill-evidence/issues/41) governs this priming surface. Asking
a run to log when it classified the shape primes the behavior the clause measures.

**Restrict what may be frozen, and stop there.** The root fix, and the tightest normative statement.
Rejected as insufficient alone: it rests on the plan author correctly judging the observable's
readability *before* results, and this review is the proof that the judgment can go wrong with
nothing behind it.

**Add the evaluation guards, and stop there.** Catches this exact defect with the smallest change to
shipped text. Rejected as insufficient alone: a review still burns its full trial matrix — fourteen
valid runs here — before failing closed, and under ADR 0003 a review that reaches the acceptance gate
carries different retirement consequences from one that never gets there.

**Both, consequenced as a plan defect.** Chosen. The two failures happened at different steps of the
same review, and each guard catches what the other cannot.

## Consequences

- **Some mechanisms become unfreezable, and that is the point.** A clause about a target's *internal*
  ordering, where the target states no output obligation, can no longer be given a reading. The
  honest response is to repair the target's text so the obligation has an output surface — which is
  what this review's candidate proposed — and then freeze a clause against that surface, rather than
  to grade narration.
- **This builds no instrument and no acceptance test for conformance evidence.** ADR 0003's option
  (b) stands open exactly as it did before, and is neither taken nor foreclosed here. What changes is
  which readings a reproduction trial may carry, not what the acceptance gate grades.
- **The installed reference changes in two steps, so consumers must reinstall.** Step 4 gains the
  admissibility condition, steps 6 and 7 gain the unread-not-proxied rule and the verbatim-restatement
  rule. Per [`../releasing.md`](../releasing.md) that is a minor bump while `0.x`, forward-tested in
  both agent hosts, with explicit `skills evidence install --force` instructions. No recorded-event
  shape, no published schema, and no Rust API is touched.
- **No completed review is rewritten.** `rev_f03acab9-1491-4313-891a-9f3e38543752`, its events, its
  report, and its closure receipt stand as recorded. This decision governs reviews frozen after it,
  and the wrong one stays on the record as the evidence that produced it.
- **What would reopen this.** A review that cannot express a mechanism it has good reason to test,
  where the target *does* carry an output obligation and the retention contract still fails to carry
  it — that would be a retention gap rather than a plan defect, and this decision would be the wrong
  home for it.
