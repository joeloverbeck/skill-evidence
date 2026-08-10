I read the repo before picking this up: `README.md`, `docs/adr/0001-storage-format.md`, `CONTRIBUTING.md`, `src/main.rs`, `src/store.rs`, `CHANGELOG.md`, `docs/status.md`, `docs/release-process.md`, `Cargo.toml`, and both scripts.

## What the repo already settles (no question needed)

| Item | Evidence | Status |
|---|---|---|
| A new ADR is required | ADR 0001 §Consequences: a machine-readable output format "would need its own ADR"; CONTRIBUTING: shipped formats need an ADR to change | **Verified** — deliverable is `docs/adr/0002-json-output.md` (0001 is the only existing ADR) |
| README gets a line for the flag | CONTRIBUTING: "Every new flag needs a line in the README's command list" | **Verified** — also retires README:11 "There is no machine-readable output mode" |
| CHANGELOG entry now, not at release | CONTRIBUTING: entry under `## Unreleased` "at the time of the change" | **Verified** |
| No new dependency | `serde_json` is already a direct dep (`Cargo.toml:10`); ADR 0001 warns a new runtime dep needs a superseding ADR | **Verified** — none needed |
| No pretty-print variant | ADR 0001 notes `to_string_pretty` is available, but one-record-per-line requires compact output | **Derived** — the two are mutually exclusive under Option A |
| This work is permitted | `docs/status.md`: "feature-frozen apart from output-format work" | **Verified** |

## A correction to my own Option A preview

My Option A sample line used `"created_at"`. ADR 0001:13–14 **explicitly rejected that exact name**:

> The name was chosen over `created_at` because a note can be edited, and `written` records the original writing.

Your "Option A" selected the *shape* — one record per line. The field names inside the sample were preview detail and are not ratified. So the name is still open, and it matters more than the shape did: ADR 0002 will freeze it as a compatibility surface.

## Branches ahead

1. **Timestamp field name** — asking now.
2. **Which commands honor `--json`** — `add` included or read paths only; this also determines whether the flag is global or per-subcommand. Next, unless you answer it here.
3. Then a full scope block for you to ratify.

---

**Question 1 — what is the timestamp field called in `--json` output?**

**Option 1 — `written` (recommended).** Line becomes `{"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"written":"2026-06-01T00:00:00Z"}`. ADR 0002 records `written` as the public name and `main.rs` serializes `store::Note` directly.
*Benefit:* one vocabulary across disk, code, docs, and output — and ADR 0001's semantic argument (a note can be edited; the stamp is the writing, not a generic creation) transfers intact to the public surface.
*Cost:* binds a public compatibility surface to a field ADR 0001 declared internal. If storage ever renames it, output must either follow — a breaking change to a shipped format, so another ADR — or grow the translation layer then.
*Rejected by:* an existing consumer script or sibling tool already keyed to `created_at`.

**Option 2 — `created_at`.** `main.rs` gains a dedicated output struct (or `#[serde(rename)]`) so the wire name is pinned independently of `Note`; ADR 0002 records the `written` → `created_at` mapping.
*Benefit:* the public surface is insulated by construction, honoring "on-disk field names are internal" most literally; `created_at` is the more conventional name for generic JSON consumers.
*Cost:* two names for one value in a deliberately small project, and ADR 0002 would re-adopt the name ADR 0001 argued against — a reader of both has to reconcile them.
*Rejected by:* nothing currently in the repo — ADR 0001 states nothing outside it reads the notes file, so there is no incumbent consumer to satisfy.

I recommend **Option 1**. Which do you want — and if you already know the answer to branch 2, give me both and I'll go straight to the scope block.