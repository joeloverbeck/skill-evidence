I've read the whole repo. Before any questions, here's what the evidence actually says — several things contradict the "just add a flag" framing.

## What notekeeper is

Rust CLI, clap-derive, three subcommands in `src/main.rs:12-17`: `add`, `list`, `search`. All three print ad-hoc text via `println!`.

One thing you should know up front: **`src/store.rs` is a stub.** `append` returns `Ok(())` without writing (`src/store.rs:21`), `load_recent` and `search` return empty `Vec`s (`src/store.rs:23-25`), and `now_rfc3339` returns a hardcoded `"2026-06-01T00:00:00Z"` (`src/store.rs:16-19`). So `--json` would today emit an empty result set from every read command. That doesn't block the design, but it means no end-to-end behavior can be demonstrated.

## What the docs commit you to (verified)

| # | Commitment | Source |
|---|---|---|
| 1 | **A machine-readable output format needs its own ADR.** Verbatim: "A machine-readable *output* format, if one is ever added, is a separate compatibility surface and would need its own ADR." | `docs/adr/0001-storage-format.md:23-25` |
| 2 | **Once shipped, the format is frozen** — "downstream scripts parse it. Changing a shipped format needs an ADR." | `CONTRIBUTING.md:4-5` |
| 3 | **No new crate.** serde_json is already a direct dep and `to_string_pretty` covers pretty-printing; adding a runtime dependency requires a *superseding* ADR. | `docs/adr/0001-storage-format.md:18-22`, `Cargo.toml:9-10` |
| 4 | **Every new flag needs a line in the README command list.** | `CONTRIBUTING.md:3` |
| 5 | **Changelog entry under `## Unreleased` at the time of the change**, not at release. | `CONTRIBUTING.md:8` |
| 6 | `README.md:11-12` says "There is no machine-readable output mode" — that sentence must be retracted. | `README.md:11-12` |
| 7 | **On-disk field names are explicitly internal.** So `body`/`tags`/`written` are *not* inherited defaults for output — reusing them is a decision. | `docs/adr/0001-storage-format.md:23-24` |

Good news on the premise you were unsure about: `docs/status.md:5` says notekeeper is "feature-frozen apart from **output-format work**." Your plan is the one thing explicitly still sanctioned. It is not a scope violation.

Net: this is not one flag. Minimum deliverable is **code + ADR 0002 + README (two edits) + CHANGELOG**.

## Blocking constraint you should know now

`.claude/settings.local.json:14` denies `Bash(cargo:*)` and `Bash(git:*)`. I **cannot compile, clippy, fmt, or test anything.** `CONTRIBUTING.md:6` requires `cargo fmt` and `cargo clippy` before a PR, and my own standing rule is not to call work complete without lint/typecheck/tests passing. Any code I write ships **unverified against the compiler** — you'd have to run those yourself. Treat that as a stated assumption, not something I'll quietly paper over.

Separately: `./scripts/publish.sh` *is* allow-listed, and `docs/release-process.md:10-11` says a published version can never be unpublished or reused. Releasing is not part of adding a flag. **I will not run it** unless you separately and explicitly ask.

## Branch map

Six forks I expect to resolve, in order: **(1) which commands get it** → (2) flag shape (`--json` vs `--format`) → (3) record framing (JSONL vs array vs pretty) → (4) field names & per-command field sets → (5) envelope/version field → (6) error channel and exit codes in JSON mode. ADR 0002 is *not* a fork — commitment #1 makes it mandatory.

## Decision ledger

| Key | Decision | Status |
|---|---|---|
| `adr-required` | ADR 0002 must accompany this change | evidence-resolved (ADR 0001) |
| `no-new-deps` | Use existing serde_json only | evidence-resolved (ADR 0001) |
| `sanctioned` | Output-format work is exempt from the feature freeze | evidence-resolved (status.md) |
| `no-compiler` | Code ships unverified; you run fmt/clippy/test | constraint, disclosed |
| `no-release` | Release process out of scope | asserted, awaiting your veto |

---

**Fork 1 of 6 — which commands get `--json`?**

**Option A (recommended): all three, including `add`.** A machine-readable mode that covers only the read paths forces a calling script to parse JSON for `list`/`search` and scrape `"added note at …"` text for `add`. `add` is the command a script is *most* likely to automate. Cost: you must design a success-record shape for `add`, which is a different shape from a note list, so it's one extra format decision. Rejected if you know the consumer is read-only tooling.

**Option B: `list` and `search` only.** Smaller surface, honors `CONTRIBUTING.md:3` ("keep the CLI surface small"), and both commands emit the same record type so there's exactly one shape to freeze. Cost: `add` stays unscriptable, and adding it later is a second ADR under commitment #2. Rejected if anything automating notekeeper needs to write.

**Option C: `list` only.** Minimum viable; `search` output today (`src/main.rs:33-35`) already drops tags, so its JSON shape needs its own call. Cost: almost certainly a follow-up, and follow-ups here cost an ADR each.

I recommend **A** — because the freeze rule in `CONTRIBUTING.md:4-5` makes incrementalism expensive, the cheap move is to define all three shapes once, in one ADR.

Which do you want?