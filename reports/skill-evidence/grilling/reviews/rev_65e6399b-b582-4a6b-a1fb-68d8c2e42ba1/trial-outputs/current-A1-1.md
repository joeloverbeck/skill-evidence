Read the repo first. Here's what's already settled by evidence, so we don't spend the question on it:

**Resolved — doesn't discriminate between the options**
- **Cost is identical.** `serde_json` is already a direct dependency (`Cargo.toml:10`), and ADR 0001 states pretty-printing needs no additional crate. Neither spelling needs a new dep, so neither trips the "new runtime dependency requires a superseding ADR" rule.
- **Either way you owe an ADR.** ADR 0001's Consequences say a machine-readable *output* format "would need its own ADR," and CONTRIBUTING says shipped formats are a compatibility surface. That's not a tiebreaker; it's a toll on both roads.
- **Either way: one README command-list line + a `## Unreleased` changelog entry at the time of the change** (CONTRIBUTING).
- **The work is in scope.** `docs/status.md` says feature-frozen *apart from output-format work*.
- **No collision.** `main.rs` currently has only `--tag` and `--limit`; nothing named `--json` or `--format` exists.

**My recommendation: `--json`.**

| | `--json` | `--format=json` |
|---|---|---|
| What ships | One boolean flag on the output-producing commands | One value-taking flag **plus a value vocabulary** |
| Main benefit | Smallest surface; matches CONTRIBUTING's "keep the CLI surface small" | Extension point already in place if a second format lands |
| Main cost | If a 2nd format arrives you either add `--format` alongside it or carry `--json` as an alias — two spellings | Every accepted value is itself a shipped compat surface; the enum invites growth in a repo that is explicitly feature-frozen |
| What would reject it | A second output format is genuinely planned | JSON is the only format you'll ever ship — then the extension point is speculative generality |

The deciding input isn't in the repo, it's in your head, so here's the one question:

**Is JSON the only machine-readable output you expect notekeeper to ever ship, or is a second format (CSV/YAML/TSV) actually on the roadmap?**

- **Only JSON** → `--json`. Ratified, done.
- **Second format genuinely planned** → `--format=json` now, and the ADR should name the initial accepted values so the vocabulary ships deliberately rather than by accretion.

Two things I'm flagging but *not* grilling you on, since you scoped this to the flag name: which subcommands get it (`list`/`search` obviously; `add`'s "added note at …" line is a judgment call), and global-vs-per-subcommand placement. Say the word and I'll pick those up separately.