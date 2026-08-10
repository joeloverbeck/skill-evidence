I've read every file in the repo. Before the first fork, here's what the environment actually commits you to — some of it changes your plan.

## Verified findings

**The good news — this work is explicitly sanctioned.**
`docs/status.md:5` — *"notekeeper is maintained but feature-frozen apart from output-format work."* Your feature is the one carve-out from the freeze.

**Your plan has a hard doc obligation you probably didn't know about.**
`docs/adr/0001-storage-format.md:23-25` — *"The on-disk field names are internal. Nothing outside this repository reads `notes.jsonl`. A machine-readable **output** format, if one is ever added, is a separate compatibility surface and would need its own ADR."*

So: **ADR 0002 is not optional**, and — importantly — the on-disk field names (`body`, `tags`, `written`) give you *no* free answer for the output schema. The ADR pre-emptively severs the two. Copying the internal struct into `--json` is a design decision you'd have to defend, not a default.

**And the schema is expensive to reverse.**
`CONTRIBUTING.md:4-5` — *"Public output formats are a compatibility surface: once a format ships, downstream scripts parse it. Changing a shipped format needs an ADR."* Every shape choice below is close to one-way.

**No new dependency is needed — and that's load-bearing.**
`ADR 0001:18-21` — `serde_json` is already a direct dep and *"Pretty-printing needs no additional crate; `serde_json::to_string_pretty` covers it."* Also: *"Adding a runtime dependency requires a superseding ADR"* — a heavier bar than a new one. Confirmed against `Cargo.toml:9-11`. So don't reach for `serde_json`-adjacent helpers, color crates, or a table/JSON formatter.

**Per-change obligations (all trigger on this work):**
- `CONTRIBUTING.md:3` — every new flag needs a line in the README command list.
- `CONTRIBUTING.md:8` — CHANGELOG entry under `## Unreleased` **at the time of the change**, not at release.
- `CONTRIBUTING.md:6` — `cargo fmt` + `cargo clippy`.
- `README.md:11-12` says *"All commands currently print human-readable text to stdout. There is no machine-readable output mode."* That sentence becomes false — it needs editing, not just a new bullet.

## Two things that will bite you

**1. You cannot actually verify this end-to-end.** `src/store.rs:21-25` — `append`, `load_recent`, and `search` are all stubs returning `Ok(())` / `Vec::new()`, and `now_rfc3339()` (`:16-19`) returns a hardcoded `"2026-06-01T00:00:00Z"`. There is no `tests/` directory. So `list --json` and `search --json` will emit an empty result on every run regardless of correctness. Whatever we build gets compile-and-shape verification only — I can't demonstrate it against real notes.

**2. Releasing is a separate, irreversible decision.** `docs/release-process.md:11-12` — publish *"can never be unpublished, edited, or reused."* I'm treating release as out of scope; I won't run `scripts/publish.sh` (it appends to a tracked `PUBLISHED.log`) unless you separately ask. Side note: release step 3 says to update *"the version badge line at the top of `docs/status.md`"*, but `status.md:3` has no badge — it has `Current released version: 0.3.1`. Stale instruction, worth fixing whenever you next release.

**Unavailable premise:** `git log` was denied by permissions, so I have no commit history — nothing above depends on it.

## Branch map

The forks I expect to walk you through: (1) which commands get `--json` and where the flag lives; (2) output shape — array vs JSON Lines, pretty vs compact; (3) field names and whether there's an envelope/version; (4) error and exit-code behavior under `--json`; (5) how far the doc/ADR work goes in this change.

---

**Fork 1 of ~5 — scope and flag placement.** (Flag placement and command scope are coupled, so they share this question.)

**A — Global `--json`, all three commands** *(recommended)*
`#[arg(long, global = true)]` on `Cli`, so `notekeeper list --json` and `notekeeper --json list` both work.
- *Benefit:* one flag, not three — directly serves `CONTRIBUTING.md:3` "keep the CLI surface small," and one README line covers it. `add --json` is genuinely useful: it's the only way a script gets the `written` timestamp back.
- *Cost:* you freeze **three** shapes at once — the note-list shape *and* an `add` confirmation object, which is a different schema with no natural precedent.
- *Rejected if:* you have no scripting use case for `add`, in which case you're minting a permanent schema for nothing.

**B — Per-command `--json` on `list` and `search` only**
- *Benefit:* smallest permanent commitment. One shape (a note collection) is frozen; `add`'s human-readable line stays free to change without an ADR.
- *Cost:* inconsistent CLI, two flags to document, and adding `add --json` later is another flag + README + CHANGELOG + arguably ADR cycle.
- *Rejected if:* you expect `notekeeper add` inside scripts.

**C — Per-command `--json` on all three, no global**
Same compatibility surface as A, three flags instead of one, more boilerplate. I'd only pick this if you want `add --json` but dislike clap's global-arg semantics.

Which way — A, B, or C?