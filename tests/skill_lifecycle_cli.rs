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
fn all_five_lifecycle_commands_refuse_missing_explicit_inputs_with_code_three() {
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

    let output = evolution.output().expect("run missing claim identity");
    assert_eq!(output.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Missing required --review-id."),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
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
    let package = fixture.path().join(".claude/skills/skill-evolution");
    fs::create_dir_all(&package).expect("create same-basename package");
    fs::write(
        package.join("SKILL.md"),
        "---\nname: skill-evolution\n---\nUnrelated package.\n",
    )
    .expect("write same-basename package");

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
}

#[test]
fn lifecycle_self_target_refusal_applies_to_all_five_commands() {
    let fixture = repository_with_demo_skill();
    let candidate = fixture.path().join(".claude/skills/demo-skill");
    let evolution_operator = repository_root().join(".claude/skills/skill-evolution");
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

    // Reached through Skill Evolution since Legacy Skill Decontamination was
    // retired; the `--decision` check is the same one, on the same flag.
    let mut missing_decision = skill_evidence();
    missing_decision
        .args(["skills", "evolution", "record-validation", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_missing_decision",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--risk-tier",
            "ordinary",
        ])
        .arg("--candidate")
        .arg(&candidate)
        .args(["--trials", "3", "--artifacts", "trials"]);
    lifecycle_clock(&mut missing_decision, "lock_missing_decision");

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
    let destination = fixture.path().join("candidate-evolution");
    fs::create_dir_all(&destination).expect("create replay candidate");
    fs::copy(
        source.join("candidate-evolution").join("SKILL.md"),
        destination.join("SKILL.md"),
    )
    .expect("copy replay candidate");
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

    // The golden's last four lines are a Legacy Skill Decontamination run. That
    // workflow was retired and its commands removed, so nothing here can write
    // them any more — the replay ends at `review_disposition`, and the
    // comparison ends with it. The byte-order guarantee is unchanged for every
    // event type still writable, which is every type this crate emits.
    let expected_prefix = format!(
        "{}\n",
        expected_text.lines().take(7).collect::<Vec<_>>().join("\n")
    );
    assert_eq!(
        fs::read_to_string(store.join("events.jsonl")).expect("read Rust replay stream"),
        expected_prefix
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
