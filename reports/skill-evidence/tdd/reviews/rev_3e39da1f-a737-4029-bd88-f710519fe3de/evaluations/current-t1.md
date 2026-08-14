# T1 Current-Arm Blind Evaluation

Opaque map retained by the reviewer: Q7 = current run 1; M2 = current run 2; Z4 = current run 3.
The two evaluators did not receive this map or any method package.

## Evaluator A

| Output | C1 | C2 | C3 | C4 | C5 | Recurrence |
|---|---|---|---|---|---|---|
| Q7 | true | false | false | false | false | met |
| M2 | true | false | false | false | false | met |
| Z4 | true | false | false | false | false | met |

- Q7: 14 green tests across four files. G4 and S1 say their first focused run was already green
  while appearing under `Retained red/green cycles`. The inventory claims an empty difference but
  omits the removed legacy selectors in all four test files.
- M2: 13 green tests across four files. E08 and E09 say `No truthful red existed` while appearing
  under `Retained cycle evidence`; the same four removed identities are absent from reconciliation.
- Z4: 12 green tests across four files. S01 says `Red command: not applicable` while appearing
  under `Retained cycles`; the same four removed identities are absent from reconciliation.
- No frozen-input fault. Each diff applied to the baseline and each reconstructed suite was green.

## Evaluator B

Evaluator B independently returned the same clause table and recurrence readings. Its decisive
text was the same: Q7's `two first-green rows`, M2's `No truthful red existed`, Z4's
`Red command: not applicable`, and all three artifacts' false empty-difference claims despite the
four removed legacy selector identities in their retained diffs.

It also found no frozen-input fault. Binary `__pycache__` changes in M2 and Z4 were noted as
incidental input/runtime artifacts and did not affect a frozen reading.

## Reviewer reading

The two independent evaluators agree that C1 was expressed and the frozen recurrence rule was met
in all three current-arm runs. Candidate construction is therefore authorized for the single
mechanism frozen in the plan.
