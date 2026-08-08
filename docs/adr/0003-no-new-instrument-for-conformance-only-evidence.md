# The evolution gate gets no new instrument for conformance-only evidence

Status: accepted (2026-08-08, GitHub [#2](https://github.com/joeloverbeck/skill-evidence/issues/2))

Skill Evidence Capture's ladder is disjunctive. Two of the four things that reach `material_failure`
are outcome claims — *"material rework required"*, *"wrong work caught before consequential
reliance"* — and two can be satisfied by **conformance** alone: *"an expected output contract
violated"* and *"a core behavior skipped"*. An incident resting on those says the run did not do
what the skill said and asserts nothing about the delivered work. The census identifies that
population by the `consequence` field rather than by rung, and this ADR follows it. Skill
Evolution's step 7 acceptance gate grades a candidate on **outcome** — whether it produces
materially better delivered work under blind comparative trials. So a gate those incidents open
meets an acceptance test their evidence cannot satisfy, and the review has nowhere to land even
when its diagnosis is right. We are building **no new instrument** for that class.
[ADR 0002](0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md) already supplies the
honest exit, and this evidence retires there.

## Why

The mismatch is real and is named in adopted text —
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md),
*Evidence authorizes only the claim it bears*: *"A gate whose acceptance test is an **outcome**
improvement may not be opened solely by **conformance** evidence and then treated as having reached
a verdict when no outcome deficit can be demonstrated."* The clause prohibits the compound. It does
not say in terms whether the opening alone is barred, and this decision reads it as barring the
verdict rather than the opening — which is why the gate below still opens on this evidence and
still costs a review. That reading is an interpretation, recorded here as one. The same document
adds *"A severity ladder is not proof of harm,"* which
[`../principles/inherited-prohibitions.md`](../principles/inherited-prohibitions.md) carries as
*no severity ladder presented as proof of harm*. Between them the pairing clause says this pairing
is wrong. It does not say which half to repair, and the
[Constitution's adoption record](../principles/README.md) says so explicitly: adoption *"does not
settle the open question in `joeloverbeck/skill-evidence#2`; it supplies the clause that question is
decided against, and the decision itself remains the owner's."*

Exactly one of the repairs the issue thread reached has since landed, and it is not this decision.
**Option (c) landed in `750cc61`, as one half of ADR 0002.** The projection now says what a
`blocked_no_valid_test` close did and did not conclude — `instrument_limited_incident_ids`,
`queued_behind_instrument_limited_review`, and *Retired as untestable* — so the close is no longer
mistakable for "reviewed, nothing found." The other half of that ADR, retiring the covered evidence
from the gate so it stops being parked, answers #1 rather than this issue — ADR 0002 is accepted
against #1 — and the census ruled (c) *"necessary but not sufficient"* precisely because *"it
supplies no exit."*

A second repair landed nearby and is easy to miscount as this issue's. `96e5dab` closed
[#4](https://github.com/joeloverbeck/skill-evidence/issues/4), which the census run filed
separately — *"a defect in capture, not in evolution's acceptance gate … separate from both #1
and #2."* `skill-evidence-capture/SKILL.md` now requires a run to record the effect as
undetermined and name the check that did not run, and closes the tiebreak route out of that
grade: *"an undetermined consequence is not an argument for grading down."* So it does reach
grading — in the opposite direction from option (a), which would have graded this class down. It
is not (a) partly landing, and it settles nothing here.

What ADR 0002 deliberately left open is the residue this ADR closes. Its penultimate consequence
reads: *"a conformance problem this instrument cannot test goes quiet after one review instead of
reappearing … GitHub #2 is where the question of an instrument that could test it belongs; this
decision does not answer it and does not depend on it."* The answer is that no such instrument gets
built now.

Three things carry that.

**The population is small and its cost is already accepted.**
[`../../reports/conformance-evidence-census.md`](../../reports/conformance-evidence-census.md)
measured 1,083 events across the three consumer stores: 330 incidents, 94 graded `material_failure`
or worse, 52 concluded reviews of which 8 landed a change and 14 rejected a candidate on its merits.
The mismatch bites 5 of 52 reviews. The gate works; a tenth of it reaches an acceptance test its
evidence cannot meet, and ADR 0002 gives those an exit.

**The parking is fixed; what remains is a cost, not a forbidden shape.** The issue's own complaint
is that such a review *"has nowhere to land even when its diagnosis is correct."* That stands, and
this decision accepts it rather than repairing it; see the first consequence below. What made it
worse was not #2's to fix: the issue closes by pointing at *"#1, on `blocked_no_valid_test`
re-entry, **which is what makes the parking permanent**,"* and the parked state was the shape
`evidence-substrate-integrity.md` forbids by name — *a disposition that leaves its triggering
evidence permanently parked … with no route out that the responsible party can actually take*. ADR
0002 ended that for `blocked_no_valid_test`, which is why the residue is a known limit of one
instrument — visible in the projection and in the census — rather than a trap. ADR 0002 also
records one place its repair deliberately did not reach: a same-hash
`superseded_by_target_version` close still labels retained evidence `queued_pre_close_evidence`
while nothing was concluded about it, a residual that ADR calls *"accepted, not overlooked."*
Nothing here changes it.

**The tracker does not authorize the work.**
[`../principles/mission-and-scope.md`](../principles/mission-and-scope.md): *"This repository's
issue tracker is not a value stream … An issue records that something was observed; it does not
authorize the work."*
[`../principles/inherited-prohibitions.md`](../principles/inherited-prohibitions.md) forbids
treating an open issue count as an obligation to continue, and `evidence-substrate-integrity.md`
lists *treating an issue in this repository's own tracker as an obligation to change this
repository* among the ways the substrate manufactures evidence for its own necessity. Closing #2
with a recorded decision and no code is a legitimate terminal answer to a question, not an evasion
of one.

## Considered options

What the owner ratified is the last entry below: record the decision, build no instrument, change no
code. The entries above it are why that was the option on the table — (a) and (b) are the census's
conclusions, which the owner's decision leaves standing rather than independently re-rules, and (c)
is a statement of what already landed elsewhere. Only (d) and the routing option were live
alternatives at the point of decision. The census is agent-authored analysis, and under
[`../principles/README.md`](../principles/README.md) it is evidence and a proposal, never
acceptance.

**(a) Grade conformance-only deviations below the evolution threshold.** The cheapest, and it would
have required the capture ladder to stop calling a skipped core behavior `material_failure`.
Rejected on two counts from the census. The 14 incidents whose `consequence` does not assert harm
fall in five skills and none of the other twelve, and the census identifies the property four of
the five share: their deliverable *is* an assurance — `code-review`'s that both axes were checked
and reconciled, `writing-great-skills`' a completion proof, `grilling`'s a ratified recap,
`skill-evolution-status`' an exact relay. For those, conformance *is* the outcome and no separate
artifact is left to be worse. On the census's §2 split, ten of the thirteen material-or-worse
incidents `code-review`, `grilling`, and `skill-evolution-status` have recorded fall in this
group — §3's revision moves one of the ten to harm-established — so grading down would take most
of what those three have, while the twelve skills that record none would be untouched.
And it would reward concealment: the census's §3 revision found that only one of the fourteen is
harm genuinely absent and independently checkable, two record harm outright, and eleven are
unobservable because the detector was the skipped step. Skip half a check and the discrepancy
surfaces; skip all of it and the record reads clean. The lowest grade would land on the failure
mode that best destroys evidence of itself. #4 repaired the reporting defect underneath this
option; the `material_failure` rung (a) would have lowered stays where it is.

**(b) Give conformance evidence its own blind acceptance test.** Trials measuring whether
executors comply rather than whether artifacts improve. Most faithful to what the evidence says,
and it cannot reach its own population: all ten the census tabulates with a recorded run condition
name accumulated volume or elapsed run length — five to seven review passes, 10–14 sub-agents,
18–30 file diffs, "present from pass R1 and persisted" — and the census says the records carrying
none *"neither support nor weaken this."* A trial executor starts fresh and short-context, so it
cannot express that constraint whatever it then measures. This is the same wall step 4 of
`authorized-review.md` already tells a reviewer to check for.

**(c) Give the no-conclusion close its own disposition.** Necessary, insufficient, and already
adopted — in ADR 0002, as part of a larger decision rather than instead of it. It made the
projection honest; it supplied no acceptance test. It is recorded here as landed, not as chosen.

**(d) Diagnose from accumulated evidence; validate longitudinally.** The census's own proposal:
accept a candidate when the omission rate drops in subsequent real runs, measured from the
evidence stream that produced the diagnosis. The diagnosis half is sound — the census survives its
own caveats on *"the recurrence rate and the cross-skill span,"* and counting recurrences of one
known omitted step needs no classifier at all. Rejected on the acceptance half, on the census's
own three counts: no precedent here, a longitudinal signal confounded by everything else that
changes between runs, and no way to distinguish "the fix worked" from "nobody ran a long session
lately." Those are enough on their own, and this decision does not add a fourth.
Its cost weighs against it too, and is also the census's: acceptance stops being same-session and
a revision lands provisionally. Whether that reaches `evidence-substrate-integrity.md`'s *"Passing
validation is not acceptance. A candidate that survives its trials has earned a proposal, and the
human accepts or does not"* is arguable — (d) keeps a human decision, it moves when the decision
is made — and this ADR does not settle it. It notes only that if a future proposal does reach that
text, [`../principles/README.md`](../principles/README.md) §*Constitutional change* is the route,
not an ADR.

**Route the blocked close to method-gap research.** The issue's own candidate 3: have the close name
[`commission-method-gap-research`](../../.claude/skills/commission-method-gap-research/SKILL.md), so
the evidence lands somewhere instead of parking. **Half its purpose was met elsewhere and half is
being declined.** ADR 0002 ended the parking, so the urgency is gone; the *lands somewhere* half is
not met and is not being built, which is the same cost the first consequence below records. Two
objections that suggested themselves do not survive checking and are recorded as withdrawn:
`commission-method-gap-research` is indeed not among the four packages `src/assets.rs` installs,
but the installed `method-gap-research-status` already names it, links into it, and ends by handing
the operator a `$commission-method-gap-research` line — so the missing package is a real
pre-existing cross-reference defect worth its own issue, not a reason to decline this. Nor would
naming it schedule anything: that same package shows the shape is *reporting* a result, which
`inherited-prohibitions.md`'s *packages report results, they do not schedule each other* permits.
The ground is simply that the owner declined it, with the parking already fixed.

**Build no new instrument; record the limit and close.** Chosen.

## Consequences

- **A conformance-only cluster this instrument cannot test still opens the gate, burns one review,
  and closes `blocked_no_valid_test`.** That review costs a session and produces a report and a
  disposition, not a repair. The cost is accepted here for the second time, with the census's
  measurement of how often it falls due: about one review in ten.
- **Retiring the cluster remains the price of that close.** ADR 0002 retires the whole covered
  cluster, so the symptom goes quiet after one review rather than reappearing. Nothing added here
  brings it back, and no route back is being built.
- **Nothing changes on any of the three consumer surfaces.** No recorded-event shape, no published
  schema, no installed package, and no Rust API. This decision is documentation only, needs no
  release, and needs nothing of any consumer. `consumer-contract.md`'s three surfaces are untouched
  on purpose: the decision was that the mismatch does not warrant reaching them.
- **The capture ladder keeps grading a skipped core behavior `material_failure`.** That grade
  records the deviation's kind, which is what a ladder is for. Reading it as harm is what
  `inherited-prohibitions.md` forbids — *no severity ladder presented as proof of harm* — and
  #4's repair is what keeps the `consequence` field from making that mistake on the ladder's
  behalf.
- **The evidence stays in the ledger forever.** Retired from the gate is not deleted; every event
  remains in `events.jsonl` and remains readable. A later decision to build an instrument starts
  from a fuller corpus, not an empty one.
- **What could reopen this, without prejudging what will.** A consumer feeling the limit in
  delivered work — an assurance-producing skill degrading in a way its store records as outcome
  evidence rather than conformance evidence — or an instrument proposal that can express a binding
  constraint of accumulated context without making landing provisional. The census's own limits
  leave ample room: it is a snapshot at 2026-08-08, `mundifold` was being appended to during
  extraction, and `what-we-bring-home` contributed 8 events and no material failures, too young to
  inform anything. This ADR sets no bar on what evidence a later proposal must clear; it records
  that today's did not warrant an instrument.
