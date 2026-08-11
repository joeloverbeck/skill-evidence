#![forbid(unsafe_code)]

use std::{fs, path::Path};

use clap::{Parser, Subcommand};
use serde_json::Value;
use skill_evidence::{
    DerivationInputs, Host, RecordInputs, RecordUseRequest, assets,
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
    let host = operator_host(operator_root);
    assets::install(operator_root, &host, false).expect("install current operating skill package");
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

fn run_claim(
    root: &Path,
    host: &Host,
    suffix: &str,
    record_operating_hash: bool,
) -> (Exit, Vec<u8>, Vec<u8>) {
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
    (exit, stdout, stderr)
}

fn claim(root: &Path, host: &Host, suffix: &str, record_operating_hash: bool) -> (Value, String) {
    let (exit, _stdout, stderr) = run_claim(root, host, suffix, record_operating_hash);
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

#[test]
fn stale_operating_package_refuses_claim_without_changing_the_evidence_store() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let operating_entrypoint = operator
        .path()
        .join(".claude/skills/skill-evolution/SKILL.md");
    fs::write(
        &operating_entrypoint,
        "---\nname: skill-evolution\n---\nSuperseded review instructions.\n",
    )
    .expect("replace current operating instructions with a superseded copy");
    let host = operator_host(operator.path());
    let consumer = authorized_consumer(&host);
    let events_path = consumer
        .path()
        .join("reports/skill-evidence/demo-skill/events.jsonl");
    let projection_path = consumer
        .path()
        .join("reports/skill-evidence/demo-skill/gate-status.json");
    let events_before = fs::read(&events_path).expect("snapshot event stream before refusal");
    let projection_before =
        fs::read(&projection_path).expect("snapshot gate projection before refusal");

    let (exit, stdout, stderr) =
        run_claim(consumer.path(), &host, "review-superseded-package", false);

    assert_eq!(exit, Exit::Refusal);
    assert!(stdout.is_empty());
    let refusal = String::from_utf8(stderr).expect("UTF-8 refusal");
    assert!(
        refusal.contains(".claude/skills/skill-evolution/SKILL.md"),
        "refusal must name the differing installed file: {refusal}"
    );
    assert!(
        refusal.contains("skills evidence install") && refusal.contains("--force"),
        "refusal must name the installer remedy: {refusal}"
    );
    assert_eq!(
        fs::read(&events_path).expect("read event stream after refusal"),
        events_before,
        "claim refusal must append no event"
    );
    assert_eq!(
        fs::read(&projection_path).expect("read gate projection after refusal"),
        projection_before,
        "claim refusal must not rewrite the projection"
    );
}

fn model_latest_claim_as_historical_without_operating_hash(root: &Path) {
    // The current writer cannot produce this shape anymore, so prepare historical input bytes
    // before exercising the reader. This is fixture construction in a temporary repository,
    // not a lifecycle operation that rewrites a consumer's append-only history.
    let stream_path = root.join("reports/skill-evidence/demo-skill/events.jsonl");
    let stream = fs::read_to_string(&stream_path).expect("read event stream");
    let mut lines = stream.lines().map(str::to_owned).collect::<Vec<_>>();
    let latest = lines.last_mut().expect("review_started line");
    let event: Value = serde_json::from_str(latest).expect("review_started JSON");
    assert_eq!(event["event_type"], "review_started");
    let hash = event["payload"]["operating_skill_hash"]
        .as_str()
        .expect("current writer operating hash");
    let identity_fragment = format!(
        ",\"operating_skill_hash\":{}",
        serde_json::to_string(hash).expect("encode operating hash")
    );
    assert!(
        latest.contains(&identity_fragment),
        "current claim bytes must carry the removable identity fragment"
    );
    *latest = latest.replacen(&identity_fragment, "", 1);
    fs::write(&stream_path, format!("{}\n", lines.join("\n")))
        .expect("write historical event fixture");
}

fn make_candidate(root: &Path) -> std::path::PathBuf {
    let candidate = root.join("reports/skill-evidence/demo-skill/reviews/candidate");
    fs::create_dir_all(&candidate).expect("create candidate directory");
    fs::write(
        candidate.join("SKILL.md"),
        "---\nname: demo-skill\n---\nCandidate body.\n",
    )
    .expect("write candidate skill");
    candidate
}

fn record_validation(root: &Path, host: &Host, review_id: &str) -> Value {
    let candidate = make_candidate(root);
    let parsed = TestCli::try_parse_from([
        "test-host".to_owned(),
        "skills".to_owned(),
        "evolution".to_owned(),
        "record-validation".to_owned(),
        "--root".to_owned(),
        root.to_str().expect("UTF-8 fixture root").to_owned(),
        "--target".to_owned(),
        ".claude/skills/demo-skill".to_owned(),
        "--review-id".to_owned(),
        review_id.to_owned(),
        "--decision".to_owned(),
        "accepted".to_owned(),
        "--risk-tier".to_owned(),
        "ordinary".to_owned(),
        "--candidate".to_owned(),
        candidate.to_str().expect("UTF-8 candidate path").to_owned(),
        "--trials".to_owned(),
        "3".to_owned(),
        "--artifacts".to_owned(),
        "reports/skill-evidence/demo-skill/reviews/trials".to_owned(),
        "--event-id".to_owned(),
        format!("evt_validation_{review_id}"),
        "--recorded-at".to_owned(),
        "2026-01-02T05:00:00Z".to_owned(),
        "--now-epoch-milliseconds".to_owned(),
        "1767323045001".to_owned(),
        "--repository-head".to_owned(),
        "fixture-head".to_owned(),
        "--session-id".to_owned(),
        format!("review-session-{review_id}"),
        "--lock-owner".to_owned(),
        format!("lock-validation-{review_id}"),
    ])
    .expect("parse mounted Skill Evolution validation");
    let TestCommand::Skills(args) = parsed.command;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let exit = cli::run(args, host, &mut stdout, &mut stderr);
    assert_eq!(exit, Exit::Success, "{}", String::from_utf8_lossy(&stderr));

    let stream_path = root.join("reports/skill-evidence/demo-skill/events.jsonl");
    let validated = skill_evidence::read_validated_event_stream(&stream_path)
        .expect("round-trip validation event through the crate reader");
    assert!(validated.integrity_errors.is_empty());
    validated.events.last().expect("validation event").clone()
}

fn land(root: &Path, host: &Host, review_id: &str) -> Value {
    let candidate = root.join("reports/skill-evidence/demo-skill/reviews/candidate");
    let parsed = TestCli::try_parse_from([
        "test-host".to_owned(),
        "skills".to_owned(),
        "evolution".to_owned(),
        "land".to_owned(),
        "--root".to_owned(),
        root.to_str().expect("UTF-8 fixture root").to_owned(),
        "--target".to_owned(),
        ".claude/skills/demo-skill".to_owned(),
        "--review-id".to_owned(),
        review_id.to_owned(),
        "--candidate".to_owned(),
        candidate.to_str().expect("UTF-8 candidate path").to_owned(),
        "--event-id".to_owned(),
        format!("evt_land_{review_id}"),
        "--recorded-at".to_owned(),
        "2026-01-02T06:00:00Z".to_owned(),
        "--now-epoch-milliseconds".to_owned(),
        "1767323045002".to_owned(),
        "--repository-head".to_owned(),
        "fixture-head".to_owned(),
        "--session-id".to_owned(),
        format!("review-session-{review_id}"),
        "--lock-owner".to_owned(),
        format!("lock-land-{review_id}"),
    ])
    .expect("parse mounted Skill Evolution landing");
    let TestCommand::Skills(args) = parsed.command;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let exit = cli::run(args, host, &mut stdout, &mut stderr);
    assert_eq!(exit, Exit::Success, "{}", String::from_utf8_lossy(&stderr));

    let stream_path = root.join("reports/skill-evidence/demo-skill/events.jsonl");
    let validated = skill_evidence::read_validated_event_stream(&stream_path)
        .expect("round-trip landing event through the crate reader");
    assert!(validated.integrity_errors.is_empty());
    validated.events.last().expect("landing event").clone()
}

fn close(root: &Path, host: &Host, review_id: &str, serial: i64) -> Value {
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

    let stream_path = root.join("reports/skill-evidence/demo-skill/events.jsonl");
    let validated = skill_evidence::read_validated_event_stream(&stream_path)
        .expect("round-trip close event through the crate reader");
    assert!(validated.integrity_errors.is_empty());
    validated.events.last().expect("close event").clone()
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
fn preflight_reports_a_superseded_operating_package_without_refusing() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());
    let consumer = authorized_consumer(&host);

    let matching = preflight(consumer.path(), &host);
    assert!(
        matching
            .as_object()
            .expect("preflight receipt")
            .get("operating_package_matches_shipped")
            .is_none(),
        "matching-package preflight output stays byte-compatible"
    );

    fs::write(
        operator
            .path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
        "Superseded preflight instructions.\n",
    )
    .expect("replace the operating package before preflight");

    let superseded = preflight(consumer.path(), &host);

    assert_eq!(superseded["operating_package_matches_shipped"], false);
    assert_eq!(
        superseded["operating_package_differing_files"],
        serde_json::json!([".claude/skills/skill-evolution/references/authorized-review.md"])
    );
}

#[test]
fn claim_records_a_stable_content_sensitive_operating_skill_hash_regardless_of_legacy_flag() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());
    let operator_target = Path::new(".claude/skills/skill-evolution");
    let expected_before = skill_evidence::hash_skill(operator.path(), operator_target, &host)
        .expect("hash operating package")
        .content_hash;

    let first = authorized_consumer(&host);
    let second = authorized_consumer(&host);
    let (first_event, first_line) = claim(first.path(), &host, "review-flag", false);
    let (second_event, second_line) = claim(second.path(), &host, "review-flag", true);

    assert_eq!(
        first_event["payload"]["operating_skill_hash"],
        expected_before
    );
    assert_eq!(
        first_event["payload"]["operating_package_matches_shipped"], true,
        "a claim under the current rendered package records matched provenance"
    );
    assert_eq!(
        second_event["payload"]["operating_skill_hash"], expected_before,
        "unchanged operating-package bytes must yield the same identity"
    );
    assert_eq!(
        second_line, first_line,
        "the retained --record-operating-skill-hash flag must be an accepted no-op"
    );
    assert!(
        first_line.ends_with(&format!(
            ",\"operating_skill_hash\":\"{expected_before}\",\"operating_package_matches_shipped\":true}}}}"
        )),
        "the computed provenance keys must be appended deterministically: {first_line}"
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
    let (changed_event, _) = claim(changed.path(), &host, "review-changed", false);

    assert_ne!(expected_after, expected_before);
    assert_eq!(
        changed_event["payload"]["operating_skill_hash"], expected_after,
        "editing any operating-package file must change the recorded identity"
    );
}

#[test]
fn validation_records_current_operating_skill_hash() {
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
    claim(consumer.path(), &host, "review-validation", false);

    let event = record_validation(consumer.path(), &host, "review-validation");

    assert_eq!(event["event_type"], "validation_completed");
    assert_eq!(event["payload"]["operating_skill_hash"], expected);
    assert_eq!(event["payload"]["operating_package_matches_shipped"], true);

    let stale_consumer = authorized_consumer(&host);
    claim(
        stale_consumer.path(),
        &host,
        "review-stale-validation",
        false,
    );
    fs::write(
        operator
            .path()
            .join(".claude/skills/skill-evolution/SKILL.md"),
        "---\nname: skill-evolution\n---\nSuperseded validation instructions.\n",
    )
    .expect("replace the operating package after claim");

    let stale_event = record_validation(stale_consumer.path(), &host, "review-stale-validation");

    assert_eq!(stale_event["event_type"], "validation_completed");
    assert_eq!(
        stale_event["payload"]["operating_package_matches_shipped"], false,
        "validation must proceed and record the superseded operating package"
    );
}

#[test]
fn landing_records_current_operating_skill_hash() {
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
    claim(consumer.path(), &host, "review-landing", false);
    record_validation(consumer.path(), &host, "review-landing");

    let event = land(consumer.path(), &host, "review-landing");

    assert_eq!(event["event_type"], "change_landed");
    assert_eq!(event["payload"]["operating_skill_hash"], expected);
    assert_eq!(event["payload"]["operating_package_matches_shipped"], true);

    let stale_consumer = authorized_consumer(&host);
    claim(stale_consumer.path(), &host, "review-stale-landing", false);
    record_validation(stale_consumer.path(), &host, "review-stale-landing");
    fs::write(
        operator
            .path()
            .join(".claude/skills/skill-evolution/SKILL.md"),
        "---\nname: skill-evolution\n---\nSuperseded landing instructions.\n",
    )
    .expect("replace the operating package after validation");

    let stale_event = land(stale_consumer.path(), &host, "review-stale-landing");

    assert_eq!(stale_event["event_type"], "change_landed");
    assert_eq!(
        stale_event["payload"]["operating_package_matches_shipped"], false,
        "landing must proceed and record the superseded operating package"
    );
}

#[test]
fn close_records_the_changed_operating_skill_hash() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());

    let matching_consumer = authorized_consumer(&host);
    claim(
        matching_consumer.path(),
        &host,
        "review-matching-close",
        false,
    );
    let matching_close = close(matching_consumer.path(), &host, "review-matching-close", 6);
    assert_eq!(
        matching_close["payload"]["operating_package_matches_shipped"],
        true
    );

    let consumer = authorized_consumer(&host);
    let (claim_event, _) = claim(consumer.path(), &host, "review-close", false);
    let claim_hash = claim_event["payload"]["operating_skill_hash"]
        .as_str()
        .expect("claim operating hash")
        .to_owned();

    fs::write(
        operator
            .path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
        "Changed operating rule.\n",
    )
    .expect("edit operating package between claim and close");
    let close_hash = skill_evidence::hash_skill(
        operator.path(),
        Path::new(".claude/skills/skill-evolution"),
        &host,
    )
    .expect("rehash changed operating package")
    .content_hash;

    let close_event = close(consumer.path(), &host, "review-close", 7);

    assert_eq!(close_event["event_type"], "review_disposition");
    assert_eq!(close_event["payload"]["operating_skill_hash"], close_hash);
    assert_eq!(
        close_event["payload"]["operating_package_matches_shipped"], false,
        "close must proceed and record the superseded operating package"
    );
    assert_ne!(
        claim_hash, close_hash,
        "each event must identify the package operating when that event was written"
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
    claim(consumer.path(), &host, "review-unknown", true);
    model_latest_claim_as_historical_without_operating_hash(consumer.path());
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

fn assert_reader_blocks_malformed_provenance_on_latest_event(
    root: &Path,
    event_type: &str,
    field: &str,
    malformed: Value,
    expected_error: &str,
) {
    let stream_path = root.join("reports/skill-evidence/demo-skill/events.jsonl");
    let stream = fs::read_to_string(&stream_path).expect("read event stream");
    let mut events = stream
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("event JSON"))
        .collect::<Vec<_>>();
    let latest = events.last_mut().expect("identity-bearing event");
    assert_eq!(latest["event_type"], event_type);
    latest["payload"][field] = malformed;
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
            .any(|error| error.contains(expected_error)),
        "malformed present provenance must not collapse into historical absence: {:?}",
        read.integrity_errors
    );
    let status = skill_evidence::derive_store(
        root,
        Path::new(".claude/skills/demo-skill"),
        &DerivationInputs {
            generated_at: "2026-01-02T05:00:00Z".to_owned(),
            now_epoch_milliseconds: 1_767_323_045_000,
            session_id: "derive-session".to_owned(),
            lock_owner: format!("lock-derive-malformed-{event_type}"),
        },
    )
    .expect("derive malformed stream into a blocked projection");
    assert_eq!(status.state, "blocked");
    assert_eq!(status.integrity_errors, read.integrity_errors);
}

#[test]
fn reader_blocks_a_malformed_present_operating_skill_hash_on_each_review_event() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());

    let claimed = authorized_consumer(&host);
    claim(claimed.path(), &host, "review-malformed-claim", true);
    assert_reader_blocks_malformed_provenance_on_latest_event(
        claimed.path(),
        "review_started",
        "operating_skill_hash",
        serde_json::json!(42),
        "operating_skill_hash must be a non-empty string when present",
    );

    let validated = authorized_consumer(&host);
    claim(validated.path(), &host, "review-malformed-validation", true);
    record_validation(validated.path(), &host, "review-malformed-validation");
    assert_reader_blocks_malformed_provenance_on_latest_event(
        validated.path(),
        "validation_completed",
        "operating_skill_hash",
        serde_json::json!(42),
        "operating_skill_hash must be a non-empty string when present",
    );

    let landed = authorized_consumer(&host);
    claim(landed.path(), &host, "review-malformed-landing", true);
    record_validation(landed.path(), &host, "review-malformed-landing");
    land(landed.path(), &host, "review-malformed-landing");
    assert_reader_blocks_malformed_provenance_on_latest_event(
        landed.path(),
        "change_landed",
        "operating_skill_hash",
        serde_json::json!(42),
        "operating_skill_hash must be a non-empty string when present",
    );

    let closed = authorized_consumer(&host);
    claim(closed.path(), &host, "review-malformed-close", true);
    close(closed.path(), &host, "review-malformed-close", 7);
    assert_reader_blocks_malformed_provenance_on_latest_event(
        closed.path(),
        "review_disposition",
        "operating_skill_hash",
        serde_json::json!(42),
        "operating_skill_hash must be a non-empty string when present",
    );
}

#[test]
fn reader_blocks_a_malformed_present_operating_package_match_on_each_review_event() {
    let operator = tempfile::tempdir().expect("temporary operator repository");
    write_operator_package(operator.path());
    let host = operator_host(operator.path());

    let claimed = authorized_consumer(&host);
    claim(claimed.path(), &host, "review-malformed-match-claim", false);
    assert_reader_blocks_malformed_provenance_on_latest_event(
        claimed.path(),
        "review_started",
        "operating_package_matches_shipped",
        serde_json::json!("unknown"),
        "operating_package_matches_shipped must be boolean when present",
    );

    let validated = authorized_consumer(&host);
    claim(
        validated.path(),
        &host,
        "review-malformed-match-validation",
        false,
    );
    record_validation(validated.path(), &host, "review-malformed-match-validation");
    assert_reader_blocks_malformed_provenance_on_latest_event(
        validated.path(),
        "validation_completed",
        "operating_package_matches_shipped",
        serde_json::json!("unknown"),
        "operating_package_matches_shipped must be boolean when present",
    );

    let landed = authorized_consumer(&host);
    claim(
        landed.path(),
        &host,
        "review-malformed-match-landing",
        false,
    );
    record_validation(landed.path(), &host, "review-malformed-match-landing");
    land(landed.path(), &host, "review-malformed-match-landing");
    assert_reader_blocks_malformed_provenance_on_latest_event(
        landed.path(),
        "change_landed",
        "operating_package_matches_shipped",
        serde_json::json!("unknown"),
        "operating_package_matches_shipped must be boolean when present",
    );

    let closed = authorized_consumer(&host);
    claim(closed.path(), &host, "review-malformed-match-close", false);
    close(closed.path(), &host, "review-malformed-match-close", 7);
    assert_reader_blocks_malformed_provenance_on_latest_event(
        closed.path(),
        "review_disposition",
        "operating_package_matches_shipped",
        serde_json::json!("unknown"),
        "operating_package_matches_shipped must be boolean when present",
    );
}
