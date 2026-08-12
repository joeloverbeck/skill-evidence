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

**Deviation**:
One observable way a run departed from what the target said, and the unit a use record records.
Two deviations in one run are distinct when their expected-and-observed facts are — a different
instruction was not followed, or a different artifact broke a different contract — and the same
deviation surfacing twice in one run is one deviation. Distinctness is read off what the session
shows; it is never a claim that two deviations have different causes.
_Avoid_: issue, problem, error, violation

**Further incident**:
A second or later incident recorded from one run, declared as such and sharing that run's run
group and task fingerprint. Each is an ordinary use record with its own event id, so a review can
name one without reaching the others, and between them they are one qualifying use. Independence
is counted per candidate cluster, so siblings landing in the same cluster contribute one
independent incident between them; siblings carrying different symptom keys land in different
clusters and contribute one to each. Recording a run's deviations apart describes what the run
did and claims nothing about their causes.

Two such records are siblings *of each other*. That is a different relation from the one
[`docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`](docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md)
calls a sibling — "a friction sibling cannot lower a `material_recurrence` bar" means a
cluster-mate, which normally comes from a *different* run. Neither usage is wrong and the ADR
is unaffected; say which relation is meant whenever both are in play.
_Avoid_: sub-incident, related incident, duplicate receipt

**Symptom key**:
A coarse clustering aid attached to an incident — `execution`, `output`, `triggering`, and five
others. It groups incidents; it does not name a cause and is never diagnostic.
_Avoid_: category, cause, tag, error type

**Run group**:
The one run a use record belongs to, and the unit the use denominator counts. Within one top-level
session, a retry or continuation of the same failed task belongs to the same run group and adds no
further use; a different top-level session cannot add another receipt for that run while the target
content hash is unchanged. A run that deviated in several observable ways records one incident per
deviation, so a run group can hold several records while remaining one use.
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
One value, carried under two field names — `authorizing_rule` on the recorded event and
`authorization_reason` on the projection — so name the concept this way in prose whichever field
is in hand.
_Avoid_: trigger, cause, justification, authorizing rule

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
elapsed run length. Naming it is what decides whether any trial can test the incident at all. A
constraint is **established** when a recorded field supports it, **unestablished** when the record
does not establish it, and **refuted** for a trigger when that trigger's recorded condition places
the failure where the proposed constraint says it should not occur. Unestablished and refuted
constraints keep a trial slot; a field name alone establishes nothing.
_Avoid_: root cause, precondition, trigger

**Witness**:
The observable in a finished run's own output that shows whether that run actually expressed the
binding constraint. Fixed before any result exists; chosen afterwards, it is not evidence. An
observable that no finished run could make read unexpressed does not show *whether*, and is not a
witness. Nor is an observable that a compliant run which finds nothing would omit: if that run
would not still emit it, it is not a witness.
_Avoid_: check, signal, indicator, assertion

**Constraint provenance citation**:
A generated whole-field copy from a covered immutable event that puts the recorded ground for a
binding constraint in the close itself. The operator supplies a constraint label, event ID, and
field pointer; the command supplies the field value. It proves which record was cited, not that the
record entails the constraint.
_Avoid_: excerpt, field class, authored note, proof

**Disposition**:
The single terminal state a review closes in. See `assets/skills/skill-evolution/` for the exact
set and their rules — this glossary names the concept, not the roster, so the two cannot drift.
_Avoid_: outcome, resolution, verdict, status

**Adjudicating disposition**:
A disposition that retires from the active set the evidence it reached a conclusion about — its
coverage list, less any untestable coverage the close named. A *non-adjudicating* disposition closes
the review while leaving the trigger evidence active, because no conclusion was reached about any of
it.
_Avoid_: terminal disposition, final disposition

**Coverage list**:
The event IDs a review records as covered when it closes — for a newly claimed review, the trigger
list re-derived from its authorization reason and frozen when the review is claimed, plus any events
an adjudicating close explicitly names. Historical claims retain the trigger list they recorded; no
stream is reinterpreted. It is what the close writes down, which is not the same as what the close
costs, and not the same as what it concluded.
_Avoid_: adjudicated set, trigger list, covered events

**Untestable coverage**:
The events an adjudicating close names within its coverage list as ones this review could not decide —
either because no trial could express the mechanism's binding constraint, or because the acceptance
gate grades outcome and the event's evidence bears no outcome claim. They stop being adjudicated, and
retire from the gate unless they still drive it on their own: what an instrument-limited disposition
does for a whole review, done one event at a time, for the ordinary case where one review's mechanisms
reached different readings. Naming an event here narrows what the close concluded; it never narrows
what the close covered, and it says nothing about whether the mechanism reproduced.
_Avoid_: partial adjudication, excluded triggers, untested set, uncovered evidence

**External owner**:
The party an `outside_target` conclusion names as responsible for a trigger it covered — a kind drawn
from the shipped ownership taxonomy, plus a stable reference — recorded for each concluded event
rather than once per close, so a review whose triggers are owned differently still has an honest
close. Naming one records a conclusion and reports it; it moves no evidence, and the named party's
gate still opens only on that party's own recorded uses. See
[`docs/adr/0007-an-outside-target-conclusion-reports-its-owner-and-routes-nothing.md`](docs/adr/0007-an-outside-target-conclusion-reports-its-owner-and-routes-nothing.md).
_Avoid_: owning skill, responsible party, culprit, routed owner

**Instrument-limited disposition**:
A non-adjudicating disposition that nonetheless retires evidence from the gate, because the review
established that this instrument cannot test it. It adjudicates nothing — the incidents stay open and
unresolved in the ledger — but they stop clustering, so they can never again reach a threshold the
review already proved untestable. The one such disposition is `blocked_no_valid_test`. Untestable
coverage does the same job for single events inside an adjudicating close — it stops them clustering
without adjudicating them — but on the narrower claim that this review could not decide them, which
is not a claim about what any instrument can test. See
[`docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`](docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md).
_Avoid_: dismissed, wontfix, closed-untestable

**Retirement reach**:
The open incidents one close moves out of the gate, at whichever scope that close retires. An
instrument-limited close reaches the ones its review's own authorization reason still names when
re-evaluated at the close: wider than the coverage list only by incidents the reason names that
arrived after the claim and before the close, and never wider than what opened the gate. An
adjudicating close reaches the untestable coverage it named, less anything the derivation declined to
retire: always inside its coverage list, and never a contemporaneous severe incident. Distinct from
the gate projection's standing retired set — a live per-hash view that later closes can grow or
shrink. A close reports its own reach; the projection reports the standing set.
_Avoid_: retired set, instrument-limited set, covered evidence

**Reach bound**:
The widest set a prospective instrument-limited close could retire, as the gate projection shows it
before the close: every open incident the review's authorization reason would name there and then. It
bounds the reach from above and never from below — a contemporaneous severe incident sits in that
cluster and is never retired — so it is what a reviewer must be able to vouch for before closing, not
a prediction of what the close will name.
_Avoid_: predicted reach, dry-run reach, projected reach

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

**Operating package**:
The installed skill package whose instructions an agent is following while it drives a lifecycle
operation — always a different skill from the target the operation is about. The crate never reads
its prose; it hashes it, and compares it against what this version ships.
_Avoid_: operator skill, driving skill, workflow package

**Superseded operating package**:
An operating package whose installed bytes differ from the ones this crate renders for the
consumer's `Host`. It says the consumer upgraded one surface and not the other; it does not say
which instruction changed, because the crate cannot read the difference — only detect it.
_Avoid_: stale package, incompatible package, outdated skill

**Withdrawal**:
The deliberate removal of every retired package the crate can still prove it shipped, including
its discovery link and empty package directories. It refuses before the first removal when a
shipped file differs, and never treats a foreign path as its own authority.
_Avoid_: uninstall, prune, cleanup

**Consumer**:
A repository that runs this lifecycle through a published version. A repository carrying a vendored
copy is not a consumer; it is a fork, and the contract does not reach it.
_Avoid_: client, dependent, downstream, user
