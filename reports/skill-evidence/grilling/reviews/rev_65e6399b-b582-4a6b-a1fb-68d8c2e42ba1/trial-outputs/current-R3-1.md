I re-read the repo against the Option A sample before continuing. Four things are settled by evidence, and one thing in my own sample is wrong.

**Verified**

- **An ADR is mandatory, not optional.** ADR 0001 (`docs/adr/0001-storage-format.md:23-25`) states a machine-readable output format "is a separate compatibility surface and would need its own ADR," and `CONTRIBUTING.md:4-5` repeats it. So ADR 0002 is a required deliverable, not a fork.
- **Two more required artifacts.** `CONTRIBUTING.md:3` — every new flag needs a line in the README command list. `CONTRIBUTING.md:7-8` — a `## Unreleased` changelog entry *at the time of the change*.
- **This work is sanctioned.** `docs/status.md:5` — "feature-frozen apart from output-format work."
- **Releasing is not part of this.** `docs/release-process.md:11` — `publish.sh` is permanent and unrepeatable. I am treating publish as out of scope and will not run it.

**Contradicted — my own sample**

My Option A example printed `"created_at"`. ADR 0001 (`docs/adr/0001-storage-format.md:13-14`) explicitly chose `written` **over** `created_at`, because a note can be edited and `written` records the original writing. You ratified Option A's *shape*; you did not ratify that field name, and I shouldn't let it through silently.

**Remaining branches** — (1) timestamp field name, (2) which commands `--json` covers, (3) error behavior under `--json`. Taking them one at a time.

---

**Fork 1 — what is the timestamp field called in `--json` output?**

- **`written`** *(recommended)* — output line reads `{"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"written":"2026-06-01T00:00:00Z"}`. Benefit: one name for one concept across storage, output, and ADR 0001's stated reasoning; `store::Note` already derives `Serialize` with that name (`src/store.rs:3-8`), so `list`/`search` serialize with no translation layer. Cost: the name is unusual to a script author who expects `created_at`, and you inherit an internal name onto a public surface that ADR 0001 had deliberately kept internal. Rejected if you'd rather the public surface use conventional names regardless of storage.
- **`created_at`** — the name from my sample. Benefit: conventional and immediately legible to consumers. Cost: two names for the same value, and it revives a name ADR 0001 rejected on semantic grounds — if note editing ever lands, `created_at` is actively misleading. Also needs an explicit rename mapping in the serializer. Rejected if you want ADR 0002 to stay consistent with 0001's vocabulary.

Which one?