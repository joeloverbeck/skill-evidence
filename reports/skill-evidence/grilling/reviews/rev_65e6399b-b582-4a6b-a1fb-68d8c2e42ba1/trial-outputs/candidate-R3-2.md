Read: `src/main.rs`, `src/store.rs`, `docs/adr/0001-storage-format.md`, `docs/status.md`, `docs/release-process.md`, `Cargo.toml`, `CONTRIBUTING.md`, `CHANGELOG.md`, `scripts/preflight.sh`, `scripts/publish.sh`, `.claude/settings.local.json`.

## Premise check on my own prior turn

**Verified.** JSONL storage with a `written` field (`src/store.rs:7`, ADR 0001:10). On-disk names are internal and a machine-readable output format is a separate compatibility surface needing its own ADR (ADR 0001:23–25). `serde_json` is already a direct dependency (`Cargo.toml:9`), so nothing here trips ADR 0001's "adding a runtime dependency requires a superseding ADR" (0001:21). `docs/status.md:5` says notekeeper is feature-frozen *apart from output-format work* — this is the sanctioned exception, so the freeze doesn't block it.

**Contradicted — my own preview.** My Option A example used `"created_at"`. ADR 0001:13–14 explicitly chose `written` **over** `created_at`, reasoning that a note can be edited and `written` records the original writing. The ADR governs storage, not output, so it doesn't formally bind us — but I silently reintroduced a name the repo had already reasoned against, and the semantic argument carries over unchanged. That's a live fork, not a settled detail. You picked the *shape* (one record per line); the field names my example carried are not ratified.

**Unavailable.** `cargo` is denied in this environment, so `fmt`/`clippy`/`build`/`test` cannot be run here. Separately, `src/store.rs` is a stub — `append`/`load_recent`/`search` return empty and `now_rfc3339` returns a hardcoded constant — so there is no runnable behavior to verify output against either way. Consequence: this grill can ratify a scope and I can write the ADR/docs/code, but I cannot satisfy the lint/typecheck/test gate here. Say the word and I'll note it as a handoff item rather than pretending it passed.

## Branches I expect to resolve

1. **Record shape** — timestamp key, and whether `schema` rides on every line.
2. **Command coverage** — which of `add` / `list` / `search` get `--json`.
3. **Error and exit behavior** under `--json`.
4. *(derived, veto-able)* **Flag placement** — per-subcommand, matching the existing `--tag`/`--limit` pattern at `src/main.rs:14-16`.
5. **Deliverable set** — ADR 0002 + README line + CHANGELOG entry + code. Releasing is out.

## Question 1 of 3 — the per-line record

Option A gives up the envelope, so every key is permanent from the first shipped release. Concretely:

**R1 — keep `schema`, use `written`** *(recommended)*
```json
{"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"written":"2026-06-01T00:00:00Z"}
```
One vocabulary across storage, ADR, and output; a reader of `notes.jsonl` and a reader of `--json` learn one name. Keeps a version handle for the day the shape changes. **Costs:** ~28 redundant bytes per line, and `written` is less familiar to consumers than `created_at`. **Rejected if** you expect third-party tooling that keys on conventional names more than on your own docs.

**R2 — keep `schema`, use `created_at`** (exactly what my earlier example showed)
```json
{"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"created_at":"2026-06-01T00:00:00Z"}
```
Familiar to anything that has parsed an API before. **Costs:** two names for one concept, permanently; and it asserts creation semantics that ADR 0001 deliberately avoided because notes can be edited. **Rejected if** you ever add an edit command — the name starts lying and you can't change it without another ADR.

**R3 — drop `schema`, use `written`**
```json
{"body":"buy milk","tags":["errand"],"written":"2026-06-01T00:00:00Z"}
```
Leanest line; `jq` consumers never see a key they don't want. **Costs:** no version marker anywhere, so a future shape change is undetectable by consumers — with no envelope, `schema` is the only place a version can live. **Rejected if** you think the shape will move at all.

Which one — R1, R2, or R3? (Mixing dimensions differently is fine; say so and I'll record it.)