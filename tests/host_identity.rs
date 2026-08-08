//! What the reference binary says it is.
//!
//! Every other suite here assumes the binary it spawns and the [`Host`] the
//! library suites construct are the same repository. Nothing enforces that:
//! `src/bin/skill-evidence.rs` builds its own value and keeps it private, so
//! the two could drift silently and every emitted-schema assertion would
//! quietly be checking a host nobody runs.
//!
//! These tests close that by observation rather than by structure — they read
//! what the binary actually emits and hold it to what `support::host()` says.

#![forbid(unsafe_code)]

mod support;

use std::fs;

use serde_json::Value;
use support::{host, repository_root, skill_evidence};
use tempfile::TempDir;

fn repository_with_demo_skill() -> TempDir {
    let fixture = tempfile::tempdir().expect("temporary repository root");
    let target = fixture.path().join(".claude/skills/demo-skill");
    fs::create_dir_all(&target).expect("create demo skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: demo-skill\n---\nDemo body.\n",
    )
    .expect("write demo skill");
    fixture
}

#[test]
fn the_reference_binary_emits_the_namespace_the_library_suites_assume() {
    let fixture = repository_with_demo_skill();
    let output = skill_evidence()
        .args(["skills", "evidence", "hash", "--root"])
        .arg(fixture.path())
        .args(["--target", ".claude/skills/demo-skill"])
        .output()
        .expect("run hash");
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).expect("hash report");
    assert_eq!(report["schema"], host().hash_schema());
}

#[test]
fn the_reference_binary_installs_packages_naming_its_own_command() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let output = skill_evidence()
        .args(["skills", "evidence", "install", "--root"])
        .arg(root.path())
        .output()
        .expect("run install");
    assert!(output.status.success());

    let capture = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evidence-capture/SKILL.md"),
    )
    .expect("read the installed capture skill");
    assert!(capture.contains(&format!("`{} skills evidence`", host().command)));
    assert!(capture.contains(&format!(
        "cargo run --locked -p {} -- skills evidence record",
        host().cargo_package
    )));
}

/// The self-target refusal reaches for the *host's* skills directory, which is
/// this repository's own. That is the fact `Host::skills_directory` exists to
/// carry, and the one an extraction into a dependency would otherwise have
/// broken silently: resolved from the crate's own source location inside a
/// cargo checkout, the path would not exist, `canonicalize` would fail, and the
/// refusal would never fire.
#[test]
fn the_reference_binarys_own_skill_package_is_a_self_target() {
    let fixture = repository_with_demo_skill();
    let self_target = repository_root().join(".claude/skills/skill-evolution");
    assert!(
        self_target.is_dir(),
        "this repository installs its own packages; the guard has nothing to catch without them"
    );

    let output = skill_evidence()
        .args(["skills", "evolution", "preflight", "--root"])
        .arg(fixture.path())
        .arg("--target")
        .arg(&self_target)
        .args([
            "--recorded-at",
            "2026-01-02T03:04:05.678Z",
            "--now-epoch-milliseconds",
            "1767320645678",
            "--session-id",
            "session_identity",
            "--lock-owner",
            "lock_identity",
        ])
        .output()
        .expect("run self-target preflight");

    assert_eq!(output.status.code(), Some(3));
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("operator_skill_path != target_skill_path"));
    assert!(error.contains("Terminal outcome: refused_self_target."));
}
