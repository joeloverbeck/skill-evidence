I read the whole repo. Before any questions — here's what the evidence already settles, including two things that change your plan and one hazard.

## Premise checks

**Confirmed: there's no JSON mode today.** `README.md:11-12` states it outright ("There is no machine-readable output mode"), and `src/main.rs:22-36` hand-formats all three commands with `println!`.

**Your plan is not optional-ADR work — the docs pre-commit you.** `docs/adr/0001-storage-format.md:23-25` says a machine-readable *output* format "is a separate compatibility surface and would need its own ADR." `CONTRIBUTING.md:4-5` adds that once a format ships, "changing a shipped format needs an ADR." So `--json` requires **ADR 0002 written before it ships**, and the schema you pick is frozen the moment it lands.

**Good news — this dodges the freeze.** `docs/status.md:5`: "feature-frozen apart from output-format work." Your feature is precisely the carved-out exception.

**No new crate, and don't reach for one.** `Cargo.toml:10` already has `serde_json`; ADR 0001:18-21 says `to_string_pretty` covers pretty-printing, and adding any runtime dependency "requires a superseding ADR." So the dependency question is closed before you ask it.

**The trap.** `store::Note` (`src/store.rs:3-8`) already derives `Serialize` with fields `body`/`tags`/`written`. The one-line version of your feature is `serde_json::to_string(&note)` — but ADR 0001:23 explicitly declares those on-disk names *internal*: "Nothing outside this repository reads `notes.jsonl`." Taking the one-liner silently promotes internal storage names into a permanently-frozen public surface, collapsing the exact separation that ADR drew. This is the decision most worth spending your attention on.

**Two obligations, already resolved — I'll just do them, no need to ask:** a README command-list line per new flag (`CONTRIBUTING.md:3`), and a `## Unreleased` changelog entry *at the time of the change* (`CONTRIBUTING.md:7-8`).

## Two things you should know before we design

**Your completion gate can't run here.** `CONTRIBUTING.md:6` requires `cargo fmt` and `cargo clippy`; `.claude/settings.local.json:14` denies `Bash(cargo:*)`. Any code I write in this environment ships uncompiled and unlinted. Also relevant: `src/store.rs:21-25` stubs `append`/`load_recent`/`search` to return empty, and `now_rfc3339` returns a hardcoded constant — there's no live data path to exercise a JSON encoder against.

**Hazard I'm steering around.** `docs/release-process.md:11-12` warns that publishing is irreversible ("can never be unpublished, edited, or reused") — and `scripts/publish.sh` sits on the permission *allow*-list, so it would run without prompting me. You asked for a JSON flag, not a release. I won't run `publish.sh` or `preflight.sh`. Flagging it so that stays a deliberate choice rather than an accident.

*(Unavailable evidence: `Bash(git:*)` is denied, so I couldn't check commit history or HEAD freshness.)*

## Branch map

Forks I expect to walk you through: **(1) schema identity** — internal struct vs. output DTO; **(2) surface** — which commands, global vs. per-command flag; **(3) envelope** — bare array vs. wrapped object vs. JSONL; **(4) schema version field**; **(5) errors and exit codes under `--json`**; **(6) compact vs. pretty**. Then what I actually build this session.

---

### Fork 1 of 6 — What defines the JSON schema?

**Option A — Explicit output DTO (recommended).** Add a serialization type in `main.rs` (or a new `output.rs`) that `Note` is mapped into; `store::Note`'s derive stays for disk only.
*Benefit:* honors ADR 0001's internal/public split; you can rename a storage field later without an ADR-gated breaking change; the output schema becomes a deliberate artifact you can document in ADR 0002.
*Cost:* a mapping function and a second struct — real duplication in a codebase whose stated value is smallness (`CONTRIBUTING.md:3`).
*Rejected if:* you decide the two schemas should be identical by policy forever, making the indirection pure overhead.

**Option B — Serialize `store::Note` directly.** `--json` prints the note struct as-is.
*Benefit:* near-zero code; disk and wire trivially agree; smallest possible diff.
*Cost:* freezes `body`/`tags`/`written` as a public contract. ADR 0001 anticipated a note being *edited* (that's why the field is `written`, not `created_at`) — if you ever add `edited`/`id`/`source`, every storage change becomes a public breaking change needing an ADR.
*Rejected if:* you expect the storage schema to grow, which ADR 0001's own reasoning suggests it might.

I recommend **A**: the cost is roughly fifteen lines, and it's the only option that keeps the compatibility promise cheap to honor later.

Which way — A or B?