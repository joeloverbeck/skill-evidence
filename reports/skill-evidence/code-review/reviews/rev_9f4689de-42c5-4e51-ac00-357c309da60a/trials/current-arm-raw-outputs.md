# Current Arm Raw Outputs

## T1 reproduction run 1

Review cannot proceed: the supplied fixed point and reviewed `HEAD` both resolve to `04166ca72f4359314bfffcbed5663a131603c93a`, so the required pinned three-dot diff is empty. The package terminates before launching reviewers.

## T1 reproduction run 2

Review blocked before reviewer launch.

Fixed point source: user-supplied current `HEAD`  
Fixed point SHA: `04166ca72f4359314bfffcbed5663a131603c93a`  
Reviewed HEAD SHA: `04166ca72f4359314bfffcbed5663a131603c93a`

The required pinned diff is empty because both endpoints are the same commit. The package cannot represent uncommitted changes using its required commit-SHA review inputs, so no Standards or Spec reviewers were launched.

## T1 reproduction run 3

Review blocked during fixed-point pinning.

- Fixed point source: repository’s current `HEAD`
- Fixed point SHA: `04166ca72f4359314bfffcbed5663a131603c93a`
- Reviewed HEAD SHA: `04166ca72f4359314bfffcbed5663a131603c93a`
- Required pinned diff: empty

The package requires a non-empty commit-to-commit pinned diff before launching reviewers. The uncommitted changes are not represented by that diff, so the Standards and Spec reviews were not launched.

## T2 committed review

## Standards

Reviewed HEAD: 0c28c988072eb70ba079b2df7a409c6cdfa03a9a  
Review pass: R1  
Standards sources checked: AGENTS.md; smell baseline  
Workflow evidence checked: N/A

No actionable findings.

## Spec

Reviewed HEAD: 0c28c988072eb70ba079b2df7a409c6cdfa03a9a  
Review pass: R1

No actionable findings.

## Findings fixed during review

None.

Reviewed HEAD `0c28c988072eb70ba079b2df7a409c6cdfa03a9a`: Standards—0 residual findings, worst none; Spec—0 residual findings, worst none; all actionable findings carry stable identities and routing fields (vacuously); aggregate-conformance gate passed; fixed-during-review ledger: None.

## T3 no-spec review

## Standards

Reviewed HEAD: 96e155f5c4064e69ba97863398d0d1e5b446e859  
Review pass: R1  
Standards sources checked: AGENTS.md; smell baseline  
Workflow evidence checked: N/A

No actionable findings.

## Spec

No spec available; Spec review skipped.

## Findings fixed during review

None.

Reviewed HEAD 96e155f5c4064e69ba97863398d0d1e5b446e859 — Standards: 0 residual findings, worst: none; Spec: skipped, worst: none; all actionable findings have stable identities and routing fields; aggregate-conformance gate passed; fixed-during-review ledger: None.

## T4 axis-separation review

## Standards

Reviewed HEAD: 2a15120ce2be84c3a32bc492bff5e8e755c356e4  
Review pass: R1  
Standards sources checked: AGENTS.md; STYLE.md; smell baseline  
Workflow evidence checked: N/A

Finding ID: R1-standards-1

Hard violation; `greet.sh:3`: `x() {` violates `STYLE.md` (“Changed shell function names must contain at least three descriptive letters”), as applied by `AGENTS.md`. It is also a judgement-call Mysterious Name smell: `x` does not reveal that the function prints a greeting. Rename it descriptively, for example `greet()`.

Repair class: behavior-neutral  
TDD re-entry required: no

## Spec

Reviewed HEAD: 2a15120ce2be84c3a32bc492bff5e8e755c356e4  
Review pass: R1

Finding ID: R1-spec-1

The added function prints the literal seven-byte sequence `hello\n`, not `hello` followed by a newline. The format string contains `\\n`, which `printf` renders as a backslash and `n`.

Spec: “Add a shell function that prints `hello`.”

Repair class: observable behavior  
TDD re-entry required: yes

## Findings fixed during review

None.

Reviewed HEAD `2a15120ce2be84c3a32bc492bff5e8e755c356e4` — Standards: 1 residual finding (worst: documented function-naming violation); Spec: 1 residual finding (worst: incorrect output bytes); every actionable finding has a stable identity and both routing fields; aggregate-conformance gate passed; fixed-during-review ledger: None.

## T5 scoped dirty-tree review

Blocked: fixed point and reviewed HEAD both resolve to `98674fc4acb647d466cd23787a4a723e9c65c387`, so the package-required pinned diff `git diff <fixed-point-sha>...<reviewed-head-sha>` is empty. The package does not define a review input for uncommitted worktree changes, so no Standards or Spec reviewers were launched.
