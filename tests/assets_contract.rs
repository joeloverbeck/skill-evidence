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
fn installed_skill_evolution_reference_writes_the_report_before_close_and_amends_it_after() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let write = reference
        .find("Before any close, write the review report")
        .expect("the report is written before close");
    let close = reference
        .find("skills evolution close")
        .expect("the compiled close command remains explicit");
    let amend = reference
        .find("After the close succeeds, amend the review report")
        .expect("the report is amended from the close receipt");
    assert!(write < close && close < amend, "reference={reference}");
    assert!(
        reference.contains(
            "--trials <count> --artifacts reports/skill-evidence/<skill-key>/reviews/<review-id>"
        ),
        "a no-candidate validation arm must carry its asserted effort into close"
    );
}

#[test]
fn installed_skill_evolution_reference_weighs_trigger_workarounds_before_freezing_the_plan() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let ownership = reference
        .find("### 3. Determine target ownership and causal mechanism")
        .expect("the ownership step remains explicit");
    let workaround = reference
        .find("read `workaround_taken` only from the raw trigger events in the evidence packet")
        .expect("the ownership step reads only the workaround evidence the packet carries");
    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    assert!(
        ownership < workaround && workaround < freeze,
        "the workaround finding belongs to ownership determination before plan freezing: reference={reference}"
    );

    for required in [
        "state that none was recorded on a trigger event",
        "Repeated suppression of the mechanism is evidence for target ownership",
        "a workaround that was taken without suppressing the mechanism is evidence against target ownership",
        "Record the direction as evidence, never as a verdict",
        "count the open incident IDs outside the trigger set and state that count, including zero",
        "Do not characterize, estimate, or reason about those incidents",
        "do not read the historical ledger or seek their payloads",
        "Reconcile the frozen plan with step 3's workaround finding",
        "without letting it replace a trial or skip or shrink the frozen trial set",
        "- Recorded-workaround finding:",
        "- Non-trigger open incident count:",
        "acceptance decision is made from the trial results alone",
    ] {
        assert!(
            reference.contains(required),
            "installed reference must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_requires_per_mechanism_trials() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let ownership = reference
        .find("### 3. Determine target ownership and causal mechanism")
        .expect("the ownership step remains explicit");
    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let validation = reference
        .find("### 6. Run blind comparative validation")
        .expect("the candidate-arm step remains explicit");
    let report = reference
        .find("# Skill Evolution Review: <skill-name>")
        .expect("the review report template remains explicit");

    for (required, section) in [
        (
            "Name one candidate mechanism for each trigger event, or explicitly group several trigger events under one shared mechanism and state why they share it.",
            &reference[ownership..freeze],
        ),
        (
            "Freeze one reproduction trial per distinct mechanism, each with its own trigger event IDs, witness, unexpressed reading, and incidence-sized run count.",
            &reference[freeze..current_arm],
        ),
        (
            "The current arm is the union of the reproduction trials; if their results disagree, proceed only on each mechanism that reproduced and report every mechanism as reproduced, not reproduced with witnesses expressed, or unable to be expressed.",
            &reference[current_arm..validation],
        ),
        (
            "Before any candidate output exists, freeze whether a candidate-arm run whose witness reads unexpressed is discounted from the comparison or replaced.",
            &reference[freeze..current_arm],
        ),
    ] {
        assert!(
            section.contains(required),
            "installed reference must preserve `{required}` in its governing step: reference={reference}"
        );
    }

    assert_eq!(
        reference[report..]
            .matches("- Trigger event → reproduction trial → witness reading:")
            .count(),
        2,
        "both evidence adjudication and results must carry the per-mechanism mapping: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_reserves_blocked_for_the_whole_review() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "When only some mechanisms are untestable, mark each as unable to be expressed in the plan and proceed with the runnable reproduction trials.",
        "`blocked_no_valid_test` remains a whole-review disposition; do not assign it to an individual mechanism.",
    ] {
        assert!(
            plan.contains(required),
            "installed reference must preserve `{required}` in the frozen-plan contract: reference={reference}"
        );
    }
}

#[test]
fn installed_event_schema_declares_close_validation_effort_as_optional() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let schema: serde_json::Value = serde_json::from_slice(
        &fs::read(
            root.path()
                .join("schemas/skill-evidence/event.v1.schema.json"),
        )
        .expect("read installed event schema"),
    )
    .expect("installed event schema JSON");
    let payload = schema
        .pointer("/allOf/1/then/properties/payload")
        .expect("review_disposition payload schema");
    assert_eq!(payload["properties"]["trial_count"]["type"], "integer");
    assert_eq!(payload["properties"]["artifacts_path"]["type"], "string");
    let required = payload["required"]
        .as_array()
        .expect("required review_disposition properties");
    assert!(!required.iter().any(|field| field == "trial_count"));
    assert!(!required.iter().any(|field| field == "artifacts_path"));
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
fn installed_skill_evolution_reference_requires_pre_close_reach_bound_review() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "Before a `blocked_no_valid_test` close, read the authorization reason and coverage list from the claim receipt"
        ),
        "the immutable claim receipt must supply both inputs to the reason-specific reach bound"
    );
    assert!(
        reference.contains(
            "Re-evaluate that authorization reason against the live candidate clusters in the gate projection"
        ),
        "the live projection must supply the reason-specific reach bound"
    );
    assert!(
        reference.contains(
            "`material_recurrence` names only material-or-worse incidents in its symptom cluster"
        ),
        "the vouch must not silently widen a material authorization to friction siblings"
    );
    assert!(
        reference.contains(
            "The projection is current as of the last recorded incident, so no extra derive run is required"
        ),
        "the reviewer must not invent an unnecessary pre-close command"
    );
    assert!(
        reference.contains(
            "This reason-specific reach bound can name incidents the close will not retire: a contemporaneous severe incident is never retired because it authorizes on its own"
        ),
        "the safely over-stated bound must not be presented as an exact preview"
    );
}

#[test]
fn installed_skill_evolution_reference_ties_mismatch_disclosure_to_unvouched_sibling() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "Vouch only for incidents inside that reason-specific reach bound; same-symptom incidents outside it do not affect this close"
        ),
        "the reviewer must not import symptom-wide vouching into the narrowed rule"
    );
}

#[test]
fn installed_skill_evolution_status_describes_reason_scoped_retirement() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let status_skill = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution-status/SKILL.md"),
    )
    .expect("read installed skill-evolution-status package");

    assert!(
        status_skill.contains(
            "Each instrument-limited close contributes only the open incidents its recorded authorization reason named at the close"
        ),
        "the standing set must be traced to reason-scoped per-close reaches"
    );
    assert!(
        status_skill
            .contains("`material_recurrence` leaves friction siblings outside the retired set"),
        "status readers must not infer the former symptom-wide material reach"
    );
    assert!(
        status_skill.contains("`severe` contributes no retired incidents"),
        "an empty severe reach must be legible at the standing-set surface"
    );
    assert!(
        status_skill.contains(
            "a missing or unrecognized authorizing rule uses the prior symptom-wide reach"
        ),
        "historical fallback must be visible to status readers"
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
