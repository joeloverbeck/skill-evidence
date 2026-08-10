| File | POSED_FORK | BRANCH_MAP_BEFORE_FIRST_FORK | MEETS_REQUIREMENT |
|---|---|---|---|
| output-1.md | yes | yes | yes |
| output-2.md | yes | yes | yes |
| output-3.md | yes | yes | yes |

## Evidence

**output-1.md**

Branch map (line 33–35), positioned before the first fork:

> ## Branch map
>
> Forks I expect to resolve with you: **(A)** which commands get the flag · **(B)** global vs per-subcommand flag · **(C)** the output shape ← *permanent* · **(D)** error contract in JSON mode. The ADR, README line, and changelog entry are already settled by the evidence above, so I won't ask about those.

First fork posed afterward (line 41):

> **Q1 — What shape does `--json` emit for `list`/`search`?**

Four forks (A–D) are identified together as a set, ahead of Q1.

---

**output-2.md**

Branch map (line 25–27), positioned before the first fork:

> ## Branch map
>
> Forks I expect to walk you through: **(1) schema identity** — internal struct vs. output DTO; **(2) surface** — which commands, global vs. per-command flag; **(3) envelope** — bare array vs. wrapped object vs. JSONL; **(4) schema version field**; **(5) errors and exit codes under `--json`**; **(6) compact vs. pretty**. Then what I actually build this session.

First fork posed afterward (line 31):

> ### Fork 1 of 6 — What defines the JSON schema?

Six forks enumerated by content, not merely counted, ahead of Fork 1.

---

**output-3.md**

Branch map (line 23–25), positioned before the first fork:

> ## Branch map
>
> Five forks I can foresee: **(F1)** whether the JSON output reuses the on-disk schema or gets its own; **(F2)** which subcommands take `--json`; **(F3)** global flag vs per-subcommand; **(F4)** error and exit-code behavior under `--json` (today `add` panics via `expect("write note")`, main.rs:24); **(F5)** whether this run writes code or stops at plan + ADR, given the stub.

First fork posed afterward (line 29):

> **F1 — does `--json` emit the on-disk schema, or its own?**

Five forks (F1–F5) named as a set ahead of F1.