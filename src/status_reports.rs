use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use regex::Regex;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use super::{
    CandidateCluster, DerivationInputs, Error, EvidenceEvent, Host, Outcome, SESSION_UNAVAILABLE,
    derive_gate, format_timestamp_milliseconds, hash_target_directory,
    parse_method_gap_family_selector, read_event_stream, refusal, target_context, unsafe_failure,
};

const MAX_LINEAGE_BYTES: u64 = 2 * 1024 * 1024;
const LINEAGE_EXTENSIONS: &[&str] = &["md", "json", "jsonl", "yaml", "yml"];

#[derive(Debug, Serialize)]
pub struct MethodGapResearchInventory {
    schema: String,
    generated_at: String,
    family_selector: String,
    family_prefix: String,
    canonical_members: usize,
    targets: Vec<MethodGapTargetInventory>,
}

#[derive(Debug, Serialize)]
struct MethodGapTargetInventory {
    target_name: String,
    target_path: String,
    target_content_hash: String,
    target_file_count: usize,
    evidence_store: Option<String>,
    evidence_integrity_errors: Vec<String>,
    current_evidence: CurrentEvidence,
    current_candidate_clusters: Vec<CandidateCluster>,
    /// Open incidents an instrument-limited close retired from the evolution gate.
    ///
    /// `current_candidate_clusters` drops them, because they no longer cluster, while
    /// `current_evidence` still lists them individually — without this the difference
    /// between the two reads as an inconsistency rather than a recorded decision.
    /// Omitted when empty.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    instrument_limited_incident_ids: Vec<String>,
    active_evidence_review_id: Option<String>,
    last_completed_evidence_review_id: Option<String>,
    lineage_candidates: Vec<LineageCandidate>,
    git: GitObservation,
}

#[derive(Debug, Serialize)]
struct CurrentEvidence {
    qualifying_uses: usize,
    outcome_counts: OutcomeCounts,
    open_incidents: Vec<OpenIncident>,
    observed_target_hashes: Vec<ObservedTargetHash>,
}

#[derive(Debug, Serialize)]
struct OpenIncident {
    event_id: String,
    recorded_at: String,
    top_level_session_id: String,
    retrospective: bool,
    task_label: String,
    task_fingerprint: String,
    outcome: String,
    symptom_key: Option<String>,
    expected: Option<String>,
    observed: Option<String>,
    consequence: Option<String>,
    workaround_taken: Option<String>,
    evidence_refs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ObservedTargetHash {
    content_hash: String,
    qualifying_uses: usize,
    first_recorded_at: String,
    last_recorded_at: String,
}

#[derive(Debug, Serialize)]
struct OutcomeCounts {
    clean: usize,
    friction: usize,
    material_failure: usize,
    severe_incident: usize,
}

#[derive(Debug, Serialize)]
struct GitObservation {
    worktree_status: Option<Vec<String>>,
    recent_commits: Option<Vec<GitCommit>>,
    unavailable_reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct GitCommit {
    commit: String,
    committed_at: String,
    subject: String,
}

#[derive(Debug)]
struct LineageFile {
    path: String,
    absolute: PathBuf,
    size: u64,
}

#[derive(Debug, Serialize)]
struct LineageCandidate {
    path: String,
    discovery: &'static str,
    kind: &'static str,
    size_bytes: u64,
    line_count: Option<usize>,
    sha256: Option<String>,
    header_excerpt: Vec<String>,
    signal_lines: Vec<String>,
    unreadable_reason: Option<String>,
}

pub fn method_gap_research_inventory(
    root: &Path,
    selector: &str,
    now_epoch_milliseconds: i64,
    host: &Host,
) -> Result<MethodGapResearchInventory, Error> {
    let prefix = parse_method_gap_family_selector(selector)?;
    let generated_at = report_timestamp(now_epoch_milliseconds)?;
    let root = root.canonicalize().map_err(|error| {
        unsafe_failure(format!(
            "Could not resolve repository root {}: {error}",
            root.display()
        ))
    })?;
    let skills_directory = root.join(".claude/skills");
    if !skills_directory.is_dir() {
        return Err(unsafe_failure(format!(
            "Canonical skill directory not found: {}",
            skills_directory.display()
        )));
    }
    let mut target_names = fs::read_dir(&skills_directory)
        .map_err(|error| {
            unsafe_failure(format!(
                "Could not read canonical skill directory {}: {error}",
                skills_directory.display()
            ))
        })?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with(&prefix))
        .filter(|name| skills_directory.join(name).join("SKILL.md").is_file())
        .collect::<Vec<_>>();
    target_names.sort();
    if target_names.is_empty() {
        return Err(unsafe_failure(format!(
            "No canonical skills match family {selector:?}."
        )));
    }
    let lineage_files = read_lineage_files(&root)?;

    let targets = target_names
        .into_iter()
        .map(|target_name| {
            let target_path = format!(".claude/skills/{target_name}");
            let target = target_context(&root, &root.join(&target_path))?;
            let hash = hash_target_directory(&target.target_real)?;
            let evidence_store = target.evidence_directory.is_dir().then(|| {
                target
                    .evidence_directory
                    .strip_prefix(&root)
                    .unwrap_or(&target.evidence_directory)
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/")
            });
            let (events, stream_errors) =
                read_event_stream(&target.evidence_directory.join("events.jsonl"))?;
            let mut integrity_errors = stream_errors;
            match validated_event_target_path(&events) {
                Ok(Some(event_path)) if event_path != target_path => {
                    integrity_errors.push(format!(
                        "evidence target path {event_path} does not match canonical target {target_path}"
                    ));
                }
                Ok(_) => {}
                Err(error) => integrity_errors.push(error),
            }
            let inputs = DerivationInputs {
                generated_at: generated_at.clone(),
                now_epoch_milliseconds,
                session_id: SESSION_UNAVAILABLE.to_owned(),
                lock_owner: "read-only-method-gap-status".to_owned(),
            };
            let gate = derive_gate(
                &target,
                &hash.content_hash,
                &events,
                integrity_errors.clone(),
                &inputs,
            );
            Ok(MethodGapTargetInventory {
                target_name,
                target_path,
                target_content_hash: hash.content_hash,
                target_file_count: hash.file_count,
                evidence_store,
                evidence_integrity_errors: integrity_errors,
                current_evidence: summarize_current_evidence(
                    &events,
                    &gate.target_content_hash,
                    &gate.open_incident_ids,
                ),
                current_candidate_clusters: gate.candidate_clusters,
                instrument_limited_incident_ids: gate.instrument_limited_incident_ids,
                active_evidence_review_id: gate.active_review_id,
                last_completed_evidence_review_id: gate.last_completed_review_id,
                lineage_candidates: lineage_candidates(
                    &lineage_files,
                    &target.target_name,
                    &target.repo_relative_path,
                ),
                git: git_observation(&root, &target.repo_relative_path),
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(MethodGapResearchInventory {
        schema: host.method_gap_inventory_schema(),
        generated_at,
        family_selector: selector.to_owned(),
        family_prefix: prefix,
        canonical_members: targets.len(),
        targets,
    })
}

pub fn skill_evolution_status(
    root: &Path,
    now_epoch_milliseconds: i64,
    session_id: &str,
) -> Result<String, Error> {
    let generated_at = report_timestamp(now_epoch_milliseconds)?;
    let root = root.canonicalize().map_err(|error| {
        unsafe_failure(format!(
            "Could not resolve repository root {}: {error}",
            root.display()
        ))
    })?;
    let evolution_skill = root.join(".claude/skills/skill-evolution/SKILL.md");
    if !evolution_skill.is_file() {
        return Err(unsafe_failure(format!(
            "Required sibling Skill Evolution not found: {}",
            evolution_skill.display()
        )));
    }
    let evolution_target = evolution_skill
        .parent()
        .expect("Skill Evolution entrypoint has a parent")
        .canonicalize()
        .map_err(|error| {
            unsafe_failure(format!(
                "Could not resolve sibling Skill Evolution {}: {error}",
                evolution_skill.display()
            ))
        })?;
    let evidence_root = root.join("reports/skill-evidence");
    let mut stores = if evidence_root.is_dir() {
        fs::read_dir(&evidence_root)
            .map_err(|error| {
                unsafe_failure(format!(
                    "Could not read evidence-store directory {}: {error}",
                    evidence_root.display()
                ))
            })?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.path())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    stores.sort();
    let stores_scanned = stores.len();
    let inputs = DerivationInputs {
        generated_at,
        now_epoch_milliseconds,
        session_id: session_id.to_owned(),
        lock_owner: "read-only-evolution-status".to_owned(),
    };
    let mut ready = Vec::new();
    let mut deferred = Vec::new();
    let mut retired = Vec::new();
    let mut blocked = Vec::new();
    let mut indeterminate = Vec::new();
    let mut omitted = 0;
    for store in &stores {
        let store_key = store
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unavailable")
            .to_owned();
        let events_path = store.join("events.jsonl");
        let has_event_stream = events_path.is_file();
        let (events, errors) = match read_event_stream(&events_path) {
            Ok(stream) => stream,
            Err(error) => {
                indeterminate.push(EvolutionIndeterminateEntry {
                    kind: "scan_failure",
                    store_key,
                    target_path: None,
                    errors: vec![error.to_string()],
                });
                continue;
            }
        };
        let (projection, projection_error) = read_gate_projection(&store.join("gate-status.json"));
        let (event_target_path, identity_error) = match validated_event_target_path(&events) {
            Ok(path) => (path, None),
            Err(error) => (None, Some(error)),
        };
        let target_path = event_target_path.or_else(|| {
            has_event_stream
                .then(|| {
                    projection
                        .as_ref()
                        .and_then(|value| value["target_repo_relative_path"].as_str())
                        .filter(|path| !path.is_empty())
                        .map(str::to_owned)
                })
                .flatten()
        });
        let Some(target_path) = target_path else {
            let has_stream_errors = !errors.is_empty();
            let mut reported_errors = errors;
            if let Some(error) = projection_error {
                reported_errors.push(error);
            }
            reported_errors.push(identity_error.unwrap_or_else(|| {
                if has_event_stream {
                    "no target path exists in events.jsonl or gate-status.json".to_owned()
                } else {
                    "events.jsonl does not exist; a gate projection alone cannot identify an evidence store"
                        .to_owned()
                }
            }));
            indeterminate.push(EvolutionIndeterminateEntry {
                kind: if has_stream_errors {
                    "integrity"
                } else {
                    "unidentified_store"
                },
                store_key,
                target_path: None,
                errors: reported_errors,
            });
            continue;
        };
        if let Some(identity_error) = identity_error {
            let mut reported_errors = errors;
            reported_errors.push(identity_error);
            indeterminate.push(EvolutionIndeterminateEntry {
                kind: "integrity",
                store_key,
                target_path: Some(target_path),
                errors: reported_errors,
            });
            continue;
        }
        if !errors.is_empty() {
            indeterminate.push(EvolutionIndeterminateEntry {
                kind: "integrity",
                store_key,
                target_path: Some(target_path),
                errors,
            });
            continue;
        }
        let recorded_target = Path::new(&target_path);
        let rooted_target = if recorded_target.is_absolute() {
            recorded_target.to_owned()
        } else {
            root.join(recorded_target)
        };
        let target = match target_context(&root, &rooted_target) {
            Ok(target) => target,
            Err(error) => {
                let mut reported_errors = Vec::new();
                if let Some(projection_error) = projection_error {
                    reported_errors.push(projection_error);
                }
                reported_errors.push(error.to_string());
                indeterminate.push(EvolutionIndeterminateEntry {
                    kind: "missing_target",
                    store_key,
                    target_path: Some(target_path),
                    errors: reported_errors,
                });
                continue;
            }
        };
        let hash = match hash_target_directory(&target.target_real) {
            Ok(hash) => hash,
            Err(error) => {
                indeterminate.push(EvolutionIndeterminateEntry {
                    kind: "scan_failure",
                    store_key,
                    target_path: None,
                    errors: vec![error.to_string()],
                });
                continue;
            }
        };
        let status = derive_gate(&target, &hash.content_hash, &events, errors, &inputs);
        if status.state == "review_in_progress" {
            let started = events.iter().rev().find(|event| {
                event.starts_ownership() && event.review_id() == status.active_review_id.as_deref()
            });
            let without_active_start = events
                .iter()
                .filter(|event| started.is_none_or(|started| event.event_id != started.event_id))
                .cloned()
                .collect::<Vec<_>>();
            let underlying = derive_gate(
                &target,
                &hash.content_hash,
                &without_active_start,
                Vec::new(),
                &inputs,
            );
            if started.is_some_and(|event| !event.is_review_start())
                && underlying.authorization_reason.is_none()
            {
                omitted += 1;
                continue;
            }
            let quarantined = underlying.authorization_reason.as_deref() == Some("severe");
            let authorization_reason = started
                .and_then(|event| {
                    event
                        .raw
                        .pointer("/payload/authorizing_rule")
                        .and_then(Value::as_str)
                })
                .map(str::to_owned)
                .or(underlying.authorization_reason)
                .unwrap_or_default();
            blocked.push(EvolutionBlockedEntry {
                target_path: target.repo_relative_path,
                authorization_reason,
                review_reentry_basis: underlying.review_reentry_basis,
                instrument_limited_incidents: underlying.instrument_limited_incident_ids.len(),
                quarantined,
                kind: EvolutionBlockedKind::ReviewInProgress {
                    review_id: status.active_review_id.unwrap_or_default(),
                    started_at: started.map(|event| event.recorded_at.clone()),
                    operator_workflow: started.and_then(|event| {
                        event
                            .raw
                            .get("operator_workflow")
                            .and_then(Value::as_str)
                            .map(str::to_owned)
                    }),
                    risk_tier: started.and_then(|event| {
                        event
                            .raw
                            .pointer("/payload/risk_tier")
                            .and_then(Value::as_str)
                            .map(str::to_owned)
                    }),
                    review_artifacts: format!("{}/reviews", repo_path(&root, store)),
                },
            });
            continue;
        }
        if status.authorization_reason.is_none() {
            let basis = status.review_reentry_basis.as_deref();
            let instrument_limited_incidents = status.instrument_limited_incident_ids.len();
            if matches!(
                basis,
                Some("queued_pre_close_evidence" | "queued_behind_instrument_limited_review")
            ) {
                deferred.push(EvolutionDeferredEntry {
                    instrument_limited: basis == Some("queued_behind_instrument_limited_review"),
                    instrument_limited_incidents,
                    target_path: target.repo_relative_path,
                });
                continue;
            }
            if instrument_limited_incidents > 0 {
                retired.push(EvolutionRetiredEntry {
                    incidents: instrument_limited_incidents,
                    still_collecting: status
                        .open_incident_ids
                        .len()
                        .saturating_sub(instrument_limited_incidents),
                    target_path: target.repo_relative_path,
                });
                continue;
            }
        }
        if status.authorization_reason.is_some() {
            let instrument_limited_incidents = status.instrument_limited_incident_ids.len();
            let quarantined = status.state.starts_with("quarantined_")
                || status.authorization_reason.as_deref() == Some("severe");
            let authorization_reason = status.authorization_reason.unwrap_or_default();
            if target.target_real == evolution_target {
                let proof = if status.threshold_session_id.is_some() {
                    SelfTargetProof::DifferentSession
                } else {
                    timer_details(status.not_before.as_deref(), now_epoch_milliseconds).map_or(
                        SelfTargetProof::ClockUnavailable,
                        |(not_before, remaining_ms)| SelfTargetProof::Clock {
                            not_before,
                            remaining_ms,
                        },
                    )
                };
                blocked.push(EvolutionBlockedEntry {
                    target_path: target.repo_relative_path,
                    authorization_reason,
                    review_reentry_basis: status.review_reentry_basis,
                    instrument_limited_incidents,
                    quarantined,
                    kind: EvolutionBlockedKind::SelfTarget { proof },
                });
            } else if status.threshold_session_id.is_some() {
                if session_id == SESSION_UNAVAILABLE {
                    blocked.push(EvolutionBlockedEntry {
                        target_path: target.repo_relative_path,
                        authorization_reason,
                        review_reentry_basis: status.review_reentry_basis,
                        instrument_limited_incidents,
                        quarantined,
                        kind: EvolutionBlockedKind::SessionHostRequired,
                    });
                } else {
                    ready.push(EvolutionReadyEntry {
                        target_path: target.repo_relative_path.clone(),
                        authorization_reason,
                        threshold_session_id: status.threshold_session_id,
                        review_reentry_basis: status.review_reentry_basis,
                        instrument_limited_incidents,
                        quarantined,
                        command: evolution_command(&target.repo_relative_path),
                        proof: EvolutionReadyProof::DifferentSession,
                    });
                }
            } else if status.threshold_session_id.is_none() {
                match timer_details(status.not_before.as_deref(), now_epoch_milliseconds) {
                    Some((not_before, remaining_ms)) if remaining_ms > 0 => {
                        blocked.push(EvolutionBlockedEntry {
                            target_path: target.repo_relative_path,
                            authorization_reason,
                            review_reentry_basis: status.review_reentry_basis,
                            instrument_limited_incidents,
                            quarantined,
                            kind: EvolutionBlockedKind::Timer {
                                not_before,
                                remaining_ms,
                            },
                        });
                    }
                    Some((not_before, _)) => {
                        ready.push(EvolutionReadyEntry {
                            target_path: target.repo_relative_path.clone(),
                            authorization_reason,
                            threshold_session_id: None,
                            review_reentry_basis: status.review_reentry_basis,
                            instrument_limited_incidents,
                            quarantined,
                            command: evolution_command(&target.repo_relative_path),
                            proof: EvolutionReadyProof::CooldownElapsed { not_before },
                        });
                    }
                    None => omitted += 1,
                }
            } else {
                omitted += 1;
            }
        } else {
            omitted += 1;
        }
    }
    ready.sort_by(|left, right| {
        right
            .quarantined
            .cmp(&left.quarantined)
            .then_with(|| left.target_path.cmp(&right.target_path))
    });
    blocked.sort_by(|left, right| {
        right
            .quarantined
            .cmp(&left.quarantined)
            .then_with(|| left.not_before().cmp(right.not_before()))
            .then_with(|| left.target_path.cmp(&right.target_path))
    });
    deferred.sort_by(|left, right| left.target_path.cmp(&right.target_path));
    retired.sort_by(|left, right| left.target_path.cmp(&right.target_path));
    indeterminate.sort_by(|left, right| {
        left.target_path
            .as_deref()
            .unwrap_or(&left.store_key)
            .cmp(right.target_path.as_deref().unwrap_or(&right.store_key))
    });
    let store_noun = if stores_scanned == 1 {
        "store"
    } else {
        "stores"
    };
    // Present only when it has something to report. Every bucket has to appear in the
    // census or the counts stop summing to the stores scanned — but a bucket that is
    // always printed would have rewritten `status-reporters-v1`'s golden, and
    // `docs/principles/inherited-prohibitions.md` forbids editing a fixture corpus to make
    // a change pass, without qualification. Omitting an empty count leaves the existing
    // bytes exactly as the JavaScript reporter emitted them.
    let retired_census = if retired.is_empty() {
        String::new()
    } else {
        format!("retired as untestable: {}; ", retired.len())
    };
    let mut report = format!(
        "# Skill Evolution Status\n\nScanned {stores_scanned} evidence {store_noun} read-only. Ready: {}; deferred after review: {}; {retired_census}blocked after eligibility: {}; indeterminate: {}; omitted as not eligible: {omitted}.",
        ready.len(),
        deferred.len(),
        blocked.len(),
        indeterminate.len()
    );
    if ready.is_empty() && blocked.is_empty() {
        report.push_str("\n\nNo eligible targets found.");
    }
    if !ready.is_empty() {
        report.push_str("\n\n## Ready to evolve\n\n");
        report.push_str(
            &ready
                .iter()
                .map(|entry| render_ready_entry(entry, session_id))
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
    }
    if !deferred.is_empty() {
        report.push_str("\n\n## Deferred after review\n\n");
        report.push_str(
            &deferred
                .iter()
                .map(render_deferred_entry)
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
    }
    if !retired.is_empty() {
        report.push_str("\n\n## Retired as untestable\n\n");
        report.push_str(
            &retired
                .iter()
                .map(render_retired_entry)
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
    }
    if !blocked.is_empty() {
        report.push_str("\n\n## Eligible but blocked\n\n");
        report.push_str(
            &blocked
                .iter()
                .map(render_blocked_entry)
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
    }
    if !indeterminate.is_empty() {
        report.push_str("\n\n## Could not determine\n\n");
        report.push_str(
            &indeterminate
                .iter()
                .map(render_indeterminate_entry)
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
    }
    report.push('\n');
    Ok(report)
}

fn report_timestamp(epoch_milliseconds: i64) -> Result<String, Error> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(epoch_milliseconds) * 1_000_000).map_err(
        |_| {
            refusal(format!(
                "Clock epoch milliseconds are outside the supported range: {epoch_milliseconds}."
            ))
        },
    )?;
    Ok(format_timestamp_milliseconds(epoch_milliseconds))
}

fn validated_event_target_path(events: &[EvidenceEvent]) -> Result<Option<String>, String> {
    let mut paths = Vec::new();
    for event in events {
        let Some(path) = event
            .raw
            .pointer("/target/repo_relative_path")
            .and_then(Value::as_str)
        else {
            continue;
        };
        if !paths.iter().any(|known| known == path) {
            paths.push(path.to_owned());
        }
    }
    if paths.len() > 1 {
        Err(format!(
            "validated events name multiple target paths: {}",
            paths.join(", ")
        ))
    } else {
        Ok(paths.into_iter().next())
    }
}

#[derive(Debug)]
struct EvolutionReadyEntry {
    target_path: String,
    authorization_reason: String,
    threshold_session_id: Option<String>,
    review_reentry_basis: Option<String>,
    instrument_limited_incidents: usize,
    quarantined: bool,
    command: String,
    proof: EvolutionReadyProof,
}

#[derive(Debug)]
enum EvolutionReadyProof {
    DifferentSession,
    CooldownElapsed { not_before: String },
}

/// A target whose threshold-supporting evidence is held behind the last completed
/// same-hash review.
#[derive(Debug)]
struct EvolutionDeferredEntry {
    target_path: String,
    /// Whether the review that laid the watermark reached a conclusion. A review that
    /// adjudicated accounted for the evidence it saw; one that closed
    /// `blocked_no_valid_test` accounted for nothing, and saying otherwise reports
    /// unexamined evidence as handled.
    instrument_limited: bool,
    instrument_limited_incidents: usize,
}

fn render_deferred_entry(entry: &EvolutionDeferredEntry) -> String {
    let mut rendered = format!("### {}\n\n", entry.target_path);
    if entry.instrument_limited {
        rendered.push_str(
            "- Re-entry basis: queued behind a review whose instrument could not test what it covered; that close reached no conclusion and adjudicated nothing, so this evidence is unexamined rather than accounted for.\n",
        );
    } else {
        rendered.push_str(
            "- Re-entry basis: queued pre-close evidence only; no threshold-supporting incident was recorded after the last completed same-hash review.\n",
        );
    }
    if let Some(line) = retired_line(entry.instrument_limited_incidents) {
        rendered.push_str(&line);
        rendered.push('\n');
    }
    rendered.push_str(
        "- Gate: Still collecting. Skill Evolution is not authorized, and no command is available.",
    );
    rendered
}

/// A target whose open evidence a review covered under an instrument-limited
/// disposition, so it no longer drives the gate.
///
/// Without its own section these targets fall into `omitted as not eligible`, which is
/// where skills that never recorded an incident go. Reporting real retired evidence as
/// indistinguishable from no evidence is the silence this section exists to break.
#[derive(Debug)]
struct EvolutionRetiredEntry {
    target_path: String,
    incidents: usize,
    /// Open incidents the close did not cover, still short of a threshold. Without this
    /// the entry reads as a closed book, when the target may be one incident from
    /// authorizing a review.
    still_collecting: usize,
}

/// The one place a count meets a noun. Every sentence built on it refers back with
/// "that evidence" rather than a pronoun or verb that would have to agree, because the
/// ten-use threshold can fire on a single incident and a blocked close can retire it.
fn open_incidents(count: usize) -> String {
    let noun = if count == 1 { "incident" } else { "incidents" };
    format!("{count} open {noun}")
}

fn render_retired_entry(entry: &EvolutionRetiredEntry) -> String {
    let mut rendered = format!(
        "### {}\n\n- Retired as untestable: {}, covered by a review that closed `blocked_no_valid_test`. That evidence no longer drives this gate, and remains in the event stream.\n",
        entry.target_path,
        open_incidents(entry.incidents)
    );
    if entry.still_collecting > 0 {
        rendered.push_str(&format!(
            "- Still collecting: {} the close did not cover, short of a threshold.\n",
            open_incidents(entry.still_collecting)
        ));
    }
    rendered.push_str(
        "- Gate: Skill Evolution is not authorized, and no command is available. Another review of the same evidence would meet the same instrument.",
    );
    rendered
}

#[derive(Debug)]
struct EvolutionIndeterminateEntry {
    kind: &'static str,
    store_key: String,
    target_path: Option<String>,
    errors: Vec<String>,
}

fn render_indeterminate_entry(entry: &EvolutionIndeterminateEntry) -> String {
    let label = entry
        .target_path
        .clone()
        .unwrap_or_else(|| format!("evidence store {}", entry.store_key));
    let mut lines = vec![
        format!("### {label}"),
        String::new(),
        format!("- Status: `{}`.", entry.kind),
    ];
    lines.extend(entry.errors.iter().map(|error| format!("- {error}")));
    lines.join("\n")
}

fn read_gate_projection(path: &Path) -> (Option<Value>, Option<String>) {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return (None, None),
        Err(error) => {
            return (
                None,
                Some(format!("Could not read gate-status.json: {error}")),
            );
        }
    };
    match serde_json::from_slice(&bytes) {
        Ok(value) => (Some(value), None),
        Err(error) => (
            None,
            Some(format!("gate-status.json is not valid JSON: {error}")),
        ),
    }
}

#[derive(Debug)]
struct EvolutionBlockedEntry {
    target_path: String,
    authorization_reason: String,
    review_reentry_basis: Option<String>,
    instrument_limited_incidents: usize,
    quarantined: bool,
    kind: EvolutionBlockedKind,
}

impl EvolutionBlockedEntry {
    fn not_before(&self) -> &str {
        match &self.kind {
            EvolutionBlockedKind::Timer { not_before, .. } => not_before,
            EvolutionBlockedKind::SessionHostRequired => "",
            EvolutionBlockedKind::ReviewInProgress { .. } => "",
            EvolutionBlockedKind::SelfTarget { .. } => "",
        }
    }
}

#[derive(Debug)]
enum EvolutionBlockedKind {
    Timer {
        not_before: String,
        remaining_ms: i64,
    },
    SessionHostRequired,
    ReviewInProgress {
        review_id: String,
        started_at: Option<String>,
        operator_workflow: Option<String>,
        risk_tier: Option<String>,
        review_artifacts: String,
    },
    SelfTarget {
        proof: SelfTargetProof,
    },
}

#[derive(Debug)]
enum SelfTargetProof {
    DifferentSession,
    Clock {
        not_before: String,
        remaining_ms: i64,
    },
    ClockUnavailable,
}

fn evolution_command(target_path: &str) -> String {
    let escaped = target_path.replace('\\', "\\\\").replace('"', "\\\"");
    format!("$skill-evolution \"{escaped}\"")
}

fn render_blocked_entry(entry: &EvolutionBlockedEntry) -> String {
    let mut lines = vec![
        format!(
            "### {}{}",
            entry.target_path,
            if entry.quarantined {
                " — QUARANTINED"
            } else {
                ""
            }
        ),
        String::new(),
        format!(
            "- Eligibility: {}.",
            explain_authorization_reason(&entry.authorization_reason)
        ),
    ];
    if let Some(line) = reentry_line(entry.review_reentry_basis.as_deref()) {
        lines.push(line.to_owned());
    }
    if let Some(line) = retired_line(entry.instrument_limited_incidents) {
        lines.push(line);
    }
    match &entry.kind {
        EvolutionBlockedKind::Timer {
            not_before,
            remaining_ms,
        } => {
            lines.push(format!(
                "- Blocker: 12-hour clock fallback; {} remaining.",
                format_duration(*remaining_ms)
            ));
            lines.push(format!(
                "- Eligible at: {} (UTC); {not_before}.",
                format_local_utc(not_before)
            ));
            lines
                .push("- Session effect: Changing sessions will not bypass this timer.".to_owned());
        }
        EvolutionBlockedKind::SessionHostRequired => {
            lines.push("- Blocker: This threshold was recorded with a top-level-session identity, so its destination proof needs a session-ID-capable host. The current host exposes no top-level-session identity.".to_owned());
            lines.push("- Route: Rerun this read-only status census from a session-ID-capable host — Claude Code (`CLAUDE_CODE_SESSION_ID`) or Codex (`CODEX_THREAD_ID`) — whose identity differs from the threshold session. Waiting will not help; the 12-hour clock never applies to a session-ID threshold.".to_owned());
        }
        EvolutionBlockedKind::ReviewInProgress {
            review_id,
            started_at,
            operator_workflow,
            risk_tier,
            review_artifacts,
        } => {
            lines.push(format!(
                "- Blocker: Active review `{review_id}` already owns the target."
            ));
            if let Some(started_at) = started_at {
                lines.push(format!(
                    "- Claimed at: {} (UTC); {started_at}.",
                    format_local_utc(started_at)
                ));
            }
            if let Some(operator_workflow) = operator_workflow {
                lines.push(format!(
                    "- Owner workflow: `{operator_workflow}`{}.",
                    risk_tier
                        .as_ref()
                        .map(|risk| format!("; risk tier `{risk}`"))
                        .unwrap_or_default()
                ));
            }
            lines.push(format!("- Review artifacts: `{review_artifacts}`."));
        }
        EvolutionBlockedKind::SelfTarget { proof } => {
            lines.push(
                "- Blocker: Skill Evolution cannot target itself, so no `$skill-evolution` command is valid."
                    .to_owned(),
            );
            lines.push(
                "- Route: Give the bounded evidence packet to an independent skill-authoring workflow."
                    .to_owned(),
            );
            match proof {
                SelfTargetProof::Clock {
                    not_before,
                    remaining_ms,
                } if *remaining_ms > 0 => lines.push(format!(
                    "- Additional clock blocker: {} remaining; {not_before}.",
                    format_duration(*remaining_ms)
                )),
                SelfTargetProof::DifferentSession => lines.push(
                    "- Session note: A different session can satisfy freshness but cannot remove the self-target prohibition."
                        .to_owned(),
                ),
                SelfTargetProof::Clock { .. } | SelfTargetProof::ClockUnavailable => {}
            }
        }
    }
    if entry.quarantined {
        lines.push("- Quarantine: Stop using this target. Immediate containment is allowed; permanent edits still require the authorized review.".to_owned());
    }
    lines.join("\n")
}

fn timer_details(not_before: Option<&str>, now_epoch_milliseconds: i64) -> Option<(String, i64)> {
    let not_before = not_before?;
    let parsed = OffsetDateTime::parse(not_before, &Rfc3339).ok()?;
    let not_before_milliseconds = i64::try_from(parsed.unix_timestamp_nanos() / 1_000_000).ok()?;
    Some((
        format_timestamp_milliseconds(not_before_milliseconds),
        (not_before_milliseconds - now_epoch_milliseconds).max(0),
    ))
}

fn format_duration(milliseconds: i64) -> String {
    if milliseconds <= 0 {
        return "0m".to_owned();
    }
    let minutes = (milliseconds + 60_000 - 1) / 60_000;
    let days = minutes / (24 * 60);
    let hours = (minutes % (24 * 60)) / 60;
    let mins = minutes % 60;
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 || days > 0 {
        parts.push(if days > 0 {
            format!("{hours:02}h")
        } else {
            format!("{hours}h")
        });
    }
    parts.push(if hours > 0 || days > 0 {
        format!("{mins:02}m")
    } else {
        format!("{mins}m")
    });
    parts.join(" ")
}

fn format_local_utc(timestamp: &str) -> String {
    let parsed = OffsetDateTime::parse(timestamp, &Rfc3339)
        .expect("validated gate timestamp parses as RFC 3339");
    let format = time::format_description::parse_borrowed::<3>(
        "[day]/[month]/[year], [hour]:[minute]:[second] UTC",
    )
    .expect("static UTC display format");
    parsed.format(&format).expect("UTC timestamp formats")
}

fn render_ready_entry(entry: &EvolutionReadyEntry, census_session_id: &str) -> String {
    let mut lines = vec![
        format!(
            "### {}{}",
            entry.target_path,
            if entry.quarantined {
                " — QUARANTINED"
            } else {
                ""
            }
        ),
        String::new(),
        format!(
            "- Eligibility: {}.",
            explain_authorization_reason(&entry.authorization_reason)
        ),
    ];
    if let Some(line) = reentry_line(entry.review_reentry_basis.as_deref()) {
        lines.push(line.to_owned());
    }
    if let Some(line) = retired_line(entry.instrument_limited_incidents) {
        lines.push(line);
    }
    match &entry.proof {
        EvolutionReadyProof::DifferentSession => {
            lines.push("- Destination proof: Paste into a session-ID-capable fresh session (Claude Code or Codex) whose top-level-session identity differs from the threshold session. A no-ID destination will be refused; waiting will not help.".to_owned());
            if entry.threshold_session_id.as_deref() == Some(census_session_id) {
                lines.push(format!(
                    "- This census session recorded the threshold (`{census_session_id}`), so it cannot be the destination; Skill Evolution would refuse here with `refused_cooldown_or_same_session`. The destination proof above still governs which session qualifies."
                ));
            }
        }
        EvolutionReadyProof::CooldownElapsed { not_before } => lines.push(format!(
            "- Destination proof: The 12-hour clock proof already passed at {} (UTC); {not_before}. A destination session ID is not required.",
            format_local_utc(not_before)
        )),
    }
    if entry.quarantined {
        lines.push("- Quarantine: Stop using this target. Immediate containment is allowed; permanent edits still require the authorized review.".to_owned());
    }
    lines.extend([
        String::new(),
        "Paste into another top-level session:".to_owned(),
        String::new(),
        "```text".to_owned(),
        entry.command.clone(),
        "```".to_owned(),
    ]);
    lines.join("\n")
}

fn explain_authorization_reason(reason: &str) -> String {
    if reason == "severe" {
        return "one contemporaneous severe incident (`severe`)".to_owned();
    }
    if reason == "ten_use_unresolved" {
        return "ten qualifying uses on the unchanged target with an open contemporaneous incident (`ten_use_unresolved`)".to_owned();
    }
    if let Some(symptom) = reason.strip_prefix("material_recurrence:") {
        return format!(
            "two independent material-failure-or-worse incidents in the `{symptom}` symptom cluster (`{reason}`)"
        );
    }
    if let Some(symptom) = reason.strip_prefix("friction_recurrence:") {
        return format!(
            "three independent friction-or-worse incidents in the `{symptom}` symptom cluster (`{reason}`)"
        );
    }
    if reason.is_empty() {
        "authorization rule unavailable".to_owned()
    } else {
        format!("gate rule `{reason}`")
    }
}

/// A target can be actionable on new evidence and still carry evidence an earlier
/// blocked close retired. Saying so here keeps the census from implying the gate holds
/// everything that was ever recorded against this target.
fn retired_line(count: usize) -> Option<String> {
    (count > 0).then(|| {
        format!(
            "- Retired as untestable: {}. An earlier `blocked_no_valid_test` close removed that evidence from this gate.",
            open_incidents(count)
        )
    })
}

fn reentry_line(basis: Option<&str>) -> Option<&'static str> {
    match basis {
        Some("first_eligibility") => Some(
            "- Gate provenance: First eligibility on this target hash; no completed same-hash review precedes this threshold.",
        ),
        Some("post_review_incident") => Some(
            "- Gate provenance: A new incident was recorded after the last completed same-hash review.",
        ),
        Some("unadjudicated_severe") => Some(
            "- Gate provenance: An earlier unadjudicated severe incident remains quarantined across the later review disposition.",
        ),
        _ => None,
    }
}

fn summarize_current_evidence(
    events: &[EvidenceEvent],
    current_hash: &str,
    open_incident_ids: &[String],
) -> CurrentEvidence {
    let current_uses = events
        .iter()
        .filter(|event| event.use_recorded().is_some() && event.target_content_hash == current_hash)
        .collect::<Vec<_>>();
    let mut outcome_counts = OutcomeCounts {
        clean: 0,
        friction: 0,
        material_failure: 0,
        severe_incident: 0,
    };
    for event in &current_uses {
        match event.use_recorded().expect("filtered use record").outcome {
            Outcome::Clean => outcome_counts.clean += 1,
            Outcome::Friction => outcome_counts.friction += 1,
            Outcome::MaterialFailure => outcome_counts.material_failure += 1,
            Outcome::SevereIncident => outcome_counts.severe_incident += 1,
        }
    }
    let open_incidents = current_uses
        .iter()
        .filter(|event| open_incident_ids.contains(&event.event_id))
        .map(|event| {
            let payload = &event.raw["payload"];
            OpenIncident {
                event_id: event.event_id.clone(),
                recorded_at: event.recorded_at.clone(),
                top_level_session_id: event.top_level_session_id.clone(),
                retrospective: payload["retrospective"].as_bool().unwrap_or(false),
                task_label: string_field(payload, "task_label"),
                task_fingerprint: string_field(payload, "task_fingerprint"),
                outcome: string_field(payload, "outcome"),
                symptom_key: optional_string_field(payload, "symptom_key"),
                expected: optional_string_field(payload, "expected"),
                observed: optional_string_field(payload, "observed"),
                consequence: optional_string_field(payload, "consequence"),
                workaround_taken: optional_string_field(payload, "workaround_taken"),
                evidence_refs: payload["evidence_refs"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect(),
            }
        })
        .collect();

    let mut observed_target_hashes = Vec::<ObservedTargetHash>::new();
    let mut positions = HashMap::<String, usize>::new();
    for event in events.iter().filter(|event| event.use_recorded().is_some()) {
        if let Some(index) = positions.get(&event.target_content_hash).copied() {
            let row = &mut observed_target_hashes[index];
            row.qualifying_uses += 1;
            row.last_recorded_at.clone_from(&event.recorded_at);
        } else {
            positions.insert(
                event.target_content_hash.clone(),
                observed_target_hashes.len(),
            );
            observed_target_hashes.push(ObservedTargetHash {
                content_hash: event.target_content_hash.clone(),
                qualifying_uses: 1,
                first_recorded_at: event.recorded_at.clone(),
                last_recorded_at: event.recorded_at.clone(),
            });
        }
    }

    CurrentEvidence {
        qualifying_uses: current_uses.len(),
        outcome_counts,
        open_incidents,
        observed_target_hashes,
    }
}

fn string_field(value: &Value, key: &str) -> String {
    value[key].as_str().unwrap_or_default().to_owned()
}

fn optional_string_field(value: &Value, key: &str) -> Option<String> {
    value[key].as_str().map(str::to_owned)
}

fn read_lineage_files(root: &Path) -> Result<Vec<LineageFile>, Error> {
    let reports = root.join("reports");
    if !reports.exists() {
        return Ok(Vec::new());
    }
    let evidence_root = reports.join("skill-evidence");
    let mut pending = vec![reports];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory).map_err(|error| {
            unsafe_failure(format!(
                "Could not read lineage directory {}: {error}",
                directory.display()
            ))
        })?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                unsafe_failure(format!(
                    "Could not read lineage entry in {}: {error}",
                    directory.display()
                ))
            })?;
            let path = entry.path();
            if path == evidence_root {
                continue;
            }
            let metadata = entry.metadata().map_err(|error| {
                unsafe_failure(format!(
                    "Could not inspect lineage path {}: {error}",
                    path.display()
                ))
            })?;
            if metadata.is_dir() {
                pending.push(path);
                continue;
            }
            let extension = path.extension().and_then(|value| value.to_str());
            if extension.is_some_and(|value| LINEAGE_EXTENSIONS.contains(&value)) {
                files.push(LineageFile {
                    path: repo_path(root, &path),
                    absolute: path,
                    size: metadata.len(),
                });
            }
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn lineage_candidates(
    files: &[LineageFile],
    target_name: &str,
    target_path: &str,
) -> Vec<LineageCandidate> {
    let lower_target_name = target_name.to_ascii_lowercase();
    let mut candidates = files
        .iter()
        .filter_map(|file| {
            let lower_name = file
                .absolute
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            let filename_match = lower_name.contains(&format!("{lower_target_name}-method-gap"));
            let content_match = !filename_match
                && file.size <= MAX_LINEAGE_BYTES
                && fs::read_to_string(&file.absolute).is_ok_and(|source| {
                    source.lines().any(|line| {
                        lineage_identifying_line(line)
                            && line.to_ascii_lowercase().contains("method-gap")
                            && (line.contains(target_path)
                                || line
                                    .to_ascii_lowercase()
                                    .contains(&format!("`{lower_target_name}`"))
                                || contains_exact_target_name(
                                    &line.to_ascii_lowercase(),
                                    &lower_target_name,
                                ))
                    })
                });
            (filename_match || content_match).then(|| {
                summarize_lineage_file(
                    file,
                    if filename_match {
                        "filename"
                    } else {
                        "content"
                    },
                )
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.path.cmp(&right.path));
    candidates
}

fn lineage_identifying_line(line: &str) -> bool {
    let line = line.trim_start();
    Regex::new(
        r"(?i)^(#{1,6}\s|[-*]\s*(?:target|audited|audit target|skill|method-gap disposition)\b|(?:target|audited|audit target|skill|method-gap disposition)\b)",
    )
    .expect("static lineage identity regex")
    .is_match(line)
}

fn contains_exact_target_name(line: &str, target_name: &str) -> bool {
    line.match_indices(target_name).any(|(index, _)| {
        let before = line[..index].chars().next_back();
        let after = line[index + target_name.len()..].chars().next();
        !before.is_some_and(is_skill_name_character) && !after.is_some_and(is_skill_name_character)
    })
}

fn is_skill_name_character(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
}

fn summarize_lineage_file(file: &LineageFile, discovery: &'static str) -> LineageCandidate {
    match fs::read(&file.absolute) {
        Ok(bytes) => {
            let source = String::from_utf8_lossy(&bytes);
            let lines = source.split('\n').collect::<Vec<_>>();
            let header_excerpt = numbered_lines(&lines, 0..lines.len().min(24));
            let signal = Regex::new(
                r"(?i)\b(status|lineage|in[- ]flight|returned|adopt(?:ed|ion)?|implement(?:ed|ation)?|land(?:ed|ing)?|defer(?:red|ral)?|reject(?:ed|ion)?|consume(?:d|r|ption)?|disposition|supersed(?:ed|ing)|reopen(?:ed|ing)?|completion)\b",
            )
            .expect("static lineage signal regex");
            let signal_indexes = lines
                .iter()
                .enumerate()
                .filter_map(|(index, line)| signal.is_match(line).then_some(index))
                .take(24)
                .collect::<Vec<_>>();
            LineageCandidate {
                path: file.path.clone(),
                discovery,
                kind: lineage_kind(&file.path),
                size_bytes: bytes.len() as u64,
                line_count: Some(lines.len()),
                sha256: Some(format!("{:x}", Sha256::digest(&bytes))),
                header_excerpt,
                signal_lines: numbered_lines(&lines, signal_indexes),
                unreadable_reason: None,
            }
        }
        Err(error) => LineageCandidate {
            path: file.path.clone(),
            discovery,
            kind: lineage_kind(&file.path),
            size_bytes: file.size,
            line_count: None,
            sha256: None,
            header_excerpt: Vec::new(),
            signal_lines: Vec::new(),
            unreadable_reason: Some(error.to_string()),
        },
    }
}

fn lineage_kind(path: &str) -> &'static str {
    let name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if name.contains("method-gap-census") {
        "census"
    } else if name.contains("method-gap-research-brief") {
        "brief"
    } else if name.contains("method-gap-research-report")
        || name.contains("method-gap-audit-report")
        || Regex::new(r"method-gap-audit\.[^.]+$")
            .expect("static returned-audit filename regex")
            .is_match(&name)
    {
        "returned_report"
    } else {
        "other"
    }
}

fn numbered_lines(lines: &[&str], indexes: impl IntoIterator<Item = usize>) -> Vec<String> {
    indexes
        .into_iter()
        .map(|index| {
            let line = lines[index];
            let text = line.chars().take(500).collect::<String>();
            let suffix = if line.chars().count() > 500 {
                " … [truncated]"
            } else {
                ""
            };
            format!("{}: {text}{suffix}", index + 1)
        })
        .collect()
}

fn git_observation(root: &Path, target_path: &str) -> GitObservation {
    let status = git_output(
        root,
        &[
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            target_path,
        ],
    );
    let log = git_output(
        root,
        &["log", "-20", "--format=%H%x09%cI%x09%s", "--", target_path],
    );
    let worktree_status = status
        .as_ref()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| nonempty_lines(&output.stdout));
    let recent_commits = log
        .as_ref()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            nonempty_lines(&output.stdout)
                .into_iter()
                .map(|line| {
                    let mut fields = line.split('\t');
                    GitCommit {
                        commit: fields.next().unwrap_or_default().to_owned(),
                        committed_at: fields.next().unwrap_or_default().to_owned(),
                        subject: fields.collect::<Vec<_>>().join("\t"),
                    }
                })
                .collect()
        });
    let unavailable = [
        git_error("status", status.as_ref()),
        git_error("log", log.as_ref()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    GitObservation {
        worktree_status,
        recent_commits,
        unavailable_reason: (!unavailable.is_empty()).then(|| unavailable.join("; ")),
    }
}

fn git_output(root: &Path, arguments: &[&str]) -> std::io::Result<std::process::Output> {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .env("LC_ALL", "C")
        .env("TZ", "UTC")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
}

fn git_error(
    label: &str,
    result: Result<&std::process::Output, &std::io::Error>,
) -> Option<String> {
    match result {
        Ok(output) if output.status.success() => None,
        Ok(output) => Some(format!(
            "{label}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )),
        Err(error) => Some(format!("{label}: {error}")),
    }
}

fn nonempty_lines(bytes: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

fn repo_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}
