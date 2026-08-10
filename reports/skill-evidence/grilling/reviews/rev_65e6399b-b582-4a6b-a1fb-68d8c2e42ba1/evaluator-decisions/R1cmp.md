All six files read. Verdicts:

| File | POSED_FORK | BRANCH_MAP_BEFORE_FIRST_FORK | MEETS_REQUIREMENT |
|---|---|---|---|
| output-1.md | yes | yes | yes |
| output-2.md | yes | yes | yes |
| output-3.md | yes | yes | yes |
| output-4.md | yes | yes | yes |
| output-5.md | yes | yes | yes |
| output-6.md | yes | yes | yes |

## Evidence

**output-1.md** — Branch map at line 24-26, immediately preceding the first fork ("**Which commands get `--json`?**", line 28):
> "## Branches I expect to resolve
> Scope (which commands) → output shape and field names → flag placement → error/exit-code behavior. First one:"

Four foreseen forks named as a set, before the first fork is posed.

**output-2.md** — Branch map at line 33-37, before the first fork ("**Q1 — What shape does `--json` emit for `list`/`search`?**", line 41):
> "## Branch map
> Forks I expect to resolve with you: **(A)** which commands get the flag · **(B)** global vs per-subcommand flag · **(C)** the output shape ← *permanent* · **(D)** error contract in JSON mode. The ADR, README line, and changelog entry are already settled by the evidence above, so I won't ask about those.
> Starting with C, since it's the one that locks."

**output-3.md** — Branch map at line 31-33, before the first fork ("**Fork 1 of 6 — which commands get `--json`?**", line 47):
> "## Branch map
> Six forks I expect to resolve, in order: **(1) which commands get it** → (2) flag shape (`--json` vs `--format`) → (3) record framing (JSONL vs array vs pretty) → (4) field names & per-command field sets → (5) envelope/version field → (6) error channel and exit codes in JSON mode. ADR 0002 is *not* a fork — commitment #1 makes it mandatory."

**output-4.md** — Branch map at line 25-27, before the first fork ("### Fork 1 of 6 — What defines the JSON schema?", line 31):
> "## Branch map
> Forks I expect to walk you through: **(1) schema identity** — internal struct vs. output DTO; **(2) surface** — which commands, global vs. per-command flag; **(3) envelope** — bare array vs. wrapped object vs. JSONL; **(4) schema version field**; **(5) errors and exit codes under `--json`**; **(6) compact vs. pretty**. Then what I actually build this session."

**output-5.md** — Branch map at line 33-35, before the first fork ("**Fork 1 of ~5 — scope and flag placement.**", line 39); also pre-announced at line 1 ("Before the first fork, here's what the environment actually commits you to"):
> "## Branch map
> The forks I expect to walk you through: (1) which commands get `--json` and where the flag lives; (2) output shape — array vs JSON Lines, pretty vs compact; (3) field names and whether there's an envelope/version; (4) error and exit-code behavior under `--json`; (5) how far the doc/ADR work goes in this change."

**output-6.md** — Branch map at line 23-25, before the first fork ("**F1 — does `--json` emit the on-disk schema, or its own?**", line 29):
> "## Branch map
> Five forks I can foresee: **(F1)** whether the JSON output reuses the on-disk schema or gets its own; **(F2)** which subcommands take `--json`; **(F3)** global flag vs per-subcommand; **(F4)** error and exit-code behavior under `--json` (today `add` panics via `expect("write note")`, main.rs:24); **(F5)** whether this run writes code or stops at plan + ADR, given the stub."

In every file the enumerated set appears in a dedicated section that ends immediately before the first posed fork; no file poses a fork or question earlier in the text (the earlier "out of scope unless you say otherwise" statements in output-2, -3, -5, and -6 are asserted defaults, not posed forks).