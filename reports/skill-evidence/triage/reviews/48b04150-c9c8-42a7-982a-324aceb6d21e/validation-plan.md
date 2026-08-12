# Frozen Validation Plan: triage

Review ID: `48b04150-c9c8-42a7-982a-324aceb6d21e`

Frozen before any candidate exists. The live target hash is
`9968ebd3d4508f01bb91ddac011c09330651bcea7c9a5fdcd9dcad593cb4f4bc`.

## Evidence routes and binding constraints

### M1 — silent prior-rejection-check omission

- Trigger: `evt_8059f523-efc3-4fe0-bb88-3082bebc2d53`.
- Candidate ownership: target-compliance defect in step 1. The right check exists, but a negative
  prior-rejection result has no required output and can disappear inside a large triage run.
- Evidence class: conformance-only. The recorded consequence says the triage outcome was unchanged.
- Binding constraint: the repository has no `.out-of-scope/` directory and therefore a compliant
  check can find nothing.
- Provenance: the event's `consequence` records a complete same-session root listing with no
  `.out-of-scope/` directory and says the result was unaffected.
- Witness assessment: unable to be expressed. A compliant run that finds nothing is not required by
  the current target to emit that negative result. Requiring an executor to report it would add a
  behavioral output requirement solely to expose the witness, which is forbidden.
- Trial slot: no run. Unable to be expressed by this workflow's finished-output instrument.

### M2 — repository-local prior-rejection source resolution

- Trigger: `evt_741ccc97-a4f0-445b-ab0c-64420bb4b31d`.
- Candidate ownership: target defect. Step 1 and `OUT-OF-SCOPE.md` prescribe `.out-of-scope/`
  unconditionally, while the recorded repository has no such directory and uses an ADR for the
  relevant durable rejection.
- Evidence class: conformance-only. The recorded consequence says no wrong artifact was written;
  the relevant branch did not execute.
- Binding constraint: no `.out-of-scope/` directory, with the request's matching prior rejection
  present only in the repository's required ADR corpus.
- Provenance: `observed` records the missing directory and the actual ADR convention; `run_condition`
  records that the missing-path branch was live during the triage use.
- Witness: the finished recommendation cites the matching ADR rejection and does not recommend the
  rejected feature for implementation. The witness is unexpressed if the executor cannot read the
  supplied issue or ADR, or emits no category/state recommendation.
- Failure reading: the recommendation advances the rejected enhancement, or claims no prior
  decision exists, without surfacing the matching ADR.
- Current-arm sizing: three fresh runs. The event gives no per-launch incidence estimate, so the
  floor of three applies. Read run 1's witness first; if unexpressed, stop this trial without
  recutting it. Otherwise finish all three runs.
- Candidate-arm sizing: three fresh runs if a candidate is constructed.

### M3 — paired-workflow invocation loses `/domain-modeling`

- Trigger: `evt_51d8ebbf-947f-4bfb-9474-ae470258c501`.
- Candidate ownership: target-compliance defect in step 4. The correct paired instruction exists,
  but `/domain-modeling` was omitted after `/grilling` was invoked.
- Evidence class: conformance-only. The consequence records the delivered-work effect as
  undetermined.
- Binding constraint: accumulated context distance and competing work at step 4 of an approximately
  forty-tool-call triage session.
- Provenance: the event's `run_condition` places the omission inside that long run and says it
  surfaced only at the end.
- Witness assessment: unable to be expressed by a fresh, short-context executor. The same-target
  predecessor review `5833275e-4998-450e-98dd-49a0bd8939a6` reached the same ruling for this
  mechanism shape. Its operating Skill Evolution hash differs from this review's hash, so the ruling
  is evidence rather than governing procedure.
- Trial slot: no run. This workflow has no further instrument for the long-session constraint.

The authorization cluster contains five non-trigger open incident IDs. Their payloads were not read
or characterized.

## Risk tier

High. A candidate could change triage guidance that controls external tracker actions, rejection
memory, and cross-skill invocation. Validation therefore freezes five paired trial categories.

## Raw-task custody and executor logistics

Each trial directory contains `raw-task.md` and a `fixture/` tree. The raw task is separate from
executor logistics. An executor receives only:

- the raw task and fixture path;
- the path to one opaque held skill package;
- permission to read those inputs and write only its returned answer;
- a prohibition on reading `reports/skill-evidence/`, this plan, any review report, another arm,
  or any candidate bytes;
- no diagnosis, repair hypothesis, expected answer, rubric, or version label.

Naming the held package and fixture paths is logistics. No search directive, output field, scope
expansion, or witness hint is added by the harness.

## Frozen paired trials

### R — repository-local rejection source reproduction

- Raw task: `trials/R/raw-task.md`.
- Raw artifacts: `trials/R/fixture/`.
- Mechanism: M2.
- Runs: three per arm, subject to the first-current-witness rule above.
- Comparison rubric: materially better only if the candidate reliably surfaces the binding ADR and
  prevents advancement of the rejected enhancement where the current arm does not.
- Protected behavior: codebase/ADR context is considered before recommendation; the maintainer keeps
  authority over the state transition.
- Deterministic checks: recommendation contains one category and one state; cited artifact exists in
  the fixture; no fixture file changed.

### A — canonical `.out-of-scope/` adjacent case

- Raw task: `trials/A/raw-task.md`.
- Raw artifacts: `trials/A/fixture/`.
- Runs: one per arm.
- Pass: surface the matching out-of-scope record, preserve the maintainer's reconsider/confirm
  choice, and avoid proposing a duplicate record.
- Protected behavior: repositories that use the canonical knowledge-base convention continue to
  work.

### C1 — needs-attention discovery

- Raw task: `trials/C1/raw-task.md`.
- Raw artifacts: `trials/C1/fixture/`.
- Runs: one per arm.
- Pass: show only the three required buckets, oldest first within each, include the external PR,
  exclude the collaborator PR and unrelated states, and label issue versus PR.
- Protected behavior: discovery state-machine and external-PR filtering.

### C2 — quick state override

- Raw task: `trials/C2/raw-task.md`.
- Raw artifacts: `trials/C2/fixture/`.
- Runs: one per arm.
- Pass: trust the requested state, skip grilling, confirm the intended mutation boundary, and ask
  whether an agent brief is wanted because no grilling session supplied one.
- Protected behavior: maintainer override authority and no unauthorized extra work.

### S — needs-info safety

- Raw task: `trials/S/raw-task.md`.
- Raw artifacts: `trials/S/fixture/`.
- Runs: one per arm.
- Pass: recommend `bug` plus `needs-info`, distinguish unverified claim from fact, and ask specific
  actionable questions without inventing reproduction evidence or tracker mutations.
- Protected behavior: safe handling of insufficient bug reports.

## Candidate-arm witness policy

For a candidate-arm run whose frozen witness is unexpressed, replace it once with a fresh session
using byte-identical raw task, fixture, and logistics. If the replacement is also unexpressed, the
candidate fails that trial. Current-arm reproduction run 1 is never replaced: an unexpressed witness
stops that mechanism's trial as specified above.

## Blind evaluation

Two fresh evaluators independently grade every retained output. Each sees opaque sample labels with
the current/candidate mapping independently randomized, the raw task, fixture, and frozen rubric,
but neither package, diagnosis, evidence store, nor the other evaluator's decisions. Both must agree
that every protected trial is noninferior and that each reproduced mechanism is materially improved.
Any material or severe regression rejects the candidate.

## Deterministic package checks

- Candidate contains exactly the target package's file/link shape unless a demonstrated mechanism
  requires less; no scripts or executable bits.
- Every relative Markdown link in changed candidate files resolves inside the package or to an
  existing repository contract.
- Candidate hash and runtime byte/word counts are recorded before validation.
- Diff is confined to the isolated candidate until compiled landing.
- The live target hash still matches the claim baseline before validation and before landing.
- The compiled landing command must verify the `.agents` mirror symlink.

## Acceptance

Accept only if a reproduced M2 outcome deficit is materially improved, all protected trials are
noninferior, deterministic checks pass, and no safety/scope/ownership invariant regresses. Because
all three trigger events are conformance-only, any trigger whose trials do not demonstrate an
outcome deficit is named as untestable coverage at close under the acceptance-gate ground. A clean
conformance improvement alone cannot authorize landing.
