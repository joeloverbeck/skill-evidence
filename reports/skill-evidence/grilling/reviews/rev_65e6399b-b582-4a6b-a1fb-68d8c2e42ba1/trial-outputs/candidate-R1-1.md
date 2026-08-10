I've read the whole repo. Your premise checks out, but the plan as stated is missing three things the docs bind you to, and there's one trap in the code.

## Premises verified

| Claim | State | Evidence |
|---|---|---|
| `--json` is permissible despite feature freeze | **verified** | `docs/status.md:5` — "feature-frozen apart from output-format work." This is the exception. |
| No machine-readable mode exists today | **verified** | `README.md:11-12`; `src/main.rs:25,29,33` — three `println!` sites, all human text |
| Git history / in-flight related work | **unavailable** | Bash `git log` was denied. I can't rule out a parallel branch already doing this. |

## What the docs commit you to (contracts, not choices)

1. **An ADR is mandatory.** `docs/adr/0001-storage-format.md:23-25` — "A machine-readable *output* format, if one is ever added, is a separate compatibility surface and would need its own ADR." Your plan needs an ADR 0002 as a deliverable, not a follow-up.
2. **No new crate.** `ADR 0001:21-22` — a new runtime dependency requires a *superseding* ADR. `ADR 0001:18-20` pre-empts the need: `serde_json::to_string_pretty` is already there. So no `serde_json`-alternative, no `tabled`, no `anyhow`.
3. **Three satellite edits, at change time.** A README command-list line per new flag (`CONTRIBUTING.md:3`), a `## Unreleased` changelog entry *now, not at release* (`CONTRIBUTING.md:7-8`), and `cargo fmt` + `cargo clippy` (`CONTRIBUTING.md:6`).
4. **The shape is a one-way door.** `CONTRIBUTING.md:4-5` — "once a format ships, downstream scripts parse it. Changing a shipped format needs an ADR." Every field-name decision below is permanent-ish on ship.

## The trap

`src/store.rs:3-8` already derives `Serialize` on `Note` with fields `body`, `tags`, `written`. The one-line implementation is `serde_json::to_string(&note)` — and that would silently publish the internal name `written` as your permanent public contract. But `ADR 0001:23-25` draws exactly the opposite line: "The on-disk field names are internal. Nothing outside this repository reads `notes.jsonl`. A machine-readable *output* format... is a separate compatibility surface." Reusing the storage struct welds the two surfaces together on day one.

Also worth knowing before you plan tests: `store.rs:21-25` are stubs (`append` is a no-op, `load_recent` and `search` return empty vectors) and `now_rfc3339` is hardcoded. There's no test directory and no dev-dependencies. You can't exercise this end-to-end in this tree as it stands.

## Branches I expect to resolve

Scope (which commands) → output shape and field names → flag placement → error/exit-code behavior. First one:

**Which commands get `--json`?**

- **A — `list` and `search` only (recommended).** Changes: two of three `println!` sites. Benefit: those are the read paths a script actually consumes, and it's the smallest surface, matching `CONTRIBUTING.md:3` "Keep the CLI surface small." Cost: a script that adds a note gets no structured confirmation back, so a create-then-read workflow needs a follow-up `list`. Rejected if: you have a caller that needs the written timestamp back from `add`.
- **B — all three, including `add`.** Changes: also `main.rs:25`, emitting the created note object instead of `added note at {ts}`. Benefit: `add` becomes scriptable; the caller gets the `written` value it would otherwise have to re-query. Cost: commits you to a second permanent output shape (a single object, not a list) for the one command where the human string is genuinely useful. Rejected if: you want exactly one output schema to specify in ADR 0002.
- **C — `list` only.** Benefit: smallest possible one-way door. Cost: `search` is the more likely scripting target of the two, so this probably gets revisited within a release — and revisiting means another ADR.

Which way do you want it?