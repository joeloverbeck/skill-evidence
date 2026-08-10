# Repository Constitution

Status: adopted

This directory defines the durable principles of `skill-evidence`. It exists because this
repository is depended upon by others that record evidence they cannot regenerate, and because a
shared layer with no constitution drifts toward serving its own tidiness rather than its
consumers.

The governing authority pattern is:

> Human intent → constrained proposal → applicable mechanical or semantic checks → inspectable
> consequences → explicit human acceptance → authoritative state

Checks are proportionate to the claim and its risk. No implementation, skill, pull request,
issue, test, generated report, or conversational inference may bypass an applicable check or
manufacture the human acceptance an authority transition requires.

## Constitutional documents

- [`mission-and-scope.md`](mission-and-scope.md) — why this repository exists, its one primary
  outcome stated as a consumer effect, why it has no value stream of its own, the roles of
  significant pre-existing artifacts, and its non-goals.
- [`consumer-contract.md`](consumer-contract.md) — the three surfaces a change can reach a
  consumer through, how each is versioned, the forward-only evidence rule, the installer's
  missing removal path, the instructions-not-executables rule for shipped packages, and the
  one-way direction of upgrade authority.
- [`evidence-substrate-integrity.md`](evidence-substrate-integrity.md) — what may and may not
  happen to recorded evidence, why structural validity is not semantic acceptance, why evidence
  authorizes only the claim it bears, and the discipline required of side effects on a consumer's
  files.
- [`inherited-prohibitions.md`](inherited-prohibitions.md) — the failure modes future work must
  not gradually normalize.

The operational counterpart lives in [`../releasing.md`](../releasing.md): how to actually cut a
release and bring each consumer forward. That document is procedure, not law — it changes as the
mechanics change, and it cannot amend anything here.

Two neighbours, neither of them constitutional. [`../../CONTEXT.md`](../../CONTEXT.md) is the
lifecycle glossary; it defers to these documents for authority language and holds only the
vocabulary layered on top. [`../adr/`](../adr/) records accepted implementation decisions; read
the directory for the current set. Being linked here promotes neither, and adopting this set
adopts neither.

## Scope and precedence

These documents govern this repository's behavior and architecture. They govern nothing about
what a consumer does with the lifecycle; each consumer's own constitution governs its own use.

Within their proper scopes:

1. An explicit decision by the repository owner supplies semantic authority.
2. This Constitution constrains how this repository may change, publish, and affect consumers.
3. The consumer contract binds every release, including one made under downstream pressure.
4. An issue or PRD authorizes bounded implementation work. It cannot change priority by
   implication, grant acceptance, or override a principle.
5. A skill package owns one capability's procedure and result. It cannot expand its own authority
   from its own instructions.
6. Code enforces adopted behavior. Passing code cannot manufacture adoption.

When two authorities appear to conflict, first ask whether they govern different scopes. If the
conflict is real, stop at the earliest affected boundary and request an exact decision. Do not
resolve it by file age, polish, merge status, or presumed intent.

## Alignment without certification paperwork

Compatibility with these documents is demonstrated through the work and its consequences, never
through a compliance form. An agent proposing work should be able to answer, where material:

- Which consumer's bottleneck does this serve?
- Which of the three surfaces does it touch, and what does a consumer have to do about it?
- Could it invalidate evidence already recorded anywhere?
- What is the cheapest evidence that would falsify its key assumption?
- What does it displace?

Do not copy these into issues. Ask them only where the answers could change scope, order, or
architecture.

## Constitutional change

An issue, implementation, merge, test result, or agent-authored report cannot amend these
principles.

A proposed change must:

1. identify the exact clause and the real pressure it cannot responsibly accommodate;
2. state what current work or protection it would displace;
3. offer the narrowest viable amendment and its strongest serious alternative;
4. expose the consequences through the cheapest honest experiment where the claim is testable;
5. remain a proposal until the owner explicitly adopts the exact amendment and scope; and
6. remove or independently rejustify dependent work if it is rejected.

A narrow one-time exception may be recorded without amending anything, naming its scope, expiry
condition, and prohibited dependency fan-out. Repetition is evidence to reconsider the principle,
not automatic amendment.

## Adoption

Adopted 2026-08-08 by the repository owner, at the exact text of this revision, as a set, against
the repository as of `df86c2f`.

Amended 2026-08-09 by the repository owner, at the exact text and placement of §*Shipped packages
carry instructions, not executables* in [`consumer-contract.md`](consumer-contract.md), against
the repository as of `82f2ce4`. No other clause changed; the constitutional-documents list above
names the new rule.

Amended 2026-08-09 by the repository owner, at the exact text in commit `8a1dc64` changing
§*The installer never removes* and its cross-reference in §*Shipped packages carry instructions,
not executables* in [`consumer-contract.md`](consumer-contract.md), against the repository as of
`8a1dc64`. No other clause changed. The amendment separates install from explicit withdrawal,
permanently bounds withdrawal to the retirement set, and preserves the breaking-change status of
package retirement and rename.

Amended 2026-08-10 by the repository owner, at the exact text and placement of the added paragraph
in the frozen-fixture-corpora bullet of §*Roles of significant pre-existing artifacts* in
[`mission-and-scope.md`](mission-and-scope.md), against the repository as of `9da3843`. No other
clause changed; the no-editing rule and *evidence, not examples* are unmoved. The amendment admits
a corpus frozen for a stream shape this repository has only just begun to write, before any
consumer holds one, and binds such a corpus to disclose in its README that it guards forward
rather than attesting to the change that introduced it. Its pressure was GitHub
[#27](https://github.com/joeloverbeck/skill-evidence/issues/27), whose new recorded shape — two
use records sharing one run group — no historical corpus could contain.

Adoption of these documents does not by itself adopt any future schema version, installer redesign,
distribution change, or resolution of an open issue. In particular it does not settle the open
question in `joeloverbeck/skill-evidence#2`; it supplies the clause that question is decided
against, and the decision itself remains the owner's. Later change follows the amendment process
above.
