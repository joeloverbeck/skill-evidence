I re-verified the repo myself rather than inheriting the earlier turn's summary. Here is what the evidence settles, and the one place it contradicts the sample you just approved.

## Verified against the repo

| Claim | State | Source |
|---|---|---|
| Output format needs its own ADR (0002) | verified | `docs/adr/0001-storage-format.md:23-25`, `CONTRIBUTING.md:4-5` |
| No new crate needed — `serde_json` is already a direct dep | verified | `Cargo.toml:10`, `docs/adr/0001-storage-format.md:18-20` |
| New flag ⇒ README command-list line; user-visible change ⇒ `## Unreleased` entry now | verified | `CONTRIBUTING.md:3,8` |
| Output-format work is the one thing not frozen | verified | `docs/status.md:5` |
| `store::Note` derives `Serialize` with `body`/`tags`/`written` | verified | `src/store.rs:3-8` |
| Notes have no id on disk; no `edit` command exists yet | verified | `src/store.rs:4-8`, `README.md:6-9` |

## The contradiction

The Option A sample you approved uses `"created_at"`. ADR 0001 explicitly considered and **rejected** that name:

> The name was chosen over `created_at` because a note can be edited, and `written` records the original writing. — `docs/adr/0001-storage-format.md:13`

The ADR also says on-disk names are internal and the output is a separate surface, so renaming is *permitted*. But the rejection was semantic, not storage-specific, and it survives the surface boundary. This has to be settled before anything else, because it defines the compatibility surface you'll be stuck with.

## Branches I expect to close

1. **Timestamp field name** — `written` vs `created_at` (below).
2. **Flag reach** — does `add` get `--json` too, and is `--json` global or per-subcommand?
3. **Failure and empty-result signaling** — how a script tells "no matches" from "the command broke".

---

**Fork 1 — what do you call the timestamp in `--json` output?**

- **`written`** *(recommended)* — output mirrors the ADR's vocabulary; one name for one concept across disk, docs, and output. **Benefit:** you never ship a public field whose name the project already ruled semantically wrong; when an `edit` command eventually lands, `written` still reads correctly and an `edited` field slots in beside it. **Cost:** script authors coming from other tools will reach for `created_at` first, and it diverges from the sample you already said yes to. **Rejected if:** your existing shell scripts or an org convention already key on `created_at`.

- **`created_at`** — ships exactly the sample you approved. **Benefit:** conventional, zero surprise for anyone writing `jq '.created_at'`. **Cost:** re-introduces the precise semantic error ADR 0001 named, and ADR 0002 has to explain why the output surface contradicts the storage ADR on the same concept; once shipped, `CONTRIBUTING.md:5` means changing it needs another ADR. **Rejected if:** you agree the edit-vs-write distinction is real and worth encoding in the public name.

Independent of the answer, the record will be built by a dedicated output struct rather than serializing `store::Note` directly — ADR 0001 calls the on-disk names internal, so the two must be free to drift.