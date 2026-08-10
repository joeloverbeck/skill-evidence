I read the whole repo. Before any questions — here's what the environment actually commits you to, because it changes the plan materially.

## Premise check

**Your premise holds:** there is genuinely no machine-readable output today (`README.md:11`), and `docs/status.md:5` says notekeeper is *"feature-frozen apart from output-format work"* — so this is the one sanctioned exception. The plan is in-bounds.

**But the docs pre-commit you to four things you didn't mention:**

| Obligation | Source |
|---|---|
| A machine-readable output format **needs its own ADR** | `docs/adr/0001-storage-format.md:23-25` |
| Once the format ships it's a **frozen compatibility surface**; changing it later needs another ADR | `CONTRIBUTING.md:2-3` |
| Every new flag needs a line in the README command list | `CONTRIBUTING.md:3` |
| Changelog entry under `## Unreleased` **at time of change**, not at release | `CONTRIBUTING.md:7-8` |

Good news: **you need zero new dependencies.** `serde_json` is already a direct dep, `Note` already derives `Serialize` (`src/store.rs:3`), and ADR 0001:18-21 pre-blesses `to_string_pretty`. That matters because ADR 0001:21 says *adding a runtime dependency requires a superseding ADR*.

## The trap in the obvious implementation

The one-line version of this feature is `serde_json::to_string(&note)`. That serializes the storage struct directly — which silently promotes the **on-disk field names into a permanent public contract**. ADR 0001:23-25 says the opposite in as many words:

> The on-disk field names are internal. Nothing outside this repository reads `notes.jsonl`. A machine-readable *output* format, if one is ever added, is a separate compatibility surface.

So the free implementation quietly welds together the two surfaces the ADR deliberately separated. That's the highest-stakes decision here, and it's the hardest to undo.

## Two things I could not verify

- **`src/store.rs` is a stub in this tree.** `append` is a no-op, `load_recent` and `search` return `Vec::new()`, `written` is hardcoded to `"2026-06-01T00:00:00Z"`, and the `_tag`/`_limit` params are unused. The comment at `store.rs:17` says the real build differs. Consequence: `list --json` and `search --json` will emit an empty result no matter what, so this work is verifiable by construction only, not end-to-end. (Relatedly, the `## Unreleased` entry claiming case-insensitive `--tag` has no corresponding code in this tree.)
- **Bash is denied in this session**, so I could not confirm `cargo` is installed or run the `cargo fmt` / `cargo clippy` that `CONTRIBUTING.md:6` requires before a PR. Recording that as unavailable, not passing.

**Out of scope unless you say otherwise:** `docs/release-process.md:5` — `./scripts/publish.sh` is irreversible and permanent. You asked for a feature, not a release. I won't touch the release path.

## Branch map

Forks I expect to resolve with you: **(A)** which commands get the flag · **(B)** global vs per-subcommand flag · **(C)** the output shape ← *permanent* · **(D)** error contract in JSON mode. The ADR, README line, and changelog entry are already settled by the evidence above, so I won't ask about those.

Starting with C, since it's the one that locks.

---

**Q1 — What shape does `--json` emit for `list`/`search`?**

**Option 1 — Envelope + explicit output struct, compact (recommended)**
```json
{"notes":[{"body":"…","tags":["…"],"written":"2026-06-01T00:00:00Z"}],"count":1}
```
A separate `#[derive(Serialize)] struct NoteOut` mapped from `store::Note`. *Benefit:* the envelope lets you add top-level fields later without a breaking change (which would otherwise cost an ADR per `CONTRIBUTING.md:2-3`), and the explicit struct keeps storage renames from leaking into the public surface, exactly as ADR 0001 intends. *Cost:* ~20 lines and a mapping that is redundant today, since the fields are currently identical. *Reject if:* you want `list --json | jq '.[]'` to work with no `.notes` accessor.

**Option 2 — Bare array, compact:** `[{…},{…}]`. *Benefit:* most idiomatic for jq, simplest diff. *Cost:* no room for top-level metadata later — adding any costs an ADR. *Reject if:* you foresee wanting count, query echo, or truncation flags.

**Option 3 — JSON Lines, mirroring `notes.jsonl`.** *Benefit:* streams, `head`-able, consistent with the storage decision. *Cost:* `--json` that isn't a single JSON document surprises `| jq .` consumers, and it maximally couples output to storage — the coupling ADR 0001:23 explicitly warns against. *Reject if:* your consumers are jq-based rather than line-based.

**Rides along with your answer (tell me if you disagree):** reuse the names `body` / `tags` / `written` but declare them in ADR 0002 as the *output* contract in its own right, so a future storage rename doesn't drag output with it. ADR 0001:13-14's reasoning for `written` over `created_at` applies just as well to output. And compact rather than pretty — `--json` is the machine path, and a second `--json-pretty` flag would fight `CONTRIBUTING.md:3`'s "keep the CLI surface small."