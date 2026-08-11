#![forbid(unsafe_code)]

mod support;

use std::{fs, path::Path, process::Command};

use serde_json::Value;
use support::{host, repository_root, skill_evidence};
use tempfile::TempDir;

fn expected_operating_skill_hash() -> String {
    skill_evidence::hash_skill(
        &repository_root(),
        Path::new(".claude/skills/skill-evolution"),
        &host(),
    )
    .expect("hash the reference host's operating Skill Evolution package")
    .content_hash
}

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

fn record_outcome(root: &Path, label: &str, session: &str, symptom: &str, outcome: &str) -> String {
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
    serde_json::from_slice::<Value>(&output.stdout).expect("record receipt JSON")["event_id"]
        .as_str()
        .expect("record receipt event id")
        .to_owned()
}

fn claim_evolution(root: &Path) -> Value {
    record_incident(root, "task a", "session-a");
    record_incident(root, "task b", "session-b");
    record_incident(root, "task c", "session-c");
    claim_existing_evolution(root)
}

fn write_review_report(root: &Path, review_id: &str, contents: &str) {
    let reviews = root.join("reports/skill-evidence/demo-skill/reviews");
    fs::create_dir_all(&reviews).expect("create review report directory");
    fs::write(reviews.join(format!("{review_id}.md")), contents).expect("write review report");
}

fn claim_existing_evolution(root: &Path) -> Value {
    claim_existing_evolution_as(
        root,
        "evt_evolution_claim",
        "rev_fixture",
        "lock_evolution_claim",
    )
}

fn claim_existing_evolution_as(
    root: &Path,
    event_id: &str,
    review_id: &str,
    lock_owner: &str,
) -> Value {
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "claim", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            event_id,
            "--review-id",
            review_id,
            "--repository-head",
            "fixture-head",
        ]);
    lifecycle_clock(&mut command, lock_owner);
    let output = command.output().expect("claim Skill Evolution review");
    assert!(
        output.status.success(),
        "claim failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    write_review_report(root, review_id, "");
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
    run_evolution_close_for_review(root, event_id, "rev_fixture", disposition, note)
}

fn run_evolution_close_for_review(
    root: &Path,
    event_id: &str,
    review_id: &str,
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
            review_id,
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

/// The refusal is what an operator sees when they try to evolve a target whose evidence a
/// blocked close retired. Reporting a bare "collecting" there, while the capture reply on
/// the very same store names the retired incidents, invites them to go looking for a gate
/// that is never going to open on that evidence.
#[test]
fn skill_evolution_preflight_refusal_names_evidence_retired_by_a_blocked_close() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let close = run_evolution_close(
        fixture.path(),
        "evt_blocked_close",
        "blocked_no_valid_test",
        Some("no fresh trial can vary the binding run length"),
    );
    assert!(
        close.status.success(),
        "{}",
        String::from_utf8_lossy(&close.stderr)
    );

    let refusal = run_evolution_preflight(
        fixture.path(),
        "lock_blocked_refusal_preflight",
        "fixture-session",
    );
    assert_eq!(refusal.status.code(), Some(3));
    let message = String::from_utf8_lossy(&refusal.stderr);
    assert!(message.contains("Gate: collecting"), "message={message}");
    assert!(
        message
            .contains("Retired as untestable: 3 (an earlier close could not decide that evidence"),
        "the refusal must say why this evidence will never open the gate: message={message}"
    );
}

/// The packet is the reviewer's whole world — `authorized-review.md` tells them to work
/// from it and not to ingest the ledger. So a packet that shows more open incidents than
/// its clusters account for, without saying why, asks the reviewer to hand-derive the
/// difference from `prior_reviews`, which is exactly the derivation this crate exists to
/// centralize.
#[test]
fn skill_evolution_preflight_packet_names_evidence_retired_by_a_blocked_close() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let close = run_evolution_close(
        fixture.path(),
        "evt_blocked_close",
        "blocked_no_valid_test",
        Some("no fresh trial can vary the binding run length"),
    );
    assert!(
        close.status.success(),
        "{}",
        String::from_utf8_lossy(&close.stderr)
    );
    // A different symptom, recorded after the close, so the gate reopens on evidence the
    // blocked review never covered.
    record_outcome(fixture.path(), "task d", "session-d", "output", "friction");
    record_outcome(fixture.path(), "task e", "session-e", "output", "friction");
    record_outcome(fixture.path(), "task f", "session-f", "output", "friction");

    let output = run_evolution_preflight(
        fixture.path(),
        "lock_blocked_reentry_preflight",
        "fixture-session",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("preflight receipt JSON");
    let packet = &receipt["evidence_packet"];
    let retired = packet["instrument_limited_incident_ids"]
        .as_array()
        .expect("packet names the retired evidence")
        .clone();
    assert_eq!(retired.len(), 3, "packet={packet}");

    let open = packet["open_incident_ids"]
        .as_array()
        .expect("packet open incidents");
    let clustered = packet["candidate_clusters"]
        .as_array()
        .expect("packet clusters")
        .iter()
        .flat_map(|cluster| {
            cluster["open_event_ids"]
                .as_array()
                .expect("cluster open events")
                .clone()
        })
        .collect::<Vec<_>>();
    for identity in &retired {
        assert!(
            open.contains(identity),
            "retired evidence stays open — nothing was adjudicated: packet={packet}"
        );
        assert!(
            !clustered.contains(identity),
            "retired evidence must not cluster: packet={packet}"
        );
    }
}

#[test]
fn skill_evolution_preflight_omits_an_unverified_historical_report_path() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let close = run_evolution_close(
        fixture.path(),
        "evt_historical_close",
        "monitor_for_recurrence",
        Some("historical close without structured effort"),
    );
    assert!(
        close.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&close.stderr)
    );
    fs::remove_file(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/reviews/rev_fixture.md"),
    )
    .expect("remove historical report after close");
    record_outcome(fixture.path(), "task d", "session-d", "output", "friction");
    record_outcome(fixture.path(), "task e", "session-e", "output", "friction");
    record_outcome(fixture.path(), "task f", "session-f", "output", "friction");

    let output = run_evolution_preflight(
        fixture.path(),
        "lock_preflight_without_historical_report",
        "fixture-session",
    );

    assert!(
        output.status.success(),
        "historical stream must remain readable: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("preflight receipt JSON");
    let prior = receipt["evidence_packet"]["prior_reviews"][0]
        .as_object()
        .expect("prior review entry");
    assert!(!prior.contains_key("report"));
    assert!(!prior.contains_key("trial_count"));
    assert!(!prior.contains_key("artifacts_path"));
}

#[test]
fn skill_evolution_preflight_carries_prior_close_validation_effort() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let mut close = skill_evidence();
    close
        .args(["skills", "evolution", "close", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_prior_close_with_effort",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--disposition",
            "monitor_for_recurrence",
            "--note",
            "two-run arm reproduced the condition without the failure",
            "--trials",
            "2",
            "--artifacts",
            "reports/skill-evidence/demo-skill/reviews/rev_fixture/trials",
        ]);
    lifecycle_clock(&mut close, "lock_prior_close_with_effort");
    let close = close.output().expect("close review with asserted effort");
    assert!(
        close.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&close.stderr)
    );
    record_outcome(fixture.path(), "task d", "session-d", "output", "friction");
    record_outcome(fixture.path(), "task e", "session-e", "output", "friction");
    record_outcome(fixture.path(), "task f", "session-f", "output", "friction");

    let output = run_evolution_preflight(
        fixture.path(),
        "lock_preflight_with_prior_effort",
        "fixture-session",
    );

    assert!(
        output.status.success(),
        "preflight failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("preflight receipt JSON");
    let prior = &receipt["evidence_packet"]["prior_reviews"][0];
    assert_eq!(prior["trial_count"], 2);
    assert_eq!(
        prior["artifacts_path"],
        "reports/skill-evidence/demo-skill/reviews/rev_fixture/trials"
    );
    assert_eq!(prior["report"], "reviews/rev_fixture.md");
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
fn skill_evolution_material_recurrence_close_reports_its_narrow_retirement_reach() {
    let clustered = repository_with_demo_skill();
    let friction = record_outcome(
        clustered.path(),
        "frictional sibling",
        "session-a",
        "execution",
        "friction",
    );
    let material_one = record_outcome(
        clustered.path(),
        "material one",
        "session-b",
        "execution",
        "material_failure",
    );
    let material_two = record_outcome(
        clustered.path(),
        "material two",
        "session-c",
        "execution",
        "material_failure",
    );
    claim_existing_evolution(clustered.path());
    let post_claim = record_outcome(
        clustered.path(),
        "post-claim sibling",
        "session-d",
        "execution",
        "friction",
    );

    let close = run_evolution_close(
        clustered.path(),
        "evt_blocked_close_reach",
        "blocked_no_valid_test",
        Some("no fresh trial can vary the binding run length"),
    );

    assert!(
        close.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&close.stderr)
    );
    let receipt: Value = serde_json::from_slice(&close.stdout).expect("close receipt JSON");
    let coverage = vec![material_one.clone(), material_two.clone()];
    let reach = vec![material_one, material_two];
    assert_eq!(
        receipt["adjudicated_event_ids"],
        serde_json::to_value(&coverage).expect("coverage JSON")
    );
    assert_eq!(
        receipt["retired_from_gate_event_ids"],
        serde_json::to_value(&reach).expect("retirement reach JSON")
    );
    let projection = gate(clustered.path());
    assert_eq!(
        receipt["retired_from_gate_event_ids"],
        projection["instrument_limited_incident_ids"]
    );
    assert_eq!(
        projection["candidate_clusters"][0]["open_event_ids"],
        serde_json::json!([friction, post_claim]),
        "friction siblings the material authorization reason could not name remain open and clustering"
    );
    let stream = fs::read_to_string(
        clustered
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let disposition: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_blocked_close_reach")
        .expect("review_disposition event");
    assert_eq!(
        disposition["payload"],
        serde_json::json!({
            "review_id": "rev_fixture",
            "disposition": "blocked_no_valid_test",
            "adjudicated_event_ids": coverage,
            "note": "no fresh trial can vary the binding run length",
            "operating_skill_hash": expected_operating_skill_hash()
        }),
        "the receipt-only retirement reach must not change recorded history"
    );
}

#[test]
fn skill_evolution_instrument_limited_close_reports_empty_reach_for_a_severe_incident() {
    let severe = repository_with_demo_skill();
    let friction = record_outcome(
        severe.path(),
        "same-symptom friction sibling",
        "friction-session",
        "execution",
        "friction",
    );
    record_outcome(
        severe.path(),
        "lone severe incident",
        "severe-session",
        "execution",
        "severe_incident",
    );
    claim_existing_evolution(severe.path());
    let close = run_evolution_close(
        severe.path(),
        "evt_blocked_close_severe",
        "blocked_no_valid_test",
        Some("the instrument cannot vary the binding constraint"),
    );

    assert!(
        close.status.success(),
        "severe close failed: {}",
        String::from_utf8_lossy(&close.stderr)
    );
    let receipt: Value = serde_json::from_slice(&close.stdout).expect("severe close receipt JSON");
    assert_eq!(
        receipt["retired_from_gate_event_ids"],
        serde_json::json!([]),
        "an empty retirement reach is meaningful and must remain present"
    );
    assert_eq!(receipt["state"], "quarantined_eligible");
    let projection = gate(severe.path());
    assert_eq!(
        projection["candidate_clusters"][0]["open_event_ids"][0], friction,
        "a severe authorization reason cannot retire a friction sibling"
    );
}

#[test]
fn skill_evolution_non_instrument_limited_close_omits_retirement_reach_after_a_blocked_close() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let blocked = run_evolution_close(
        fixture.path(),
        "evt_first_blocked_close",
        "blocked_no_valid_test",
        Some("the first cluster cannot be tested by this instrument"),
    );
    assert!(
        blocked.status.success(),
        "blocked close failed: {}",
        String::from_utf8_lossy(&blocked.stderr)
    );
    let blocked: Value =
        serde_json::from_slice(&blocked.stdout).expect("blocked close receipt JSON");
    let earlier_reach = blocked["retired_from_gate_event_ids"]
        .as_array()
        .expect("blocked close retirement reach")
        .clone();
    assert!(!earlier_reach.is_empty());

    record_outcome(
        fixture.path(),
        "later output one",
        "session-d",
        "output",
        "friction",
    );
    record_outcome(
        fixture.path(),
        "later output two",
        "session-e",
        "output",
        "friction",
    );
    record_outcome(
        fixture.path(),
        "later output three",
        "session-f",
        "output",
        "friction",
    );
    claim_existing_evolution_as(
        fixture.path(),
        "evt_later_claim",
        "rev_later",
        "lock_later_claim",
    );
    let later = run_evolution_close_for_review(
        fixture.path(),
        "evt_later_close",
        "rev_later",
        "monitor_for_recurrence",
        Some("the current arm reproduced the condition without the failure"),
    );

    assert!(
        later.status.success(),
        "later close failed: {}",
        String::from_utf8_lossy(&later.stderr)
    );
    let receipt: Value = serde_json::from_slice(&later.stdout).expect("later close receipt JSON");
    assert!(
        receipt.get("retired_from_gate_event_ids").is_none(),
        "a later adjudicating close must not report an earlier close's retirement: {receipt}"
    );
    assert_eq!(
        gate(fixture.path())["instrument_limited_incident_ids"],
        Value::Array(earlier_reach),
        "the omission must hold while the standing retired set remains non-empty"
    );
}

/// A close reports the incidents *it* moved out of the gate. Naming coverage an earlier
/// close already retired moves nothing, so it must not appear in this close's reach.
#[test]
fn naming_already_retired_coverage_adds_nothing_to_this_closes_reach() {
    let fixture = repository_with_demo_skill();
    let first_claim = claim_evolution(fixture.path());
    let already_retired = first_claim["trigger_event_ids"][0]
        .as_str()
        .expect("trigger event id")
        .to_owned();
    let blocked = run_evolution_close(
        fixture.path(),
        "evt_earlier_blocked_close",
        "blocked_no_valid_test",
        Some("the execution cluster cannot be tested by this instrument"),
    );
    assert!(
        blocked.status.success(),
        "first close failed: {}",
        String::from_utf8_lossy(&blocked.stderr)
    );
    for session in ["session-e", "session-f", "session-g"] {
        record_outcome(fixture.path(), "output task", session, "output", "friction");
    }
    claim_existing_evolution_as(
        fixture.path(),
        "evt_second_claim",
        "rev_second",
        "lock_second_claim",
    );
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "close", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_second_close",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_second",
            "--disposition",
            "monitor_for_recurrence",
            "--note",
            "the output cluster did not reproduce",
            "--adjudicate",
            &already_retired,
            "--instrument-limited",
            &already_retired,
        ]);
    lifecycle_clock(&mut command, "lock_second_close");

    let output = command.output().expect("close second review");

    assert!(
        output.status.success(),
        "second close failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("second close receipt JSON");
    assert_eq!(
        receipt["retired_from_gate_event_ids"],
        serde_json::json!([]),
        "an earlier close already moved that incident out of the gate, so this one moved nothing"
    );
}

#[test]
fn skill_evolution_instrument_limited_close_reports_only_its_own_retirement_reach() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let first = run_evolution_close(
        fixture.path(),
        "evt_first_instrument_limited_close",
        "blocked_no_valid_test",
        Some("the execution cluster cannot be tested by this instrument"),
    );
    assert!(
        first.status.success(),
        "first close failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    let first: Value = serde_json::from_slice(&first.stdout).expect("first close receipt JSON");
    let first_reach = first["retired_from_gate_event_ids"]
        .as_array()
        .expect("first retirement reach")
        .clone();

    let second_reach = vec![
        record_outcome(
            fixture.path(),
            "output one",
            "session-d",
            "output",
            "friction",
        ),
        record_outcome(
            fixture.path(),
            "output two",
            "session-e",
            "output",
            "friction",
        ),
        record_outcome(
            fixture.path(),
            "output three",
            "session-f",
            "output",
            "friction",
        ),
    ];
    claim_existing_evolution_as(
        fixture.path(),
        "evt_second_instrument_limited_claim",
        "rev_second_instrument_limited",
        "lock_second_instrument_limited_claim",
    );
    let second = run_evolution_close_for_review(
        fixture.path(),
        "evt_second_instrument_limited_close",
        "rev_second_instrument_limited",
        "blocked_no_valid_test",
        Some("the output cluster cannot be tested by this instrument"),
    );

    assert!(
        second.status.success(),
        "second close failed: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    let receipt: Value = serde_json::from_slice(&second.stdout).expect("second close receipt JSON");
    assert_eq!(
        receipt["retired_from_gate_event_ids"],
        serde_json::to_value(&second_reach).expect("second reach JSON"),
        "the receipt names this close's retirement reach, not the standing retired set"
    );
    let mut standing = first_reach;
    standing.extend(second_reach.into_iter().map(Value::String));
    assert_eq!(
        gate(fixture.path())["instrument_limited_incident_ids"],
        Value::Array(standing),
        "the projection remains the standing per-hash retired set"
    );
}

#[test]
fn skill_evolution_close_records_optional_validation_effort_as_asserted() {
    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let mut command = skill_evidence();
    command
        .args(["skills", "evolution", "close", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_close_with_effort",
            "--repository-head",
            "fixture-head",
            "--review-id",
            "rev_fixture",
            "--disposition",
            "monitor_for_recurrence",
            "--note",
            "two-run arm reproduced the condition without the failure",
            "--trials",
            "2",
            "--artifacts",
            "reports/skill-evidence/demo-skill/reviews/rev_fixture/trials",
        ]);
    lifecycle_clock(&mut command, "lock_close_with_effort");

    let output = command.output().expect("close review with asserted effort");

    assert!(
        output.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let disposition: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_close_with_effort")
        .expect("review_disposition event");
    assert_eq!(
        disposition["payload"],
        serde_json::json!({
            "review_id": "rev_fixture",
            "disposition": "monitor_for_recurrence",
            "adjudicated_event_ids": claim["trigger_event_ids"],
            "note": "two-run arm reproduced the condition without the failure",
            "trial_count": 2,
            "artifacts_path": "reports/skill-evidence/demo-skill/reviews/rev_fixture/trials",
            "operating_skill_hash": expected_operating_skill_hash()
        })
    );
    let projection = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/gate-status.json"),
    )
    .expect("read gate projection");
    assert!(!projection.contains("trial_count"));
    assert!(!projection.contains("artifacts_path"));
    assert_event_stream_matches_the_published_schema(fixture.path());
}

#[test]
fn skill_evolution_close_refuses_when_the_review_report_is_absent_without_writing() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let store = fixture.path().join("reports/skill-evidence/demo-skill");
    fs::remove_file(store.join("reviews/rev_fixture.md"))
        .expect("remove review report for refusal case");
    let stream_path = store.join("events.jsonl");
    let projection_path = store.join("gate-status.json");
    let stream_before = fs::read(&stream_path).expect("read event stream before refusal");
    let projection_before =
        fs::read(&projection_path).expect("read gate projection before refusal");

    let output = run_evolution_close(
        fixture.path(),
        "evt_close_without_report",
        "monitor_for_recurrence",
        Some("current arm reproduced the condition"),
    );

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("reports/skill-evidence/demo-skill/reviews/rev_fixture.md"),
        "refusal must name the expected report path: {error}"
    );
    assert_eq!(
        fs::read(&stream_path).expect("read event stream after refusal"),
        stream_before,
        "a missing report must append no event"
    );
    assert_eq!(
        fs::read(&projection_path).expect("read gate projection after refusal"),
        projection_before,
        "a missing report must not rewrite the projection"
    );
    assert!(
        !store.join(".lock").exists(),
        "the refusal must release the evidence-store lock"
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
fn skill_evolution_close_records_coverage_its_instrument_could_not_test() {
    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let untestable = claim["trigger_event_ids"][0]
        .as_str()
        .expect("trigger event id")
        .to_owned();
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
            "monitor_for_recurrence",
            "--note",
            "two mechanisms did not reproduce; the third could not be expressed",
            "--instrument-limited",
            &untestable,
        ]);
    lifecycle_clock(&mut command, "lock_evolution_close");

    let output = command.output().expect("close Skill Evolution review");

    assert!(
        output.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
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
    assert_eq!(
        disposition["payload"]["instrument_limited_event_ids"],
        serde_json::json!([untestable]),
        "the close records which of its coverage it reached no conclusion about"
    );
    assert_eq!(
        disposition["payload"]["adjudicated_event_ids"]
            .as_array()
            .expect("coverage list")
            .len(),
        3,
        "the coverage list still names every trigger the claim carried"
    );
    let projection = gate(fixture.path());
    assert_eq!(
        projection["open_incident_ids"],
        serde_json::json!([untestable]),
        "an untestable trigger stays open, because the review concluded nothing about it"
    );
    assert_eq!(
        projection["instrument_limited_incident_ids"],
        serde_json::json!([untestable]),
        "and it retires from the gate as untestable rather than as adjudicated"
    );
    assert_eq!(projection["candidate_clusters"], serde_json::json!([]));
    assert_event_stream_matches_the_published_schema(fixture.path());
}

fn close_naming_untestable_coverage(
    root: &Path,
    event_id: &str,
    disposition: &str,
    named: &[&str],
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
            "--note",
            "note",
        ]);
    for identity in named {
        command.args(["--instrument-limited", identity]);
    }
    lifecycle_clock(&mut command, "lock_untestable_close");
    command.output().expect("close Skill Evolution review")
}

#[test]
fn close_refuses_naming_untestable_coverage_for_a_non_adjudicating_disposition() {
    for disposition in ["blocked_no_valid_test", "superseded_by_target_version"] {
        let fixture = repository_with_demo_skill();
        let claim = claim_evolution(fixture.path());
        let trigger = claim["trigger_event_ids"][0]
            .as_str()
            .expect("trigger event id")
            .to_owned();
        let stream_path = fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl");
        let stream_before = fs::read(&stream_path).expect("read event stream before refusal");

        let output = close_naming_untestable_coverage(
            fixture.path(),
            "evt_untestable_non_adjudicating",
            disposition,
            &[&trigger],
        );

        assert_eq!(output.status.code(), Some(3), "disposition {disposition}");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(
            error.contains("--instrument-limited"),
            "{disposition}: {error}"
        );
        assert!(error.contains(disposition), "{disposition}: {error}");
        assert_eq!(
            fs::read(&stream_path).expect("read event stream after refusal"),
            stream_before,
            "disposition {disposition}"
        );
    }
}

#[test]
fn close_refuses_naming_untestable_coverage_it_does_not_cover() {
    for (case, outside) in [
        ("an incident that arrived after the threshold froze", None),
        ("an event id no stream holds", Some("evt_absent")),
    ] {
        let fixture = repository_with_demo_skill();
        claim_evolution(fixture.path());
        let outside = outside.map_or_else(
            || {
                record_outcome(
                    fixture.path(),
                    "task d",
                    "session-d",
                    "execution",
                    "friction",
                )
            },
            str::to_owned,
        );
        let stream_path = fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl");
        let stream_before = fs::read(&stream_path).expect("read event stream before refusal");

        let output = close_naming_untestable_coverage(
            fixture.path(),
            "evt_untestable_outside_coverage",
            "monitor_for_recurrence",
            &[&outside],
        );

        assert_eq!(output.status.code(), Some(3), "{case}");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains(&outside), "{case}: {error}");
        assert_eq!(
            fs::read(&stream_path).expect("read event stream after refusal"),
            stream_before,
            "{case}"
        );
    }
}

/// The review both issues were filed about: every covered trigger is conformance-only, so
/// none may reach the outcome-graded verdict, while the candidate was genuinely built and
/// rejected. Naming the whole coverage list must be allowed. Routing such a review to
/// `blocked_no_valid_test` instead would retire its whole symptom cluster — wider than
/// this close ever covered — and declare untestable a mechanism its trials tested.
#[test]
fn an_adjudicating_close_may_name_its_whole_coverage_list_without_widening() {
    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let triggers = claim["trigger_event_ids"]
        .as_array()
        .expect("trigger event ids")
        .iter()
        .map(|identity| identity.as_str().expect("trigger event id").to_owned())
        .collect::<Vec<_>>();
    let named = triggers.iter().map(String::as_str).collect::<Vec<_>>();
    let sibling = record_outcome(
        fixture.path(),
        "task d",
        "session-d",
        "execution",
        "friction",
    );

    let output = close_naming_untestable_coverage(
        fixture.path(),
        "evt_untestable_whole_coverage",
        "monitor_for_recurrence",
        &named,
    );

    assert!(
        output.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let projection = gate(fixture.path());
    let mut retired = projection["instrument_limited_incident_ids"]
        .as_array()
        .expect("instrument limited incident ids")
        .iter()
        .map(|identity| identity.as_str().expect("event id").to_owned())
        .collect::<Vec<_>>();
    retired.sort();
    let mut expected = triggers.clone();
    expected.sort();
    assert_eq!(
        retired, expected,
        "reach is the names and nothing else, so an uncovered sibling is untouched"
    );
    assert!(
        projection["open_incident_ids"]
            .as_array()
            .expect("open incident ids")
            .iter()
            .any(|identity| identity == &serde_json::json!(sibling)),
        "the sibling this close never covered stays open"
    );
    assert_eq!(
        projection["candidate_clusters"]
            .as_array()
            .expect("candidate clusters")
            .len(),
        1,
        "and stays in its cluster, where only genuinely new evidence can reach a threshold"
    );
}

/// `--adjudicate` dedups before it writes. This list lands in the same append-only
/// event and can never be corrected, so it must too — `["X","X"]` is a shape
/// `adjudicated_event_ids` can never take.
#[test]
fn close_records_repeated_untestable_names_once() {
    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let untestable = claim["trigger_event_ids"][0]
        .as_str()
        .expect("trigger event id")
        .to_owned();

    let output = close_naming_untestable_coverage(
        fixture.path(),
        "evt_repeated_names_close",
        "monitor_for_recurrence",
        &[&untestable, &untestable],
    );

    assert!(
        output.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stream = fs::read_to_string(
        fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read event stream");
    let disposition: Value = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .find(|event| event["event_id"] == "evt_repeated_names_close")
        .expect("review_disposition event");
    assert_eq!(
        disposition["payload"]["instrument_limited_event_ids"],
        serde_json::json!([untestable])
    );
    assert_event_stream_matches_the_published_schema(fixture.path());
}

#[test]
fn an_adjudicating_close_reports_the_coverage_it_retired_as_untestable() {
    let fixture = repository_with_demo_skill();
    let claim = claim_evolution(fixture.path());
    let untestable = claim["trigger_event_ids"][0]
        .as_str()
        .expect("trigger event id")
        .to_owned();
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
            "monitor_for_recurrence",
            "--note",
            "one mechanism's binding constraint was inexpressible in a fresh session",
            "--instrument-limited",
            &untestable,
        ]);
    lifecycle_clock(&mut command, "lock_evolution_close");

    let output = command.output().expect("close Skill Evolution review");

    assert!(
        output.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout).expect("close receipt JSON");
    assert_eq!(
        receipt["retired_from_gate_event_ids"],
        serde_json::json!([untestable]),
        "retiring evidence silently would trade one dishonest surface for another"
    );
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

/// A wholly conformance-only cluster reaches step 7, is graded on outcome it never
/// claimed, and must still land somewhere. The installed reference sends it to
/// `blocked_no_valid_test` when no trial could express any mechanism, and says a rejected
/// candidate does not bar that disposition, so the compiled command has to agree.
#[test]
fn a_rejected_candidate_does_not_bar_the_blocked_disposition() {
    let fixture = repository_with_demo_skill();
    claim_evolution(fixture.path());
    let candidate = make_candidate(
        fixture.path(),
        "candidate graded on outcome it never claimed",
    );
    let mut reject_validation = skill_evidence();
    reject_validation
        .args(["skills", "evolution", "record-validation", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--event-id",
            "evt_conformance_rejected_validation",
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
        .arg(&candidate)
        .args([
            "--trials",
            "3",
            "--artifacts",
            "reports/skill-evidence/demo-skill/reviews/trials",
            "--summary",
            "no outcome deficit demonstrated for any covered trigger",
        ]);
    lifecycle_clock(
        &mut reject_validation,
        "lock_conformance_rejected_validation",
    );
    let rejection = reject_validation
        .output()
        .expect("record rejected validation");
    assert!(
        rejection.status.success(),
        "record-validation failed: {}",
        String::from_utf8_lossy(&rejection.stderr)
    );

    let close = run_evolution_close(
        fixture.path(),
        "evt_conformance_blocked_close",
        "blocked_no_valid_test",
        Some("every trigger is conformance-only and the acceptance gate grades outcome"),
    );

    assert!(
        close.status.success(),
        "close failed: {}",
        String::from_utf8_lossy(&close.stderr)
    );
    let receipt: Value = serde_json::from_slice(&close.stdout).expect("close receipt JSON");
    assert_eq!(receipt["disposition"], "blocked_no_valid_test");
    assert_eq!(
        gate(fixture.path())["open_incident_ids"]
            .as_array()
            .expect("open incident ids")
            .len(),
        3,
        "a non-adjudicating close concludes about nothing, so every trigger stays open"
    );
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

    write_review_report(
        fixture.path(),
        "rev_e32e7983-e876-4cf3-8537-019d2e37ce84",
        "",
    );

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
    let operating_skill_hash = expected_operating_skill_hash();
    let mut expected_lines = expected_text
        .lines()
        .take(7)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    for line in &mut expected_lines[3..] {
        assert!(line.ends_with("}}"), "lifecycle event closes payload last");
        line.insert_str(
            line.len() - 2,
            &format!(",\"operating_skill_hash\":\"{operating_skill_hash}\""),
        );
    }
    let expected_prefix = format!("{}\n", expected_lines.join("\n"));
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
