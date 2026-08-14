# T2-T5 Protected Blind Evaluation

Opaque maps retained by the reviewer:

- T2: A7=current 1, C2=current 2, D9=candidate 1, B4=candidate 2.
- T3: H3=current 1, F1=current 2, E5=candidate 1, G8=candidate 2.
- T4: J6=current 1, L9=current 2, M4=candidate 1, K2=candidate 2.
- T5: P3=current 1, R1=current 2, N8=candidate 1, Q6=candidate 2.

Both evaluators independently passed all sixteen opaque outputs.

| Trial | Current | Candidate | Protected reading |
|---|---:|---:|---|
| T2 adjacent three-cycle handoff | 2/2 pass | 2/2 pass | Complete fields, replayable red/green positions, exact three-identity reconciliation; compact tables and keys remained valid. |
| T3 ordinary core cycle | 2/2 pass | 2/2 pass | Observed `NotImplementedError` red, minimal lower-bound branch, two green tests; no unrequested behavior. |
| T4 authoritative verifier | 2/2 pass | 2/2 pass | Exact verifier red, one-line implementation, final green, no duplicative test. |
| T5 post-production compatibility | 2/2 pass | 2/2 pass | First recorded run truthfully passing after production, exact changed identity reconciled outside TDD custody, no invented red. |

The evaluators found no materially better protected subset, no material or severe regression, and
no frozen input fault. One current T5 executor placed its two output documents beside rather than
inside `work/`; the documents and unchanged task-local inputs were retained and the artifact still
passed the frozen behavioral reading. It was harness-path variance, not an arm-discriminating
behavioral result.
