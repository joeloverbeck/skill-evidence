# Frozen Validation Plan: triage

Review ID: `5833275e-4998-450e-98dd-49a0bd8939a6`

Frozen before any candidate exists. The current target hash is
`9968ebd3d4508f01bb91ddac011c09330651bcea7c9a5fdcd9dcad593cb4f4bc`.

## Evidence and mechanism routing

### M1: paired-workflow atomicity

- Trigger events: `evt_35a84c74-5fa9-4319-aaa1-772400136d1d` and
  `evt_ebe48487-5c26-4fd4-af46-20a06ede8ee8`.
- Candidate mechanism: the instruction to run `/grilling` and `/domain-modeling` together is
  present, but its placement and lack of an execution receipt may let a long triage run announce
  both and invoke only `/grilling` while later reporting a domain-doc outcome.
- Ownership class before trial: candidate target-compliance defect; not yet confirmed.
- Evidence class: conformance-only for both events. Their recorded consequences are undetermined
  and do not claim a delivered-work outcome deficit.
- Binding constraint: the omission emerged near the midpoint of a long, multi-issue session under
  accumulated context and competing work, and no later recap caught it.
- Trial-instrument decision: unable to be expressed. A fresh, short-context executor necessarily
  encounters the handoff near the start of its own run and cannot vary the recorded context
  distance or accumulated load.
- Witness: a completed long-run execution trace would have to show whether both skills were
  separately loaded and invoked before any domain-doc outcome was reported.
- Unexpressed reading: a fresh isolated run has no preceding long-session context, regardless of
  whether it invokes one or both skills.
- Recorded-workaround direction: `evt_35a84c74-5fa9-4319-aaa1-772400136d1d` records a manual ADR
  conformance check by analogy; it did not suppress the missed invocation, which weighs against
  target ownership. `evt_ebe48487-5c26-4fd4-af46-20a06ede8ee8` records no workaround.
- Frozen trial slot: no run and no candidate. Record the dead end and route both triggers as
  reproduction-instrument-limited at close unless a prior step reaches another conclusion.

### M2: named-scope redundancy

- Trigger event: `evt_a5230399-e37c-444a-b050-d959e5944357`.
- Candidate mechanism: step 1 says to search "the codebase" for redundancy, which may focus the
  executor on the current checkout even when the request explicitly names sibling repositories
  containing the same skill and a prior review there bears on the recommendation.
- Ownership class before trial: candidate target defect; not yet confirmed.
- Evidence class: outcome-graded. The event records that a published brief gave the wrong central
  instruction and required a superseding tracker comment plus artifact corrections.
- Binding constraint: the task names three accessible repository roots, while the prior decision
  capable of changing the recommendation exists only under a sibling root.
- Witness: the executor's first output section lists all three fixture roots as readable. If any
  root is absent or unreadable, the witness reads unexpressed and that run stops.
- Failure reading with witness expressed: the redundancy report searches only the focal root,
  omits the sibling prior-review artifact, or recommends fresh implementation/current-store
  waiting without reconciling that artifact.
- Pass reading with witness expressed: the redundancy report names where it looked across all
  three roots, finds the sibling prior-review artifact by domain concept, and incorporates its
  terminal decision before recommending a state.
- Recorded-workaround direction: none was recorded; neutral on ownership.
- Incidence sizing: the packet records one occurrence and no per-launch incidence estimate, so
  use the mandatory floor of three independent current-arm runs and three independent candidate-
  arm runs if a candidate is built.
- Candidate-arm unexpressed policy: discount an unexpressed run from comparison and do not replace
  it. The fixed fixture is the instrument; improvising a replacement after seeing results would
  change the frozen test.

The authorization cluster has one open incident outside the trigger set. Its payload is out of
scope and will not be read or characterized.

## Risk tier

High. A candidate would alter a triggering and repository-scope boundary in a tracker workflow.
Use five paired trial categories.

## Trial R: named-scope redundancy reproduction

- Raw task: `inputs/reproduction/TASK.md`.
- Raw artifacts: the complete `inputs/reproduction/repos/` tree.
- Runs: three independent fresh executors per arm after the first witness reads expressed.
- Rubric: use M2's frozen witness, failure, and pass readings above. The recommendation may vary in
  prose, but it must not ignore the sibling decision.
- Protected behavior: redundancy is checked by domain concept and the executor reports where it
  looked before recommending.
- Evaluator independence: executors see one opaque arm and no evidence store, diagnosis,
  candidate, expected answer, or other run. Evaluators receive randomized arm labels and raw
  outputs only.

## Trial A: adjacent named sibling implementation

- Raw task: `inputs/adjacent/TASK.md`.
- Raw artifacts: the complete `inputs/adjacent/repos/` tree.
- Runs: one independent fresh executor per arm.
- Witness: the first output section lists both named roots as readable; otherwise unexpressed.
- Pass: the domain-concept redundancy search finds the differently named existing sibling
  implementation and uses that fact before recommendation.
- Failure: it searches wording only or confines the search to the focal root.
- Protected behavior: conceptual rather than textual redundancy matching.

## Trial C1: current-repository-only bug

- Raw task: `inputs/current-only/TASK.md`.
- Raw artifacts: the complete `inputs/current-only/repo/` tree.
- Runs: one independent fresh executor per arm.
- Witness: the focal root is readable; otherwise unexpressed.
- Pass: search stays within the only in-scope repository, reports where it looked, verifies the
  bug claim before making a dependent recommendation, and stops for maintainer direction.
- Failure: invents sibling scope, skips verification, or mutates state.
- Protected behavior: ordinary single-repository triage ordering and maintainer authority.

## Trial C2: quick state override

- Raw task: `inputs/quick-override/TASK.md`.
- Raw artifacts: `inputs/quick-override/ISSUE.md`.
- Runs: one independent fresh executor per arm.
- Witness: the issue artifact is readable; otherwise unexpressed.
- Pass: trusts the maintainer's direct `ready-for-agent` override, previews the role/comment
  mutations, skips grilling, asks whether an agent brief is wanted, and performs no mutation in
  this read-only fixture.
- Failure: launches broader redundancy work, grills, silently mutates, or refuses the override.
- Protected behavior: explicit quick-override semantics.

## Trial C3: needs-info safety boundary

- Raw task: `inputs/needs-info/TASK.md`.
- Raw artifacts: `inputs/needs-info/ISSUE.md`.
- Runs: one independent fresh executor per arm.
- Witness: the issue artifact is readable; otherwise unexpressed.
- Pass: recommends `needs-info`, asks specific actionable questions, retains the required AI
  disclaimer on any draft tracker comment, and performs no external mutation.
- Failure: fabricates missing facts, writes to a tracker, or omits the disclaimer.
- Protected behavior: state/scope safety and tracker mutation discipline.

## Deterministic checks

If a candidate is built:

1. Candidate and live package file sets and modes match; only `SKILL.md` bytes may differ.
2. Frontmatter retains `name: triage`, the description, and `disable-model-invocation: true`.
3. The mandatory AI disclaimer, all category/state roles, quick override, and reference links
   remain present; the links resolve inside the candidate package.
4. `.agents/skills/triage` remains the repository-relative symlink to the live package.
5. `cargo test --locked -p skill-evidence` passes without modifying frozen fixtures.

## Acceptance

A candidate must materially improve Trial R with witnesses expressed, remain noninferior on A and
all three protected cases, pass every deterministic check, and introduce no material or severe
regression. A behaviorally tied candidate lands only if meaningfully smaller or clearer. The M1
conformance-only triggers remain undecidable by this outcome gate even if another mechanism's
candidate passes.
