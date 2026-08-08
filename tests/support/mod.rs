//! Shared fixtures for the suites here.
//!
//! Each integration-test binary compiles this module separately and uses the
//! subset it needs, so items unused by any one suite are expected here rather
//! than dead.
#![allow(dead_code)]

use std::{path::PathBuf, process::Command};

use skill_evidence::Host;

/// This repository's root.
///
/// The crate sits at the root here, unlike in a workspace host where it is
/// nested under `crates/`.
pub fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// A command that spawns the reference binary.
///
/// The compiled-CLI suites reach the surface this way on purpose. They assert
/// exit codes and emitted bytes *absolutely*, against a separately compiled
/// process — a suite that called `cli::run` directly would move with any change
/// that moved both sides identically, and would stop holding the exit-code
/// contract still.
///
/// The working directory is deliberately left alone. Cargo runs these from the
/// package root, which here is the repository root and does hold
/// `.claude/skills/` — so every suite that passes `--root <fixture> --target
/// .claude/skills/<name>` is, incidentally, exercising the precedence that
/// `target_resolution.rs` pins: the named repository wins.
///
/// Pinning the directory instead would hide that, and hiding it is how the
/// inverted precedence survived this long.
pub fn skill_evidence() -> Command {
    Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
}

/// The identity the reference binary declares, mirrored for the library suites.
///
/// Kept in step with `src/bin/skill-evidence.rs` by
/// `reference_binary_and_test_host_agree_on_identity` in `host_identity.rs`,
/// so a suite that asserts an emitted schema string is asserting the same host
/// the binary runs as.
pub fn host() -> Host {
    Host {
        namespace: "skill-evidence".to_owned(),
        command: "skill-evidence".to_owned(),
        cargo_package: "skill-evidence".to_owned(),
        skills_directory: repository_root().join(".claude/skills"),
    }
}
