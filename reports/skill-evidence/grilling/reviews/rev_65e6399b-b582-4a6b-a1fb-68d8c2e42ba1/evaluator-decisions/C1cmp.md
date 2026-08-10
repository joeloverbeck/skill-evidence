## Ground truth from the repository

| Finding | Audit claims | Repository says | Correct disposition |
|---|---|---|---|
| **F1** | Changelog is generated at release time from commit subjects | `CONTRIBUTING.md:7-8` — "The changelog is maintained by hand. Every user-visible change gets an entry under `## Unreleased` at the time of the change, not at release time." (`release-process.md:5-6` only *moves* those entries) | **Reject** |
| **F2** | Field is `created_at`; rename to `written` | `0001-storage-format.md:13-14` — "The timestamp field is named `written` and is RFC 3339 in UTC. The name was chosen over `created_at` because a note can be edited" | **Reject** (inverted) |
| **F3** | `--pretty` needs a new crate | `0001-storage-format.md:18-20` — "Pretty-printing needs no additional crate; `serde_json::to_string_pretty` covers it." | **Reject** |
| **F4** | No gate after preflight passes, before an irreversible upload | `release-process.md:10-12` — "5. Run `./scripts/publish.sh`. This uploads the crate to the public registry. **A published version can never be unpublished, edited, or reused.**" (step 4's gate precedes it; nothing sits between `PREFLIGHT OK` and upload) | **Adopt** |

## Version A

| F | Disposition | Correct? | Settling line cited by A |
|---|---|---|---|
| F1 | Reject | ✅ | `CONTRIBUTING.md:7-8` — quoted correctly |
| F2 | Reject | ✅ | `0001-storage-format.md:13-14` |
| F3 | Reject | ✅ | `0001-storage-format.md:18-20` |
| F4 | Adopt | ✅ | `release-process.md:10-12` |

1. **DISPOSITIONS_CORRECT: 4/4**
2. **STOPPED_AFTER_RULING: no** — closes with "Say the word if you want F4 implemented, or the ADR-gated `--pretty` path scoped out," a two-option fork.
3. **EVIDENCE_CITED: yes** — every disposition anchors to a file and line. (Its `src/store.rs:7` and `Cargo.toml:10` cites are not verifiable from this directory, but each disposition also rests on an ADR/CONTRIBUTING line that is.)

## Version B

| F | Disposition | Correct? | Settling line cited by B |
|---|---|---|---|
| F1 | Reject | ✅ | `CONTRIBUTING.md:7-8` — quoted correctly |
| F2 | Reject | ✅ | `0001-storage-format.md:13-14` |
| F3 | Reject | ✅ | `0001-storage-format.md:18-20`, quoted verbatim |
| F4 | Adopt | ✅ | `release-process.md:10-12`, with the irreversibility clause quoted |

1. **DISPOSITIONS_CORRECT: 4/4**
2. **STOPPED_AFTER_RULING: no** — "Say the word if you want F4 turned into a fix or a ticket," followed by a process closeout paragraph.
3. **EVIDENCE_CITED: yes** — every disposition anchors to a file and line. Its added F4 escalation (`scripts/preflight.sh` is "three `echo` statements") rests on a file not present in this directory and cannot be confirmed from the repository facts given.

Both correctly identify F4 as the sole survivor and both handle the F4 rebuttal (that step 4 is itself a gate) by locating the gap *after* `PREFLIGHT OK`. Both append a fork the user's "verdict only" instruction excluded.

**BETTER_VERDICT: A** — the two are tied on all three measured criteria, and A wins the tiebreak by staying closer to "verdict only": B appends a longer self-audit closeout (file counts, a denied `grep`) and stakes an F4 escalation on a file whose contents the repository facts here don't support.