# Fail closed at claim when the installed operating package is superseded

Status: accepted (2026-08-11, GitHub [#31](https://github.com/joeloverbeck/skill-evidence/issues/31))

A consumer can upgrade the compiled crate while leaving an older Skill Evolution package installed.
The lifecycle will detect that by rendering the shipped Skill Evolution package for the consumer's
`Host` and comparing it byte-for-byte against the installed copy — the comparison `install` already
performs, reused rather than reinvented.

What the detection does depends on which surface is running, because the surfaces are not alike:

| Surface | Behavior |
|---|---|
| `skills evolution-status`, `skills evolution preflight` | Report the mismatch and name the differing files. Neither refuses. |
| `skills evolution claim` | Refuse (exit `3`) before any event or projection write. |
| `skills evolution record-validation`, `land`, `close` | Proceed, recording the computed mismatch on the event. |

Every lifecycle event records `operating_skill_hash` unconditionally, and alongside it a computed
`operating_package_matches_shipped`. Both are computed by the crate. Neither is asserted by the
caller, and no flag overrides the claim refusal.

## Why

`claim` is the only write that starts a review and spends gate authority. Refusing there costs a
consumer nothing they cannot recover: no review exists yet, and the remedy — `skills evidence
install --force`, then claim — restores the package to a state the crate can vouch for. The harm
this ADR exists to prevent is precisely a claim under superseded rules. In the run that produced
[#31](https://github.com/joeloverbeck/skill-evidence/issues/31), a `ten_use_unresolved`
authorization backed by 112 qualifying uses was consumed and its trigger retired from the gate,
under instructions the consumer had not upgraded. That authorization cannot be returned.

The continuation terminals are a different case, and the difference is constitutional.
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
forbids *"a projection whose only escape is an event that no honest process would produce"* and
requires that a gate reaching a state with no honest exit be given one. A consumer who claims under
package A, upgrades the crate, and then meets a refusal at `close` has exactly one route out:
`install --force`, after which the close records `operating_skill_hash` for package B on a review
conducted under A. The record would assert something false, permanently, in the one surface
[`../principles/consumer-contract.md`](../principles/consumer-contract.md) calls unrecoverable. A
started review must be able to terminate honestly, so the continuation terminals proceed and record
what was actually operating.

Three measurements set the cost of getting this wrong in the other direction. The shipped Skill
Evolution package has taken 9 distinct forms across 11 published versions, and 14 across the
repository's own history in three days; a gate that blocked every terminal would block a
release-tracking consumer at 8 of the 10 version transitions. Two of the three consumers
(`playbench`, `mundifold`) are drifted today and will be refused at `claim` the moment this lands.
And the lifecycle has no abandon path — an active review holds the gate — so a refusal at `close`
has no exit that is both available and honest.

## Considered options

**Capability or protocol handshake.** Rejected, and the fork it implies does not survive contact
with the code. The compiled lifecycle touches the installed package in exactly two places: it
canonicalizes the path to refuse a self-target, and it hashes the directory. It never parses a byte
of the prose. The capabilities named in the originating incident — instructing
`--record-operating-skill-hash`, ordering report before close — are agent behaviors, not crate
inputs, and both are now enforced by compiled code regardless of what the prose says. A handshake
could therefore only be the package declaring its own provenance: a self-asserted, forgeable version
string, carrying a permanent capability registry, and inheriting the same bootstrap gap byte
equality has — no package this crate has ever shipped carries such a declaration, so every installed
copy in existence fails the handshake exactly as it fails the byte comparison.

**Byte equality refusing at every terminal.** Rejected. It is the strongest reading of "fail
closed" and the simplest surface, but it is the option that manufactures the dishonest event
described above. Giving it an honest exit would mean building an abandon command — new authority
over an active review, to serve a case the asymmetric split resolves without it.

**Record the mismatch and never refuse.** Rejected. It leaves
`assets/skills/skill-evolution-status/SKILL.md` promising that the compiled command "fails safely
when that contract is absent or incompatible" while it does no such thing, and unconditional
identity does not undo a spent authorization. Provenance recorded after the gate has moved
describes the loss; it does not prevent it.

**Refusal everywhere with an operator override flag.** Rejected on record shape.
[`../principles/evidence-substrate-integrity.md`](../principles/evidence-substrate-integrity.md)
requires that provenance be computed, not asserted by the caller. An
`--acknowledge-superseded-operating-package` flag would make the operator assert a fact the crate
can compute by comparing bytes, and would hand an agent mid-run a flag whose reflexive use is
indistinguishable from its considered use.

**Asymmetric gating on the surface's role.** Chosen. It refuses at the one write where refusal
costs nothing and prevents everything, reports on the reads so an agent learns before doing
semantic work, and lets a started review end truthfully. It adds no flag, no declaration format,
and no new authority over an active review, and the comparison it needs already exists in
`assets::install`.

## Consequences

- **`playbench` and `mundifold` cannot claim a Skill Evolution review until they reinstall.** Both
  are drifted today. Every divergent file in both is byte-identical to a version this crate
  actually shipped, so `skills evidence install --force` is the whole remedy. Reviews already
  claimed there remain closable.
- **The release carrying this needs a release note naming both surfaces** — the compiled refusal
  and the installed package whose staleness triggers it — per
  [`../releasing.md`](../releasing.md).
- **`operating_package_matches_shipped` is additive and optional.** Absent means the event was
  written before the crate computed it, not `false`. The frozen corpora under
  `fixtures/skill-evidence/` stay valid unedited; that is the proof, not a formality.
- **The comparison covers the Skill Evolution package only**, not the whole installed asset set. The
  gate names the package whose rules govern the operation being performed. `skills evidence record`
  is untouched: it belongs to a different package and a different workflow, and blocking a
  consumer's incident capture would be a worse harm than the one being closed.
- **A consumer's stale installed `schemas/skill-evidence/event.v1.schema.json` is not covered.** The
  crate writes from code, not from the installed schema. `playbench` and `mundifold` both carry a
  copy predating the `review_started` conditional block, which their own tooling would use to reject
  events the current writer emits. That is a real inconsistency and a separate issue.
- **What could reopen this.** The refusal has no override because localization of an installed
  package has never been observed: across three consumers and all thirteen shipped assets, every
  divergent file matches an exact version this crate shipped, and not one carries a local edit. ADR
  [0004](0004-method-gap-research-status-severs-its-commission-dependency.md)'s localization
  precedent concerns the commission skill, which is deliberately not an installed asset. A consumer
  that deliberately localizes an installed Skill Evolution package would be refused at `claim` with
  no route but to discard its localization, and that is the evidence that reopens this decision.
