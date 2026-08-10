I read the repo before forming an opinion. Your main worry is resolved by evidence, but it's pointing at the wrong gate.

## Verified findings

**No new dependency is needed — for JSON pretty-printing.** `serde_json = "1"` is already a direct dependency (`Cargo.toml:10`), and `docs/adr/0001-storage-format.md:18-20` pre-answers this in its Consequences: *"Serialization and pretty-printing are both provided by `serde_json` … Pretty-printing needs no additional crate; `serde_json::to_string_pretty` covers it."* That ADR is Accepted and its premise still holds against current `Cargo.toml`. So: **verified, no crate, no question needed.**

**The real gate is a different clause in the same ADR.** `ADR 0001:23-25` says the on-disk field names are internal, and *"A machine-readable output format, if one is ever added, is a separate compatibility surface and would need its own ADR."* `CONTRIBUTING.md:4-5` backs this: once a format ships, downstream scripts parse it. Right now `README.md:11-12` states there is **no** machine-readable output mode — so a JSON `--pretty` isn't a formatting tweak, it's the introduction of that surface. **The cost of this feature is an ADR, not a crate.**

Two more verified premises: `docs/status.md:5` freezes features *"apart from output-format work"* — so this work is in-scope. And `ADR 0001:21-22` means any new runtime dep needs a *superseding* ADR, which raises the price of the branch where your worry would be real.

Two caveats: `src/store.rs:21-25` — `append`/`load_recent`/`search` are stubs returning empty/`Ok(())`, so nothing here can be exercised end-to-end. And I couldn't check git history (Bash denied), so I can't tell whether a `--pretty`/`--json` attempt was made and reverted before; not material to the fork below.

## Branches I expect

(1) what `--pretty` actually renders — below; (2) whether ADR 0002 gets written; (3) which subcommands take the flag; (4) whether `written`/`body`/`tags` are exposed as-is; (5) the landing contract in `CONTRIBUTING.md` (README flag line, `## Unreleased` entry, fmt/clippy).

## Fork 1 — what does `--pretty` print?

Everything else depends on this, and it's the one that decides whether your dependency worry is live at all.

**A. Pretty-printed JSON (recommended).** `--pretty` switches `list`/`search` from text lines to indented JSON via `serde_json::to_string_pretty`.
- *Benefit:* zero new dependencies, explicitly blessed by ADR 0001; fills the gap README names.
- *Cost:* ships a compatibility surface, so it needs its own ADR — and it publicizes `written`, whose name ADR 0001 justified *on the grounds that it's internal*. That justification stops holding the moment it's in stdout.
- *Rejected if:* you don't want machine-readable output at all, only a nicer-looking terminal.

**B. Prettier human text** — color, alignment, a table.
- *Benefit:* better to read; doesn't create a JSON contract.
- *Cost:* this is the only branch where your worry is real. `comfy-table`/`owo-colors` = a new runtime dep = a **superseding** ADR per ADR 0001:21-22. Hand-rolling means ANSI codes plus TTY/`NO_COLOR` detection by hand. Also, per CONTRIBUTING's format clause, restyling the *existing* text output is itself a change to a shipped format.
- *Rejected if:* the dependency budget is the binding constraint — then A is strictly cheaper.

**C. Both — `--json` for the mode, `--pretty` as its indent modifier.** Cleanest semantics long-term, but `CONTRIBUTING.md:3` says keep the CLI surface small, and it's two flags to document instead of one.

Which one did you mean? If it's A, my next question is whether you want `--pretty` to *be* the JSON switch or to presuppose a `--json` you haven't built yet.