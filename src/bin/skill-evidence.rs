//! The reference host.
//!
//! Two things need this binary. The compiled-CLI suites need a process to spawn
//! that asserts the surface's bytes and statuses absolutely, against something
//! separately compiled rather than against the same `run` they call. And a
//! repository with no Rust of its own — or one that has not yet mounted the
//! subcommand into its own binary — needs a way to drive the lifecycle at all.
//!
//! It is deliberately thin. Everything a host would write for itself is here
//! and nothing else, so it doubles as the worked example of mounting
//! [`skill_evidence::cli`].

#![forbid(unsafe_code)]

use std::{
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
};

use clap::{Parser, Subcommand};
use skill_evidence::{Host, cli};

#[derive(Debug, Parser)]
#[command(name = "skill-evidence")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Skills(cli::SkillsArgs),
}

fn main() -> ExitCode {
    let parsed = Cli::parse();
    let mut out = io::stdout().lock();
    let mut err = io::stderr().lock();
    let exit = match parsed.command {
        Command::Skills(args) => cli::run(args, &host(), &mut out, &mut err),
    };
    out.flush().ok();
    err.flush().ok();
    exit_code(exit)
}

/// This binary's own identity.
///
/// `skills_directory` resolves from `CARGO_MANIFEST_DIR` because that is where
/// a host's own packages live relative to its own source — the same resolution
/// every mounting host performs, differing only in how many levels up the
/// repository root sits.
fn host() -> Host {
    Host {
        namespace: "skill-evidence".to_owned(),
        command: "skill-evidence".to_owned(),
        cargo_package: "skill-evidence".to_owned(),
        skills_directory: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".claude/skills"),
    }
}

/// The process status each outcome reports.
///
/// These three numbers are the contract every current host shares; a host that
/// wanted different ones would write this function differently and nothing else.
fn exit_code(exit: cli::Exit) -> ExitCode {
    match exit {
        cli::Exit::Success => ExitCode::SUCCESS,
        cli::Exit::UnsafeFailure => ExitCode::from(1),
        cli::Exit::Refusal => ExitCode::from(3),
    }
}
