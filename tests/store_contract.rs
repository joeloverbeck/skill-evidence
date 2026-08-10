#![forbid(unsafe_code)]

use std::{fs, path::Path};

mod support;

use serde_json::Value;
use skill_evidence::{DerivationInputs, RecordInputs, RecordUseRequest, derive_store, record_use};

#[test]
fn record_library_uses_explicit_clock_uuid_repository_and_session_inputs() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target_relative = ".claude/skills/demo-skill";
    let target = root.path().join(target_relative);
    fs::create_dir_all(&target).expect("create target");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: demo-skill\n---\nDemo.\n",
    )
    .expect("write target");

    let receipt = record_use(
        root.path(),
        Path::new(target_relative),
        &RecordUseRequest {
            outcome: "friction".to_owned(),
            task_label: "Explicit input test".to_owned(),
            symptom_key: Some("output".to_owned()),
            expected: Some("expected".to_owned()),
            observed: Some("observed".to_owned()),
            consequence: Some("consequence".to_owned()),
            workaround: Some("workaround".to_owned()),
            run_condition: Some("29 selectors; caught at pre-review".to_owned()),
            retrospective: false,
            evidence_refs: Vec::new(),
            same_run_group: Some("explicit-run-group".to_owned()),
            further_incident: false,
        },
        &RecordInputs {
            event_id: "evt_explicit".to_owned(),
            recorded_at: "2026-01-02T03:04:05.678Z".to_owned(),
            now_epoch_milliseconds: 1_767_320_645_678,
            repository_head: "explicit-head".to_owned(),
            session_id: "explicit-session".to_owned(),
            lock_owner: "explicit-lock-owner".to_owned(),
        },
        &support::host(),
    )
    .expect("record explicit event");

    assert_eq!(receipt.schema, "skill-evidence.skill-evidence.record.v1");
    assert_eq!(receipt.event_id, "evt_explicit");
    assert_eq!(receipt.gate_status.generated_at, "2026-01-02T03:04:05.678Z");
    assert_eq!(
        receipt.gate_status.derivation_session_id.as_deref(),
        Some("explicit-session")
    );
    let event: Value = serde_json::from_str(
        fs::read_to_string(
            root.path()
                .join("reports/skill-evidence/demo-skill/events.jsonl"),
        )
        .expect("read stream")
        .trim_end(),
    )
    .expect("event JSON");
    assert_eq!(event["event_id"], "evt_explicit");
    assert_eq!(event["recorded_at"], "2026-01-02T03:04:05.678Z");
    assert_eq!(event["target"]["repo_head"], "explicit-head");
    assert_eq!(event["top_level_session_id"], "explicit-session");
    assert_eq!(
        event["payload"]["run_condition"],
        "29 selectors; caught at pre-review"
    );
    assert_eq!(event["payload"]["workaround_taken"], "workaround");
}

#[test]
fn derive_library_refuses_invalid_explicit_inputs_without_creating_a_store() {
    for inputs in [
        DerivationInputs {
            generated_at: "not-a-timestamp".to_owned(),
            now_epoch_milliseconds: 0,
            session_id: "explicit-session".to_owned(),
            lock_owner: "explicit-lock-owner".to_owned(),
        },
        DerivationInputs {
            generated_at: "2026-01-02T03:04:05.678Z".to_owned(),
            now_epoch_milliseconds: 0,
            session_id: String::new(),
            lock_owner: "explicit-lock-owner".to_owned(),
        },
        DerivationInputs {
            generated_at: "2026-01-02T03:04:05.678Z".to_owned(),
            now_epoch_milliseconds: 0,
            session_id: "explicit-session".to_owned(),
            lock_owner: String::new(),
        },
    ] {
        let root = tempfile::tempdir().expect("temporary repository");
        let target_relative = ".claude/skills/demo-skill";
        let target = root.path().join(target_relative);
        fs::create_dir_all(&target).expect("create target");
        fs::write(
            target.join("SKILL.md"),
            "---\nname: demo-skill\n---\nDemo.\n",
        )
        .expect("write target");

        let error = derive_store(root.path(), Path::new(target_relative), &inputs)
            .expect_err("invalid derivation inputs must refuse");
        assert_eq!(error.class(), skill_evidence::ErrorClass::Refusal);
        assert!(
            !root.path().join("reports/skill-evidence").exists(),
            "input refusal must precede evidence-store creation"
        );
    }
}

#[test]
fn duplicate_explicit_event_id_refuses_without_mutating_the_store() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target_relative = ".claude/skills/demo-skill";
    let target = root.path().join(target_relative);
    fs::create_dir_all(&target).expect("create target");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: demo-skill\n---\nDemo.\n",
    )
    .expect("write target");

    let request = |task_label: &str, same_run_group: &str| RecordUseRequest {
        outcome: "clean".to_owned(),
        task_label: task_label.to_owned(),
        symptom_key: None,
        expected: None,
        observed: None,
        consequence: None,
        workaround: None,
        run_condition: None,
        retrospective: false,
        evidence_refs: Vec::new(),
        same_run_group: Some(same_run_group.to_owned()),
        further_incident: false,
    };
    let inputs = |recorded_at: &str| RecordInputs {
        event_id: "evt_duplicate".to_owned(),
        recorded_at: recorded_at.to_owned(),
        now_epoch_milliseconds: 1_767_320_645_678,
        repository_head: "explicit-head".to_owned(),
        session_id: "explicit-session".to_owned(),
        lock_owner: "explicit-lock-owner".to_owned(),
    };

    record_use(
        root.path(),
        Path::new(target_relative),
        &request("First distinct use", "first-run"),
        &inputs("2026-01-02T03:04:05.678Z"),
        &support::host(),
    )
    .expect("record first event");

    let evidence_directory = root.path().join("reports/skill-evidence/demo-skill");
    let events_path = evidence_directory.join("events.jsonl");
    let projection_path = evidence_directory.join("gate-status.json");
    let events_before = fs::read(&events_path).expect("read event stream before refusal");
    let projection_before =
        fs::read(&projection_path).expect("read gate projection before refusal");

    let error = record_use(
        root.path(),
        Path::new(target_relative),
        &request("Second distinct use", "second-run"),
        &inputs("2026-01-02T04:04:05.678Z"),
        &support::host(),
    )
    .expect_err("a duplicate explicit event ID must be refused");

    assert_eq!(error.class(), skill_evidence::ErrorClass::Refusal);
    assert_eq!(
        fs::read(&events_path).expect("read event stream after refusal"),
        events_before
    );
    assert_eq!(
        fs::read(&projection_path).expect("read gate projection after refusal"),
        projection_before
    );
}

#[test]
fn record_library_enforces_self_receipt_policy_without_caller_configuration() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target_relative = ".claude/skills/skill-evidence-capture";
    let target = root.path().join(target_relative);
    fs::create_dir_all(&target).expect("create target");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: skill-evidence-capture\n---\nCapture.\n",
    )
    .expect("write target");

    let error = record_use(
        root.path(),
        Path::new(target_relative),
        &RecordUseRequest {
            outcome: "clean".to_owned(),
            task_label: "Self receipt".to_owned(),
            symptom_key: None,
            expected: None,
            observed: None,
            consequence: None,
            workaround: None,
            run_condition: None,
            retrospective: false,
            evidence_refs: Vec::new(),
            same_run_group: Some("self-receipt".to_owned()),
            further_incident: false,
        },
        &RecordInputs {
            event_id: "evt_self".to_owned(),
            recorded_at: "2026-01-02T03:04:05.678Z".to_owned(),
            now_epoch_milliseconds: 1_767_320_645_678,
            repository_head: "explicit-head".to_owned(),
            session_id: "explicit-session".to_owned(),
            lock_owner: "explicit-lock-owner".to_owned(),
        },
        &support::host(),
    )
    .expect_err("the library must enforce its self-receipt rule");

    assert_eq!(error.class(), skill_evidence::ErrorClass::Refusal);
    assert!(
        !root.path().join("reports/skill-evidence").exists(),
        "self-receipt refusal must precede evidence-store creation"
    );
}
