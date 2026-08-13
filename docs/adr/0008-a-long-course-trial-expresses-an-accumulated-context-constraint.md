# A long-course reproduction trial expresses an accumulated-context constraint

Status: accepted (2026-08-12, GitHub [#48](https://github.com/joeloverbeck/skill-evidence/issues/48))

Amended: 2026-08-13, GitHub [#51](https://github.com/joeloverbeck/skill-evidence/issues/51) — the release forward-test reached a terminal state this decision makes ordinary and the enumeration had no name for; `not_reproduced_witnesses_expressed` is added. No instrument decision changed.

Supersedes the instrument half of
[ADR 0003](0003-no-new-instrument-for-conformance-only-evidence.md) for constraints of accumulated
context, volume, and run length. That decision's conformance/outcome ruling is untouched and still
governs; what it declined for a **conformance-only** population it did not decide for this one.

ADR 0003 named what could reopen it: a consumer feeling the limit in delivered work, or an
instrument proposal that expresses accumulated context without making landing provisional. Both
fired, the first on exactly one close, which §*Why* names and rests on. Fifteen
`blocked_no_valid_test` closes now stand across the three consumer stores — five in
`playbench`, four in `mundifold`, six in `what-we-bring-home` — ten of them recorded after that
decision. This ADR reads **five** of those fifteen closely — the three #48 nominates as its material
evidence, plus two more — and checks the recorded evidence classes of five others; the remaining
five are counted and never examined. Those five are exactly the closes recorded *before* ADR 0003,
so every one of the ten recorded after it was examined one way or the other.
It reads one further close that is **not** among them and is not a consumer's — this repository's
own `domain-modeling` `8f1a4f65-fc16-4cac-90aa-cc223195ad01`, one of three such closes in this
repository's own store, kept separate throughout and never added to the fifteen, because a
repository must not manufacture evidence for its own necessity. At least one of the five consumer
closes read closely is outside the population ADR 0003 measured:
`playbench`'s `rev_6a875d58-0bcd-4813-b037-d2c4e2040843` records its own trigger as
**outcome-graded**, and it reached no acceptance gate at all — it closed one step earlier, at step
4, on the **reproduction** instrument.

The premise that no trial could reach them was this repository's own text, and it does not survive
reading. `authorized-review.md` step 4 said: *"A trial executor starts fresh and short-context, so
a constraint that exists only deep inside a long run is inexpressible."* **Fresh** is computed and
load-bearing — distinct top-level sessions are what make the two arms independent. **Short** is
neither. A fresh executor given a task whose own work runs through many phases accumulates context
inside the trial. The instrument was never barred from running long; one sentence asserted it was,
and the two properties were conflated.

So the instrument gains a **long-course reproduction trial**: one raw task whose own work carries a
fresh executor to the scale the packet establishes, before it reaches the mechanism's failure
boundary, run paired on both arms under the blinding rules step 6 already imposes. The exit stays
for the residue one executor session cannot reach at all.

## Why

**The reopening condition is met, and it is met by a recorded classification rather than by this
decision's own reading.** The issue's census reproduces exactly: 15 closes, 10 recorded strictly
after 2026-08-08. The class a trigger bears is decided by the review that read it, at step 3, and
written into its report — never inferred here from a `consequence` field, because rework effort
recorded in that field is not by itself an outcome claim, and reading it as one is the severity
ladder mistake [`../principles/inherited-prohibitions.md`](../principles/inherited-prohibitions.md)
forbids.

Read that way, **the first disjunct is satisfied by one close, and that close is the whole of the
consumer warrant.** ADR 0003's clause names *"an assurance-producing skill degrading in a way its
store records as outcome evidence rather than conformance evidence."* `what-we-bring-home`'s
`rev_eb6dccae-387e-44d8-b56a-46cee85217d3`, closed 2026-08-11, matches every term of it. The target
is `grilling`, one of the four skills the census identifies as assurance-producing — the ones whose
deliverable *is* an assurance, alongside `code-review`, `writing-great-skills`, and
`skill-evolution-status`. Its report records two of its seven triggers as **outcome-graded**. Its
undecidable ground is *"reproduction instrument for all seven triggers. A fresh short-context
executor cannot vary genuine accumulated context load, lateness, or multi-phase duration."* And its
binding constraints are accumulation in terms: *"thousands of source lines or many tracker items and
dispositions, followed by intervening ratification or publication before a later reconciliation
exposes the miss"*; *"multiple TDD/implementation phases and successive mid-execution forks over a
long run after ledger initialization"*.

**That close is also the one this decision reaches**, which is what makes the disjunct load-bearing
rather than decorative. It is barred from a trial by the sentence being removed here, and by the
predecessor rule this decision also qualifies: it records that `rev_a633c2a4-70a2-484e-9a52-c32e7f6a0cea`
*"already passed an equivalent transition-rich unchanged-current trial 3/3, so repeating that fresh
trial is forbidden waste rather than an expression of these incidents' binding condition."* A
long-course reproduction is not that equivalent trial, and step 4 now says so.

**The rest of the post-ADR consumer population does not carry the clause, and this ADR does not
pretend it does.** Every other consumer close whose report records evidence classes records
conformance-only triggers: `what-we-bring-home`'s `rev_0ad6193e`, `review_cd0ba3c0`, `rev_019ff0f9`,
`review_4ae2b1d4`, and `review_f114e04f`. `playbench`'s `rev_6a875d58-0bcd-4813-b037-d2c4e2040843`
records an outcome-graded trigger but is not assurance-producing and closed on an unrecorded
constraint, discussed below. `mundifold`'s two post-ADR closes and `playbench`'s
`rev_d26b8ab8-80d3-4c57-a134-da70fd7adf89` record no evidence classes at all, so they establish
nothing either way about the clause, whatever they show about reach. One qualifying close is what
the clause asks for — it is written in the singular — and one is what there is.

**The second disjunct is satisfied independently.**
[`../../reports/skill-evolution-independent-instrument-research-report.md`](../../reports/skill-evolution-independent-instrument-research-report.md)
proposes an instrument for accumulated context that keeps landing same-session — *"an instrument
proposal that can express a binding constraint of accumulated context without making landing
provisional"*, in the clause's own words. It depends on no close at all, and the clause is
disjunctive, so either alone would reopen the decision.

**Neither disjunct is a gate, which is why the count matters less than it looks.** ADR 0003 headed
that consequence *"What could reopen this, without prejudging what will"* and closed it *"This ADR
sets no bar on what evidence a later proposal must clear."* The clause illustrates rather than
admits. So the warrant here does not improve by adding closes that do not carry it, and does not
fail because the five pre-ADR closes went unexamined. What the wider population contributes is only
this: consumers reach this exit often, and at least one reached it on a ground the shipped text
already denied — which is a reason to look at the exit, not itself a reason to move it.

**These closes are blocked on the reproduction instrument, not on the acceptance gate.**
`blocked_no_valid_test` is chosen at steps 4 and 5, when no trial can express any mechanism, so
these reviews never reached step 7 at all. Nothing here gives conformance evidence an acceptance
test, and the pairing clause in
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
is untouched.

**ADR 0003's option (b) is left without its recorded ground, and this decision does not re-decide
it.** That option — give conformance evidence its own blind acceptance test — was rejected on
exactly one basis: *"it cannot reach its own population … A trial executor starts fresh and
short-context, so it cannot express that constraint whatever it then measures."* That is the
sentence this decision finds conflated, so for a constraint of accumulated volume or run length the
recorded ground no longer supports the rejection. It does not follow that (b) is revived. Building
an acceptance test that grades conformance is a far larger change than lengthening a reproduction
trial: it engages the pairing clause directly, where this decision does not touch it at all. So
(b)'s status is **open rather than declined**, awaiting its own decision on its own merits, and
saying that is more honest than leaving a rejection standing on a premise this same commit
retires.

**The reach check is per close, and it is done from what each close said its constraint was.** A
decision that narrows an exit owes a check that the new instrument reaches recorded closes, read one
at a time, never inferred from a trigger's `run_condition` field — a field describing a long session
does not establish that the close turned on length. Read that way the instrument reaches some of
these closes and not others, and the split is worth stating, because the ones it misses show where
the real limit lies.

**Four closes are reached.** `rev_eb6dccae-387e-44d8-b56a-46cee85217d3` is the first, for the
reasons above. `playbench`'s `rev_d26b8ab8-80d3-4c57-a134-da70fd7adf89` — the second close #48
nominates as its material evidence, and `rev_6a875d58` and `rev_eb6dccae` are the first and third —
is the second reached: its note records *"three explicit-contract violations after 10 TDD
cycles, 8 review-reentry rows, and 7 review passes in one 24-file run"* and names the constraint
*"target-compliance loss from contract-read recency and ledger-assembly distance deep in a long
run"*. That is one run, and a task whose own deliverable requires those cycles reaches it. Its
report records no evidence classes, so it adds reach without bearing on the clause. It also turns on
the predecessor rule this decision qualifies: *"Same-hash predecessors showed fresh readers
satisfying inventory derivation, replayable-command, and already-satisfied-red duties"* — a fresh
short read passing is exactly the reading a long-course trial exists to test against scale.
`what-we-bring-home`'s `review_4ae2b1d4-e0d5-439b-8c03-1c7cd619786d` is the third,
though its triggers are conformance-only: it rests on late evidence-custody failures whose recorded
conditions are work volume inside one session — *"One GitHub issue implementation with three
red-green cycles … the discrepancies surfaced after implementation across Standards review passes R1
through R3"*; *"reconciled 140 changed selectors"*; *"a five-commit GitHub #27 implementation with
228 final tests"*. Each is a scale a task's own deliverable can require, none names elapsed
wall-clock or accumulation across sessions, and most disclaim elapsed time outright — *"elapsed
session time is not determinable"*. This repository's own `domain-modeling`
`8f1a4f65-fc16-4cac-90aa-cc223195ad01` is the fourth, at *"roughly forty-five tool calls"* in one
companion-mode session, and is corroboration only.

**Two closes are not, and neither would be helped by a longer course.** `mundifold`'s
`rev_f14fdb94-aae5-47aa-8b21-d2c2e3fd6da6` records a constraint, in its close note, that is
*"context distance rather than volume: GitHub #18 slipped at six selectors"* — volume falsified for
it by its own smallest incident — and is blocked on two grounds this decision expressly preserves,
both quoted from that same note: *"No finished trial run yields any observable in its own output or
artifacts that shows whether the rule was at attention distance when the artifact was authored"*,
which is step 4's witness rule, and an incidence of *"only 3 of 20 qualifying uses on this hash"*,
too low for a paired trial set of the permitted size to separate a candidate effect. Its review
report states the same grounds in different words; the close note is quoted throughout because it is
the recorded event. `playbench`'s `rev_6a875d58-0bcd-4813-b037-d2c4e2040843` records its
ground, in its close note, as *"Fresh short-context trials cannot express the **unrecorded** load or
skill-consultation condition"*; its report puts the same ground as *"an unrecorded long-run
context-distance or skill-consultation condition"*. The second phrasing names the very class this
decision now reaches, and it changes nothing, because the operative word in both is **unrecorded**:
step 4 already holds that *"only a constraint the evidence packet establishes can support an unable
to be expressed marking"* and that *"an unestablished constraint keeps its trial slot"*, whatever
kind of constraint is asserted. That close took an exit the shipped text already denied it. Had the
constraint been recorded, this decision would now send it to a trial — which is the sharpest
illustration available of the difference between the two failures.

**So one consumer close carries both the clause and the reach, and the rest carry less.** That close
is `rev_eb6dccae`, and everything this decision claims about consumer pressure rests on it.
`rev_d26b8ab8` adds reach and records no classes. The third reached consumer close,
`review_4ae2b1d4`, records all five of its triggers as
*"conformance-only … behavior-neutral evidence repair or additional review effort before final
acceptance, not a demonstrated deficit in delivered product behavior"*, so it sits in the population
ADR 0003 already measured and adds reach without adding pressure. The self-hosted occurrence adds
neither, because
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
bars this repository's own store from standing as pressure for a change to this repository. A single
qualifying close is a thinner base than fifteen closes made it look, and it is stated at its true
size rather than at the size the count suggests.

**Reaching a conformance-only close is worth something, and it is worth exactly one thing.** Its
triggers do not become adjudicable: step 7 grades outcome, so a conformance-only trigger still
routes to `--instrument-limited` on ADR 0003's acceptance-gate ground, which this decision leaves
untouched, and the same evidence still retires. What changes is upstream of that. A review that
could only close `blocked_no_valid_test` now builds a candidate, runs it against the current arm,
and can reach `resolved_by_change` — the skill itself gets repaired, which is the outcome the
lifecycle exists for. The trigger's fate and the target's fate were always separable, and this is
the case that separates them.

One close is a narrower warrant than fifteen, and it is the warrant this decision actually has. It
is sufficient on its own terms — the clause is written in the singular and sets no bar — and it does
not stand alone anyway, for a reason independent of any close: **the sentence being removed was
false.** A fresh executor is not thereby a short one, and shipped instructions asserting otherwise
send every future reviewer to an exit on a premise that does not hold, whatever their evidence bears.
Correcting that is warranted by the text being wrong; `rev_eb6dccae` shows what it costs when the
text is followed, on an assurance-producing skill, in a consumer store, with outcome evidence, on
the accumulation constraint the removed sentence declared inexpressible.

**The corroborating occurrence shows the shape at its sharpest.** This repository's own
`domain-modeling` review `8f1a4f65-fc16-4cac-90aa-cc223195ad01` classifies both its triggers
*outcome-graded*, records its undecidable ground as *reproduction instrument*, and is the second
same-shape ruling on these target bytes after `5d1d35bb-95d9-4316-a36f-3c13559250ff`. Its retained
`run_condition` citations establish the constraint as *"roughly forty-five tool calls"* in one
companion-mode session, with the deviations landing *"in the ratified write near the end of the
session"*. That is a scale a single fresh executor can reach by working. It is corroboration rather
than the consumer pressure this decision rests on, because a repository must not manufacture
evidence for its own necessity.

**Landing does not become provisional, which is what ADR 0003's option (d) could not avoid.**
Acceptance stays same-session: the same frozen plan, the same blind paired trials, the same step 7
gate, the same `record-validation` seam with its `--trials` floor. A long-course trial is a trial
with a longer task, not a new authority. *"Passing validation is not acceptance"* still holds, and
the human still accepts or does not.

**The residue keeps the exit honest.** Some constraints remain out of reach of one session:
accumulation across separate sessions, and elapsed wall-clock a run cannot produce. Those keep
`blocked_no_valid_test` and ADR 0002's exit, so no gate is left in a state whose only exit is
evidence its own structure prevents from existing.

## Considered options

**Decline again and record the continuing cost.** The cheapest, and it was the live alternative.
Rejected because the reopening condition ADR 0003 wrote is met, and because the ground it would
have to be declined on — that a fresh trial cannot run long — is the sentence this decision found
to be wrong.

**Commission the external pilot.**
[`../../reports/skill-evolution-independent-instrument-research-report.md`](../../reports/skill-evolution-independent-instrument-research-report.md)
recommends a bounded external full-course comparison with independent actors, sealed arm
identities, a hidden provenance ledger, held-out mutation calibration, and predeclared statistics.
Not chosen for this issue, and not refuted: it is a research program whose own Stage 0 is the
decision to commission it, with no repository change before Stage 12 behind two further owner
gates, so adopting it would answer #48 with an authorization rather than the implementation-ready
method the issue asks for. Its **method family is what is adopted here**, right-sized. That report
scoped itself to the quarantined self-hosted operator reviewing its own package, where step 6's
ordinary blinding cannot supply independence and the external apparatus has to. For a consumer
reviewing an ordinary target skill, step 6 already supplies it — fresh sessions, no diagnosis, no
version label, no access to the evidence store. Building the apparatus anyway would be a lifecycle
platform beyond the smallest slice real consumer use has proved necessary.

**Captured-trace replay or checkpoint continuation.** Resume an executor from a saved long-context
state and swap the arm. Rejected on pairing: the context was accumulated under one package, so the
candidate arm inherits a history the current package produced and the two arms are no longer
comparable. The research report independently holds checkpoint continuation insufficient without a
clean-start full-course anchor.

**Diagnose from accumulated evidence; validate longitudinally.** Foreclosed. ADR 0003 rejected it
on three counts, and #48 puts provisional landing and longitudinal acceptance out of scope pending
their own authority decision.

**Let trial cost keep the exit.** Considered and declined at the point of ratification. A
long-course trial is genuinely expensive, and a reviewer facing that cost could be permitted to
close `blocked_no_valid_test` under a disclosed budget bound. Rejected because such a close would
record that *this instrument cannot test the evidence* when what happened is that nobody wanted to
pay for the test — a caller assertion promoted to a derived fact about the instrument, and a
retirement of real evidence from the gate on a false warrant. Expense decides whether to claim the
review. It never decides what a close asserts.

**Freeze a long-course paired reproduction trial.** Chosen.

## Consequences

- **The installed-package surface moves, and only that one.** `skill-evolution/references/authorized-review.md`
  gains the long-course routing in step 4 and two report-template lines — the frozen scale with its
  established source, and separately whether the runs reached it. No recorded-event shape, no
  published schema, no Rust API, no CLI surface, no new event type, no new field on any record. The
  `record-validation` seam is untouched, which is why this needed no code. Consumers need
  `skills evidence install --force` to receive it, per [`../releasing.md`](../releasing.md) §5 and
  §6; nothing is retired or renamed, so no consumer deletes a directory by hand.
- **Reviews of an accumulation-bound mechanism get materially more expensive, deliberately, and
  the appetite is bounded.** Only the reproduction trial slot goes long-course — the adjacent case
  and the core-regression case stay ordinary — and the existing incidence-sized run count still
  governs. That is the whole of the accepted burden: no harness, no scheduler, no new command, and
  no maintainer release work beyond the forced install this change already requires.
- **A mis-sized long-course task lands on the honest exit rather than being re-cut.** Step 5's rule
  is unchanged: a first witness reading unexpressed stops that trial and marks the mechanism unable
  to be expressed, even where better sizing would have reached the scale. That cost is accepted on
  purpose. A task re-cut until its scale appears is a task shaped to its answer, which is the
  failure mode [#41](https://github.com/joeloverbeck/skill-evidence/issues/41) recorded, and the
  frozen-before-results discipline is worth more than a recovered trial.
- **`blocked_no_valid_test` keeps its meaning and loses one route in, not a measured population.**
  Of the five consumer closes read closely above, three would have taken a long-course trial instead
  of this exit, and so would this repository's own; how many of the ten not read closely would is
  not known, and five of those ten predate ADR 0003 entirely. It still asserts that this
  instrument cannot test what it covered, still retires that evidence from the gate at ADR 0002's
  reach, and still concludes nothing. What changes is which reviews may honestly reach it: an
  accumulation, volume, or run-length constraint the packet establishes now takes a trial rather
  than settling the question by itself. **The narrowing is to the reachability ground alone**, and
  that ground is one of several. A constraint with no available witness, an observable that cannot
  read unexpressed, an incidence too low for the permitted trial set to separate arms, and step 5's
  first-witness reading all still route a mechanism to unable to be expressed on their own grounds —
  `mundifold`'s close above turns on the first and third of those. A review can therefore still
  reach this exit honestly without any constraint being unreachable, and the cross-session and
  elapsed-wall-clock residue is what remains of the reachability ground specifically, not of the
  exit.
- **A terminal outcome that was rare becomes ordinary, and it needed a name.** Before this
  decision an accumulation-bound mechanism was marked unable to be expressed and its review closed
  `blocked_no_valid_test`, so the reading *not reproduced with witnesses expressed* arrived mostly
  alongside an untestable sibling — which is the only shape `mixed_no_candidate` was defined for.
  A long-course trial changes that: the mechanism now gets tested, and a trial that reaches its
  scale without the failure recurring leaves every mechanism on that single reading. The
  enumeration had no name for it while still requiring one — *"Name the terminal outcome in the
  report and completion"* — so a reviewer could not comply. `not_reproduced_witnesses_expressed` fills
  it, on the same `monitor_for_recurrence` disposition. This was found by the `0.11.0` release
  forward-test on its first run, not predicted here, and it is recorded as this decision's
  consequence because this decision is what made the state common.
- **Intermittency's routing outcome is unchanged, and deliberately so; only its stated ground
  moved.** The sentence this decision rewrote carried volume, late-run, and intermittent conditions
  together to one exit. Splitting it moves the first two and leaves the third reaching that same
  exit, because nothing in the reopening evidence bears on an intermittent condition and narrowing
  an exit no evidence reached would be this repository inventing its own importance. What did change
  is why: the clause grounded itself on *"a fresh short-context single-run trial"*, and this decision
  retires the short-context half of that phrase everywhere. Left standing, it would have invited the
  opposite of what is intended here — a reviewer arguing that a long-course trial escapes the clause
  and so narrowing the intermittency exit by accident. The ground is now the single run, which is
  the reason one run cannot express an intermittent failure whatever its length.
- **A claimed review that cannot afford its trial has no new exit, and this decision creates none.**
  Sizing is known at step 4, after step 1 has already appended `review_started` and taken the lock,
  so the maintainer's judgment about whether to spend the session lands on an already-claimed
  review. Cost is not a ground for `blocked_no_valid_test`, and no unclaim or abandon route exists
  or is being built — that would be a command surface, which this decision deliberately does not
  reach. The review runs its trial, or it stays claimed until a later review supersedes it on a
  moved target. Accepted and disclosed rather than repaired here.
- **A predecessor's short-context ruling stops binding.** Step 4's rule against re-deriving a
  same-hash predecessor's finding by rerunning equivalent trials stays, and a long-course
  reproduction is now stated not to be an equivalent trial. This is called out because the
  occurrence that motivated this decision declined to run any current arm partly on its
  predecessor's ruling, so leaving the rule unqualified would have foreclosed the instrument in
  precisely the case it was built for.
- **No retired evidence comes back.** ADR 0002's retirement reach is untouched, no unretirement
  event exists or is being built, and the fifteen closes above stay closed. The route forward is
  the one already relied on: evidence recorded after a close drives the gate, and editing a target
  clears the hash. The corpus a later review starts from is fuller, not restored.
- **ADR 0003's other half stands.** Conformance-only triggers still reach no verdict from an
  outcome gate, still route per trigger at step 9, and still retire as untestable coverage. This
  decision narrows one instrument limit and creates no acceptance test for a claim its evidence
  does not bear.
- **What would reopen this.** A long-course trial that demonstrably reaches its established scale
  and still cannot separate the arms would say the instrument, not the trial length, is the limit.
  So would consumer practice showing the cost is unbearable in the ordinary case — which is a
  reason to revisit this decision, and never a reason to record an instrument limit in a close.
  **Resting on one close is itself a reopening condition, in both directions.** If a later review of
  `grilling` runs the long-course trial `rev_eb6dccae` could not and the arms separate, the single
  close was enough. If instead the closes that follow keep arriving with constraints that have no
  witness or too little incidence — the shape `mundifold` recorded — then the binding limit was
  never trial length, this change bought little, and the honest response is to say so rather than to
  lengthen trials further.
