#![forbid(unsafe_code)]

mod support;

use std::{fs, path::Path, process::Command};

use serde_json::Value;
use support::{repository_root, skill_evidence};
use tempfile::TempDir;

fn repository_with_demo_skill() -> TempDir {
    let fixture = tempfile::tempdir().expect("temporary repository root");
    let target = fixture.path().join(".claude/skills/demo-skill");
    fs::create_dir_all(&target).expect("create demo skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: demo-skill\n---\nDemo body v1.\n",
    )
    .expect("write demo skill");
    fixture
}

fn lifecycle_clock(command: &mut Command, owner: &str) {
    lifecycle_clock_for_session(command, owner, "fixture-session");
}

fn lifecycle_clock_for_session(command: &mut Command, owner: &str, session: &str) {
    command.args([
        "--recorded-at",
        "2026-01-02T03:04:05.000Z",
        "--now-epoch-milliseconds",
        "1767323045000",
        "--session-id",
        session,
        "--lock-owner",
        owner,
    ]);
}

fn lifecycle_arguments(
    command: &mut Command,
    root: &Path,
    target: &Path,
    event: bool,
    omitted: Option<&str>,
) {
    command.arg("--root").arg(root).arg("--target").arg(target);
    for (flag, value) in [
        ("--recorded-at", "2026-01-02T03:04:05.000Z"),
        ("--now-epoch-milliseconds", "1767323045000"),
        ("--session-id", "fixture-session"),
        ("--lock-owner", "lock_lifecycle_contract"),
    ] {
        if omitted != Some(flag) {
            command.args([flag, value]);
        }
    }
    if event {
        for (flag, value) in [
            ("--event-id", "evt_lifecycle_contract"),
            ("--repository-head", "fixture-head"),
        ] {
            if omitted != Some(flag) {
                command.args([flag, value]);
            }
        }
    }
}

fn run_evolution_preflight(root: &Path, owner: &str, session: &str) -> std::process::Output {
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "preflight", "--root"])
        .arg(root)
        .args(["--target", ".claude/skills/demo-skill"]);
    lifecycle_clock_for_session(&mut command, owner, session);
    command.output().expect("run Skill Evolution preflight")
}

fn run_decontamination_preflight(
    root: &Path,
    owner: &str,
    basis: Option<&str>,
    basis_ref: Option<&str>,
    basis_note: Option<&str>,
) -> std::process::Output {
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "preflight", "--root"])
        .arg(root)
        .args(["--target", ".claude/skills/demo-skill"]);
    if let Some(basis) = basis {
        command.args(["--basis", basis]);
    }
    if let Some(basis_ref) = basis_ref {
        command.args(["--basis-ref", basis_ref]);
    }
    if let Some(basis_note) = basis_note {
        command.args(["--basis-note", basis_note]);
    }
    lifecycle_clock(&mut command, owner);
    command
        .output()
        .expect("run Legacy Skill Decontamination preflight")
}

fn gate(root: &Path) -> Value {
    serde_json::from_slice(
        &fs::read(root.join("reports/skill-evidence/demo-skill/gate-status.json"))
            .expect("read gate projection"),
    )
    .expect("gate projection JSON")
}

fn assert_event_stream_matches_the_published_schema(root: &Path) {
    let schema: Value = serde_json::from_slice(
        &fs::read(repository_root().join("schemas/skill-evidence/event.v1.schema.json"))
            .expect("read event schema"),
    )
    .expect("event schema JSON");
    let validator = jsonschema::validator_for(&schema).expect("compile event schema");
    let stream = fs::read_to_string(root.join("reports/skill-evidence/demo-skill/events.jsonl"))
        .expect("read lifecycle event stream");
    for (index, line) in stream.lines().enumerate() {
        let event: Value = serde_json::from_str(line).expect("lifecycle event JSON");
        assert!(
            validator.is_valid(&event),
            "event line {} must match the published v1 schema: {}",
            index + 1,
            line
        );
    }
}

fn record_incident(root: &Path, label: &str, session: &str) {
    record_outcome(root, label, session, "execution", "friction");
}

fn record_outcome(root: &Path, label: &str, session: &str, symptom: &str, outcome: &str) {
    let output = skill_evidence()
        .args(["skills", "evidence", "record", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            outcome,
            "--task-label",
            label,
            "--symptom-key",
            symptom,
            "--expected",
            "expected",
            "--observed",
            "observed",
            "--consequence",
            "consequence",
            "--run-condition",
            "fixture incident",
            "--evidence-ref",
            "logs/fixture.txt",
            "--session-id",
            session,
        ])
        .output()
        .expect("record fixture incident");
    assert!(
        output.status.success(),
        "record failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn claim_evolution(root: &Path) -> Value {
    record_incident(root, "task a", "session-a");
    record_incident(root, "task b", "session-b");
    record_incident(root, "task c", "session-c");
    claim_existing_evolution(root)
}

fn claim_existing_evolution(root: &Path) -> Value {
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "claim", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_evolution_claim",
            "--review-id",
            "rev_fixture",
            "--repository-head",
            "fixture-head",
        ]);
    lifecycle_clock(&mut command, "lock_evolution_claim");
    let output = command.output().expect("claim Skill Evolution review");
    assert!(
        output.status.success(),
        "claim failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("claim receipt JSON")
}

fn make_candidate(root: &Path, body: &str) -> std::path::PathBuf {
    let candidate = root.join("reports/skill-evidence/demo-skill/reviews/candidate");
    fs::create_dir_all(&candidate).expect("create candidate");
    fs::write(
        candidate.join("SKILL.md"),
        format!("---\nname: demo-skill\n---\n{body}\n"),
    )
    .expect("write candidate skill");
    candidate
}

fn accept_evolution_candidate(root: &Path, candidate: &Path) {
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "record-validation", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_accepted_validation",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--decision",
            "accepted",
            "--risk-tier",
            "ordinary",
        ])
        .arg("--candidate")
        .arg(candidate)
        .args([
            "--trials",
            "3",
            "--artifacts",
            "reports/skill-evidence/demo-skill/reviews/trials",
        ]);
    lifecycle_clock(&mut command, "lock_accepted_validation");
    let output = command.output().expect("accept candidate validation");
    assert!(
        output.status.success(),
        "validation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn land_evolution_candidate(root: &Path) -> Value {
    claim_evolution(root);
    let candidate = make_candidate(root, "Demo body v2 (repaired).");
    accept_evolution_candidate(root, &candidate);
    let output = run_evolution_land(
        root,
        &candidate,
        "evt_evolution_landed",
        "lock_evolution_land",
    );
    assert!(
        output.status.success(),
        "landing failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("landing receipt JSON")
}

fn run_evolution_land(
    root: &Path,
    candidate: &Path,
    event_id: &str,
    lock_owner: &str,
) -> std::process::Output {
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "land", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            event_id,
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
        ])
        .arg("--candidate")
        .arg(candidate);
    lifecycle_clock(&mut command, lock_owner);
    command.output().expect("land evolution candidate")
}

fn run_evolution_close(
    root: &Path,
    event_id: &str,
    disposition: &str,
    note: Option<&str>,
) -> std::process::Output {
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "close", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            event_id,
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--disposition",
            disposition,
        ]);
    if let Some(note) = note {
        command.args(["--note", note]);
    }
    lifecycle_clock(
        &mut command,
        &format!("lock_{}", event_id.replace("evt_", "")),
    );
    command.output().expect("close evolution review")
}

fn claim_decontamination(root: &Path) -> Value {
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "claim", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_claim",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--basis",
            "owner-confirmed",
        ]);
    lifecycle_clock(&mut command, "lock_decontamination_claim");
    let output = command.output().expect("claim decontamination run");
    assert!(
        output.status.success(),
        "claim failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("decontamination claim JSON")
}

fn make_decontamination_candidate(root: &Path) -> std::path::PathBuf {
    let candidate = root.join("reports/skill-evidence/demo-skill/decontamination/candidate");
    fs::create_dir_all(&candidate).expect("create decontamination candidate");
    fs::write(
        candidate.join("SKILL.md"),
        "---\nname: demo-skill\n---\nDemo body v2 (decontaminated).\n",
    )
    .expect("write decontamination candidate");
    candidate
}

fn accept_decontamination_candidate(root: &Path, candidate: &Path) {
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "record-validation", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_validation",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--decision",
            "accepted",
        ])
        .arg("--candidate")
        .arg(candidate)
        .args([
            "--trials",
            "5",
            "--artifacts",
            "reports/skill-evidence/demo-skill/decontamination/trials",
        ]);
    lifecycle_clock(&mut command, "lock_decontamination_validation");
    let output = command.output().expect("accept decontamination candidate");
    assert!(
        output.status.success(),
        "validation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn land_decontamination_candidate(root: &Path) -> Value {
    claim_decontamination(root);
    let candidate = make_decontamination_candidate(root);
    accept_decontamination_candidate(root, &candidate);
    let output = run_decontamination_land(
        root,
        &candidate,
        "evt_decontamination_landed",
        "lock_decontamination_land",
    );
    assert!(
        output.status.success(),
        "landing failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("landing receipt JSON")
}

fn run_decontamination_land(
    root: &Path,
    candidate: &Path,
    event_id: &str,
    lock_owner: &str,
) -> std::process::Output {
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "land", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            event_id,
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
        ])
        .arg("--candidate")
        .arg(candidate);
    lifecycle_clock(&mut command, lock_owner);
    command.output().expect("land decontamination candidate")
}

fn run_decontamination_complete(
    root: &Path,
    event_id: &str,
    outcome: &str,
    note: Option<&str>,
) -> std::process::Output {
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "complete", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            event_id,
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--outcome",
            outcome,
        ]);
    if let Some(note) = note {
        command.args(["--note", note]);
    }
    lifecycle_clock(
        &mut command,
        &format!("lock_{}", event_id.replace("evt_", "")),
    );
    command.output().expect("complete decontamination run")
}

#[test]
fn skill_evolution_preflight_refuses_a_closed_gate_with_the_legacy_contract() {
    let fixture = repository_with_demo_skill();
    let output = run_evolution_preflight(
        fixture.path(),
        "lock_evolution_preflight",
        "fixture-session",
    );

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).expect("UTF-8 refusal"),
        "Skill Evolution not authorized.\n\
Gate: closed.\n\
Failed condition: authorized_workflow == \"skill-evolution\" AND state IN {eligible, quarantined_eligible}.\n\
No target analysis or modification performed.\n\
Terminal outcome: refused_closed_gate.\n"
    );
    let projection = gate(fixture.path());
    assert_eq!(projection["generated_at"], "2026-01-02T03:04:05.000Z");
    assert_eq!(projection["state"], "closed");
    assert!(
        !fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl")
            .exists()
    );
}

#[test]
fn all_ten_lifecycle_commands_refuse_missing_explicit_inputs_with_code_three() {
    let fixture = repository_with_demo_skill();
    let candidate = make_candidate(fixture.path(), "candidate");
    let target = Path::new(".claude/skills/demo-skill");
    let mut cases = Vec::new();

    let mut evolution_preflight = skill_evidence();
    evolution_preflight.args(["skills", "evolution", "preflight"]);
    lifecycle_arguments(
        &mut evolution_preflight,
        fixture.path(),
        target,
        false,
        Some("--session-id"),
    );
    cases.push(("evolution preflight", evolution_preflight, "--session-id"));

    let mut evolution_claim = skill_evidence();
    evolution_claim
        .args(["skills", "evolution", "claim"])
        .args(["--review-id", "rev_fixture"]);
    lifecycle_arguments(
        &mut evolution_claim,
        fixture.path(),
        target,
        true,
        Some("--event-id"),
    );
    cases.push(("evolution claim", evolution_claim, "--event-id"));

    let mut evolution_validation = skill_evidence();
    evolution_validation
        .args(["skills", "evolution", "record-validation"])
        .args([
            "--review-id",
            "rev_fixture",
            "--decision",
            "accepted",
            "--risk-tier",
            "ordinary",
        ])
        .arg("--candidate")
        .arg(&candidate)
        .args(["--trials", "3", "--artifacts", "trials"]);
    lifecycle_arguments(
        &mut evolution_validation,
        fixture.path(),
        target,
        true,
        Some("--repository-head"),
    );
    cases.push((
        "evolution record-validation",
        evolution_validation,
        "--repository-head",
    ));

    let mut evolution_land = skill_evidence();
    evolution_land
        .args(["skills", "evolution", "land"])
        .args(["--review-id", "rev_fixture"])
        .arg("--candidate")
        .arg(&candidate);
    lifecycle_arguments(
        &mut evolution_land,
        fixture.path(),
        target,
        true,
        Some("--recorded-at"),
    );
    cases.push(("evolution land", evolution_land, "--recorded-at"));

    let mut evolution_close = skill_evidence();
    evolution_close
        .args(["skills", "evolution", "close"])
        .args([
            "--review-id",
            "rev_fixture",
            "--disposition",
            "monitor_for_recurrence",
            "--note",
            "rationale",
        ]);
    lifecycle_arguments(
        &mut evolution_close,
        fixture.path(),
        target,
        true,
        Some("--now-epoch-milliseconds"),
    );
    cases.push((
        "evolution close",
        evolution_close,
        "--now-epoch-milliseconds",
    ));

    let mut decontamination_preflight = skill_evidence();
    decontamination_preflight
        .args(["skills", "decontamination", "preflight"])
        .args(["--basis", "owner-confirmed"]);
    lifecycle_arguments(
        &mut decontamination_preflight,
        fixture.path(),
        target,
        false,
        Some("--lock-owner"),
    );
    cases.push((
        "decontamination preflight",
        decontamination_preflight,
        "--lock-owner",
    ));

    let mut decontamination_claim = skill_evidence();
    decontamination_claim
        .args(["skills", "decontamination", "claim"])
        .args(["--run-id", "dec_fixture", "--basis", "owner-confirmed"]);
    lifecycle_arguments(
        &mut decontamination_claim,
        fixture.path(),
        target,
        true,
        Some("--session-id"),
    );
    cases.push((
        "decontamination claim",
        decontamination_claim,
        "--session-id",
    ));

    let mut decontamination_validation = skill_evidence();
    decontamination_validation
        .args(["skills", "decontamination", "record-validation"])
        .args(["--run-id", "dec_fixture", "--decision", "accepted"])
        .arg("--candidate")
        .arg(&candidate)
        .args(["--trials", "5", "--artifacts", "trials"]);
    lifecycle_arguments(
        &mut decontamination_validation,
        fixture.path(),
        target,
        true,
        Some("--event-id"),
    );
    cases.push((
        "decontamination record-validation",
        decontamination_validation,
        "--event-id",
    ));

    let mut decontamination_land = skill_evidence();
    decontamination_land
        .args(["skills", "decontamination", "land"])
        .args(["--run-id", "dec_fixture"])
        .arg("--candidate")
        .arg(&candidate);
    lifecycle_arguments(
        &mut decontamination_land,
        fixture.path(),
        target,
        true,
        Some("--repository-head"),
    );
    cases.push((
        "decontamination land",
        decontamination_land,
        "--repository-head",
    ));

    let mut decontamination_complete = skill_evidence();
    decontamination_complete
        .args(["skills", "decontamination", "complete"])
        .args([
            "--run-id",
            "dec_fixture",
            "--outcome",
            "healthy_no_change",
            "--note",
            "rationale",
        ]);
    lifecycle_arguments(
        &mut decontamination_complete,
        fixture.path(),
        target,
        true,
        Some("--recorded-at"),
    );
    cases.push((
        "decontamination complete",
        decontamination_complete,
        "--recorded-at",
    ));

    for (label, mut command, missing) in cases {
        let output = command.output().expect("run missing explicit input case");
        assert_eq!(output.status.code(), Some(3), "{label}");
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains(&format!("Missing required {missing}.")),
            "{label}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn lifecycle_claim_identities_are_explicit_code_three_refusals() {
    let fixture = repository_with_demo_skill();
    let mut evolution = skill_evidence();
    evolution.args(["skills", "evolution", "claim"]);
    lifecycle_arguments(
        &mut evolution,
        fixture.path(),
        Path::new(".claude/skills/demo-skill"),
        true,
        None,
    );
    let mut decontamination = skill_evidence();
    decontamination
        .args(["skills", "decontamination", "claim"])
        .args(["--basis", "owner-confirmed"]);
    lifecycle_arguments(
        &mut decontamination,
        fixture.path(),
        Path::new(".claude/skills/demo-skill"),
        true,
        None,
    );

    for (label, mut command, expected) in [
        ("evolution", evolution, "Missing required --review-id."),
        (
            "decontamination",
            decontamination,
            "Missing required --run-id.",
        ),
    ] {
        let output = command.output().expect("run missing claim identity");
        assert_eq!(output.status.code(), Some(3), "{label}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected),
            "{label}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn skill_evolution_preflight_refuses_self_target_without_creating_a_store() {
    let fixture = repository_with_demo_skill();
    let self_target = repository_root().join(".claude/skills/skill-evolution");
    let mut self_command = skill_evidence();
    self_command
        .args(["skills", "evolution", "preflight", "--root"])
        .arg(fixture.path())
        .arg("--target")
        .arg(self_target);
    lifecycle_clock(&mut self_command, "lock_evolution_self_target");
    let self_refusal = self_command.output().expect("run self-target preflight");
    assert_eq!(self_refusal.status.code(), Some(3));
    let self_error = String::from_utf8_lossy(&self_refusal.stderr);
    assert!(self_error.contains("operator_skill_path != target_skill_path"));
    assert!(self_error.contains("Terminal outcome: refused_self_target."));
    assert!(
        !fixture
            .path()
            .join("reports/skill-evidence/skill-evolution")
            .exists()
    );
}

#[test]
fn lifecycle_self_target_checks_canonical_paths_instead_of_basenames() {
    let fixture = repository_with_demo_skill();
    for name in ["skill-evolution", "legacy-skill-decontamination"] {
        let package = fixture.path().join(".claude/skills").join(name);
        fs::create_dir_all(&package).expect("create same-basename package");
        fs::write(
            package.join("SKILL.md"),
            format!("---\nname: {name}\n---\nUnrelated package.\n"),
        )
        .expect("write same-basename package");
    }

    let mut evolution_command = skill_evidence();
    evolution_command.args(["skills", "evolution", "preflight"]);
    lifecycle_arguments(
        &mut evolution_command,
        fixture.path(),
        Path::new(".claude/skills/skill-evolution"),
        false,
        None,
    );
    let evolution = evolution_command
        .output()
        .expect("preflight unrelated same-basename package");
    assert_eq!(evolution.status.code(), Some(3));
    let evolution_error = String::from_utf8_lossy(&evolution.stderr);
    assert!(evolution_error.contains("Terminal outcome: refused_closed_gate."));
    assert!(!evolution_error.contains("refused_self_target"));

    let mut decontamination_command = skill_evidence();
    decontamination_command.args(["skills", "decontamination", "preflight"]);
    lifecycle_arguments(
        &mut decontamination_command,
        fixture.path(),
        Path::new(".claude/skills/legacy-skill-decontamination"),
        false,
        None,
    );
    decontamination_command.args(["--basis", "owner-confirmed"]);
    let decontamination = decontamination_command
        .output()
        .expect("preflight unrelated same-basename package");
    assert!(
        decontamination.status.success(),
        "{}",
        String::from_utf8_lossy(&decontamination.stderr)
    );
}

#[test]
fn lifecycle_self_target_refusal_applies_to_all_ten_commands() {
    let fixture = repository_with_demo_skill();
    let candidate = fixture.path().join(".claude/skills/demo-skill");
    let evolution_operator = repository_root().join(".claude/skills/skill-evolution");
    let decontamination_operator =
        repository_root().join(".claude/skills/legacy-skill-decontamination");
    let mut cases = Vec::new();

    let mut evolution_preflight = skill_evidence();
    evolution_preflight.args(["skills", "evolution", "preflight"]);
    lifecycle_arguments(
        &mut evolution_preflight,
        fixture.path(),
        &evolution_operator,
        false,
        None,
    );
    cases.push(("evolution preflight", evolution_preflight));

    let mut evolution_claim = skill_evidence();
    evolution_claim
        .args(["skills", "evolution", "claim"])
        .args(["--review-id", "rev_fixture"]);
    lifecycle_arguments(
        &mut evolution_claim,
        fixture.path(),
        &evolution_operator,
        true,
        None,
    );
    cases.push(("evolution claim", evolution_claim));

    let mut evolution_validation = skill_evidence();
    evolution_validation
        .args(["skills", "evolution", "record-validation"])
        .args([
            "--review-id",
            "rev_fixture",
            "--decision",
            "accepted",
            "--risk-tier",
            "ordinary",
        ])
        .arg("--candidate")
        .arg(&candidate)
        .args(["--trials", "3", "--artifacts", "trials"]);
    lifecycle_arguments(
        &mut evolution_validation,
        fixture.path(),
        &evolution_operator,
        true,
        None,
    );
    cases.push(("evolution record-validation", evolution_validation));

    let mut evolution_land = skill_evidence();
    evolution_land
        .args(["skills", "evolution", "land"])
        .args(["--review-id", "rev_fixture"])
        .arg("--candidate")
        .arg(&candidate);
    lifecycle_arguments(
        &mut evolution_land,
        fixture.path(),
        &evolution_operator,
        true,
        None,
    );
    cases.push(("evolution land", evolution_land));

    let mut evolution_close = skill_evidence();
    evolution_close
        .args(["skills", "evolution", "close"])
        .args([
            "--review-id",
            "rev_fixture",
            "--disposition",
            "monitor_for_recurrence",
            "--note",
            "rationale",
        ]);
    lifecycle_arguments(
        &mut evolution_close,
        fixture.path(),
        &evolution_operator,
        true,
        None,
    );
    cases.push(("evolution close", evolution_close));

    let mut decontamination_preflight = skill_evidence();
    decontamination_preflight
        .args(["skills", "decontamination", "preflight"])
        .args(["--basis", "owner-confirmed"]);
    lifecycle_arguments(
        &mut decontamination_preflight,
        fixture.path(),
        &decontamination_operator,
        false,
        None,
    );
    cases.push(("decontamination preflight", decontamination_preflight));

    let mut decontamination_claim = skill_evidence();
    decontamination_claim
        .args(["skills", "decontamination", "claim"])
        .args(["--run-id", "dec_fixture", "--basis", "owner-confirmed"]);
    lifecycle_arguments(
        &mut decontamination_claim,
        fixture.path(),
        &decontamination_operator,
        true,
        None,
    );
    cases.push(("decontamination claim", decontamination_claim));

    let mut decontamination_validation = skill_evidence();
    decontamination_validation
        .args(["skills", "decontamination", "record-validation"])
        .args(["--run-id", "dec_fixture", "--decision", "accepted"])
        .arg("--candidate")
        .arg(&candidate)
        .args(["--trials", "5", "--artifacts", "trials"]);
    lifecycle_arguments(
        &mut decontamination_validation,
        fixture.path(),
        &decontamination_operator,
        true,
        None,
    );
    cases.push((
        "decontamination record-validation",
        decontamination_validation,
    ));

    let mut decontamination_land = skill_evidence();
    decontamination_land
        .args(["skills", "decontamination", "land"])
        .args(["--run-id", "dec_fixture"])
        .arg("--candidate")
        .arg(&candidate);
    lifecycle_arguments(
        &mut decontamination_land,
        fixture.path(),
        &decontamination_operator,
        true,
        None,
    );
    cases.push(("decontamination land", decontamination_land));

    let mut decontamination_complete = skill_evidence();
    decontamination_complete
        .args(["skills", "decontamination", "complete"])
        .args([
            "--run-id",
            "dec_fixture",
            "--outcome",
            "healthy_no_change",
            "--note",
            "rationale",
        ]);
    lifecycle_arguments(
        &mut decontamination_complete,
        fixture.path(),
        &decontamination_operator,
        true,
        None,
    );
    cases.push(("decontamination complete", decontamination_complete));

    for (label, mut command) in cases {
        let output = command.output().expect("run self-target lifecycle command");
        assert_eq!(output.status.code(), Some(3), "{label}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("refused_self_target"),
            "{label}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn skill_evolution_preflight_authorizes_a_fresh_session_with_the_bounded_packet() {
    let fixture = repository_with_demo_skill();
    record_incident(fixture.path(), "task a", "session-a");
    record_incident(fixture.path(), "task b", "session-b");
    record_incident(fixture.path(), "task c", "session-c");
    let output = run_evolution_preflight(
        fixture.path(),
        "lock_evolution_authorized",
        "fixture-session",
    );

    assert!(
        output.status.success(),
        "preflight failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("preflight receipt JSON");
    assert_eq!(receipt["authorized"], true);
    assert_eq!(receipt["gate"]["state"], "eligible");
    assert_eq!(
        receipt["gate"]["authorization_reason"],
        "friction_recurrence:execution"
    );
    assert_eq!(receipt["evidence_dir"], "reports/skill-evidence/demo-skill");
    assert_eq!(
        receipt["evidence_packet"]["qualifying_uses_on_current_hash"],
        3
    );
    assert_eq!(
        receipt["evidence_packet"]["trigger_events"]
            .as_array()
            .expect("trigger events")
            .len(),
        3
    );
    assert_eq!(
        receipt["evidence_packet"]["cited_evidence_refs"],
        serde_json::json!(["logs/fixture.txt"])
    );
    assert_eq!(
        receipt["evidence_packet"]["prior_reviews"],
        serde_json::json!([])
    );
}

#[test]
fn skill_evolution_preflight_enforces_the_fresh_session_gate() {
    let fixture = repository_with_demo_skill();
    record_incident(fixture.path(), "task a", "session-a");
    record_incident(fixture.path(), "task b", "session-b");
    record_incident(fixture.path(), "task c", "session-c");

    let output =
        run_evolution_preflight(fixture.path(), "lock_evolution_same_session", "session-c");

    assert_eq!(output.status.code(), Some(3));
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("Gate: eligible_pending_cooldown."));
    assert!(error.contains("cooldown_or_different_session_condition_passed"));
    assert!(error.contains("Terminal outcome: refused_cooldown_or_same_session."));
}

#[test]
fn skill_evolution_preflight_authorizes_a_quarantined_severe_incident() {
    let severe = repository_with_demo_skill();
    record_outcome(
        severe.path(),
        "deploy",
        "session-severe",
        "state",
        "severe_incident",
    );
    let quarantined_output = run_evolution_preflight(
        severe.path(),
        "lock_evolution_quarantined",
        "fixture-session",
    );

    assert!(
        quarantined_output.status.success(),
        "{}",
        String::from_utf8_lossy(&quarantined_output.stderr)
    );
    let quarantined_receipt: Value =
        serde_json::from_slice(&quarantined_output.stdout).expect("quarantined receipt JSON");
    assert_eq!(quarantined_receipt["gate"]["state"], "quarantined_eligible");
    assert_eq!(
        quarantined_receipt["gate"]["authorization_reason"],
        "severe"
    );
}

#[test]
fn skill_evolution_preflight_fails_closed_for_a_corrupt_event_stream() {
    let corrupt = repository_with_demo_skill();
    record_incident(corrupt.path(), "task a", "session-a");
    fs::write(
        corrupt
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
        "not json\n",
    )
    .expect("corrupt event stream");
    let corrupt_refusal =
        run_evolution_preflight(corrupt.path(), "lock_evolution_corrupt", "fixture-session");

    assert_eq!(corrupt_refusal.status.code(), Some(3));
    let corrupt_error = String::from_utf8_lossy(&corrupt_refusal.stderr);
    assert!(corrupt_error.contains("Gate: blocked."));
    assert!(corrupt_error.contains("event_stream_integrity_valid"));
}

#[test]
fn skill_evolution_preflight_classifies_prior_reviews_by_target_hash() {
    let prior = repository_with_demo_skill();
    claim_evolution(prior.path());
    let first_close = run_evolution_close(
        prior.path(),
        "evt_prior_close",
        "monitor_for_recurrence",
        Some("current arm reproduced the condition"),
    );
    assert!(
        first_close.status.success(),
        "{}",
        String::from_utf8_lossy(&first_close.stderr)
    );
    record_outcome(prior.path(), "task d", "session-d", "output", "friction");
    record_outcome(prior.path(), "task e", "session-e", "output", "friction");
    record_outcome(prior.path(), "task f", "session-f", "output", "friction");
    let prior_output =
        run_evolution_preflight(prior.path(), "lock_prior_preflight", "fixture-session");
    assert!(
        prior_output.status.success(),
        "{}",
        String::from_utf8_lossy(&prior_output.stderr)
    );
    let prior_receipt: Value =
        serde_json::from_slice(&prior_output.stdout).expect("prior-review packet JSON");
    assert_eq!(
        prior_receipt["evidence_packet"]["related_prior_dispositions"],
        serde_json::json!([])
    );
    let reviews = prior_receipt["evidence_packet"]["prior_reviews"]
        .as_array()
        .expect("prior reviews");
    assert_eq!(reviews.len(), 1);
    assert_eq!(reviews[0]["review_id"], "rev_fixture");
    assert_eq!(reviews[0]["disposition"], "monitor_for_recurrence");
    assert_eq!(reviews[0]["same_target_hash"], true);
    assert_eq!(reviews[0]["note"], "current arm reproduced the condition");
    assert_eq!(reviews[0]["report"], "reviews/rev_fixture.md");

    fs::write(
        prior.path().join(".claude/skills/demo-skill/SKILL.md"),
        "---\nname: demo-skill\n---\nRewritten body.\n",
    )
    .expect("rewrite live target");
    record_outcome(prior.path(), "task g", "session-g", "state", "friction");
    record_outcome(prior.path(), "task h", "session-h", "state", "friction");
    record_outcome(prior.path(), "task i", "session-i", "state", "friction");
    let changed_output = run_evolution_preflight(
        prior.path(),
        "lock_changed_prior_preflight",
        "fixture-session",
    );
    assert!(
        changed_output.status.success(),
        "{}",
        String::from_utf8_lossy(&changed_output.stderr)
    );
    let changed_receipt: Value =
        serde_json::from_slice(&changed_output.stdout).expect("changed-hash packet JSON");
    assert_eq!(
        changed_receipt["evidence_packet"]["prior_reviews"][0]["same_target_hash"],
        false
    );
}

#[test]
fn skill_evolution_claim_appends_the_explicit_review_identity_and_owns_the_target() {
    let fixture = repository_with_demo_skill();
    record_incident(fixture.path(), "task a", "session-a");
    record_incident(fixture.path(), "task b", "session-b");
    record_incident(fixture.path(), "task c", "session-c");
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "claim", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_evolution_claim",
            "--review-id",
            "rev_fixture",
            "--repository-head",
            "fixture-head",
            "--risk-tier",
            "provisional",
        ]);
    lifecycle_clock(&mut command, "lock_evolution_claim");

    let output = command.output().expect("claim Skill Evolution review");

    assert!(
        output.status.success(),
        "claim failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("claim receipt JSON");
    assert_eq!(receipt["review_id"], "rev_fixture");
    assert_eq!(receipt["state"], "review_in_progress");
    assert_eq!(receipt["risk_tier"], "provisional");
    assert_eq!(receipt["evidence_dir"], "reports/skill-evidence/demo-skill");
    let events = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let started: Value = events
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_type"] == "review_started")
        .expect("review_started event");
    assert_eq!(started["event_id"], "evt_evolution_claim");
    assert_eq!(started["recorded_at"], "2026-01-02T03:04:05.000Z");
    assert_eq!(started["operator_workflow"], "skill-evolution");
    assert_eq!(started["target"]["repo_head"], "fixture-head");
    assert_eq!(started["top_level_session_id"], "fixture-session");
    assert_eq!(started["payload"]["review_id"], "rev_fixture");
    assert_eq!(
        started["payload"]["session_or_cooldown_proof"],
        serde_json::json!({
            "type": "different_session",
            "threshold_session_id": "session-c",
            "review_session_id": "fixture-session"
        })
    );
    assert_eq!(gate(fixture.path())["active_review_id"], "rev_fixture");
}

#[test]
fn skill_evolution_active_review_blocks_another_preflight() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());

    let owned_refusal = run_evolution_preflight(
        fixture.path(),
        "lock_evolution_owned_preflight",
        "fixture-session",
    );

    assert_eq!(owned_refusal.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&owned_refusal.stderr).contains("Gate: review_in_progress."));
}

#[test]
fn skill_evolution_active_review_blocks_a_competing_claim() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let mut claim_again = skill_evidence();
    claim_again
        .args(["skills", "evolution", "claim", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_second_evolution_claim",
            "--review-id",
            "rev_second",
            "--repository-head",
            "fixture-head",
        ]);
    lifecycle_clock(&mut claim_again, "lock_second_evolution_claim");
    let second_refusal = claim_again
        .output()
        .expect("claim while review owns target");

    assert_eq!(second_refusal.status.code(), Some(3));
}

#[test]
fn skill_evolution_record_validation_enforces_the_trial_floor_and_freezes_candidate_bytes() {
    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let candidate = make_candidate(fixture.path(), "Demo body v2.");
    let run = |trials: &str, event_id: &str, lock_owner: &str| {
        let mut command = skill_evidence();
        command
            .args(["skills", "evolution", "record-validation", "--root"])
            .arg(fixture.path())
            .args(["--target", ".claude/skills/demo-skill"])
            .args(["--event-id", event_id])
            .args(["--repository-head", "fixture-head"])
            .args([
                "--review-id",
                claim["review_id"].as_str().expect("review id"),
            ])
            .args(["--decision", "accepted"])
            .args(["--risk-tier", "ordinary"])
            .arg("--candidate")
            .arg(&candidate)
            .args([
                "--trials",
                trials,
                "--artifacts",
                "reports/skill-evidence/demo-skill/reviews/trials",
            ]);
        lifecycle_clock(&mut command, lock_owner);
        command.output().expect("record candidate validation")
    };

    let accepted = run("3", "evt_accepted_validation", "lock_accepted_validation");
    assert!(
        accepted.status.success(),
        "validation failed: {}",
        String::from_utf8_lossy(&accepted.stderr)
    );
    let receipt: Value = serde_json::from_slice(&accepted.stdout).expect("validation receipt JSON");
    assert_eq!(receipt["recorded"], "evt_accepted_validation");
    assert_eq!(receipt["decision"], "accepted");
    assert_eq!(receipt["risk_tier"], "ordinary");
    assert_eq!(receipt["trial_count"], 3);
    assert_eq!(
        receipt["candidate_hash"]
            .as_str()
            .expect("candidate hash")
            .len(),
        64
    );
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let validation: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_accepted_validation")
        .expect("validation_completed event");
    assert_eq!(validation["event_type"], "validation_completed");
    assert_eq!(validation["payload"]["review_id"], "rev_fixture");
    assert_eq!(
        validation["payload"]["candidate_hash"],
        receipt["candidate_hash"]
    );
}

#[test]
fn skill_evolution_record_validation_refuses_fewer_than_three_paired_trials() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let candidate = make_candidate(fixture.path(), "Demo body v2.");
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "record-validation", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_short_validation",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--decision",
            "accepted",
            "--risk-tier",
            "ordinary",
        ])
        .arg("--candidate")
        .arg(candidate)
        .args([
            "--trials",
            "2",
            "--artifacts",
            "reports/skill-evidence/demo-skill/reviews/trials",
        ]);
    lifecycle_clock(&mut command, "lock_short_validation");

    let output = command.output().expect("record too-short validation");

    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).contains("at least 3 paired trials"));
    assert!(
        !fs::read_to_string(
            fixture
                .path()
                .join("reports/skill-evidence/demo-skill/events.jsonl")
        )
        .expect("read event stream")
        .contains("evt_short_validation")
    );
}

#[test]
fn skill_evolution_land_verifies_and_replaces_the_candidate_and_keeps_the_baseline_backup() {
    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let candidate = make_candidate(fixture.path(), "Demo body v2 (repaired).");
    accept_evolution_candidate(fixture.path(), &candidate);
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "land", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_evolution_landed",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
        ])
        .arg("--candidate")
        .arg(&candidate);
    lifecycle_clock(&mut command, "lock_evolution_land");

    let output = command.output().expect("land evolution candidate");

    assert!(
        output.status.success(),
        "landing failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("landing receipt JSON");
    assert_eq!(receipt["landed"], true);
    assert_eq!(receipt["before_hash"], claim["target_hash"]);
    assert_ne!(receipt["after_hash"], receipt["before_hash"]);
    assert_eq!(
        receipt["changed_files"],
        serde_json::json!({"added": [], "removed": [], "modified": ["SKILL.md"]})
    );
    assert_eq!(receipt["mirror_status"], "absent");
    assert!(
        fs::read_to_string(fixture.path().join(".claude/skills/demo-skill/SKILL.md"))
            .expect("read landed target")
            .contains("repaired")
    );
    assert!(
        fs::read_to_string(
            fixture
                .path()
                .join(receipt["backup"].as_str().expect("backup path"))
                .join("SKILL.md")
        )
        .expect("read baseline backup")
        .contains("v1")
    );
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let landed: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_evolution_landed")
        .expect("change_landed event");
    assert_eq!(landed["event_type"], "change_landed");
    assert_eq!(landed["payload"]["after_hash"], receipt["after_hash"]);
}

#[test]
fn skill_evolution_land_refuses_a_candidate_without_accepted_validation() {
    let unvalidated = repository_with_demo_skill();
    claim_evolution(unvalidated.path());
    let unvalidated_candidate = make_candidate(unvalidated.path(), "unvalidated candidate");
    let early = run_evolution_land(
        unvalidated.path(),
        &unvalidated_candidate,
        "evt_early_land",
        "lock_early_land",
    );
    assert_eq!(early.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&early.stderr)
            .contains("No accepted validation_completed event exists")
    );
}

#[test]
fn skill_evolution_land_refuses_candidate_drift_after_validation() {
    let unvalidated = repository_with_demo_skill();
    claim_evolution(unvalidated.path());
    let unvalidated_candidate = make_candidate(unvalidated.path(), "validated candidate");
    accept_evolution_candidate(unvalidated.path(), &unvalidated_candidate);
    fs::write(
        unvalidated_candidate.join("SKILL.md"),
        "---\nname: demo-skill\n---\npost-validation drift\n",
    )
    .expect("drift validated candidate");
    let drift = run_evolution_land(
        unvalidated.path(),
        &unvalidated_candidate,
        "evt_drifted_land",
        "lock_drifted_land",
    );
    assert_eq!(drift.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&drift.stderr).contains("not exactly those validated"));
    assert!(
        fs::read_to_string(
            unvalidated
                .path()
                .join(".claude/skills/demo-skill/SKILL.md")
        )
        .expect("read unmodified live target")
        .contains("v1")
    );
}

#[test]
fn skill_evolution_land_refuses_a_live_target_that_moved_after_claim() {
    let moved = repository_with_demo_skill();
    claim_evolution(moved.path());
    let moved_candidate = make_candidate(moved.path(), "validated before concurrent edit");
    accept_evolution_candidate(moved.path(), &moved_candidate);
    fs::write(
        moved.path().join(".claude/skills/demo-skill/SKILL.md"),
        "---\nname: demo-skill\n---\nconcurrent edit\n",
    )
    .expect("move live target");
    let moved_refusal = run_evolution_land(
        moved.path(),
        &moved_candidate,
        "evt_moved_land",
        "lock_moved_land",
    );
    assert_eq!(moved_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&moved_refusal.stderr)
            .contains("no longer equals the review baseline")
    );
}

#[cfg(unix)]
#[test]
fn shared_landing_restores_the_baseline_when_landed_byte_verification_fails() {
    use std::os::unix::fs::symlink;

    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let candidate = make_candidate(fixture.path(), "temporary candidate body");
    fs::remove_file(candidate.join("SKILL.md")).expect("remove regular candidate skill");
    symlink(
        fixture.path().join(".claude/skills/demo-skill/SKILL.md"),
        candidate.join("SKILL.md"),
    )
    .expect("create candidate symlink to the live skill");
    fs::write(
        candidate.join("candidate-only.txt"),
        "forces a distinct hash\n",
    )
    .expect("write candidate-only file");
    accept_evolution_candidate(fixture.path(), &candidate);
    let baseline_bytes = fs::read(fixture.path().join(".claude/skills/demo-skill/SKILL.md"))
        .expect("read baseline bytes");
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "land", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_failed_landing",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
        ])
        .arg("--candidate")
        .arg(&candidate);
    lifecycle_clock(&mut command, "lock_failed_landing");

    let output = command
        .output()
        .expect("drive landing verification failure");

    assert_eq!(output.status.code(), Some(1));
    let error = String::from_utf8(output.stderr).expect("UTF-8 landing failure");
    assert!(error.contains("Landing verification failed:"), "{error}");
    assert!(
        error.contains("Live baseline restored from backup."),
        "{error}"
    );
    assert_eq!(
        fs::read(fixture.path().join(".claude/skills/demo-skill/SKILL.md"))
            .expect("read restored target"),
        baseline_bytes
    );
    assert_eq!(
        fs::read(fixture.path().join(
            "reports/skill-evidence/demo-skill/reviews/rev_fixture/pre-land-backup/SKILL.md"
        ))
        .expect("read backup"),
        baseline_bytes
    );
    assert_eq!(gate(fixture.path())["active_review_id"], "rev_fixture");
    assert_eq!(
        gate(fixture.path())["target_content_hash"],
        claim["target_hash"]
    );
    assert!(
        !fs::read_to_string(
            fixture
                .path()
                .join("reports/skill-evidence/demo-skill/events.jsonl")
        )
        .expect("read event stream")
        .contains("evt_failed_landing")
    );
}

#[test]
fn skill_evolution_close_records_the_disposition_and_retires_the_trigger_events() {
    let fixture = repository_with_demo_skill();
    land_evolution_candidate(fixture.path());
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "close", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_evolution_close",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--disposition",
            "resolved_by_change",
            "--note",
            "mechanism repaired and validated",
        ]);
    lifecycle_clock(&mut command, "lock_evolution_close");

    let output = command.output().expect("close Skill Evolution review");

    assert!(
        output.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("close receipt JSON");
    assert_eq!(receipt["closed"], "rev_fixture");
    assert_eq!(receipt["disposition"], "resolved_by_change");
    assert_eq!(receipt["state"], "closed");
    assert_eq!(
        receipt["adjudicated_event_ids"]
            .as_array()
            .expect("adjudicated event ids")
            .len(),
        3
    );
    let projection = gate(fixture.path());
    assert_eq!(projection["active_review_id"], Value::Null);
    assert_eq!(projection["last_completed_review_id"], "rev_fixture");
    assert_eq!(projection["open_incident_ids"], serde_json::json!([]));
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let disposition: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_evolution_close")
        .expect("review_disposition event");
    assert_eq!(disposition["event_type"], "review_disposition");
    assert_eq!(
        disposition["payload"]["note"],
        "mechanism repaired and validated"
    );
    assert_event_stream_matches_the_published_schema(fixture.path());
}

#[test]
fn skill_evolution_close_refuses_adjudicate_for_non_adjudicating_dispositions() {
    for disposition in ["blocked_no_valid_test", "superseded_by_target_version"] {
        let fixture = repository_with_demo_skill();
        claim_evolution(fixture.path());
        let stream_path = fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl");
        let stream_before = fs::read(&stream_path).expect("read event stream before refusal");
        let mut command = skill_evidence();
        command
            .args(["skills", "evolution", "close", "--root"])
            .arg(fixture.path())
            .args([
                "--target",
                ".claude/skills/demo-skill",
                "--event-id",
                "evt_non_adjudicating_close",
                "--repository-head",
                "fixture-head",
                "--review-id",
                "rev_fixture",
                "--disposition",
                disposition,
                "--note",
                "review reached no conclusion",
                "--adjudicate",
                "evt_evolution_claim",
            ]);
        lifecycle_clock(&mut command, "lock_non_adjudicating_close");

        let output = command.output().expect("close Skill Evolution review");

        assert_eq!(output.status.code(), Some(3), "disposition {disposition}");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains("--adjudicate"), "{disposition}: {error}");
        assert!(error.contains(disposition), "{disposition}: {error}");
        assert_eq!(
            fs::read(&stream_path).expect("read event stream after refusal"),
            stream_before,
            "disposition {disposition}"
        );
    }
}

#[test]
fn skill_evolution_close_requires_a_note() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let missing_note = run_evolution_close(
        fixture.path(),
        "evt_missing_note",
        "monitor_for_recurrence",
        None,
    );

    assert_eq!(missing_note.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&missing_note.stderr).contains("--note"));
}

#[test]
fn skill_evolution_resolved_disposition_requires_a_landed_change() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let no_landing = run_evolution_close(
        fixture.path(),
        "evt_no_landing",
        "resolved_by_change",
        Some("not landed"),
    );

    assert_eq!(no_landing.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&no_landing.stderr).contains("requires a change_landed event"));
}

#[test]
fn skill_evolution_rejected_disposition_requires_a_rejected_validation() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let no_rejection = run_evolution_close(
        fixture.path(),
        "evt_no_rejection",
        "candidate_rejected_validation",
        Some("not rejected"),
    );

    assert_eq!(no_rejection.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&no_rejection.stderr).contains("decision=rejected"));
}

#[test]
fn skill_evolution_close_refuses_a_second_disposition() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let monitored = run_evolution_close(
        fixture.path(),
        "evt_monitored",
        "monitor_for_recurrence",
        Some("mechanism not established; watch for recurrence"),
    );
    assert!(
        monitored.status.success(),
        "{}",
        String::from_utf8_lossy(&monitored.stderr)
    );
    let repeated = run_evolution_close(
        fixture.path(),
        "evt_repeated_close",
        "monitor_for_recurrence",
        Some("already closed"),
    );
    assert_eq!(repeated.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&repeated.stderr).contains("already has a review_disposition"));
}

#[test]
fn skill_evolution_rejected_candidate_forbids_landing_and_supports_close() {
    let rejected = repository_with_demo_skill();
    claim_evolution(rejected.path());
    let rejected_candidate = make_candidate(rejected.path(), "candidate with a regression");
    let mut reject_validation = skill_evidence();
    reject_validation
        .args(["skills", "evolution", "record-validation", "--root"])
        .arg(rejected.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_rejected_validation",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--decision",
            "rejected",
            "--risk-tier",
            "ordinary",
        ])
        .arg("--candidate")
        .arg(&rejected_candidate)
        .args([
            "--trials",
            "3",
            "--artifacts",
            "reports/skill-evidence/demo-skill/reviews/trials",
            "--summary",
            "regression on core case",
        ]);
    lifecycle_clock(&mut reject_validation, "lock_rejected_validation");
    let rejection = reject_validation
        .output()
        .expect("record rejected validation");
    assert!(
        rejection.status.success(),
        "{}",
        String::from_utf8_lossy(&rejection.stderr)
    );
    let forbidden_land = run_evolution_land(
        rejected.path(),
        &rejected_candidate,
        "evt_forbidden_rejected_land",
        "lock_forbidden_rejected_land",
    );
    assert_eq!(forbidden_land.status.code(), Some(3));
    let rejected_close = run_evolution_close(
        rejected.path(),
        "evt_rejected_close",
        "candidate_rejected_validation",
        Some("regression on core case; no second candidate"),
    );
    assert!(
        rejected_close.status.success(),
        "{}",
        String::from_utf8_lossy(&rejected_close.stderr)
    );
}

#[test]
fn legacy_decontamination_preflight_preserves_the_basis_gate_and_eligibility_packet() {
    let fixture = repository_with_demo_skill();
    let mut missing = skill_evidence();
    missing
        .args(["skills", "decontamination", "preflight", "--root"])
        .arg(fixture.path())
        .args(["--target", ".claude/skills/demo-skill"]);
    lifecycle_clock(&mut missing, "lock_decontamination_missing_basis");

    let refused = missing.output().expect("run basis refusal");

    assert_eq!(refused.status.code(), Some(3));
    assert_eq!(
        String::from_utf8(refused.stderr).expect("UTF-8 refusal"),
        "Legacy Skill Decontamination not eligible.\n\
Gate: not derived.\n\
Failed condition: accepted_legacy_basis_provided (--basis owner-confirmed|audit-history|imported|routed-review).\n\
No target analysis or modification performed.\n\
Terminal outcome: refused_not_legacy_eligible.\n"
    );

    let mut eligible = skill_evidence();
    eligible
        .args(["skills", "decontamination", "preflight", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--basis",
            "owner-confirmed",
        ]);
    lifecycle_clock(&mut eligible, "lock_decontamination_preflight");
    let output = eligible.output().expect("run eligible preflight");

    assert!(
        output.status.success(),
        "preflight failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value =
        serde_json::from_slice(&output.stdout).expect("decontamination preflight JSON");
    assert_eq!(receipt["eligible"], true);
    assert_eq!(receipt["gate_state"], "closed");
    assert_eq!(receipt["legacy_basis"]["basis"], "owner-confirmed");
    assert_eq!(receipt["legacy_basis"]["ref"], Value::Null);
    assert_eq!(receipt["legacy_basis"]["note"], Value::Null);
    assert_eq!(receipt["open_incident_count"], 0);
    assert_eq!(receipt["prior_completions"], serde_json::json!([]));
    assert_eq!(receipt["min_paired_trials"], 5);
    assert_eq!(
        receipt["artifacts_dir"],
        "reports/skill-evidence/demo-skill/decontamination"
    );
}

#[test]
fn legacy_decontamination_audit_history_basis_requires_provenance() {
    let fixture = repository_with_demo_skill();
    let no_note = run_decontamination_preflight(
        fixture.path(),
        "lock_decontamination_audit_without_note",
        Some("audit-history"),
        None,
        None,
    );

    assert_eq!(no_note.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&no_note.stderr)
            .contains("audit_history_basis_describes_provenance")
    );
}

#[test]
fn legacy_decontamination_routed_basis_requires_an_existing_disposition() {
    let fixture = repository_with_demo_skill();
    let bad_route = run_decontamination_preflight(
        fixture.path(),
        "lock_decontamination_invalid_route",
        Some("routed-review"),
        Some("evt_missing"),
        None,
    );

    assert_eq!(bad_route.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&bad_route.stderr)
            .contains("routed_review_basis_cites_existing_review_disposition")
    );
}

#[test]
fn legacy_decontamination_preflight_refuses_self_target() {
    let fixture = repository_with_demo_skill();
    let self_target = repository_root().join(".claude/skills/legacy-skill-decontamination");
    let mut self_command = skill_evidence();
    self_command
        .args(["skills", "decontamination", "preflight", "--root"])
        .arg(fixture.path())
        .arg("--target")
        .arg(self_target)
        .args(["--basis", "owner-confirmed"]);
    lifecycle_clock(&mut self_command, "lock_decontamination_self_target");
    let self_refusal = self_command
        .output()
        .expect("run decontamination self-target");
    assert_eq!(self_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&self_refusal.stderr)
            .contains("operator_skill_path != target_skill_path")
    );
}

#[test]
fn legacy_decontamination_preflight_refuses_pending_skill_evolution_authorization() {
    let pending = repository_with_demo_skill();
    record_incident(pending.path(), "task a", "session-a");
    record_incident(pending.path(), "task b", "session-b");
    record_incident(pending.path(), "task c", "session-c");
    let pending_refusal = run_decontamination_preflight(
        pending.path(),
        "lock_decontamination_pending",
        Some("owner-confirmed"),
        None,
        None,
    );

    assert_eq!(pending_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&pending_refusal.stderr)
            .contains("no_pending_skill_evolution_authorization")
    );
}

#[test]
fn legacy_decontamination_preflight_refuses_an_active_review_owner() {
    let pending = repository_with_demo_skill();
    record_incident(pending.path(), "task a", "session-a");
    record_incident(pending.path(), "task b", "session-b");
    record_incident(pending.path(), "task c", "session-c");
    claim_existing_evolution(pending.path());
    let owned_refusal = run_decontamination_preflight(
        pending.path(),
        "lock_decontamination_owned",
        Some("owner-confirmed"),
        None,
        None,
    );

    assert_eq!(owned_refusal.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&owned_refusal.stderr).contains("no_other_review_owns_target"));
}

#[test]
fn legacy_decontamination_preflight_fails_closed_for_a_corrupt_event_stream() {
    let corrupt = repository_with_demo_skill();
    record_incident(corrupt.path(), "seed", "session-a");
    fs::write(
        corrupt
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
        "not json\n",
    )
    .expect("corrupt event stream");
    let corrupt_refusal = run_decontamination_preflight(
        corrupt.path(),
        "lock_decontamination_corrupt",
        Some("owner-confirmed"),
        None,
        None,
    );

    assert_eq!(corrupt_refusal.status.code(), Some(3));
    let corrupt_error = String::from_utf8_lossy(&corrupt_refusal.stderr);
    assert!(corrupt_error.contains("Gate: blocked."));
    assert!(corrupt_error.contains("event_stream_integrity_valid"));
}

#[test]
fn legacy_decontamination_claim_refuses_fewer_than_five_provisional_trials() {
    let fixture = repository_with_demo_skill();
    let mut short = skill_evidence();
    short
        .args(["skills", "decontamination", "claim", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_short_claim",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_short",
            "--basis",
            "owner-confirmed",
            "--trials",
            "3",
        ]);
    lifecycle_clock(&mut short, "lock_decontamination_short_claim");
    let short_refusal = short.output().expect("claim too-short run");
    assert_eq!(short_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&short_refusal.stderr).contains("--trials must be an integer >= 5")
    );
}

#[test]
fn legacy_decontamination_claim_snapshots_the_baseline_and_owns_the_target() {
    let fixture = repository_with_demo_skill();
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "claim", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_claim",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--basis",
            "owner-confirmed",
            "--trials",
            "6",
            "--risk-rationale",
            "governs stateful actions",
        ]);
    lifecycle_clock(&mut command, "lock_decontamination_claim");

    let output = command.output().expect("claim decontamination run");

    assert!(
        output.status.success(),
        "claim failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value =
        serde_json::from_slice(&output.stdout).expect("decontamination claim JSON");
    assert_eq!(receipt["run_id"], "dec_fixture");
    assert_eq!(receipt["state"], "review_in_progress");
    assert_eq!(receipt["provisional_trial_count"], 6);
    assert_eq!(receipt["evidence_dir"], "reports/skill-evidence/demo-skill");
    assert!(
        fs::read_to_string(
            fixture
                .path()
                .join(receipt["baseline_copy"].as_str().expect("baseline path"))
                .join("SKILL.md")
        )
        .expect("read baseline copy")
        .contains("v1")
    );
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let started: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_decontamination_claim")
        .expect("decontamination_started event");
    assert_eq!(started["event_type"], "decontamination_started");
    assert_eq!(started["payload"]["review_id"], "dec_fixture");
    assert_eq!(
        started["payload"]["legacy_basis"]["basis"],
        "owner-confirmed"
    );
    assert_eq!(
        started["payload"]["risk_rationale"],
        "governs stateful actions"
    );
    assert_eq!(gate(fixture.path())["active_review_id"], "dec_fixture");
}

#[test]
fn legacy_decontamination_active_run_blocks_another_preflight() {
    let fixture = repository_with_demo_skill();
    claim_decontamination(fixture.path());
    let owned_refusal = run_decontamination_preflight(
        fixture.path(),
        "lock_decontamination_owned_preflight",
        Some("owner-confirmed"),
        None,
        None,
    );

    assert_eq!(owned_refusal.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&owned_refusal.stderr).contains("no_other_review_owns_target"));
}

#[test]
fn legacy_decontamination_active_run_blocks_a_competing_claim() {
    let fixture = repository_with_demo_skill();
    claim_decontamination(fixture.path());
    let mut claim_again = skill_evidence();
    claim_again
        .args(["skills", "decontamination", "claim", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_second_decontamination_claim",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_second",
            "--basis",
            "owner-confirmed",
        ]);
    lifecycle_clock(&mut claim_again, "lock_second_decontamination_claim");
    let second_refusal = claim_again
        .output()
        .expect("claim a second decontamination run");
    assert_eq!(second_refusal.status.code(), Some(3));
}

#[test]
fn legacy_decontamination_claim_is_byte_deterministic_under_fixed_inputs() {
    let fixture = repository_with_demo_skill();
    let first = claim_decontamination(fixture.path());
    let deterministic_peer = repository_with_demo_skill();
    let mut peer_claim = skill_evidence();
    peer_claim
        .args(["skills", "decontamination", "claim", "--root"])
        .arg(deterministic_peer.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_claim",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--basis",
            "owner-confirmed",
        ]);
    lifecycle_clock(&mut peer_claim, "lock_decontamination_claim");
    let peer_output = peer_claim.output().expect("claim deterministic peer run");
    assert!(
        peer_output.status.success(),
        "{}",
        String::from_utf8_lossy(&peer_output.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&peer_output.stdout).expect("peer claim receipt"),
        first
    );
    for relative in [
        "reports/skill-evidence/demo-skill/events.jsonl",
        "reports/skill-evidence/demo-skill/gate-status.json",
    ] {
        assert_eq!(
            fs::read(fixture.path().join(relative)).expect("read first deterministic artifact"),
            fs::read(deterministic_peer.path().join(relative))
                .expect("read peer deterministic artifact"),
            "{relative} must be byte-identical under fixed inputs"
        );
    }
}

#[test]
fn legacy_decontamination_record_validation_enforces_five_trials_and_high_risk() {
    let fixture = repository_with_demo_skill();
    claim_decontamination(fixture.path());
    let candidate = make_decontamination_candidate(fixture.path());
    let run = |trials: &str, event_id: &str, lock_owner: &str| {
        let mut command = skill_evidence();
        command
            .args(["skills", "decontamination", "record-validation", "--root"])
            .arg(fixture.path())
            .args(["--target", ".claude/skills/demo-skill"])
            .args(["--event-id", event_id])
            .args(["--repository-head", "fixture-head"])
            .args(["--run-id", "dec_fixture", "--decision", "accepted"])
            .arg("--candidate")
            .arg(&candidate)
            .args([
                "--trials",
                trials,
                "--artifacts",
                "reports/skill-evidence/demo-skill/decontamination/trials",
            ]);
        lifecycle_clock(&mut command, lock_owner);
        command.output().expect("record decontamination validation")
    };

    let accepted = run(
        "5",
        "evt_decontamination_validation",
        "lock_decontamination_validation",
    );
    assert!(
        accepted.status.success(),
        "validation failed: {}",
        String::from_utf8_lossy(&accepted.stderr)
    );
    let receipt: Value = serde_json::from_slice(&accepted.stdout).expect("validation receipt JSON");
    assert_eq!(receipt["recorded"], "evt_decontamination_validation");
    assert_eq!(receipt["decision"], "accepted");
    assert_eq!(receipt["risk_tier"], "high");
    assert_eq!(receipt["trial_count"], 5);
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let validation: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_decontamination_validation")
        .expect("validation_completed event");
    assert_eq!(
        validation["operator_workflow"],
        "legacy-skill-decontamination"
    );
    assert_eq!(validation["payload"]["risk_tier"], "high");
}

#[test]
fn legacy_decontamination_record_validation_refuses_fewer_than_five_trials() {
    let fixture = repository_with_demo_skill();
    claim_decontamination(fixture.path());
    let candidate = make_decontamination_candidate(fixture.path());
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "record-validation", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_short",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--decision",
            "accepted",
        ])
        .arg("--candidate")
        .arg(candidate)
        .args([
            "--trials",
            "4",
            "--artifacts",
            "reports/skill-evidence/demo-skill/decontamination/trials",
        ]);
    lifecycle_clock(&mut command, "lock_decontamination_short");

    let output = command.output().expect("record too-short validation");

    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).contains("at least 5 paired trials"));
    assert!(
        !fs::read_to_string(
            fixture
                .path()
                .join("reports/skill-evidence/demo-skill/events.jsonl")
        )
        .expect("read event stream")
        .contains("evt_decontamination_short")
    );
}

#[test]
fn legacy_decontamination_land_consumes_the_shared_verified_landing_mechanics() {
    let fixture = repository_with_demo_skill();
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        fs::create_dir_all(fixture.path().join(".agents/skills"))
            .expect("create agent skill mirror directory");
        symlink(
            "../../.claude/skills/demo-skill",
            fixture.path().join(".agents/skills/demo-skill"),
        )
        .expect("create agent skill mirror");
    }
    let claim = claim_decontamination(fixture.path());
    let candidate = make_decontamination_candidate(fixture.path());
    accept_decontamination_candidate(fixture.path(), &candidate);
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "land", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_landed",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
        ])
        .arg("--candidate")
        .arg(&candidate);
    lifecycle_clock(&mut command, "lock_decontamination_land");

    let output = command.output().expect("land decontamination candidate");

    assert!(
        output.status.success(),
        "landing failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("landing receipt JSON");
    assert_eq!(receipt["landed"], true);
    assert_eq!(receipt["before_hash"], claim["target_hash"]);
    assert_eq!(
        receipt["changed_files"],
        serde_json::json!({"added": [], "removed": [], "modified": ["SKILL.md"]})
    );
    #[cfg(unix)]
    assert_eq!(receipt["mirror_status"], "ok");
    assert!(
        receipt["backup"]
            .as_str()
            .expect("backup path")
            .contains("decontamination/dec_fixture/pre-land-backup")
    );
    assert!(
        fs::read_to_string(fixture.path().join(".claude/skills/demo-skill/SKILL.md"))
            .expect("read landed target")
            .contains("decontaminated")
    );
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let landed: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_decontamination_landed")
        .expect("change_landed event");
    assert_eq!(landed["operator_workflow"], "legacy-skill-decontamination");
}

#[test]
fn legacy_decontamination_land_refuses_a_candidate_without_accepted_validation() {
    let unvalidated = repository_with_demo_skill();
    claim_decontamination(unvalidated.path());
    let unvalidated_candidate = make_decontamination_candidate(unvalidated.path());
    let early = run_decontamination_land(
        unvalidated.path(),
        &unvalidated_candidate,
        "evt_early_decontamination_land",
        "lock_early_decontamination_land",
    );
    assert_eq!(early.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&early.stderr)
            .contains("No accepted validation_completed event exists")
    );
}

#[test]
fn legacy_decontamination_land_refuses_candidate_drift_after_validation() {
    let unvalidated = repository_with_demo_skill();
    claim_decontamination(unvalidated.path());
    let unvalidated_candidate = make_decontamination_candidate(unvalidated.path());
    accept_decontamination_candidate(unvalidated.path(), &unvalidated_candidate);
    fs::write(
        unvalidated_candidate.join("SKILL.md"),
        "---\nname: demo-skill\n---\npost-validation decontamination drift\n",
    )
    .expect("drift validated decontamination candidate");
    let drift = run_decontamination_land(
        unvalidated.path(),
        &unvalidated_candidate,
        "evt_drifted_decontamination_land",
        "lock_drifted_decontamination_land",
    );
    assert_eq!(drift.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&drift.stderr).contains("not exactly those validated"));
}

#[test]
fn legacy_decontamination_moved_target_requires_a_superseded_completion() {
    let moved = repository_with_demo_skill();
    claim_decontamination(moved.path());
    let moved_candidate = make_decontamination_candidate(moved.path());
    accept_decontamination_candidate(moved.path(), &moved_candidate);
    fs::write(
        moved.path().join(".claude/skills/demo-skill/SKILL.md"),
        "---\nname: demo-skill\n---\nconcurrent edit\n",
    )
    .expect("move live target during decontamination");
    let moved_refusal = run_decontamination_land(
        moved.path(),
        &moved_candidate,
        "evt_moved_decontamination_land",
        "lock_moved_decontamination_land",
    );
    assert_eq!(moved_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&moved_refusal.stderr)
            .contains("complete with superseded_by_target_version")
    );
    let wrong_completion = run_decontamination_complete(
        moved.path(),
        "evt_wrong_moved_completion",
        "healthy_no_change",
        Some("wrong outcome"),
    );
    assert_eq!(wrong_completion.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&wrong_completion.stderr)
            .contains("only valid outcome is superseded_by_target_version")
    );
    let superseded = run_decontamination_complete(
        moved.path(),
        "evt_superseded_completion",
        "superseded_by_target_version",
        Some("target changed mid-run"),
    );
    assert!(
        superseded.status.success(),
        "{}",
        String::from_utf8_lossy(&superseded.stderr)
    );
    let mut eligible_again = skill_evidence();
    eligible_again
        .args(["skills", "decontamination", "preflight", "--root"])
        .arg(moved.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--basis",
            "owner-confirmed",
        ]);
    lifecycle_clock(&mut eligible_again, "lock_decontamination_after_superseded");
    let eligible_output = eligible_again
        .output()
        .expect("run after-superseded preflight");
    assert!(
        eligible_output.status.success(),
        "{}",
        String::from_utf8_lossy(&eligible_output.stderr)
    );
    let eligible_receipt: Value =
        serde_json::from_slice(&eligible_output.stdout).expect("eligible packet JSON");
    assert_eq!(
        eligible_receipt["prior_completions"]
            .as_array()
            .expect("prior completions")
            .len(),
        1
    );
}

#[cfg(unix)]
#[test]
fn legacy_decontamination_land_records_broken_mirror_status_without_rolling_back() {
    use std::os::unix::fs::symlink;

    let broken_mirror = repository_with_demo_skill();
    fs::create_dir_all(broken_mirror.path().join(".agents/skills"))
        .expect("create broken mirror directory");
    fs::create_dir_all(broken_mirror.path().join(".claude/skills/wrong-skill"))
        .expect("create wrong mirror target");
    fs::write(
        broken_mirror
            .path()
            .join(".claude/skills/wrong-skill/SKILL.md"),
        "---\nname: wrong-skill\n---\nWrong target.\n",
    )
    .expect("write wrong mirror target");
    symlink(
        "../../.claude/skills/wrong-skill",
        broken_mirror.path().join(".agents/skills/demo-skill"),
    )
    .expect("create wrong agent skill mirror");
    let broken_claim = claim_decontamination(broken_mirror.path());
    let broken_candidate = make_decontamination_candidate(broken_mirror.path());
    accept_decontamination_candidate(broken_mirror.path(), &broken_candidate);
    let baseline = fs::read(
        broken_mirror
            .path()
            .join(".claude/skills/demo-skill/SKILL.md"),
    )
    .expect("read broken-mirror baseline");
    let broken_landing = run_decontamination_land(
        broken_mirror.path(),
        &broken_candidate,
        "evt_broken_mirror_land",
        "lock_broken_mirror_land",
    );

    assert!(
        broken_landing.status.success(),
        "{}",
        String::from_utf8_lossy(&broken_landing.stderr)
    );
    let broken_receipt: Value =
        serde_json::from_slice(&broken_landing.stdout).expect("broken-mirror receipt JSON");
    assert_eq!(broken_receipt["mirror_status"], "broken");
    assert_ne!(
        fs::read(
            broken_mirror
                .path()
                .join(".claude/skills/demo-skill/SKILL.md")
        )
        .expect("read broken-mirror landed target"),
        baseline
    );
    assert!(
        fs::read_to_string(
            broken_mirror
                .path()
                .join(
                    broken_claim["evidence_dir"]
                        .as_str()
                        .expect("broken-mirror evidence directory"),
                )
                .join("events.jsonl")
        )
        .expect("read broken-mirror event stream")
        .contains("evt_broken_mirror_land")
    );
}

#[test]
fn legacy_decontamination_complete_records_the_terminal_outcome_and_closes_the_run() {
    let fixture = repository_with_demo_skill();
    land_decontamination_candidate(fixture.path());
    let mut command = skill_evidence();
    command
        .args(["skills", "decontamination", "complete", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_complete",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--outcome",
            "validated_simplification_landed",
            "--note",
            "noninferior on all five paired trials and smaller",
        ]);
    lifecycle_clock(&mut command, "lock_decontamination_complete");

    let output = command.output().expect("complete decontamination run");

    assert!(
        output.status.success(),
        "complete failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("completion receipt JSON");
    assert_eq!(receipt["completed"], "dec_fixture");
    assert_eq!(receipt["outcome"], "validated_simplification_landed");
    assert_eq!(receipt["state"], "closed");
    assert_eq!(
        receipt["report_path"],
        "reports/skill-evidence/demo-skill/decontamination/dec_fixture.md"
    );
    let projection = gate(fixture.path());
    assert_eq!(projection["active_review_id"], Value::Null);
    assert_eq!(projection["last_completed_review_id"], "dec_fixture");
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let completed: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_decontamination_complete")
        .expect("decontamination_completed event");
    assert_eq!(completed["event_type"], "decontamination_completed");
    assert_eq!(
        completed["payload"]["note"],
        "noninferior on all five paired trials and smaller"
    );
    assert_event_stream_matches_the_published_schema(fixture.path());
}

#[test]
fn legacy_decontamination_landed_change_requires_the_landed_outcome() {
    let fixture = repository_with_demo_skill();
    land_decontamination_candidate(fixture.path());
    let output = run_decontamination_complete(
        fixture.path(),
        "evt_wrong_landed_outcome",
        "healthy_no_change",
        Some("wrong after landing"),
    );

    assert_eq!(output.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("only valid outcome is validated_simplification_landed")
    );
}

#[test]
fn legacy_decontamination_preflight_refuses_an_already_completed_target_version() {
    let fixture = repository_with_demo_skill();
    land_decontamination_candidate(fixture.path());
    let completion = run_decontamination_complete(
        fixture.path(),
        "evt_decontamination_complete",
        "validated_simplification_landed",
        Some("noninferior on all five paired trials and smaller"),
    );
    assert!(completion.status.success());
    let rerun_refusal = run_decontamination_preflight(
        fixture.path(),
        "lock_decontamination_completed_rerun",
        Some("owner-confirmed"),
        None,
        None,
    );

    assert_eq!(rerun_refusal.status.code(), Some(3));
    let rerun_error = String::from_utf8_lossy(&rerun_refusal.stderr);
    assert!(rerun_error.contains("no_completed_decontamination_covers_target_version"));
    assert!(rerun_error.contains("Terminal outcome: refused_already_completed."));
}

#[test]
fn legacy_decontamination_landed_outcome_requires_a_landed_change() {
    let fixture = repository_with_demo_skill();
    claim_decontamination(fixture.path());
    let no_land = run_decontamination_complete(
        fixture.path(),
        "evt_no_decontamination_land",
        "validated_simplification_landed",
        Some("not landed"),
    );

    assert_eq!(no_land.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&no_land.stderr).contains("requires a change_landed event"));
}

#[test]
fn legacy_decontamination_rejected_outcome_requires_a_rejected_validation() {
    let fixture = repository_with_demo_skill();
    claim_decontamination(fixture.path());
    let no_rejection = run_decontamination_complete(
        fixture.path(),
        "evt_no_decontamination_rejection",
        "candidate_rejected_validation",
        Some("not rejected"),
    );

    assert_eq!(no_rejection.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&no_rejection.stderr).contains("decision=rejected"));
}

#[test]
fn legacy_decontamination_complete_requires_a_note() {
    let fixture = repository_with_demo_skill();
    claim_decontamination(fixture.path());
    let no_note = run_decontamination_complete(
        fixture.path(),
        "evt_no_decontamination_note",
        "healthy_no_change",
        None,
    );

    assert_eq!(no_note.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&no_note.stderr).contains("--note"));
}

#[test]
fn legacy_decontamination_rejected_candidate_forbids_landing_and_supports_completion() {
    let policy = repository_with_demo_skill();
    claim_decontamination(policy.path());
    let rejected_candidate = make_decontamination_candidate(policy.path());
    let mut reject_validation = skill_evidence();
    reject_validation
        .args(["skills", "decontamination", "record-validation", "--root"])
        .arg(policy.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_decontamination_rejected_validation",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--decision",
            "rejected",
        ])
        .arg("--candidate")
        .arg(&rejected_candidate)
        .args([
            "--trials",
            "5",
            "--artifacts",
            "reports/skill-evidence/demo-skill/decontamination/trials",
            "--summary",
            "regression on fragile branch",
        ]);
    lifecycle_clock(
        &mut reject_validation,
        "lock_decontamination_rejected_validation",
    );
    let rejection = reject_validation
        .output()
        .expect("record rejected decontamination validation");
    assert!(
        rejection.status.success(),
        "{}",
        String::from_utf8_lossy(&rejection.stderr)
    );
    let forbidden_land = run_decontamination_land(
        policy.path(),
        &rejected_candidate,
        "evt_forbidden_decontamination_land",
        "lock_forbidden_decontamination_land",
    );
    assert_eq!(forbidden_land.status.code(), Some(3));
    let rejected = run_decontamination_complete(
        policy.path(),
        "evt_decontamination_rejected",
        "candidate_rejected_validation",
        Some("regression on fragile branch; current skill retained"),
    );
    assert!(
        rejected.status.success(),
        "{}",
        String::from_utf8_lossy(&rejected.stderr)
    );
    let repeated = run_decontamination_complete(
        policy.path(),
        "evt_decontamination_repeated",
        "candidate_rejected_validation",
        Some("already complete"),
    );
    assert_eq!(repeated.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&repeated.stderr)
            .contains("already has a decontamination_completed")
    );
}

#[test]
fn legacy_decontamination_blocked_rerun_requires_new_corpus_material() {
    let blocked = repository_with_demo_skill();
    claim_decontamination(blocked.path());
    let blocked_completion = run_decontamination_complete(
        blocked.path(),
        "evt_decontamination_blocked",
        "blocked_no_valid_test",
        Some("no representative corpus constructible from available history"),
    );
    assert!(
        blocked_completion.status.success(),
        "{}",
        String::from_utf8_lossy(&blocked_completion.stderr)
    );
    let bare_refusal = run_decontamination_preflight(
        blocked.path(),
        "lock_decontamination_bare_retry",
        Some("owner-confirmed"),
        None,
        None,
    );
    assert_eq!(bare_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&bare_refusal.stderr)
            .contains("blocked_rerun_names_new_corpus_material")
    );
    let supported = run_decontamination_preflight(
        blocked.path(),
        "lock_decontamination_supported_retry",
        Some("owner-confirmed"),
        None,
        Some("six real qualifying-use transcripts recorded since the blocked run"),
    );
    assert!(
        supported.status.success(),
        "{}",
        String::from_utf8_lossy(&supported.stderr)
    );
}

#[test]
fn legacy_decontamination_superseded_outcome_requires_target_drift() {
    let healthy = repository_with_demo_skill();
    claim_decontamination(healthy.path());
    let premature_superseded = run_decontamination_complete(
        healthy.path(),
        "evt_premature_superseded",
        "superseded_by_target_version",
        Some("target unchanged"),
    );
    assert_eq!(premature_superseded.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&premature_superseded.stderr)
            .contains("requires the live target to differ")
    );
}

#[test]
fn legacy_decontamination_healthy_completion_blocks_same_hash_rerun() {
    let healthy = repository_with_demo_skill();
    claim_decontamination(healthy.path());
    let healthy_completion = run_decontamination_complete(
        healthy.path(),
        "evt_healthy_completion",
        "healthy_no_change",
        Some("accretion already minimal; candidate not meaningfully simpler"),
    );
    assert!(
        healthy_completion.status.success(),
        "{}",
        String::from_utf8_lossy(&healthy_completion.stderr)
    );
    let same_hash_refusal = run_decontamination_preflight(
        healthy.path(),
        "lock_decontamination_same_hash_retry",
        Some("owner-confirmed"),
        None,
        None,
    );

    assert_eq!(same_hash_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&same_hash_refusal.stderr).contains("refused_already_completed")
    );
}

#[test]
fn legacy_decontamination_changed_healthy_baseline_requires_routed_review() {
    let healthy = repository_with_demo_skill();
    claim_decontamination(healthy.path());
    let healthy_completion = run_decontamination_complete(
        healthy.path(),
        "evt_healthy_completion",
        "healthy_no_change",
        Some("accretion already minimal; candidate not meaningfully simpler"),
    );
    assert!(healthy_completion.status.success());
    fs::write(
        healthy.path().join(".claude/skills/demo-skill/SKILL.md"),
        "---\nname: demo-skill\n---\nlater imported change\n",
    )
    .expect("change target after healthy completion");
    let owner_refusal = run_decontamination_preflight(
        healthy.path(),
        "lock_decontamination_owner_retry",
        Some("owner-confirmed"),
        None,
        None,
    );
    assert_eq!(owner_refusal.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&owner_refusal.stderr)
            .contains("legacy_baseline_already_adjudicated")
    );
    claim_evolution(healthy.path());
    let routed_disposition = run_evolution_close(
        healthy.path(),
        "evt_routed_review",
        "monitor_for_recurrence",
        Some("legacy-style accretion; route to decontamination"),
    );
    assert!(
        routed_disposition.status.success(),
        "{}",
        String::from_utf8_lossy(&routed_disposition.stderr)
    );
    let routed = run_decontamination_preflight(
        healthy.path(),
        "lock_decontamination_routed_retry",
        Some("routed-review"),
        Some("evt_routed_review"),
        None,
    );
    assert!(
        routed.status.success(),
        "{}",
        String::from_utf8_lossy(&routed.stderr)
    );
}

#[test]
fn legacy_semantic_omissions_remain_code_three_refusals() {
    let fixture = repository_with_demo_skill();
    let candidate = make_candidate(fixture.path(), "candidate");

    let mut missing_target = skill_evidence();
    missing_target
        .args(["skills", "evolution", "preflight", "--root"])
        .arg(fixture.path());
    lifecycle_clock(&mut missing_target, "lock_missing_target");

    let mut missing_review = skill_evidence();
    missing_review
        .args(["skills", "evolution", "record-validation", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_missing_review",
            "--repository-head",
            "fixture-head",
            "--decision",
            "accepted",
            "--risk-tier",
            "ordinary",
        ])
        .arg("--candidate")
        .arg(&candidate)
        .args(["--trials", "3", "--artifacts", "trials"]);
    lifecycle_clock(&mut missing_review, "lock_missing_review");

    let mut missing_candidate = skill_evidence();
    missing_candidate
        .args(["skills", "evolution", "land", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_missing_candidate",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
        ]);
    lifecycle_clock(&mut missing_candidate, "lock_missing_candidate");

    let mut missing_decision = skill_evidence();
    missing_decision
        .args(["skills", "decontamination", "record-validation", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_missing_decision",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
        ])
        .arg("--candidate")
        .arg(&candidate)
        .args(["--trials", "5", "--artifacts", "trials"]);
    lifecycle_clock(&mut missing_decision, "lock_missing_decision");

    let mut missing_outcome = skill_evidence();
    missing_outcome
        .args(["skills", "decontamination", "complete", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_missing_outcome",
            "--repository-head",
            "fixture-head",
            "--run-id",
            "dec_fixture",
            "--note",
            "rationale",
        ]);
    lifecycle_clock(&mut missing_outcome, "lock_missing_outcome");

    for (label, mut command, expected) in [
        (
            "target",
            missing_target,
            "Missing required --target <skill-dir>.",
        ),
        ("review", missing_review, "Missing required --review-id."),
        (
            "candidate",
            missing_candidate,
            "Missing required --target <skill-dir>.",
        ),
        (
            "decision",
            missing_decision,
            "--decision must be accepted|rejected.",
        ),
        ("outcome", missing_outcome, "--outcome must be one of"),
    ] {
        let output = command.output().expect("run semantic omission case");
        assert_eq!(output.status.code(), Some(3), "{label}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected),
            "{label}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn rust_appended_lifecycle_events_keep_the_javascript_byte_order() {
    let source = repository_root().join("fixtures/skill-evidence/lifecycle-v1");
    let expected_stream = fs::read(source.join("reports/skill-evidence/demo-skill/events.jsonl"))
        .expect("read pre-migration JavaScript golden stream");
    let expected_text = String::from_utf8(expected_stream.clone()).expect("golden stream is UTF-8");
    let fixture = tempfile::tempdir().expect("temporary replay repository");
    let target = fixture.path().join(".claude/skills/demo-skill");
    let store = fixture.path().join("reports/skill-evidence/demo-skill");
    fs::create_dir_all(&target).expect("create replay target");
    fs::create_dir_all(&store).expect("create replay store");
    fs::copy(
        source.join(
            "reports/skill-evidence/demo-skill/reviews/rev_e32e7983-e876-4cf3-8537-019d2e37ce84/pre-land-backup/SKILL.md",
        ),
        target.join("SKILL.md"),
    )
    .expect("restore pre-evolution target bytes");
    for candidate in ["candidate-evolution", "candidate-decontamination"] {
        let destination = fixture.path().join(candidate);
        fs::create_dir_all(&destination).expect("create replay candidate");
        fs::copy(
            source.join(candidate).join("SKILL.md"),
            destination.join("SKILL.md"),
        )
        .expect("copy replay candidate");
    }
    let initial_stream = format!(
        "{}\n",
        expected_text.lines().take(3).collect::<Vec<_>>().join("\n")
    );
    fs::write(store.join("events.jsonl"), initial_stream).expect("seed pre-review events");

    let add_event_inputs = |command: &mut Command,
                            event_id: &str,
                            recorded_at: &str,
                            now_ms: &str,
                            session_id: &str,
                            lock_owner: &str| {
        command.args(["--root"]).arg(fixture.path()).args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            event_id,
            "--recorded-at",
            recorded_at,
            "--now-epoch-milliseconds",
            now_ms,
            "--repository-head",
            "8d64e43da7d18315758e95311870a72e774081d5",
            "--session-id",
            session_id,
            "--lock-owner",
            lock_owner,
        ]);
    };
    let run = |mut command: Command, label: &str| {
        let output = command.output().expect("run golden replay command");
        assert!(
            output.status.success(),
            "{label}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };

    let mut evolution_claim = skill_evidence();
    evolution_claim
        .args(["skills", "evolution", "claim"])
        .args(["--review-id", "rev_e32e7983-e876-4cf3-8537-019d2e37ce84"]);
    add_event_inputs(
        &mut evolution_claim,
        "evt_8a56770b-88c5-4f8c-839b-50982c2c8135",
        "2026-07-31T09:35:52.463Z",
        "1785490552463",
        "fixture-review-session",
        "lock_golden_evolution_claim",
    );
    run(evolution_claim, "evolution claim");

    let mut evolution_validation = skill_evidence();
    evolution_validation
        .args(["skills", "evolution", "record-validation"])
        .args([
            "--review-id",
            "rev_e32e7983-e876-4cf3-8537-019d2e37ce84",
            "--decision",
            "accepted",
            "--risk-tier",
            "ordinary",
            "--candidate",
            "candidate-evolution",
            "--trials",
            "3",
            "--artifacts",
            "reports/skill-evidence/demo-skill/reviews/fixture",
        ]);
    add_event_inputs(
        &mut evolution_validation,
        "evt_00bdbbd8-bc5a-4c43-b4e4-dd041581d30c",
        "2026-07-31T09:35:58.784Z",
        "1785490558784",
        "fixture-review-session",
        "lock_golden_evolution_validation",
    );
    run(evolution_validation, "evolution validation");

    let mut evolution_land = skill_evidence();
    evolution_land.args(["skills", "evolution", "land"]).args([
        "--review-id",
        "rev_e32e7983-e876-4cf3-8537-019d2e37ce84",
        "--candidate",
        "candidate-evolution",
    ]);
    add_event_inputs(
        &mut evolution_land,
        "evt_ee5a3666-0047-477d-be60-429ebf41d1fe",
        "2026-07-31T09:36:03.953Z",
        "1785490563953",
        "fixture-review-session",
        "lock_golden_evolution_land",
    );
    run(evolution_land, "evolution land");

    let mut evolution_close = skill_evidence();
    evolution_close
        .args(["skills", "evolution", "close"])
        .args([
            "--review-id",
            "rev_e32e7983-e876-4cf3-8537-019d2e37ce84",
            "--disposition",
            "resolved_by_change",
            "--note",
            "pre-migration fixture evolution completed",
        ]);
    add_event_inputs(
        &mut evolution_close,
        "evt_8a4ad6aa-70d6-4189-a8de-976c60774af2",
        "2026-07-31T09:36:09.454Z",
        "1785490569454",
        "fixture-review-session",
        "lock_golden_evolution_close",
    );
    run(evolution_close, "evolution close");

    let mut decontamination_claim = skill_evidence();
    decontamination_claim
        .args(["skills", "decontamination", "claim"])
        .args([
            "--run-id",
            "dec_13763440-083e-4d19-87e5-51a2f1f743dc",
            "--basis",
            "owner-confirmed",
            "--trials",
            "5",
        ]);
    add_event_inputs(
        &mut decontamination_claim,
        "evt_ae4e4c29-4fc5-493a-9981-48bd27ef5048",
        "2026-07-31T09:36:15.185Z",
        "1785490575185",
        "fixture-decontamination-session",
        "lock_golden_decontamination_claim",
    );
    run(decontamination_claim, "decontamination claim");

    let mut decontamination_validation = skill_evidence();
    decontamination_validation
        .args(["skills", "decontamination", "record-validation"])
        .args([
            "--run-id",
            "dec_13763440-083e-4d19-87e5-51a2f1f743dc",
            "--decision",
            "accepted",
            "--candidate",
            "candidate-decontamination",
            "--trials",
            "5",
            "--artifacts",
            "reports/skill-evidence/demo-skill/decontamination/fixture",
        ]);
    add_event_inputs(
        &mut decontamination_validation,
        "evt_2791e73b-55da-4602-b9a7-05079ab7da4a",
        "2026-07-31T09:36:21.300Z",
        "1785490581300",
        "fixture-decontamination-session",
        "lock_golden_decontamination_validation",
    );
    run(decontamination_validation, "decontamination validation");

    let mut decontamination_land = skill_evidence();
    decontamination_land
        .args(["skills", "decontamination", "land"])
        .args([
            "--run-id",
            "dec_13763440-083e-4d19-87e5-51a2f1f743dc",
            "--candidate",
            "candidate-decontamination",
        ]);
    add_event_inputs(
        &mut decontamination_land,
        "evt_bda00034-3865-4f72-bf20-e876edadb6c8",
        "2026-07-31T09:36:27.943Z",
        "1785490587943",
        "fixture-decontamination-session",
        "lock_golden_decontamination_land",
    );
    run(decontamination_land, "decontamination land");

    let mut decontamination_complete = skill_evidence();
    decontamination_complete
        .args(["skills", "decontamination", "complete"])
        .args([
            "--run-id",
            "dec_13763440-083e-4d19-87e5-51a2f1f743dc",
            "--outcome",
            "validated_simplification_landed",
            "--note",
            "pre-migration fixture decontamination completed",
        ]);
    add_event_inputs(
        &mut decontamination_complete,
        "evt_e0a090b2-6aa7-460f-a00e-df3b6887d102",
        "2026-07-31T09:36:33.724Z",
        "1785490593724",
        "fixture-decontamination-session",
        "lock_golden_decontamination_complete",
    );
    run(decontamination_complete, "decontamination complete");

    assert_eq!(
        fs::read(store.join("events.jsonl")).expect("read Rust replay stream"),
        expected_stream
    );
    let javascript_review_line = expected_text
        .lines()
        .nth(3)
        .expect("JavaScript review event");
    let sorted_rejected_alternative = serde_json::to_string(
        &serde_json::from_str::<Value>(javascript_review_line).expect("parse JavaScript event"),
    )
    .expect("serialize rejected sorted alternative");
    assert_ne!(javascript_review_line, sorted_rejected_alternative);
}
