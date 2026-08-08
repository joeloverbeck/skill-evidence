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
