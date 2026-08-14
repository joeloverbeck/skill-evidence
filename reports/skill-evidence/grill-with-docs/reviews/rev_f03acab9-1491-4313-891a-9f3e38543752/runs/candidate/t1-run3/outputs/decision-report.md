# Decision Report

## Verdict

The review card is partially correct: F2 is verified, while F1 and F3 are contradicted by the authoritative fixture.

| Finding | Disposition | Evidence and rationale |
|---|---|---|
| F1 | Reject | `inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:3` requires at least three runs for each runnable reproduction when the first witness reads expressed, using three as the floor absent evidence for a higher incidence. That directly contradicts the claimed permission for one run and optional incidence-sized repetition. |
| F2 | Uphold | `inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:5` requires exactly one concluded or instrument-limited route for every covered event and refuses missing, duplicate, conflicting, unknown, or out-of-coverage routes before writing. `inputs/repository/src/evolution.rs:8-9` supplies the corresponding exact-disjoint-partition validation seam. |
| F3 | Reject | The prohibition exists in prose at `inputs/repository/assets/skills/skill-evolution/SKILL.md:3`, and the compiled seam also refuses self-targeting: `inputs/repository/src/evolution.rs:1-4` compares the target and operating hashes and returns `Refusal::SelfTarget` when they match. |

## Unresolved forks

None. The complete authoritative fixture resolves every claim without unavailable or pending evidence.

## Scope and provenance

In scope: adjudication of F1-F3 against the complete `inputs/repository/` fixture tree. Out of scope: changes to inputs, implementation, domain documentation, or external systems.

Provenance: `inputs/review-card.md` was checked against all three files in `inputs/repository/`.

Convention checked: claims were assigned warrant only from the authoritative fixture, with prose requirements distinguished from compiled enforcement. No formatter, validator, mirror, or pointer proof applies to this Markdown-only decision report.

Domain modeling was considered; no domain-doc update was needed because the fixture contains no governing glossary or ADR surface, and the adjudication crystallized no new domain term or architectural decision.
