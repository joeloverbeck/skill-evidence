# `blocked_no_valid_test` retires its evidence from the evolution gate

Status: accepted (2026-08-08, GitHub [#1](https://github.com/joeloverbeck/skill-evidence/issues/1))

Amended: 2026-08-09, GitHub [#13](https://github.com/joeloverbeck/skill-evidence/issues/13) — reporting now distinguishes one close's retirement reach from the projection's standing retired set.

Amended: 2026-08-09, GitHub [#14](https://github.com/joeloverbeck/skill-evidence/issues/14) — reviewers now vouch for the live reach bound before an instrument-limited close; a preview command was rejected because existing artifacts already supply that bound.

Amended: 2026-08-09, GitHub [#16](https://github.com/joeloverbeck/skill-evidence/issues/16) — retirement now reaches only what the close's own authorization reason could name; the friction-sibling justification below was measured wrong for `material_recurrence` and is corrected. A correction path for an erroneous vouch is recorded as declined rather than left open.

Amended: 2026-08-10, GitHub [#23](https://github.com/joeloverbeck/skill-evidence/issues/23) — an adjudicating close can now name part of its coverage list as untestable and retire it on this decision's warrant without adjudicating it. Reach is unmoved and the treadmill reasoning below re-runs unchanged; see the consequence on untestable coverage.

Amended: 2026-08-11, GitHub [#32](https://github.com/joeloverbeck/skill-evidence/issues/32) and [#35](https://github.com/joeloverbeck/skill-evidence/issues/35) — the coverage list now freezes when the review is claimed rather than when the threshold first fires, because the reach consequence below described a concurrency that never occurred. The reopening clause fired, was tested against its first real occurrence, and correction/supersession is declined a second time on a condition that bites.

Amended: 2026-08-11, GitHub [#33](https://github.com/joeloverbeck/skill-evidence/issues/33) — the re-anchored coverage list reaches the reviewer with full payloads, so the vouch's missing inputs narrow to the claim-to-close span. That residue is accepted under disclosure and carries no open tracker; the reopening condition is stated where the residue is.

Amended: 2026-08-12, GitHub [#45](https://github.com/joeloverbeck/skill-evidence/issues/45) — the provenance vouch now carries checked, generated whole-field citations and refuses when any instrument-limited event lacks one; the semantic reach-bound vouch remains a disclosed judgment rather than a compiled refusal.

Amended: 2026-08-12, GitHub [#48](https://github.com/joeloverbeck/skill-evidence/issues/48) — the *Why* below rested on a fresh short-context trial being unable to express accumulated volume or run length. [ADR 0008](0008-a-long-course-trial-expresses-an-accumulated-context-constraint.md) found that premise conflated a *fresh* executor with a *short* one, so the door-leads-nowhere argument now rests on the residue that survives a long-course trial. The retirement decision and its reach are unchanged.

Amended: 2026-08-13, GitHub [#55](https://github.com/joeloverbeck/skill-evidence/issues/55) — the live gate projection's trigger list now names what a review would cover if claimed at that derivation, rather than retaining the narrower threshold-time list until an evolution command happens to rewrite it. Claim-time freezing is unchanged.

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
it then measures. A fifth incident hits the same wall as the first four. **Half of that wall has
since come down.** [ADR 0008](0008-a-long-course-trial-expresses-an-accumulated-context-constraint.md)
reaches accumulated volume and run length with a long-course reproduction trial, so a constraint of
that kind no longer settles the question by being named — what survives on reachability grounds is
cross-session accumulation and elapsed wall-clock. The exit itself is unmoved and stays wider than
that residue: a constraint with no available witness, or an incidence too low to separate arms,
still arrives here. That narrows one of several routes in; it changes nothing about what the exit
then does, which is the decision this ADR records.

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
- **Retirement reaches what the close's own authorization reason could name, and stops at the close.**
  The trigger list is frozen when the threshold fires, so an incident arriving while the review runs
  is never in it even though the reviewer can see it in the live projection and vouches for it.
  Retiring only the listed IDs would hand the next review a lower bar than the first one faced — two
  new incidents where the threshold takes three — which is the treadmill wearing a different hat. So
  the reach is the authorization reason re-evaluated at the moment of the close: `friction_recurrence`
  and `ten_use_unresolved` name their whole cluster and so retire it, `material_recurrence` names
  only its material-or-worse subset and retires only that, and `severe` names one incident that is
  itself never retired, so it retires nothing.
  **The original text reached the whole symptom under every rule, and the argument for that was
  wrong in two places.** It said a `material_recurrence` list "never names the cluster's merely
  frictional siblings at all," which is true, and inferred that those siblings must therefore be
  retired — but a friction sibling cannot lower a `material_recurrence` bar, which counts only
  material-or-worse incidents, so it takes nothing from the next review of that rule. It also said
  the lower bar was "permanently so in the friction-sibling case"; it is not, because a later
  `friction_recurrence` close names the whole cluster and sweeps those siblings, so the cost was one
  extra review session rather than a permanent discount. Widening past that reason bought
  that session at the price of retiring evidence the review never examined, keyed on a symptom the
  glossary defines as never diagnostic.
  The bound matters as much as the reach: evidence recorded *after* the close is new evidence and
  drives the gate, or the symptom would be silenced forever, which is a worse failure than the trap.
  Retirement also applies only to closes whose review ran against the current target hash — a finding
  about what this instrument cannot test says nothing about a target that has since changed.
- **The narrowing was measured before it was adopted, and it moves no gate.** Across the six live
  stores in this repository, `playbench`, and `mundifold`, 27 incidents stood retired: 22 named in a
  coverage list, and 5 reached only by the symptom-wide rule. The new reach frees 2 of those 5, all
  in one store, and every affected cluster stays below its threshold — no consumer's gate state
  changes on upgrade. What changes is future: a freed incident counts toward the next threshold, so
  a symptom can reopen on less new evidence than before. The remaining 3 are late arrivals the
  reviewer saw and vouched for, and they stay retired.
- **The derivation reads `authorizing_rule` off the `review_started` event, which `event.v1` does
  not require.** Every `review_started` this crate has ever written carries it, and all 91 across
  the three stores do. It is still an unvalidated field, so a stream lacking it — or carrying a rule
  this version does not recognize — falls back to the previous symptom-wide reach rather than
  failing or narrowing on a guess. Requiring the field in the published schema is a separate
  contract decision and is not taken here.
- **The close receipt and the projection name retirement at their distinct scopes.** A
  `blocked_no_valid_test` receipt carries `retired_from_gate_event_ids`, the retirement reach of that
  close, even when the list is empty; so does an adjudicating close that named untestable coverage,
  reporting the names the derivation actually retired. Every other close omits the key. The projection
  keeps `instrument_limited_incident_ids` as the standing per-hash retired set, which can include
  earlier closes and can shrink after later adjudication. `skills evolution-status` reports targets
  carrying that standing set under **Retired as untestable** rather than folding them into the omitted
  count next to skills that never recorded an incident. Retiring evidence silently at either surface
  would trade one dishonest projection for another.
- Evidence a blocked close did not cover, still deferred behind its watermark, now reports
  `queued_behind_instrument_limited_review` instead of `queued_pre_close_evidence`.
- **Untestable coverage carries this warrant into an adjudicating close, one event at a time.**
  A whole-review disposition cannot hold a review whose mechanisms reached different readings, and a
  multi-incident authorization routinely produces one: `friction_recurrence` needs three independent
  incidents, which routinely means three distinct mechanisms, and the trial set now runs one
  reproduction per mechanism. An adjudicating close therefore names the coverage this instrument
  could not decide — no trial could express the mechanism, or the acceptance gate grades outcome and
  the evidence bears no outcome claim ([ADR 0003](0003-no-new-instrument-for-conformance-only-evidence.md)) —
  and those retire as instrument-limited rather than adjudicated — open in the ledger, out of the
  clusters. Naming an event asserts only that this review could not decide it; it never asserts that
  the mechanism failed to reproduce, and the per-mechanism readings stay in the review report.
  **The severe carve-out below reaches this channel too, and costs more here.** A named
  contemporaneous severe incident stops being adjudicated and is still not retired, so it keeps
  authorizing every session — where before this change an adjudicating close would have retired it
  and ended the loop. That is the bargain the carve-out already struck for a blocked close, struck
  again for the same reason: a projection claiming a severe incident stopped driving the gate while it
  demonstrably still does is the worse failure. The reference states the cost at the point of choice.
  Without the channel the projection asserts a conclusion the frozen plan had already
  pre-registered as unreachable, which is the mislabel this decision exists to prevent, reached from
  the other direction.
  **The reach argument above re-runs and lands unchanged, because reach does not move.** The named
  events all sit inside a coverage list this close already accounted for, so nothing outside
  that list changes character and no reason-scoped widening applies to them: a close that examined its
  coverage mechanism by mechanism has already said which mechanisms it could not decide, and inferring
  more would retire evidence the review never examined — what #16 narrowed away from. Nor is this the
  treadmill: an untestable trigger left merely *uncovered* would stay open **and** clustering, meeting
  the same instrument wall on the next review, which is the shape the considered options above reject.
  Retiring it is the same honest exit, at trigger granularity.
  **Naming the whole coverage list is allowed.** Reading a close that named everything as a review
  that concluded nothing would be wrong: a review can decide none of what it covered while having
  concluded a great deal, because the acceptance gate grades outcome and cannot decide a covered
  trigger whose evidence bears no outcome claim, however thoroughly the trials tested its mechanism
  ([ADR 0003](0003-no-new-instrument-for-conformance-only-evidence.md)). Refusing it would push
  exactly that review onto `blocked_no_valid_test`, whose reach is the authorization reason's whole
  cluster — strictly wider than the close covered, retiring evidence the review never examined, while
  asserting an untestability its own trials disprove. Which limit a review met is semantic and the
  command cannot see it, so the reference decides it at step 9 and the command polices only what it
  can check: coverage the close does not hold. The close receipt reports the named retirement in
  `retired_from_gate_event_ids`, so this channel is no quieter than the disposition-level one.
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
  **The #23 amendment does move it**, adding optional `instrument_limited_event_ids` to
  `review_disposition`. Absent means what every close already asserted — everything covered was
  concluded — so no history is reinterpreted, the canonical writer omits the key entirely when unused,
  and the frozen corpora stay byte-identical. That payload sets no `additionalProperties: false`, and
  the reader checks only the keys it knows, so a consumer still on a stale installed `event.v1` keeps
  validating streams that carry it. `releasing.md` §2 governs it regardless.
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
  The #23 amendment breaks it again the same way: `EvolutionCloseRequest` gains `instrument_limited`
  and carries no `#[non_exhaustive]` either, and `skills evolution close` gains `--instrument-limited`,
  which is the mounted command surface `consumer-contract.md` names as contract however little
  `cargo build` notices. **The release carrying that amendment is a minor bump from whatever precedes
  it**, and must require `skills evidence install --force`, because the installed review text and the
  installed `event.v1` both move with it. The version is again not moved here.
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
  **The list now freezes at the claim, and that corrects a factual claim made above.** The reach
  consequence justified widening past the coverage list by saying the trigger list is frozen when the
  threshold fires, "so an incident arriving *while the review runs* is never in it." Both occurrences
  measured since say otherwise. In `joeloverbeck/playbench` the authorizing trigger sat at stream
  index 77 and `review_started` at index 125; all 14 retired `output` incidents were recorded before
  the review started and none during it. In this repository's `code-review` store the trigger sat at
  index 1 and `review_started` at index 22, the same shape. The gap was never concurrency. The
  derivation sets its fired trigger at the first event satisfying a threshold and never re-anchors, so
  the list froze at first eligibility and then aged for as long as the gate went unclaimed — 48 events
  in the first case. The vouch was asked to span thirteen incidents that entered the cluster before
  the reviewer had standing to look at them, which is not a judgment a reviewer can make and not the
  judgment this consequence described. Freezing at the claim leaves reach reason-scoped and
  re-evaluated at the close, so the treadmill argument re-runs unchanged and the next review's bar
  does not move; what shrinks is the span the vouch must cross, to what genuinely arrives between
  claim and close — for both measured occurrences, nothing.
  **The provenance vouch now refuses; the reach-bound vouch still does not.** Every event whose
  retirement rests on an untestability claim must carry at least one checked citation to a complete
  `run_condition`, `observed`, `consequence`, or `workaround_taken` field on that covered immutable
  event. The operator supplies only the constraint label and pointer; the command copies the field,
  records it, echoes it in the close receipt, and refuses write-free when the pointer or required
  coverage is missing or malformed. The two grounds on which this decision declined refusal no
  longer reach that structural provenance check: its inputs are supplied, and the operator authors
  no recorded fact. The command still cannot decide whether the copied field entails the constraint,
  or whether a claim-to-close arrival belongs inside the live reach bound. Those remain semantic
  judgments, and disclosure stays the response for that residue.
  **The vouch's inputs are supplied for everything the claim freezes, and that was the point.**
  The packet's `trigger_events` is built from the projection's trigger identity list, so re-anchoring
  that list to the reason-scoped cluster hands the reviewer every frozen coverage member's full raw
  event with no packet change at all. What is left without payloads is only what arrives between the
  claim and the close — the same span the freeze already narrowed, empty in both measured occurrences
  — and step 9 already routes an unvouchable member to a disclosed mismatch rather than a judgment.
  [#33](https://github.com/joeloverbeck/skill-evidence/issues/33) was filed against the withheld
  payloads and is closed as delivered by this amendment; the residue it named is accepted here under
  disclosure, with no open tracker. What would reopen it is a close whose reach turned on a
  claim-to-close arrival the reviewer could not read — not the span merely being non-empty.
  `trigger_event_ids` on `review_started` changes meaning here, so the release carrying this moves an
  installed and a compiled surface together and requires `skills evidence install --force` per
  [`../releasing.md`](../releasing.md). The `Coverage list` and `Retirement reach` entries in
  `CONTEXT.md` both name the old freeze point in so many words and move with the implementation, not
  ahead of it. `Reach bound` does not name it and stays accurate as written, but it is read against
  the coverage list and the two converge once the freeze moves, so it is reviewed rather than assumed
  unchanged.
  **The live projection now publishes that same reason-scoped coverage before the claim.**
  `trigger_event_ids` means the open events the current authorization reason says a review would
  cover if claimed as of the derivation. It does not preserve the identities that happened to be in
  the cluster when the threshold first fired: `authorization_reason` and `threshold_session_id`
  already preserve why and when authorization fired, while the projection's role in the reach-bound
  vouch is to show what the review would cover now. The former split made identical streams alternate
  between two published values depending on whether `skills evidence derive` or an evolution
  preflight wrote last. Re-anchoring now belongs to the derivation itself, so derive, preflight, claim,
  and the evidence packet share one value. Claim still freezes that list into `review_started`;
  `severe` still keeps its one triggering incident; retrospective incidents remain outside a
  `ten_use_unresolved` list; and an absent named cluster or an unresolvable trigger symptom leaves the
  threshold derivation intact rather than replacing it with an empty list.

  This changes regenerable projection output, not recorded history. No event shape or frozen corpus
  moves. The published `gate-status.v1` description and its installed copy move with the compiled
  behavior, so the release is a minor bump while `0.x` and consumers must preview the differing schema
  with `skills evidence install --root .` before deliberately repeating with `--force`. What would
  reopen this decision is a real consumer need for exact fire-time event identity that
  `authorization_reason` and `threshold_session_id` cannot answer. That would warrant a distinct
  published field rather than overloading `trigger_event_ids` with two meanings again.
- **A wrong vouch stays wrong: there is no correction, supersession, or unretirement event, and
  none is being built.** The reach is mechanical, but the judgment that authorizes it — that this
  instrument cannot vary the named binding constraint — is semantic, and nothing in the stream can
  check it. `joeloverbeck/skill-evidence#16` recorded a real instance: one covered incident's
  expected and observed facts were a static repository condition a fresh fixture could vary, while
  the close asserted a single untestable long-run constraint over every incident it reached.
  Three grounds for declining the repair, in the order they bind. An event saying *the earlier
  vouch was wrong* is prose written into a record so the system can read it back as truth, which is
  the shape the considered options above reject three times over and which
  [`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
  forbids as *records are generated, never authored*. With the reach narrowed to the authorizing
  rule, a wrong vouch can no longer take evidence the review never had reason to examine — the rule
  that opened the gate is the rule that bounds the loss. And an honest exit already exists and is
  the one this decision has relied on since it was accepted: evidence recorded after the close
  drives the gate, and editing the target clears the hash. The residue is that a testable incident
  already on disk can stay out of the gate on the strength of a judgment that was wrong, and that
  is accepted here rather than overlooked.
  **That clause fired, and the answer is the same.**
  [#35](https://github.com/joeloverbeck/skill-evidence/issues/35) asked for a bounded,
  source-qualified, revocation-only release. It cited `rev_278c4810-c838-415f-bcda-54de8d6457e9`, an
  `outside_target` close in this repository, which never runs the vouch step this clause is about —
  that step is `blocked_no_valid_test`-only. The occurrence that does satisfy the clause is
  `joeloverbeck/playbench`'s `rev_d26b8ab8-80d3-4c57-a134-da70fd7adf89`, a `blocked_no_valid_test`
  close that vouched for one incident, disclosed that it could not vouch for the other thirteen, and
  retired all fourteen. Declined a second time, on three grounds. The claim-time freeze above removes
  the mechanism that produced the disputed reach, so the machinery would be built for a defect this
  amendment stops causing. The honest exit was measured rather than assumed: `playbench`'s `tdd` store
  keeps nine non-retired open incidents still clustering, one of them a `material_recurrence` away
  from re-firing, and this repository's `code-review` store reopens on a single incident recorded
  after its disposition — retired evidence stays in `open_incident_ids` and leaves `candidate_clusters`
  only, so the ledger never lost it. And the transition's cost is the highest this repository can pay:
  an irreversible reader-version floor across three consumers holding append-only history no release
  can regenerate, bought to release two residues that both still have a working door.
  **The old reopening condition was too easy to satisfy and is replaced.** "A close whose reach is
  disputed *after* the vouch step has shipped and been used" was met within two days of that step
  shipping, by a review that disclosed the mismatch exactly as instructed — it fired on the procedure
  working, not on it failing. What reopens this now is a residue with no door: a retired or
  adjudicated incident whose gate provably cannot reopen on new evidence, rather than one that merely
  waits for it. That is what the honest exit is standing on, so that is what should reverse this.
