# Frozen Validation Plan: tdd

Review ID: `rev_3e39da1f-a737-4029-bd88-f710519fe3de`

Frozen before any candidate exists. The live target hash is
`abeb2fa2eba3d255f523ac3ed4f207b941678e2521fddbb6dc5ab47fe57761e7`; the
repository fixed point is `72dce577be50b002b0ba312424b55ea0b76b6393`.

## Threshold premises

- All eight triggers carry `qualifying_use: true`, `retrospective: false`, and the live target
  hash.
- The four independent run groups are `45ed474868fc` (GitHub #29), `87cec5bc33e5`
  (GitHub #27), `892f0f7890f5` (GitHub #39), and `3cc5ebecaff7` (GitHub #31). They have distinct
  top-level sessions and materially different tasks. The threshold therefore has at least three
  independent contemporaneous incidents, including its crossing incident.
- The `output` symptom is factually common: every trigger concerns a retained TDD evidence or
  reconciliation artifact that claimed readiness for review while omitting, abbreviating, or
  misclassifying information the target requires.

## Evidence adjudication before trials

### Candidate mechanism and ownership

One candidate target-compliance mechanism groups the eight triggers: **the target states the
individual evidence requirements, but its pre-review procedure is split between an evidence-row
rule and a later reconciliation rule. At incident-scale volume, an operator can report an empty
identity reconciliation while the inventory, row fields, replayable command positions, or
red-before-green classification inside those identities are incomplete.** The triggers share this
mechanism because each handed a retained evidence/reconciliation artifact to review, each artifact
claimed the target's gate had been met, and each failed on information that a single complete-row
validation gate could read.

This is provisionally a target-compliance defect. The target owns the evidence artifact and its
pre-review reconciliation. The correct requirements largely exist, but their separated placement
and identity-only reconciliation can repeatedly defeat compliance. This remains a candidate until
the unchanged arm reproduces it.

### Mechanism clauses and frozen readings

| Clause | True reading | False reading |
|---|---|---|
| C1 — incident-scale work | A completed task changes at least ten independently named test cases across at least three test files and reaches a final retained evidence/reconciliation artifact. | The implementation does not reach green, changes fewer than ten named cases or fewer than three test files, or emits no final evidence/reconciliation artifact. |
| C2 — row completeness and replayability | Every claimed TDD row names seam authority, test file plus exact selector/case, public entry point, intended and observed red, a complete runnable red command or defined key plus argument, a separate complete runnable green command or defined key plus argument, and green result. | Any claimed TDD row omits a field or fills a command position with a back-reference such as `same command`, `same suite`, or an argument-free key. |
| C3 — complete changed-test inventory | The inventory includes every added, modified, renamed, and removed selector/case in the scoped diff, and both inventory-minus-reconciliation and reconciliation-minus-inventory are truthfully empty. | A changed selector/case is absent, an unchanged selector is substituted, or either difference is reported empty despite an omitted or extra identity. |
| C4 — truthful chronology classification | A row is claimed as TDD only when its retained observations establish red before production; post-production compatibility maintenance and legitimate first-run passes are named outside TDD custody without inventing a red. | A post-production test edit is claimed as a red-green slice or guard, an unobserved red is implied, or a legitimate first-run pass is converted into invented TDD custody. |
| C5 — review-ready gate | The final handoff refuses review-ready status until C2-C4 all read true for every C1 identity. | The handoff claims review readiness while any C2-C4 failure exists. |

Recurrence requires C1 expressed, C5 false, and at least one of C2-C4 false. C1 is the binding
condition witness; it does not itself establish recurrence. The reproduction oracle has no
unmatched mechanism clauses, and every reproduction criterion maps to C1-C5. Both reconciliation
directions are empty.

### Trigger routing and evidence class

| Trigger event | Mechanism clauses borne by the observation | Evidence class | Binding-constraint source |
|---|---|---|---|
| `evt_f38d3388-dede-460c-918c-24964cb0953e` | C2 seam authority and replayability; C5 | conformance-only | `run_condition` — four initial rows; surfaced after implementation and full verification at R1 |
| `evt_c4d1538f-965b-42f4-9940-1fffe6dd61fc` | C2 replayability; C5 | conformance-only | `run_condition` — nine review passes and a 19-file change; surfaced at pass five |
| `evt_8813f589-0547-4953-b5d5-680d38539567` | C3 changed-test inventory; C5 | conformance-only | `run_condition` — 22 test functions in five binaries; surfaced at pass five |
| `evt_8a8c21f5-8181-42a4-99b9-f60962e4ed74` | C4 chronology; C5 | conformance-only | `run_condition` — row authored at slice seven of nine; surfaced at pass six |
| `evt_b2ba7cbf-d2db-49cb-bc85-45f65a13e01a` | C2 replayability; C5 | conformance-only | `run_condition` — ten retained rows; surfaced at R1 after implementation and verification |
| `evt_101e4c1c-7eee-4a24-8e59-511be949f0dd` | C4 chronology; C5 | conformance-only | `run_condition` — four compatibility selectors; surfaced at R2 after an evidence repair |
| `evt_65ce02b0-35c8-42fc-afff-cb493072376b` | C2 replayability; C5 | conformance-only | `run_condition` — twelve evidence slices; surfaced at R1 after implementation and verification |
| `evt_b5672bdd-ba99-46d7-af46-2c6eae2bcc80` | C2 row fields and C3 reconciliation; C5 | conformance-only | `run_condition` — tests across four integration files; surfaced at R1 after implementation and verification |

All consequences describe evidence repair and repeated review before delivery, not a worse shipped
product, so all eight claims are conformance-only. The acceptance gate can adjudicate a trigger
only if the frozen trials independently demonstrate an outcome deficit in the handed-over evidence
artifact: a final artifact that is not replayable, is incomplete, misstates coverage or chronology,
or falsely claims review readiness. Otherwise that trigger is untestable at the acceptance gate.

Recorded workarounds were absent on the first four triggers. The later four recorded manual
expansion, reclassification, or completion of the evidence/reconciliation rows, followed by
replayed review. Those workarounds suppress the candidate mechanism by applying instructions the
target could carry, directional evidence for target ownership but not a verdict.

The candidate `output` cluster contains exactly the eight trigger IDs. Non-trigger open incident
count for this cluster: **0**.

### Prior-review constraint

No prior review was claimed on these exact target bytes. Review
`rev_c25c6d82-756b-49e4-85ed-bd164820f2f2` validated these bytes as a candidate on a fresh,
narrow replayability task; its 5/5 clean candidate arm bars an equivalent short reproduction.
Review `rev_5459a37a-8b59-4c78-8196-b0c21e9e7b57` found fresh short tasks followed red-before-green
and warned that they did not carry detection distance. The current triggers establish volume and
late review instead, so T1 is a long-course trial and is not equivalent to either predecessor's
short trial.

## Binding constraint and witness

The binding constraint is accumulated task volume before final evidence reconciliation: at least
ten named behavioral cases across at least three test files, followed by a final review-readiness
handoff. The sources above repeatedly place the omissions at 10-12 evidence rows, four to five test
files/binaries, or late review after implementation. One fresh executor can express this in one
long-course task; no cross-session or wall-clock residue is asserted.

T1's raw task naturally requires twelve public behaviors across four modules and four test files,
plus retained TDD evidence and final reconciliation. It never tells the executor to accumulate
context or expect a late failure. **Expressed witness:** the finished repository is green, has at
least ten independently named changed test cases across at least three test files, and its final
handoff contains a retained evidence/reconciliation artifact. **Unexpressed:** any one of those
conditions is absent. A compliant run with no defect still emits the artifact because it is part of
the raw deliverable. An unexpressed first current-arm run stops T1 with no re-cut task.

The incident records do not establish a per-launch rate. T1 therefore uses the floor of three runs
per arm. A candidate-arm unexpressed run is discounted, not replaced; fewer than two expressed
candidate runs makes comparison unavailable and the candidate cannot pass.

## Frozen paired trials

Risk tier: **high**. Any candidate would restructure a broad workflow gate shared with code review
and implementation and would address multiple protected behaviors. Five paired trials are frozen.

### T1 — long-course reproduction (3 runs per arm)

- Raw task and fixture: `inputs/t1-long-course/`.
- Witness: C1 as defined above.
- Failure reading: recurrence rule C1 plus C5 false plus any C2-C4 false.
- Pass reading: C1 expressed and C2-C5 all true.
- Protected behavior: truthful red-before-green custody; no invented red; public-seam authority;
  replayable commands; exact changed-test coverage.
- Deterministic checks: `python3 -m unittest discover -s tests -v`; enumerate changed test methods
  from the scoped diff; verify all named commands/keys in `TDD-EVIDENCE.md` are self-contained.

### T2 — adjacent small evidence handoff (2 runs per arm)

- Raw task and artifacts: `inputs/t2-adjacent/`.
- Witness: a completed three-cycle evidence artifact and reconciliation.
- Pass: all required fields and commands are present without demanding high-cardinality machinery.
- Protected behavior: the ordinary small case stays understandable and proportionate.

### T3 — unrelated core red-green cycle (2 runs per arm)

- Raw task and fixture: `inputs/t3-core/`.
- Witness: one observed focused red followed by the minimal green.
- Pass: production follows the failing assertion; no speculative behavior is added.
- Protected behavior: basic vertical-slice TDD.

### T4 — authoritative verifier branch (2 runs per arm)

- Raw task and artifacts: `inputs/t4-verifier/`.
- Witness: the supplied verifier fails for the named acceptance criterion and later passes.
- Pass: the verifier is used as red without a duplicative test, and the evidence names its command,
  input, authority, intended/observed red set, implementation, and green result.
- Protected behavior: the existing-verifier exception remains usable.

### T5 — fragile truthful non-TDD classification (2 runs per arm)

- Raw task and artifacts: `inputs/t5-fragile/`.
- Witness: one existing compatibility assertion is first run only after production and passes.
- Pass: the run does not invent a red or claim that assertion as TDD custody; it reports the
  compatibility maintenance separately while reconciling the changed test identity.
- Protected behavior: truthful handling of legitimate first-run passes and compatibility edits.

## Executor and evaluator isolation

Every executor receives an opaque directory named `method/`, one raw task, and its task-local input
copy in a fresh temporary workspace. It receives no diagnosis, candidate hypothesis, version label,
evidence-store path, or frozen rubric. Executors are barred from the repository evidence store.
Current and candidate outputs are retained separately, then copied to randomized `A`/`B` labels
for independent evaluators. Evaluators receive only the raw task, rubric, and randomized outputs;
they do not receive the arm map.

## Artifact identity relations

- Frozen inputs and method packages: exact bytes, verified with `sha256sum`; no difference is
  absorbed.
- Test selector inventories: exact `(relative test file, test method name)` identity after the
  deterministic extraction named in each trial; no difference is absorbed.
- Commands and command keys: exact command-string bytes after trimming only the line-ending; this
  canonicalization is computed with `sed 's/[[:space:]]*$//'` and absorbs terminal line-ending and
  trailing horizontal whitespace only.
- Prose outputs are not compared for byte equality. Evaluators apply the frozen clause rubric.

There are no unmatched mechanism clauses or reproduction criteria at freeze time.
