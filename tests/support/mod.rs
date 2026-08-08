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
/// The working directory is load-bearing, not tidiness. `resolve_target` tries
/// a relative `--target` against the current directory *before* `--root`, so a
/// suite that runs from a directory holding `.claude/skills/` silently audits
/// this repository instead of its fixture. That was invisible while these
/// suites lived in a nested CLI package whose own directory had no skills; here
/// the package root is the repository root, and it is not invisible at all.
/// `tests/` has no `.claude/skills/`, so the root-relative candidate wins,
/// which is the resolution these suites have always been exercising.
///
/// Cases that are *about* current-directory behavior override this with their
/// own `current_dir`.
pub fn skill_evidence() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_skill-evidence"));
    command.current_dir(repository_root().join("tests"));
    command
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
