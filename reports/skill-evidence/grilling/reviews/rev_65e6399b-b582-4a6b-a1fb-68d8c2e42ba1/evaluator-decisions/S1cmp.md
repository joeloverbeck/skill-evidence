## What the repository settles

The dependency question is **already answered by the repo, in the affirmative-no**. `0001-storage-format.md:18-20` (Consequences of an Accepted ADR):

```
- Serialization and pretty-printing are both provided by `serde_json`, which is already a
  direct dependency. Pretty-printing needs no additional crate; `serde_json::to_string_pretty`
  covers it.
```

And the same section closes the door on the alternative, `0001-storage-format.md:21-22`:

```
- Adding a runtime dependency requires a superseding ADR. This project keeps its dependency
  set deliberately small.
```

Two adjacent lines set the real cost. `README.md:11-12`: *"All commands currently print human-readable text to stdout. There is no machine-readable output mode."* And `0001-storage-format.md:23-25`: *"A machine-readable \*output\* format, if one is ever added, is a separate compatibility surface and would need its own ADR."* So a JSON `--pretty` costs an ADR, not a crate.

Note: only `README.md`, `CONTRIBUTING.md`, and `0001-storage-format.md` exist here. Both versions also cite `Cargo.toml:10`, `src/main.rs`, `src/store.rs`, `docs/status.md`, and `docs/release-process.md`, which I cannot check; I graded on the three files present.

## Version A

1. **RESOLVED_FROM_EVIDENCE: yes.** Opens with "already settled by the record," labels the worry "contradicted, for JSON," and never routes the crate question back to the user. The dependency only reappears as a *consequence* priced into Option B ("your dependency worry becomes real here"), attached to the superseding-ADR gate.
2. **CITED_CORRECTLY: yes.** ADR `:18-20` quoted verbatim; `:21-22` and `:23-25` characterized accurately; `README.md:11-12` verbatim; `CONTRIBUTING.md:3` (README flag line + small CLI surface), `:4-5` (shipped format), `:7-8` (`## Unreleased` at time of change) all faithful. Where the repo is genuinely ambiguous — whether today's human text already counts as a shipped format — it hedges with "arguably," which is the correct reading of `CONTRIBUTING.md:4-5`.
3. **ASKED_ANYWAY: no.** The closing ask is which output surface and which flag shape, not whether to take a dependency.

## Version B

1. **RESOLVED_FROM_EVIDENCE: yes.** Strongest single line of either version: *"verified, no crate, no question needed."* It also checks that the Accepted ADR's premise still holds against current `Cargo.toml` rather than trusting the ADR's age, and reframes the cost as "an ADR, not a crate."
2. **CITED_CORRECTLY: partial.** ADR `:18-20`, `:21-22`, `:23-25`, `README.md:11-12`, `CONTRIBUTING.md:3` and `:4-5` are all accurate. One claim misreads the ADR: it says exposing `written` "publicizes `written`, whose name ADR 0001 justified *on the grounds that it's internal*." The ADR justifies the *name* on different grounds — `:13-14`, `written` over `created_at` "because a note can be edited" — and internality is a separate consequence at `:23`. The inference (a public surface changes the calculus) is reasonable; the attribution to the ADR is not. It also states flatly that restyling existing text "is itself a change to a shipped format," where `CONTRIBUTING.md:4-5` says "public output formats," leaving it a judgment call.
3. **ASKED_ANYWAY: no.** Explicitly retires the question before the fork.

**BETTER_ON_EVIDENCE_USE: A** — both resolve the dependency question from the ADR without kicking it back to the user, but every one of A's citations to the checkable files is faithful and hedged where the repo is ambiguous, while B misattributes the ADR's rationale for the `written` field name.