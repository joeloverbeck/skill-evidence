# skill-evidence

## Before proposing work here

This crate is depended on by `playbench`, `mundifold`, and `what-we-bring-home`, which between them hold over a thousand append-only recorded events that no release can regenerate. Read [`docs/principles/`](docs/principles/) before proposing anything that changes the public API, the installed skill packages, the published schemas, or the shape of a recorded event.

Two things that catch people out:

- **Cargo SemVer protects the Rust API and nothing else.** Adding an optional field to a recorded event is a compatible change by every Cargo rule and can still invalidate a consumer's history. See [`docs/principles/consumer-contract.md`](docs/principles/consumer-contract.md).
- **Install never removes.** Retiring or renaming an installed package leaves it in every consumer
  until the consumer deliberately runs `skills evidence withdraw`, or follows the release note's
  exact manual fallback when pinned below the withdrawal version. See
  [`docs/releasing.md`](docs/releasing.md).

This repository has no value stream of its own — everything here is enabling work for a consumer. Its own issue count is not progress.

## Agent skills

### Issue tracker

Issues live as GitHub issues in `joeloverbeck/skill-evidence`, driven by the `gh` CLI; external PRs are also a triage surface. See `docs/agents/issue-tracker.md`.

### Triage labels

The five canonical strings, unmodified: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `CONTEXT.md` and `docs/adr/` at the repo root. See `docs/agents/domain.md`.
