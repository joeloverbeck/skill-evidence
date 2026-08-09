# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

**This repo is single-context**: one `CONTEXT.md` and one `docs/adr/` at the root.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root — the domain glossary.
- **`docs/adr/`** — read ADRs that touch the area you're about to work in.

If any of these files don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront. The `/domain-modeling` skill (reached via `/grill-with-docs` and `/improve-codebase-architecture`) creates them lazily when terms or decisions actually get resolved.

## File structure

```
/
├── CONTEXT.md
├── docs/adr/
│   ├── 0001-....md
│   └── 0002-....md
└── src/
```

Both `CONTEXT.md` and `docs/adr/` exist. Nothing should look for a `CONTEXT-MAP.md`: this repo is single-context, and there is no per-context `src/<context>/docs/adr/`. If the crate ever splits into bounded contexts, add `CONTEXT-MAP.md` at the root then, pointing at one `CONTEXT.md` per context.

`CONTEXT.md` opens with a deference rule: governance, compatibility, and authority language belongs to [`../principles/`](../principles/), and the glossary holds only the lifecycle vocabulary layered on top. It is also upstream for `playbench`, `mundifold`, and `what-we-bring-home`, whose glossaries defer here for lifecycle terms rather than redefining them.

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in `CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal — either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/domain-modeling`).

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 (event-sourced orders) — but worth reopening because…_

## Amending an accepted ADR: this repo's revision identifier

`/domain-modeling`'s ADR-format reference prescribes an `Amended: <date> (<revision>) — <what changed>`
line, and a companion note "wherever this repo already records the revision (the changelog or
iteration report that drove it)." Neither clause lands here as written, and that gap is what this
section closes — the skill arrived by wholesale copy and will arrive that way again, so recording the
localization here rather than editing the skill is the point. Editing it would also change
`domain-modeling`'s target content hash and discard the evidence accumulated against the current one.

- **The revision identifier is the GitHub issue that drove the amendment.** This repo has no changelog,
  no iteration report, and no revision number; its ADRs are anchored by acceptance date and issue.
  The established line form, used throughout
  [`../adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`](../adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md), is:

  > `Amended: <date>, GitHub [#N](https://github.com/joeloverbeck/skill-evidence/issues/N) — <what changed>.`

  Do not invent a version or revision number to fill the reference's `(<revision>)` slot.
- **The companion note is vacuous here and needs no substitute.** It is conditional on the repo
  *already* recording the revision somewhere. This one does not, and
  [`../principles/consumer-contract.md`](../principles/consumer-contract.md) lists a maintained
  changelog among the obligations this repository does not accept — a courtesy, never a duty — so
  there is no destination to write to and none should be created for the sake of the line.

Amending in place still applies only when the decision stands. An actual reversal or replacement gets
a new ADR that supersedes the old one, per the reference.
