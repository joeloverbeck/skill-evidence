#![forbid(unsafe_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use skill_evidence::DerivationInputs;

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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
