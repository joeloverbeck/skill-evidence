# Mission and Scope

Status: accepted constitutional principle

## Mission

This repository is the shared implementation of the skill-evidence lifecycle: recording factual
receipts about completed skill uses, deriving gate projections from those receipts, and gating
skill revision on what the receipts actually support.

It exists because the same implementation was hand-copied between repositories. It was
extracted after that pressure was real and repeated, not forecast — the first copy was written
in one repository, the second was pasted into another, and a third repository was about to need
it. Extraction is therefore justified by observed repetition, which is the only justification
this repository's existence has ever had or needs.

## The one primary outcome, stated as a consumer effect

**A repository that wants the skill-evidence lifecycle can run it — machinery, command surface,
and the skill packages an agent reads — without hand-copying any of it, and can keep running it
across upgrades without losing or invalidating evidence it has already recorded.**

That outcome is stated so that materially different mechanisms could satisfy it. A Rust crate
with a mountable clap surface and installed Markdown packages is the current mechanism. It is
not constitutive of the mission. If a consumer's real use showed that a different distribution
shape served the same effect better, the crate is replaceable; the effect is not.

## This repository has no value stream of its own

Every output here is enabling work. Nothing this repository produces is a primary artifact for
anyone. The primary artifacts are downstream: the games, worlds, and works the consuming
repositories exist to make.

Three consequences follow, and they are the reason this document exists:

1. **Priority comes from a consumer's bottleneck, never from this repository's own tidiness.**
   Work here is warranted when it is the shortest evidence-bearing path through a bottleneck a
   consumer actually has. An improvement that is elegant, obviously correct, partly implemented,
   or satisfying to engineer gains no priority from those properties.

2. **This repository's issue tracker is not a value stream.** An open issue count, a backlog, a
   closure history, and a tidy board are not progress and create no obligation to continue. An
   issue records that something was observed; it does not authorize the work.

3. **A defect here is only a defect if a consumer can feel it.** An internal inconsistency no
   consumer's real use can reach may be recorded and left alone.

## Where authority sits

The repository owner is the semantic authority. Agent-authored analysis, passing tests, merged
code, closed issues, and completion reports are proposals and evidence — never acceptance.

This repository holds no authority over what a consumer does with the lifecycle. It does not
decide which skills a consumer gates, when a consumer reviews one, or whether a consumer's
recorded evidence is any good. It supplies the machinery and the contract; the consumer's own
constitution governs its use.

The direction is one-way and load-bearing. A consumer's need is evidence that can motivate a
bounded change here. It does not make that consumer's roadmap this repository's outcome, and it
does not let a downstream deadline authorize a change that breaks the other consumers.

## Roles of significant pre-existing artifacts

- **The four installed skill packages** (`skill-evidence-capture`, `skill-evolution`,
  `skill-evolution-status`, `method-gap-research-status`) are part of the contract, not
  incidental data. A consumer that receives the library without the packages has machinery and
  no operator for it. They ship inside the published crate for that reason.
- **The retained decontamination readers.** `EventType::DecontaminationStarted` and
  `DecontaminationCompleted`, their gate derivation, and their place in the published event
  schema exist because completed runs sit in a consumer's recorded evidence and that evidence is
  immutable. The workflow that *wrote* them is retired and will not return. The readers are not a
  workflow, not a backlog, and not a migration duty — they are the shape of an obligation this
  repository has to data it already caused to exist. See
  [`evidence-substrate-integrity.md`](evidence-substrate-integrity.md).
- **The frozen fixture corpora** under `fixtures/skill-evidence/*-v1/` are regression assets that
  stand in for consumer history. They are evidence, not examples, and they are not edited to make
  a change pass.

## Major non-goals

Unless explicitly reprioritized, this repository does not own:

- any consumer's domain, workflow, or creative judgment;
- a general agent-skill framework, plugin system, or lifecycle platform for arbitrary workflows;
- generalization toward a hypothetical fourth consumer, or any abstraction whose only pressure is
  predicted rather than observed;
- migration of a consumer off a vendored copy, except where the consumer asks for it;
- deciding whether a consumer's recorded evidence is sufficient, honest, or well-graded;
- a hosted service, dashboard, aggregation across repositories, or multi-user anything;
- restoring retired workflows, or preserving rejected architecture for possible future usefulness.

## Deployment assumptions

The intended maintainer is the repository owner. The repository is public on GitHub and published
to crates.io so that private consumer repositories can resolve it as an ordinary dependency
without credential plumbing.

Publication is distribution, not a claim that an ecosystem exists. As of adoption there are no
outside users, forks, or dependents. The obligations this repository accepts toward strangers are
stated in [`consumer-contract.md`](consumer-contract.md) and are deliberately modest; they are not
expanded on the strength of a hypothetical audience.
