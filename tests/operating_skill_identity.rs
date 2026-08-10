#![forbid(unsafe_code)]

use std::{fs, path::Path};

use clap::{Parser, Subcommand};
use serde_json::Value;
use skill_evidence::{
    DerivationInputs, Host, RecordInputs, RecordUseRequest,
    cli::{self, Exit},
};

#[derive(Debug, Parser)]
#[command(name = "test-host")]
struct TestCli {
    #[command(subcommand)]
    command: TestCommand,
}

#[derive(Debug, Subcommand)]
enum TestCommand {
    Skills(cli::SkillsArgs),
}

fn operator_host(operator_root: &Path) -> Host {
    Host {
        namespace: "test-host".to_owned(),
        command: "test-host".to_owned(),
        cargo_package: "test-host".to_owned(),
        skills_directory: operator_root.join(".claude/skills"),
    }
}

fn write_operator_package(operator_root: &Path) {
    let package = operator_root.join(".claude/skills/skill-evolution");
    fs::create_dir_all(&package).expect("create operating skill package");
    fs::write(
        package.join("SKILL.md"),
        "---\nname: skill-evolution\n---\nReview instructions.\n",
    )
    .expect("write operating skill package");
}

fn authorized_consumer(host: &Host) -> tempfile::TempDir {
    let consumer = tempfile::tempdir().expect("temporary consumer repository");
    let target = consumer.path().join(".claude/skills/demo-skill");
    fs::create_dir_all(&target).expect("create target skill");
    fs::write(
        target.join("SKILL.md"),
        "---\nname: demo-skill\n---\nDemo body.\n",
    )
    .expect("write target skill");

    record_authorizing_incidents(consumer.path(), host, 1..=3, "execution");
    consumer
}

fn record_authorizing_incidents(
    root: &Path,
    host: &Host,
    serials: std::ops::RangeInclusive<i64>,
    symptom_key: &str,
) {
    for serial in serials {
        skill_evidence::record_use(
            root,
            Path::new(".claude/skills/demo-skill"),
            &RecordUseRequest {
                outcome: "friction".to_owned(),
                task_label: format!("independent task {serial}"),
                symptom_key: Some(symptom_key.to_owned()),
                expected: Some("expected".to_owned()),
                observed: Some("observed".to_owned()),
                consequence: Some("consequence".to_owned()),
                workaround: None,
                run_condition: Some("fixture condition".to_owned()),
                retrospective: false,
                evidence_refs: vec!["logs/fixture.txt".to_owned()],
                same_run_group: None,
                further_incident: false,
            },
            &RecordInputs {
                event_id: format!("evt_incident_{serial}"),
                recorded_at: format!("2026-01-02T{serial:02}:00:00Z"),
                now_epoch_milliseconds: 1_767_323_045_000 + serial,
                repository_head: "fixture-head".to_owned(),
                session_id: format!("session-{serial}"),
                lock_owner: format!("lock-incident-{serial}"),
            },
            host,
        )
        .expect("record authorizing incident");
    }
}

fn claim(root: &Path, host: &Host, suffix: &str, record_operating_hash: bool) -> (Value, String) {
    let mut arguments = vec![
        "test-host".to_owned(),
        "skills".to_owned(),
        "evolution".to_owned(),
        "claim".to_owned(),
        "--root".to_owned(),
        root.to_str().expect("UTF-8 fixture root").to_owned(),
        "--target".to_owned(),
        ".claude/skills/demo-skill".to_owned(),
        "--review-id".to_owned(),
        suffix.to_owned(),
        "--risk-tier".to_owned(),
        "provisional".to_owned(),
        "--event-id".to_owned(),
        format!("evt_claim_{suffix}"),
        "--recorded-at".to_owned(),
        "2026-01-02T04:00:00Z".to_owned(),
        "--now-epoch-milliseconds".to_owned(),
        "1767323045000".to_owned(),
        "--repository-head".to_owned(),
        "fixture-head".to_owned(),
        "--session-id".to_owned(),
        format!("review-session-{suffix}"),
        "--lock-owner".to_owned(),
        format!("lock-claim-{suffix}"),
    ];
    if record_operating_hash {
        arguments.push("--record-operating-skill-hash".to_owned());
    }
    let parsed = TestCli::try_parse_from(arguments).expect("parse mounted Skill Evolution claim");
    let TestCommand::Skills(args) = parsed.command;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let exit = cli::run(args, host, &mut stdout, &mut stderr);
    assert_eq!(exit, Exit::Success, "{}", String::from_utf8_lossy(&stderr));

    let stream_path = root.join("reports/skill-evidence/demo-skill/events.jsonl");
    let stream = fs::read_to_string(&stream_path).expect("read event stream");
    let line = stream
        .lines()
        .last()
        .expect("review_started line")
        .to_owned();
    let event = serde_json::from_str(&line).expect("review_started JSON");
    let validated = skill_evidence::read_validated_event_stream(&stream_path)
        .expect("round-trip event stream through the crate reader");
    assert!(validated.integrity_errors.is_empty());
    assert_eq!(validated.events.last(), Some(&event));
    (event, line)
}

fn close(root: &Path, host: &Host, review_id: &str, serial: i64) {
    let report = root.join(format!(
        "reports/skill-evidence/demo-skill/reviews/{review_id}.md"
    ));
    fs::create_dir_all(report.parent().expect("review directory"))
        .expect("create review directory");
    fs::write(&report, "# Review report\n").expect("write review report");
    let parsed = TestCli::try_parse_from([
        "test-host".to_owned(),
        "skills".to_owned(),
        "evolution".to_owned(),
        "close".to_owned(),
        "--root".to_owned(),
        root.to_str().expect("UTF-8 fixture root").to_owned(),
        "--target".to_owned(),
        ".claude/skills/demo-skill".to_owned(),
        "--review-id".to_owned(),
        review_id.to_owned(),
        "--disposition".to_owned(),
        "monitor_for_recurrence".to_owned(),
        "--note".to_owned(),
        "fixture review complete".to_owned(),
        "--event-id".to_owned(),
        format!("evt_close_{review_id}"),
        "--recorded-at".to_owned(),
        format!("2026-01-02T{serial:02}:30:00Z"),
        "--now-epoch-milliseconds".to_owned(),
        (1_767_323_045_000_i64 + serial).to_string(),
        "--repository-head".to_owned(),
        "fixture-head".to_owned(),
        "--session-id".to_owned(),
        format!("review-session-{review_id}"),
        "--lock-owner".to_owned(),
        format!("lock-close-{review_id}"),
    ])
    .expect("parse mounted Skill Evolution close");
    let TestCommand::Skills(args) = parsed.command;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let exit = cli::run(args, host, &mut stdout, &mut stderr);
    assert_eq!(exit, Exit::Success, "{}", String::from_utf8_lossy(&stderr));
}

fn preflight(root: &Path, host: &Host) -> Value {
    let parsed = TestCli::try_parse_from([
        "test-host",
        "skills",
        "evolution",
        "preflight",
        "--root",
        root.to_str().expect("UTF-8 fixture root"),
        "--target",
        ".claude/skills/demo-skill",
        "--recorded-at",
        "2026-01-02T13:00:00Z",
        "--now-epoch-milliseconds",
        "1767323045000",
        "--session-id",
        "preflight-session",
        "--lock-owner",
        "lock-preflight",
    ])
    .expect("parse mounted Skill Evolution preflight");
    let TestCommand::Skills(args) = parsed.command;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let exit = cli::run(args, host, &mut stdout, &mut stderr);
    assert_eq!(exit, Exit::Success, "{}", String::from_utf8_lossy(&stderr));
    serde_json::from_slice(&stdout).expect("preflight receipt JSON")
}

#[test]
fn opt_in_claim_records_a_stable_content_sensitive_operating_skill_hash() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());
    let operator_target = Path::new(".claude/skills/skill-evolution");
    let expected_before = skill_evidence::hash_skill(operator.path(), operator_target, &host)
        .expect("hash operating package")
        .content_hash;

    let first = authorized_consumer(&host);
    let second = authorized_consumer(&host);
    let (first_event, first_line) = claim(first.path(), &host, "review-first", true);
    let (second_event, _) = claim(second.path(), &host, "review-second", true);

    assert_eq!(
        first_event["payload"]["operating_skill_hash"],
        expected_before
    );
    assert_eq!(
        second_event["payload"]["operating_skill_hash"], expected_before,
        "unchanged operating-package bytes must yield the same identity"
    );
    assert!(
        first_line.ends_with(&format!(
            ",\"operating_skill_hash\":\"{expected_before}\"}}}}"
        )),
        "the optional key must be appended deterministically: {first_line}"
    );

    fs::write(
        operator
            .path()
            .join(".claude/skills/skill-evolution/references.md"),
        "Additional operating rule.\n",
    )
    .expect("edit operating package");
    let expected_after = skill_evidence::hash_skill(operator.path(), operator_target, &host)
        .expect("rehash edited operating package")
        .content_hash;
    let changed = authorized_consumer(&host);
    let (changed_event, _) = claim(changed.path(), &host, "review-changed", true);

    assert_ne!(expected_after, expected_before);
    assert_eq!(
        changed_event["payload"]["operating_skill_hash"], expected_after,
        "editing any operating-package file must change the recorded identity"
    );
}

#[test]
fn prior_reviews_surface_recorded_operating_hashes_and_omit_unknown_ones() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());
    let expected = skill_evidence::hash_skill(
        operator.path(),
        Path::new(".claude/skills/skill-evolution"),
        &host,
    )
    .expect("hash operating package")
    .content_hash;
    let consumer = authorized_consumer(&host);

    claim(consumer.path(), &host, "review-recorded", true);
    close(consumer.path(), &host, "review-recorded", 5);
    record_authorizing_incidents(consumer.path(), &host, 6..=8, "output");
    claim(consumer.path(), &host, "review-unknown", false);
    close(consumer.path(), &host, "review-unknown", 9);
    record_authorizing_incidents(consumer.path(), &host, 10..=12, "state");

    let receipt = preflight(consumer.path(), &host);
    let prior = receipt["evidence_packet"]["prior_reviews"]
        .as_array()
        .expect("prior reviews");
    let recorded = prior
        .iter()
        .find(|review| review["review_id"] == "review-recorded")
        .expect("recorded-identity predecessor");
    let unknown = prior
        .iter()
        .find(|review| review["review_id"] == "review-unknown")
        .expect("unknown-identity predecessor");

    assert_eq!(recorded["operating_skill_hash"], expected);
    assert!(
        unknown
            .as_object()
            .expect("prior review object")
            .get("operating_skill_hash")
            .is_none(),
        "absence means the predecessor did not record an operating identity"
    );
}

#[test]
fn reader_blocks_a_malformed_present_operating_skill_hash() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());
    let consumer = authorized_consumer(&host);
    claim(consumer.path(), &host, "review-malformed", true);
    let stream_path = consumer
        .path()
        .join("reports/skill-evidence/demo-skill/events.jsonl");
    let stream = fs::read_to_string(&stream_path).expect("read event stream");
    let mut events = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .collect::<Vec<_>>();
    events.last_mut().expect("review_started event")["payload"]["operating_skill_hash"] =
        serde_json::json!(42);
    fs::write(
        &stream_path,
        format!(
            "{}\n",
            events
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        ),
    )
    .expect("write malformed fixture event");

    let read = skill_evidence::read_validated_event_stream(&stream_path)
        .expect("read malformed event stream");
    assert!(
        read.integrity_errors
            .iter()
            .any(|error| error
                .contains("operating_skill_hash must be a non-empty string when present")),
        "a malformed present identity must not collapse into historical absence: {:?}",
        read.integrity_errors
    );
    let status = skill_evidence::derive_store(
        consumer.path(),
        Path::new(".claude/skills/demo-skill"),
        &DerivationInputs {
            generated_at: "2026-01-02T05:00:00Z".to_owned(),
            now_epoch_milliseconds: 1_767_323_045_000,
            session_id: "derive-session".to_owned(),
            lock_owner: "lock-derive-malformed".to_owned(),
        },
    )
    .expect("derive malformed stream into a blocked projection");
    assert_eq!(status.state, "blocked");
    assert_eq!(status.integrity_errors, read.integrity_errors);
}
