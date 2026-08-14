# Decision Report

## Verdict

Adopt F2. Reject F1 and F3. The complete authoritative fixture tree resolves all three findings; no verdict-changing fork remains.

## Dispositions

### F1 — Reject

The claim that a deterministic-looking task may use a single reproduction run is contradicted. The authorized-review procedure requires at least three runs for each runnable reproduction when the first witness reads expressed, unless the evidence supports a higher incidence estimate; determinism is not stated as an exception and incidence-sized repetition is not optional under that condition (`inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:3`).

Coverage: contradicted by direct authority.

### F2 — Adopt

The procedure requires every event in the coverage list to have exactly one concluded or instrument-limited route, and it refuses missing, duplicate, conflicting, unknown, and out-of-coverage routes before any write (`inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:5`). The compiled validator independently enforces an exact disjoint partition of coverage into concluded and limited events (`inputs/repository/src/evolution.rs:8-10`).

Coverage: verified by direct procedural authority and the compiled enforcement seam.

### F3 — Reject

Self-targeting is prohibited in the installed skill prose (`inputs/repository/assets/skills/skill-evolution/SKILL.md:3`), but it is not prose-only: the compiled validator compares the target hash with the operating skill hash and returns `Refusal::SelfTarget` when they match (`inputs/repository/src/evolution.rs:1-5`).

Coverage: contradicted by direct authority.

## Unresolved Verdict-Changing Forks

None.

## Scope and Closeout

In scope: adjudication of F1-F3 against every file in `inputs/repository/`. Out of scope: changing the fixture, implementing repairs, or contacting external systems.

Provenance: `inputs/review-card.md` was adjudicated against the complete authoritative fixture tree: `assets/skills/skill-evolution/SKILL.md`, `assets/skills/skill-evolution/references/authorized-review.md`, and `src/evolution.rs` under `inputs/repository/`.

Convention checked: authorized-review reproduction and coverage-partition rules, plus installed-package self-targeting policy and its compiled refusal seam. No formatter, validator, mirror, or pointer check was supplied for this report artifact, and no repository completion contract appears in the fixture tree.

Domain-doc outcome: domain modeling was considered, but no glossary term or ADR-worthy structural decision crystallized; the fixture tree contains no governing domain-doc surface requiring an update.
