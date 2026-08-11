#![forbid(unsafe_code)]

use std::{fs, path::Path};

use serde_json::{Value, json};

#[test]
fn skill_key_for_target_derives_the_canonical_evidence_store_key() {
    let root = tempfile::tempdir().expect("temporary repository");
    let target = root.path().join(".claude/skills/game-evidence");
    fs::create_dir_all(&target).expect("create fixture skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: game-evidence\n---\n# game-evidence\n",
    )
    .expect("write fixture skill");

    assert_eq!(
        skill_evidence::skill_key_for_target(
            root.path(),
            Path::new(".claude/skills/game-evidence")
        )
        .expect("derive public skill key"),
        "game-evidence"
    );
}

#[test]
fn read_validated_event_stream_returns_valid_events_and_integrity_errors() {
    let root = tempfile::tempdir().expect("temporary repository");
    let events_path = root.path().join("events.jsonl");
    let event = json!({
        "schema_version": 1,
        "event_id": "evt_read_contract",
        "event_type": "use_recorded",
        "recorded_at": "2026-07-25T11:59:59.000Z",
        "operator_workflow": "skill-evidence-capture",
        "target": {
            "name": "game-evidence",
            "repo_relative_path": ".claude/skills/game-evidence",
            "content_hash": "fixture-hash",
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "fixture-session",
        "payload": {
            "qualifying_use": true,
            "retrospective": false,
            "task_label": "read contract",
            "task_fingerprint": "read-contract",
            "outcome": "clean",
            "symptom_key": Value::Null,
            "expected": Value::Null,
            "observed": Value::Null,
            "consequence": Value::Null,
            "workaround_taken": Value::Null,
            "evidence_refs": [],
            "same_run_group": "read-contract"
        }
    });
    fs::write(&events_path, format!("{event}\nnot valid JSON\n"))
        .expect("write fixture event stream");

    let stream = skill_evidence::read_validated_event_stream(&events_path)
        .expect("read public validated event stream");
    assert_eq!(stream.events, [event]);
    assert_eq!(stream.integrity_errors.len(), 1);
    assert!(stream.integrity_errors[0].contains("line 2: not valid JSON"));
}

#[test]
fn present_external_owners_must_exactly_match_concluded_outside_target_coverage() {
    let root = tempfile::tempdir().expect("temporary repository");
    let events_path = root.path().join("events.jsonl");
    let event = json!({
        "schema_version": 1,
        "event_id": "evt_malformed_external_owners",
        "event_type": "review_disposition",
        "recorded_at": "2026-07-25T11:59:59.000Z",
        "operator_workflow": "skill-evolution",
        "target": {
            "name": "game-evidence",
            "repo_relative_path": ".claude/skills/game-evidence",
            "content_hash": "fixture-hash",
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "fixture-session",
        "payload": {
            "review_id": "rev_read_contract",
            "disposition": "outside_target",
            "adjudicated_event_ids": ["evt_concluded", "evt_undecidable"],
            "instrument_limited_event_ids": ["evt_undecidable"],
            "external_owners": [{
                "event_id": "evt_undecidable",
                "kind": "skill",
                "reference": ".claude/skills/code-review"
            }],
            "note": "the owner list is attached to the wrong route"
        }
    });
    fs::write(&events_path, format!("{event}\n")).expect("write malformed owner event");

    let stream = skill_evidence::read_validated_event_stream(&events_path)
        .expect("read public validated event stream");

    assert!(stream.events.is_empty());
    assert_eq!(stream.integrity_errors.len(), 1);
    assert!(
        stream.integrity_errors[0].contains("external_owners")
            && stream.integrity_errors[0].contains("concluded coverage"),
        "integrity error must explain the owner-to-conclusion mismatch: {:?}",
        stream.integrity_errors
    );
}

#[test]
fn external_owner_entries_reject_undeclared_fields() {
    let root = tempfile::tempdir().expect("temporary repository");
    let events_path = root.path().join("events.jsonl");
    let event = json!({
        "schema_version": 1,
        "event_id": "evt_external_owner_with_extra_field",
        "event_type": "review_disposition",
        "recorded_at": "2026-07-25T11:59:59.000Z",
        "operator_workflow": "skill-evolution",
        "target": {
            "name": "game-evidence",
            "repo_relative_path": ".claude/skills/game-evidence",
            "content_hash": "fixture-hash",
            "repo_head": "fixture-head"
        },
        "top_level_session_id": "fixture-session",
        "payload": {
            "review_id": "rev_read_contract",
            "disposition": "outside_target",
            "adjudicated_event_ids": ["evt_concluded"],
            "external_owners": [{
                "event_id": "evt_concluded",
                "kind": "skill",
                "reference": ".claude/skills/code-review",
                "route": "concluded"
            }],
            "note": "the owner entry contains an undeclared route key"
        }
    });
    fs::write(&events_path, format!("{event}\n")).expect("write malformed owner event");

    let stream = skill_evidence::read_validated_event_stream(&events_path)
        .expect("read public validated event stream");

    assert!(stream.events.is_empty());
    assert_eq!(stream.integrity_errors.len(), 1);
    assert!(
        stream.integrity_errors[0].contains("external_owners")
            && stream.integrity_errors[0].contains("containing exactly event_id")
            && stream.integrity_errors[0].contains("and reference"),
        "integrity error must explain the closed owner-entry shape: {:?}",
        stream.integrity_errors
    );
}
