I read the README, ADR 0001, CONTRIBUTING, CHANGELOG, `src/main.rs`, `src/store.rs`, `Cargo.toml`, `docs/status.md`, `docs/release-process.md`, and both scripts. Before the next fork, two things from my previous turn need correcting, because one of them is inside the option you just picked.

## Corrections to my Option A

**The field name in my sample was wrong.** I showed `"created_at"`. ADR 0001 (Accepted, 2026-03-08) names the timestamp `written` and states the name "was chosen over `created_at` because a note can be edited, and `written` records the original writing." My recommended sample renamed the field to the exact name the ADR rejected. That is now an open fork, not a detail.

**"Streams line by line" was half true.** It holds for your consumers — `head`, `grep`, and `while read` each work on whole records. It does not hold inside notekeeper: `store::load_recent` and `store::search` both return `Vec<Note>` (`src/store.rs:23-25`), so the process materializes every result before printing. The choice of A over B still stands on the consumer-side benefit; I just overstated B's cost.

## Settled by the repo — no question needed

| # | Decision | Evidence |
|---|---|---|
| E1 | A new ADR (0002) ships with this change | ADR 0001 §Consequences: an output format "would need its own ADR"; CONTRIBUTING line 4-5 |
| E2 | `## Unreleased` changelog entry written at the time of the change | CONTRIBUTING line 7-8 |
| E3 | README command list gains the flag; the "There is no machine-readable output mode" sentence (`README.md:11-12`) gets rewritten | CONTRIBUTING line 3 |
| E4 | No new dependency — `serde_json` is already a direct dep and covers this | `Cargo.toml:10`; ADR 0001 forbids new runtime deps without a superseding ADR |
| E5 | `cargo fmt` + `cargo clippy` before the PR | CONTRIBUTING line 6 |
| E6 | Note bodies containing newlines stay one record per line, since `serde_json` escapes them as `\n` | serde_json behavior; this is the concrete reason A beats C |
| E7 | Releasing/publishing is **out of scope** for this work | `scripts/publish.sh:4` appends permanently and `docs/release-process.md:11-12` says a published version can never be unpublished — I will not run it |

## Branches I expect to put to you

1. Timestamp field name in the output (below)
2. Which commands get the flag — `list`/`search` only, or `add` too
3. Whether the per-record `"schema"` tag stays
4. What `--json` does on error and what exit code it returns

---

### Fork 1 — what is the timestamp field called in the public output?

**Option A (recommended): `written`, emitted from a dedicated output struct.**
The public name matches ADR 0001's reasoning, and a separate struct honors the ADR's own framing that output is "a separate compatibility surface" from the internal on-disk names.
*Benefit:* a future storage-field rename can't silently change what your scripts parse.
*Cost:* a mapping layer that today looks like pure duplication — the fields are identical.
*Rejects if:* you want the public name to differ from disk on day one; then the struct is still right but this name isn't.

**Option B: `created_at`.**
*Benefit:* the widest ecosystem convention; most `jq` snippets and downstream tools expect it.
*Cost:* it contradicts an Accepted ADR's explicit reasoning, so ADR 0002 has to record the divergence and justify carrying two names for one concept.
*Rejects if:* you don't want disk and output disagreeing.

**Option C (lighter): `written`, serialized straight from `store::Note` — no mapping layer.**
*Benefit:* smallest diff; `Note` already derives `Serialize` (`src/store.rs:3`), so this is nearly free.
*Cost:* welds the public format to the on-disk struct, which is the exact coupling ADR 0001 line 23-25 warns against.
*Rejects if:* you expect the storage struct to change independently.

Which one?