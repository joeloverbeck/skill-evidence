# Decision Report

## Verdict

Reject F1 and F3; adopt F2. The complete authoritative fixture resolves every material claim, so no outcome-changing fork remains.

## Dispositions

### F1 — Reject

The claim is contradicted. The authorized procedure requires at least three runs for each runnable reproduction when the first witness reads `expressed`; absent evidence for a higher incidence estimate, three is the floor, not an optional repetition count.

Stable evidence: `inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:3`.

### F2 — Adopt

The claim is verified. The procedure requires every event in the coverage list to have exactly one concluded or instrument-limited route and refuses missing, duplicate, conflicting, unknown, or out-of-coverage routes before any write. The compiled validator delegates to an exact disjoint-partition check.

Stable evidence: `inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:5`; `inputs/repository/src/evolution.rs:8-10`.

### F3 — Reject

The claim is contradicted. Prose forbids self-targeting, and the compiled validation seam compares the target hash with the operating skill hash and returns `Refusal::SelfTarget` when they match.

Stable evidence: `inputs/repository/assets/skills/skill-evolution/SKILL.md:3`; `inputs/repository/src/evolution.rs:1-5`.

## Boundaries and closeout

- In scope: adjudicating F1-F3 against the complete fixture tree.
- Out of scope: altering the fixture, implementing changes, or contacting external systems.
- Unresolved verdict-changing forks: none.
- Domain-doc outcome: domain modeling was considered; no domain-doc update was needed because the adjudication applies existing fixture terms and crystallizes no new domain term or ADR-worthy structural decision.
- Completion proof: the report was reconciled against every file in the authoritative fixture tree; no fixture-specific formatter, validator, mirror, or pointer check is declared.

Provenance: `inputs/review-card.md` adjudicated against `inputs/repository/` (`assets/skills/skill-evolution/SKILL.md`, `assets/skills/skill-evolution/references/authorized-review.md`, and `src/evolution.rs`).
