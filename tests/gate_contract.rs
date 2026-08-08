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

    fn target(&self) -> Value {
        json!({
            "name": "demo-skill",
            "repo_relative_path": self.target_relative,
            "content_hash": self.target_hash,
            "repo_head": "fixture-head"
        })
    }

    /// The claim and close a completed review leaves behind. Every test that needs the
    /// pair differs only in the review id, the disposition, and the coverage.
    fn review(&self, review_id: &str, disposition: &str, covered: &[&str]) -> [Value; 2] {
        [
            json!({
                "schema_version": 1,
                "event_id": format!("evt_review_started_{review_id}"),
                "event_type": "review_started",
                "recorded_at": "2026-01-02T20:00:00Z",
                "operator_workflow": "skill-evolution",
                "target": self.target(),
                "top_level_session_id": format!("review-session-{review_id}"),
                "payload": {"review_id": review_id}
            }),
            json!({
                "schema_version": 1,
                "event_id": format!("evt_review_disposition_{review_id}"),
                "event_type": "review_disposition",
                "recorded_at": "2026-01-02T21:00:00Z",
                "operator_workflow": "skill-evolution",
                "target": self.target(),
                "top_level_session_id": format!("review-session-{review_id}"),
                "payload": {
                    "review_id": review_id,
                    "disposition": disposition,
                    "adjudicated_event_ids": covered
                }
            }),
        ]
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
    let target = fixture.target();
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
    events.extend(fixture.review(
        "review-queued",
        "candidate_rejected_validation",
        &["evt_2", "evt_3"],
    ));
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

/// `blocked_no_valid_test` reaches no conclusion about the skill, so it adjudicates
/// nothing — but it does establish that this instrument cannot test the evidence it
/// covered. That evidence stops driving the gate. It stays an open incident, because
/// nothing was decided about it; it simply no longer clusters, so it can never again
/// reach a threshold the review already proved untestable.
#[test]
fn blocked_no_valid_test_retires_covered_incidents_from_the_gate() {
    let fixture = Fixture::new();
    let mut events = (1..=3)
        .map(|serial| {
            fixture.use_event(
                serial,
                "friction",
                Some("execution"),
                &format!("session-{serial}"),
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture.review(
        "review-blocked",
        "blocked_no_valid_test",
        &["evt_1", "evt_2", "evt_3"],
    ));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "collecting");
    assert_eq!(status.authorization_reason, None);
    assert_eq!(
        status.open_incident_ids,
        ["evt_1", "evt_2", "evt_3"],
        "a blocked close adjudicates nothing, so the incidents remain open in the ledger"
    );
    assert!(
        status.candidate_clusters.is_empty(),
        "evidence a review proved untestable must stop clustering, or one new incident \
         re-fires the same threshold the instrument already failed: {:?}",
        status.candidate_clusters
    );
    assert_eq!(
        status.review_reentry_basis, None,
        "nothing is queued behind the close: the covered evidence left the gate"
    );
}

/// The trigger list is frozen when the threshold fires, but incidents keep arriving while
/// the review runs — issue #1's own `grilling` cluster had "a fourth in the same cluster
/// open too". A straggler the close did not list has the same symptom and the same binding
/// constraint the instrument could not vary, so leaving it clustered lowers the bar for
/// the next review instead of resetting it: two new incidents would re-authorize a
/// threshold that takes three.
#[test]
fn a_blocked_close_retires_the_stragglers_in_the_clusters_it_covered() {
    let fixture = Fixture::new();
    // Three fire the threshold; the fourth lands while the review is still open, so it is
    // never in the frozen trigger list.
    let mut events = (1..=4)
        .map(|serial| {
            fixture.use_event(
                serial,
                "friction",
                Some("execution"),
                &format!("session-{serial}"),
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture.review(
        "review-blocked",
        "blocked_no_valid_test",
        &["evt_1", "evt_2", "evt_3"],
    ));
    // Two genuinely new incidents. Three are needed for friction_recurrence, so these
    // must not be enough on their own.
    events.extend((5..=6).map(|serial| {
        fixture.use_event(
            serial,
            "friction",
            Some("execution"),
            &format!("session-{serial}"),
        )
    }));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(
        status.instrument_limited_incident_ids,
        ["evt_1", "evt_2", "evt_3", "evt_4"],
        "the straggler shares the symptom the instrument could not test"
    );
    assert_eq!(
        status.authorization_reason, None,
        "two new incidents must not re-authorize a threshold that takes three: {:?}",
        status.candidate_clusters
    );
    assert_eq!(status.state, "collecting");
}

/// A `material_recurrence` trigger list holds only the material incidents, so a cluster's
/// merely-frictional siblings are never in it whatever the timing. Left clustered they
/// would discount every future review of that symptom, not just the next one.
#[test]
fn a_blocked_close_retires_frictional_siblings_a_material_trigger_list_cannot_name() {
    let fixture = Fixture::new();
    let mut events = vec![
        fixture.use_event(1, "friction", Some("output"), "session-1"),
        fixture.use_event(2, "material_failure", Some("output"), "session-2"),
        fixture.use_event(3, "material_failure", Some("output"), "session-3"),
    ];
    events.extend(fixture.review(
        "review-blocked",
        "blocked_no_valid_test",
        &["evt_2", "evt_3"],
    ));
    events.extend((4..=5).map(|serial| {
        fixture.use_event(
            serial,
            "friction",
            Some("output"),
            &format!("session-{serial}"),
        )
    }));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(
        status.instrument_limited_incident_ids,
        ["evt_1", "evt_2", "evt_3"],
        "the frictional sibling shares the retired symptom"
    );
    assert_eq!(status.authorization_reason, None);
    assert_eq!(status.state, "collecting");
}

/// Evidence recorded after an instrument-limited close is new evidence, whatever its
/// symptom. Retirement that kept reaching forward would silence the symptom permanently —
/// the gate would never speak about it again, which is a worse failure than the trap.
#[test]
fn a_blocked_close_does_not_retire_evidence_recorded_after_it() {
    let fixture = Fixture::new();
    let mut events = (1..=3)
        .map(|serial| {
            fixture.use_event(
                serial,
                "friction",
                Some("execution"),
                &format!("session-{serial}"),
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture.review(
        "review-blocked",
        "blocked_no_valid_test",
        &["evt_1", "evt_2", "evt_3"],
    ));
    events.extend((4..=6).map(|serial| {
        fixture.use_event(
            serial,
            "friction",
            Some("execution"),
            &format!("session-{serial}"),
        )
    }));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(
        status.instrument_limited_incident_ids,
        ["evt_1", "evt_2", "evt_3"],
        "only evidence that existed when the review closed is retired"
    );
    assert_eq!(
        status.authorization_reason.as_deref(),
        Some("friction_recurrence:execution"),
        "three genuinely new incidents reach the threshold on their own"
    );
    assert_eq!(status.trigger_event_ids, ["evt_4", "evt_5", "evt_6"]);
}

/// Retirement and the watermark can come from different closes. Here a blocked close
/// retires one cluster, and a later adjudicating close lays the watermark over another —
/// so the projection reports `queued_pre_close_evidence` (that close did reach a
/// conclusion) while still carrying retired evidence the adjudicating close never touched.
#[test]
fn retirement_and_the_watermark_can_come_from_different_closes() {
    let fixture = Fixture::new();
    let target = fixture.target();
    let review = |serial: usize, review_id: &str, event_type: &str, payload: Value| {
        json!({
            "schema_version": 1,
            "event_id": format!("evt_{review_id}_{event_type}"),
            "event_type": event_type,
            "recorded_at": format!("2026-01-02T{serial:02}:30:00Z"),
            "operator_workflow": "skill-evolution",
            "target": target,
            "top_level_session_id": format!("review-session-{review_id}"),
            "payload": payload
        })
    };
    let mut events = (1..=3)
        .map(|serial| {
            fixture.use_event(
                serial,
                "friction",
                Some("execution"),
                &format!("session-{serial}"),
            )
        })
        .collect::<Vec<_>>();
    events.push(review(
        3,
        "blocked",
        "review_started",
        json!({"review_id": "rev-blocked"}),
    ));
    events.push(review(
        4,
        "blocked",
        "review_disposition",
        json!({
            "review_id": "rev-blocked",
            "disposition": "blocked_no_valid_test",
            "adjudicated_event_ids": ["evt_1", "evt_2", "evt_3"]
        }),
    ));
    // A second cluster that reaches its threshold and is never reviewed — this is what
    // ends up queued behind the watermark.
    events.extend((5..=7).map(|serial| {
        fixture.use_event(
            serial,
            "friction",
            Some("output"),
            &format!("session-{serial}"),
        )
    }));
    // A third, reviewed to a real conclusion. Its close lays the watermark.
    events.extend((8..=9).map(|serial| {
        fixture.use_event(
            serial,
            "material_failure",
            Some("state"),
            &format!("session-{serial}"),
        )
    }));
    events.push(review(
        10,
        "closed",
        "review_started",
        json!({"review_id": "rev-closed"}),
    ));
    events.push(review(
        11,
        "closed",
        "review_disposition",
        json!({
            "review_id": "rev-closed",
            "disposition": "monitor_for_recurrence",
            "adjudicated_event_ids": ["evt_8", "evt_9"]
        }),
    ));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(
        status.instrument_limited_incident_ids,
        ["evt_1", "evt_2", "evt_3"],
        "the adjudicating close retired nothing; the earlier blocked close did"
    );
    assert_eq!(
        status.review_reentry_basis.as_deref(),
        Some("queued_pre_close_evidence"),
        "the watermark close reached a conclusion, so its own label is the honest one"
    );
}

/// The severe carve-out is justified by severe incidents authorizing on their own, ahead
/// of any watermark — but a *retrospective* one never fires that trigger, while still
/// counting toward a cluster. Carving it out therefore protects nothing and leaves it
/// discounting the next review, so the carve-out has to key on the property that actually
/// justifies it.
#[test]
fn a_blocked_close_retires_a_retrospective_severe_incident_it_covered() {
    let fixture = Fixture::new();
    let mut retrospective = fixture.use_event(1, "severe_incident", Some("execution"), "session-1");
    retrospective["payload"]["retrospective"] = json!(true);
    retrospective["payload"]["evidence_refs"] = json!(["logs/retrospective-severe.txt"]);
    let mut events = vec![retrospective];
    events.extend((2..=3).map(|serial| {
        fixture.use_event(
            serial,
            "friction",
            Some("execution"),
            &format!("session-{serial}"),
        )
    }));
    events.extend(fixture.review(
        "review-blocked",
        "blocked_no_valid_test",
        &["evt_1", "evt_2", "evt_3"],
    ));
    events.extend((4..=5).map(|serial| {
        fixture.use_event(
            serial,
            "friction",
            Some("execution"),
            &format!("session-{serial}"),
        )
    }));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(
        status.instrument_limited_incident_ids,
        ["evt_1", "evt_2", "evt_3"],
        "a retrospective severe incident authorizes nothing, so nothing is protected by \
         leaving it clustered"
    );
    assert_eq!(
        status.authorization_reason, None,
        "two new incidents must not re-authorize a threshold that takes three: {:?}",
        status.candidate_clusters
    );
}

/// Retiring evidence from the gate is the honest exit; retiring it silently is not.
/// A reader of the projection alone must be able to see that real incidents stopped
/// driving this gate, and which ones.
#[test]
fn the_projection_names_the_evidence_a_blocked_close_retired() {
    let fixture = Fixture::new();
    let mut events = (1..=3)
        .map(|serial| {
            fixture.use_event(
                serial,
                "friction",
                Some("execution"),
                &format!("session-{serial}"),
            )
        })
        .collect::<Vec<_>>();
    events.extend(fixture.review(
        "review-blocked",
        "blocked_no_valid_test",
        &["evt_1", "evt_2", "evt_3"],
    ));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(
        status.instrument_limited_incident_ids,
        ["evt_1", "evt_2", "evt_3"]
    );
}

/// The exit exists to undo a deferral, and a severe incident was never deferred: it
/// authorizes on its own, ahead of the watermark. Retiring one would leave the projection
/// claiming the incident stopped driving the gate while it still authorizes the review.
#[test]
fn a_blocked_close_does_not_quiet_a_severe_incident() {
    let fixture = Fixture::new();
    let mut events = vec![fixture.use_event(1, "severe_incident", Some("execution"), "session-1")];
    events.extend(fixture.review("review-blocked-severe", "blocked_no_valid_test", &["evt_1"]));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "quarantined_eligible");
    assert_eq!(status.authorization_reason.as_deref(), Some("severe"));
    assert_eq!(
        status.review_reentry_basis.as_deref(),
        Some("unadjudicated_severe")
    );
    assert!(
        status.instrument_limited_incident_ids.is_empty(),
        "a severe incident is not retired by an instrument-limited close: {:?}",
        status.instrument_limited_incident_ids
    );
    assert_eq!(status.candidate_clusters.len(), 1);
}

/// A review claimed on one cluster leaves the others accumulating. When it closes
/// having reached no conclusion, evidence it never covered is still deferred behind
/// it — but calling that `queued_pre_close_evidence` reports it as accounted for by a
/// review that accounted for nothing. The ledger says inconclusive; so must the
/// projection.
#[test]
fn evidence_behind_an_instrument_limited_close_is_not_reported_as_accounted_for() {
    let fixture = Fixture::new();
    let mut events = (1..=3)
        .map(|serial| {
            fixture.use_event(
                serial,
                "friction",
                Some("execution"),
                &format!("session-{serial}"),
            )
        })
        .collect::<Vec<_>>();
    events.extend((4..=6).map(|serial| {
        fixture.use_event(
            serial,
            "friction",
            Some("output"),
            &format!("session-{serial}"),
        )
    }));
    events.extend(fixture.review(
        "review-blocked",
        "blocked_no_valid_test",
        &["evt_1", "evt_2", "evt_3"],
    ));
    fixture.write_events(&events);

    let status = fixture.derive("fresh-session", 1_767_398_400_000);
    assert_eq!(status.state, "collecting");
    assert_eq!(status.authorization_reason, None);
    assert_eq!(
        status.review_reentry_basis.as_deref(),
        Some("queued_behind_instrument_limited_review")
    );
    assert_eq!(
        status.instrument_limited_incident_ids,
        ["evt_1", "evt_2", "evt_3"],
        "only the covered cluster left the gate"
    );
}

/// `superseded_by_target_version` reaches no conclusion either, but it is not
/// instrument-limited: nothing was established about whether the evidence can be
/// tested, only that the target moved underneath the review. Its evidence keeps
/// driving the gate exactly as before.
///
/// The blocked-close half of this contract is
/// [`blocked_no_valid_test_retires_covered_incidents_from_the_gate`] and
/// [`a_blocked_close_does_not_quiet_a_severe_incident`].
#[test]
fn superseded_by_target_version_does_not_retire_covered_incidents() {
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
            name: "superseded review queues retained pre-close evidence",
            disposition: "superseded_by_target_version",
            outcomes: &["friction", "friction", "friction"],
            expected_state: "collecting",
            expected_reason: None,
            expected_reentry_basis: "queued_pre_close_evidence",
        },
        Case {
            name: "superseded severe review remains quarantined",
            disposition: "superseded_by_target_version",
            outcomes: &["severe_incident"],
            expected_state: "quarantined_eligible",
            expected_reason: Some("severe"),
            expected_reentry_basis: "unadjudicated_severe",
        },
    ] {
        let fixture = Fixture::new();
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
        events.extend(fixture.review(
            "review-non-adjudicating",
            case.disposition,
            &covered_ids.iter().map(String::as_str).collect::<Vec<_>>(),
        ));
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
    events.extend(fixture.review(
        "review-post",
        "candidate_rejected_validation",
        &["evt_2", "evt_3", "evt_4"],
    ));
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
    events.extend(fixture.review(
        "review-masked",
        "candidate_rejected_validation",
        &["evt_3", "evt_4", "evt_5"],
    ));
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
    let target = fixture.target();
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
    let target = fixture.target();
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
