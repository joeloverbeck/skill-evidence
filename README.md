# skill-evidence

Evidence-gated revision for agent skills.

An agent finishes using a skill. This crate records one factual receipt about that use, appends it
to an immutable stream, and derives from the accumulated stream whether there is yet enough
independent evidence to justify *changing* that skill. Revision is refused until there is — and
when it is allowed, the replacement must survive blind validation before it lands.

The point is to stop skills from being edited on a hunch. A skill gets revised because several
independent runs went wrong in the same way, not because someone reread it and had an idea.

## The loop

1. **Capture.** After a completed use — clean or not — one receipt is recorded: whether the use
   qualified, its outcome (`clean`, `friction`, `material_failure`, `severe_incident`), and for
   anything non-clean the expected/observed/consequence facts plus a coarse symptom key.
2. **Accumulate.** Receipts append to `reports/skill-evidence/<skill>/events.jsonl`. Nothing is
   ever rewritten, reordered, or removed.
3. **Derive.** A gate projection is computed from the stream: how much qualifying use the skill's
   *current content* has seen, which incidents are open, and whether any cluster of them is
   independent enough to authorize a review.
4. **Revise, if authorized.** A review can only be claimed against the exact content hash the
   evidence was recorded against, and only when the freshly derived gate says so. A candidate must
   beat the incumbent under blind comparative trials before it may be landed.

Independence is the load-bearing part: two incidents count as independent only when they come from
distinct top-level agent sessions. Without that, "it went wrong three times" can mean one bad
afternoon.

## What you need

- **Rust 1.93+**, edition 2024.
- **An agent setup that reads `.claude/skills/` or `.agents/skills/`.** The installed operator
  packages are Markdown that an agent reads; the crate writes both locations.
- A repository to run it in. Evidence is per-repository and stays there.

## The four skill packages

The crate ships the operators, not just the machinery — a host that gets the library without these
has an engine and no driver.

| Package | What it does |
|---|---|
| `skill-evidence-capture` | Records one receipt after a completed use of another skill, without diagnosing or changing it |
| `skill-evolution` | Evidence-gated revision: hard-refuses unless the derived gate authorizes it, lands only a blind-validated candidate |
| `skill-evolution-status` | Read-only readiness census across the repository's gated skills |
| `method-gap-research-status` | Read-only census recommending one skill, or none, for method-gap research |

## Using it standalone

```console
cargo install skill-evidence
skill-evidence skills evidence install --root .
```

That writes the four packages under `.claude/skills/`, their `.agents/skills/` discovery links, and
two versioned schemas under `schemas/skill-evidence/`. It **refuses rather than clobbering** a file
that differs from what it ships, naming every one and writing nothing; `--force` replaces them.
Running it without `--force` is therefore a free preview of what an upgrade would change.

## Mounting it in your own CLI

Most hosts want the lifecycle under their own binary name. Provide four names once:

```rust
use std::path::PathBuf;
use skill_evidence::Host;

fn host() -> Host {
    Host {
        // Prefixes every schema identity this crate emits.
        namespace: "my-project".to_owned(),
        // The binary an operator types.
        command: "my-project".to_owned(),
        // The Cargo package that provides it, for `cargo run -p`.
        cargo_package: "my-project-cli".to_owned(),
        // Where *this* repository keeps its own skill packages, used to refuse
        // a workflow that targets itself. Resolve it from the host's own
        // `CARGO_MANIFEST_DIR`, never from the audited `--root`.
        skills_directory: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(std::path::Path::parent)
            .expect("CLI crate is two levels below the repository root")
            .join(".claude/skills"),
    }
}
```

Then hang `cli::SkillsArgs` wherever your command tree wants it:

```rust
#[derive(clap::Subcommand)]
enum Command {
    Skills(skill_evidence::cli::SkillsArgs),
    // …your own commands
}

let exit = match command {
    Command::Skills(args) => skill_evidence::cli::run(args, &host(), &mut out, &mut err),
    // …
};
```

`cli::Exit` reports meaning and the host maps it onto process exit codes. **The three meanings are
contract, not convention:** `0` success, `1` unsafe failure, `3` refusal. A refusal is the system
working — it means authority was absent and nothing was written.

`src/bin/skill-evidence.rs` is a complete minimal host and doubles as the worked example.

## Command surface

```
skills evidence     derive | hash | record | install
skills evolution    preflight | claim | record-validation | land | close
skills evolution-status
skills method-gap-research-status <family-selector>
```

## Library only

```toml
skill-evidence = { version = "0.1", default-features = false }
```

Drops the `cli` feature, and with it `clap` and `uuid`. The lifecycle API and the installer remain.

## Versioning and compatibility

Three surfaces reach a consumer independently, and Cargo's SemVer protects only the first:

- **the Rust API** — ordinary SemVer, breakage is loud and at compile time;
- **the installed assets** under `.claude/skills/` and `schemas/` — recoverable, but the installer
  has no uninstall, so a package retired upstream strands its directory in your repository;
- **the recorded evidence** in `events.jsonl` — **not recoverable.** Pinning an older version rolls
  back the reader, not the history that is already written to disk.

Because of the third, changes to a recorded event's shape are additive and optional or they take a
new schema version whose reader accepts both. A frozen corpus covering every v1 event type is
replayed and schema-validated on every run of the test suite.

Full rules: [consumer-contract.md](https://github.com/joeloverbeck/skill-evidence/blob/main/docs/principles/consumer-contract.md).
Release and upgrade procedure: [releasing.md](https://github.com/joeloverbeck/skill-evidence/blob/main/docs/releasing.md).

## Status and expectations

This crate was extracted for use across a small number of repositories and is published so they can
depend on it normally. It is `0.x` and the recorded-event shape is still moving; `1.0.0` will mean
that has stopped, not that a milestone was reached.

Being honest about what is not on offer, per the
[consumer contract](https://github.com/joeloverbeck/skill-evidence/blob/main/docs/principles/consumer-contract.md):
no deprecation window before a removal, no changelog as an obligation, no support commitment, no
backports. Breaking changes do take a version bump Cargo will honor, and the published schemas and
event shape do follow the forward-only rule above. Issues and observations are welcome; work
motivated by an outside request is not promised.

## License

MIT. See [LICENSE](LICENSE).

## Working on this crate

```console
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
```

The compiled-CLI suites spawn the reference binary rather than calling `run`, so they hold the
exit-code and emitted-byte contracts against a separately compiled process. See
`tests/support/mod.rs` in the repository for why the working directory they run from is
load-bearing.

Governance, compatibility rules, and the vocabulary this crate uses live in
[docs/principles/](https://github.com/joeloverbeck/skill-evidence/blob/main/docs/principles/) and
[CONTEXT.md](https://github.com/joeloverbeck/skill-evidence/blob/main/CONTEXT.md). They are adopted,
not aspirational — a change that contradicts them is a change to them first.

Everything lives in one crate on purpose: the machinery, the command surface, and the Markdown an
agent actually reads are one contract, and splitting them produces installations where the engine
and the driver disagree. The crate was extracted after the same ~13,000 lines were copied by hand
into a second repository, six lines apart.
