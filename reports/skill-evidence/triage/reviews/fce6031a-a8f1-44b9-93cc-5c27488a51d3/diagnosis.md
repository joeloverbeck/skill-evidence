# Authorized diagnosis

## Threshold premises

- All three trigger events are qualifying uses of target hash
  `5bc32c8a31d750428f7a914248487ce1c884f6b87f112a2d3fff1c087cd31596`.
- They came from three distinct top-level sessions and three distinct issue-triage tasks (#13,
  #14, and #15), not retries or continuations.
- All three were recorded contemporaneously on 2026-08-09; the threshold-crossing event was the
  #15 use.
- `execution` is a factually plausible common symptom: each run had to resolve an instruction
  conflict while executing the documented triage workflow.

## Candidate mechanisms and ownership

1. **Target defect — verification is sequenced after a recommendation that depends on it.**
   `SKILL.md` step 2 requires the recommendation to say whether the request is already
   implemented and then wait for direction. Step 3 performs the verification needed to establish
   that premise. The #14 and #15 uses both moved verification ahead of recommendation to avoid an
   unsound recommendation. The target owns the contradictory ordering.
2. **Target compliance defect — the durability rule erases substantive artifact identity.**
   `AGENT-BRIEF.md` categorically prohibits file paths without distinguishing an incidental
   implementation location from a document, package reference, schema, or template that is
   itself part of the requested contract. The #13 use replaced exact artifact identities with
   descriptions to comply. The target owns the over-broad prohibition and its salience.

Both mechanisms proceed to validation. No outside owner is implicated, and neither mechanism is
task-specific: the first applies whenever a recommendation turns on a claim that inspection does
not settle; the second applies whenever the work's subject is a durable repository artifact.
