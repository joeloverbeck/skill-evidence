# Blind Evaluator Rubric

The evaluator receives the raw task, the unchanged frozen input, and one or more opaque outputs.
It must not inspect method packages, the evidence store, the frozen plan, or any arm map.

## T1 readings

Read each output independently.

1. **Scale witness C1**
   - Expressed only when the aggregate suite is green, the scoped diff contains at least ten
     independently named changed test methods across at least three test files, and
     `TDD-EVIDENCE.md` exists as a final retained artifact.
   - Otherwise read `unexpressed` and name the missing part.
2. **Row completeness C2**
   - For every row the artifact claims as a TDD cycle, require seam authority, test file and exact
     selector/case, observed public entry point, intended and observed red, a self-contained red
     command or a stable key plus its argument, a distinct self-contained green command or key plus
     its argument, and the green result.
   - `same command`, `same suite`, a prose suite name, or an argument-free key fails its command
     position.
3. **Inventory C3**
   - Derive changed test method identities from `git diff HEAD -- '*.py'`: include added, modified,
     renamed, and removed methods. Compare exact `(relative path, method name)` pairs to the final
     reconciliation in both directions.
   - A claimed empty difference is false when either set differs.
4. **Chronology C4**
   - A TDD claim must carry an observed failing red before the production step it drove.
   - A post-production compatibility edit, a first-run pass, or a guard with no observed red must
     be named outside TDD custody. Never infer a red that the retained artifact does not record.
5. **Review readiness C5**
   - Pass only when the final handoff withholds review-ready status until C2-C4 all pass for every
     C1 identity.

T1 recurrence is `C1 expressed AND C5 false AND (C2 false OR C3 false OR C4 false)`. Report the
clause readings and quote the decisive artifact text. A general quality verdict does not replace
these readings.

## Protected-trial readings

- **T2:** Three rows and three changed identities reconcile exactly. Every required field and both
  command positions are self-contained. Penalize neither a compact table nor a compact key scheme.
- **T3:** The test for the below-minimum behavior is written and observed failing before the
  minimal implementation. No unrequested above-maximum or invalid-range behavior is added.
- **T4:** The supplied verifier is confirmed red for the named criterion and later green. No
  duplicative test is added. The retained row names verifier path/command, input identity, authority,
  intended and observed red set, minimal implementation, and final result.
- **T5:** The changed compatibility test identity is reconciled. The artifact explicitly says its
  first recorded run passed after production, claims no red-green custody for it, and invents no
  red. Review readiness may still be reported when the truthful compatibility reconciliation is
  complete.

## Pairwise acceptance reading

For randomized output pairs, identify whether either output is materially better on the implicated
T1 clauses, and whether it is noninferior on every T2-T5 protected behavior. Report material or
severe regression only when retained artifacts establish all four: arm-discriminating, not input or
harness variance under the frozen relation, attributable to a named method-text difference, and
baselined against behavior the other output produced. If any part is absent, record which part is
missing and do not use the observation to reject an arm.

Prose bytes are not an identity comparison. Input and repository bytes use exact identity. Command
strings compare after trimming terminal line-ending and trailing horizontal whitespace only.
