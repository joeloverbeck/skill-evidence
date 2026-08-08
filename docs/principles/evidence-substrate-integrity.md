# Evidence Substrate Integrity

Status: accepted constitutional principle

This repository's entire domain is control records: what a skill use actually did, what a gate
derives from an accumulation of those records, and what a review is thereby authorized to do.
Control records are recovery and authorization infrastructure. The moment they become a second
source of truth, a body of paperwork, or a machine for justifying their own machinery, the
substrate has failed regardless of whether every test passes.

## Records are append-only and never rewritten

An event, once appended, is history. Derivation reads it and writes a projection alongside it;
derivation never edits, reorders, compacts, or removes an event. The frozen-corpus test asserts
exactly this — that replaying a stream through the current reader leaves the stream
byte-identical.

This is not a performance concession or a storage convention. It is what makes a recorded
receipt worth anything: an event that can be revised after the fact records an opinion, not an
observation.

The obligation outlives the workflow that created it. The retired Legacy Skill Decontamination
workflow will never write another event, and its command surface is gone for good. Its two event
types, their gate derivation, and their place in the published schema stay forever, because
completed runs sit in a consumer's history and that history is immutable. **A retired writer does
not retire its readers.**

## Structural validity is not semantic acceptance

A schema-valid event is well-formed. It is not true, not sufficient, and not a judgment.

- A derived gate projection is a mechanical function of recorded evidence. It reports what the
  evidence supports. It does not decide that a skill is bad, and its opening is not a finding.
- A completed review is not an accepted change. Landing is a separate, authorized, receipted act.
- Passing validation is not acceptance. A candidate that survives its trials has earned a
  proposal, and the human accepts or does not.
- Refusal is a first-class outcome, not a failure. The command surface refuses (exit `3`) when
  authority is absent, and a refusal is the system working.

## Evidence authorizes only the claim it bears

This is the rule that decides the hardest cases, so it is stated at length.

A recorded incident bears the claim its own fields establish, and no more. An incident whose
`consequence` field records that no defect reached the delivered work bears a claim about
**conformance** — the run did not do what the skill said. It does not bear a claim about
**outcome** — that the delivered work was worse than it would otherwise have been.

Both claims are real. Conformance evidence is genuine evidence, and a skill whose instructions
are routinely skipped has a real problem. But the two claims are not interchangeable, and the
substrate must not let one be spent as though it were the other. Specifically:

- A gate whose acceptance test is an **outcome** improvement may not be opened solely by
  **conformance** evidence and then treated as having reached a verdict when no outcome deficit
  can be demonstrated. That is not a finding about the skill; it is a mismatch between what was
  recorded and what was asked of it.
- Symmetrically, a conformance defect is not disproved by the absence of an outcome deficit. "No
  harm was demonstrated" answers a question that was never the claim.
- Where a severity ladder grades a deviation as material on structural grounds alone, the grade
  records the deviation's kind, not its harm. **A severity ladder is not proof of harm.**

Where conformance and outcome are conflated, the defect is in the pairing of evidence to
acceptance test — not in the grading half and not in the validation half. Repairing only one half
moves the mismatch rather than resolving it.

## The substrate may not manufacture evidence for its own necessity

A gate that can be satisfied only by evidence the gate's own structure prevents from existing is
not a gate; it is a trap that will keep producing work.

Concrete forms this takes here, all forbidden:

- A disposition that leaves its triggering evidence permanently parked — neither retired nor
  actionable — with no route out that the responsible party can actually take.
- A projection whose only escape is an event that no honest process would produce.
- Counting reviews run, incidents captured, or gates closed as evidence that the lifecycle is
  working. Those are throughput. The lifecycle is working when a consumer's skills got better.
- Treating an issue in this repository's own tracker as an obligation to change this repository.

When a gate reaches a state with no honest exit, the correct response is to give it an honest
exit, not to wait for evidence that the situation is structurally incapable of producing.

## Records are generated, never authored

A control record is produced mechanically from what actually happened. Nobody fills in a form.

- A capture records a factual receipt about a completed use — including a clean one — without
  diagnosing, grading intent, or changing the thing it observed.
- Provenance, hashes, session identity, and timestamps are computed, not asserted by the caller.
- Where the caller must assert something the system cannot compute, that assertion is recorded
  verbatim as an assertion, attributed, and never silently promoted to a derived fact.

If a proposed feature requires an operator to write prose into a record so the system can later
read it back as truth, the feature is paperwork and the answer is no.

## Side effects on a consumer's files

The lifecycle modifies real skill files in a consumer's repository. Every such act carries, and
must keep carrying:

- **Narrow authority** — landing happens only for a claimed review that reached the one outcome
  that authorizes it;
- **An expected-state check** — a baseline snapshot taken before, compared after, so the act
  knows what it actually changed;
- **Idempotency** — a repeated or interrupted act does not double-apply;
- **An inspectable receipt** — a durable record of what changed, sufficient to review or reverse
  without reconstructing a transcript.

A refusal that leaves no event and no report is the correct behavior for an unauthorized act. An
act that half-applies and records nothing is the worst outcome available and must not be
reachable.

## Session and target identity are load-bearing

Independence claims rest on distinct top-level sessions, and target identity rests on a content
hash of the skill under review. Both are computed here, and both are the reason a gate's evidence
can be trusted at all.

Weakening either — accepting a caller-supplied session id as proof of independence, or matching a
target by path when its content has changed — does not merely lose precision. It makes every
downstream authorization claim unfounded, silently and retroactively.
