#![forbid(unsafe_code)]

mod support;

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::{Value, json};
use support::{repository_root, skill_evidence};

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create copied fixture directory");
    for entry in fs::read_dir(source).expect("read fixture directory") {
        let entry = entry.expect("read fixture entry");
        let target = destination.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy fixture file");
        }
    }
}

fn parity_fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().expect("temporary parity repository");
    let source = repository_root().join("fixtures/skill-evidence/status-reporters-v1");
    copy_tree(&source.join(".claude"), &root.path().join(".claude"));
    copy_tree(&source.join("reports"), &root.path().join("reports"));
    for arguments in [
        vec!["init", "--quiet"],
        vec!["add", "."],
        vec!["commit", "--quiet", "-m", "Fixture baseline"],
    ] {
        let output = Command::new("git")
            .args(arguments)
            .current_dir(root.path())
            .env("GIT_AUTHOR_NAME", "Fixture")
            .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
            .env("GIT_COMMITTER_NAME", "Fixture")
            .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
            .env("GIT_AUTHOR_DATE", "2026-07-25T12:00:00Z")
            .env("GIT_COMMITTER_DATE", "2026-07-25T12:00:00Z")
            .env("LC_ALL", "C")
            .env("TZ", "UTC")
            .output()
            .expect("run Git fixture command");
        assert!(
            output.status.success(),
            "git fixture command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    root
}

fn skill_hash(root: &std::path::Path, target: &str) -> String {
    let output = skill_evidence()
        .args(["skills", "evidence", "hash", "--root"])
        .arg(root)
        .args(["--target", target])
        .output()
        .expect("hash fixture skill");
    assert!(output.status.success());
    serde_json::from_slice::<Value>(&output.stdout).expect("hash JSON")["content_hash"]
        .as_str()
        .expect("content hash")
        .to_owned()
}

fn fixture_use_event(
    target_name: &str,
    target_hash: &str,
    event_id: &str,
    recorded_at: &str,
    session_id: &str,
    outcome: &str,
) -> Value {
    let clean = outcome == "clean";
    json!({
        "schema_version": 1,
        "event_id": event_id,
        "event_type": "use_recorded",
        "recorded_at": recorded_at,
        "operator_workflow": "skill-evidence-capture",
        "target": {
            "name": target_name,
            "repo_relative_path": format!(".claude/skills/{target_name}"),
            "content_hash": target_hash,
            "repo_head": "fixture-head"
        },
        "top_level_session_id": session_id,
        "payload": {
            "qualifying_use": true,
            "retrospective": false,
            "task_label": event_id,
            "task_fingerprint": event_id,
            "outcome": outcome,
            "symptom_key": (!clean).then_some("execution"),
            "expected": (!clean).then_some("expected"),
            "observed": (!clean).then_some("observed"),
            "consequence": (!clean).then_some("consequence"),
            "workaround_taken": Value::Null,
            "evidence_refs": [],
            "same_run_group": event_id
        }
    })
}

/// The two events a blocked Skill Evolution review leaves behind: the claim that froze
/// `trigger_event_ids`, and the close that covers them. Every test that needs this pair
/// differs only in the target, the review id, the rule that authorized it, and coverage.
fn fixture_blocked_close(
    target_name: &str,
    target_hash: &str,
    review_id: &str,
    authorizing_rule: &str,
    covered: &[&str],
) -> [Value; 2] {
    let target = json!({
        "name": target_name,
        "repo_relative_path": format!(".claude/skills/{target_name}"),
        "content_hash": target_hash,
        "repo_head": "fixture-head"
    });
    [
        json!({
            "schema_version": 1,
            "event_id": format!("evt_review_started_{review_id}"),
            "event_type": "review_started",
            "recorded_at": "2026-07-21T11:59:50.000Z",
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": {
                "review_id": review_id,
                "target_hash": target_hash,
                "trigger_event_ids": covered,
                "authorizing_rule": authorizing_rule,
                "risk_tier": "provisional",
                "session_or_cooldown_proof": { "type": "different_session" }
            }
        }),
        json!({
            "schema_version": 1,
            "event_id": format!("evt_review_disposition_{review_id}"),
            "event_type": "review_disposition",
            "recorded_at": "2026-07-21T11:59:51.000Z",
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": {
                "review_id": review_id,
                "disposition": "blocked_no_valid_test",
                "adjudicated_event_ids": covered
            }
        }),
    ]
}

fn create_fixture_skill(root: &Path, name: &str) {
    let skill = root.join(".claude/skills").join(name);
    fs::create_dir_all(&skill).expect("create fixture skill");
    fs::write(
        skill.join("SKILL.md"),
        format!("---\nname: {name}\n---\n# {name}\n"),
    )
    .expect("write fixture skill");
}

struct ReviewStartedFixture<'a> {
    target_name: &'a str,
    target_hash: &'a str,
    event_id: &'a str,
    event_type: &'a str,
    recorded_at: &'a str,
    review_id: &'a str,
    trigger_event_ids: &'a [&'a str],
    authorizing_rule: &'a str,
}

fn fixture_review_started(fixture: ReviewStartedFixture<'_>) -> Value {
    let ReviewStartedFixture {
        target_name,
        target_hash,
        event_id,
        event_type,
        recorded_at,
        review_id,
        trigger_event_ids,
        authorizing_rule,
    } = fixture;
    json!({
        "schema_version": 1,
        "event_id": event_id,
        "event_type": event_type,
        "recorded_at": recorded_at,
        "operator_workflow": if event_type == "review_started" {
            "skill-evolution"
        } else {
            "legacy-skill-decontamination"
        },
        "target": {
            "name": target_name,
            "repo_relative_path": format!(".claude/skills/{target_name}"),
            "content_hash": target_hash,
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": review_id,
            "target_hash": target_hash,
            "trigger_event_ids": trigger_event_ids,
            "authorizing_rule": authorizing_rule,
            "risk_tier": "provisional",
            "session_or_cooldown_proof": { "type": "different_session" }
        }
    })
}

fn fixture_review_disposition(
    target_name: &str,
    target_hash: &str,
    event_id: &str,
    recorded_at: &str,
    review_id: &str,
    adjudicated_event_ids: &[&str],
) -> Value {
    json!({
        "schema_version": 1,
        "event_id": event_id,
        "event_type": "review_disposition",
        "recorded_at": recorded_at,
        "operator_workflow": "skill-evolution",
        "target": {
            "name": target_name,
            "repo_relative_path": format!(".claude/skills/{target_name}"),
            "content_hash": target_hash,
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": review_id,
            "disposition": "candidate_rejected_validation",
            "adjudicated_event_ids": adjudicated_event_ids
        }
    })
}

fn write_events(root: &std::path::Path, store_key: &str, events: &[Value]) {
    let store = root.join("reports/skill-evidence").join(store_key);
    fs::create_dir_all(&store).expect("create fixture evidence store");
    fs::write(
        store.join("events.jsonl"),
        events
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            + if events.is_empty() { "" } else { "\n" },
    )
    .expect("write fixture event stream");
}

struct FileSnapshot {
    bytes: Vec<u8>,
    modified: std::time::SystemTime,
}

fn snapshot_files(paths: &[PathBuf]) -> Vec<FileSnapshot> {
    paths
        .iter()
        .map(|path| FileSnapshot {
            bytes: fs::read(path).expect("read governed fixture file"),
            modified: fs::metadata(path)
                .expect("read governed fixture metadata")
                .modified()
                .expect("read governed fixture mtime"),
        })
        .collect()
}

fn assert_files_unchanged(paths: &[PathBuf], before: &[FileSnapshot]) {
    for (path, snapshot) in paths.iter().zip(before) {
        assert_eq!(
            fs::read(path).expect("reread governed fixture file"),
            snapshot.bytes
        );
        assert_eq!(
            fs::metadata(path)
                .expect("reread governed fixture metadata")
                .modified()
                .expect("reread governed fixture mtime"),
            snapshot.modified
        );
    }
}

#[test]
fn method_gap_research_status_refuses_an_unbounded_family_selector() {
    let output = skill_evidence()
        .args([
            "skills",
            "method-gap-research-status",
            "*",
            "--root",
            ".",
            "--now-epoch-milliseconds",
            "1784980800000",
        ])
        .output()
        .expect("run Method-Gap Research Status");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Invalid family selector"),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn method_gap_research_status_refuses_an_empty_terminal_family_segment() {
    let output = skill_evidence()
        .args([
            "skills",
            "method-gap-research-status",
            "game--*",
            "--root",
            ".",
            "--now-epoch-milliseconds",
            "1784980800000",
        ])
        .output()
        .expect("run Method-Gap Research Status");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Invalid family selector"));
}

/// Retired incidents leave `current_candidate_clusters` but stay in the per-incident
/// evidence summary, so without naming them the inventory reads as though it lost track
/// of its own incidents rather than recording a decision about them.
#[test]
fn method_gap_research_status_names_evidence_retired_by_a_blocked_close() {
    let root = tempfile::tempdir().expect("temporary repository");
    create_fixture_skill(root.path(), "game-retired");
    let inventory = |root: &Path| {
        let output = skill_evidence()
            .args(["skills", "method-gap-research-status", "game-*", "--root"])
            .arg(root)
            .args(["--now-epoch-milliseconds", "1784980800000"])
            .output()
            .expect("run Method-Gap Research Status");
        assert!(output.status.success());
        serde_json::from_slice::<Value>(&output.stdout).expect("inventory JSON")
    };

    let hash = inventory(root.path())["targets"][0]["target_content_hash"]
        .as_str()
        .expect("current target hash")
        .to_owned();
    let retired = ["evt_retired_a", "evt_retired_b", "evt_retired_c"];
    let mut events = retired
        .iter()
        .enumerate()
        .map(|(index, event_id)| {
            fixture_use_event(
                "game-retired",
                &hash,
                event_id,
                &format!("2026-07-21T11:59:4{index}.000Z"),
                &format!("retired-session-{index}"),
                "friction",
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture_blocked_close(
        "game-retired",
        &hash,
        "rev_retired",
        "friction_recurrence:execution",
        &retired,
    ));
    write_events(root.path(), "game-retired", &events);

    let target_inventory = inventory(root.path())["targets"][0].clone();
    assert_eq!(
        target_inventory["instrument_limited_incident_ids"],
        json!(retired)
    );
    assert_eq!(target_inventory["current_candidate_clusters"], json!([]));
}

#[test]
fn method_gap_research_status_separates_current_and_historical_evidence() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target = root.path().join(".claude/skills/game-evidence");
    fs::create_dir_all(&target).expect("create fixture skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: game-evidence\n---\n# game-evidence\n",
    )
    .expect("write fixture skill");

    let initial = skill_evidence()
        .args(["skills", "method-gap-research-status", "game-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", "1784980800000"])
        .output()
        .expect("run initial inventory");
    assert!(initial.status.success());
    let initial: Value = serde_json::from_slice(&initial.stdout).expect("initial inventory JSON");
    let current_hash = initial["targets"][0]["target_content_hash"]
        .as_str()
        .expect("current target hash");

    let event = |id: &str, hash: &str, outcome: &str, symptom: Option<&str>| {
        let clean = outcome == "clean";
        json!({
            "schema_version": 1,
            "event_id": id,
            "event_type": "use_recorded",
            "recorded_at": "2026-07-25T11:59:59.000Z",
            "operator_workflow": "skill-evidence-capture",
            "target": {
                "name": "game-evidence",
                "repo_relative_path": ".claude/skills/game-evidence",
                "content_hash": hash,
                "repo_head": "fixture-head"
            },
            "top_level_session_id": "fixture-session",
            "payload": {
                "qualifying_use": true,
                "retrospective": false,
                "task_label": id,
                "task_fingerprint": id,
                "outcome": outcome,
                "symptom_key": symptom,
                "expected": (!clean).then_some("expected"),
                "observed": (!clean).then_some("observed"),
                "consequence": (!clean).then_some("consequence"),
                "workaround_taken": Value::Null,
                "evidence_refs": [],
                "same_run_group": id
            }
        })
    };
    let events = [
        event("evt_old", "old-hash", "clean", None),
        event(
            "evt_current",
            current_hash,
            "material_failure",
            Some("unknown"),
        ),
    ];
    let store = root.path().join("reports/skill-evidence/game-evidence");
    fs::create_dir_all(&store).expect("create evidence store");
    fs::write(
        store.join("events.jsonl"),
        events
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n",
    )
    .expect("write evidence stream");
    let output = skill_evidence()
        .args(["skills", "method-gap-research-status", "game-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", "1784980800000"])
        .output()
        .expect("run evidence inventory");
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).expect("inventory JSON");
    let evidence = &report["targets"][0]["current_evidence"];
    assert_eq!(evidence["qualifying_uses"], 1);
    assert_eq!(evidence["outcome_counts"]["material_failure"], 1);
    assert_eq!(evidence["open_incidents"][0]["event_id"], "evt_current");
    assert_eq!(
        evidence["observed_target_hashes"].as_array().unwrap().len(),
        2
    );
}

/// `qualifying_uses` is the same domain term the gate projection counts, and the glossary
/// gives it one meaning: a run, not a record. A reporter still counting records publishes a
/// different number than `gate-status.json` for the same stream under the same name, and a
/// reader has no way to tell which of the two answers the question they asked.
#[test]
fn method_gap_research_status_counts_run_groups_rather_than_records() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target = root.path().join(".claude/skills/game-evidence");
    fs::create_dir_all(&target).expect("create fixture skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: game-evidence\n---\n# game-evidence\n",
    )
    .expect("write fixture skill");

    let inventory = || {
        let output = skill_evidence()
            .args(["skills", "method-gap-research-status", "game-*", "--root"])
            .arg(root.path())
            .args(["--now-epoch-milliseconds", "1784980800000"])
            .output()
            .expect("run evidence inventory");
        assert!(output.status.success());
        serde_json::from_slice::<Value>(&output.stdout).expect("inventory JSON")
    };
    let current_hash = inventory()["targets"][0]["target_content_hash"]
        .as_str()
        .expect("current target hash")
        .to_owned();

    let sibling = |id: &str, hash: &str, run_group: &str| {
        json!({
            "schema_version": 1,
            "event_id": id,
            "event_type": "use_recorded",
            "recorded_at": "2026-07-25T11:59:59.000Z",
            "operator_workflow": "skill-evidence-capture",
            "target": {
                "name": "game-evidence",
                "repo_relative_path": ".claude/skills/game-evidence",
                "content_hash": hash,
                "repo_head": "fixture-head"
            },
            "top_level_session_id": "fixture-session",
            "payload": {
                "qualifying_use": true,
                "retrospective": false,
                "task_label": run_group,
                "task_fingerprint": run_group,
                "outcome": "friction",
                "symptom_key": "execution",
                "expected": "expected",
                "observed": "observed",
                "consequence": "consequence",
                "workaround_taken": Value::Null,
                "run_condition": "condition",
                "evidence_refs": [],
                "same_run_group": run_group
            }
        })
    };
    // One run that deviated twice on the current hash, and one on a superseded hash.
    let events = [
        sibling("evt_old", "old-hash", "run-old"),
        sibling("evt_one", &current_hash, "run-current"),
        sibling("evt_two", &current_hash, "run-current"),
    ];
    let store = root.path().join("reports/skill-evidence/game-evidence");
    fs::create_dir_all(&store).expect("create evidence store");
    fs::write(
        store.join("events.jsonl"),
        events
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n",
    )
    .expect("write evidence stream");

    let report = inventory();
    let evidence = &report["targets"][0]["current_evidence"];
    assert_eq!(
        evidence["qualifying_uses"], 1,
        "two siblings record one run, so the current hash has seen one use"
    );
    assert_eq!(
        evidence["outcome_counts"]["friction"], 2,
        "outcome counts stay per record — they describe incidents, not exercise"
    );
    let current_row = evidence["observed_target_hashes"]
        .as_array()
        .expect("observed target hashes")
        .iter()
        .find(|row| row["content_hash"] == current_hash.as_str())
        .expect("the current hash is observed");
    assert_eq!(
        current_row["qualifying_uses"], 1,
        "the per-hash tally counts runs under the same name"
    );
}

#[test]
fn method_gap_research_status_discovers_only_target_identifying_lineage() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["game-arc", "game-arc-planning"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let archive = root.path().join("reports/archive");
    fs::create_dir_all(&archive).expect("create report archive");
    fs::write(
        root.path()
            .join("reports/game-arc-planning-method-gap-research-brief.md"),
        "# Research Brief — Method-Gap Audit of game-arc-planning\n",
    )
    .expect("write filename lineage");
    fs::write(
        archive.join("alternate.md"),
        "Method-gap disposition for `.claude/skills/game-arc`.\n",
    )
    .expect("write content lineage");

    let output = skill_evidence()
        .args(["skills", "method-gap-research-status", "game-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", "1784980800000"])
        .output()
        .expect("run lineage inventory");
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).expect("inventory JSON");
    let targets = report["targets"].as_array().expect("target inventory");
    let short = targets
        .iter()
        .find(|target| target["target_name"] == "game-arc")
        .expect("short target");
    let long = targets
        .iter()
        .find(|target| target["target_name"] == "game-arc-planning")
        .expect("long target");
    assert_eq!(
        short["lineage_candidates"][0]["path"],
        "reports/archive/alternate.md"
    );
    assert_eq!(short["lineage_candidates"].as_array().unwrap().len(), 1);
    assert_eq!(
        long["lineage_candidates"][0]["path"],
        "reports/game-arc-planning-method-gap-research-brief.md"
    );
    assert_eq!(long["lineage_candidates"].as_array().unwrap().len(), 1);
    assert!(
        short["lineage_candidates"][0]["signal_lines"]
            .as_array()
            .is_some_and(|lines| !lines.is_empty())
    );
}

#[test]
fn method_gap_research_status_preserves_the_legacy_audit_filename_classifier() {
    let root = tempfile::tempdir().expect("temporary repository");
    let skill = root.path().join(".claude/skills/game-audit");
    fs::create_dir_all(&skill).expect("create fixture skill");
    fs::write(
        skill.join("SKILL.md"),
        "---\nname: game-audit\n---\n# game-audit\n",
    )
    .expect("write fixture skill");
    let reports = root.path().join("reports");
    fs::create_dir_all(&reports).expect("create reports directory");
    fs::write(
        reports.join("game-audit-method-gap-audit.md"),
        "# Returned audit\n",
    )
    .expect("write legacy returned audit");
    fs::write(
        reports.join("game-audit-method-gap-audit.v1.md"),
        "# Versioned audit artifact\n",
    )
    .expect("write versioned audit artifact");

    let output = skill_evidence()
        .args(["skills", "method-gap-research-status", "game-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", "1784980800000"])
        .output()
        .expect("run lineage inventory");
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).expect("inventory JSON");
    let candidates = report["targets"][0]["lineage_candidates"]
        .as_array()
        .expect("lineage candidates");
    assert_eq!(candidates[0]["kind"], "returned_report");
    assert_eq!(candidates[1]["kind"], "other");
}

#[test]
fn skill_evolution_status_renders_a_complete_zero_store_census() {
    let root = tempfile::tempdir().expect("temporary repository");
    let operator = root.path().join(".claude/skills/skill-evolution");
    fs::create_dir_all(&operator).expect("create sibling evolution skill");
    fs::write(
        operator.join("SKILL.md"),
        "---\nname: skill-evolution\n---\n# Skill Evolution\n",
    )
    .expect("write sibling evolution skill");

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "unavailable",
        ])
        .output()
        .expect("run Skill Evolution Status");

    assert!(
        output.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 status report"),
        "# Skill Evolution Status\n\nScanned 0 evidence stores read-only. Ready: 0; deferred after review: 0; blocked after eligibility: 0; indeterminate: 0; omitted as not eligible: 0.\n\nNo eligible targets found.\n"
    );
    assert!(!root.path().join("reports").exists());
}

#[test]
fn skill_evolution_status_keeps_an_unelapsed_clock_threshold_blocked() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "timer-waiting"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let hash = skill_hash(root.path(), ".claude/skills/timer-waiting");
    let events = [
        fixture_use_event(
            "timer-waiting",
            &hash,
            "evt_timer_a",
            "2026-07-21T09:59:58.000Z",
            "unavailable",
            "friction",
        ),
        fixture_use_event(
            "timer-waiting",
            &hash,
            "evt_timer_b",
            "2026-07-21T09:59:59.000Z",
            "unavailable",
            "friction",
        ),
        fixture_use_event(
            "timer-waiting",
            &hash,
            "evt_timer_c",
            "2026-07-21T10:00:00.000Z",
            "unavailable",
            "friction",
        ),
    ];
    write_events(root.path(), "timer-waiting", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "unavailable",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("## Eligible but blocked"));
    assert!(report.contains("10h 00m remaining"));
    assert!(report.contains("2026-07-21T22:00:00.000Z"));
    assert!(report.contains("Changing sessions will not bypass this timer"));
    assert!(!report.contains("$skill-evolution"));
}

#[test]
fn skill_evolution_status_marks_an_elapsed_clock_threshold_ready() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "timer-ready"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let hash = skill_hash(root.path(), ".claude/skills/timer-ready");
    let events = [
        fixture_use_event(
            "timer-ready",
            &hash,
            "evt_elapsed_a",
            "2026-07-20T22:59:58.000Z",
            "unavailable",
            "friction",
        ),
        fixture_use_event(
            "timer-ready",
            &hash,
            "evt_elapsed_b",
            "2026-07-20T22:59:59.000Z",
            "unavailable",
            "friction",
        ),
        fixture_use_event(
            "timer-ready",
            &hash,
            "evt_elapsed_c",
            "2026-07-20T23:00:00.000Z",
            "unavailable",
            "friction",
        ),
    ];
    write_events(root.path(), "timer-ready", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "unavailable",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("## Ready to evolve"));
    assert!(report.contains("clock proof already passed"));
    assert!(report.contains("2026-07-21T11:00:00.000Z"));
    assert!(report.contains("$skill-evolution \".claude/skills/timer-ready\""));
}

#[test]
fn skill_evolution_status_requires_a_session_capable_host_for_session_thresholds() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "session-host-required"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let hash = skill_hash(root.path(), ".claude/skills/session-host-required");
    let events = [
        fixture_use_event(
            "session-host-required",
            &hash,
            "evt_host_a",
            "2026-07-21T10:59:58.000Z",
            "session-a",
            "friction",
        ),
        fixture_use_event(
            "session-host-required",
            &hash,
            "evt_host_b",
            "2026-07-21T10:59:59.000Z",
            "session-b",
            "friction",
        ),
        fixture_use_event(
            "session-host-required",
            &hash,
            "evt_host_c",
            "2026-07-21T11:00:00.000Z",
            "session-c",
            "friction",
        ),
    ];
    write_events(root.path(), "session-host-required", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "unavailable",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("session-ID-capable host"));
    assert!(report.contains("Waiting will not help"));
    assert!(!report.contains("$skill-evolution"));
}

#[test]
fn skill_evolution_status_reports_an_active_evolution_review_as_owned() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "claimed"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let hash = skill_hash(root.path(), ".claude/skills/claimed");
    let mut events = vec![
        fixture_use_event(
            "claimed",
            &hash,
            "evt_claimed_a",
            "2026-07-21T09:59:58.000Z",
            "session-a",
            "friction",
        ),
        fixture_use_event(
            "claimed",
            &hash,
            "evt_claimed_b",
            "2026-07-21T09:59:59.000Z",
            "session-b",
            "friction",
        ),
        fixture_use_event(
            "claimed",
            &hash,
            "evt_claimed_c",
            "2026-07-21T10:00:00.000Z",
            "session-c",
            "friction",
        ),
    ];
    events.push(json!({
        "schema_version": 1,
        "event_id": "evt_review_started",
        "event_type": "review_started",
        "recorded_at": "2026-07-21T11:00:00.000Z",
        "operator_workflow": "skill-evolution",
        "target": {
            "name": "claimed",
            "repo_relative_path": ".claude/skills/claimed",
            "content_hash": hash,
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": "rev_active_fixture",
            "target_hash": hash,
            "trigger_event_ids": ["evt_claimed_a", "evt_claimed_b", "evt_claimed_c"],
            "authorizing_rule": "friction_recurrence:execution",
            "risk_tier": "provisional",
            "session_or_cooldown_proof": { "type": "different_session" }
        }
    }));
    write_events(root.path(), "claimed", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("Active review `rev_active_fixture` already owns the target"));
    assert!(report.contains("Owner workflow: `skill-evolution`; risk tier `provisional`"));
    assert!(report.contains("reports/skill-evidence/claimed/reviews"));
    assert!(!report.contains("$skill-evolution"));
}

#[test]
fn skill_evolution_status_defers_queued_pre_close_evidence() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "queued-after-review"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let hash = skill_hash(root.path(), ".claude/skills/queued-after-review");
    let queued = fixture_use_event(
        "queued-after-review",
        &hash,
        "evt_queued",
        "2026-07-21T11:59:40.000Z",
        "queued-session",
        "friction",
    );
    let trigger_a = fixture_use_event(
        "queued-after-review",
        &hash,
        "evt_review_a",
        "2026-07-21T11:59:41.000Z",
        "trigger-a",
        "material_failure",
    );
    let trigger_b = fixture_use_event(
        "queued-after-review",
        &hash,
        "evt_review_b",
        "2026-07-21T11:59:42.000Z",
        "trigger-b",
        "material_failure",
    );
    let started = json!({
        "schema_version": 1,
        "event_id": "evt_review_started_deferred",
        "event_type": "review_started",
        "recorded_at": "2026-07-21T11:59:51.000Z",
        "operator_workflow": "skill-evolution",
        "target": {
            "name": "queued-after-review",
            "repo_relative_path": ".claude/skills/queued-after-review",
            "content_hash": hash,
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": "rev_deferred",
            "target_hash": hash,
            "trigger_event_ids": ["evt_review_a", "evt_review_b"],
            "authorizing_rule": "material_recurrence:execution",
            "risk_tier": "provisional",
            "session_or_cooldown_proof": { "type": "different_session" }
        }
    });
    let disposition = json!({
        "schema_version": 1,
        "event_id": "evt_review_disposition_deferred",
        "event_type": "review_disposition",
        "recorded_at": "2026-07-21T11:59:52.000Z",
        "operator_workflow": "skill-evolution",
        "target": {
            "name": "queued-after-review",
            "repo_relative_path": ".claude/skills/queued-after-review",
            "content_hash": hash,
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": "rev_deferred",
            "disposition": "candidate_rejected_validation",
            "adjudicated_event_ids": ["evt_review_a", "evt_review_b"]
        }
    });
    let mut events = vec![queued, trigger_a, trigger_b];
    for index in 0..7 {
        events.push(fixture_use_event(
            "queued-after-review",
            &hash,
            &format!("evt_clean_{index}"),
            "2026-07-21T11:59:43.000Z",
            &format!("clean-session-{index}"),
            "clean",
        ));
    }
    events.extend([started, disposition]);
    write_events(root.path(), "queued-after-review", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(
        report.contains("## Deferred after review"),
        "report={report}"
    );
    assert!(report.contains("queued pre-close evidence only"));
    assert!(!report.contains("$skill-evolution"));
}

/// Routing evidence out of the gate is the honest exit; letting the census fall silent
/// about it is not. Before this, a target whose only evidence a blocked close covered
/// simply stopped being counted — the census reported it under "omitted as not eligible"
/// alongside skills that never recorded an incident at all.
#[test]
fn skill_evolution_status_reports_evidence_retired_as_untestable() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "blocked-after-review"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/blocked-after-review");
    let triggers = ["evt_trigger_a", "evt_trigger_b", "evt_trigger_c"];
    let mut events = triggers
        .iter()
        .enumerate()
        .map(|(index, event_id)| {
            fixture_use_event(
                "blocked-after-review",
                &hash,
                event_id,
                &format!("2026-07-21T11:59:4{index}.000Z"),
                &format!("trigger-session-{index}"),
                "friction",
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture_blocked_close(
        "blocked-after-review",
        &hash,
        "rev_blocked",
        "friction_recurrence:execution",
        &triggers,
    ));
    write_events(root.path(), "blocked-after-review", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(
        report.contains("## Retired as untestable"),
        "report={report}"
    );
    assert!(
        report.contains(".claude/skills/blocked-after-review"),
        "report={report}"
    );
    assert!(
        report.contains("3 open incident"),
        "the census must name how much evidence left the gate: report={report}"
    );
    assert!(
        report.contains("retired as untestable: 1;"),
        "the summary counts must account for this store, not drop it: report={report}"
    );
    assert!(!report.contains("$skill-evolution"));
}

/// A review claimed on one cluster closes having reached no conclusion; a second
/// cluster it never covered is still deferred behind it. Reporting that as "queued
/// pre-close evidence only" tells the operator the evidence was accounted for by a
/// review that accounted for nothing.
#[test]
fn skill_evolution_status_distinguishes_evidence_queued_behind_an_instrument_limited_close() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "inconclusive-after-review"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/inconclusive-after-review");
    let covered = ["evt_covered_a", "evt_covered_b", "evt_covered_c"];
    let mut events = covered
        .iter()
        .enumerate()
        .map(|(index, event_id)| {
            fixture_use_event(
                "inconclusive-after-review",
                &hash,
                event_id,
                &format!("2026-07-21T11:59:4{index}.000Z"),
                &format!("covered-session-{index}"),
                "friction",
            )
        })
        .collect::<Vec<_>>();
    events.extend((0..3).map(|index| {
        let mut event = fixture_use_event(
            "inconclusive-after-review",
            &hash,
            &format!("evt_uncovered_{index}"),
            &format!("2026-07-21T11:59:5{index}.000Z"),
            &format!("uncovered-session-{index}"),
            "friction",
        );
        event["payload"]["symptom_key"] = json!("output");
        event
    }));
    events.extend(fixture_blocked_close(
        "inconclusive-after-review",
        &hash,
        "rev_inconclusive",
        "friction_recurrence:execution",
        &covered,
    ));
    write_events(root.path(), "inconclusive-after-review", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(
        report.contains("## Deferred after review"),
        "report={report}"
    );
    assert!(
        report.contains("whose instrument could not test what it covered"),
        "the operator must not read an inconclusive close as an accounting: report={report}"
    );
    assert!(
        !report.contains("queued pre-close evidence only"),
        "report={report}"
    );
    assert!(
        report.contains(
            "- Retired as untestable: 3 open incidents. An earlier close could not \
             decide that evidence, and removed it from this gate."
        ),
        "the deferred entry must also account for what the same close retired: report={report}"
    );
    assert!(!report.contains("$skill-evolution"));
}

/// A target can be actionable on new evidence and still carry evidence an earlier
/// blocked close retired. The retired half is easy to lose here, because the entry is
/// filed under "Ready to evolve" and nothing else on the page would mention it — and a
/// reviewer who does not know a cluster was already ruled untestable may go looking for
/// it.
#[test]
fn skill_evolution_status_reports_retired_evidence_on_a_target_that_is_also_ready() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "ready-with-retired"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/ready-with-retired");
    let retired = ["evt_retired_a", "evt_retired_b", "evt_retired_c"];
    let mut events = retired
        .iter()
        .enumerate()
        .map(|(index, event_id)| {
            fixture_use_event(
                "ready-with-retired",
                &hash,
                event_id,
                &format!("2026-07-21T11:59:4{index}.000Z"),
                &format!("retired-session-{index}"),
                "friction",
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture_blocked_close(
        "ready-with-retired",
        &hash,
        "rev_retired",
        "friction_recurrence:execution",
        &retired,
    ));
    // Recorded after the close, so this cluster clears the watermark and authorizes on
    // its own — the retired cluster contributes nothing to it.
    events.extend((0..3).map(|index| {
        let mut event = fixture_use_event(
            "ready-with-retired",
            &hash,
            &format!("evt_fresh_{index}"),
            &format!("2026-07-21T11:59:5{}.000Z", index + 2),
            &format!("fresh-incident-session-{index}"),
            "friction",
        );
        event["payload"]["symptom_key"] = json!("output");
        event
    }));
    write_events(root.path(), "ready-with-retired", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("## Ready to evolve"), "report={report}");
    assert!(
        report.contains(
            "- Retired as untestable: 3 open incidents. An earlier close could not \
             decide that evidence, and removed it from this gate."
        ),
        "a ready target must still declare what an earlier close retired: report={report}"
    );
}

/// Evidence also retires as untestable when an adjudicating close names the coverage its
/// instrument could not test. The projection records one flat retired set and never which
/// close produced which member, so no report may name a disposition it cannot see.
#[test]
fn skill_evolution_status_does_not_attribute_a_retirement_to_an_unrecorded_disposition() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "mixed-close"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/mixed-close");
    let mut events = vec![
        fixture_use_event(
            "mixed-close",
            &hash,
            "evt_tested",
            "2026-07-21T11:59:40.000Z",
            "session-a",
            "friction",
        ),
        fixture_use_event(
            "mixed-close",
            &hash,
            "evt_untestable",
            "2026-07-21T11:59:45.000Z",
            "session-b",
            "friction",
        ),
    ];
    let mut close = fixture_blocked_close(
        "mixed-close",
        &hash,
        "rev_mixed",
        "friction_recurrence:execution",
        &["evt_tested", "evt_untestable"],
    );
    close[1]["payload"]["disposition"] = json!("monitor_for_recurrence");
    close[1]["payload"]["instrument_limited_event_ids"] = json!(["evt_untestable"]);
    events.extend(close);
    write_events(root.path(), "mixed-close", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");

    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(
        report.contains("## Retired as untestable"),
        "report={report}"
    );
    assert!(
        !report.contains("blocked_no_valid_test"),
        "this retirement came from an adjudicating close, so naming that disposition is a claim the projection cannot support: report={report}"
    );
}

/// The ten-use threshold fires on a single open contemporaneous incident, so a blocked
/// close can retire exactly one — and every count in this report reaches an English
/// sentence. Guards the singular that the plural-only assertions elsewhere cannot see.
#[test]
fn skill_evolution_status_renders_a_single_retired_incident_grammatically() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "one-retired"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/one-retired");
    let mut events = vec![fixture_use_event(
        "one-retired",
        &hash,
        "evt_only",
        "2026-07-21T11:59:40.000Z",
        "only-session",
        "friction",
    )];
    events.extend(fixture_blocked_close(
        "one-retired",
        &hash,
        "rev_one",
        "ten_use_unresolved",
        &["evt_only"],
    ));
    write_events(root.path(), "one-retired", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(
        report.contains(
            "- Retired as untestable: 1 open incident, covered by a review that could \
             not decide it. That evidence no longer drives this gate, and remains in \
             the event stream."
        ),
        "report={report}"
    );
    assert!(!report.contains("1 open incidents"), "report={report}");
}

/// A target in the retired bucket can still be accumulating. Reporting only what left
/// the gate, with "another review would meet the same instrument", reads as a closed book
/// — when the target may be one incident away from authorizing a review on evidence that
/// close never touched.
#[test]
fn skill_evolution_status_reports_evidence_still_collecting_beside_retired_evidence() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "retired-and-collecting"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/retired-and-collecting");
    let retired = ["evt_retired_a", "evt_retired_b", "evt_retired_c"];
    let mut events = retired
        .iter()
        .enumerate()
        .map(|(index, event_id)| {
            fixture_use_event(
                "retired-and-collecting",
                &hash,
                event_id,
                &format!("2026-07-21T11:59:4{index}.000Z"),
                &format!("retired-session-{index}"),
                "friction",
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture_blocked_close(
        "retired-and-collecting",
        &hash,
        "rev_retired",
        "friction_recurrence:execution",
        &retired,
    ));
    // Two on another symptom: real, open, and one short of a threshold.
    events.extend((0..2).map(|index| {
        let mut event = fixture_use_event(
            "retired-and-collecting",
            &hash,
            &format!("evt_live_{index}"),
            &format!("2026-07-21T11:59:5{}.000Z", index + 2),
            &format!("live-session-{index}"),
            "friction",
        );
        event["payload"]["symptom_key"] = json!("output");
        event
    }));
    write_events(root.path(), "retired-and-collecting", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(
        report.contains("## Retired as untestable"),
        "report={report}"
    );
    assert!(
        report.contains(
            "- Still collecting: 2 open incidents the close did not cover, short of a threshold."
        ),
        "the entry must not read as a closed book: report={report}"
    );
}

#[test]
fn skill_evolution_status_reopens_for_a_post_review_incident() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "reopened-after-review"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/reopened-after-review");
    let mut older = fixture_use_event(
        "reopened-after-review",
        &hash,
        "evt_older_open",
        "2026-07-21T11:59:40.000Z",
        "older-session",
        "friction",
    );
    older["payload"]["symptom_key"] = json!("output");
    let mut trigger_a = fixture_use_event(
        "reopened-after-review",
        &hash,
        "evt_review_a",
        "2026-07-21T11:59:41.000Z",
        "trigger-a",
        "friction",
    );
    trigger_a["payload"]["symptom_key"] = json!("tool-compatibility");
    let mut trigger_b = fixture_use_event(
        "reopened-after-review",
        &hash,
        "evt_review_b",
        "2026-07-21T11:59:42.000Z",
        "trigger-b",
        "friction",
    );
    trigger_b["payload"]["symptom_key"] = json!("tool-compatibility");
    let mut trigger_c = fixture_use_event(
        "reopened-after-review",
        &hash,
        "evt_review_c",
        "2026-07-21T11:59:43.000Z",
        "trigger-c",
        "friction",
    );
    trigger_c["payload"]["symptom_key"] = json!("tool-compatibility");
    let mut events = vec![older, trigger_a, trigger_b, trigger_c];
    for index in 0..5 {
        events.push(fixture_use_event(
            "reopened-after-review",
            &hash,
            &format!("evt_clean_{index}"),
            "2026-07-21T11:59:44.000Z",
            &format!("clean-session-{index}"),
            "clean",
        ));
    }
    events.push(fixture_review_started(ReviewStartedFixture {
        target_name: "reopened-after-review",
        target_hash: &hash,
        event_id: "evt_review_started",
        event_type: "review_started",
        recorded_at: "2026-07-21T11:59:50.000Z",
        review_id: "rev_reopened",
        trigger_event_ids: &["evt_review_a", "evt_review_b", "evt_review_c"],
        authorizing_rule: "friction_recurrence:tool-compatibility",
    }));
    events.push(fixture_review_disposition(
        "reopened-after-review",
        &hash,
        "evt_review_disposition",
        "2026-07-21T11:59:51.000Z",
        "rev_reopened",
        &["evt_review_a", "evt_review_b", "evt_review_c"],
    ));
    events.push(fixture_use_event(
        "reopened-after-review",
        &hash,
        "evt_post_review",
        "2026-07-21T11:59:52.000Z",
        "post-review-session",
        "friction",
    ));
    write_events(root.path(), "reopened-after-review", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("## Ready to evolve"), "report={report}");
    assert!(report.contains("new incident was recorded after the last completed same-hash review"));
    assert!(report.contains("$skill-evolution \".claude/skills/reopened-after-review\""));
}

#[test]
fn skill_evolution_status_renders_quarantined_eligibility() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "quarantined"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/quarantined");
    write_events(
        root.path(),
        "quarantined",
        &[fixture_use_event(
            "quarantined",
            &hash,
            "evt_severe",
            "2026-07-21T11:59:52.000Z",
            "threshold-session",
            "severe_incident",
        )],
    );

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("### .claude/skills/quarantined — QUARANTINED"));
    assert!(report.contains("Quarantine: Stop using this target"));
    assert!(report.contains("$skill-evolution \".claude/skills/quarantined\""));
}

#[test]
fn skill_evolution_status_warns_when_the_census_recorded_the_threshold() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "same-session"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/same-session");
    let events = [
        fixture_use_event(
            "same-session",
            &hash,
            "evt_same_a",
            "2026-07-21T11:59:50.000Z",
            "session-a",
            "friction",
        ),
        fixture_use_event(
            "same-session",
            &hash,
            "evt_same_b",
            "2026-07-21T11:59:51.000Z",
            "session-b",
            "friction",
        ),
        fixture_use_event(
            "same-session",
            &hash,
            "evt_same_c",
            "2026-07-21T11:59:52.000Z",
            "threshold-session",
            "friction",
        ),
    ];
    write_events(root.path(), "same-session", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "threshold-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("## Ready to evolve"));
    assert!(report.contains("This census session recorded the threshold (`threshold-session`)"));
    assert!(report.contains("refused_cooldown_or_same_session"));
}

#[test]
fn skill_evolution_status_omits_an_ineligible_decontamination_claim() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "legacy-claimed-only"] {
        create_fixture_skill(root.path(), name);
    }
    let hash = skill_hash(root.path(), ".claude/skills/legacy-claimed-only");
    write_events(
        root.path(),
        "legacy-claimed-only",
        &[fixture_review_started(ReviewStartedFixture {
            target_name: "legacy-claimed-only",
            target_hash: &hash,
            event_id: "evt_decontamination_started",
            event_type: "decontamination_started",
            recorded_at: "2026-07-21T11:59:52.000Z",
            review_id: "rev_decontamination",
            trigger_event_ids: &[],
            authorizing_rule: "legacy_decontamination",
        })],
    );

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("omitted as not eligible: 1"));
    assert!(!report.contains("## Eligible but blocked"));
    assert!(!report.contains("$skill-evolution"));
}

#[test]
fn skill_evolution_status_routes_self_target_eligibility_to_an_independent_writer() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target = root.path().join(".claude/skills/skill-evolution");
    fs::create_dir_all(&target).expect("create evolution skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: skill-evolution\n---\n# Skill Evolution\n",
    )
    .expect("write evolution skill");
    let hash = skill_hash(root.path(), ".claude/skills/skill-evolution");
    let events = [
        fixture_use_event(
            "skill-evolution",
            &hash,
            "evt_self_a",
            "2026-07-21T10:59:58.000Z",
            "session-a",
            "friction",
        ),
        fixture_use_event(
            "skill-evolution",
            &hash,
            "evt_self_b",
            "2026-07-21T10:59:59.000Z",
            "session-b",
            "friction",
        ),
        fixture_use_event(
            "skill-evolution",
            &hash,
            "evt_self_c",
            "2026-07-21T11:00:00.000Z",
            "session-c",
            "friction",
        ),
    ];
    write_events(root.path(), "skill-evolution", &events);

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("Skill Evolution cannot target itself"));
    assert!(report.contains("independent skill-authoring workflow"));
    assert!(!report.contains("$skill-evolution \".claude/skills/skill-evolution\""));
}

#[test]
fn skill_evolution_status_surfaces_corrupt_streams_and_missing_targets() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "corrupt"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let corrupt_hash = skill_hash(root.path(), ".claude/skills/corrupt");
    write_events(
        root.path(),
        "corrupt",
        &[fixture_use_event(
            "corrupt",
            &corrupt_hash,
            "evt_corrupt",
            "2026-07-21T11:00:00.000Z",
            "session-a",
            "clean",
        )],
    );
    let corrupt_stream = root
        .path()
        .join("reports/skill-evidence/corrupt/events.jsonl");
    let mut corrupt_bytes = fs::read_to_string(&corrupt_stream).expect("read corrupt stream");
    corrupt_bytes.push_str("not-json\n");
    fs::write(&corrupt_stream, corrupt_bytes).expect("corrupt event stream");
    write_events(
        root.path(),
        "missing",
        &[fixture_use_event(
            "missing",
            "missing-target-hash",
            "evt_missing",
            "2026-07-21T11:00:00.000Z",
            "session-a",
            "clean",
        )],
    );

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784635200000",
            "--session-id",
            "unavailable",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("## Could not determine"));
    assert!(report.contains("Status: `integrity`"));
    assert!(report.contains("line 2: not valid JSON"));
    assert!(report.contains("Status: `missing_target`"));
    assert!(report.contains(".claude/skills/missing"));
}

#[test]
fn skill_evolution_status_isolates_a_store_scan_failure() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "healthy"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let failed_store = root.path().join("reports/skill-evidence/broken");
    fs::create_dir_all(failed_store.join("events.jsonl"))
        .expect("create unreadable-as-file event path");
    let healthy_store = root.path().join("reports/skill-evidence/healthy");
    fs::create_dir_all(&healthy_store).expect("create healthy store");
    fs::write(healthy_store.join("events.jsonl"), b"").expect("write empty event stream");
    fs::write(
        healthy_store.join("gate-status.json"),
        "{\n  \"target_repo_relative_path\": \".claude/skills/healthy\"\n}\n",
    )
    .expect("write healthy identity projection");

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784980800000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("Scanned 2 evidence stores read-only"));
    assert!(report.contains("Status: `scan_failure`"));
    assert!(report.contains("indeterminate: 1; omitted as not eligible: 1"));
}

#[test]
fn method_gap_research_status_inventories_only_the_selected_canonical_family() {
    let root = parity_fixture();
    let governed_paths = ["game-alpha", "game-beta"]
        .into_iter()
        .flat_map(|store| {
            ["events.jsonl", "gate-status.json"].map(|name| {
                root.path()
                    .join("reports/skill-evidence")
                    .join(store)
                    .join(name)
            })
        })
        .collect::<Vec<_>>();
    let before = snapshot_files(&governed_paths);
    let expected = fs::read(repository_root().join(
        "fixtures/skill-evidence/status-reporters-v1/method-gap-research-status.expected.json",
    ))
    .expect("read Method-Gap JavaScript golden");

    let output = skill_evidence()
        .args(["skills", "method-gap-research-status", "game-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", "1784980800000"])
        .output()
        .expect("run compiled Method-Gap Research Status");

    assert!(output.status.success());
    assert_eq!(output.stdout, expected);
    assert_files_unchanged(&governed_paths, &before);
    assert!(
        Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(root.path())
            .output()
            .expect("inspect parity worktree")
            .stdout
            .is_empty()
    );
}

#[test]
fn method_gap_research_status_leaves_the_git_index_unchanged() {
    let root = parity_fixture();
    let skill_path = root.path().join(".claude/skills/game-alpha/SKILL.md");
    let unchanged_skill = fs::read(&skill_path).expect("read tracked skill bytes");
    fs::write(&skill_path, unchanged_skill).expect("refresh tracked skill stat metadata");
    let index_path = root.path().join(".git/index");
    let before = snapshot_files(std::slice::from_ref(&index_path));

    let output = skill_evidence()
        .args(["skills", "method-gap-research-status", "game-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", "1784980800000"])
        .output()
        .expect("run compiled Method-Gap Research Status");

    assert!(output.status.success());
    assert_files_unchanged(std::slice::from_ref(&index_path), &before);
}

#[test]
fn skill_evolution_status_marks_a_session_threshold_ready_in_another_session() {
    let root = parity_fixture();
    let governed_paths = ["game-alpha", "game-beta"]
        .into_iter()
        .flat_map(|store| {
            ["events.jsonl", "gate-status.json"].map(|name| {
                root.path()
                    .join("reports/skill-evidence")
                    .join(store)
                    .join(name)
            })
        })
        .collect::<Vec<_>>();
    let before = snapshot_files(&governed_paths);
    let expected = fs::read(
        repository_root()
            .join("fixtures/skill-evidence/status-reporters-v1/skill-evolution-status.expected.md"),
    )
    .expect("read Skill Evolution JavaScript golden");

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784980800000",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run compiled Skill Evolution Status");

    assert!(output.status.success());
    assert_eq!(output.stdout, expected);
    assert_files_unchanged(&governed_paths, &before);
    assert!(
        Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(root.path())
            .output()
            .expect("inspect parity worktree")
            .stdout
            .is_empty()
    );
}

#[test]
fn skill_evolution_status_uses_a_stored_projection_only_for_target_identity() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "projection-only"] {
        let skill = root.path().join(".claude/skills").join(name);
        fs::create_dir_all(&skill).expect("create fixture skill");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\n---\n# {name}\n"),
        )
        .expect("write fixture skill");
    }
    let store = root.path().join("reports/skill-evidence/projection-only");
    fs::create_dir_all(&store).expect("create projection-only store");
    fs::write(store.join("events.jsonl"), b"").expect("write empty event stream");
    fs::write(
        store.join("gate-status.json"),
        "{\n  \"schema_version\": 1,\n  \"target_name\": \"projection-only\",\n  \"target_repo_relative_path\": \".claude/skills/projection-only\",\n  \"state\": \"deliberately_stale_fixture\"\n}\n",
    )
    .expect("write stale identity projection");

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            "1784980800000",
            "--session-id",
            "unavailable",
        ])
        .output()
        .expect("run Skill Evolution Status");
    assert!(output.status.success());
    let report = String::from_utf8(output.stdout).expect("UTF-8 status report");
    assert!(report.contains("indeterminate: 0; omitted as not eligible: 1"));
    assert!(!report.contains("## Could not determine"));
}

#[test]
fn skill_evolution_status_keeps_a_projection_only_directory_unidentified_after_derive_refuses() {
    let root = tempfile::tempdir().expect("temporary repository");
    for name in ["skill-evolution", "projection-only"] {
        create_fixture_skill(root.path(), name);
    }
    let store = root.path().join("reports/skill-evidence/projection-only");
    fs::create_dir_all(&store).expect("create projection-only directory");
    let projection_path = store.join("gate-status.json");
    fs::write(
        &projection_path,
        serde_json::to_vec_pretty(&json!({
            "schema_version": 1,
            "generated_at": "2026-07-25T12:00:00Z",
            "target_content_hash": "stray-projection-hash",
            "qualifying_uses_on_current_hash": 0,
            "open_incident_ids": [],
            "candidate_clusters": [],
            "state": "closed",
            "authorized_workflow": null,
            "authorization_reason": null,
            "trigger_event_ids": [],
            "threshold_session_id": null,
            "not_before": null,
            "active_review_id": null,
            "last_completed_review_id": null,
            "review_reentry_basis": null,
            "target_name": "projection-only",
            "target_repo_relative_path": ".claude/skills/projection-only",
            "derivation_session_id": null
        }))
        .expect("serialize valid stray projection"),
    )
    .expect("write valid stray projection");
    let research_path = store.join("research-artifact.md");
    fs::write(&research_path, "# Not an evidence stream\n").expect("write other store content");
    let governed_paths = [projection_path, research_path];
    let before_files = snapshot_files(&governed_paths);
    let store_entries = || {
        let mut names = fs::read_dir(&store)
            .expect("read projection-only directory")
            .map(|entry| entry.expect("read directory entry").file_name())
            .collect::<Vec<_>>();
        names.sort();
        names
    };
    let before_entries = store_entries();
    let status = || {
        skill_evidence()
            .args(["skills", "evolution-status", "--root"])
            .arg(root.path())
            .args([
                "--now-epoch-milliseconds",
                "1784980800000",
                "--session-id",
                "unavailable",
            ])
            .output()
            .expect("run Skill Evolution Status")
    };

    let before = status();
    assert!(before.status.success());
    assert!(before.stderr.is_empty());
    let report = String::from_utf8_lossy(&before.stdout);
    assert!(report.contains("indeterminate: 1; omitted as not eligible: 0"));
    assert!(report.contains(
        "### evidence store projection-only\n\n- Status: `unidentified_store`.\n- events.jsonl does not exist"
    ));

    let derive = skill_evidence()
        .args(["skills", "evidence", "derive", "--root"])
        .arg(root.path())
        .args([
            "--target",
            ".claude/skills/projection-only",
            "--session-id",
            "derive-session",
        ])
        .output()
        .expect("attempt to derive without an event stream");
    assert_eq!(derive.status.code(), Some(3));
    assert!(derive.stdout.is_empty());
    assert_eq!(
        String::from_utf8(derive.stderr).expect("UTF-8 refusal diagnostic"),
        format!(
            "Cannot derive a gate projection because the event stream does not exist: {}. Nothing modified.\n",
            store.join("events.jsonl").display()
        )
    );
    assert_files_unchanged(&governed_paths, &before_files);
    assert_eq!(store_entries(), before_entries);

    let after = status();
    assert!(after.status.success());
    assert!(after.stderr.is_empty());
    assert_eq!(after.stdout, before.stdout);
}

#[test]
fn method_gap_research_status_roots_targets_in_the_selected_repository() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target = root.path().join(".claude/skills/skill-evolution");
    fs::create_dir_all(&target).expect("create colliding fixture skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: skill-evolution\n---\n# Fixture collision\n",
    )
    .expect("write colliding fixture skill");
    let expected_hash = skill_evidence()
        .args(["skills", "evidence", "hash", "--root"])
        .arg(root.path())
        .arg("--target")
        .arg(&target)
        .output()
        .expect("hash absolute fixture target");
    assert!(expected_hash.status.success());
    let expected_hash = serde_json::from_slice::<Value>(&expected_hash.stdout).expect("hash JSON")
        ["content_hash"]
        .as_str()
        .expect("fixture hash")
        .to_owned();

    let output = skill_evidence()
        .current_dir(repository_root())
        .args(["skills", "method-gap-research-status", "skill-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", "1784980800000"])
        .output()
        .expect("run rooted Method-Gap Research Status");
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).expect("inventory JSON");
    assert_eq!(
        report["targets"][0]["target_path"],
        ".claude/skills/skill-evolution"
    );
    assert_eq!(report["targets"][0]["target_content_hash"], expected_hash);
    assert_eq!(report["targets"][0]["target_file_count"], 1);
}

#[test]
fn method_gap_research_status_refuses_an_out_of_range_explicit_clock_without_panicking() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target = root.path().join(".claude/skills/game-clock");
    fs::create_dir_all(&target).expect("create fixture skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: game-clock\n---\n# Clock fixture\n",
    )
    .expect("write fixture skill");

    let output = skill_evidence()
        .args(["skills", "method-gap-research-status", "game-*", "--root"])
        .arg(root.path())
        .args(["--now-epoch-milliseconds", &i64::MAX.to_string()])
        .output()
        .expect("run Method-Gap Research Status");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Clock epoch milliseconds are outside the supported range"));
    assert!(!stderr.contains("panicked"));
}

#[test]
fn skill_evolution_status_refuses_an_out_of_range_explicit_clock_without_panicking() {
    let root = tempfile::tempdir().expect("temporary repository");

    let output = skill_evidence()
        .args(["skills", "evolution-status", "--root"])
        .arg(root.path())
        .args([
            "--now-epoch-milliseconds",
            &i64::MAX.to_string(),
            "--session-id",
            "unavailable",
        ])
        .output()
        .expect("run Skill Evolution Status");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Clock epoch milliseconds are outside the supported range"));
    assert!(!stderr.contains("panicked"));
}
