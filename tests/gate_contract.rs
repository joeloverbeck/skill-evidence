#![forbid(unsafe_code)]

use std::{fs, path::Path};

mod support;

use serde_json::{Value, json};
use skill_evidence::{DerivationInputs, GateStatus, derive_store, hash_skill};
use tempfile::TempDir;

struct Fixture {
    root: TempDir,
    target_relative: &'static str,
    target_hash: String,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().expect("temporary repository");
        let target_relative = ".claude/skills/demo-skill";
        let target = root.path().join(target_relative);
        fs::create_dir_all(&target).expect("create target skill");
        fs::write(
            target.join("SKILL.md"),
            "---\nname: demo-skill\n---\nDemo body.\n",
        )
        .expect("write target skill");
        let target_hash = hash_skill(root.path(), Path::new(target_relative), &support::host())
            .expect("hash target")
            .content_hash;
        Self {
            root,
            target_relative,
            target_hash,
        }
    }

    fn write_events(&self, events: &[Value]) {
        let evidence = self.root.path().join("reports/skill-evidence/demo-skill");
        fs::create_dir_all(&evidence).expect("create evidence directory");
        let mut bytes = events
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        bytes.push('\n');
        fs::write(evidence.join("events.jsonl"), bytes).expect("write events");
    }

    fn derive(&self, session_id: &str, now_millis: i64) -> GateStatus {
        derive_store(
            self.root.path(),
            Path::new(self.target_relative),
            &DerivationInputs {
                generated_at: "2026-01-03T00:00:00Z".to_owned(),
                now_epoch_milliseconds: now_millis,
                session_id: session_id.to_owned(),
                lock_owner: "fixture-derive-lock".to_owned(),
            },
        )
        .expect("derive gate")
    }

    fn use_event(
        &self,
        serial: usize,
        outcome: &str,
        symptom_key: Option<&str>,
        session_id: &str,
    ) -> Value {
        json!({
            "schema_version": 1,
            "event_id": format!("evt_{serial}"),
            "event_type": "use_recorded",
            "recorded_at": format!("2026-01-02T{serial:02}:00:00Z"),
            "operator_workflow": "skill-evidence-capture",
            "target": {
                "name": "demo-skill",
                "repo_relative_path": self.target_relative,
                "content_hash": self.target_hash,
                "repo_head": "fixture-head"
            },
            "top_level_session_id": session_id,
            "payload": {
                "qualifying_use": true,
                "retrospective": false,
                "task_label": format!("task {serial}"),
                "task_fingerprint": format!("fingerprint-{serial}"),
                "outcome": outcome,
                "symptom_key": symptom_key,
                "expected": symptom_key.map(|_| "expected"),
                "observed": symptom_key.map(|_| "observed"),
                "consequence": symptom_key.map(|_| "consequence"),
                "workaround_taken": null,
                "run_condition": symptom_key.map(|_| "condition"),
                "evidence_refs": [],
                "same_run_group": format!("run-{serial}")
            }
        })
    }
}

#[test]
fn friction_recurrence_requires_a_fresh_top_level_session() {
    let fixture = Fixture::new();
    fixture.write_events(&[
        fixture.use_event(1, "friction", Some("execution"), "session-a"),
        fixture.use_event(2, "friction", Some("execution"), "session-b"),
        fixture.use_event(3, "friction", Some("execution"), "session-c"),
    ]);

    let same_session = fixture.derive("session-c", 1_767_398_400_000);
    assert_eq!(same_session.state, "eligible_pending_cooldown");
    assert_eq!(
        same_session.authorization_reason.as_deref(),
        Some("friction_recurrence:execution")
    );
    assert_eq!(
        same_session.threshold_session_id.as_deref(),
        Some("session-c")
    );
    assert_eq!(same_session.trigger_event_ids, ["evt_1", "evt_2", "evt_3"]);

    let fresh_session = fixture.derive("session-d", 1_767_398_400_000);
    assert_eq!(fresh_session.state, "eligible");
    assert_eq!(
        fresh_session.derivation_session_id.as_deref(),
        Some("session-d")
    );
}

#[test]
fn two_independent_material_failures_fire_the_material_recurrence_gate() {
    let fixture = Fixture::new();
    fixture.write_events(&[
        fixture.use_event(1, "material_failure", Some("output"), "session-a"),
        fixture.use_event(2, "material_failure", Some("output"), "session-b"),
    ]);

    let status = fixture.derive("session-b", 1_767_398_400_000);
    assert_eq!(status.state, "eligible_pending_cooldown");
    assert_eq!(
        status.authorization_reason.as_deref(),
        Some("material_recurrence:output")
    );
    assert_eq!(status.trigger_event_ids, ["evt_1", "evt_2"]);
}

#[test]
fn one_contemporaneous_severe_incident_quarantines_the_target() {
    let fixture = Fixture::new();
    fixture.write_events(&[fixture.use_event(
        1,
        "severe_incident",
        Some("state"),
        "threshold-session",
    )]);

    let same_session = fixture.derive("threshold-session", 1_767_398_400_000);
    assert_eq!(same_session.state, "quarantined_pending_cooldown");
    assert_eq!(same_session.authorization_reason.as_deref(), Some("severe"));
    assert_eq!(same_session.trigger_event_ids, ["evt_1"]);

    let fresh_session = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(fresh_session.state, "quarantined_eligible");
}

#[test]
fn unavailable_threshold_session_uses_the_twelve_hour_clock() {
    let fixture = Fixture::new();
    fixture.write_events(&[
        fixture.use_event(1, "friction", Some("cost"), "unavailable"),
        fixture.use_event(2, "friction", Some("cost"), "unavailable"),
        fixture.use_event(3, "friction", Some("cost"), "unavailable"),
    ]);

    let before_deadline = fixture.derive("fresh-host-session", 1_767_365_999_999);
    assert_eq!(before_deadline.state, "eligible_pending_cooldown");
    assert_eq!(before_deadline.threshold_session_id, None);
    assert_eq!(
        before_deadline.not_before.as_deref(),
        Some("2026-01-02T15:00:00.000Z")
    );

    let at_deadline = fixture.derive("fresh-host-session", 1_767_366_000_000);
    assert_eq!(at_deadline.state, "eligible");
}

#[test]
fn every_v1_event_type_is_accepted_and_review_ownership_is_derived() {
    let fixture = Fixture::new();
    let target = json!({
        "name": "demo-skill",
        "repo_relative_path": fixture.target_relative,
        "content_hash": fixture.target_hash,
        "repo_head": "fixture-head"
    });
    let lifecycle = |serial: usize, event_type: &str, payload: Value| {
        json!({
            "schema_version": 1,
            "event_id": format!("evt_{serial}"),
            "event_type": event_type,
            "recorded_at": format!("2026-01-02T0{serial}:00:00Z"),
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": payload
        })
    };
    fixture.write_events(&[
        fixture.use_event(1, "friction", Some("execution"), "session-a"),
        lifecycle(2, "review_started", json!({"review_id": "review-1"})),
        lifecycle(3, "validation_completed", json!({"review_id": "review-1"})),
        lifecycle(4, "change_landed", json!({"review_id": "review-1"})),
        lifecycle(
            5,
            "decontamination_started",
            json!({"review_id": "decontamination-1"}),
        ),
        lifecycle(
            6,
            "decontamination_completed",
            json!({"review_id": "decontamination-1"}),
        ),
        lifecycle(
            7,
            "review_disposition",
            json!({
                "review_id": "review-1",
                "disposition": "monitor_for_recurrence",
                "adjudicated_event_ids": ["evt_1"]
            }),
        ),
    ]);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "closed");
    assert_eq!(status.open_incident_ids, Vec::<String>::new());
    assert_eq!(status.active_review_id, None);
    assert_eq!(
        status.last_completed_review_id.as_deref(),
        Some("decontamination-1")
    );
}

#[test]
fn ten_uses_with_one_open_contemporaneous_incident_fire_the_ten_use_gate() {
    let fixture = Fixture::new();
    let mut events = vec![fixture.use_event(1, "friction", Some("cost"), "incident-session")];
    for serial in 2..=10 {
        events.push(fixture.use_event(serial, "clean", None, &format!("session-{serial}")));
    }
    fixture.write_events(&events);

    let status = fixture.derive("session-10", 1_767_398_400_000);
    assert_eq!(status.qualifying_uses_on_current_hash, 10);
    assert_eq!(status.state, "eligible_pending_cooldown");
    assert_eq!(
        status.authorization_reason.as_deref(),
        Some("ten_use_unresolved")
    );
    assert_eq!(status.trigger_event_ids, ["evt_1"]);
}

#[test]
fn retrospective_tenth_use_does_not_complete_the_ten_use_gate() {
    let fixture = Fixture::new();
    let mut events = vec![fixture.use_event(1, "friction", Some("cost"), "incident-session")];
    for serial in 2..=9 {
        events.push(fixture.use_event(serial, "clean", None, &format!("session-{serial}")));
    }
    let mut retrospective = fixture.use_event(10, "clean", None, "retrospective-session");
    retrospective["payload"]["retrospective"] = Value::Bool(true);
    retrospective["payload"]["evidence_refs"] = json!(["logs/retrospective-use.txt"]);
    events.push(retrospective);
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.qualifying_uses_on_current_hash, 10);
    assert_eq!(status.state, "collecting");
    assert_eq!(status.authorization_reason, None);
    assert_eq!(status.threshold_session_id, None);
}

#[test]
fn first_ten_use_threshold_wins_over_a_later_severe_incident() {
    let fixture = Fixture::new();
    let mut events = vec![fixture.use_event(1, "friction", Some("cost"), "incident-session")];
    for serial in 2..=10 {
        events.push(fixture.use_event(serial, "clean", None, &format!("session-{serial}")));
    }
    events.push(fixture.use_event(11, "severe_incident", Some("state"), "later-session"));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.qualifying_uses_on_current_hash, 11);
    assert_eq!(status.state, "eligible");
    assert_eq!(
        status.authorization_reason.as_deref(),
        Some("ten_use_unresolved")
    );
    assert_eq!(status.trigger_event_ids, ["evt_1"]);
    assert_eq!(status.threshold_session_id.as_deref(), Some("session-10"));
}

#[test]
fn completed_same_hash_review_does_not_reopen_from_queued_pre_close_evidence() {
    let fixture = Fixture::new();
    let mut events = vec![
        fixture.use_event(1, "friction", Some("execution"), "session-1"),
        fixture.use_event(2, "material_failure", Some("output"), "session-2"),
        fixture.use_event(3, "material_failure", Some("output"), "session-3"),
    ];
    for serial in 4..=10 {
        events.push(fixture.use_event(serial, "clean", None, &format!("session-{serial}")));
    }
    let target = json!({
        "name": "demo-skill",
        "repo_relative_path": fixture.target_relative,
        "content_hash": fixture.target_hash,
        "repo_head": "fixture-head"
    });
    events.push(json!({
        "schema_version": 1,
        "event_id": "evt_review_started",
        "event_type": "review_started",
        "recorded_at": "2026-01-02T11:00:00Z",
        "operator_workflow": "skill-evolution",
        "target": target,
        "top_level_session_id": "review-session",
        "payload": {"review_id": "review-queued"}
    }));
    events.push(json!({
        "schema_version": 1,
        "event_id": "evt_review_disposition",
        "event_type": "review_disposition",
        "recorded_at": "2026-01-02T12:00:00Z",
        "operator_workflow": "skill-evolution",
        "target": target,
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": "review-queued",
            "disposition": "candidate_rejected_validation",
            "adjudicated_event_ids": ["evt_2", "evt_3"]
        }
    }));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "collecting");
    assert_eq!(status.authorization_reason, None);
    assert_eq!(
        status.review_reentry_basis.as_deref(),
        Some("queued_pre_close_evidence")
    );
    assert_eq!(status.open_incident_ids, ["evt_1"]);
}

#[test]
fn non_adjudicating_evolution_dispositions_do_not_retire_covered_incidents() {
    struct Case {
        name: &'static str,
        disposition: &'static str,
        outcomes: &'static [&'static str],
        expected_state: &'static str,
        expected_reason: Option<&'static str>,
        expected_reentry_basis: &'static str,
    }

    for case in [
        Case {
            name: "blocked review queues retained pre-close evidence",
            disposition: "blocked_no_valid_test",
            outcomes: &["friction", "friction", "friction"],
            expected_state: "collecting",
            expected_reason: None,
            expected_reentry_basis: "queued_pre_close_evidence",
        },
        Case {
            name: "superseded review queues retained pre-close evidence",
            disposition: "superseded_by_target_version",
            outcomes: &["friction", "friction", "friction"],
            expected_state: "collecting",
            expected_reason: None,
            expected_reentry_basis: "queued_pre_close_evidence",
        },
        Case {
            name: "blocked severe review remains quarantined",
            disposition: "blocked_no_valid_test",
            outcomes: &["severe_incident"],
            expected_state: "quarantined_eligible",
            expected_reason: Some("severe"),
            expected_reentry_basis: "unadjudicated_severe",
        },
    ] {
        let fixture = Fixture::new();
        let target = json!({
            "name": "demo-skill",
            "repo_relative_path": fixture.target_relative,
            "content_hash": fixture.target_hash,
            "repo_head": "fixture-head"
        });
        let mut events = case
            .outcomes
            .iter()
            .enumerate()
            .map(|(index, outcome)| {
                fixture.use_event(
                    index + 1,
                    outcome,
                    Some("execution"),
                    &format!("session-{}", index + 1),
                )
            })
            .collect::<Vec<_>>();
        let covered_ids = (1..=case.outcomes.len())
            .map(|serial| format!("evt_{serial}"))
            .collect::<Vec<_>>();
        events.push(json!({
            "schema_version": 1,
            "event_id": "evt_review_started",
            "event_type": "review_started",
            "recorded_at": "2026-01-02T10:00:00Z",
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": {"review_id": "review-non-adjudicating"}
        }));
        events.push(json!({
            "schema_version": 1,
            "event_id": "evt_review_disposition",
            "event_type": "review_disposition",
            "recorded_at": "2026-01-02T11:00:00Z",
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": {
                "review_id": "review-non-adjudicating",
                "disposition": case.disposition,
                "adjudicated_event_ids": covered_ids
            }
        }));
        fixture.write_events(&events);

        let status = fixture.derive("fresh-session", 1_767_398_400_000);
        assert_eq!(status.state, case.expected_state, "{}", case.name);
        assert_eq!(
            status.authorization_reason.as_deref(),
            case.expected_reason,
            "{}",
            case.name
        );
        assert_eq!(
            status.review_reentry_basis.as_deref(),
            Some(case.expected_reentry_basis),
            "{}",
            case.name
        );
        assert_eq!(status.open_incident_ids, covered_ids, "{}", case.name);
        assert_eq!(status.candidate_clusters.len(), 1, "{}", case.name);
        assert_eq!(
            status.candidate_clusters[0].open_event_ids, covered_ids,
            "{}",
            case.name
        );
    }
}

#[test]
fn every_adjudicating_evolution_disposition_retires_covered_incidents() {
    for disposition in [
        "resolved_by_change",
        "closed_no_skill_defect",
        "outside_target",
        "insufficient_independence",
        "monitor_for_recurrence",
        "candidate_rejected_validation",
    ] {
        let fixture = Fixture::new();
        let target = json!({
            "name": "demo-skill",
            "repo_relative_path": fixture.target_relative,
            "content_hash": fixture.target_hash,
            "repo_head": "fixture-head"
        });
        fixture.write_events(&[
            fixture.use_event(1, "friction", Some("execution"), "session-1"),
            json!({
                "schema_version": 1,
                "event_id": "evt_review_started",
                "event_type": "review_started",
                "recorded_at": "2026-01-02T02:00:00Z",
                "operator_workflow": "skill-evolution",
                "target": target,
                "top_level_session_id": "review-session",
                "payload": {"review_id": "review-adjudicating"}
            }),
            json!({
                "schema_version": 1,
                "event_id": "evt_review_disposition",
                "event_type": "review_disposition",
                "recorded_at": "2026-01-02T03:00:00Z",
                "operator_workflow": "skill-evolution",
                "target": target,
                "top_level_session_id": "review-session",
                "payload": {
                    "review_id": "review-adjudicating",
                    "disposition": disposition,
                    "adjudicated_event_ids": ["evt_1"]
                }
            }),
        ]);

        let status = fixture.derive("fresh-session", 1_767_398_400_000);
        assert_eq!(status.state, "closed", "disposition {disposition}");
        assert!(
            status.open_incident_ids.is_empty(),
            "disposition {disposition}"
        );
        assert!(
            status.candidate_clusters.is_empty(),
            "disposition {disposition}"
        );
    }
}

#[test]
fn post_review_incident_reopens_ten_use_gate_with_its_bounded_cluster() {
    let fixture = Fixture::new();
    let mut events = vec![
        fixture.use_event(1, "friction", Some("output"), "session-1"),
        fixture.use_event(2, "friction", Some("tool-compatibility"), "session-2"),
        fixture.use_event(3, "friction", Some("tool-compatibility"), "session-3"),
        fixture.use_event(4, "friction", Some("tool-compatibility"), "session-4"),
    ];
    for serial in 5..=9 {
        events.push(fixture.use_event(serial, "clean", None, &format!("session-{serial}")));
    }
    let target = json!({
        "name": "demo-skill",
        "repo_relative_path": fixture.target_relative,
        "content_hash": fixture.target_hash,
        "repo_head": "fixture-head"
    });
    events.push(json!({
        "schema_version": 1,
        "event_id": "evt_review_started",
        "event_type": "review_started",
        "recorded_at": "2026-01-02T10:00:00Z",
        "operator_workflow": "skill-evolution",
        "target": target,
        "top_level_session_id": "review-session",
        "payload": {"review_id": "review-post"}
    }));
    events.push(json!({
        "schema_version": 1,
        "event_id": "evt_review_disposition",
        "event_type": "review_disposition",
        "recorded_at": "2026-01-02T11:00:00Z",
        "operator_workflow": "skill-evolution",
        "target": target,
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": "review-post",
            "disposition": "candidate_rejected_validation",
            "adjudicated_event_ids": ["evt_2", "evt_3", "evt_4"]
        }
    }));
    events.push(fixture.use_event(10, "friction", Some("execution"), "session-10"));
    fixture.write_events(&events);

    let status = fixture.derive("session-10", 1_767_398_400_000);
    assert_eq!(status.state, "eligible_pending_cooldown");
    assert_eq!(
        status.authorization_reason.as_deref(),
        Some("ten_use_unresolved")
    );
    assert_eq!(
        status.review_reentry_basis.as_deref(),
        Some("post_review_incident")
    );
    assert_eq!(status.trigger_event_ids, ["evt_10"]);
}

#[test]
fn queued_pre_close_threshold_does_not_mask_a_later_post_review_incident() {
    let fixture = Fixture::new();
    let mut events = vec![
        fixture.use_event(1, "material_failure", Some("output"), "session-1"),
        fixture.use_event(2, "material_failure", Some("output"), "session-2"),
        fixture.use_event(3, "friction", Some("tool-compatibility"), "session-3"),
        fixture.use_event(4, "friction", Some("tool-compatibility"), "session-4"),
        fixture.use_event(5, "friction", Some("tool-compatibility"), "session-5"),
    ];
    for serial in 6..=10 {
        events.push(fixture.use_event(serial, "clean", None, &format!("session-{serial}")));
    }
    let target = json!({
        "name": "demo-skill",
        "repo_relative_path": fixture.target_relative,
        "content_hash": fixture.target_hash,
        "repo_head": "fixture-head"
    });
    events.push(json!({
        "schema_version": 1,
        "event_id": "evt_review_started",
        "event_type": "review_started",
        "recorded_at": "2026-01-02T11:00:00Z",
        "operator_workflow": "skill-evolution",
        "target": target,
        "top_level_session_id": "review-session",
        "payload": {"review_id": "review-masked"}
    }));
    events.push(json!({
        "schema_version": 1,
        "event_id": "evt_review_disposition",
        "event_type": "review_disposition",
        "recorded_at": "2026-01-02T12:00:00Z",
        "operator_workflow": "skill-evolution",
        "target": target,
        "top_level_session_id": "review-session",
        "payload": {
            "review_id": "review-masked",
            "disposition": "candidate_rejected_validation",
            "adjudicated_event_ids": ["evt_3", "evt_4", "evt_5"]
        }
    }));
    events.push(fixture.use_event(11, "friction", Some("execution"), "session-11"));
    fixture.write_events(&events);

    let status = fixture.derive("session-11", 1_767_398_400_000);
    assert_eq!(
        status.authorization_reason.as_deref(),
        Some("ten_use_unresolved")
    );
    assert_eq!(status.trigger_event_ids, ["evt_11"]);
    assert_eq!(
        status.review_reentry_basis.as_deref(),
        Some("post_review_incident")
    );
}

#[test]
fn derive_does_not_remove_or_bypass_a_foreign_store_lock() {
    let fixture = Fixture::new();
    let evidence = fixture
        .root
        .path()
        .join("reports/skill-evidence/demo-skill");
    let lock = evidence.join(".lock");
    fs::create_dir_all(&lock).expect("create foreign lock");
    fs::write(lock.join("owner"), "foreign-owner").expect("write foreign owner");
    fs::write(evidence.join("gate-status.json"), b"foreign projection\n")
        .expect("write existing projection");

    let error = derive_store(
        fixture.root.path(),
        Path::new(fixture.target_relative),
        &DerivationInputs {
            generated_at: "2026-01-03T00:00:00Z".to_owned(),
            now_epoch_milliseconds: 1_767_398_400_000,
            session_id: "derive-session".to_owned(),
            lock_owner: "derive-owner".to_owned(),
        },
    )
    .expect_err("foreign lock must block derive");

    assert_eq!(error.class(), skill_evidence::ErrorClass::UnsafeFailure);
    assert_eq!(
        fs::read_to_string(lock.join("owner")).expect("foreign owner remains"),
        "foreign-owner"
    );
    assert_eq!(
        fs::read(evidence.join("gate-status.json")).expect("projection remains"),
        b"foreign projection\n"
    );
}

#[test]
fn repeated_same_session_and_task_incidents_are_not_independent() {
    let fixture = Fixture::new();
    let mut events = (1..=3)
        .map(|serial| fixture.use_event(serial, "friction", Some("execution"), "same-session"))
        .collect::<Vec<_>>();
    for event in &mut events {
        event["payload"]["task_fingerprint"] = json!("same-task");
    }
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "collecting");
    assert_eq!(status.candidate_clusters[0].independent_incidents, 1);
}

#[test]
fn retrospective_incident_can_support_but_not_complete_a_threshold() {
    let fixture = Fixture::new();
    let mut retrospective = fixture.use_event(3, "friction", Some("cost"), "retrospective-session");
    retrospective["payload"]["retrospective"] = json!(true);
    retrospective["payload"]["evidence_refs"] = json!(["reports/evidence.txt"]);
    fixture.write_events(&[
        fixture.use_event(1, "friction", Some("cost"), "session-1"),
        fixture.use_event(2, "friction", Some("cost"), "session-2"),
        retrospective,
    ]);
    assert_eq!(
        fixture.derive("fresh-session", 1_767_398_400_000).state,
        "collecting"
    );

    let mut events = fs::read_to_string(
        fixture
            .root
            .path()
            .join("reports/skill-evidence/demo-skill/events.jsonl"),
    )
    .expect("read events")
    .lines()
    .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
    .collect::<Vec<_>>();
    events.push(fixture.use_event(4, "friction", Some("cost"), "session-4"));
    fixture.write_events(&events);
    assert_eq!(
        fixture.derive("session-4", 1_767_398_400_000).state,
        "eligible_pending_cooldown"
    );
}

#[test]
fn ten_clean_uses_authorize_nothing() {
    let fixture = Fixture::new();
    let events = (1..=10)
        .map(|serial| fixture.use_event(serial, "clean", None, &format!("session-{serial}")))
        .collect::<Vec<_>>();
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.qualifying_uses_on_current_hash, 10);
    assert_eq!(status.state, "closed");
    assert_eq!(status.authorized_workflow, None);
}

#[test]
fn target_hash_change_partitions_prospective_evidence() {
    let fixture = Fixture::new();
    fixture.write_events(&[
        fixture.use_event(1, "friction", Some("execution"), "session-1"),
        fixture.use_event(2, "friction", Some("execution"), "session-2"),
    ]);
    fs::write(
        fixture
            .root
            .path()
            .join(fixture.target_relative)
            .join("SKILL.md"),
        "---\nname: demo-skill\n---\nChanged body.\n",
    )
    .expect("change target");

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.qualifying_uses_on_current_hash, 0);
    assert_eq!(status.open_incident_ids, Vec::<String>::new());
    assert_eq!(status.state, "closed");
    assert_ne!(status.target_content_hash, fixture.target_hash);
}

#[test]
fn active_review_owns_the_target() {
    let fixture = Fixture::new();
    let target = json!({
        "name": "demo-skill",
        "repo_relative_path": fixture.target_relative,
        "content_hash": fixture.target_hash,
        "repo_head": "fixture-head"
    });
    fixture.write_events(&[
        fixture.use_event(1, "friction", Some("execution"), "session-1"),
        json!({
            "schema_version": 1,
            "event_id": "evt_review_started",
            "event_type": "review_started",
            "recorded_at": "2026-01-02T02:00:00Z",
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": {"review_id": "active-review"}
        }),
    ]);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "review_in_progress");
    assert_eq!(status.active_review_id.as_deref(), Some("active-review"));
}

#[test]
fn unadjudicated_severe_incident_remains_quarantined_across_later_disposition() {
    let fixture = Fixture::new();
    let target = json!({
        "name": "demo-skill",
        "repo_relative_path": fixture.target_relative,
        "content_hash": fixture.target_hash,
        "repo_head": "fixture-head"
    });
    fixture.write_events(&[
        fixture.use_event(1, "material_failure", Some("output"), "session-1"),
        fixture.use_event(2, "material_failure", Some("output"), "session-2"),
        fixture.use_event(3, "severe_incident", Some("state"), "session-3"),
        json!({
            "schema_version": 1,
            "event_id": "evt_review_started",
            "event_type": "review_started",
            "recorded_at": "2026-01-02T04:00:00Z",
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": {"review_id": "review-severe"}
        }),
        json!({
            "schema_version": 1,
            "event_id": "evt_review_disposition",
            "event_type": "review_disposition",
            "recorded_at": "2026-01-02T05:00:00Z",
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": "review-session",
            "payload": {
                "review_id": "review-severe",
                "disposition": "candidate_rejected_validation",
                "adjudicated_event_ids": ["evt_1", "evt_2"]
            }
        }),
    ]);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "quarantined_eligible");
    assert_eq!(status.authorization_reason.as_deref(), Some("severe"));
    assert_eq!(
        status.review_reentry_basis.as_deref(),
        Some("unadjudicated_severe")
    );
    assert_eq!(status.trigger_event_ids, ["evt_3"]);
}

#[test]
fn optional_incident_text_fields_refuse_non_string_or_empty_values_on_read() {
    for (field, invalid) in [
        ("workaround_taken", json!(false)),
        ("workaround_taken", json!("")),
        ("run_condition", json!(false)),
        ("run_condition", json!("")),
    ] {
        let fixture = Fixture::new();
        let mut event = fixture.use_event(1, "friction", Some("execution"), "session-1");
        event["payload"][field] = invalid;
        fixture.write_events(&[event]);

        let status = fixture.derive("fresh-session", 1_767_398_400_000);
        assert_eq!(status.state, "blocked", "field {field}");
        assert!(
            status
                .integrity_errors
                .iter()
                .any(|error| error.contains(field)),
            "missing {field} integrity error: {:?}",
            status.integrity_errors
        );
    }
}
