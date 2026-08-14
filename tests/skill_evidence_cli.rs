#![forbid(unsafe_code)]

mod support;

use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use serde_json::{Value, json};
use support::{repository_root, skill_evidence};
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

fn clean_record(root: &Path, task_label: &str, session_id: &str) -> Command {
    let mut command = skill_evidence();
    command
        .args(["skills", "evidence", "record", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "clean",
            "--task-label",
            task_label,
            "--session-id",
            session_id,
        ]);
    command
}

fn incident_record(root: &Path, task_label: &str, session_id: &str, observed: &str) -> Command {
    let mut command = skill_evidence();
    command
        .args(["skills", "evidence", "record", "--root"])
        .arg(root)
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "material_failure",
            "--task-label",
            task_label,
            "--session-id",
            session_id,
            "--symptom-key",
            "output",
            "--expected",
            "expected",
            "--observed",
            observed,
            "--consequence",
            "consequence",
            "--run-condition",
            "condition",
        ]);
    command
}

fn recorded_events(root: &Path) -> Vec<Value> {
    fs::read_to_string(root.join("reports/skill-evidence/demo-skill/events.jsonl"))
        .expect("read event stream")
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("recorded event JSON"))
        .collect()
}

/// The demo skill's content hash, which every seeded event must carry to sit on the current
/// target hash rather than a superseded one.
const DEMO_SKILL_HASH: &str = "8fd064beeb351b7698023277ac023aa2dfd2cca632a069018a6e1c739316e5bf";

fn seeded_target() -> Value {
    json!({
        "name": "demo-skill",
        "repo_relative_path": ".claude/skills/demo-skill",
        "content_hash": DEMO_SKILL_HASH,
        "repo_head": "fixture-head"
    })
}

/// One seeded `use_recorded` event.
///
/// The task label and fingerprint are the run group, so seeding two events under one group
/// gives them the sibling shape a further-incident record writes.
///
/// `run_condition` is a parameter rather than a constant because `event.v1` leaves it
/// optional: a stream omitting the key entirely is a shape this crate wrote before the key
/// existed, and one caller below seeds exactly that on purpose. Collapsing it would delete
/// that coverage silently.
fn seeded_use_event(
    event_id: &str,
    hour: usize,
    session_id: &str,
    run_group: &str,
    outcome: &str,
    symptom_key: &str,
    run_condition: Option<&str>,
) -> Value {
    let mut event = json!({
        "schema_version": 1,
        "event_id": event_id,
        "event_type": "use_recorded",
        "recorded_at": format!("2026-01-02T{hour:02}:00:00Z"),
        "operator_workflow": "skill-evidence-capture",
        "target": seeded_target(),
        "top_level_session_id": session_id,
        "payload": {
            "qualifying_use": true,
            "retrospective": false,
            "task_label": run_group,
            "task_fingerprint": run_group,
            "outcome": outcome,
            "symptom_key": symptom_key,
            "expected": "expected",
            "observed": "observed",
            "consequence": "consequence",
            "workaround_taken": Value::Null,
            "evidence_refs": [],
            "same_run_group": run_group
        }
    });
    if let Some(run_condition) = run_condition {
        event["payload"]["run_condition"] = json!(run_condition);
    }
    event
}

/// The claim and close a completed review leaves behind, an hour apart.
fn seeded_review(
    review_id: &str,
    disposition: &str,
    adjudicated: &[&str],
    hour: usize,
) -> [Value; 2] {
    [
        json!({
            "schema_version": 1,
            "event_id": format!("evt_review_started_{review_id}"),
            "event_type": "review_started",
            "recorded_at": format!("2026-01-02T{hour:02}:00:00Z"),
            "operator_workflow": "skill-evolution",
            "target": seeded_target(),
            "top_level_session_id": "review-session",
            "payload": {"review_id": review_id}
        }),
        json!({
            "schema_version": 1,
            "event_id": format!("evt_review_disposition_{review_id}"),
            "event_type": "review_disposition",
            "recorded_at": format!("2026-01-02T{:02}:00:00Z", hour + 1),
            "operator_workflow": "skill-evolution",
            "target": seeded_target(),
            "top_level_session_id": "review-session",
            "payload": {
                "review_id": review_id,
                "disposition": disposition,
                "adjudicated_event_ids": adjudicated
            }
        }),
    ]
}

fn seed_event_stream(root: &Path, events: &[Value]) {
    let evidence_directory = root.join("reports/skill-evidence/demo-skill");
    fs::create_dir_all(&evidence_directory).expect("create evidence directory");
    fs::write(
        evidence_directory.join("events.jsonl"),
        events
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n",
    )
    .expect("write seeded event stream");
}

/// One completed run can deviate in several distinct ways, and compressing them into a
/// single record costs a later review the ability to name exactly the deviation its
/// instrument could not test — the whole event retires instead, testable siblings
/// included. A declared sibling is an ordinary record sharing the run's group, so the run
/// still counts once while each deviation stays separately addressable.
#[test]
fn skill_evidence_further_incident_records_a_sibling_of_the_same_run() {
    let fixture = repository_with_demo_skill();
    let first = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "four briefs cited file paths",
    )
    .output()
    .expect("record the run's first incident");
    assert!(
        first.status.success(),
        "first incident failed: stdout={} stderr={}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );

    let undeclared = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "the binding coupling was buried mid-comment",
    )
    .output()
    .expect("record an undeclared second incident");
    assert_eq!(undeclared.status.code(), Some(3));
    let refusal = String::from_utf8_lossy(&undeclared.stderr);
    assert!(refusal.contains("Duplicate receipt refused"), "{refusal}");
    assert!(
        refusal.contains("a genuinely distinct use needs a distinct --task-label"),
        "an undeclared duplicate must still meet the unchanged refusal: {refusal}"
    );

    let sibling = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "the binding coupling was buried mid-comment",
    )
    .arg("--further-incident")
    .output()
    .expect("record the declared sibling");
    assert!(
        sibling.status.success(),
        "declared sibling failed: stdout={} stderr={}",
        String::from_utf8_lossy(&sibling.stdout),
        String::from_utf8_lossy(&sibling.stderr)
    );

    let events = recorded_events(fixture.path());
    assert_eq!(events.len(), 2);
    assert_ne!(events[0]["event_id"], events[1]["event_id"]);
    assert_eq!(
        events[0]["payload"]["same_run_group"], events[1]["payload"]["same_run_group"],
        "siblings record one run, so they share its group"
    );
    assert_eq!(
        events[0]["payload"]["task_fingerprint"],
        events[1]["payload"]["task_fingerprint"]
    );
    assert_eq!(
        events[1]["payload"]["observed"],
        "the binding coupling was buried mid-comment"
    );
}

/// The use denominator counts run groups, so the group is what a use is. A caller-supplied
/// group is an assertion; the derived one is computed from target, task label, and
/// top-level session. Letting the two flags combine would be the one path where an
/// asserted string both bypasses the duplicate refusal and decides the denominator — a
/// fixed group across a hash would hold it at one use forever. A further incident comes
/// from the same session by construction, so the derived group is already the right one
/// and no override is ever needed.
#[test]
fn skill_evidence_further_incident_refuses_a_caller_supplied_run_group() {
    let fixture = repository_with_demo_skill();
    let first = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "four briefs cited file paths",
    )
    .output()
    .expect("record the run's first incident");
    assert!(first.status.success());

    let combined = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "the binding coupling was buried mid-comment",
    )
    .args(["--same-run-group", "asserted-group"])
    .arg("--further-incident")
    .output()
    .expect("record with both flags");
    assert_eq!(combined.status.code(), Some(3));
    let refusal = String::from_utf8_lossy(&combined.stderr);
    assert!(
        refusal.contains("--further-incident") && refusal.contains("--same-run-group"),
        "the refusal must name both flags: {refusal}"
    );
    assert!(
        refusal.contains("Nothing recorded."),
        "a refusal writes nothing and says so: {refusal}"
    );
    assert_eq!(recorded_events(fixture.path()).len(), 1);
}

/// The flag declares a run this store already recorded. With nothing recorded for that
/// run it declares something untrue, and honouring it would turn a narrow opt-in into a
/// standing way to record under a group nobody can see — the first record of a run is an
/// ordinary use and must be recorded as one.
#[test]
fn skill_evidence_further_incident_refuses_a_run_with_no_recorded_sibling() {
    let fixture = repository_with_demo_skill();
    let orphan = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "the binding coupling was buried mid-comment",
    )
    .arg("--further-incident")
    .output()
    .expect("record a further incident of nothing");
    assert_eq!(orphan.status.code(), Some(3));
    let refusal = String::from_utf8_lossy(&orphan.stderr);
    assert!(
        refusal.contains("--further-incident") && refusal.contains("Nothing recorded."),
        "the refusal must name the flag and say nothing was written: {refusal}"
    );
    // The match is on this session and this task's label against the current hash. Explaining
    // it by run-group membership describes the resolution this command stopped using, and
    // "on the unchanged target" asserts something false whenever the run this operator means
    // was recorded before an edit.
    assert!(
        !refusal.contains("on the unchanged target"),
        "a sibling may sit on a superseded hash, so the target is not asserted unchanged: {refusal}"
    );
    assert!(
        refusal.contains("in this top-level session"),
        "the refusal must explain itself by the rule the code applies: {refusal}"
    );
    assert!(
        !fixture
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl")
            .exists(),
        "a refusal writes no event stream at all"
    );
}

/// A further incident shares its siblings' run group, and a run whose first record was
/// written with `--same-run-group` carries a group no derivation reproduces. Deriving one
/// anyway leaves that run unable to record a second deviation from the very session that
/// recorded it, with no exit that does not mint a second group for one run.
///
/// The group is still never taken from the caller here: it is read off the record already in
/// the store, which is what keeps the denominator on identity the store owns.
#[test]
fn skill_evidence_further_incident_joins_a_run_recorded_under_a_supplied_group() {
    let fixture = repository_with_demo_skill();
    let first = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "four briefs cited file paths",
    )
    .args(["--same-run-group", "carried-over-group"])
    .output()
    .expect("record the run under a supplied group");
    assert!(
        first.status.success(),
        "first record failed: stderr={}",
        String::from_utf8_lossy(&first.stderr)
    );

    let sibling = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "the binding coupling was buried mid-comment",
    )
    .arg("--further-incident")
    .output()
    .expect("record a further incident of that run");
    assert!(
        sibling.status.success(),
        "a run recorded under a supplied group must still take further incidents: stderr={}",
        String::from_utf8_lossy(&sibling.stderr)
    );

    let events = recorded_events(fixture.path());
    assert_eq!(events.len(), 2);
    assert_eq!(
        events[1]["payload"]["same_run_group"], "carried-over-group",
        "the sibling joins the group its run already carries"
    );
    assert_eq!(
        events[0]["payload"]["task_fingerprint"],
        events[1]["payload"]["task_fingerprint"]
    );
}

/// A run group is derived from the target, the task label and the top-level session, so a
/// further incident cannot reach a run whose records were written in an earlier session. That
/// limit is a consequence of keeping the denominator on computed identity, and it is
/// tolerable — but the refusal must not answer it with the advice meant for a typo. Recording
/// that deviation as a fresh use mints a second run group for one run and counts it twice,
/// which is exactly the inflation the run-group denominator exists to prevent.
#[test]
fn skill_evidence_further_incident_refuses_a_run_recorded_in_another_session() {
    let fixture = repository_with_demo_skill();
    assert!(
        incident_record(
            fixture.path(),
            "One triage run",
            "session-a",
            "four briefs cited file paths",
        )
        .output()
        .expect("record the run's first incident")
        .status
        .success()
    );

    let later_session = incident_record(
        fixture.path(),
        "One triage run",
        "session-b",
        "a deviation of that run, noticed in a later session",
    )
    .arg("--further-incident")
    .output()
    .expect("record a further incident from another session");
    assert_eq!(later_session.status.code(), Some(3));
    let refusal = String::from_utf8_lossy(&later_session.stderr);
    assert!(
        refusal.contains("session-a"),
        "the refusal must name the session that holds the run: {refusal}"
    );
    assert!(
        refusal.contains("would count one run twice"),
        "and must not advise recording it as a fresh use: {refusal}"
    );
    assert!(
        !refusal.contains("record it without the flag"),
        "that advice is for a run with no record at all, and here it inflates the denominator: {refusal}"
    );
    assert_eq!(recorded_events(fixture.path()).len(), 1);
}

/// A clean record asserts no skill-attributable friction was visible in that run. Recording
/// one as a sibling of an incident would put both claims about a single run on disk, and the
/// stream is append-only, so nothing could ever resolve them.
///
/// The asymmetry is deliberate and runs the other way: a run first recorded clean can still
/// gain an incident, because noticing a deviation later is an honest addition rather than a
/// contradiction.
#[test]
fn skill_evidence_further_incident_refuses_a_clean_outcome() {
    let fixture = repository_with_demo_skill();
    assert!(
        incident_record(
            fixture.path(),
            "One triage run",
            "session-a",
            "four briefs cited file paths",
        )
        .output()
        .expect("record the run's first incident")
        .status
        .success()
    );

    let clean_sibling = clean_record(fixture.path(), "One triage run", "session-a")
        .arg("--further-incident")
        .output()
        .expect("record a clean sibling");
    assert_eq!(clean_sibling.status.code(), Some(3));
    let refusal = String::from_utf8_lossy(&clean_sibling.stderr);
    assert!(
        refusal.contains("--further-incident") && refusal.contains("Nothing recorded."),
        "the refusal must name the flag and say nothing was written: {refusal}"
    );
    assert_eq!(recorded_events(fixture.path()).len(), 1);

    let other = repository_with_demo_skill();
    assert!(
        clean_record(other.path(), "One triage run", "session-a")
            .output()
            .expect("record a run as clean")
            .status
            .success()
    );
    let later_incident = incident_record(
        other.path(),
        "One triage run",
        "session-a",
        "a deviation noticed after the clean receipt",
    )
    .arg("--further-incident")
    .output()
    .expect("record an incident of a run first recorded clean");
    assert!(
        later_incident.status.success(),
        "a run recorded clean must still be able to gain an incident: stderr={}",
        String::from_utf8_lossy(&later_incident.stderr)
    );
    assert_eq!(recorded_events(other.path()).len(), 2);
}

/// The clause exists to explain a divergence between the two counts the reply prints, so it
/// must be keyed on what those counts actually read. Adjudicated siblings leave
/// `open_incident_ids` and stop inflating the open count — the divergence is gone, and a
/// reply still explaining it sends the operator looking for something that is no longer
/// there. An untestable-coverage sibling stays open and keeps the clause honestly.
#[test]
fn capture_reply_drops_the_sibling_clause_once_a_close_adjudicated_them() {
    let fixture = repository_with_demo_skill();
    let incident = |event_id: &str, hour: usize, session: &str, group: &str| {
        seeded_use_event(
            event_id,
            hour,
            session,
            group,
            "material_failure",
            "output",
            Some("condition"),
        )
    };
    let mut events = vec![
        // One run that deviated twice, and a second run that deviated once.
        incident("evt_sibling_one", 1, "session-a", "run-a"),
        incident("evt_sibling_two", 2, "session-a", "run-a"),
        incident("evt_other_run", 3, "session-b", "run-b"),
    ];
    events.extend(seeded_review(
        "rev_adjudicating",
        "closed_no_skill_defect",
        &["evt_sibling_one", "evt_sibling_two"],
        4,
    ));
    seed_event_stream(fixture.path(), &events);

    let output = clean_record(fixture.path(), "A later clean use", "session-c")
        .arg("--human")
        .output()
        .expect("record a use after the close");
    assert!(output.status.success());
    let reply = String::from_utf8(output.stdout).expect("human reply UTF-8");
    assert!(
        reply.contains("open incidents: 1"),
        "only the unadjudicated run stays open: reply={reply}"
    );
    assert!(
        reply.contains("qualifying uses on current target hash: 3"),
        "reply={reply}"
    );
    assert!(
        !reply.contains("counts once"),
        "the counts no longer diverge, so there is nothing to reconcile: reply={reply}"
    );
}

/// Every sibling is its own open incident, so the open-incident count now rises faster than
/// the use count — and the collecting reply prints both, side by side, to an operator who
/// has no other view of the store. Two incidents against one use reads as a miscount unless
/// the reply says which of the two it is. The clause appears only where the divergence does;
/// a store whose runs recorded one incident each reads exactly as it always has.
#[test]
fn capture_reply_reconciles_open_incidents_against_uses_when_a_run_recorded_several() {
    let fixture = repository_with_demo_skill();
    assert!(
        incident_record(
            fixture.path(),
            "One triage run",
            "session-a",
            "four briefs cited file paths",
        )
        .arg("--human")
        .output()
        .expect("record the run's first incident")
        .status
        .success()
    );

    let sibling = incident_record(
        fixture.path(),
        "One triage run",
        "session-a",
        "the binding coupling was buried mid-comment",
    )
    .args(["--further-incident", "--human"])
    .output()
    .expect("record the declared sibling");
    assert!(sibling.status.success());
    let reply = String::from_utf8(sibling.stdout).expect("human reply UTF-8");
    assert!(reply.contains("open incidents: 2"), "reply={reply}");
    assert!(
        reply.contains("qualifying uses on current target hash: 1"),
        "reply={reply}"
    );
    assert!(
        reply.contains("a run that recorded several incidents counts once"),
        "the reply must reconcile the two counts it prints: reply={reply}"
    );

    let ordinary = repository_with_demo_skill();
    let single = incident_record(
        ordinary.path(),
        "An ordinary run",
        "session-a",
        "one deviation",
    )
    .arg("--human")
    .output()
    .expect("record one ordinary incident");
    assert!(single.status.success());
    let ordinary_reply = String::from_utf8(single.stdout).expect("human reply UTF-8");
    assert!(
        ordinary_reply.contains("open incidents: 1"),
        "reply={ordinary_reply}"
    );
    assert!(
        !ordinary_reply.contains("counts once"),
        "a store with no siblings must read exactly as it always has: reply={ordinary_reply}"
    );
}

/// The capture reply is the surface an operator sees on every single use, so it is
/// where a half-told story does the most damage. Once a blocked close retires a
/// cluster, the reply still counts those incidents as open while having no cluster
/// left to name — it must say where they went rather than trail off into an empty
/// symptom list.
#[test]
fn capture_reply_names_evidence_retired_by_a_blocked_close() {
    let fixture = repository_with_demo_skill();
    let triggers = ["evt_1", "evt_2", "evt_3"];
    // Seeded without a run condition: the key is optional in `event.v1` and these are the
    // shape this crate wrote before it existed.
    let mut events = triggers
        .iter()
        .enumerate()
        .map(|(index, event_id)| {
            seeded_use_event(
                event_id,
                index,
                &format!("session-{index}"),
                event_id,
                "friction",
                "execution",
                None,
            )
        })
        .collect::<Vec<_>>();
    events.extend(seeded_review(
        "rev_blocked",
        "blocked_no_valid_test",
        &triggers,
        10,
    ));
    seed_event_stream(fixture.path(), &events);

    let output = clean_record(fixture.path(), "Task after blocked close", "later-session")
        .arg("--human")
        .output()
        .expect("run skill-evidence record");
    assert!(output.status.success());
    let reply = String::from_utf8(output.stdout).expect("human reply UTF-8");
    assert!(
        !reply.contains("independent by symptom: )"),
        "an empty symptom list tells the operator nothing: reply={reply}"
    );
    assert!(
        reply.contains("retired as untestable: 3"),
        "the reply must account for the incidents it still counts as open: reply={reply}"
    );
}

#[test]
fn skill_evidence_hash_is_stable_and_read_only() {
    let fixture = repository_with_demo_skill();

    let run = || {
        skill_evidence()
            .args(["skills", "evidence", "hash", "--root"])
            .arg(fixture.path())
            .args(["--target", ".claude/skills/demo-skill"])
            .output()
            .expect("run skill-evidence hash")
    };

    let first = run();
    assert!(
        first.status.success(),
        "hash failed: stdout={} stderr={}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );
    let report: Value = serde_json::from_slice(&first.stdout).expect("hash report JSON");
    assert_eq!(report["schema"], "skill-evidence.skill-evidence.hash.v1");
    assert_eq!(
        report["content_hash"],
        "8fd064beeb351b7698023277ac023aa2dfd2cca632a069018a6e1c739316e5bf"
    );
    assert_eq!(report["file_count"], 1);

    let second = run();
    assert_eq!(second.stdout, first.stdout);
    fs::write(
        fixture.path().join(".claude/skills/demo-skill/SKILL.md"),
        "---\nname: demo-skill\n---\nChanged body.\n",
    )
    .expect("change target");
    let third = run();
    assert!(third.status.success());
    let changed: Value = serde_json::from_slice(&third.stdout).expect("changed hash report");
    assert_ne!(changed["content_hash"], report["content_hash"]);
    assert!(!fixture.path().join("reports").exists());
}

#[test]
fn skill_evidence_hash_uses_legacy_forward_slashes_for_nested_paths() {
    let fixture = repository_with_demo_skill();
    let nested = fixture
        .path()
        .join(".claude/skills/demo-skill/assets/nested.txt");
    fs::create_dir_all(nested.parent().expect("nested parent")).expect("create nested directory");
    fs::write(&nested, "Nested evidence.\n").expect("write nested target file");

    let output = skill_evidence()
        .args(["skills", "evidence", "hash", "--root"])
        .arg(fixture.path())
        .args(["--target", ".claude/skills/demo-skill"])
        .output()
        .expect("hash nested skill");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).expect("hash report JSON");
    assert_eq!(
        report["content_hash"],
        "f65a264c97d81d5f67107ecf22d5bb7afccd668c72fd8ecf8778cb4d8e9c911b"
    );
    assert_eq!(report["file_count"], 2);
    assert!(!fixture.path().join("reports").exists());
}

#[test]
fn skill_evidence_derive_refuses_without_creating_a_missing_evidence_store() {
    let fixture = repository_with_demo_skill();
    let evidence_directory = fixture.path().join("reports/skill-evidence/demo-skill");
    let events_path = evidence_directory.join("events.jsonl");

    let output = skill_evidence()
        .args(["skills", "evidence", "derive", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--session-id",
            "derive-session",
        ])
        .output()
        .expect("run skill-evidence derive without an event stream");

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).expect("UTF-8 refusal diagnostic"),
        format!(
            "Cannot derive a gate projection because the event stream does not exist: {}. Nothing modified.\n",
            events_path.display()
        )
    );
    assert!(!evidence_directory.exists());
}

#[test]
fn skill_evidence_derive_accepts_an_empty_event_stream() {
    let fixture = repository_with_demo_skill();
    let evidence_directory = fixture.path().join("reports/skill-evidence/demo-skill");
    fs::create_dir_all(&evidence_directory).expect("create evidence directory");
    let events_path = evidence_directory.join("events.jsonl");
    fs::write(&events_path, b"").expect("write empty event stream");

    let output = skill_evidence()
        .args(["skills", "evidence", "derive", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--session-id",
            "derive-session",
        ])
        .output()
        .expect("derive an empty event stream");

    assert!(
        output.status.success(),
        "derive failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let status: Value = serde_json::from_slice(&output.stdout).expect("gate status JSON");
    assert_eq!(status["state"], "closed");
    assert_eq!(status["qualifying_uses_on_current_hash"], 0);
    assert_eq!(
        serde_json::from_slice::<Value>(
            &fs::read(evidence_directory.join("gate-status.json"))
                .expect("read persisted projection")
        )
        .expect("persisted projection JSON"),
        status
    );
    assert_eq!(fs::read(events_path).expect("reread empty stream"), b"");
}

#[test]
fn skill_evidence_derive_accepts_legacy_v1_stream_without_rewriting_events() {
    let fixture = repository_with_demo_skill();
    let evidence_directory = fixture.path().join("reports/skill-evidence/demo-skill");
    fs::create_dir_all(&evidence_directory).expect("create evidence directory");
    let events_path = evidence_directory.join("events.jsonl");
    let legacy_event = json!({
        "schema_version": 1,
        "event_id": "evt_legacy",
        "event_type": "use_recorded",
        "recorded_at": "2026-01-02T03:04:05.678Z",
        "operator_workflow": "skill-evidence-capture",
        "target": {
            "name": "demo-skill",
            "repo_relative_path": ".claude/skills/demo-skill",
            "content_hash": "8fd064beeb351b7698023277ac023aa2dfd2cca632a069018a6e1c739316e5bf",
            "repo_head": "unavailable"
        },
        "top_level_session_id": "legacy-session",
        "payload": {
            "qualifying_use": true,
            "retrospective": false,
            "task_label": "legacy incident",
            "task_fingerprint": "legacy-fingerprint",
            "outcome": "friction",
            "symptom_key": "execution",
            "expected": "expected",
            "observed": "observed",
            "consequence": "consequence",
            "workaround_taken": null,
            "evidence_refs": [],
            "same_run_group": "legacy-run"
        }
    });
    let event_bytes = format!("{legacy_event}\n").into_bytes();
    fs::write(&events_path, &event_bytes).expect("write legacy stream");

    let output = skill_evidence()
        .args(["skills", "evidence", "derive", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--session-id",
            "fresh-session",
        ])
        .output()
        .expect("run skill-evidence derive");

    assert!(
        output.status.success(),
        "derive failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let status: Value = serde_json::from_slice(&output.stdout).expect("gate status JSON");
    assert_eq!(status["schema_version"], 1);
    assert_eq!(status["state"], "collecting");
    assert_eq!(status["qualifying_uses_on_current_hash"], 1);
    assert_eq!(status["open_incident_ids"], json!(["evt_legacy"]));
    assert_eq!(status["derivation_session_id"], "fresh-session");
    assert_eq!(
        fs::read(&events_path).expect("read legacy stream"),
        event_bytes
    );
    let persisted: Value = serde_json::from_slice(
        &fs::read(evidence_directory.join("gate-status.json")).expect("read gate projection"),
    )
    .expect("persisted gate JSON");
    assert_eq!(persisted, status);
}

#[test]
fn skill_evidence_record_appends_one_event_and_refreshes_the_projection() {
    let fixture = repository_with_demo_skill();

    let output = skill_evidence()
        .args(["skills", "evidence", "record", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "clean",
            "--task-label",
            "Task one",
            "--session-id",
            "record-session",
            "--human",
        ])
        .output()
        .expect("run skill-evidence record");

    assert!(
        output.status.success(),
        "record failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence_directory = fixture.path().join("reports/skill-evidence/demo-skill");
    let event_lines = fs::read_to_string(evidence_directory.join("events.jsonl"))
        .expect("read event stream")
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_eq!(event_lines.len(), 1);
    let event: Value = serde_json::from_str(&event_lines[0]).expect("recorded event JSON");
    assert_eq!(event["schema_version"], 1);
    assert_eq!(event["event_type"], "use_recorded");
    assert_eq!(event["operator_workflow"], "skill-evidence-capture");
    assert_eq!(event["top_level_session_id"], "record-session");
    assert_eq!(event["target"]["repo_head"], "unavailable");
    assert_eq!(event["payload"]["task_label"], "Task one");
    assert_eq!(event["payload"]["outcome"], "clean");
    assert_eq!(event["payload"]["run_condition"], Value::Null);
    assert_eq!(
        String::from_utf8(output.stdout).expect("human reply UTF-8"),
        format!(
            "Evidence recorded: {}.\nGate: closed.\nNo action authorized.\n",
            event["event_id"].as_str().expect("event id")
        )
    );
    let gate: Value = serde_json::from_slice(
        &fs::read(evidence_directory.join("gate-status.json")).expect("read gate"),
    )
    .expect("gate status JSON");
    assert_eq!(gate["state"], "closed");
    assert_eq!(gate["qualifying_uses_on_current_hash"], 1);
}

#[test]
fn skill_evidence_default_run_group_distinguishes_top_level_sessions() {
    let fixture = repository_with_demo_skill();

    let first = clean_record(fixture.path(), "Recurring task", "session-a")
        .output()
        .expect("record first session");
    assert!(
        first.status.success(),
        "first record failed: stdout={} stderr={}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );

    let second = clean_record(fixture.path(), " recurring   TASK ", "session-b")
        .output()
        .expect("record second session");
    assert!(
        second.status.success(),
        "second record failed: stdout={} stderr={}",
        String::from_utf8_lossy(&second.stdout),
        String::from_utf8_lossy(&second.stderr)
    );

    let evidence = fixture.path().join("reports/skill-evidence/demo-skill");
    let events = fs::read_to_string(evidence.join("events.jsonl"))
        .expect("read event stream")
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("recorded event JSON"))
        .collect::<Vec<_>>();
    assert_eq!(events.len(), 2);
    assert_eq!(
        events[0]["payload"]["task_fingerprint"],
        events[1]["payload"]["task_fingerprint"]
    );
    assert_ne!(
        events[0]["payload"]["same_run_group"],
        events[1]["payload"]["same_run_group"]
    );
    let gate: Value = serde_json::from_slice(
        &fs::read(evidence.join("gate-status.json")).expect("read gate projection"),
    )
    .expect("gate status JSON");
    assert_eq!(gate["qualifying_uses_on_current_hash"], 2);
}

#[test]
fn skill_evidence_legacy_run_group_blocks_only_its_top_level_session() {
    let fixture = repository_with_demo_skill();
    let evidence = fixture.path().join("reports/skill-evidence/demo-skill");
    fs::create_dir_all(&evidence).expect("create evidence directory");
    let legacy_event = json!({
        "schema_version": 1,
        "event_id": "evt_legacy_run_group",
        "event_type": "use_recorded",
        "recorded_at": "2026-01-02T03:04:05.678Z",
        "operator_workflow": "skill-evidence-capture",
        "target": {
            "name": "demo-skill",
            "repo_relative_path": ".claude/skills/demo-skill",
            "content_hash": "8fd064beeb351b7698023277ac023aa2dfd2cca632a069018a6e1c739316e5bf",
            "repo_head": "legacy-head"
        },
        "top_level_session_id": "session-a",
        "payload": {
            "qualifying_use": true,
            "retrospective": false,
            "task_label": "Recurring task",
            "task_fingerprint": "cabe095136df1427",
            "outcome": "clean",
            "symptom_key": null,
            "expected": null,
            "observed": null,
            "consequence": null,
            "workaround_taken": null,
            "evidence_refs": [],
            "same_run_group": "32458e96b550"
        }
    });
    fs::write(evidence.join("events.jsonl"), format!("{legacy_event}\n"))
        .expect("write legacy event");

    let different_session = clean_record(fixture.path(), " recurring   TASK ", "session-b")
        .output()
        .expect("record different session");
    assert!(
        different_session.status.success(),
        "different-session record failed: stdout={} stderr={}",
        String::from_utf8_lossy(&different_session.stdout),
        String::from_utf8_lossy(&different_session.stderr)
    );

    let same_session = clean_record(fixture.path(), "Recurring task", "session-a")
        .output()
        .expect("record matching legacy session");
    assert_eq!(same_session.status.code(), Some(3));
    let refusal = String::from_utf8_lossy(&same_session.stderr);
    assert!(refusal.contains("Duplicate receipt refused"), "{refusal}");
    assert!(refusal.contains("evt_legacy_run_group"), "{refusal}");
    assert!(
        !refusal.contains("different top-level session"),
        "{refusal}"
    );

    assert_eq!(
        fs::read_to_string(evidence.join("events.jsonl"))
            .expect("read event stream")
            .lines()
            .count(),
        2
    );
}

#[test]
fn skill_evidence_explicit_cross_session_run_group_refusal_identifies_the_prior_use() {
    let fixture = repository_with_demo_skill();
    let explicit_record = |task_label: &str, session_id: &str| {
        let mut command = clean_record(fixture.path(), task_label, session_id);
        command.args(["--same-run-group", "deliberate-continuation"]);
        command
    };

    let first = explicit_record("First task label", "session-a")
        .output()
        .expect("record explicit run group");
    assert!(
        first.status.success(),
        "first record failed: stdout={} stderr={}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );
    let events_path = fixture
        .path()
        .join("reports/skill-evidence/demo-skill/events.jsonl");
    let prior: Value = serde_json::from_str(
        fs::read_to_string(&events_path)
            .expect("read prior event")
            .trim_end(),
    )
    .expect("prior event JSON");
    let projection_path = fixture
        .path()
        .join("reports/skill-evidence/demo-skill/gate-status.json");
    let events_before_refusal = fs::read(&events_path).expect("read event stream before refusal");
    let projection_before_refusal =
        fs::read(&projection_path).expect("read projection before refusal");

    let duplicate = explicit_record("Different task label", "session-b")
        .output()
        .expect("record cross-session continuation");
    assert_eq!(duplicate.status.code(), Some(3));
    assert!(
        duplicate.stdout.is_empty(),
        "unexpected stdout: {duplicate:?}"
    );
    let refusal = String::from_utf8_lossy(&duplicate.stderr);
    for required in [
        "Duplicate receipt refused",
        prior["event_id"].as_str().expect("prior event id"),
        "session-a",
        prior["recorded_at"]
            .as_str()
            .expect("prior recorded timestamp"),
        "different top-level session",
        "cannot receive another receipt while the target is unchanged",
        "do not record the same run again as a new use",
        "Nothing recorded",
    ] {
        assert!(
            refusal.contains(required),
            "missing {required:?} in {refusal}"
        );
    }
    assert!(
        !refusal.contains("--same-run-group"),
        "the refusing flag must not be presented again as the remedy: {refusal}"
    );
    assert_eq!(
        fs::read(&events_path).expect("read unchanged event stream"),
        events_before_refusal
    );
    assert_eq!(
        fs::read(&projection_path).expect("read unchanged projection"),
        projection_before_refusal
    );
}

#[test]
fn skill_evidence_refusals_and_integrity_failures_do_not_mutate_the_store() {
    let fixture = repository_with_demo_skill();
    assert!(
        clean_record(fixture.path(), "same task", "session-1")
            .output()
            .expect("record initial event")
            .status
            .success()
    );
    let evidence = fixture.path().join("reports/skill-evidence/demo-skill");
    let events_path = evidence.join("events.jsonl");
    let projection_path = evidence.join("gate-status.json");
    let original_events = fs::read(&events_path).expect("initial events");
    let original_projection = fs::read(&projection_path).expect("initial projection");

    let duplicate = clean_record(fixture.path(), "Same  TASK", "session-1")
        .output()
        .expect("run duplicate record");
    assert_eq!(duplicate.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&duplicate.stderr).contains("Duplicate receipt refused"));

    let invalid_clean = skill_evidence()
        .args(["skills", "evidence", "record", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "clean",
            "--task-label",
            "invalid clean",
            "--symptom-key",
            "cost",
            "--session-id",
            "session-3",
        ])
        .output()
        .expect("run invalid clean record");
    assert_eq!(invalid_clean.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&invalid_clean.stderr)
            .contains("--symptom-key is not allowed for a clean outcome")
    );

    let retrospective = skill_evidence()
        .args(["skills", "evidence", "record", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "clean",
            "--task-label",
            "retrospective without evidence",
            "--retrospective",
            "--session-id",
            "session-4",
        ])
        .output()
        .expect("run inadmissible retrospective");
    assert_eq!(retrospective.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&retrospective.stderr).contains("memory alone is inadmissible")
    );

    let incident_without_condition = skill_evidence()
        .args(["skills", "evidence", "record", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "friction",
            "--task-label",
            "incident without condition",
            "--symptom-key",
            "output",
            "--expected",
            "expected",
            "--observed",
            "observed",
            "--consequence",
            "consequence",
            "--session-id",
            "session-5",
        ])
        .output()
        .expect("run invalid incident");
    assert_eq!(incident_without_condition.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&incident_without_condition.stderr)
            .contains("--run-condition is required for a non-clean outcome")
    );

    assert_eq!(
        fs::read(&events_path).expect("events after refusals"),
        original_events
    );
    assert_eq!(
        fs::read(&projection_path).expect("projection after refusals"),
        original_projection
    );

    fs::write(
        &events_path,
        [original_events.as_slice(), b"not json\n"].concat(),
    )
    .expect("corrupt event stream");
    let corrupt_events = fs::read(&events_path).expect("corrupt events");
    let record_on_corruption = clean_record(fixture.path(), "new task", "session-6")
        .output()
        .expect("record on corrupt stream");
    assert_eq!(record_on_corruption.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&record_on_corruption.stderr)
            .contains("Event stream integrity failure")
    );
    assert_eq!(
        fs::read(&events_path).expect("events after integrity failure"),
        corrupt_events
    );
    assert_eq!(
        fs::read(&projection_path).expect("projection after integrity failure"),
        original_projection
    );

    let derive = skill_evidence()
        .args(["skills", "evidence", "derive", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--session-id",
            "session-7",
        ])
        .output()
        .expect("derive corrupt stream");
    assert!(derive.status.success());
    let blocked: Value = serde_json::from_slice(&derive.stdout).expect("blocked projection");
    assert_eq!(blocked["state"], "blocked");
    assert!(
        blocked["integrity_errors"][0]
            .as_str()
            .is_some_and(|error| error.contains("not valid JSON"))
    );
    assert_eq!(
        fs::read(&events_path).expect("events after blocked derive"),
        corrupt_events
    );
}

#[test]
fn skill_evidence_conflicting_host_sessions_fail_closed_before_store_creation() {
    let fixture = repository_with_demo_skill();
    let output = skill_evidence()
        .args(["skills", "evidence", "record", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "clean",
            "--task-label",
            "conflicting session",
        ])
        .env("CLAUDE_CODE_SESSION_ID", "claude-session")
        .env("CODEX_THREAD_ID", "codex-thread")
        .output()
        .expect("run record with conflicting sessions");

    assert_eq!(output.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("Conflicting top-level-session identities")
    );
    assert!(!fixture.path().join("reports").exists());
}

fn assert_cross_host_threshold_becomes_eligible(
    threshold_environment: (&str, &str),
    derivation_environment: (&str, &str),
) {
    let fixture = repository_with_demo_skill();
    let incident =
        |task_label: &str, session_id: Option<&str>, environment: Option<(&str, &str)>| {
            let mut command = skill_evidence();
            command
                .args(["skills", "evidence", "record", "--root"])
                .arg(fixture.path())
                .args([
                    "--target",
                    ".claude/skills/demo-skill",
                    "--outcome",
                    "friction",
                    "--task-label",
                    task_label,
                    "--symptom-key",
                    "execution",
                    "--expected",
                    "expected",
                    "--observed",
                    "observed",
                    "--consequence",
                    "consequence",
                    "--run-condition",
                    "cross-host parity fixture",
                ])
                .env_remove("CLAUDE_CODE_SESSION_ID")
                .env_remove("CODEX_THREAD_ID");
            if let Some(session_id) = session_id {
                command.args(["--session-id", session_id]);
            }
            if let Some((name, value)) = environment {
                command.env(name, value);
            }
            command.output().expect("record cross-host incident")
        };

    for (task_label, session_id) in [
        ("first incident", "session-a"),
        ("second incident", "session-b"),
    ] {
        let output = incident(task_label, Some(session_id), None);
        assert!(
            output.status.success(),
            "record failed: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let threshold = incident("threshold incident", None, Some(threshold_environment));
    assert!(
        threshold.status.success(),
        "threshold record failed: stdout={} stderr={}",
        String::from_utf8_lossy(&threshold.stdout),
        String::from_utf8_lossy(&threshold.stderr)
    );

    let derive = skill_evidence()
        .args(["skills", "evidence", "derive", "--root"])
        .arg(fixture.path())
        .args(["--target", ".claude/skills/demo-skill"])
        .env_remove("CLAUDE_CODE_SESSION_ID")
        .env_remove("CODEX_THREAD_ID")
        .env(derivation_environment.0, derivation_environment.1)
        .output()
        .expect("derive cross-host threshold");
    assert!(
        derive.status.success(),
        "derive failed: stdout={} stderr={}",
        String::from_utf8_lossy(&derive.stdout),
        String::from_utf8_lossy(&derive.stderr)
    );
    let status: Value = serde_json::from_slice(&derive.stdout).expect("gate status JSON");
    assert_eq!(status["state"], "eligible");
    assert_eq!(status["threshold_session_id"], threshold_environment.1);
    assert_eq!(status["derivation_session_id"], derivation_environment.1);
}

#[test]
fn claude_session_threshold_becomes_eligible_in_a_different_codex_thread() {
    assert_cross_host_threshold_becomes_eligible(
        ("CLAUDE_CODE_SESSION_ID", "claude-threshold"),
        ("CODEX_THREAD_ID", "codex-derivation"),
    );
}

#[test]
fn codex_thread_threshold_becomes_eligible_in_a_different_claude_session() {
    assert_cross_host_threshold_becomes_eligible(
        ("CODEX_THREAD_ID", "codex-threshold"),
        ("CLAUDE_CODE_SESSION_ID", "claude-derivation"),
    );
}

#[test]
fn skill_evidence_self_receipts_are_incident_only_and_require_evidence() {
    let fixture = tempfile::tempdir().expect("temporary repository root");
    let target = fixture.path().join(".claude/skills/skill-evidence-capture");
    fs::create_dir_all(&target).expect("create capture skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: skill-evidence-capture\n---\nCapture.\n",
    )
    .expect("write capture skill");

    let base = || {
        let mut command = skill_evidence();
        command
            .args(["skills", "evidence", "record", "--root"])
            .arg(fixture.path())
            .args([
                "--target",
                ".claude/skills/skill-evidence-capture",
                "--task-label",
                "self receipt",
                "--session-id",
                "self-session",
            ]);
        command
    };

    let clean = base()
        .args(["--outcome", "clean"])
        .output()
        .expect("run clean self receipt");
    assert_eq!(clean.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&clean.stderr).contains("incident-only"));

    let without_evidence = base()
        .args([
            "--outcome",
            "material_failure",
            "--symptom-key",
            "execution",
            "--expected",
            "expected",
            "--observed",
            "observed",
            "--consequence",
            "consequence",
            "--run-condition",
            "condition",
        ])
        .output()
        .expect("run incident self receipt without evidence");
    assert_eq!(without_evidence.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&without_evidence.stderr).contains("must cite concrete evidence")
    );

    let with_evidence = base()
        .args([
            "--outcome",
            "material_failure",
            "--symptom-key",
            "execution",
            "--expected",
            "expected",
            "--observed",
            "observed",
            "--consequence",
            "consequence",
            "--run-condition",
            "condition",
            "--evidence-ref",
            "reports/failed-capture.txt",
        ])
        .output()
        .expect("run supported incident self receipt");
    assert!(
        with_evidence.status.success(),
        "{}",
        String::from_utf8_lossy(&with_evidence.stderr)
    );
}

#[test]
fn concurrent_skill_evidence_records_are_serialized_into_one_current_projection() {
    let fixture = repository_with_demo_skill();
    let evidence = fixture.path().join("reports/skill-evidence/demo-skill");
    let lock = evidence.join(".lock");
    fs::create_dir_all(&lock).expect("create test-owned lock");
    fs::write(lock.join("owner"), "test-owner").expect("write test lock owner");

    let mut first = clean_record(fixture.path(), "concurrent task one", "session-one");
    first.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut second = clean_record(fixture.path(), "concurrent task two", "session-two");
    second.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut first = first.spawn().expect("spawn first record");
    let mut second = second.spawn().expect("spawn second record");
    thread::sleep(Duration::from_millis(150));
    assert_eq!(first.try_wait().expect("poll first writer"), None);
    assert_eq!(second.try_wait().expect("poll second writer"), None);
    assert!(!evidence.join("events.jsonl").exists());
    assert!(!evidence.join("gate-status.json").exists());
    assert_eq!(
        fs::read_to_string(lock.join("owner")).expect("read test owner"),
        "test-owner"
    );

    fs::remove_file(lock.join("owner")).expect("remove test owner");
    fs::remove_dir(&lock).expect("release test-owned lock");
    let first = first.wait_with_output().expect("wait for first record");
    let second = second.wait_with_output().expect("wait for second record");
    assert!(
        first.status.success(),
        "first record failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        second.status.success(),
        "second record failed: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    for output in [&first, &second] {
        let receipt: Value =
            serde_json::from_slice(&output.stdout).expect("machine-readable record receipt");
        assert_eq!(receipt["schema"], "skill-evidence.skill-evidence.record.v1");
        assert_eq!(receipt["gate_status"]["state"], "closed");
        assert!(
            receipt["terminal_reply"]
                .as_str()
                .is_some_and(|reply| reply.starts_with("Evidence recorded: evt_"))
        );
    }

    let stream = fs::read_to_string(evidence.join("events.jsonl")).expect("read event stream");
    assert_eq!(stream.lines().count(), 2);
    for line in stream.lines() {
        let _: Value = serde_json::from_str(line).expect("each event line is complete JSON");
    }
    let projection: Value = serde_json::from_slice(
        &fs::read(evidence.join("gate-status.json")).expect("read projection"),
    )
    .expect("projection JSON");
    assert_eq!(projection["state"], "closed");
    assert_eq!(projection["qualifying_uses_on_current_hash"], 2);
    assert!(!evidence.join(".lock").exists());
}

/// A lock the crate will not break is only escapable if the operator is told they may
/// break it. [`Error`](skill_evidence::Error) deliberately omits that sentence,
/// because naming the act means naming a binary — so the surface that prints the
/// error is the only place the escape route exists.
#[test]
fn a_record_blocked_by_an_abandoned_lock_tells_the_operator_how_to_release_it() {
    let fixture = repository_with_demo_skill();
    let evidence = fixture.path().join("reports/skill-evidence/demo-skill");
    let lock = evidence.join(".lock");
    fs::create_dir_all(&lock).expect("create abandoned lock");
    fs::write(lock.join("owner"), "lock_a-run-that-died").expect("write abandoned owner");

    let blocked = clean_record(
        fixture.path(),
        "task behind an abandoned lock",
        "session-blocked",
    )
    .output()
    .expect("run record behind the abandoned lock");

    assert_eq!(blocked.status.code(), Some(1));
    let report = String::from_utf8_lossy(&blocked.stderr);
    assert!(
        report.contains("lock_a-run-that-died"),
        "the operator must learn who holds it: {report}"
    );
    assert!(
        report.contains(lock.to_string_lossy().as_ref()),
        "the operator must learn which lock: {report}"
    );
    assert!(
        report.contains(
            &support::host().recovery_instruction(skill_evidence::Recovery::ReleaseEvidenceLock)
        ),
        "the operator must learn what to do about it: {report}"
    );
    assert!(
        lock.is_dir(),
        "the lock is the operator's to release, not the command's"
    );
    assert!(
        !evidence.join("events.jsonl").exists(),
        "a record that never got the lock appends nothing"
    );
}

#[test]
fn skill_evidence_event_schema_matches_the_compiled_use_record_contract() {
    let root = repository_root();
    let schema: Value = serde_json::from_slice(
        &fs::read(root.join("schemas/skill-evidence/event.v1.schema.json"))
            .expect("read event schema"),
    )
    .expect("event schema JSON");
    let validator = jsonschema::validator_for(&schema).expect("compile event schema");

    let clean_fixture = repository_with_demo_skill();
    assert!(
        clean_record(clean_fixture.path(), "schema clean", "schema-session")
            .output()
            .expect("record schema clean event")
            .status
            .success()
    );
    let clean: Value = serde_json::from_str(
        fs::read_to_string(
            clean_fixture
                .path()
                .join("reports/skill-evidence/demo-skill/events.jsonl"),
        )
        .expect("read clean event")
        .trim_end(),
    )
    .expect("clean event JSON");

    let incident_fixture = repository_with_demo_skill();
    let incident_output = skill_evidence()
        .args(["skills", "evidence", "record", "--root"])
        .arg(incident_fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--outcome",
            "friction",
            "--task-label",
            "schema incident",
            "--symptom-key",
            "output",
            "--expected",
            "expected",
            "--observed",
            "observed",
            "--consequence",
            "consequence",
            "--run-condition",
            "schema conformance case",
            "--session-id",
            "schema-session",
        ])
        .output()
        .expect("record schema incident");
    assert!(
        incident_output.status.success(),
        "{}",
        String::from_utf8_lossy(&incident_output.stderr)
    );
    let incident: Value = serde_json::from_str(
        fs::read_to_string(
            incident_fixture
                .path()
                .join("reports/skill-evidence/demo-skill/events.jsonl"),
        )
        .expect("read incident event")
        .trim_end(),
    )
    .expect("incident event JSON");

    assert!(
        validator.is_valid(&clean),
        "producer clean must match schema"
    );
    assert!(
        validator.is_valid(&incident),
        "producer incident must match schema"
    );

    let mut clean_with_incident_field = clean.clone();
    clean_with_incident_field["payload"]["expected"] = json!("not null");
    let mut incident_with_null_required = incident.clone();
    incident_with_null_required["payload"]["expected"] = Value::Null;
    let mut incident_with_empty_required = incident.clone();
    incident_with_empty_required["payload"]["observed"] = json!("");
    let mut incident_with_empty_optional = incident.clone();
    incident_with_empty_optional["payload"]["run_condition"] = json!("");
    let mut retrospective_without_evidence = clean;
    retrospective_without_evidence["payload"]["retrospective"] = Value::Bool(true);

    for (case, invalid) in [
        ("clean non-null incident field", clean_with_incident_field),
        ("non-clean null required field", incident_with_null_required),
        (
            "non-clean empty required field",
            incident_with_empty_required,
        ),
        (
            "non-clean empty optional field",
            incident_with_empty_optional,
        ),
        (
            "retrospective without evidence",
            retrospective_without_evidence,
        ),
    ] {
        assert!(!validator.is_valid(&invalid), "schema must reject {case}");
        let fixture = repository_with_demo_skill();
        let evidence = fixture.path().join("reports/skill-evidence/demo-skill");
        fs::create_dir_all(&evidence).expect("create invalid-case evidence directory");
        fs::write(
            evidence.join("events.jsonl"),
            format!("{invalid}\n").as_bytes(),
        )
        .expect("write invalid event");
        let derive = skill_evidence()
            .args(["skills", "evidence", "derive", "--root"])
            .arg(fixture.path())
            .args([
                "--target",
                ".claude/skills/demo-skill",
                "--session-id",
                "schema-reader",
            ])
            .output()
            .expect("derive invalid event");
        assert!(derive.status.success(), "{case}");
        let status: Value =
            serde_json::from_slice(&derive.stdout).expect("blocked gate projection");
        assert_eq!(status["state"], "blocked", "{case}");
    }
}

#[test]
fn skill_evidence_v1_event_and_gate_contracts_are_published() {
    let root = repository_root();
    let event: Value = serde_json::from_slice(
        &fs::read(root.join("schemas/skill-evidence/event.v1.schema.json"))
            .expect("read event schema"),
    )
    .expect("event schema JSON");
    assert_eq!(event["$id"], "skill-evidence.skill-evidence.event.v1");
    assert_eq!(
        event["properties"]["event_type"]["enum"],
        json!([
            "use_recorded",
            "review_started",
            "review_disposition",
            "validation_completed",
            "change_landed",
            "decontamination_started",
            "decontamination_completed"
        ])
    );

    let gate: Value = serde_json::from_slice(
        &fs::read(root.join("schemas/skill-evidence/gate-status.v1.schema.json"))
            .expect("read gate schema"),
    )
    .expect("gate schema JSON");
    assert_eq!(gate["$id"], "skill-evidence.skill-evidence.gate-status.v1");
    assert_eq!(
        gate["properties"]["state"]["enum"],
        json!([
            "closed",
            "collecting",
            "eligible_pending_cooldown",
            "eligible",
            "quarantined_pending_cooldown",
            "quarantined_eligible",
            "review_in_progress",
            "blocked"
        ])
    );
}

#[test]
fn capture_skill_routes_record_only_through_the_compiled_rust_command() {
    let root = repository_root();
    let skill = fs::read_to_string(root.join(".claude/skills/skill-evidence-capture/SKILL.md"))
        .expect("read capture skill");
    assert!(
        skill.contains("cargo run --locked -p skill-evidence -- skills evidence record"),
        "capture skill must name the compiled record command"
    );
    assert!(
        !skill.contains("node .claude/skills/skill-evidence-capture/scripts/evidence.mjs record"),
        "capture skill must not retain its JavaScript production command"
    );
    let openai =
        fs::read_to_string(root.join(".claude/skills/skill-evidence-capture/agents/openai.yaml"))
            .expect("read capture client metadata");
    assert!(openai.contains("allow_implicit_invocation: false"));
    assert!(openai.contains("Record a receipt for a completed skill use"));

    #[cfg(unix)]
    {
        assert_eq!(
            fs::read_link(root.join(".agents/skills/skill-evidence-capture"))
                .expect("read discovery link"),
            Path::new("../../.claude/skills/skill-evidence-capture")
        );
    }
}

#[test]
fn skill_evidence_defaults_repository_root_to_the_git_toplevel() {
    let fixture = repository_with_demo_skill();
    let git = Command::new("git")
        .args(["init", "--quiet"])
        .arg(fixture.path())
        .output()
        .expect("initialize fixture repository");
    assert!(
        git.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&git.stderr)
    );
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
        .expect("hash from nested repository path");

    assert!(
        output.status.success(),
        "hash failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).expect("hash report");
    assert_eq!(report["file_count"], 1);
}

#[test]
fn invalid_utf8_event_stream_derives_blocked_and_refuses_append() {
    let fixture = repository_with_demo_skill();
    let evidence = fixture.path().join("reports/skill-evidence/demo-skill");
    fs::create_dir_all(&evidence).expect("create evidence directory");
    let events = evidence.join("events.jsonl");
    fs::write(&events, [0xff, b'\n']).expect("write invalid UTF-8 stream");
    let original = fs::read(&events).expect("read invalid stream");

    let derive = skill_evidence()
        .args(["skills", "evidence", "derive", "--root"])
        .arg(fixture.path())
        .args([
            "--target",
            ".claude/skills/demo-skill",
            "--session-id",
            "derive-session",
        ])
        .output()
        .expect("derive invalid UTF-8 stream");
    assert!(
        derive.status.success(),
        "derive failed: {}",
        String::from_utf8_lossy(&derive.stderr)
    );
    let blocked: Value = serde_json::from_slice(&derive.stdout).expect("blocked status");
    assert_eq!(blocked["state"], "blocked");
    assert!(
        blocked["integrity_errors"][0]
            .as_str()
            .is_some_and(|error| error.contains("UTF-8"))
    );

    let record = clean_record(fixture.path(), "blocked append", "record-session")
        .output()
        .expect("record invalid UTF-8 stream");
    assert_eq!(record.status.code(), Some(1));
    assert_eq!(fs::read(events).expect("stream remains"), original);
}

#[test]
fn missing_or_non_skill_targets_refuse_without_creating_a_store() {
    let fixture = repository_with_demo_skill();
    let record_target = |target: &str, label: &str, session: &str| {
        skill_evidence()
            .args(["skills", "evidence", "record", "--root"])
            .arg(fixture.path())
            .args([
                "--target",
                target,
                "--outcome",
                "clean",
                "--task-label",
                label,
                "--session-id",
                session,
            ])
            .output()
            .expect("record selected target")
    };
    let missing = record_target("no/such/skill", "missing target", "session-1");
    assert_eq!(missing.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("not found"));

    let plain = fixture.path().join("plain-directory");
    fs::create_dir_all(&plain).expect("create plain directory");
    let non_skill = record_target("plain-directory", "plain target", "session-2");
    assert_eq!(non_skill.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&non_skill.stderr).contains("no SKILL.md"));
    assert!(!fixture.path().join("reports").exists());
}
