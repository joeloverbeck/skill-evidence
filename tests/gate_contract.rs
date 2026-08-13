#![forbid(unsafe_code)]
//! What the evidence store on disk adds to a derivation.
//!
//! The clustering, threshold and retirement rules are not here: they are behavioural
//! tests of the gate derivation itself and live beside it in `src/gate.rs`, where they
//! need no repository, no lock and no projection file. What remains here is what only
//! a real store can show: that deriving respects a lock it does not own, that the hash
//! it derives against is the target's own content, and that the acts which touch a
//! consumer's files either complete or leave nothing behind.

use std::{fs, path::Path};

mod support;

use serde_json::{Value, json};
use skill_evidence::{
    DerivationInputs, GateStatus, RecordInputs, RecordUseRequest, derive_store, hash_skill,
    record_use,
};
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

/// The projection is swapped into place atomically, so a failure at the swap leaves
/// the recorded stream exactly as it was and does not half-write the projection.
///
/// `docs/principles/evidence-substrate-integrity.md` puts this at the top of what a
/// side effect on a consumer's files must carry: "An act that half-applies and records
/// nothing is the worst outcome available and must not be reachable." Deriving reads
/// the stream and never writes it, so the check here is that the failure stays inside
/// the projection and the append-only stream is untouched.
#[test]
fn a_failed_projection_swap_leaves_the_recorded_stream_untouched() {
    let fixture = Fixture::new();
    fixture.write_events(&[
        fixture.use_event(1, "friction", Some("execution"), "session-1"),
        fixture.use_event(2, "friction", Some("execution"), "session-2"),
    ]);
    let evidence = fixture
        .root
        .path()
        .join("reports/skill-evidence/demo-skill");
    let events_path = evidence.join("events.jsonl");
    let recorded = fs::read(&events_path).expect("read recorded stream");

    // A non-empty directory where the projection belongs: `rename` cannot replace it.
    let projection = evidence.join("gate-status.json");
    fs::create_dir_all(&projection).expect("stand a directory where the projection goes");
    fs::write(projection.join("occupant"), b"not a projection\n").expect("occupy it");

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
    .expect_err("an unreplaceable projection must fail the derivation");

    assert_eq!(error.class(), skill_evidence::ErrorClass::UnsafeFailure);
    assert!(
        error.to_string().contains("atomically replace"),
        "the failure names the swap it could not complete: {error}"
    );
    assert_eq!(
        fs::read(&events_path).expect("stream survives"),
        recorded,
        "deriving reads the stream and never rewrites it, least of all when it fails"
    );
    assert_eq!(
        fs::read_to_string(projection.join("occupant")).expect("occupant survives"),
        "not a projection\n",
        "nothing was half-written over what stood where the projection goes"
    );
}

/// A failed append records nothing: no event, no projection, and no temporary left to
/// be mistaken for one.
///
/// `docs/principles/evidence-substrate-integrity.md` names this the worst outcome
/// available — "An act that half-applies and records nothing … must not be reachable."
/// `record_use` stages the projection before it appends precisely so the append can
/// still fail safely, and this is the test of that ordering.
///
/// A dangling symlink is what makes the failure reachable without a fault-injection
/// seam: reading it raises `NotFound`, which the reader absorbs as an empty stream, so
/// the run proceeds all the way to the append and only then cannot open the path.
#[cfg(unix)]
#[test]
fn a_failed_append_leaves_no_event_and_no_projection() {
    let fixture = Fixture::new();
    let evidence = fixture
        .root
        .path()
        .join("reports/skill-evidence/demo-skill");
    fs::create_dir_all(&evidence).expect("create evidence directory");
    std::os::unix::fs::symlink("./absent/events.jsonl", evidence.join("events.jsonl"))
        .expect("dangle the stream at a path that cannot be opened");

    let error = record_use(
        fixture.root.path(),
        Path::new(fixture.target_relative),
        &RecordUseRequest {
            outcome: "friction".to_owned(),
            task_label: "Append must fail safely".to_owned(),
            symptom_key: Some("execution".to_owned()),
            expected: Some("expected".to_owned()),
            observed: Some("observed".to_owned()),
            consequence: Some("consequence".to_owned()),
            workaround: None,
            run_condition: Some("first append against an empty store".to_owned()),
            retrospective: false,
            evidence_refs: Vec::new(),
            same_run_group: None,
            further_incident: false,
        },
        &RecordInputs {
            event_id: "evt_unappendable".to_owned(),
            recorded_at: "2026-01-02T03:04:05.678Z".to_owned(),
            now_epoch_milliseconds: 1_767_320_645_678,
            repository_head: "head".to_owned(),
            session_id: "session".to_owned(),
            lock_owner: "lock-owner".to_owned(),
        },
        &support::host(),
    )
    .expect_err("an unappendable stream must fail the record");

    assert_eq!(error.class(), skill_evidence::ErrorClass::UnsafeFailure);
    assert!(
        !evidence.join(".gate-status.json.tmp").exists(),
        "the staged projection is discarded when the append it was staged for fails"
    );
    assert!(
        !evidence.join("gate-status.json").exists(),
        "no projection is published for an event that was never recorded"
    );
    assert!(
        !evidence.join("absent").exists(),
        "nothing was created behind the dangling stream"
    );
}
