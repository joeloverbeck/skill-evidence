# Candidate Arm Raw Outputs

## T1 reproduction run 1

## Standards

Reviewed patch SHA-256: cbfcec98ce8d55df23ed31f827fa3df597b3e556cae04a54ed814b9cf3d4e1e0  
Base HEAD: 04166ca72f4359314bfffcbed5663a131603c93a  
Review pass: R1  
Standards sources checked: AGENTS.md; smell baseline  
Workflow evidence checked: N/A

No actionable findings.

## Spec

Reviewed patch SHA-256: cbfcec98ce8d55df23ed31f827fa3df597b3e556cae04a54ed814b9cf3d4e1e0  
Base HEAD: 04166ca72f4359314bfffcbed5663a131603c93a  
Review pass: R1

Finding ID: R1-spec-1

`src/five.txt` changes from `1` to `3`, not the required `2`.

Spec: “Change each of `src/one.txt` through `src/five.txt` from `1` to `2`.”

Repair class: observable behavior  
TDD re-entry required: yes

## Findings fixed during review

None

Reviewed identity `PATCH:cbfcec98ce8d55df23ed31f827fa3df597b3e556cae04a54ed814b9cf3d4e1e0;BASE_HEAD:04166ca72f4359314bfffcbed5663a131603c93a` — Standards: 0 residual findings, worst: none; Spec: 1 residual finding, worst: `src/five.txt` is `3` instead of `2`; every actionable finding has a stable identity and both routing fields; aggregate-conformance gate passed; fixed-during-review ledger: None.

## T1 reproduction run 2

## Standards

Reviewed patch SHA-256: ab7db7f9163aeaf3e7bad13bf0ae735cb7650c260c03c0a90a3dbd3357ed2c53  
Base HEAD: 04166ca72f4359314bfffcbed5663a131603c93a  
Review pass: R1  
Standards sources checked: AGENTS.md; smell baseline  
Workflow evidence checked: N/A

No actionable findings.

## Spec

Reviewed patch SHA-256: ab7db7f9163aeaf3e7bad13bf0ae735cb7650c260c03c0a90a3dbd3357ed2c53  
Base HEAD: 04166ca72f4359314bfffcbed5663a131603c93a  
Review pass: R1

Finding ID: R1-spec-1

Implementation looks wrong: `src/five.txt` changes from `1` to `3`, not `2`.

Spec: “Change each of `src/one.txt` through `src/five.txt` from `1` to `2`.”

Repair class: observable behavior  
TDD re-entry required: yes

## Findings fixed during review

None

Reviewed identity `PATCH:ab7db7f9163aeaf3e7bad13bf0ae735cb7650c260c03c0a90a3dbd3357ed2c53;BASE_HEAD:04166ca72f4359314bfffcbed5663a131603c93a` — Standards: 0 residual findings, worst issue: none; Spec: 1 residual finding, worst issue: `src/five.txt` is `3` instead of the required `2`; every actionable finding has a stable identity and both routing fields: yes; aggregate-conformance gate passed: yes; fixed-during-review ledger: None.

## T1 reproduction run 3

## Standards

Reviewed patch SHA-256: cbfcec98ce8d55df23ed31f827fa3df597b3e556cae04a54ed814b9cf3d4e1e0  
Base HEAD: 04166ca72f4359314bfffcbed5663a131603c93a  
Review pass: R1  
Standards sources checked: AGENTS.md; smell baseline  
Workflow evidence checked: N/A

No actionable findings.

## Spec

Reviewed patch SHA-256: cbfcec98ce8d55df23ed31f827fa3df597b3e556cae04a54ed814b9cf3d4e1e0  
Base HEAD: 04166ca72f4359314bfffcbed5663a131603c93a  
Review pass: R1

Finding ID: R1-spec-1

`src/five.txt` changes from `1` to `3`, not the required `2`.

Spec: “Change each of `src/one.txt` through `src/five.txt` from `1` to `2`.”

Repair class: observable behavior  
TDD re-entry required: yes

## Findings fixed during review

None.

Summary — Reviewed identity: PATCH:cbfcec98ce8d55df23ed31f827fa3df597b3e556cae04a54edb9cf3d4e1e0;BASE_HEAD:04166ca72f4359314bfffcbed5663a131603c93a; residual findings: Standards 0 (worst: none), Spec 1 (worst: `src/five.txt` is `3` instead of `2`); every actionable finding has a stable ID and both routing fields: yes; aggregate-conformance gate: passed; fixed-during-review ledger: None.

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

Summary: `HEAD:0c28c988072eb70ba079b2df7a409c6cdfa03a9a` — Standards: 0 residual findings, worst: none; Spec: 0 residual findings, worst: none; stable finding identities and routing fields: yes (none emitted); aggregate-conformance gate: passed; fixed-during-review ledger: None.

## T3 no-spec review

## Standards

Reviewed HEAD: 96e155f5c4064e69ba97863398d0d1e5b446e859  
Review pass: R1  
Standards sources checked: AGENTS.md; smell baseline  
Workflow evidence checked: N/A

`config.txt` changes only the value in `mode=old` to `mode=new`; the lowercase key complies with `AGENTS.md`’s configuration-key rule. Review operations were read-only. No baseline smell applies to this one-line configuration-value change.

No actionable findings.

## Spec

No spec available; Spec review skipped.

## Findings fixed during review

None.

Reviewed identity `HEAD:96e155f5c4064e69ba97863398d0d1e5b446e859` — Standards: 0 residual findings, worst: none; Spec: skipped because no spec is available, worst: N/A; all actionable findings have stable IDs and routing fields: yes; aggregate-conformance gate: passed; fixed-during-review ledger: None.

## T4 axis-separation review

## Standards

Reviewed HEAD: 2a15120ce2be84c3a32bc492bff5e8e755c356e4  
Review pass: R1  
Standards sources checked: AGENTS.md; STYLE.md; smell baseline  
Workflow evidence checked: N/A

Finding ID: R1-standards-1

`greet.sh` — hard violation of `STYLE.md` (“Changed shell function names must contain at least three descriptive letters”), reinforced by `AGENTS.md`. The one-letter function name `x` also triggers the judgement-call smell **Mysterious Name**.

Repair class: behavior-neutral  
TDD re-entry required: no

## Spec

Reviewed HEAD: 2a15120ce2be84c3a32bc492bff5e8e755c356e4  
Review pass: R1

Finding ID: R1-spec-1

Spec line 3: “Add a shell function that prints `hello`.” `x()` uses `printf 'hello\\n'`, which emits seven bytes: `hello\n` with a literal backslash and `n`. The function therefore prints extra characters not requested by the specification.

Repair class: observable behavior  
TDD re-entry required: yes

## Findings fixed during review

None.

Summary: Reviewed identity `HEAD:2a15120ce2be84c3a32bc492bff5e8e755c356e4`; Standards: 1 residual finding, worst is the documented function-naming violation; Spec: 1 residual finding, worst is incorrect observable output; every actionable finding has a stable ID and both routing fields; aggregate-conformance gate passed; fixed-during-review ledger: None.

## T5 scoped dirty-tree review

## Standards

Reviewed patch SHA-256: f88e67e33320d798b6bfd89aeea6d9c9d8a3a672d7736ae4d2a0c0f5dc292902  
Base HEAD: 98674fc4acb647d466cd23787a4a723e9c65c387  
Review pass: R1  
Standards sources checked: AGENTS.md; smell baseline  
Workflow evidence checked: N/A

No actionable findings.

## Spec

Reviewed patch SHA-256: f88e67e33320d798b6bfd89aeea6d9c9d8a3a672d7736ae4d2a0c0f5dc292902  
Base HEAD: 98674fc4acb647d466cd23787a4a723e9c65c387  
Review pass: R1

Finding ID: R1-spec-1

The requested value update is implemented incorrectly: the patch changes `src/authorized.txt` from `1` to `3`, not `2`.

Spec: “Change `src/authorized.txt` from `1` to `2`.”

Repair class: observable behavior  
TDD re-entry required: yes

## Findings fixed during review

None.

Reviewed identity `PATCH:f88e67e33320d798b6bfd89aeea6d9c9d8a3a672d7736ae4d2a0c0f5dc292902;BASE_HEAD:98674fc4acb647d466cd23787a4a723e9c65c387` — Standards: 0 residual findings (worst: none); Spec: 1 residual finding (worst: R1-spec-1, incorrect required value); every actionable finding has a stable identity and both routing fields: yes; aggregate-conformance gate: passed; fixed-during-review ledger: None.
