# skill-evidence

The skill-evidence lifecycle, as a crate: use records, gate projections,
evolution reviews, legacy decontamination, and the four skill packages that
drive them.

A repository that works this way needs all of it — the machinery, the command
surface, and the Markdown packages an agent actually reads. Keeping them in one
place is the point; this crate was extracted after the same ~13,000 lines were
copied by hand into a second repository, six lines apart.

## What a host provides

Four names, once:

```rust
use std::path::PathBuf;
use skill_evidence::Host;

fn host() -> Host {
    Host {
        // Prefixes every schema identity this crate emits.
        namespace: "playbench".to_owned(),
        // The binary an operator types.
        command: "playbench".to_owned(),
        // The Cargo package that provides it, for `cargo run -p`.
        cargo_package: "playbench-cli".to_owned(),
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

## Mounting the command surface

Hang `cli::SkillsArgs` wherever the host's own command tree wants it:

```rust
#[derive(clap::Subcommand)]
enum Command {
    Skills(skill_evidence::cli::SkillsArgs),
    // …the host's own commands
}

let exit = match command {
    Command::Skills(args) => skill_evidence::cli::run(args, &host(), &mut out, &mut err),
    // …
};
```

`cli::Exit` reports meaning; the host maps it onto process exit codes. Both
current hosts use `0` / `1` / `3` for success, unsafe failure, and refusal.

`src/bin/skill-evidence.rs` is the whole of a minimal host and doubles as the
worked example.

## Installing the skill packages

```console
<host-command> skills evidence install --root .
```

Writes the four packages under `.claude/skills/`, their `.agents/skills/`
discovery links, and the two versioned contracts under
`schemas/skill-evidence/`, with the host's own names substituted in. Refuses
rather than clobbering a file that has been edited locally; `--force` replaces
it.

## Using it without a host binary

```console
cargo install --git https://github.com/joeloverbeck/skill-evidence
skill-evidence skills evidence install --root .
```

## Library only

```toml
skill-evidence = { git = "…", tag = "…", default-features = false }
```

Drops `clap` and `uuid`, and with them `cli`. The lifecycle API and the
installer remain.

## Verification

```console
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
```

The compiled-CLI suites spawn the reference binary rather than calling `run`, so
they hold the exit-code and emitted-byte contracts against a separately compiled
process. See `tests/support/mod.rs` for why the working directory they run from
is load-bearing.
