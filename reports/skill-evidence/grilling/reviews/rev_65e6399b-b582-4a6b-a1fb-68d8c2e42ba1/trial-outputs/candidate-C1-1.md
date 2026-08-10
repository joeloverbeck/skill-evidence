## Verdict — 1 of 4 findings survives

Verified every claim by direct read against all 12 files in the repo (single-threaded, no delegation; full coverage — nothing unavailable or pending). No code was changed.

| # | Disposition | Basis |
|---|---|---|
| F1 | **Reject** — premise contradicted | `CONTRIBUTING.md:7-8` |
| F2 | **Reject** — stated backwards | `src/store.rs:7`, `docs/adr/0001-storage-format.md:13-14` |
| F3 | **Reject** — premise contradicted | `docs/adr/0001-storage-format.md:18-20` |
| F4 | **Adopt** — accurate, highest stakes | `docs/release-process.md:10-12`, `scripts/publish.sh` |

**F1 — Reject.** `CONTRIBUTING.md:7-8` states the changelog is maintained by hand with an entry under `## Unreleased` "at the time of the change, not at release time." Step 1 of the release process only *moves* those existing entries into a dated section; nothing is generated from commit subjects. `CHANGELOG.md:3-6` shows the Unreleased block already populated. The recommendation is the repo's current documented and practiced policy.

**F2 — Reject.** The field is `written` (`src/store.rs:7,12`), not `created_at`. The finding has the direction inverted: `created_at` is precisely what ADR 0001 rejected, with reasoning — a note can be edited, so `written` records the original writing (`docs/adr/0001-storage-format.md:13-14`). It also conflates a field *name* with RFC 3339, which governs the value format; `written` already holds RFC 3339 UTC. Acting on this would rename toward the rejected name and require a superseding ADR (line 21).

**F3 — Reject.** ADR 0001 lines 18-20 address this directly: `serde_json::to_string_pretty` covers pretty-printing, and `serde_json` is already a direct dependency (`Cargo.toml:10`). No new crate. Adding one would need a superseding ADR (line 21). There *is* a real adjacent constraint the finding misses: a `--pretty` machine-readable mode is a new compatibility surface needing its own ADR (ADR lines 23-25; `CONTRIBUTING.md:4-5`) plus a README command-list line (`CONTRIBUTING.md:3`). That is not what F3 says, and its recommendation is wrong as written.

**F4 — Adopt.** Correct and material. Step 5 invokes `./scripts/publish.sh` immediately after preflight; the script uploads with no prompt, and `docs/release-process.md:11-12` states a published version can never be unpublished, edited, or reused. Preflight is an automated gate *before* the decision point — nothing halts between `PREFLIGHT OK` and an irreversible upload. An explicit human confirmation before the upload is warranted.

**Meta:** the three rejected findings all contradict ADR 0001 or CONTRIBUTING.md — the reviewer appears not to have read either. Treat the report's remaining authority as low; F4 stands on its own re-verified merits, not on the report's credibility.

No completion contract applies (no code touched; `cargo fmt`/`clippy` are PR-time gates).

Say the word if you want F4 implemented, or the ADR-gated `--pretty` path scoped out.