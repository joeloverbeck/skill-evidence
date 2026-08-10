# Diagnosis — triage review 1a00f33c-19d3-4e28-ba1e-d17d8e61f205

Authorizing rule: `material_recurrence:output`.
Triggers: `evt_22e0abe5-3665-4d4c-a716-1136122304be`, `evt_41dfe1f4-5795-4ede-95b2-1aa8a6bd3b4c`.
Baseline target hash: `5bc32c8a31d750428f7a914248487ce1c884f6b87f112a2d3fff1c087cd31596`.

## Threshold premises (step 2)

| Premise | Result |
| --- | --- |
| Both triggers are qualifying uses | confirmed — `qualifying_use: true` on both |
| Genuinely independent | confirmed — distinct top-level sessions `53c2aa05-…` / `ceb33341-…`, distinct task fingerprints `00b3a287d5f102aa` / `27cfba1e0f0df3a2`, distinct run groups, materially different tasks (one issue end to end vs. four issues) |
| At least one contemporaneous, incl. threshold-crossing | confirmed — both `retrospective: false`; the threshold-crossing `evt_41dfe1f4` is contemporaneous |
| Trigger hashes match the live target | confirmed — both `5bc32c8a…`, equal to the preflight's live hash |
| Cluster plausible as a common symptom | confirmed — `output`: both are defects in a published triage artifact found after publication |

## Predecessor constraint (same target hash)

Review `fce6031a-a8f1-44b9-93cc-5c27488a51d3` judged these exact bytes. Two of its findings bind
this review:

- It confirmed a **recommendation-before-verification ordering** defect and built a candidate that
  both blind evaluators materially preferred on the ordering trials. The candidate was rejected only
  because the frozen gate demanded material improvement on *both* frozen reproductions and the
  artifact-identity pair tied.
- Its **artifact-identity reproduction (t02)** ran the unchanged current arm fresh and short-context
  on a task whose subject was exact artifact identity, and the current arm **passed**: it produced a
  complete, durable brief without prescribing current checkout paths. That is a same-hash record
  that the durability wording is followed when freshly read.

## Trigger → mechanism → ownership → evidence class

### M1 — unchecked acceptance-criteria satisfiability (`evt_22e0abe5`)

The brief contract grades acceptance criteria one at a time — "Each criterion should be
independently verifiable" — and requires nothing that would catch a criteria *set* which no
implementing agent can satisfy. `SKILL.md` step 1 directs codebase exploration through the domain
glossary and ADRs; neither it nor `AGENT-BRIEF.md` directs the author to check the behavior the
brief is about to prescribe against behavior the repository already pins, and step 5 posts the brief
with a bare link to the contract. The recorded failure is exactly that shape: the brief prescribed
behavior contradicting a passing test while separately forbidding an edit to an existing contract
test, so the two criteria could not both be satisfied.

- **Ownership:** target defect — missing guidance, causally connected. Proceeds.
- **Evidence class:** outcome-graded. The `consequence` records that a self-contradictory spec was
  published to a public tracker labelled `ready-for-agent` and required a correcting amendment
  comment plus a prepended pointer. The delivered artifact was worse; the clause that no downstream
  work was misled bounds the blast radius beyond the artifact, not the artifact itself.

### M2 — durability drift discovered only after publication (`evt_41dfe1f4`)

This trigger's `expected` is squarely the durability contract: a brief states durable interfaces,
does not cite file paths, and is the contract a later session works from. Both recorded deviations
are failures of that one property — four published briefs cited `tests/assets_contract.rs` and
`src/lib.rs`, and the brief on the issue that cannot be decided alone buried its binding coupling
mid-comment under a heading announcing something else. Every deviation surfaced after all triage
writes were complete.

- **Ownership:** candidate target compliance defect — the rule exists and is stated plainly; its
  placement in a reference doc read once at the head of a long run is the candidate explanation for
  repeated non-compliance. Proceeds as a class, but see the frozen plan: its binding constraint is
  not expressible by this instrument.
- **Evidence class:** mixed. Outcome-graded for the coupling and the title — two published artifacts
  required correction after delivery (comment rewritten via API PATCH, title replaced).
  Conformance-only for the path citations, whose effect the `consequence` records as undetermined:
  "the check that would reveal it is an agent working the brief after those paths move, and it has
  not run."

## Recorded-workaround finding (raw trigger events only)

- `evt_22e0abe5` — "Posted an amendment comment resolving the conflict and prepended a pointer to it
  on the original brief, rather than rewriting the published brief." This is remedial, applied after
  publication; it repaired an instance and suppressed nothing. By the standing reading, a workaround
  taken without suppressing the mechanism is evidence **against** target ownership for M1.
- `evt_41dfe1f4` — "Before writing briefs, re-verified the reporter's central factual claim against
  the cited frozen plan and review report, which overturned it; the skill's step order places that
  verification after the recommendation has already been delivered." This one *is* a suppression: an
  operator inserted a verification the target does not require at that point, which is evidence
  **for** target ownership. Note precisely what it suppressed — the recommendation-before-
  verification ordering, the mechanism the same-hash predecessor already confirmed — not the
  durability drift M2 names. The two artifact deviations M2 names were not suppressed; they were
  corrected after delivery.

Direction, recorded as evidence and not as a verdict: the only genuine suppression on record points
at a mechanism outside this review's coverage, and both mechanisms this review does cover carry
purely remedial workarounds. That is directional evidence against target ownership for M1 and M2,
and it is why the frozen plan below leans on the reproduction trial rather than on the workaround
record.

## Non-trigger open incidents

Open incident IDs outside the trigger set for this authorization: **1**. Not characterized, not read.
