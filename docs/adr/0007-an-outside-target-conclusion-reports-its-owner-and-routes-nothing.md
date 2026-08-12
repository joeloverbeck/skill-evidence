# An `outside_target` conclusion names an owner and reports it; it does not route evidence

Status: accepted (2026-08-11, GitHub [#37](https://github.com/joeloverbeck/skill-evidence/issues/37))

Amended: 2026-08-12, GitHub [#46](https://github.com/joeloverbeck/skill-evidence/issues/46) — the
reopening clause met its first real occurrence and did not fire; a post-session handoff into the
owner's store is declined on a sharpened ground, and the census that decided it is recorded below.
No routing decision changed.

An adjudicating close that concludes `outside_target` records a positive external owner — a kind
drawn from the shipped ownership taxonomy and a stable reference — per concluded event. That owner
is reported: it survives in the event, in the close receipt, and in the user-facing completion,
where the repository owner can act on it.

**It is not routed.** No command writes a second evidence store on behalf of a named owner, no
projection reports an attribution as an obligation outstanding against that owner, and no close
refuses for want of proof that the owner's store holds a corresponding incident. The named owner's
gate is opened by the owner's own recorded uses, exactly as every other gate is.

The installed reference's step-3 instruction — *"Route outside-target evidence to its owner
factually"* — is replaced with a statement of what naming the owner actually accomplishes under
this workflow. The prohibitions beside it, against proposing an unsanctioned repair and against
editing another owner from the review, are unchanged.

## Why

The observed occurrence is well-formed and is the green control, not the defect. Review
`a382d1f5-312f-42b8-b683-21f3910df3a3` on `.claude/skills/implement` concluded two triggers
`outside_target` and named the owner in `note` as *"which the current code-review contract owns."*
[#36](https://github.com/joeloverbeck/skill-evidence/issues/36) then recorded the attributed
mechanism as its own issue and required that its repair "proceeds only through a freshly authorized
Skill Evolution review" — while `code-review`'s gate reads `queued_pre_close_evidence` and needs a
new open incident recorded after its last disposition, which an attribution cannot supply.

That is real, and it is not what it looked like. Three separate premises had to be checked before a
mechanism could be justified, and each failed:

**The gate is not trapped, and it is not even the relevant gate.** This repository's `code-review`
gate holds 21 qualifying uses and 5 open incidents on hash `58ee690e`, with a standing
`ten_use_unresolved` candidate held only by the post-review-incident test. But all four copies of
that skill are byte-identical, so all four repositories gate on the same hash — and the other three
are `eligible` right now: `playbench` at 110 qualifying uses, `mundifold` at 38,
`what-we-bring-home` at 13. `playbench` had in fact already claimed one and closed it
`candidate_rejected_validation` before [#36](https://github.com/joeloverbeck/skill-evidence/issues/36)
was filed here.
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
bars *"a gate whose acceptance test is … evidence the gate's own structure prevents from existing."*
Nothing prevented that evidence; it had already been produced, in a store this repository's
`prior_reviews` cannot see.

**The gate is not the only authority.**
[`../principles/mission-and-scope.md`](../principles/mission-and-scope.md) makes the repository
owner the semantic authority. `.claude/skills/triage/SKILL.md` was edited directly outside any
review during the same program's coordination work; the triage store holds no `change_landed`
event for it. A direct edit is available and costs the target's accumulated evidence when its
content hash resets. An evidence-driven authorization is one route to a repair, not the only one.

**Evidence not reaching another target's gate is the design working.** A gate derives from
observations *of its own target*. An attribution is one reviewer's assertion about a skill they did
not use, made from a bounded packet that deliberately excludes the other skill's ledger. Writing it
into the owner's store would author a record rather than generate one, which the same principle
document forbids outright.

What genuinely fails is smaller and is an instruction defect. Step 3 names an obligation the
workflow cannot discharge, cannot evidence, and cannot report: a review that honors it and a review
that ignores it produce byte-identical output. An instruction with no mechanism and no observable
is prose infrastructure, and
[`../principles/inherited-prohibitions.md`](../principles/inherited-prohibitions.md) bars *"no
prose-only return contract or hand-maintained routing graph between skill packages used as the
interoperability layer; packages report results, they do not schedule each other."* Repairing the
instruction, and making the owner reach the human through the receipt and the completion, closes
that gap without adding machinery for a trap that was measured and found absent.

## Considered options

**Record the obligation, discharge it manually.** Rejected. The close would emit an outstanding
routing obligation and the projection would report attributions whose owner store holds no
corresponding incident. This is the hand-authored routing record
[`../principles/inherited-prohibitions.md`](../principles/inherited-prohibitions.md) bars, and its
central artifact — a per-attribution discharge status — is a certification nothing generates and
nothing consumes. It also needs a second payload key beyond the owner, and every key on the
installed surface costs a release and a three-consumer ritual.

**Require discharge before append.** Rejected. The close would refuse until the operator supplied
proof that a corresponding incident exists in the named owner's store. It is the strongest
guarantee and the worst incentive: the cheapest way past a refusal is to write the incident the
reviewer believes ought to exist, which is precisely the authored record
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
forbids. It also introduces a cross-store precondition the command surface has never had, against
three consumers holding append-only histories under a one-store-per-invocation contract — a
consumer-contract change of the kind
[`../principles/consumer-contract.md`](../principles/consumer-contract.md) says a version bump does
not protect.

**Accept that routing is outside the machinery and record only that.** Rejected as incomplete
rather than wrong. Its ruling is the one adopted here, but stopping there leaves step 3 instructing
an act nothing can perform, and leaves the named owner buried in a `note` field no surface reads.
The instruction would keep producing issues shaped like
[#36](https://github.com/joeloverbeck/skill-evidence/issues/36), written against a gate that was
never going to fire for them.

**Repair the instruction and report the owner.** Chosen. It keeps the correct ruling — attribution
is a conclusion, not a transfer — and pays the one cost that ruling incurs: making the conclusion
visible to the only party who can act on it. It adds no payload key of its own, riding the
per-event owner key that
[#34](https://github.com/joeloverbeck/skill-evidence/issues/34) adds for its own reasons. It adds
no cross-store write, no new gate concept, and no new authority. A reviewer who attributes ownership
and one who does not now produce visibly different output, which is the observable the instruction
never had.

## Consequences

- **[#37](https://github.com/joeloverbeck/skill-evidence/issues/37) lands strictly after
  [#34](https://github.com/joeloverbeck/skill-evidence/issues/34) and bundles into the same
  release.** Both mutate the installed Skill Evolution authorized-review reference and the close
  command's receipt construction. Shipping them separately would cost two minor bumps, two
  `skills evidence install --force` cycles, and two runs of [`../releasing.md`](../releasing.md) §6
  across three consumers for one repair.
- **The recorded-event shape gains nothing from this decision.** The owner key arrives with
  [#34](https://github.com/joeloverbeck/skill-evidence/issues/34); this decision consumes it. The
  frozen corpora under `fixtures/skill-evidence/` stay valid unedited.
- **[#36](https://github.com/joeloverbeck/skill-evidence/issues/36) is not unblocked by this, and
  never needed to be.** It was re-homed on 2026-08-11: `playbench` owns the repair, having already
  confirmed the mechanism under blind paired trials and had its candidate rejected on a material
  regression. This repository adopts the landed result. That vindicates the decision rather than
  straining it — the attribution named a real owner, and the owner's own gate opened on the owner's
  own use, exactly as designed. What the attribution could never have done is *tell this repository*
  that the work was already underway somewhere else; only a human reading four stores could. That is
  the limit this decision accepts, not a defect it leaves behind.
- **An attributed owner that the maintainer never acts on leaves no trace beyond the close.** This
  is deliberate. A durable per-attribution follow-up record would be the certification artifact
  rejected above. The completion states the attribution once, to the human, at the moment it is
  made.
- **What could reopen this.** Two `outside_target` closes exist across this repository's history.
  One (`b935f940-6b12-4a6e-9904-92308fc3c6e9`) names a sibling skill in the same repository, whose
  gate is reachable by that skill's own use. The other (`evt_009bea16-3679-4dfd-821f-59f696169620`)
  names "caller/session" in prose — not an owner kind the taxonomy admits, and one
  [#34](https://github.com/joeloverbeck/skill-evidence/issues/34) makes unsupplyable. Evidence that
  reopens this: an attribution naming an owner
  whose gate cannot re-authorize by any honest route — a retired skill, a skill in another
  repository, or one whose use has stopped — because that is the trap this decision measured and
  found absent. A count of unactioned attributions is not that evidence; it is throughput, and
  [`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
  says so.
- **The reopening clause was tested once, on 2026-08-12, and did not fire.**
  [#46](https://github.com/joeloverbeck/skill-evidence/issues/46) raised the first occurrence to
  reach it. A `triage` review in `playbench` (`rev_138058a5-36cb-4a77-bfe2-640fdd546a2c`)
  attributed an issue-body disclaimer deviation to `skill` → `.claude/skills/to-issues`, whose
  qualifying use for that same publication run was already recorded — in a top-level session that
  had closed before the attribution existed.

  The clause asks whether the named owner's gate can re-authorize by an honest route. Across the
  whole structured population it can. `external_owners` has existed only since
  [#34](https://github.com/joeloverbeck/skill-evidence/issues/34), so that population is three
  closes and eleven owner entries across the four repositories. Six of the eleven name a
  `contract` or `environment` owner — `AGENTS.md` sections, a Codex execution environment — which
  have no evidence store, no gate, and never will; no handoff could serve them, and none is owed.
  The other five name a skill, and both skills are alive: `to-issues` in `playbench` stood
  `eligible` on 18 qualifying uses, `tdd` in `what-we-bring-home` `collecting` on ten open
  incidents.

  What [#46](https://github.com/joeloverbeck/skill-evidence/issues/46) asked for instead — a route
  into the owner's store for a deviation of a run whose session has closed — is declined, on this
  decision's own ground made sharper. The owner store can accept another incident for a recorded
  run only from the session that recorded it, because a caller who names a run they were not
  observed to be in is asserting run membership, and
  [`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
  makes session identity load-bearing precisely so that no downstream authorization rests on a
  caller's assertion. The observable half of that occurrence already has a home:
  [#27](https://github.com/joeloverbeck/skill-evidence/issues/27) landed `--further-incident` on
  2026-08-10, one day after the run in question, so a deviation of a co-run sibling noticed during
  the session is recorded against that sibling today. What stays unrecordable is a reviewer's later
  attribution, which is not the owner's evidence because the reviewer did not use the owner.

  The same session boundary also refuses a cross-session *continuation* receipt, and there the
  refusal advertises a remedy that cannot work. That is a defect in the refusal rather than in this
  decision, and is [#47](https://github.com/joeloverbeck/skill-evidence/issues/47).
