# The acceptance gate spends only arm-discriminating evidence

Status: accepted (2026-08-13, GitHub [#49](https://github.com/joeloverbeck/skill-evidence/issues/49))

Step 7 grades a candidate against the current skill. A reading both arms return identically
discriminates nothing between them, and a difference the plan never froze a comparison for is not a
reading at all. Three rules follow, all of them in the installed authorized-review reference.

**A frozen-input fault voids its criterion.** When both arms return the same reading and the frozen
input's own bytes contradict what the plan asserted about them, that is a defect in the plan rather
than a result about either arm. It is established by reading the retained artifact and quoting the
refuting bytes — never by re-running, and never by re-cutting the fixture, because a fixture re-cut
after results is a fixture shaped to its answer. The refuted criterion is void for the gate;
everything else that trial froze still counts. A void criterion supports no unable-to-be-expressed
marking, no `--instrument-limited` naming, and no `blocked_no_valid_test`: a plan defect is not an
instrument limit.

**Artifact sameness is frozen before outcomes exist.** Every reading that asks whether two artifacts
are the same carries an **artifact identity relation** fixed in step 4 — exact bytes by default, or
one named deterministic canonicalization with the exact command that computes it and the difference
it is allowed to absorb. A frozen reading that names no relation is governed by exact bytes; a
comparison the plan did not freeze at all cannot carry an adverse claim at step 7, where it is
recorded and decides nothing by itself; no relation may be chosen,
widened, or narrowed once results exist; and step 6 compares only under the frozen relation and
retains both sides of what it compared. This is the witness rule applied to a reading class it had
not reached.

**An adverse observation must be attributed.** Calling any observation a **material candidate
regression** requires all four: arm-discriminating, not variance, attributable to the candidate's
text — the clause it adds, the clause it removes, or the exact difference responsible where no
single clause is — and baselined against a behavior the current arm produced. A severe regression is
established the same way, because the four parts settle causation and not severity. The bar stays open
to an observation no frozen criterion covers, because a severe regression nobody anticipated is what
it exists to catch — but being frozen is not one of the four and does not stand in for them, so a
frozen criterion both arms fail is no more a regression than an unfrozen one. Where the candidate
supplies a capability the current arm never had, the fourth fails by construction; the observation
is regraded against mechanism resolution and against necessary, minimal, outcome-supported growth,
where it can still reject the candidate. An observation failing any of the four is recorded with
which part it failed and cannot by itself reject.

That narrows the open bar in one place. An unanticipated regression turning on two artifacts being
the same cannot be established by the review that first notices it, because the relation deciding
sameness would have to be chosen after the result it decides. The narrowing is accepted and the
installed reference states it where a reviewer meets it, rather than leaving it to this record.

Voiding reaches every criterion the gate reads by comparing the arms, the reproduction reading
included. Safety, scope, and ownership
invariants, and the deterministic checks the candidate must pass before landing, are absolute: the
current skill failing them too does not satisfy them. Apart from the narrowing just named, the
gate's terms do not move. What moves is that a claim under them is established from the retained
artifacts instead of asserted at adjudication.

## Why

The occurrence is review `rev_9f4689de-42c5-4e51-ac00-357c309da60a` on `code-review`, closed
`candidate_rejected_validation` at `b47aa34`. It rejected a candidate its own report calls
"materially better on M2", on two grounds, and neither was one this gate could spend against the
candidate. The first was a frozen criterion both arms failed; the second was never frozen at all.

**T4 was a fixture fault counted as a result.** The trial's committed patch adds
`printf 'hello\\n'` — a doubled backslash, so the function prints a literal `\n` and no line ending
— while `SPEC.md` asks for a function that prints `hello`. Both arms reported the Spec finding the
frozen rubric expected neither to report. The report names this correctly, as "symmetric rather than
an arm regression", and then counts it anyway on the ground that the gate "requires the frozen suite
to pass". No such requirement exists. The shipped gate asks for noninferiority on protected behavior
and no material or severe regression; the review's own frozen acceptance gate asks that "T2-T5 are
noninferior on every protected behavior". A result both arms share is noninferior. Step 7's one
absolute sentence about a frozen suite rerunning covers a mechanical candidate defect corrected
before any behavioral trial, which this was not.

**T1 was an unfrozen comparison introduced at adjudication.** The candidate arm reported patch
identity `cbfcec98…` in runs 1 and 3 and `ab7db7f9…` in run 2. The frozen deterministic check — the
scoped-diff SHA-256 before and after each run — read `cbfcec98…` in all six repositories and passed,
and the mechanism's own suppression clause C3 reads identity consistency *within* a report, which
the review's deterministic-check record reads as satisfied. Cross-run identity of the candidate's
reported identity was frozen nowhere. "Material identity instability" is a criterion that first
existed at step 7.

Run 3's retained summary line in fact carries a 61-character identity against its own axis headers'
64. Whether that is a within-report inconsistency C3 reads, or a truncation in the retained
transcript rather than in what the run produced, is exactly the question the *not variance* part
puts to a reviewer — and this decision does not answer it, because it is not the ground the
rejection was recorded on and the close stands either way. It is named because a decision resting on
what the frozen readings actually read should not quietly pass over one of them.

GitHub [#49](https://github.com/joeloverbeck/skill-evidence/issues/49) reports the two
serializations as the same patch under `git patch-id --stable`, differing only in abbreviated versus
full blob IDs in their `index` lines. Nothing in this repository establishes that: only the
`cbfcec98…` serialization is retained, as `inputs/t1-reproduction/worktree.patch`, and the
`ab7db7f9…` bytes exist nowhere. That absence is the point rather than an oversight. An equivalence
a reviewer computes after seeing which arm it helps is the same defect as a witness chosen after a
result, so the claim could not rescue the candidate even with the bytes on disk. A review that wants
that comparison has to freeze its relation in step 4 and retain both sides at step 6 — which is why
the *not variance* part is unmet wherever the comparison was never frozen at all.

Both grounds are the same error at different depths, and
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
already names it one level up. *Evidence authorizes only the claim it bears*: a conformance record
does not bear an outcome claim, and by the same reasoning a reading both arms return bears a claim
about the trial and none about either arm. *Records are generated, never authored*: a criterion that
first appears at adjudication is authored. Step 4 already fixes witnesses before outcomes exist
"because one chosen after a result is not evidence"; identity relations are a reading class that
rule had not reached, and the attribution standard is what stops an unfrozen reading walking in
under the open regression bar.

The close stands. Records are append-only, so `rev_9f4689de-42c5-4e51-ac00-357c309da60a` remains
`candidate_rejected_validation` with its events and close receipt untouched, and this decision
reopens neither the review nor the candidate it rejected. It changes the next review.

## Considered options

**Keep a symmetric failure adverse, and require disclosure.** Rejected. It is the status quo plus a
paragraph, and it leaves the substrate spending an input fault as candidate-specific evidence while
saying out loud that it is doing so. Disclosure is not a check; a reviewer who discloses and a
reviewer who does not both reject the same candidate, so the rule has no observable consequence,
which [`../principles/inherited-prohibitions.md`](../principles/inherited-prohibitions.md) is the
standing objection to.

**Invalidate the whole trial a frozen-input fault touches.** Rejected as wider than the fault. A
faulty premise refutes one reading; the same trial's protected behaviors, deterministic checks, and
repository-state assertions were observed honestly and are exactly the evidence a reviewer needs to
tell an input fault from a real symmetric failure. Discarding them buys nothing, and if the trial
discarded is the reproduction trial it produces a review with no valid reading and no honest
disposition — see the consequence below, which is deliberately left open rather than answered with
machinery.

**Re-cut the fixture and rerun the affected trial.** Rejected, and already barred. Step 4's
long-course rule and step 5's first-witness rule both refuse a re-cut prompt or fixture after
results, on the ground that a task re-cut until its answer appears is a task shaped to its answer. A
fault discovered after unblinding is the strongest case for that rule, not an exception to it.

**Exact bytes always, with no equivalence relation available.** Rejected. It is unambiguous and it
makes every incidental serialization difference a finding — the same trap approached from the other
side, and one that would have rejected this candidate faster rather than more honestly. It also
forecloses a plan that legitimately needs to compare canonical forms, for no gain: the danger was
never that a relation exists, it was that one gets chosen after the result it decides.

**Admit a post-hoc equivalence check for defensive use only.** Rejected, though it is the option
that most directly rescues the T1 reasoning. A check that may only remove a finding looks safe
because it cannot manufacture an improvement, but it hands a reviewer a general instrument for
dissolving any adverse artifact difference after seeing it, and the reviewer choosing the
canonicalization already knows which arm it helps. GitHub
[#49](https://github.com/joeloverbeck/skill-evidence/issues/49) required any equivalence rule to be
frozen before outcome adjudication, and this is the option that clause excludes.

**Allow only frozen criteria to reject.** Rejected. It is the tightest possible answer to
adjudication-time invention and it deletes the gate's open regression bar, which exists so that a
severe unanticipated regression can stop a candidate. A validation plan cannot enumerate every way a
candidate can be worse, and a rule that says otherwise would ship a candidate whose harm nobody
thought to freeze a reading for.

**Void the criterion, freeze the relation, attribute the observation.** Chosen. Each of the three
rules answers one of the two grounds and none of them touches the gate's terms, so what a reviewer
must clear is what it always was, with one exception this decision states rather than hides: the
sameness narrowing above. What otherwise changes is that a claim under those terms must be
established from retained artifacts rather than asserted. The three report rows make the
difference visible — a reviewer who applies the rule and one who does not now produce different
reports, which is the observable the alternatives above lack.

## Consequences

- **Installed-package surface only.** No Rust API, command surface, recorded-event shape, or
  published schema moves, and the frozen corpora under `fixtures/skill-evidence/` replay unedited.
  The installed bytes do move, so it is still a minor bump under
  [`../releasing.md`](../releasing.md) §1 and every consumer runs `skills evidence install --force`.
  No package is retired or renamed, so nothing is owed to withdrawal.
- **No ADR is superseded, and two are reinforced.**
  [`0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`](0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md)
  keeps its boundary and
  [`0008-a-long-course-trial-expresses-an-accumulated-context-constraint.md`](0008-a-long-course-trial-expresses-an-accumulated-context-constraint.md)
  keeps its reach: a plan defect was never an instrument limit, and this decision says so at the one
  place a reviewer holding a voided criterion would be tempted to reach for one.
- **The open regression bar loses exactly one case, and the reference says so.** The sameness case
  named in the decision above is the only term of the gate this decision narrows. Every other
  unanticipated regression stays reachable, including one caused by text the candidate deletes and
  one emergent from the candidate as a whole, which is why attribution names a removal and a
  whole-candidate difference alongside an added clause.
- **Step 4 costs one more freeze.** Every sameness reading now carries a relation. The default is
  exact bytes and is free to state; the cost falls only on a plan that wants a canonicalization, and
  it is the cost of naming a command and the difference it absorbs.
- **A review a fault voids entirely has no disposition, and gets none here.** If a frozen-input
  fault ever voided the reproduction reading for every mechanism of a review that also reached no
  step-3 conclusion, none of the eight dispositions would be honest: `blocked_no_valid_test` asserts
  an instrument limit the fault does not establish and would retire the evidence from the gate on
  it, and `superseded_by_target_version` is simply false. That state has not occurred — the observed
  fault sits on a regression trial, where a reproduction trial's own witness rules already catch an
  input that fails to instantiate the triggering clause. Adding a ninth disposition would move the
  compiled enum, the published schema, and three consumers' readers for a shape nothing has
  produced.
- **What could reopen this.** One review whose reproduction reading is voided by a frozen-input
  fault with no step-3 conclusion beside it, which is the state the bullet above declines to build
  for. Also: an attribution standard that a reviewer satisfies for an observation later shown to be
  harness variance, which would say the four parts are checkable but not sufficient. A count of
  voided criteria is not that evidence, and
  [`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
  says why — it is throughput.
