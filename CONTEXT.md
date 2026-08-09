# Skill Evidence

The shared implementation of the skill-evidence lifecycle: recording factual receipts about
completed skill uses, deriving gate projections from them, and gating skill revision on what those
receipts actually support.

> **Authoritative terms** for this repository's governance, compatibility, and authority language
> are defined in `docs/principles/` (the constitutional corpus). This file defers to it and holds
> only the lifecycle vocabulary this repository's layer introduces on top; it never restates
> upstream terms. In particular, *conformance evidence*, *outcome evidence*, and the *three
> surfaces* are defined in `docs/principles/`, not here.

> **This file is upstream for the consuming repositories.** `playbench`, `mundifold`, and
> `what-we-bring-home` all run this lifecycle. When one of their glossaries needs a term below, it
> defers here rather than redefining it.

## Language

### Evidence

**Use record**:
One factual receipt about a completed use of a skill, clean or not, recorded without diagnosing or
changing the thing it observed. The unit of evidence in this repository.
_Avoid_: log entry, capture, report, observation

**Qualifying use**:
A use that cleared the bar for being recorded at all — the target governed real work and the run
reached a terminal outcome. A use that does not qualify produces no record, rather than a record
marked unqualified.
_Avoid_: valid use, countable use

**Outcome**:
The single classification every use record carries: `clean`, `friction`, `material_failure`, or
`severe_incident`. This is the canonical term. *Severity* names the ranking over those same four
values — a cluster's `max_severity` is the highest outcome in it — and is not a separate field.
_Avoid_: grade, result, status

**Incident**:
A use record whose outcome is not `clean`. Every incident carries a symptom key and the
expected/observed/consequence facts.
_Avoid_: failure, defect, bug

**Symptom key**:
A coarse clustering aid attached to an incident — `execution`, `output`, `triggering`, and five
others. It groups incidents; it does not name a cause and is never diagnostic.
_Avoid_: category, cause, tag, error type

**Run group**:
The dedup unit for use records. A retry or continuation of the same failed task belongs to the
same run group, so it produces one receipt, not several.
_Avoid_: attempt, batch, session group

**Evidence store**:
One target's accumulated evidence — its append-only event stream plus the projection derived from
it — living at `reports/skill-evidence/<skill-key>/` in a consumer's repository.
_Avoid_: log directory, evidence folder, database

### Identity

**Target**:
The skill a use record is about, or that a review would revise. Always another skill, never the
one doing the recording.
_Avoid_: subject, skill under test

**Target content hash**:
The identity of one exact version of a target's content. Evidence accumulates against a hash, so
editing a target starts a fresh accumulation rather than inheriting the old one.
_Avoid_: version, revision, fingerprint

**Skill key**:
The stable directory name a target's evidence store lives under. Normally the target's own name,
widened to distinguish it when a different skill of the same name exists elsewhere in the
repository — so two same-named skills never share one store.
_Avoid_: skill id, slug, store name

**Top-level session**:
The unit of independence. Two incidents count as independent only when they come from distinct
top-level sessions — this is what makes a recurrence claim mean anything.
_Avoid_: conversation, run, invocation

### Gate

**Gate projection**:
The derived view of what an evidence store currently supports, written alongside the stream it was
derived from. It is regenerable output, never a second source of truth, and it never edits the
stream it reads.
_Avoid_: gate status file, state, cache, summary

**Gate state**:
Where a target currently stands in the lifecycle — collecting evidence, eligible, serving a
cooldown, quarantined, under review, blocked, or closed. `schemas/skill-evidence/gate-status.v1.schema.json`
holds the exact roster; this glossary names the concept so the two cannot drift. A gate opening
reports what the evidence supports; it is not a finding that the skill is bad.
_Avoid_: status, phase, stage

**Candidate cluster**:
A symptom-keyed group of open incidents on the current target hash, carrying its independent-incident
count and its maximum outcome. Clusters are what reach a threshold; individual incidents are not.
_Avoid_: group, bucket, batch

**Authorization reason**:
Why a gate authorizes a workflow right now, recorded alongside which workflow is authorized.
Distinct from the gate state, which says only where the target stands, and distinct from the
re-entry basis, which is specifically about becoming actionable again after a completed review.
_Avoid_: trigger, cause, justification

**Re-entry basis**:
The recorded ground on which a target becomes actionable again after a completed review on the same
hash. Absent a basis, evidence that survived a review stays deferred.
_Avoid_: reopen reason, retry basis

### Review

**Review**:
One authorized attempt to revise a target, claimed against a specific gate and target hash. It runs
only when the freshly derived gate authorizes it.
_Avoid_: audit, revision, pass, evolution

**Candidate**:
A proposed replacement for the target, built inside a review. It has no authority until it survives
validation and is landed.
_Avoid_: draft, patch, fix, proposal

**Binding constraint**:
The condition without which the recorded failure does not occur — load, volume, context distance,
elapsed run length. Naming it is what decides whether any trial can test the incident at all.
_Avoid_: root cause, precondition, trigger

**Witness**:
The observable in a finished run's own output that shows whether that run actually expressed the
binding constraint. Fixed before any result exists; chosen afterwards, it is not evidence.
_Avoid_: check, signal, indicator, assertion

**Disposition**:
The single terminal state a review closes in. See `assets/skills/skill-evolution/` for the exact
set and their rules — this glossary names the concept, not the roster, so the two cannot drift.
_Avoid_: outcome, resolution, verdict, status

**Adjudicating disposition**:
A disposition that retires its triggering evidence from the active set, because the review actually
reached a conclusion about it. A *non-adjudicating* disposition closes the review while leaving the
trigger evidence active, because no conclusion was reached.
_Avoid_: terminal disposition, final disposition

**Instrument-limited disposition**:
A non-adjudicating disposition that nonetheless retires its covered evidence from the gate, because
the review established that this instrument cannot test that evidence. It adjudicates nothing — the
incidents stay open and unresolved in the ledger — but they stop clustering, so they can never again
reach a threshold the review already proved untestable. The one such disposition is
`blocked_no_valid_test`. See
[`docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`](docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md).
_Avoid_: dismissed, wontfix, closed-untestable

**Landing**:
The one authorized act that modifies a live target, carrying a baseline snapshot, a diff, and a
durable receipt. Landing is separate from acceptance: it applies a validated candidate, it does not
declare it good.
_Avoid_: merge, apply, commit, ship

**Refusal**:
A first-class terminal result — exit code `3` — where the command declines for want of authority
and writes nothing at all. A refusal is the system working, not an error.
_Avoid_: failure, rejection, error, denial

### Distribution

**Host**:
The consuming repository's identity as this crate sees it: its namespace, its operator-facing
command, its cargo package, and its own skills directory. One `Host` value is everything this crate
needs to speak as the repository that mounted it.
_Avoid_: config, client, integration, adapter

**Skill package**:
One installed Markdown operator — the thing an agent actually reads — written into a consumer's
`.claude/skills/`. Part of the contract, not incidental data: the library without the packages is
machinery with no operator. That directory also holds skills this crate does not install; sharing
it does not make one a skill package, and a package may not cite one.
_Avoid_: skill, asset, template, bundle

**Orphaned package**:
A skill package still present in a consumer that this crate no longer ships. Distinct from a skill
the consumer created under the same directory, which was never this crate's to remove.
_Avoid_: stale package, leftover, residue, dead skill

**Consumer**:
A repository that runs this lifecycle through a published version. A repository carrying a vendored
copy is not a consumer; it is a fork, and the contract does not reach it.
_Avoid_: client, dependent, downstream, user
