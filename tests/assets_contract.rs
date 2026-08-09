use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

use skill_evidence::{Host, assets};

const RETIRED_PACKAGE: &str = "legacy-skill-decontamination";
const RETIRED_FILES: &[(&str, &str)] = &[
    (
        "SKILL.md",
        include_str!("../assets/retired-skills/legacy-skill-decontamination/SKILL.md"),
    ),
    (
        "agents/openai.yaml",
        include_str!("../assets/retired-skills/legacy-skill-decontamination/agents/openai.yaml"),
    ),
    (
        "references/eligible-run.md",
        include_str!(
            "../assets/retired-skills/legacy-skill-decontamination/references/eligible-run.md"
        ),
    ),
];

fn host() -> Host {
    Host {
        namespace: "demo".to_owned(),
        command: "demo".to_owned(),
        cargo_package: "demo-cli".to_owned(),
        skills_directory: PathBuf::from("/nonexistent/.claude/skills"),
    }
}

fn rendered_for(template: &str, command: &str, cargo_package: &str, namespace: &str) -> String {
    template
        .replace("{{cargo_package}}", cargo_package)
        .replace("{{command}}", command)
        .replace("{{namespace}}", namespace)
}

fn write_pristine_retired_package(root: &std::path::Path) {
    write_retired_package_for(root, "demo", "demo-cli", "demo");
}

fn write_reference_host_retired_package(root: &std::path::Path) {
    write_retired_package_for(root, "skill-evidence", "skill-evidence", "skill-evidence");
}

fn write_retired_package_for(
    root: &std::path::Path,
    command: &str,
    cargo_package: &str,
    namespace: &str,
) {
    let package = root.join(".claude/skills").join(RETIRED_PACKAGE);
    for (relative_path, template) in RETIRED_FILES {
        let destination = package.join(relative_path);
        fs::create_dir_all(destination.parent().expect("retired file parent"))
            .expect("create retired file parent");
        fs::write(
            destination,
            rendered_for(template, command, cargo_package, namespace),
        )
        .expect("write retired file");
    }

    #[cfg(unix)]
    {
        let links = root.join(".agents/skills");
        fs::create_dir_all(&links).expect("create discovery directory");
        std::os::unix::fs::symlink(
            format!("../../.claude/skills/{RETIRED_PACKAGE}"),
            links.join(RETIRED_PACKAGE),
        )
        .expect("create retired discovery link");
    }
}

fn snapshot_tree(root: &std::path::Path) -> BTreeMap<String, Vec<u8>> {
    let mut snapshot = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let mut entries = fs::read_dir(&directory)
            .expect("read snapshot directory")
            .collect::<Result<Vec<_>, _>>()
            .expect("read snapshot entries");
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let relative = path
                .strip_prefix(root)
                .expect("snapshot path below root")
                .to_string_lossy()
                .replace('\\', "/");
            let metadata = fs::symlink_metadata(&path).expect("snapshot metadata");
            if metadata.file_type().is_dir() {
                snapshot.insert(relative, b"directory".to_vec());
                pending.push(path);
            } else if metadata.file_type().is_symlink() {
                snapshot.insert(
                    relative,
                    format!(
                        "link:{}",
                        fs::read_link(&path).expect("snapshot link").display()
                    )
                    .into_bytes(),
                );
            } else {
                snapshot.insert(relative, fs::read(path).expect("snapshot file"));
            }
        }
    }
    snapshot
}

#[test]
fn installed_skill_evolution_reference_reports_the_close_retirement_reach() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains("Read `retired_from_gate_event_ids` from the close receipt"),
        "the reviewer must read the per-close retirement reach from the irreversible action's receipt"
    );
    assert!(
        reference.contains("state that retirement reach in the user-facing completion"),
        "the receipt needs an operator-visible reader"
    );
    assert!(
        reference.contains("- Retirement reach event IDs:"),
        "the durable review report needs a home for the close's retirement reach"
    );
}

#[test]
fn install_reports_a_retired_package_without_writing_or_removing_it() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let retired_skill = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/SKILL.md");
    fs::create_dir_all(retired_skill.parent().expect("retired package directory"))
        .expect("create retired package directory");
    fs::write(&retired_skill, "consumer-owned sentinel\n").expect("write sentinel");

    let receipt = assets::install(root.path(), &host(), false).expect("install current assets");

    assert_eq!(
        receipt.orphaned_packages,
        vec!["legacy-skill-decontamination".to_owned()]
    );
    assert_eq!(
        fs::read_to_string(&retired_skill).expect("read sentinel after install"),
        "consumer-owned sentinel\n",
        "install must neither write nor remove a retired template"
    );

    let forced = assets::install(root.path(), &host(), true).expect("force install current assets");
    assert_eq!(
        forced.orphaned_packages,
        vec!["legacy-skill-decontamination".to_owned()]
    );
    assert_eq!(
        fs::read_to_string(&retired_skill).expect("read sentinel after forced install"),
        "consumer-owned sentinel\n",
        "install --force must not authorize removal"
    );
}

#[cfg(unix)]
#[test]
fn install_reports_a_retired_package_when_only_its_discovery_link_remains() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let links = root.path().join(".agents/skills");
    fs::create_dir_all(&links).expect("create discovery directory");
    let link = links.join(RETIRED_PACKAGE);
    std::os::unix::fs::symlink(format!("../../.claude/skills/{RETIRED_PACKAGE}"), &link)
        .expect("create retired discovery link");

    let receipt = assets::install(root.path(), &host(), false).expect("install current assets");

    assert_eq!(receipt.orphaned_packages, vec![RETIRED_PACKAGE.to_owned()]);
    assert_eq!(
        fs::read_link(&link).expect("install retained retired link"),
        PathBuf::from(format!("../../.claude/skills/{RETIRED_PACKAGE}"))
    );
}

#[test]
fn withdraw_removes_a_pristine_retired_package_and_reports_every_effect() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_pristine_retired_package(root.path());
    let empty_subdirectory = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/scripts/nested");
    fs::create_dir_all(&empty_subdirectory).expect("create empty retired-package subdirectory");

    let receipt = assets::withdraw(root.path(), &host(), false).expect("withdraw retired package");

    assert_eq!(
        receipt.removed_files,
        vec![
            ".claude/skills/legacy-skill-decontamination/SKILL.md".to_owned(),
            ".claude/skills/legacy-skill-decontamination/agents/openai.yaml".to_owned(),
            ".claude/skills/legacy-skill-decontamination/references/eligible-run.md".to_owned(),
        ]
    );
    assert!(receipt.forced_files.is_empty());
    assert!(receipt.retained.is_empty());
    assert!(
        receipt
            .removed_directories
            .contains(&".claude/skills/legacy-skill-decontamination".to_owned())
    );
    assert!(
        receipt
            .removed_directories
            .contains(&".claude/skills/legacy-skill-decontamination/scripts/nested".to_owned())
    );
    assert!(
        !root
            .path()
            .join(".claude/skills")
            .join(RETIRED_PACKAGE)
            .exists()
    );
    assert!(root.path().join(".claude/skills").is_dir());

    #[cfg(unix)]
    {
        assert_eq!(
            receipt.removed_links,
            vec![".agents/skills/legacy-skill-decontamination".to_owned()]
        );
        assert!(root.path().join(".agents/skills").is_dir());
    }
}

#[test]
fn withdraw_cli_refuses_an_edited_retired_file_without_changing_the_tree() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let edited = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/SKILL.md");
    fs::write(&edited, "consumer edit\n").expect("edit retired file");
    let evidence = root.path().join("reports/skill-evidence/demo/events.jsonl");
    fs::create_dir_all(evidence.parent().expect("evidence directory"))
        .expect("create evidence directory");
    fs::write(&evidence, b"immutable evidence sentinel\n").expect("write evidence sentinel");
    let before = snapshot_tree(root.path());

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("run compiled withdraw command");

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 refusal");
    assert!(stderr.contains(".claude/skills/legacy-skill-decontamination/SKILL.md"));
    assert!(stderr.contains("--force"));
    assert_eq!(snapshot_tree(root.path()), before);
}

#[cfg(unix)]
#[test]
fn withdrawal_unsafe_failure_reports_every_effect_already_applied() {
    use std::os::unix::fs::PermissionsExt;

    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let package = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination");
    let blocked_directory = package.join("agents");
    fs::set_permissions(&blocked_directory, fs::Permissions::from_mode(0o500))
        .expect("make the later removal fail");

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("run compiled withdrawal with a later I/O failure");

    fs::set_permissions(&blocked_directory, fs::Permissions::from_mode(0o700))
        .expect("restore directory permissions for cleanup");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(
        !package.join("SKILL.md").exists(),
        "the first removal happened"
    );
    assert!(
        package.join("agents/openai.yaml").is_file(),
        "the failing removal did not happen"
    );
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 unsafe failure");
    let (_, receipt) = stderr
        .trim_end()
        .split_once(" Partial withdrawal receipt: ")
        .expect("unsafe failure carries an inspectable partial receipt");
    let receipt: serde_json::Value =
        serde_json::from_str(receipt).expect("partial withdrawal receipt JSON");
    assert_eq!(
        receipt["removed_files"],
        serde_json::json!([".claude/skills/legacy-skill-decontamination/SKILL.md"])
    );
    assert_eq!(receipt["removed_links"], serde_json::json!([]));
    assert_eq!(receipt["removed_directories"], serde_json::json!([]));
}

#[test]
fn install_refusal_leaves_the_whole_tree_byte_identical() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("initial install");
    let edited = root.path().join(".claude/skills/skill-evolution/SKILL.md");
    fs::write(&edited, "consumer edit\n").expect("edit installed file");
    fs::create_dir_all(root.path().join("unrelated/empty")).expect("create unrelated directory");
    let before = snapshot_tree(root.path());

    let error = assets::install(root.path(), &host(), false).expect_err("refuse edited file");

    assert_eq!(error.class(), skill_evidence::ErrorClass::Refusal);
    assert_eq!(snapshot_tree(root.path()), before);
}

#[test]
fn withdraw_force_removes_an_edited_retired_file_and_names_it() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let edited_path = ".claude/skills/legacy-skill-decontamination/SKILL.md";
    fs::write(root.path().join(edited_path), "consumer edit\n").expect("edit retired file");

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .arg("--force")
        .output()
        .expect("run compiled forced withdrawal");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let receipt: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("withdrawal receipt JSON");
    assert_eq!(receipt["forced_files"], serde_json::json!([edited_path]));
    assert!(
        receipt["removed_files"]
            .as_array()
            .expect("removed files")
            .contains(&serde_json::json!(edited_path))
    );
    assert!(!root.path().join(edited_path).exists());
}

fn assert_foreign_file_is_retained(force: bool) {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_pristine_retired_package(root.path());
    let foreign_path = ".claude/skills/legacy-skill-decontamination/consumer-owned/notes.txt";
    let foreign = root.path().join(foreign_path);
    fs::create_dir_all(foreign.parent().expect("foreign file parent"))
        .expect("create foreign file parent");
    fs::write(&foreign, "do not remove\n").expect("write foreign file");

    let receipt = assets::withdraw(root.path(), &host(), force).expect("withdraw shipped files");

    assert_eq!(
        fs::read_to_string(&foreign).expect("read foreign file"),
        "do not remove\n"
    );
    assert!(
        root.path()
            .join(".claude/skills/legacy-skill-decontamination")
            .is_dir()
    );
    assert_eq!(
        receipt.retained,
        vec![assets::RetainedPath {
            path: foreign_path.to_owned(),
            reason: "path was not shipped by this crate".to_owned(),
        }]
    );
}

#[test]
fn withdraw_retains_a_foreign_file_without_force() {
    assert_foreign_file_is_retained(false);
}

#[test]
fn withdraw_force_does_not_remove_a_foreign_file() {
    assert_foreign_file_is_retained(true);
}

#[cfg(unix)]
#[test]
fn withdraw_retains_a_discovery_link_that_points_elsewhere() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_pristine_retired_package(root.path());
    let link = root
        .path()
        .join(".agents/skills/legacy-skill-decontamination");
    fs::remove_file(&link).expect("remove expected link");
    std::os::unix::fs::symlink("../../somewhere-else", &link).expect("create foreign link");

    let receipt = assets::withdraw(root.path(), &host(), false).expect("withdraw package files");

    assert_eq!(
        fs::read_link(&link).expect("retained alternate link"),
        PathBuf::from("../../somewhere-else")
    );
    assert_eq!(
        receipt.retained,
        vec![assets::RetainedPath {
            path: ".agents/skills/legacy-skill-decontamination".to_owned(),
            reason: "discovery link points to ../../somewhere-else instead of ../../.claude/skills/legacy-skill-decontamination".to_owned(),
        }]
    );
}

#[cfg(unix)]
#[test]
fn withdraw_does_not_follow_a_retired_package_symlink() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let outside = root.path().join("consumer-owned-package");
    for (relative_path, template) in RETIRED_FILES {
        let destination = outside.join(relative_path);
        fs::create_dir_all(destination.parent().expect("outside file parent"))
            .expect("create outside file parent");
        fs::write(
            destination,
            rendered_for(template, "demo", "demo-cli", "demo"),
        )
        .expect("write outside file");
    }
    let package_parent = root.path().join(".claude/skills");
    fs::create_dir_all(&package_parent).expect("create package parent");
    let package_link = package_parent.join(RETIRED_PACKAGE);
    std::os::unix::fs::symlink("../../consumer-owned-package", &package_link)
        .expect("create package symlink");
    let before = snapshot_tree(&outside);

    let receipt = assets::withdraw(root.path(), &host(), false).expect("retain package symlink");

    assert_eq!(snapshot_tree(&outside), before);
    assert!(receipt.removed_files.is_empty());
    assert_eq!(
        receipt.retained,
        vec![assets::RetainedPath {
            path: ".claude/skills/legacy-skill-decontamination".to_owned(),
            reason: "retired package path is not a directory created by this crate".to_owned(),
        }]
    );
}

#[test]
fn withdraw_is_idempotent() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let first = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("first compiled withdrawal");
    assert_eq!(first.status.code(), Some(0));

    let second = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("second compiled withdrawal");

    assert_eq!(second.status.code(), Some(0));
    assert!(second.stderr.is_empty());
    let second: serde_json::Value =
        serde_json::from_slice(&second.stdout).expect("second withdrawal receipt JSON");
    assert_eq!(
        second,
        serde_json::json!({
            "schema_version": 1,
            "removed_files": [],
            "forced_files": [],
            "removed_directories": [],
            "removed_links": [],
            "retained": [],
        })
    );
}

#[test]
fn withdraw_is_a_no_op_when_no_retired_package_was_ever_installed() {
    let root = tempfile::tempdir().expect("temporary repository root");

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("compiled empty withdrawal");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let receipt: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("empty withdrawal receipt JSON");
    assert_eq!(receipt["removed_files"], serde_json::json!([]));
    assert_eq!(receipt["forced_files"], serde_json::json!([]));
    assert_eq!(receipt["removed_directories"], serde_json::json!([]));
    assert_eq!(receipt["removed_links"], serde_json::json!([]));
    assert_eq!(receipt["retained"], serde_json::json!([]));
}

#[test]
fn asset_operations_never_change_recorded_evidence() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let evidence = root.path().join("reports/skill-evidence/demo/events.jsonl");
    fs::create_dir_all(evidence.parent().expect("evidence directory"))
        .expect("create evidence directory");
    fs::write(&evidence, b"immutable evidence sentinel\n").expect("write evidence sentinel");
    let expected = fs::read(&evidence).expect("read evidence before asset operations");

    assets::install(root.path(), &host(), false).expect("install");
    assets::install(root.path(), &host(), true).expect("force install");
    write_pristine_retired_package(root.path());
    assets::withdraw(root.path(), &host(), false).expect("withdraw");
    write_pristine_retired_package(root.path());
    let edited = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/SKILL.md");
    fs::write(&edited, "consumer edit\n").expect("edit retired file");
    assets::withdraw(root.path(), &host(), true).expect("force withdraw");

    assert_eq!(
        fs::read(&evidence).expect("read evidence afterward"),
        expected
    );
}
