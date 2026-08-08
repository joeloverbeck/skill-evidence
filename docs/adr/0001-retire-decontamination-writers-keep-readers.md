# Retire Legacy Skill Decontamination's writers, keep every reader

Status: accepted (2026-08-08, commit `7034bd4`)

Legacy Skill Decontamination was a one-time migration off `skill-audit`, a skill retired before
this crate existed. Its eligibility gate never computed provenance — it required the caller to
assert one of four `--basis` values about that retired skill's history — so nothing about it
outlives the skill it migrated away from. It ran in exactly one repository, where 12 skills started
it and 12 completed it, with no run left open. We removed the ability to write another
decontamination event, and deliberately kept every code path that *reads* one.

## Why

Shipping the workflow from a crate whose entire purpose is reuse across three repositories would
have handed two of them a dead command family. `mundifold` has no `skill-audit` history, so no
honest basis exists there, and a new repository's skills are newly written — which the workflow's
own boundary excludes outright ("no legacy audit history, no eligibility").

Keeping the readers is not sentiment about dead code. Twelve completed runs sit in playbench's
recorded evidence; that evidence is append-only and immutable; and the gate still projects from it.
Deleting `EventType::DecontaminationStarted` / `DecontaminationCompleted`, their gate derivation, or
their place in the published event schema would invalidate history that no release can regenerate
and no consumer can recover by pinning an older version.

## Considered options

**Remove everything, writers and readers.** Simplest tree, and the only option that leaves no
apparently-unreachable code. Rejected: it breaks playbench's twelve completed runs, and it is the
exact failure the consumer contract exists to prevent.

**Keep everything, writers included.** No decisions to explain, no asymmetry. Rejected: two of the
three consumers would receive a command family they can never honestly invoke, and its eligibility
gate would keep inviting callers to assert a basis that does not exist.

**Keep readers only.** Chosen. The write path is gone; the read path is permanent.

## Consequences

- A reader encountering `EventType::DecontaminationStarted` will find no writer, no
  `skills decontamination` command, and no operator package. That is intended, and this ADR is the
  answer to "why is this here?"
- The two event types are **permanent** members of the published `event.v1` schema. They are not
  candidates for cleanup in a future tidying pass.
- The generalized rule is recorded in
  [`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md):
  *a retired writer does not retire its readers.*
- `tests/lifecycle_compatibility.rs` asserts that the frozen corpora still cover all seven v1 event
  types, both decontamination types included, so a later change cannot quietly drop coverage.
- Two collapses followed and were taken rather than left dangling: `LifecycleWorkflow` had one
  variant left and became the one refusal it selected; `LandingOwnerKind` had one constructed
  variant and its noun is now inline.
- Restoring the workflow is explicitly a non-goal under
  [`../principles/mission-and-scope.md`](../principles/mission-and-scope.md). Reversing this would
  need a repository with real `skill-audit` history, which none of the three consumers has.
