I've read the repo. Findings first — several of these change your plan before you get to design it.

## Verified premises

**`--json` is genuinely new, and it's the one thing not frozen.** README.md:11-12 — "All commands currently print human-readable text to stdout. There is no machine-readable output mode." And docs/status.md:5 — "feature-frozen apart from output-format work." Your plan is precisely the carve-out. Not blocked.

**The docs commit you to a new ADR.** This is the big one. ADR 0001:23-25: "The on-disk field names are internal… A machine-readable *output* format, if one is ever added, is a separate compatibility surface and would need its own ADR." CONTRIBUTING.md:4-5 backs it: "Public output formats are a compatibility surface: once a format ships, downstream scripts parse it." So **ADR 0002 is a required deliverable of this work**, not a nice-to-have.

**No new dependency needed — and you're not allowed one anyway.** ADR 0001:18-20 says `serde_json` already covers serialization *and* pretty-printing (`to_string_pretty`). ADR 0001:21-22: "Adding a runtime dependency requires a superseding ADR." You can stay inside that constraint; just don't reach for a JSON-formatting crate.

**Two mechanical obligations attach at commit time:** a README command-list line for the new flag (CONTRIBUTING.md:3), and a `## Unreleased` CHANGELOG entry "at the time of the change, not at release time" (CONTRIBUTING.md:7-8).

## Contradicted premise

**`src/store.rs` is a stub.** `append` returns `Ok(())` with no I/O, `load_recent` and `search` both return `Vec::new()` (store.rs:21-25), and `now_rfc3339` returns a hardcoded literal (store.rs:16-19). So today `list` and `search` print nothing regardless of format. There's also no `tests/` directory. You cannot end-to-end verify JSON output against real notes in this tree — any "it works" claim would be about the serializer only.

**Unavailable:** git history (the Bash call was denied), so I can't check whether output-format work was already attempted and reverted.

## Out of scope — flagging, not touching

docs/release-process.md:11-12 ends in `scripts/publish.sh`, which is irreversible ("can never be unpublished, edited, or reused") and appends to `PUBLISHED.log`. Nothing in your ask requires releasing, so I won't run preflight or publish. Adding `--json` needs the changelog entry, not a version bump.

## Branch map

Five forks I can foresee: **(F1)** whether the JSON output reuses the on-disk schema or gets its own; **(F2)** which subcommands take `--json`; **(F3)** global flag vs per-subcommand; **(F4)** error and exit-code behavior under `--json` (today `add` panics via `expect("write note")`, main.rs:24); **(F5)** whether this run writes code or stops at plan + ADR, given the stub.

---

**F1 — does `--json` emit the on-disk schema, or its own?**

*Option A (recommended): define a dedicated output struct.* Even if v1's field names come out identical to disk (`body`, `tags`, `written`), the output type is its own thing that ADR 0002 pins down. **Benefit:** honors ADR 0001:23-25 literally — storage stays internal and renameable, output stays stable for downstream scripts. **Cost:** ~10 extra lines and a second struct to keep in sync. **What would reject it:** if you'd rather guarantee the two never diverge, in which case A is overhead.

*Option B: `#[derive(Serialize)]` on `store::Note` straight to stdout.* **Benefit:** smallest possible diff — the derive already exists (store.rs:3), so it's nearly free, and it fits CONTRIBUTING's "keep the CLI surface small." **Cost:** welds `written` — a name ADR 0001:13-15 chose for *storage* reasons — into a permanent public contract. After it ships, renaming the on-disk field means either breaking consumers or adding the mapping layer you skipped. **What would reject it:** any expectation that the storage format evolves independently.

Which way?