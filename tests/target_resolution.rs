//! Which skill a relative `--target` names.
//!
//! Both the repository under `--root` and the working directory can hold a
//! `.claude/skills/<name>`, and until this suite existed nothing said which one
//! won. The answer was the working directory, inherited from the JavaScript
//! helper this crate was ported from, documented nowhere, and pinned by no test
//! — which is how it survived long enough to be found by an extraction rather
//! than by a failure.
//!
//! It is the repository now, and these hold it there.

#![forbid(unsafe_code)]

mod support;

use std::{fs, path::Path};

use serde_json::Value;
use support::skill_evidence;
use tempfile::TempDir;

/// A repository root holding one skill whose `SKILL.md` says `marker`.
fn repository_with(marker: &str, file_count: usize) -> TempDir {
    let root = tempfile::tempdir().expect("temporary repository root");
    let target = root.path().join(".claude/skills/demo-skill");
    fs::create_dir_all(&target).expect("create demo skill");
    fs::write(
        target.join("SKILL.md"),
        format!("---\nname: demo-skill\n---\n{marker}\n"),
    )
    .expect("write demo skill");
    // A second file distinguishes the two trees in the hash report, so a test
    // can say which one was read without depending on the digest itself.
    for extra in 1..file_count {
        fs::write(target.join(format!("extra-{extra}.md")), "padding\n").expect("write padding");
    }
    root
}

fn hash_from(working_directory: &Path, root: &Path) -> Value {
    let output = skill_evidence()
        .current_dir(working_directory)
        .args(["skills", "evidence", "hash", "--root"])
        .arg(root)
        .args(["--target", ".claude/skills/demo-skill"])
        .output()
        .expect("run hash");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("hash report")
}

/// The failure this suite exists for: two trees, both holding the named path,
/// and the operator said which one with `--root`.
#[test]
fn the_named_repository_wins_over_the_working_directory() {
    let named = repository_with("The repository the operator named.", 1);
    let bystander = repository_with("A tree that merely happens to be the cwd.", 3);

    let report = hash_from(bystander.path(), named.path());
    assert_eq!(
        report["file_count"], 1,
        "hashing read the working directory's copy, not the one --root named"
    );
}

/// The working directory is still a fallback, so a path meaningful only from
/// where the operator stands keeps resolving. Dropping the candidate would have
/// been a smaller change and a worse one.
#[test]
fn the_working_directory_still_resolves_what_the_root_cannot() {
    let named = tempfile::tempdir().expect("root without the skill");
    let elsewhere = repository_with("Only reachable from the working directory.", 2);

    let report = hash_from(elsewhere.path(), named.path());
    assert_eq!(report["file_count"], 2);
}

/// With no `--root`, the root is the git toplevel the working directory sits
/// inside, so the two candidates name the same tree and a subdirectory can
/// still spell the target from the root.
#[test]
fn a_subdirectory_resolves_a_root_relative_target_with_no_root_flag() {
    let fixture = repository_with("Resolved from a nested working directory.", 1);
    let git = std::process::Command::new("git")
        .args(["init", "--quiet"])
        .arg(fixture.path())
        .output()
        .expect("initialize fixture repository");
    assert!(git.status.success());
    let nested = fixture.path().join("nested/work");
    fs::create_dir_all(&nested).expect("create nested working directory");

    let output = skill_evidence()
        .current_dir(&nested)
        .args([
            "skills",
            "evidence",
            "hash",
            "--target",
            ".claude/skills/demo-skill",
        ])
        .output()
        .expect("run hash from a nested directory");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).expect("hash report");
    assert_eq!(report["file_count"], 1);
}
