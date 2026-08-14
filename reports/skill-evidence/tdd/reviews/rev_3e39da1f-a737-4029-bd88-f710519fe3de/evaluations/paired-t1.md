# T1 Paired Blind Evaluation

Opaque arm map retained by the reviewer:

- A13 = current run 1; B42 = candidate run 2
- C77 = current run 3; D05 = candidate run 1
- E91 = current run 2; F28 = candidate run 3

Neither evaluator received the map or either method package.

Both evaluators independently returned the same table:

| Output | C1 | C2 | C3 | C4 | C5 | Recurrence |
|---|---|---|---|---|---|---|
| A13 | expressed | false | false | false | false | yes |
| B42 | expressed | true | true | true | true | no |
| C77 | expressed | false | false | false | false | yes |
| D05 | expressed | true | true | true | true | no |
| E91 | expressed | false | false | false | false | yes |
| F28 | expressed | true | true | true | true | no |

The evaluators therefore identified `{B42, D05, F28}` as materially better than
`{A13, C77, E91}` on the implicated mechanism. Their discriminating evidence was consistent:
the better set classified first-run passes and post-production selector edits outside TDD custody,
included all four removed legacy selector identities, and made truthful empty-difference claims.
The other set retained redless cases inside cycle custody, omitted those removals, and nonetheless
claimed complete reconciliation.

All six aggregate suites were independently green. No frozen-input fault or T1-visible protected
regression was established. Both evaluators withheld a full acceptance conclusion until the
separate T2-T5 outputs were available, as the frozen gate required.
