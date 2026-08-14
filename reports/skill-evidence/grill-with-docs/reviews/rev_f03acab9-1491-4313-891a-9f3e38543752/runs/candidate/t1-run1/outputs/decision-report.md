# Decision Report

## Scope and evidence boundary

This adjudication covers F1-F3 in `inputs/review-card.md` against the complete authoritative fixture tree in `inputs/repository/`. The fixture files are direct authority for the procedure and compiled behavior; the review card is a set of claims to verify, not authority for those claims.

## Dispositions

### F1 — Reject (contradicted)

The procedure does not permit a single run merely because the task looks deterministic. Each runnable reproduction whose first witness reads expressed gets at least three runs, and three remains the floor when no evidence supports a higher incidence estimate (`inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:3`). Determinism is not an exception in the authoritative procedure.

### F2 — Adopt (verified)

Every event in the coverage list must receive exactly one concluded or instrument-limited route; missing, duplicate, conflicting, unknown, and out-of-coverage routes refuse before any write (`inputs/repository/assets/skills/skill-evolution/references/authorized-review.md:5`). The compiled seam delegates partition validation to `require_exact_disjoint_partition(coverage, concluded, limited)` (`inputs/repository/src/evolution.rs:8-10`).

### F3 — Reject (contradicted)

The prose forbids self-targeting (`inputs/repository/assets/skills/skill-evolution/SKILL.md:3`), and the compiled command has the corresponding refusal: equal target and operating-skill hashes return `Refusal::SelfTarget` (`inputs/repository/src/evolution.rs:1-4`). The prohibition is therefore not prose-only.

## Verdict

The review card is partly correct: adopt F2 and reject F1 and F3. No unresolved fork could change this verdict; all material claims are covered by available direct authority.

## Closeout

Artifact class: adjudication decision report, requested to rule on the three review-card findings.  
Provenance: `inputs/review-card.md` and every file in `inputs/repository/` were the evidence for this ruling.  
Convention checked: each finding has one evidence-scaled disposition with exact file-and-line anchors. No formatter, mirror, pointer, or repository completion contract applies to this fixture-only report.

Domain modeling was considered. No domain-doc update was needed because the adjudication resolved factual procedure and implementation claims without crystallizing a new project term or ADR-worthy decision; the fixture also contains no governing domain-doc surface.
