I read the whole repo before forming questions. Your dependency worry is already settled by the record — but it's settled in a way that reveals a bigger question underneath it.

## Premise checks

**"We may need a new dependency for pretty-printing" — contradicted, for JSON.** `docs/adr/0001-storage-format.md:18-20` addresses this directly: *"Serialization and pretty-printing are both provided by `serde_json`, which is already a direct dependency. Pretty-printing needs no additional crate; `serde_json::to_string_pretty` covers it."* `Cargo.toml:10` confirms `serde_json = "1"` is a direct dep. The same ADR (`:21-22`) adds that adding a runtime dependency would require a *superseding* ADR — so a new crate isn't just unnecessary, it's gated.

**"notekeeper has machine-readable output to pretty-print" — contradicted.** `README.md:11-12`: *"All commands currently print human-readable text to stdout. There is no machine-readable output mode."* `src/main.rs:29` and `:34` confirm it — plain `println!` with `  ` separators. There is no JSON on any output path today.

That's the crux: **`--pretty` has nothing to prettify yet.** The flag presupposes an output format that doesn't exist, so "add `--pretty`" is really "add an output surface, then a toggle on it." Which surface you pick is what determines whether your dependency worry was ever live.

**"The project can accept this work" — verified.** `docs/status.md:5`: feature-frozen *"apart from output-format work"* — this is precisely the carve-out.

## Branch map

1. **What `--pretty` prettifies** — JSON output, the existing human text, or both. (Asking now.)
2. **Which commands get it** — `list`/`search` only, or `add` too.
3. **Gate obligations** that follow from branch 1 — ADR requirements differ per option.
4. **Derived, not forks:** `CONTRIBUTING.md:3` requires a README command-list line for every new flag; `CONTRIBUTING.md:7-8` requires a `## Unreleased` changelog entry at the time of the change. Both apply to any option.
5. **Out of scope unless you say otherwise:** releasing. `docs/release-process.md:11-12` — publishing is permanent and unreversible; you asked about a flag, not a release.

## Fork 1 — what does `--pretty` prettify?

**Option A (recommended): pretty-printed JSON output.** Adds a machine-readable mode, with `--pretty` selecting indented rather than compact JSON. Uses `serde_json::to_string_pretty`; `Note` already derives `Serialize` (`src/store.rs:3`). **Benefit:** zero new dependencies, exactly as ADR 0001 anticipated; fits the output-format carve-out in the freeze. **Cost:** `docs/adr/0001-storage-format.md:23-25` says a machine-readable output format is a separate compatibility surface that *"would need its own ADR"* — so this option owes a new ADR, and per `CONTRIBUTING.md:4-5` the format is frozen for downstream parsers once shipped. **Rejected if:** you don't want to own a new public compatibility surface.

**Option B: prettify the existing human text** (aligned columns, color, boxes). **Benefit:** no new public format surface, no format-compat ADR. **Cost:** this is the option that actually needs a new crate (a table or color library), which `docs/adr/0001-storage-format.md:21-22` says requires a *superseding* ADR — your dependency worry becomes real here, and it's the harder ADR to write. Also, `CONTRIBUTING.md:4-5` arguably makes the current text a shipped format already, so changing it may need an ADR regardless. **Rejected if:** you want to keep the dependency set at four crates.

**Option C: both** — `--json` plus `--pretty` affecting each. **Benefit:** one coherent output story. **Cost:** both ADRs, roughly double the surface, and contradicts `CONTRIBUTING.md:3` ("keep the CLI surface small"). **Rejected if:** you want one reviewable change.

One sub-decision folded into A, flag me if you disagree: a bare `--pretty` with no way to request JSON is an odd CLI shape. I'd pair it as `--json` (compact) + `--pretty` (implies JSON, indented), or collapse to `--format text|json|json-pretty`. I lean toward the latter as the smaller surface.

**Which option, and if A, which flag shape?**