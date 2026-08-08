#![forbid(unsafe_code)]

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;
use skill_evidence::DerivationInputs;

/// Every event type this crate has ever written, including the two whose writer
/// — Legacy Skill Decontamination — is retired. A reader is not retired when its
/// writer is: completed runs sit in consumer history, and that history is
/// immutable.
const V1_EVENT_TYPES: &[&str] = &[
    "use_recorded",
    "review_started",
    "validation_completed",
    "change_landed",
    "review_disposition",
    "decontamination_started",
    "decontamination_completed",
];

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every frozen store under `fixtures/skill-evidence/`, sorted so a failure names
/// the same path on every machine.
fn frozen_stores() -> Vec<PathBuf> {
    let mut stores = Vec::new();
    let corpora = repository_root().join("fixtures/skill-evidence");
    for corpus in fs::read_dir(&corpora).expect("read frozen corpora directory") {
        let corpus = corpus.expect("read corpus entry").path();
        let reports = corpus.join("reports/skill-evidence");
        let Ok(entries) = fs::read_dir(&reports) else {
            continue;
        };
        for entry in entries {
            let store = entry.expect("read store entry").path();
            if store.join("events.jsonl").is_file() {
                stores.push(store);
            }
        }
    }
    stores.sort();
    assert!(
        !stores.is_empty(),
        "the frozen corpora are the only standing proof that a schema change has not \
         invalidated consumer history; finding none means the fixtures moved or were deleted"
    );
    stores
}

fn compile(schema_file: &str) -> jsonschema::Validator {
    let schema: Value = serde_json::from_slice(
        &fs::read(
            repository_root()
                .join("schemas/skill-evidence")
                .join(schema_file),
        )
        .unwrap_or_else(|error| panic!("read {schema_file}: {error}")),
    )
    .unwrap_or_else(|error| panic!("parse {schema_file}: {error}"));
    jsonschema::validator_for(&schema)
        .unwrap_or_else(|error| panic!("compile {schema_file}: {error}"))
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create copied fixture directory");
    for entry in fs::read_dir(source).expect("read fixture directory") {
        let entry = entry.expect("read fixture entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().expect("fixture entry type").is_dir() {
            copy_directory(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("copy fixture file");
        }
    }
}

#[test]
fn rust_derives_the_exact_javascript_lifecycle_fixture_projection_without_rewriting_events() {
    let source = repository_root().join("fixtures/skill-evidence/lifecycle-v1");
    let temporary = tempfile::tempdir().expect("temporary fixture repository");
    copy_directory(&source, temporary.path());
    let store = temporary.path().join("reports/skill-evidence/demo-skill");
    let events_path = store.join("events.jsonl");
    let projection_path = store.join("gate-status.json");
    let original_events = fs::read(&events_path).expect("read JavaScript event stream");
    let expected_projection = fs::read(&projection_path).expect("read JavaScript gate projection");

    let status = skill_evidence::derive_store(
        temporary.path(),
        Path::new(".claude/skills/demo-skill"),
        &DerivationInputs {
            generated_at: "2026-07-31T09:36:33.724Z".to_owned(),
            now_epoch_milliseconds: 1_800_000_000_000,
            session_id: "fixture-decontamination-session".to_owned(),
            lock_owner: "lock_rust_fixture_derivation".to_owned(),
        },
    )
    .expect("derive JavaScript fixture through Rust");

    assert_eq!(status.state, "closed");
    assert_eq!(
        status.last_completed_review_id.as_deref(),
        Some("dec_13763440-083e-4d19-87e5-51a2f1f743dc")
    );
    assert_eq!(
        fs::read(&projection_path).expect("read Rust gate projection"),
        expected_projection
    );
    assert_eq!(
        fs::read(&events_path).expect("reread event stream"),
        original_events
    );
}

/// The forward-only rule, mechanically.
///
/// `derive_store` above proves the frozen streams still *project*. It does not
/// prove they still *validate*: a schema edit that makes a field required, or
/// narrows an enum, leaves derivation working and silently invalidates the
/// history sitting in three consumer repositories. Nothing recovers from that,
/// because the data is already written and pinning an older version rolls back
/// the reader, not the events.
#[test]
fn every_frozen_event_still_validates_against_the_published_schema() {
    let validator = compile("event.v1.schema.json");
    for store in frozen_stores() {
        let stream =
            fs::read_to_string(store.join("events.jsonl")).expect("read frozen event stream");
        for (index, line) in stream.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let event: Value = serde_json::from_str(line).expect("frozen event JSON");
            assert!(
                validator.is_valid(&event),
                "{}:{} no longer validates against the published v1 event schema. \
                 This is recorded history, so the schema is wrong, not the fixture: {}",
                store.display(),
                index + 1,
                line
            );
        }
    }
}

// There is deliberately no companion test validating the frozen `gate-status.json`
// files against the gate-status schema, and adding one is a mistake that has
// already been made once.
//
// Two reasons. A projection is derived and regenerable, so it is not the surface
// the forward-only rule protects — `events.jsonl` is. And the projections in
// `status-reporters-v1` are the literal bytes `{"stale":"projection"}`: degenerate
// on purpose, because their job is to prove the status reporters re-derive instead
// of trusting whatever is on disk. They are supposed to fail validation.
//
// Freshly derived projections *are* schema-checked, in `skill_lifecycle_cli.rs`
// and `skill_evidence_cli.rs`, and the test above holds the derived bytes against
// the frozen expected projection.

/// Guards the guard.
///
/// Without this, the cheapest way to make the validation test above pass is to
/// delete the events that stopped validating. Coverage of all seven v1 event
/// types is what makes the corpora evidence rather than examples.
#[test]
fn the_frozen_corpora_still_cover_every_v1_event_type() {
    let mut observed = BTreeSet::new();
    for store in frozen_stores() {
        let stream =
            fs::read_to_string(store.join("events.jsonl")).expect("read frozen event stream");
        for line in stream.lines().filter(|line| !line.trim().is_empty()) {
            let event: Value = serde_json::from_str(line).expect("frozen event JSON");
            if let Some(event_type) = event["event_type"].as_str() {
                observed.insert(event_type.to_owned());
            }
        }
    }

    let missing: Vec<&&str> = V1_EVENT_TYPES
        .iter()
        .filter(|expected| !observed.contains(**expected))
        .collect();
    assert!(
        missing.is_empty(),
        "the frozen corpora no longer cover {missing:?}; a fixture was edited or deleted, \
         which is the one repair the consumer contract forbids"
    );
}
