# `blocked_no_valid_test` retires its evidence from the evolution gate

Status: accepted (2026-08-08, GitHub [#1](https://github.com/joeloverbeck/skill-evidence/issues/1))

Amended: 2026-08-09, GitHub [#13](https://github.com/joeloverbeck/skill-evidence/issues/13) — reporting now distinguishes one close's retirement reach from the projection's standing retired set.

Amended: 2026-08-09, GitHub [#14](https://github.com/joeloverbeck/skill-evidence/issues/14) — reviewers now vouch for the live reach bound before an instrument-limited close; a preview command was rejected because existing artifacts already supply that bound.

A Skill Evolution review that closes `blocked_no_valid_test` reached no conclusion, so it adjudicates
nothing and its trigger evidence stays open. That close still laid a watermark, and the watermark
deferred the very evidence that opened the gate — labelled `queued_pre_close_evidence`, as though a
review had accounted for it. The evidence was simultaneously open and already-accounted-for, with no
route out that anyone could take.

We made `blocked_no_valid_test` **instrument-limited**: it retires its covered evidence from the
gate without adjudicating it, and the projection says so.

## Why

The code made the adjudicating/non-adjudicating distinction in one place and dropped it in another:
retirement was filtered by `EVOLUTION_ADJUDICATING_DISPOSITIONS`, the watermark matched any
disposition at all. Only `blocked_no_valid_test` is caught by that gap in practice, because it is
the only disposition that reaches no conclusion *while the target stands still*. A
`superseded_by_target_version` review is normally superseded because the target changed, so its
`review_started` carries the old hash and the watermark's same-hash test does not match it.

That is a tendency, not a rule the derivation enforces, and it is worth stating plainly because an
earlier draft of this ADR got it wrong. `evolution_close` performs no hash comparison for that
disposition, so an ordinary preflight → claim → close sequence reaches a same-hash superseded
watermark — a target edited mid-review and then reverted is enough. Such a close also reaches no
conclusion, so its retained evidence is labelled `queued_pre_close_evidence` while nothing was
concluded about it: the same mislabel this decision fixes for `blocked_no_valid_test`.

**That residual is accepted, not overlooked.** The scope ratified for this change keeps
`superseded_by_target_version` behaviour unchanged, and the two cases are not alike enough to share
a repair: a blocked close establishes something about the instrument, while a superseded close
establishes only that the target moved, so neither retirement nor the instrument-limited label fits
it. It is also not a trap — the gate reopens on the next incident, without needing the instrument to
change. `superseded_by_target_version_does_not_retire_covered_incidents` pins the unchanged
behaviour so the decision stays visible.

The result is what [`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
forbids by name: *a disposition that leaves its triggering evidence permanently parked — neither
retired nor actionable — with no route out that the responsible party can actually take.* The only
door was a new post-review incident, and the census in
[`../../reports/conformance-evidence-census.md`](../../reports/conformance-evidence-census.md) shows
why that door leads nowhere: every conformance-only incident carrying a run condition names
accumulated volume or elapsed run length, which a fresh short-context trial cannot express whatever
it then measures. A fifth incident hits the same wall as the first four.

The same document supplies the remedy: *when a gate reaches a state with no honest exit, the correct
response is to give it an honest exit, not to wait for evidence that the situation is structurally
incapable of producing.*

## Considered options

**A caller-supplied instrument-change basis note re-enters the gate.** The shape the issue was filed
in, mirroring what Legacy Skill Decontamination did. Rejected three times over. The precedent was
removed in `7034bd4` and [ADR 0001](0001-retire-decontamination-writers-keep-readers.md) records that
decontamination's caller-asserted `--basis` was part of why that workflow could not be shared.
*Records are generated, never authored* forbids prose written into a record so the system can read it
back as truth. And the census says no honest note could be written for this population anyway, which
makes the door itself the forbidden shape — *a projection whose only escape is an event that no
honest process would produce*.

**Exclude non-adjudicating dispositions from the watermark.** The one-line symmetry fix. Rejected: it
converts the trap into a treadmill. The gate re-authorizes on the same evidence, the next review
meets the identical instrument wall and closes blocked again, and cooldown is keyed on session
identity or twelve hours, so it fires again every session.

**Report the state honestly and invent no exit.** Necessary and insufficient. It stops the projection
saying *handled* when the ledger says *inconclusive*, and supplies no route out. Adopted as part of
this decision rather than instead of it.

**Retire the covered evidence from the gate.** Chosen. It is the honest exit, and it is safe against
the treadmill precisely because the evidence that could re-fire is then genuinely different evidence.

## Consequences

- Evidence covered by a `blocked_no_valid_test` close stays in `open_incident_ids` — nothing was
  adjudicated — but leaves `candidate_clusters`, so it cannot reach a threshold again. Re-firing that
  cluster requires enough genuinely new incidents to meet the threshold on their own.
- **Retirement reaches the covered clusters, not just the listed IDs, and stops at the close.** The
  trigger list is frozen when the threshold fires, so an incident arriving while the review runs is
  never in it, and a `material_recurrence` list never names the cluster's merely-frictional siblings
  at all. Those share the symptom whose binding constraint the instrument could not vary. Retiring
  only the listed IDs would hand the next review a lower bar than the first one faced — two new
  incidents where the threshold takes three, and permanently so in the friction-sibling case, which
  is the treadmill wearing a different hat. The bound matters as much as the reach: evidence recorded
  *after* the close is new evidence and drives the gate, or the symptom would be silenced forever,
  which is a worse failure than the trap. Retirement also applies only to closes whose review ran
  against the current target hash — a finding about what this instrument cannot test says nothing
  about a target that has since changed.
- **The close receipt and the projection name retirement at their distinct scopes.** A
  `blocked_no_valid_test` receipt carries `retired_from_gate_event_ids`, the retirement reach of that
  close, even when the list is empty; every non-instrument-limited close omits the key. The projection
  keeps `instrument_limited_incident_ids` as the standing per-hash retired set, which can include
  earlier closes and can shrink after later adjudication. `skills evolution-status` reports targets
  carrying that standing set under **Retired as untestable** rather than folding them into the omitted
  count next to skills that never recorded an incident. Retiring evidence silently at either surface
  would trade one dishonest projection for another.
- Evidence a blocked close did not cover, still deferred behind its watermark, now reports
  `queued_behind_instrument_limited_review` instead of `queued_pre_close_evidence`.
- **A contemporaneous severe incident is never retired this way.** It authorizes from
  `open_incident` alone, ahead of any watermark, so it was never trapped; listing one as
  instrument-limited would have the projection claim it stopped driving the gate while it demonstrably
  still does. The consequence is that a severe incident whose review closes blocked keeps
  re-authorizing. That treadmill is pre-existing and deliberately left alone — trading it for a lost
  safety claim is the worse bargain.
- **The carve-out keys on that property, not on severity.** A *retrospective* severe incident is
  skipped before the severe trigger, so it authorizes nothing and merely counts toward a cluster.
  Carving it out would protect no safety claim and would leave it discounting every later review of
  that symptom — two new incidents where the threshold takes three, the same lower bar this decision
  closes elsewhere. It is therefore retired like any other covered evidence.
- **No recorded-event shape changed.** `adjudicated_event_ids` was already written for every
  disposition and already required by `event.v1`, so the coverage list this decision reads is already
  on disk in every consumer's history. This is a read-side change, and the frozen corpora prove it:
  every fixture under `fixtures/skill-evidence/` is byte-identical to before this change — event
  streams, expected projection, and the status-reporter goldens alike. The census summary omits its
  `retired as untestable` count when zero specifically so that stays true.
- **The installed-asset surface moves too, and a partial upgrade is the hazard.**
  `consumer-contract.md` names three surfaces and warns that conflating them is the mistake it
  exists to prevent, so this one gets its own bullet. `gate-status.v1.schema.json` gains
  `queued_behind_instrument_limited_review` and `instrument_limited_incident_ids`, and both
  `skill-evolution/references/authorized-review.md` and `skill-evolution-status/SKILL.md` gain
  prose describing the new contract. A consumer that runs `cargo update` **without**
  `skills evidence install --force` hits this on the common path, not just the rare one: both
  schemas set `additionalProperties: false`, so a stale installed `gate-status.v1` rejects any
  projection carrying `instrument_limited_incident_ids` at all, and the new enum value compounds it.
  That consumer also reads skill text still saying `blocked_no_valid_test` retires nothing. The
  release must name both, per [`../releasing.md`](../releasing.md) §5 and §6. Nothing is retired or
  renamed, so no consumer has to delete a directory by hand.
- **The Rust API break is real even though the evidence surface is untouched.** `GateStatus` is a
  public struct without `#[non_exhaustive]`, so gaining `instrument_limited_incident_ids` breaks any
  downstream struct-literal construction. Per [`../releasing.md`](../releasing.md) §1 that is a minor
  bump while `0.x`: **the release carrying this must be `0.2.0`**, not a patch. The version is not
  moved here, because this repository cuts releases as their own commits.
- The cost, accepted: a conformance problem this instrument cannot test goes quiet after one review
  instead of reappearing. It is visible in the projection and in the census, but nothing will prompt
  again. GitHub [#2](https://github.com/joeloverbeck/skill-evidence/issues/2) is where the question of
  an instrument that *could* test it belongs; this decision does not answer it and does not depend on
  it.
- Retiring a real cluster is now a consequence of choosing this disposition. The Skill Evolution
  reference says so at the point of choice, because a reviewer reaching for `blocked_no_valid_test`
  to quiet an untested cluster now costs that cluster its future.
- **Before an instrument-limited close, the reviewer vouches for its reach bound.** The claim receipt
  supplies the frozen coverage list; the live gate projection supplies every current candidate
  cluster for the symptoms that list touches, including incidents recorded after the claim. The
  reviewer confirms that the named binding constraint plausibly covers every open incident in that
  bound. The bound deliberately errs upward because a contemporaneous severe incident remains in
  its cluster while never retiring; if the reviewer cannot vouch for a sibling, the symptom-keyed
  close still happens and the mismatch is disclosed in the report and completion. A pre-append
  preview command was considered and rejected: these existing artifacts already bound the reach,
  while a new flag beside the irreversible close would change a command surface consumers invoke by
  name and hypothetical derivation would add machinery without improving the decision.
