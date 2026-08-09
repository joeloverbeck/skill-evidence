#![forbid(unsafe_code)]

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub mod assets;
#[cfg(feature = "cli")]
pub mod cli;
mod host;
mod status_reports;

pub use host::{Host, Recovery};
pub use status_reports::skill_evolution_status;
pub use status_reports::{MethodGapResearchInventory, method_gap_research_inventory};

pub const SESSION_UNAVAILABLE: &str = "unavailable";
const SYMPTOM_KEYS: &[&str] = &[
    "triggering",
    "execution",
    "output",
    "state",
    "tool-compatibility",
    "coordination",
    "cost",
    "unknown",
];
const DISPOSITIONS: &[&str] = &[
    "resolved_by_change",
    "closed_no_skill_defect",
    "outside_target",
    "insufficient_independence",
    "monitor_for_recurrence",
    "superseded_by_target_version",
    "candidate_rejected_validation",
    "blocked_no_valid_test",
];
const EVOLUTION_ADJUDICATING_DISPOSITIONS: &[&str] = &[
    "resolved_by_change",
    "closed_no_skill_defect",
    "outside_target",
    "insufficient_independence",
    "monitor_for_recurrence",
    "candidate_rejected_validation",
];
/// Dispositions that reach no conclusion about the skill and still retire their
/// covered evidence from the gate, because the review established that this
/// instrument cannot test it.
///
/// See [`docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`](../docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md).
/// The events stay in the stream forever and stay open in the projection; they
/// stop driving a gate whose own instrument proved unable to act on them.
const EVOLUTION_INSTRUMENT_LIMITED_DISPOSITIONS: &[&str] = &["blocked_no_valid_test"];
const USE_PAYLOAD_KEYS: &[&str] = &[
    "qualifying_use",
    "retrospective",
    "task_label",
    "task_fingerprint",
    "outcome",
    "symptom_key",
    "expected",
    "observed",
    "consequence",
    "workaround_taken",
    "evidence_refs",
    "same_run_group",
];
const USE_PAYLOAD_OPTIONAL_KEYS: &[&str] = &["run_condition"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    Refusal,
    UnsafeFailure,
}

#[derive(Debug)]
pub struct Error {
    class: ErrorClass,
    message: String,
    recovery: Option<Recovery>,
}

impl Error {
    #[must_use]
    pub fn class(&self) -> ErrorClass {
        self.class
    }

    /// The operator action that would finish what this error interrupted, when
    /// there is one.
    ///
    /// [`Display`](std::fmt::Display) deliberately omits it: rendering the
    /// action means naming a binary, and a host that prints this error knows
    /// its own name. See [`Host::recovery_instruction`].
    #[must_use]
    pub fn recovery(&self) -> Option<Recovery> {
        self.recovery
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

/// What hashing a skill directory found, before anyone decides to emit it.
///
/// Almost every caller in this crate wants only `content_hash`, to compare a
/// directory against what the evidence says it was. [`HashReport`] is the same
/// facts addressed to a reader outside the process, which is the only reason a
/// schema identity is involved at all.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DirectoryHash {
    content_hash: String,
    file_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HashReport {
    pub schema: String,
    pub content_hash: String,
    pub file_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedEventStream {
    pub events: Vec<Value>,
    pub integrity_errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationInputs {
    pub generated_at: String,
    pub now_epoch_milliseconds: i64,
    pub session_id: String,
    pub lock_owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleInputs {
    pub recorded_at: String,
    pub now_epoch_milliseconds: i64,
    pub session_id: String,
    pub lock_owner: String,
    pub operator_skill: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleEventInputs {
    pub event_id: String,
    pub recorded_at: String,
    pub now_epoch_milliseconds: i64,
    pub repository_head: String,
    pub session_id: String,
    pub lock_owner: String,
    pub operator_skill: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolutionClaimRequest {
    pub review_id: String,
    pub risk_tier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolutionValidationRequest {
    pub review_id: String,
    pub decision: String,
    pub risk_tier: String,
    pub candidate: PathBuf,
    pub trials: String,
    pub artifacts: String,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolutionLandRequest {
    pub review_id: String,
    pub candidate: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolutionCloseRequest {
    pub review_id: String,
    pub disposition: String,
    pub note: String,
    pub adjudicate: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirectoryDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandingMechanicsReceipt {
    pub after_hash: String,
    pub changed_files: DirectoryDiff,
    pub mirror_status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidatedLandingPreparation {
    candidate_hash: String,
    backup_directory: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidatedLandingPolicy<'a> {
    owner_id: &'a str,
    backup_directory: PathBuf,
    event_id: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordUseRequest {
    pub outcome: String,
    pub task_label: String,
    pub symptom_key: Option<String>,
    pub expected: Option<String>,
    pub observed: Option<String>,
    pub consequence: Option<String>,
    pub workaround: Option<String>,
    pub run_condition: Option<String>,
    pub retrospective: bool,
    pub evidence_refs: Vec<String>,
    pub same_run_group: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordInputs {
    pub event_id: String,
    pub recorded_at: String,
    pub now_epoch_milliseconds: i64,
    pub repository_head: String,
    pub session_id: String,
    pub lock_owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecordReceipt {
    pub schema: String,
    pub event_id: String,
    pub gate_status: GateStatus,
    pub terminal_reply: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CandidateCluster {
    pub symptom_key: String,
    pub open_event_ids: Vec<String>,
    pub independent_incidents: usize,
    pub max_severity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GateStatus {
    pub schema_version: u8,
    pub generated_at: String,
    pub target_content_hash: String,
    pub qualifying_uses_on_current_hash: usize,
    pub open_incident_ids: Vec<String>,
    pub candidate_clusters: Vec<CandidateCluster>,
    pub state: String,
    pub authorized_workflow: Option<String>,
    pub authorization_reason: Option<String>,
    pub trigger_event_ids: Vec<String>,
    pub threshold_session_id: Option<String>,
    pub not_before: Option<String>,
    pub active_review_id: Option<String>,
    pub last_completed_review_id: Option<String>,
    pub review_reentry_basis: Option<String>,
    pub target_name: String,
    pub target_repo_relative_path: String,
    pub derivation_session_id: Option<String>,
    /// Open incidents a review covered under an instrument-limited disposition, so
    /// they no longer cluster and can no longer reach a threshold.
    ///
    /// Omitted when empty, like `integrity_errors`: the overwhelming majority of
    /// projections have none, and every projection ever written before this field
    /// existed must keep validating.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instrument_limited_incident_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub integrity_errors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventType {
    UseRecorded,
    ReviewStarted,
    ReviewDisposition,
    ValidationCompleted,
    ChangeLanded,
    DecontaminationStarted,
    DecontaminationCompleted,
}

impl EventType {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "use_recorded" => Some(Self::UseRecorded),
            "review_started" => Some(Self::ReviewStarted),
            "review_disposition" => Some(Self::ReviewDisposition),
            "validation_completed" => Some(Self::ValidationCompleted),
            "change_landed" => Some(Self::ChangeLanded),
            "decontamination_started" => Some(Self::DecontaminationStarted),
            "decontamination_completed" => Some(Self::DecontaminationCompleted),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Clean,
    Friction,
    MaterialFailure,
    SevereIncident,
}

impl Outcome {
    const ALLOWED: &'static str = "clean|friction|material_failure|severe_incident";

    fn parse(value: &str) -> Option<Self> {
        match value {
            "clean" => Some(Self::Clean),
            "friction" => Some(Self::Friction),
            "material_failure" => Some(Self::MaterialFailure),
            "severe_incident" => Some(Self::SevereIncident),
            _ => None,
        }
    }

    fn severity(self) -> u8 {
        match self {
            Self::Clean => 0,
            Self::Friction => 1,
            Self::MaterialFailure => 2,
            Self::SevereIncident => 3,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Friction => "friction",
            Self::MaterialFailure => "material_failure",
            Self::SevereIncident => "severe_incident",
        }
    }
}

#[derive(Debug, Clone)]
struct UseRecordedEvent {
    retrospective: bool,
    task_fingerprint: String,
    same_run_group: String,
    outcome: Outcome,
    symptom_key: Option<String>,
}

#[derive(Debug, Clone)]
enum EventKind {
    UseRecorded(UseRecordedEvent),
    ReviewStarted {
        review_id: String,
    },
    ReviewDisposition {
        review_id: String,
        disposition: String,
        adjudicated_event_ids: Vec<String>,
    },
    ValidationCompleted {
        review_id: String,
    },
    ChangeLanded {
        review_id: String,
    },
    DecontaminationStarted {
        review_id: String,
    },
    DecontaminationCompleted {
        review_id: String,
    },
}

#[derive(Debug, Clone)]
struct EvidenceEvent {
    event_id: String,
    recorded_at: String,
    target_content_hash: String,
    top_level_session_id: String,
    kind: EventKind,
    raw: Value,
}

impl EvidenceEvent {
    fn from_validated(event: &Value) -> Self {
        let payload = event
            .get("payload")
            .and_then(Value::as_object)
            .expect("validated payload");
        let event_type = event
            .get("event_type")
            .and_then(Value::as_str)
            .and_then(EventType::parse)
            .expect("validated event type");
        let review_id = || {
            payload
                .get("review_id")
                .and_then(Value::as_str)
                .expect("validated review id")
                .to_owned()
        };
        let kind = match event_type {
            EventType::UseRecorded => EventKind::UseRecorded(UseRecordedEvent {
                retrospective: payload
                    .get("retrospective")
                    .and_then(Value::as_bool)
                    .expect("validated retrospective flag"),
                task_fingerprint: payload
                    .get("task_fingerprint")
                    .and_then(Value::as_str)
                    .expect("validated task fingerprint")
                    .to_owned(),
                same_run_group: payload
                    .get("same_run_group")
                    .and_then(Value::as_str)
                    .expect("validated run group")
                    .to_owned(),
                outcome: payload
                    .get("outcome")
                    .and_then(Value::as_str)
                    .and_then(Outcome::parse)
                    .expect("validated outcome"),
                symptom_key: payload
                    .get("symptom_key")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            }),
            EventType::ReviewStarted => EventKind::ReviewStarted {
                review_id: review_id(),
            },
            EventType::ReviewDisposition => EventKind::ReviewDisposition {
                review_id: review_id(),
                disposition: payload
                    .get("disposition")
                    .and_then(Value::as_str)
                    .expect("validated review disposition")
                    .to_owned(),
                adjudicated_event_ids: payload
                    .get("adjudicated_event_ids")
                    .and_then(Value::as_array)
                    .expect("validated adjudicated event identities")
                    .iter()
                    .map(|identity| {
                        identity
                            .as_str()
                            .expect("validated adjudicated event identity")
                            .to_owned()
                    })
                    .collect(),
            },
            EventType::ValidationCompleted => EventKind::ValidationCompleted {
                review_id: review_id(),
            },
            EventType::ChangeLanded => EventKind::ChangeLanded {
                review_id: review_id(),
            },
            EventType::DecontaminationStarted => EventKind::DecontaminationStarted {
                review_id: review_id(),
            },
            EventType::DecontaminationCompleted => EventKind::DecontaminationCompleted {
                review_id: review_id(),
            },
        };
        Self {
            event_id: event
                .get("event_id")
                .and_then(Value::as_str)
                .expect("validated event id")
                .to_owned(),
            recorded_at: event
                .get("recorded_at")
                .and_then(Value::as_str)
                .expect("validated event timestamp")
                .to_owned(),
            target_content_hash: event
                .pointer("/target/content_hash")
                .and_then(Value::as_str)
                .expect("validated target content hash")
                .to_owned(),
            top_level_session_id: event
                .get("top_level_session_id")
                .and_then(Value::as_str)
                .expect("validated top-level session identity")
                .to_owned(),
            kind,
            raw: event.clone(),
        }
    }

    fn review_id(&self) -> Option<&str> {
        match &self.kind {
            EventKind::UseRecorded(_) => None,
            EventKind::ReviewStarted { review_id }
            | EventKind::ReviewDisposition { review_id, .. }
            | EventKind::ValidationCompleted { review_id }
            | EventKind::ChangeLanded { review_id }
            | EventKind::DecontaminationStarted { review_id }
            | EventKind::DecontaminationCompleted { review_id } => Some(review_id),
        }
    }

    fn use_recorded(&self) -> Option<&UseRecordedEvent> {
        match &self.kind {
            EventKind::UseRecorded(recorded) => Some(recorded),
            _ => None,
        }
    }

    fn is_review_start(&self) -> bool {
        matches!(self.kind, EventKind::ReviewStarted { .. })
    }

    fn starts_ownership(&self) -> bool {
        matches!(
            self.kind,
            EventKind::ReviewStarted { .. } | EventKind::DecontaminationStarted { .. }
        )
    }

    fn terminates_ownership(&self) -> bool {
        matches!(
            self.kind,
            EventKind::ReviewDisposition { .. }
                | EventKind::ChangeLanded { .. }
                | EventKind::DecontaminationCompleted { .. }
        )
    }
}

#[derive(Debug)]
enum ThresholdReason {
    Severe,
    MaterialRecurrence(String),
    FrictionRecurrence(String),
    TenUseUnresolved,
}

impl ThresholdReason {
    fn as_string(&self) -> String {
        match self {
            Self::Severe => "severe".to_owned(),
            Self::MaterialRecurrence(symptom) => {
                format!("material_recurrence:{symptom}")
            }
            Self::FrictionRecurrence(symptom) => {
                format!("friction_recurrence:{symptom}")
            }
            Self::TenUseUnresolved => "ten_use_unresolved".to_owned(),
        }
    }

    fn is_severe(&self) -> bool {
        matches!(self, Self::Severe)
    }
}

#[derive(Debug)]
struct ThresholdTrigger {
    reason: ThresholdReason,
    trigger_event_ids: Vec<String>,
    threshold_session_id: Option<String>,
    fired_at: String,
    event_index: usize,
}

pub fn resolve_top_level_session_id(
    explicit: Option<&str>,
    claude_code_session_id: Option<&str>,
    codex_thread_id: Option<&str>,
) -> Result<String, Error> {
    if let Some(explicit) = explicit {
        return Ok(if explicit.is_empty() {
            SESSION_UNAVAILABLE.to_owned()
        } else {
            explicit.to_owned()
        });
    }
    let present = [
        ("CLAUDE_CODE_SESSION_ID", claude_code_session_id),
        ("CODEX_THREAD_ID", codex_thread_id),
    ]
    .into_iter()
    .filter_map(|(name, value)| {
        value
            .filter(|value| !value.is_empty())
            .map(|value| (name, value))
    })
    .collect::<Vec<_>>();
    if present.is_empty() {
        return Ok(SESSION_UNAVAILABLE.to_owned());
    }
    if present.iter().any(|(_, value)| *value != present[0].1) {
        let identities = present
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(refusal(format!(
            "Conflicting top-level-session identities: {identities}. Refusing to guess which host owns this top-level session; nothing was recorded or modified. Unset all but one supported session variable (or pass an explicit --session-id) and retry."
        )));
    }
    Ok(present[0].1.to_owned())
}

pub fn parse_method_gap_family_selector(selector: &str) -> Result<String, Error> {
    let stem = selector.strip_suffix("-*");
    let valid = stem.is_some_and(|stem| {
        !stem.is_empty()
            && stem
                .bytes()
                .next()
                .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            && stem
                .bytes()
                .next_back()
                .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            && stem
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    });
    if !valid {
        return Err(refusal(format!(
            "Invalid family selector {selector:?}. Use one literal skill-name prefix followed by a terminal \"*\", for example \"game-*\"."
        )));
    }
    Ok(format!("{}-", stem.expect("validated family stem")))
}

pub fn hash_skill(root: &Path, target: &Path, host: &Host) -> Result<HashReport, Error> {
    let target = target_context(root, target)?;
    let hash = hash_target_directory(&target.target_real)?;
    Ok(HashReport {
        schema: host.hash_schema(),
        content_hash: hash.content_hash,
        file_count: hash.file_count,
    })
}

pub fn skill_key_for_target(root: &Path, target: &Path) -> Result<String, Error> {
    let rooted_target = if target.is_absolute() {
        target.to_owned()
    } else {
        root.join(target)
    };
    let target = target_context(root, &rooted_target)?;
    target
        .evidence_directory
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .ok_or_else(|| unsafe_failure("Derived evidence-store key is not valid UTF-8.".to_owned()))
}

pub fn read_validated_event_stream(path: &Path) -> Result<ValidatedEventStream, Error> {
    let (events, integrity_errors) = read_event_stream(path)?;
    Ok(ValidatedEventStream {
        events: events.into_iter().map(|event| event.raw).collect(),
        integrity_errors,
    })
}

pub fn derive_store(
    root: &Path,
    target: &Path,
    inputs: &DerivationInputs,
) -> Result<GateStatus, Error> {
    validate_derivation_inputs(inputs)?;
    let target = target_context(root, target)?;
    fs::create_dir_all(&target.evidence_directory).map_err(|error| {
        unsafe_failure(format!(
            "Could not create evidence directory {}: {error}",
            target.evidence_directory.display()
        ))
    })?;
    let _lock = EvidenceLock::acquire(&target.evidence_directory, &inputs.lock_owner)?;
    let hash = hash_target_directory(&target.target_real)?;
    let events_path = target.evidence_directory.join("events.jsonl");
    let (events, integrity_errors) = read_event_stream(&events_path)?;
    let status = derive_gate(
        &target,
        &hash.content_hash,
        &events,
        integrity_errors,
        inputs,
    );
    write_gate_status(&target.evidence_directory, &status)?;
    Ok(status)
}

pub fn evolution_preflight(
    root: &Path,
    target: &Path,
    inputs: &LifecycleInputs,
) -> Result<Value, Error> {
    let derivation_inputs = lifecycle_derivation_inputs(inputs)?;
    let target = lifecycle_target_context(root, target, &inputs.operator_skill)?;
    fs::create_dir_all(&target.evidence_directory).map_err(|error| {
        unsafe_failure(format!(
            "Could not create evidence directory {}: {error}",
            target.evidence_directory.display()
        ))
    })?;
    let _lock = EvidenceLock::acquire(&target.evidence_directory, &inputs.lock_owner)?;
    let (events, hash, status) = authorize_evolution(&target, &derivation_inputs)?;
    Ok(serde_json::json!({
        "authorized": true,
        "target": {
            "name": target.target_name,
            "repo_relative_path": target.repo_relative_path,
            "content_hash": hash.content_hash
        },
        "evidence_dir": target
            .evidence_directory
            .strip_prefix(&target.repository_root)
            .unwrap_or(&target.evidence_directory)
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/"),
        "gate": {
            "state": status.state,
            "authorization_reason": status.authorization_reason,
            "trigger_event_ids": status.trigger_event_ids,
            "threshold_session_id": status.threshold_session_id
        },
        "evidence_packet": evolution_evidence_packet(&events, &status)
    }))
}

pub fn evolution_claim(
    root: &Path,
    target: &Path,
    request: &EvolutionClaimRequest,
    inputs: &LifecycleEventInputs,
) -> Result<Value, Error> {
    if !matches!(
        request.risk_tier.as_str(),
        "provisional" | "ordinary" | "high"
    ) {
        return Err(refusal(
            "--risk-tier must be one of provisional|ordinary|high".to_owned(),
        ));
    }
    if request.review_id.is_empty() {
        return Err(refusal("Missing required --review-id.".to_owned()));
    }
    let derivation_inputs = lifecycle_event_derivation_inputs(inputs)?;
    let target = lifecycle_target_context(root, target, &inputs.operator_skill)?;
    fs::create_dir_all(&target.evidence_directory).map_err(|error| {
        unsafe_failure(format!(
            "Could not create evidence directory {}: {error}",
            target.evidence_directory.display()
        ))
    })?;
    let _lock = EvidenceLock::acquire(&target.evidence_directory, &inputs.lock_owner)?;
    let (mut events, hash, status) = authorize_evolution(&target, &derivation_inputs)?;
    let proof = status.threshold_session_id.as_ref().map_or_else(
        || {
            serde_json::json!({
                "type": "cooldown_elapsed",
                "not_before": status.not_before,
                "claimed_at": inputs.recorded_at
            })
        },
        |threshold_session_id| {
            serde_json::json!({
                "type": "different_session",
                "threshold_session_id": threshold_session_id,
                "review_session_id": inputs.session_id
            })
        },
    );
    let event = lifecycle_event(
        &target,
        &hash.content_hash,
        "skill-evolution",
        "review_started",
        serde_json::json!({
            "review_id": request.review_id,
            "target_hash": hash.content_hash,
            "trigger_event_ids": status.trigger_event_ids,
            "authorizing_rule": status.authorization_reason,
            "risk_tier": request.risk_tier,
            "session_or_cooldown_proof": proof
        }),
        inputs,
    );
    let after = append_lifecycle_event(
        &target,
        &hash.content_hash,
        &mut events,
        event,
        &derivation_inputs,
    )?;
    if after.active_review_id.as_deref() != Some(request.review_id.as_str()) {
        return Err(unsafe_failure(format!(
            "Claim appended but another review owns the target ({}). Stop without semantic analysis.",
            after.active_review_id.as_deref().unwrap_or("none")
        )));
    }
    Ok(serde_json::json!({
        "review_id": request.review_id,
        "state": after.state,
        "target_hash": hash.content_hash,
        "authorizing_rule": status.authorization_reason,
        "trigger_event_ids": status.trigger_event_ids,
        "risk_tier": request.risk_tier,
        "evidence_dir": evidence_relative_path(&target)
    }))
}

pub fn evolution_record_validation(
    root: &Path,
    target: &Path,
    request: &EvolutionValidationRequest,
    inputs: &LifecycleEventInputs,
) -> Result<Value, Error> {
    if request.review_id.is_empty() {
        return Err(refusal("Missing required --review-id.".to_owned()));
    }
    if !matches!(request.decision.as_str(), "accepted" | "rejected") {
        return Err(refusal("--decision must be accepted|rejected.".to_owned()));
    }
    if !matches!(request.risk_tier.as_str(), "ordinary" | "high") {
        return Err(refusal(
            "--risk-tier must be final: ordinary|high".to_owned(),
        ));
    }
    let trial_count = request
        .trials
        .parse::<usize>()
        .ok()
        .filter(|count| *count >= 1)
        .ok_or_else(|| refusal("--trials must be a positive integer.".to_owned()))?;
    let minimum = if request.risk_tier == "high" { 5 } else { 3 };
    if request.decision == "accepted" && trial_count < minimum {
        return Err(refusal(format!(
            "An accepted {} change requires at least {minimum} paired trials; got {trial_count}.",
            request.risk_tier
        )));
    }
    if request.artifacts.is_empty() {
        return Err(refusal(
            "Missing required --artifacts <path to retained trial outputs>.".to_owned(),
        ));
    }
    let derivation_inputs = lifecycle_event_derivation_inputs(inputs)?;
    let target = lifecycle_target_context(root, target, &inputs.operator_skill)?;
    let candidate_real = resolve_target(&target.repository_root, &request.candidate)?;
    let _lock = EvidenceLock::acquire(&target.evidence_directory, &inputs.lock_owner)?;
    let mut events = read_valid_lifecycle_stream(&target)?;
    find_review_start(&events, &request.review_id)?;
    let hash = hash_target_directory(&target.target_real)?;
    require_active_ownership(
        &target,
        &events,
        &hash.content_hash,
        &request.review_id,
        &derivation_inputs,
        "Review",
    )?;
    let candidate_hash = hash_target_directory(&candidate_real)?;
    let candidate_path = candidate_real
        .strip_prefix(&target.repository_root)
        .unwrap_or(&candidate_real)
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/");
    let event = lifecycle_event(
        &target,
        &hash.content_hash,
        "skill-evolution",
        "validation_completed",
        serde_json::json!({
            "review_id": request.review_id,
            "decision": request.decision,
            "risk_tier": request.risk_tier,
            "candidate_hash": candidate_hash.content_hash,
            "candidate_path": candidate_path,
            "trial_count": trial_count,
            "artifacts_path": request.artifacts,
            "summary": request.summary
        }),
        inputs,
    );
    append_lifecycle_event(
        &target,
        &hash.content_hash,
        &mut events,
        event,
        &derivation_inputs,
    )?;
    Ok(serde_json::json!({
        "recorded": inputs.event_id,
        "decision": request.decision,
        "risk_tier": request.risk_tier,
        "candidate_hash": candidate_hash.content_hash,
        "trial_count": trial_count
    }))
}

pub fn evolution_land(
    root: &Path,
    target: &Path,
    request: &EvolutionLandRequest,
    inputs: &LifecycleEventInputs,
) -> Result<Value, Error> {
    if request.review_id.is_empty() {
        return Err(refusal("Missing required --review-id.".to_owned()));
    }
    let derivation_inputs = lifecycle_event_derivation_inputs(inputs)?;
    let target = lifecycle_target_context(root, target, &inputs.operator_skill)?;
    let candidate_real = resolve_target(&target.repository_root, &request.candidate)?;
    let _lock = EvidenceLock::acquire(&target.evidence_directory, &inputs.lock_owner)?;
    let mut events = read_valid_lifecycle_stream(&target)?;
    let review = find_review_start(&events, &request.review_id)?;
    let live_hash = hash_target_directory(&target.target_real)?.content_hash;
    require_active_ownership(
        &target,
        &events,
        &live_hash,
        &request.review_id,
        &derivation_inputs,
        "Review",
    )?;
    let baseline = review
        .raw
        .pointer("/payload/target_hash")
        .and_then(Value::as_str)
        .expect("validated review_started target hash")
        .to_owned();
    if live_hash != baseline {
        return Err(refusal(format!(
            "Live target hash {}… no longer equals the review baseline {}…; the target changed since the claim. Authorization expired — stop without landing.",
            &live_hash[..12],
            &baseline[..baseline.len().min(12)]
        )));
    }
    let preparation = prepare_validated_landing(
        &target,
        &events,
        &live_hash,
        &candidate_real,
        ValidatedLandingPolicy {
            owner_id: &request.review_id,
            backup_directory: target
                .evidence_directory
                .join("reviews")
                .join(&request.review_id)
                .join("pre-land-backup"),
            event_id: &inputs.event_id,
        },
    )?;
    let mirror = expected_mirror_path(&target);
    let landing = land_validated_candidate(
        &target.target_real,
        &candidate_real,
        &preparation.backup_directory,
        &baseline,
        &preparation.candidate_hash,
        mirror.as_deref(),
    )?;
    let event = lifecycle_event(
        &target,
        &landing.after_hash,
        "skill-evolution",
        "change_landed",
        serde_json::json!({
            "review_id": request.review_id,
            "before_hash": baseline,
            "after_hash": landing.after_hash,
            "changed_files": landing.changed_files,
            "mirror_status": landing.mirror_status
        }),
        inputs,
    );
    append_lifecycle_event(
        &target,
        &landing.after_hash,
        &mut events,
        event,
        &derivation_inputs,
    )?;
    Ok(serde_json::json!({
        "landed": true,
        "before_hash": baseline,
        "after_hash": landing.after_hash,
        "changed_files": landing.changed_files,
        "mirror_status": landing.mirror_status,
        "backup": preparation
            .backup_directory
            .strip_prefix(&target.repository_root)
            .unwrap_or(&preparation.backup_directory)
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/")
    }))
}

pub fn evolution_close(
    root: &Path,
    target: &Path,
    request: &EvolutionCloseRequest,
    inputs: &LifecycleEventInputs,
) -> Result<Value, Error> {
    if request.review_id.is_empty() {
        return Err(refusal("Missing required --review-id.".to_owned()));
    }
    if !DISPOSITIONS.contains(&request.disposition.as_str()) {
        return Err(refusal(format!(
            "--disposition must be one of {}",
            DISPOSITIONS.join("|")
        )));
    }
    if request.note.is_empty() {
        return Err(refusal(
            "Missing required --note: record the adjudication rationale in the immutable event."
                .to_owned(),
        ));
    }
    if !EVOLUTION_ADJUDICATING_DISPOSITIONS.contains(&request.disposition.as_str())
        && !request.adjudicate.is_empty()
    {
        return Err(refusal(format!(
            "--adjudicate is not allowed with non-adjudicating disposition {}. Nothing done.",
            request.disposition
        )));
    }
    let derivation_inputs = lifecycle_event_derivation_inputs(inputs)?;
    let target = lifecycle_target_context(root, target, &inputs.operator_skill)?;
    let _lock = EvidenceLock::acquire(&target.evidence_directory, &inputs.lock_owner)?;
    let mut events = read_valid_lifecycle_stream(&target)?;
    let review = find_review_start(&events, &request.review_id)?;
    let trigger_event_ids = review
        .raw
        .pointer("/payload/trigger_event_ids")
        .and_then(Value::as_array)
        .expect("validated review trigger identities")
        .iter()
        .map(|identity| {
            identity
                .as_str()
                .expect("validated review trigger identity")
                .to_owned()
        })
        .collect::<Vec<_>>();
    if events.iter().any(|event| {
        matches!(event.kind, EventKind::ReviewDisposition { .. })
            && event.review_id() == Some(request.review_id.as_str())
    }) {
        return Err(refusal(format!(
            "Review {} already has a review_disposition. Nothing done.",
            request.review_id
        )));
    }
    let hash = hash_target_directory(&target.target_real)?.content_hash;
    let landed = events.iter().any(|event| {
        matches!(event.kind, EventKind::ChangeLanded { .. })
            && event.review_id() == Some(request.review_id.as_str())
    });
    let before = if landed {
        if request.disposition != "resolved_by_change" {
            return Err(refusal(
                "A change already landed for this review; the only valid disposition is resolved_by_change."
                    .to_owned(),
            ));
        }
        None
    } else {
        let before = require_active_ownership(
            &target,
            &events,
            &hash,
            &request.review_id,
            &derivation_inputs,
            "Review",
        )?;
        if request.disposition == "resolved_by_change" {
            return Err(refusal(
                "resolved_by_change requires a change_landed event for this review; land first or pick a no-change disposition."
                    .to_owned(),
            ));
        }
        Some(before)
    };
    if request.disposition == "candidate_rejected_validation" {
        let latest = events.iter().rfind(|event| {
            matches!(event.kind, EventKind::ValidationCompleted { .. })
                && event.review_id() == Some(request.review_id.as_str())
        });
        if latest.and_then(|event| {
            event
                .raw
                .pointer("/payload/decision")
                .and_then(Value::as_str)
        }) != Some("rejected")
        {
            return Err(refusal(
                "candidate_rejected_validation requires the latest validation_completed for this review to record decision=rejected."
                    .to_owned(),
            ));
        }
    }
    let known = events
        .iter()
        .map(|event| event.event_id.as_str())
        .collect::<HashSet<_>>();
    for identity in &request.adjudicate {
        if !known.contains(identity.as_str()) {
            return Err(refusal(format!(
                "--adjudicate references unknown event_id {identity}. Nothing done."
            )));
        }
    }
    let mut adjudicated = trigger_event_ids;
    for identity in &request.adjudicate {
        if !adjudicated.contains(identity) {
            adjudicated.push(identity.clone());
        }
    }
    let event = lifecycle_event(
        &target,
        &hash,
        "skill-evolution",
        "review_disposition",
        serde_json::json!({
            "review_id": request.review_id,
            "disposition": request.disposition,
            "adjudicated_event_ids": adjudicated,
            "note": request.note
        }),
        inputs,
    );
    let after = append_lifecycle_event(&target, &hash, &mut events, event, &derivation_inputs)?;
    let mut receipt = serde_json::json!({
        "closed": request.review_id,
        "disposition": request.disposition,
        "adjudicated_event_ids": adjudicated,
        "state": after.state
    });
    if EVOLUTION_INSTRUMENT_LIMITED_DISPOSITIONS.contains(&request.disposition.as_str()) {
        let previously_retired = before
            .as_ref()
            .map(|status| {
                status
                    .instrument_limited_incident_ids
                    .iter()
                    .map(String::as_str)
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();
        let retirement_reach = after
            .instrument_limited_incident_ids
            .iter()
            .filter(|identity| !previously_retired.contains(identity.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        receipt["retired_from_gate_event_ids"] = serde_json::json!(retirement_reach);
    }
    Ok(receipt)
}

fn authorize_evolution(
    target: &TargetContext,
    inputs: &DerivationInputs,
) -> Result<(Vec<EvidenceEvent>, DirectoryHash, GateStatus), Error> {
    let hash = hash_target_directory(&target.target_real)?;
    let (events, integrity_errors) =
        read_event_stream(&target.evidence_directory.join("events.jsonl"))?;
    let status = derive_gate(
        target,
        &hash.content_hash,
        &events,
        integrity_errors,
        inputs,
    );
    write_gate_status(&target.evidence_directory, &status)?;
    if status.state == "blocked" {
        return Err(evolution_refusal(
            "blocked",
            "event_stream_integrity_valid",
            "refused_closed_gate",
            status.instrument_limited_incident_ids.len(),
        ));
    }
    if status.state == "review_in_progress" {
        return Err(evolution_refusal(
            "review_in_progress",
            &format!(
                "gate_status.active_review_id IS null (active: {})",
                status.active_review_id.as_deref().unwrap_or("none")
            ),
            "refused_closed_gate",
            status.instrument_limited_incident_ids.len(),
        ));
    }
    if matches!(
        status.state.as_str(),
        "eligible_pending_cooldown" | "quarantined_pending_cooldown"
    ) {
        return Err(evolution_refusal(
            &status.state,
            "cooldown_or_different_session_condition_passed",
            "refused_cooldown_or_same_session",
            status.instrument_limited_incident_ids.len(),
        ));
    }
    if status.authorized_workflow.as_deref() != Some("skill-evolution")
        || !matches!(status.state.as_str(), "eligible" | "quarantined_eligible")
    {
        return Err(evolution_refusal(
            &status.state,
            "authorized_workflow == \"skill-evolution\" AND state IN {eligible, quarantined_eligible}",
            "refused_closed_gate",
            status.instrument_limited_incident_ids.len(),
        ));
    }
    if status.target_content_hash != hash.content_hash {
        return Err(evolution_refusal(
            &status.state,
            "current_target_content_hash == gate_status.target_content_hash",
            "refused_closed_gate",
            status.instrument_limited_incident_ids.len(),
        ));
    }
    Ok((events, hash, status))
}

fn read_valid_lifecycle_stream(target: &TargetContext) -> Result<Vec<EvidenceEvent>, Error> {
    let (events, integrity_errors) =
        read_event_stream(&target.evidence_directory.join("events.jsonl"))?;
    if !integrity_errors.is_empty() {
        return Err(unsafe_failure(format!(
            "Event stream integrity failure — nothing done:\n  {}",
            integrity_errors.join("\n  ")
        )));
    }
    Ok(events)
}

fn find_review_start<'a>(
    events: &'a [EvidenceEvent],
    review_id: &str,
) -> Result<&'a EvidenceEvent, Error> {
    events
        .iter()
        .find(|event| {
            matches!(event.kind, EventKind::ReviewStarted { .. })
                && event.review_id() == Some(review_id)
        })
        .ok_or_else(|| {
            refusal(format!(
                "No review_started event found for review {review_id}. Nothing done."
            ))
        })
}

fn require_active_ownership(
    target: &TargetContext,
    events: &[EvidenceEvent],
    current_hash: &str,
    expected_id: &str,
    inputs: &DerivationInputs,
    label: &str,
) -> Result<GateStatus, Error> {
    let status = derive_gate(target, current_hash, events, Vec::new(), inputs);
    if status.active_review_id.as_deref() != Some(expected_id) {
        return Err(refusal(format!(
            "{label} {expected_id} does not own the target (active review: {}). Nothing done.",
            status.active_review_id.as_deref().unwrap_or("none")
        )));
    }
    Ok(status)
}

fn evolution_evidence_packet(events: &[EvidenceEvent], status: &GateStatus) -> Value {
    let by_id = events
        .iter()
        .map(|event| (event.event_id.as_str(), event))
        .collect::<HashMap<_, _>>();
    let triggers = status
        .trigger_event_ids
        .iter()
        .filter_map(|identity| by_id.get(identity.as_str()).copied())
        .collect::<Vec<_>>();
    let trigger_keys = triggers
        .iter()
        .filter_map(|event| {
            event
                .raw
                .pointer("/payload/symptom_key")
                .and_then(Value::as_str)
        })
        .collect::<HashSet<_>>();
    let related_prior_dispositions = events
        .iter()
        .filter(|event| {
            matches!(event.kind, EventKind::ReviewDisposition { .. })
                && event
                    .raw
                    .pointer("/payload/adjudicated_event_ids")
                    .and_then(Value::as_array)
                    .is_some_and(|identities| {
                        identities.iter().any(|identity| {
                            identity
                                .as_str()
                                .and_then(|identity| by_id.get(identity).copied())
                                .and_then(|adjudicated| {
                                    adjudicated
                                        .raw
                                        .pointer("/payload/symptom_key")
                                        .and_then(Value::as_str)
                                })
                                .is_some_and(|key| trigger_keys.contains(key))
                        })
                    })
        })
        .map(|event| event.raw.clone())
        .collect::<Vec<_>>();
    let review_hashes = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ReviewStarted { review_id } => Some((
                review_id.as_str(),
                event
                    .raw
                    .pointer("/payload/target_hash")
                    .and_then(Value::as_str)
                    .unwrap_or(event.target_content_hash.as_str()),
            )),
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    let prior_reviews = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ReviewDisposition {
                review_id,
                adjudicated_event_ids,
                ..
            } => Some(serde_json::json!({
                "review_id": review_id,
                "disposition": event.raw.pointer("/payload/disposition").and_then(Value::as_str),
                "same_target_hash": review_hashes.get(review_id.as_str()).copied()
                    == Some(status.target_content_hash.as_str()),
                "note": event.raw.pointer("/payload/note").and_then(Value::as_str),
                "adjudicated_event_ids": adjudicated_event_ids,
                "report": format!("reviews/{review_id}.md")
            })),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut cited_evidence_refs = Vec::new();
    for reference in triggers.iter().flat_map(|event| {
        event
            .raw
            .pointer("/payload/evidence_refs")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
    }) {
        if !cited_evidence_refs.contains(&reference) {
            cited_evidence_refs.push(reference);
        }
    }
    serde_json::json!({
        "trigger_events": triggers.iter().map(|event| &event.raw).collect::<Vec<_>>(),
        "qualifying_uses_on_current_hash": status.qualifying_uses_on_current_hash,
        "open_incident_ids": status.open_incident_ids,
        // Without this the packet shows more open incidents than its clusters account
        // for, and the reviewer — told to work from the packet and not the ledger — has
        // to rediscover the reason from `prior_reviews`.
        "instrument_limited_incident_ids": status.instrument_limited_incident_ids,
        "candidate_clusters": status.candidate_clusters,
        "prior_reviews": prior_reviews,
        "related_prior_dispositions": related_prior_dispositions,
        "cited_evidence_refs": cited_evidence_refs
    })
}

fn validate_derivation_inputs(inputs: &DerivationInputs) -> Result<(), Error> {
    if OffsetDateTime::parse(&inputs.generated_at, &Rfc3339).is_err() {
        return Err(refusal(
            "The explicit derivation clock is not an RFC 3339 timestamp.".to_owned(),
        ));
    }
    for (label, value) in [
        ("session identity", inputs.session_id.as_str()),
        ("lock owner identity", inputs.lock_owner.as_str()),
    ] {
        if value.is_empty() {
            return Err(refusal(format!("Missing explicit {label}.")));
        }
    }
    Ok(())
}

fn lifecycle_derivation_inputs(inputs: &LifecycleInputs) -> Result<DerivationInputs, Error> {
    let derivation_inputs = DerivationInputs {
        generated_at: inputs.recorded_at.clone(),
        now_epoch_milliseconds: inputs.now_epoch_milliseconds,
        session_id: inputs.session_id.clone(),
        lock_owner: inputs.lock_owner.clone(),
    };
    validate_derivation_inputs(&derivation_inputs)?;
    Ok(derivation_inputs)
}

fn lifecycle_event_derivation_inputs(
    inputs: &LifecycleEventInputs,
) -> Result<DerivationInputs, Error> {
    let derivation_inputs = DerivationInputs {
        generated_at: inputs.recorded_at.clone(),
        now_epoch_milliseconds: inputs.now_epoch_milliseconds,
        session_id: inputs.session_id.clone(),
        lock_owner: inputs.lock_owner.clone(),
    };
    validate_derivation_inputs(&derivation_inputs)?;
    for (label, value) in [
        ("event identity", inputs.event_id.as_str()),
        ("repository identity", inputs.repository_head.as_str()),
    ] {
        if value.is_empty() {
            return Err(refusal(format!("Missing explicit {label}.")));
        }
    }
    Ok(derivation_inputs)
}

fn lifecycle_event(
    target: &TargetContext,
    target_hash: &str,
    operator_workflow: &str,
    event_type: &str,
    payload: Value,
    inputs: &LifecycleEventInputs,
) -> Value {
    serde_json::json!({
        "schema_version": 1,
        "event_id": inputs.event_id,
        "event_type": event_type,
        "recorded_at": inputs.recorded_at,
        "operator_workflow": operator_workflow,
        "target": {
            "name": target.target_name,
            "repo_relative_path": target.repo_relative_path,
            "content_hash": target_hash,
            "repo_head": inputs.repository_head
        },
        "top_level_session_id": inputs.session_id,
        "payload": payload
    })
}

fn append_lifecycle_event(
    target: &TargetContext,
    current_hash: &str,
    events: &mut Vec<EvidenceEvent>,
    event: Value,
    inputs: &DerivationInputs,
) -> Result<GateStatus, Error> {
    let seen_ids = events
        .iter()
        .map(|existing| existing.event_id.clone())
        .collect::<HashSet<_>>();
    let validation_errors = validate_event(&event, &seen_ids);
    if !validation_errors.is_empty() {
        return Err(unsafe_failure(format!(
            "Constructed event failed validation — nothing appended:\n  {}",
            validation_errors.join("\n  ")
        )));
    }
    events.push(EvidenceEvent::from_validated(&event));
    let status = derive_gate(target, current_hash, events, Vec::new(), inputs);
    let projection_temporary = target.evidence_directory.join(".gate-status.json.tmp");
    prepare_gate_status(&projection_temporary, &status)?;
    let events_path = target.evidence_directory.join("events.jsonl");
    if let Err(error) = append_lifecycle_event_line(&events_path, &event) {
        let _ = fs::remove_file(&projection_temporary);
        return Err(error);
    }
    let projection = target.evidence_directory.join("gate-status.json");
    fs::rename(&projection_temporary, &projection).map_err(|error| {
        unsafe_failure_with_recovery(
            format!(
                "Evidence event appended, but the gate projection could not be atomically replaced ({}).",
                error
            ),
            Recovery::RederiveGate,
        )
    })?;
    Ok(status)
}

fn evidence_relative_path(target: &TargetContext) -> String {
    target
        .evidence_directory
        .strip_prefix(&target.repository_root)
        .unwrap_or(&target.evidence_directory)
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}

fn evolution_refusal(
    state: &str,
    condition: &str,
    outcome: &str,
    instrument_limited_incidents: usize,
) -> Error {
    // A refusal that says only "collecting" invites the operator to wait for a gate that
    // will never open on this evidence. If a close retired some, say so here — this is the
    // surface they hit when they try to act.
    let retired = match instrument_limited_incidents {
        0 => String::new(),
        count => format!(
            "\nRetired as untestable: {count} (an earlier blocked_no_valid_test close removed that evidence from this gate)."
        ),
    };
    refusal(format!(
        "Skill Evolution not authorized.\nGate: {state}.{retired}\nFailed condition: {condition}.\nNo target analysis or modification performed.\nTerminal outcome: {outcome}."
    ))
}

pub fn record_use(
    root: &Path,
    target: &Path,
    request: &RecordUseRequest,
    inputs: &RecordInputs,
    host: &Host,
) -> Result<RecordReceipt, Error> {
    validate_record_request(request)?;
    if OffsetDateTime::parse(&inputs.recorded_at, &Rfc3339).is_err() {
        return Err(refusal(
            "The explicit record clock is not an RFC 3339 timestamp.".to_owned(),
        ));
    }
    for (label, value) in [
        ("event identity", inputs.event_id.as_str()),
        ("repository identity", inputs.repository_head.as_str()),
        ("session identity", inputs.session_id.as_str()),
        ("lock owner identity", inputs.lock_owner.as_str()),
    ] {
        if value.is_empty() {
            return Err(refusal(format!("Missing explicit {label}.")));
        }
    }

    let target = target_context(root, target)?;
    let capture_skill_directory = target
        .repository_root
        .join(".claude/skills/skill-evidence-capture");
    let is_capture_skill = if capture_skill_directory.exists() {
        capture_skill_directory.canonicalize().map_err(|error| {
            unsafe_failure(format!(
                "Could not resolve the Skill Evidence Capture directory {}: {error}",
                capture_skill_directory.display()
            ))
        })? == target.target_real
    } else {
        false
    };
    if is_capture_skill {
        if !matches!(
            Outcome::parse(&request.outcome),
            Some(Outcome::MaterialFailure | Outcome::SevereIncident)
        ) {
            return Err(refusal(
                "Self-receipts are incident-only: Skill Evidence Capture never records its own clean or friction uses. Nothing recorded."
                    .to_owned(),
            ));
        }
        if request.evidence_refs.is_empty() {
            return Err(refusal(
                "A self-targeted incident must cite concrete evidence of the failed capture attempt (--evidence-ref). Nothing recorded."
                    .to_owned(),
            ));
        }
    }

    fs::create_dir_all(&target.evidence_directory).map_err(|error| {
        unsafe_failure(format!(
            "Could not create evidence directory {}: {error}",
            target.evidence_directory.display()
        ))
    })?;
    let _lock = EvidenceLock::acquire(&target.evidence_directory, &inputs.lock_owner)?;
    let hash = hash_target_directory(&target.target_real)?;
    let events_path = target.evidence_directory.join("events.jsonl");
    let (mut events, integrity_errors) = read_event_stream(&events_path)?;
    if !integrity_errors.is_empty() {
        return Err(unsafe_failure(format!(
            "Event stream integrity failure — nothing recorded:\n  {}",
            integrity_errors.join("\n  ")
        )));
    }

    let normalized_label = request
        .task_label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let task_fingerprint = short_digest(normalized_label.as_bytes(), 16);
    let legacy_run_group = short_digest(
        format!("{}::{normalized_label}", target.target_name).as_bytes(),
        12,
    );
    let same_run_group = request.same_run_group.clone().unwrap_or_else(|| {
        short_digest(
            format!(
                "{}::{normalized_label}::{}",
                target.target_name, inputs.session_id
            )
            .as_bytes(),
            12,
        )
    });
    if let Some(duplicate) = events.iter().find(|event| {
        event.target_content_hash == hash.content_hash
            && event.use_recorded().is_some_and(|recorded| {
                recorded.same_run_group == same_run_group
                    || (recorded.same_run_group == legacy_run_group
                        && event.top_level_session_id == inputs.session_id)
            })
    }) {
        let message = if duplicate.top_level_session_id == inputs.session_id {
            format!(
                "Duplicate receipt refused: run group {same_run_group} already recorded on the unchanged target ({}). A retry or continuation of the same task is the same qualifying use; a genuinely distinct use needs a distinct --task-label.",
                duplicate.event_id
            )
        } else {
            format!(
                "Duplicate receipt refused: run group {same_run_group} already recorded on the unchanged target by event {} from different top-level session {} at {}. Use --same-run-group to declare a deliberate cross-session continuation of that group; a continuation is the same qualifying use.",
                duplicate.event_id, duplicate.top_level_session_id, duplicate.recorded_at
            )
        };
        return Err(refusal(message));
    }

    let clean = Outcome::parse(&request.outcome) == Some(Outcome::Clean);
    let event = serde_json::json!({
        "schema_version": 1,
        "event_id": inputs.event_id,
        "event_type": "use_recorded",
        "recorded_at": inputs.recorded_at,
        "operator_workflow": "skill-evidence-capture",
        "target": {
            "name": target.target_name,
            "repo_relative_path": target.repo_relative_path,
            "content_hash": hash.content_hash,
            "repo_head": inputs.repository_head
        },
        "top_level_session_id": inputs.session_id,
        "payload": {
            "qualifying_use": true,
            "retrospective": request.retrospective,
            "task_label": request.task_label,
            "task_fingerprint": task_fingerprint,
            "outcome": request.outcome,
            "symptom_key": if clean { None::<&str> } else { request.symptom_key.as_deref() },
            "expected": if clean { None::<&str> } else { request.expected.as_deref() },
            "observed": if clean { None::<&str> } else { request.observed.as_deref() },
            "consequence": if clean { None::<&str> } else { request.consequence.as_deref() },
            "workaround_taken": if clean { None::<&str> } else { request.workaround.as_deref() },
            "run_condition": if clean { None::<&str> } else { request.run_condition.as_deref() },
            "evidence_refs": request.evidence_refs,
            "same_run_group": same_run_group
        }
    });
    let seen_ids = events
        .iter()
        .map(|existing| existing.event_id.clone())
        .collect::<HashSet<_>>();
    let validation_errors = validate_event(&event, &seen_ids);
    if !validation_errors.is_empty() {
        return Err(refusal(format!(
            "Constructed event is invalid: {}",
            validation_errors.join("; ")
        )));
    }

    events.push(EvidenceEvent::from_validated(&event));
    let derivation_inputs = DerivationInputs {
        generated_at: inputs.recorded_at.clone(),
        now_epoch_milliseconds: inputs.now_epoch_milliseconds,
        session_id: inputs.session_id.clone(),
        lock_owner: inputs.lock_owner.clone(),
    };
    let status = derive_gate(
        &target,
        &hash.content_hash,
        &events,
        Vec::new(),
        &derivation_inputs,
    );
    let projection_temporary = target.evidence_directory.join(".gate-status.json.tmp");
    prepare_gate_status(&projection_temporary, &status)?;
    if let Err(error) = append_event_line(&events_path, &event) {
        let _ = fs::remove_file(&projection_temporary);
        return Err(error);
    }
    let projection = target.evidence_directory.join("gate-status.json");
    fs::rename(&projection_temporary, &projection).map_err(|error| {
        unsafe_failure_with_recovery(
            format!(
                "Evidence recorded: {}, but the gate projection could not be atomically replaced ({}).",
                inputs.event_id, error
            ),
            Recovery::RederiveGate,
        )
    })?;
    let terminal_reply = build_reply(&inputs.event_id, &status);
    Ok(RecordReceipt {
        schema: host.record_schema(),
        event_id: inputs.event_id.clone(),
        gate_status: status,
        terminal_reply,
    })
}

fn validate_record_request(request: &RecordUseRequest) -> Result<(), Error> {
    if Outcome::parse(&request.outcome).is_none() {
        return Err(refusal(format!(
            "--outcome must be one of {}",
            Outcome::ALLOWED
        )));
    }
    if request.task_label.is_empty() {
        return Err(refusal("Missing required --task-label.".to_owned()));
    }
    if request.outcome == "clean" {
        for (flag, value) in [
            ("symptom-key", request.symptom_key.as_ref()),
            ("expected", request.expected.as_ref()),
            ("observed", request.observed.as_ref()),
            ("consequence", request.consequence.as_ref()),
            ("workaround", request.workaround.as_ref()),
            ("run-condition", request.run_condition.as_ref()),
        ] {
            if value.is_some() {
                return Err(refusal(format!(
                    "--{flag} is not allowed for a clean outcome."
                )));
            }
        }
    } else {
        if request
            .symptom_key
            .as_deref()
            .is_none_or(|key| !SYMPTOM_KEYS.contains(&key))
        {
            return Err(refusal(format!(
                "--symptom-key must be one of {}",
                SYMPTOM_KEYS.join("|")
            )));
        }
        for (flag, value) in [
            ("expected", request.expected.as_deref()),
            ("observed", request.observed.as_deref()),
            ("consequence", request.consequence.as_deref()),
        ] {
            if value.is_none_or(str::is_empty) {
                return Err(refusal(format!(
                    "--{flag} is required for a non-clean outcome."
                )));
            }
        }
        if request.run_condition.as_deref().is_none_or(str::is_empty) {
            return Err(refusal(
                "--run-condition is required for a non-clean outcome: state the observable condition the use ran under (volume, elapsed length, how far from the start the failure surfaced, which gate caught it). A later review needs it to judge whether any fresh trial can reproduce this incident."
                    .to_owned(),
            ));
        }
    }
    if request.retrospective && request.evidence_refs.is_empty() {
        return Err(refusal(
            "Retrospective evidence requires a concrete recoverable reference (artifact, diff, log, transcript); memory alone is inadmissible. Nothing recorded."
                .to_owned(),
        ));
    }
    if request
        .evidence_refs
        .iter()
        .any(|reference| reference.is_empty())
    {
        return Err(refusal(
            "--evidence-ref values must be non-empty.".to_owned(),
        ));
    }
    Ok(())
}

fn short_digest(bytes: &[u8], length: usize) -> String {
    format!("{:x}", Sha256::digest(bytes))[..length].to_owned()
}

struct EvidenceLock {
    directory: PathBuf,
    owner: String,
}

impl EvidenceLock {
    fn acquire(evidence_directory: &Path, owner: &str) -> Result<Self, Error> {
        let directory = evidence_directory.join(".lock");
        let mut acquired = false;
        for _ in 0..40 {
            match fs::create_dir(&directory) {
                Ok(()) => {
                    acquired = true;
                    break;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    thread::sleep(Duration::from_millis(50));
                }
                Err(error) => {
                    return Err(unsafe_failure(format!(
                        "Could not acquire evidence lock at {}: {error}; nothing recorded.",
                        directory.display()
                    )));
                }
            }
        }
        if !acquired {
            return Err(unsafe_failure(format!(
                "Could not acquire evidence lock at {}; nothing recorded.",
                directory.display()
            )));
        }
        let owner_path = directory.join("owner");
        if let Err(error) = fs::write(&owner_path, owner) {
            let _ = fs::remove_dir(&directory);
            return Err(unsafe_failure(format!(
                "Could not establish evidence lock ownership at {}: {error}; nothing recorded.",
                directory.display()
            )));
        }
        Ok(Self {
            directory,
            owner: owner.to_owned(),
        })
    }
}

impl Drop for EvidenceLock {
    fn drop(&mut self) {
        let owner_path = self.directory.join("owner");
        let owns_lock = fs::read_to_string(&owner_path).is_ok_and(|owner| owner == self.owner);
        if owns_lock {
            let _ = fs::remove_file(owner_path);
            let _ = fs::remove_dir(&self.directory);
        }
    }
}

fn append_event_line(path: &Path, event: &Value) -> Result<(), Error> {
    let line =
        serde_json::to_vec(event).expect("serializing a validated evidence event cannot fail");
    append_serialized_line(path, line)
}

fn append_lifecycle_event_line(path: &Path, event: &Value) -> Result<(), Error> {
    append_serialized_line(path, serialize_legacy_ordered_lifecycle_event(event)?)
}

fn append_serialized_line(path: &Path, mut line: Vec<u8>) -> Result<(), Error> {
    line.push(b'\n');
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| {
            unsafe_failure(format!(
                "Could not open event stream {} for append: {error}",
                path.display()
            ))
        })?;
    file.write_all(&line).map_err(|error| {
        unsafe_failure(format!(
            "Could not append a complete event to {}: {error}",
            path.display()
        ))
    })?;
    file.sync_all().map_err(|error| {
        unsafe_failure(format!(
            "Could not synchronize appended event in {}: {error}",
            path.display()
        ))
    })
}

fn encoded_json(value: &Value) -> String {
    serde_json::to_string(value).expect("serializing a validated event field cannot fail")
}

fn ordered_json_object(parts: &[(&str, String)]) -> String {
    format!(
        "{{{}}}",
        parts
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}:{value}",
                    encoded_json(&Value::String((*key).to_owned()))
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn serialize_legacy_ordered_lifecycle_event(event: &Value) -> Result<Vec<u8>, Error> {
    let payload = &event["payload"];
    let payload = match event["event_type"].as_str() {
        Some("review_started") => {
            let proof = &payload["session_or_cooldown_proof"];
            let proof = if proof["type"] == "different_session" {
                ordered_json_object(&[
                    ("type", encoded_json(&proof["type"])),
                    (
                        "threshold_session_id",
                        encoded_json(&proof["threshold_session_id"]),
                    ),
                    (
                        "review_session_id",
                        encoded_json(&proof["review_session_id"]),
                    ),
                ])
            } else {
                ordered_json_object(&[
                    ("type", encoded_json(&proof["type"])),
                    ("not_before", encoded_json(&proof["not_before"])),
                    ("claimed_at", encoded_json(&proof["claimed_at"])),
                ])
            };
            ordered_json_object(&[
                ("review_id", encoded_json(&payload["review_id"])),
                ("target_hash", encoded_json(&payload["target_hash"])),
                (
                    "trigger_event_ids",
                    encoded_json(&payload["trigger_event_ids"]),
                ),
                (
                    "authorizing_rule",
                    encoded_json(&payload["authorizing_rule"]),
                ),
                ("risk_tier", encoded_json(&payload["risk_tier"])),
                ("session_or_cooldown_proof", proof),
            ])
        }
        Some("validation_completed") => ordered_json_object(&[
            ("review_id", encoded_json(&payload["review_id"])),
            ("decision", encoded_json(&payload["decision"])),
            ("risk_tier", encoded_json(&payload["risk_tier"])),
            ("candidate_hash", encoded_json(&payload["candidate_hash"])),
            ("candidate_path", encoded_json(&payload["candidate_path"])),
            ("trial_count", encoded_json(&payload["trial_count"])),
            ("artifacts_path", encoded_json(&payload["artifacts_path"])),
            ("summary", encoded_json(&payload["summary"])),
        ]),
        Some("change_landed") => {
            let changed = &payload["changed_files"];
            let changed = ordered_json_object(&[
                ("added", encoded_json(&changed["added"])),
                ("removed", encoded_json(&changed["removed"])),
                ("modified", encoded_json(&changed["modified"])),
            ]);
            ordered_json_object(&[
                ("review_id", encoded_json(&payload["review_id"])),
                ("before_hash", encoded_json(&payload["before_hash"])),
                ("after_hash", encoded_json(&payload["after_hash"])),
                ("changed_files", changed),
                ("mirror_status", encoded_json(&payload["mirror_status"])),
            ])
        }
        Some("review_disposition") => ordered_json_object(&[
            ("review_id", encoded_json(&payload["review_id"])),
            ("disposition", encoded_json(&payload["disposition"])),
            (
                "adjudicated_event_ids",
                encoded_json(&payload["adjudicated_event_ids"]),
            ),
            ("note", encoded_json(&payload["note"])),
        ]),
        Some(event_type) => {
            return Err(unsafe_failure(format!(
                "Cannot serialize unsupported lifecycle event type {event_type}."
            )));
        }
        None => {
            return Err(unsafe_failure(
                "Cannot serialize lifecycle event without event_type.".to_owned(),
            ));
        }
    };
    let target = &event["target"];
    let target = ordered_json_object(&[
        ("name", encoded_json(&target["name"])),
        (
            "repo_relative_path",
            encoded_json(&target["repo_relative_path"]),
        ),
        ("content_hash", encoded_json(&target["content_hash"])),
        ("repo_head", encoded_json(&target["repo_head"])),
    ]);
    Ok(ordered_json_object(&[
        ("schema_version", encoded_json(&event["schema_version"])),
        ("event_id", encoded_json(&event["event_id"])),
        ("event_type", encoded_json(&event["event_type"])),
        ("recorded_at", encoded_json(&event["recorded_at"])),
        (
            "operator_workflow",
            encoded_json(&event["operator_workflow"]),
        ),
        ("target", target),
        (
            "top_level_session_id",
            encoded_json(&event["top_level_session_id"]),
        ),
        ("payload", payload),
    ])
    .into_bytes())
}

fn build_reply(event_id: &str, status: &GateStatus) -> String {
    let head = format!("Evidence recorded: {event_id}.");
    match status.state.as_str() {
        "closed" => format!("{head}\nGate: closed.\nNo action authorized."),
        "collecting" => {
            let clusters = status
                .candidate_clusters
                .iter()
                .map(|cluster| format!("{}={}", cluster.symptom_key, cluster.independent_incidents))
                .collect::<Vec<_>>()
                .join(", ");
            // Open incidents retired by an instrument-limited close are still counted as
            // open — nothing was adjudicated — but they cluster no longer, so the symptom
            // list can now be empty while the count is not. Reporting the count without
            // saying where those incidents went reads as a gate quietly failing to
            // advance.
            let clusters = if clusters.is_empty() {
                "none".to_owned()
            } else {
                clusters
            };
            let retired = match status.instrument_limited_incident_ids.len() {
                0 => String::new(),
                count => format!(" retired as untestable: {count};"),
            };
            format!(
                "{head}\nGate: collecting — open incidents: {} (independent by symptom: {clusters});{retired} qualifying uses on current target hash: {}.\nNo action authorized.",
                status.open_incident_ids.len(),
                status.qualifying_uses_on_current_hash
            )
        }
        "eligible_pending_cooldown" => format!(
            "{head}\nGate: eligible pending fresh-session/cooldown requirement.\nSkill Evolution is not authorized in this session. No target action authorized."
        ),
        "eligible" => format!(
            "{head}\nGate: eligible for Skill Evolution after a fresh derivation in a permitted session.\nNo target action performed by Evidence Capture."
        ),
        "quarantined_pending_cooldown" | "quarantined_eligible" => format!(
            "{head}\nGate: target quarantined pending fresh Skill Evolution eligibility.\nStop using the target. Immediate operational containment is allowed; permanent skill edits are not."
        ),
        "review_in_progress" => format!(
            "{head}\nGate: review_in_progress — active review {} owns the target.\nNo action authorized.",
            status.active_review_id.as_deref().unwrap_or("unavailable")
        ),
        state => format!("{head}\nGate: {state}.\nNo action authorized."),
    }
}

#[derive(Debug)]
struct TargetContext {
    repository_root: PathBuf,
    target_real: PathBuf,
    target_name: String,
    repo_relative_path: String,
    evidence_directory: PathBuf,
}

/// Resolves the target a lifecycle command names, refusing when it is the very
/// package the running workflow is defined by.
///
/// Skill Evolution is the only workflow left that writes lifecycle events, so
/// the refusal has one shape. This took a `LifecycleWorkflow` selector while
/// Legacy Skill Decontamination also wrote them.
fn lifecycle_target_context(
    root: &Path,
    target: &Path,
    operator_skill: &Path,
) -> Result<TargetContext, Error> {
    let target = target_context(root, target)?;
    if fs::canonicalize(operator_skill)
        .ok()
        .is_some_and(|operator_real| operator_real == target.target_real)
    {
        return Err(evolution_refusal(
            "not derived (self-target)",
            "operator_skill_path != target_skill_path",
            "refused_self_target",
            0,
        ));
    }
    Ok(target)
}

fn target_context(root: &Path, target: &Path) -> Result<TargetContext, Error> {
    let root = canonical_directory(root, "repository root")?;
    let target_real = resolve_target(&root, target)?;
    let target_name = target_real
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| refusal("Target skill name is not valid UTF-8.".to_owned()))?
        .to_owned();
    let relative = target_real
        .strip_prefix(&root)
        .ok()
        .map_or_else(|| target_real.clone(), Path::to_path_buf);
    let repo_relative_path = relative
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/");
    let key = skill_key(&root, &target_real, &target_name);
    let evidence_directory = root.join("reports/skill-evidence").join(key);
    Ok(TargetContext {
        repository_root: root,
        target_real,
        target_name,
        repo_relative_path,
        evidence_directory,
    })
}

fn skill_key(root: &Path, target_real: &Path, target_name: &str) -> String {
    for skill_root in [".claude/skills", ".agents/skills"] {
        let candidate = root.join(skill_root).join(target_name);
        if candidate.exists()
            && candidate
                .canonicalize()
                .is_ok_and(|canonical| canonical != target_real)
        {
            let relative = target_real
                .strip_prefix(root)
                .unwrap_or(target_real)
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            let digest = Sha256::digest(relative.as_bytes());
            return format!("{target_name}-{:x}", digest)[..target_name.len() + 9].to_owned();
        }
    }
    target_name.to_owned()
}

fn hash_target_directory(target: &Path) -> Result<DirectoryHash, Error> {
    let files = list_files(target)?;
    let mut hasher = Sha256::new();
    for relative_path in &files {
        hasher.update(path_bytes(relative_path));
        hasher.update([0]);
        let bytes = fs::read(target.join(relative_path)).map_err(|error| {
            unsafe_failure(format!(
                "Could not read target file {}: {error}",
                relative_path.display()
            ))
        })?;
        hasher.update(bytes);
        hasher.update([0]);
    }
    Ok(DirectoryHash {
        content_hash: format!("{:x}", hasher.finalize()),
        file_count: files.len(),
    })
}

fn read_event_stream(path: &Path) -> Result<(Vec<EvidenceEvent>, Vec<String>), Error> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok((Vec::new(), Vec::new()));
        }
        Err(error) => {
            return Err(unsafe_failure(format!(
                "Could not read event stream {}: {error}",
                path.display()
            )));
        }
    };
    let text = match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => {
            return Ok((
                Vec::new(),
                vec![format!(
                    "event stream is not valid UTF-8: {}",
                    path.display()
                )],
            ));
        }
    };
    let mut events = Vec::new();
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();
    for (line_index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(line) {
            Ok(event) => {
                let event_errors = validate_event(&event, &seen_ids);
                if event_errors.is_empty() {
                    if let Some(event_id) = non_empty_string(event.get("event_id")) {
                        seen_ids.insert(event_id.to_owned());
                    }
                    events.push(EvidenceEvent::from_validated(&event));
                } else {
                    errors.extend(
                        event_errors
                            .into_iter()
                            .map(|message| format!("line {}: {message}", line_index + 1)),
                    );
                }
            }
            Err(_) => errors.push(format!("line {}: not valid JSON", line_index + 1)),
        }
    }
    Ok((events, errors))
}

fn validate_event(event: &Value, seen_ids: &HashSet<String>) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(event) = event.as_object() else {
        return vec!["event is not an object".to_owned()];
    };
    if event.get("schema_version").and_then(Value::as_u64) != Some(1) {
        errors.push("schema_version must be 1".to_owned());
    }
    match non_empty_string(event.get("event_id")) {
        None => errors.push("event_id missing".to_owned()),
        Some(event_id) if seen_ids.contains(event_id) => {
            errors.push(format!("duplicate event_id {event_id}"));
        }
        Some(_) => {}
    }
    let event_type_name = non_empty_string(event.get("event_type"));
    let event_type = event_type_name.and_then(EventType::parse);
    if event_type.is_none() {
        errors.push(format!(
            "unknown event_type {}",
            event
                .get("event_type")
                .map_or_else(|| "null".to_owned(), Value::to_string)
        ));
    }
    let timestamp_valid = non_empty_string(event.get("recorded_at"))
        .is_some_and(|timestamp| OffsetDateTime::parse(timestamp, &Rfc3339).is_ok());
    if !timestamp_valid {
        errors.push("recorded_at is not a parseable timestamp".to_owned());
    }
    if non_empty_string(event.get("operator_workflow")).is_none() {
        errors.push("operator_workflow missing".to_owned());
    }
    match event.get("target").and_then(Value::as_object) {
        None => errors.push("target missing".to_owned()),
        Some(target) => {
            for key in ["name", "repo_relative_path", "content_hash", "repo_head"] {
                if non_empty_string(target.get(key)).is_none() {
                    errors.push(format!("target.{key} missing"));
                }
            }
        }
    }
    if non_empty_string(event.get("top_level_session_id")).is_none() {
        errors.push("top_level_session_id missing".to_owned());
    }
    let Some(payload) = event.get("payload").and_then(Value::as_object) else {
        errors.push("payload missing".to_owned());
        return errors;
    };

    if event_type == Some(EventType::UseRecorded) {
        let missing = USE_PAYLOAD_KEYS
            .iter()
            .copied()
            .filter(|key| !payload.contains_key(*key))
            .collect::<Vec<_>>();
        let unknown = payload
            .keys()
            .filter(|key| {
                !USE_PAYLOAD_KEYS.contains(&key.as_str())
                    && !USE_PAYLOAD_OPTIONAL_KEYS.contains(&key.as_str())
            })
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() || !unknown.is_empty() {
            let mut message = format!(
                "use_recorded payload keys must be [{}] plus any of [{}]",
                USE_PAYLOAD_KEYS.join(", "),
                USE_PAYLOAD_OPTIONAL_KEYS.join(", ")
            );
            if !missing.is_empty() {
                message.push_str(&format!("; missing: {}", missing.join(", ")));
            }
            if !unknown.is_empty() {
                message.push_str(&format!("; unknown: {}", unknown.join(", ")));
            }
            errors.push(message);
            return errors;
        }
        if payload.get("qualifying_use").and_then(Value::as_bool) != Some(true) {
            errors.push("qualifying_use must be true".to_owned());
        }
        if payload
            .get("retrospective")
            .and_then(Value::as_bool)
            .is_none()
        {
            errors.push("retrospective must be boolean".to_owned());
        }
        for key in ["task_label", "task_fingerprint", "same_run_group"] {
            if non_empty_string(payload.get(key)).is_none() {
                errors.push(format!("{key} missing"));
            }
        }
        let outcome = non_empty_string(payload.get("outcome")).and_then(Outcome::parse);
        if outcome.is_none() {
            errors.push(format!("outcome must be one of {}", Outcome::ALLOWED));
        }
        let evidence_refs = payload.get("evidence_refs").and_then(Value::as_array);
        if evidence_refs.is_none_or(|references| {
            references
                .iter()
                .any(|reference| non_empty_string(Some(reference)).is_none())
        }) {
            errors.push("evidence_refs must be an array of non-empty strings".to_owned());
        } else if payload.get("retrospective").and_then(Value::as_bool) == Some(true)
            && evidence_refs.is_some_and(Vec::is_empty)
        {
            errors.push("retrospective events require at least one evidence_ref".to_owned());
        }
        if outcome == Some(Outcome::Clean) {
            for key in [
                "symptom_key",
                "expected",
                "observed",
                "consequence",
                "workaround_taken",
            ] {
                if payload.get(key).is_some_and(|value| !value.is_null()) {
                    errors.push(format!("{key} must be null for a clean outcome"));
                }
            }
            if payload
                .get("run_condition")
                .is_some_and(|value| !value.is_null())
            {
                errors.push("run_condition must be null for a clean outcome".to_owned());
            }
        } else {
            if non_empty_string(payload.get("symptom_key"))
                .is_none_or(|key| !SYMPTOM_KEYS.contains(&key))
            {
                errors.push(format!(
                    "symptom_key must be one of {}",
                    SYMPTOM_KEYS.join("|")
                ));
            }
            for key in ["expected", "observed", "consequence"] {
                if non_empty_string(payload.get(key)).is_none() {
                    errors.push(format!("{key} required for a non-clean outcome"));
                }
            }
            for key in ["workaround_taken", "run_condition"] {
                if payload.get(key).is_some_and(|value| {
                    !value.is_null() && non_empty_string(Some(value)).is_none()
                }) {
                    errors.push(format!("{key} must be null or a non-empty string"));
                }
            }
        }
    } else if event_type == Some(EventType::ReviewDisposition) {
        if non_empty_string(payload.get("review_id")).is_none() {
            errors.push("review_id missing".to_owned());
        }
        if non_empty_string(payload.get("disposition"))
            .is_none_or(|disposition| !DISPOSITIONS.contains(&disposition))
        {
            errors.push(format!(
                "disposition must be one of {}",
                DISPOSITIONS.join("|")
            ));
        }
        if payload
            .get("adjudicated_event_ids")
            .and_then(Value::as_array)
            .is_none_or(|identities| {
                identities.is_empty()
                    || identities
                        .iter()
                        .any(|identity| non_empty_string(Some(identity)).is_none())
            })
        {
            errors.push("adjudicated_event_ids must be a non-empty array of event ids".to_owned());
        }
    } else if non_empty_string(payload.get("review_id")).is_none() {
        errors.push(format!(
            "{} payload requires review_id",
            event_type_name.unwrap_or("unknown")
        ));
    }
    errors
}

fn non_empty_string(value: Option<&Value>) -> Option<&str> {
    value
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
}

fn derive_gate(
    target: &TargetContext,
    current_hash: &str,
    events: &[EvidenceEvent],
    integrity_errors: Vec<String>,
    inputs: &DerivationInputs,
) -> GateStatus {
    let mut status = GateStatus {
        schema_version: 1,
        generated_at: inputs.generated_at.clone(),
        target_content_hash: current_hash.to_owned(),
        qualifying_uses_on_current_hash: 0,
        open_incident_ids: Vec::new(),
        candidate_clusters: Vec::new(),
        state: "closed".to_owned(),
        authorized_workflow: None,
        authorization_reason: None,
        trigger_event_ids: Vec::new(),
        threshold_session_id: None,
        not_before: None,
        active_review_id: None,
        last_completed_review_id: None,
        review_reentry_basis: None,
        target_name: target.target_name.clone(),
        target_repo_relative_path: target.repo_relative_path.clone(),
        derivation_session_id: (inputs.session_id != "unavailable")
            .then(|| inputs.session_id.clone()),
        instrument_limited_incident_ids: Vec::new(),
        integrity_errors,
    };
    if !status.integrity_errors.is_empty() {
        status.state = "blocked".to_owned();
        return status;
    }
    let adjudicated = events
        .iter()
        .flat_map(|event| match &event.kind {
            EventKind::ReviewDisposition {
                disposition,
                adjudicated_event_ids,
                ..
            } if EVOLUTION_ADJUDICATING_DISPOSITIONS.contains(&disposition.as_str()) => {
                adjudicated_event_ids.iter().map(String::as_str)
            }
            _ => [].iter().map(String::as_str),
        })
        .collect::<HashSet<_>>();
    let recorded_symptoms = events
        .iter()
        .filter_map(|event| {
            let symptom = event.use_recorded()?.symptom_key.as_deref()?;
            Some((event.event_id.as_str(), symptom))
        })
        .collect::<HashMap<_, _>>();
    let review_starts = events
        .iter()
        .filter(|event| event.is_review_start())
        .map(|event| (event.review_id().expect("review start identity"), event))
        .collect::<Vec<_>>();
    // The watermark and the retirement scan both key on this, and they have to agree: a
    // close that lays a watermark is exactly a close whose findings are about the target
    // as it stands now.
    let ran_on_current_hash = |review_id: &str| {
        review_starts.iter().any(|(started_id, start)| {
            *started_id == review_id && start.target_content_hash == current_hash
        })
    };
    let mut last_same_hash_disposition_index = None;
    let mut last_same_hash_close_was_instrument_limited = false;
    for (index, event) in events.iter().enumerate() {
        let EventKind::ReviewDisposition { disposition, .. } = &event.kind else {
            continue;
        };
        let review_id = event.review_id().expect("review disposition identity");
        if ran_on_current_hash(review_id) {
            last_same_hash_disposition_index = Some(index);
            last_same_hash_close_was_instrument_limited =
                EVOLUTION_INSTRUMENT_LIMITED_DISPOSITIONS.contains(&disposition.as_str());
        }
    }
    // What an instrument-limited close retires, and how far.
    //
    // The trigger list is frozen when the threshold fires, so incidents that arrive while
    // the review runs are never in it — and a `material_recurrence` list never contains
    // the cluster's merely-frictional siblings at all. Those share the symptom whose
    // binding constraint the instrument could not vary, so retiring only the listed IDs
    // would leave the next review a lower bar than the first one faced, permanently in the
    // friction-sibling case. Retirement therefore reaches the whole covered cluster, and
    // stops at the close: evidence recorded afterwards is new, and drives the gate.
    //
    // Restricted to closes whose review ran against the current hash. A finding about what
    // this instrument cannot test says nothing about a target that has since changed.
    let instrument_limited_closes = events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| {
            let EventKind::ReviewDisposition {
                disposition,
                adjudicated_event_ids,
                ..
            } = &event.kind
            else {
                return None;
            };
            if !EVOLUTION_INSTRUMENT_LIMITED_DISPOSITIONS.contains(&disposition.as_str()) {
                return None;
            }
            if !ran_on_current_hash(event.review_id().expect("review disposition identity")) {
                return None;
            }
            let covered = adjudicated_event_ids
                .iter()
                .map(String::as_str)
                .collect::<HashSet<_>>();
            let symptoms = covered
                .iter()
                .filter_map(|identity| recorded_symptoms.get(identity).copied())
                .collect::<HashSet<_>>();
            Some((index, covered, symptoms))
        })
        .collect::<Vec<_>>();
    let started = events
        .iter()
        .filter(|event| event.starts_ownership())
        .collect::<Vec<_>>();
    let terminated = events
        .iter()
        .filter(|event| event.terminates_ownership())
        .filter_map(|event| event.review_id())
        .collect::<HashSet<_>>();
    let active_starts = started
        .iter()
        .filter(|event| {
            event
                .review_id()
                .is_some_and(|review_id| !terminated.contains(review_id))
        })
        .copied()
        .collect::<Vec<_>>();
    let completed_starts = started
        .iter()
        .filter(|event| {
            event
                .review_id()
                .is_some_and(|review_id| terminated.contains(review_id))
        })
        .copied()
        .collect::<Vec<_>>();
    status.active_review_id = active_starts
        .last()
        .and_then(|event| event.review_id())
        .map(str::to_owned);
    status.last_completed_review_id = completed_starts
        .last()
        .and_then(|event| event.review_id())
        .map(str::to_owned);
    let event_positions = events
        .iter()
        .enumerate()
        .map(|(index, event)| (event.event_id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let mut cluster_events = Vec::<(String, Vec<&EvidenceEvent>)>::new();
    let mut fired = None;
    let mut queued_pre_close_evidence = false;
    for (event_index, event) in events.iter().enumerate() {
        let Some(recorded) = event.use_recorded() else {
            continue;
        };
        if event.target_content_hash != current_hash {
            continue;
        }
        status.qualifying_uses_on_current_hash += 1;
        let open_incident =
            recorded.outcome != Outcome::Clean && !adjudicated.contains(event.event_id.as_str());
        // A contemporaneous severe incident authorizes on its own below, from
        // `open_incident` alone, so it was never deferred and this exit has no trap to
        // release for it. Listing one here would have the projection report that it
        // stopped driving the gate while it demonstrably still does.
        //
        // A *retrospective* severe incident is a different animal despite the same
        // outcome: the loop skips it before that trigger, so it authorizes nothing and
        // only counts toward a cluster. Carving it out would protect nothing and leave it
        // discounting every later review of the symptom — so the carve-out keys on the
        // property that justifies it, not on severity alone.
        let authorizes_on_its_own =
            recorded.outcome == Outcome::SevereIncident && !recorded.retrospective;
        let instrument_limited_incident = open_incident
            && !authorizes_on_its_own
            && instrument_limited_closes
                .iter()
                .any(|(close, covered, symptoms)| {
                    covered.contains(event.event_id.as_str())
                        || (event_index < *close
                            && recorded
                                .symptom_key
                                .as_deref()
                                .is_some_and(|symptom| symptoms.contains(symptom)))
                });
        if open_incident {
            status.open_incident_ids.push(event.event_id.clone());
        }
        if instrument_limited_incident {
            status
                .instrument_limited_incident_ids
                .push(event.event_id.clone());
        }
        let symptom = if open_incident && !instrument_limited_incident {
            let symptom = recorded
                .symptom_key
                .as_deref()
                .expect("validated symptom key")
                .to_owned();
            if !cluster_events.iter().any(|(key, _)| key == &symptom) {
                cluster_events.push((symptom.clone(), Vec::new()));
                status.candidate_clusters.push(CandidateCluster {
                    symptom_key: symptom.clone(),
                    open_event_ids: Vec::new(),
                    independent_incidents: 0,
                    max_severity: "friction".to_owned(),
                });
            }
            let events_for_cluster = &mut cluster_events
                .iter_mut()
                .find(|(key, _)| key == &symptom)
                .expect("cluster event list was inserted")
                .1;
            events_for_cluster.push(event);
            let cluster = status
                .candidate_clusters
                .iter_mut()
                .find(|cluster| cluster.symptom_key == symptom)
                .expect("cluster was inserted");
            cluster.open_event_ids.push(event.event_id.clone());
            cluster.independent_incidents = independent_count(events_for_cluster);
            if recorded.outcome.severity()
                > Outcome::parse(&cluster.max_severity)
                    .expect("candidate severity")
                    .severity()
            {
                cluster.max_severity = recorded.outcome.as_str().to_owned();
            }
            Some(symptom)
        } else {
            None
        };

        if fired.is_some() || recorded.retrospective {
            continue;
        }
        if open_incident && recorded.outcome == Outcome::SevereIncident {
            fired = Some(threshold_trigger(
                ThresholdReason::Severe,
                &[event],
                event,
                event_index,
            ));
            continue;
        }

        let mut candidate = None;
        if let Some(symptom) = symptom.as_deref() {
            let events_for_cluster = &cluster_events
                .iter()
                .find(|(key, _)| key == symptom)
                .expect("open incident cluster exists")
                .1;
            let material = events_for_cluster
                .iter()
                .copied()
                .filter(|event| {
                    event
                        .use_recorded()
                        .expect("cluster contains use records")
                        .outcome
                        .severity()
                        >= Outcome::MaterialFailure.severity()
                })
                .collect::<Vec<_>>();
            if recorded.outcome.severity() >= Outcome::MaterialFailure.severity()
                && independent_count(&material) >= 2
            {
                candidate = Some(threshold_trigger(
                    ThresholdReason::MaterialRecurrence(symptom.to_owned()),
                    &material,
                    event,
                    event_index,
                ));
            } else if independent_count(events_for_cluster) >= 3 {
                candidate = Some(threshold_trigger(
                    ThresholdReason::FrictionRecurrence(symptom.to_owned()),
                    events_for_cluster,
                    event,
                    event_index,
                ));
            }
        }
        let open_contemporaneous = cluster_events
            .iter()
            .flat_map(|(_, cluster)| cluster.iter().copied())
            .filter(|incident| {
                incident
                    .use_recorded()
                    .is_some_and(|recorded| !recorded.retrospective)
            })
            .collect::<Vec<_>>();
        if candidate.is_none()
            && status.qualifying_uses_on_current_hash >= 10
            && !open_contemporaneous.is_empty()
        {
            let anchor = last_same_hash_disposition_index
                .and_then(|watermark| {
                    open_contemporaneous.iter().copied().find(|incident| {
                        event_positions
                            .get(incident.event_id.as_str())
                            .is_some_and(|index| *index > watermark)
                    })
                })
                .unwrap_or(open_contemporaneous[0]);
            let symptom = anchor
                .use_recorded()
                .and_then(|recorded| recorded.symptom_key.as_deref())
                .expect("validated symptom key");
            let triggers = cluster_events
                .iter()
                .find(|(key, _)| key == symptom)
                .expect("anchor cluster exists")
                .1
                .iter()
                .filter(|event| {
                    event
                        .use_recorded()
                        .is_some_and(|recorded| !recorded.retrospective)
                })
                .copied()
                .collect::<Vec<_>>();
            candidate = Some(threshold_trigger(
                ThresholdReason::TenUseUnresolved,
                &triggers,
                event,
                event_index,
            ));
        }
        if let Some(candidate) = candidate {
            if let Some(watermark) = last_same_hash_disposition_index {
                let has_post_review_incident = open_contemporaneous.iter().any(|incident| {
                    event_positions
                        .get(incident.event_id.as_str())
                        .is_some_and(|index| *index > watermark)
                });
                if event_index > watermark && has_post_review_incident {
                    fired = Some(candidate);
                } else {
                    queued_pre_close_evidence = true;
                }
            } else {
                fired = Some(candidate);
            }
        }
    }
    if status.active_review_id.is_some() {
        status.state = "review_in_progress".to_owned();
    } else if let Some(trigger) = fired {
        let severe = trigger.reason.is_severe();
        status.authorized_workflow = Some("skill-evolution".to_owned());
        status.authorization_reason = Some(trigger.reason.as_string());
        status.trigger_event_ids = trigger.trigger_event_ids;
        status.review_reentry_basis = Some(
            if last_same_hash_disposition_index.is_none() {
                "first_eligibility"
            } else if severe
                && last_same_hash_disposition_index.is_some_and(|index| trigger.event_index < index)
            {
                "unadjudicated_severe"
            } else {
                "post_review_incident"
            }
            .to_owned(),
        );
        status.threshold_session_id = trigger.threshold_session_id.clone();
        let cooldown_passed = if let Some(threshold_session_id) = trigger.threshold_session_id {
            inputs.session_id != "unavailable" && inputs.session_id != threshold_session_id
        } else {
            let fired_at = OffsetDateTime::parse(&trigger.fired_at, &Rfc3339)
                .expect("validated event timestamp parses");
            let not_before_milliseconds =
                i64::try_from(fired_at.unix_timestamp_nanos() / 1_000_000)
                    .expect("event timestamp milliseconds fit in i64")
                    + 12 * 60 * 60 * 1_000;
            status.not_before = Some(format_timestamp_milliseconds(not_before_milliseconds));
            inputs.now_epoch_milliseconds >= not_before_milliseconds
        };
        status.state = cooldown_state(severe, cooldown_passed).to_owned();
    } else if !status.open_incident_ids.is_empty() {
        status.state = "collecting".to_owned();
    }
    if queued_pre_close_evidence && status.state == "collecting" {
        // Evidence the last same-hash review never covered is deferred behind it either
        // way. What differs is what that deferral means: a review that adjudicated
        // accounted for the evidence it saw, while an instrument-limited close accounted
        // for nothing. Keyed on instrument-limited rather than on non-adjudicating in
        // general, because `superseded_by_target_version` also reaches no conclusion and
        // its reporting is deliberately left alone — see ADR 0002.
        status.review_reentry_basis = Some(
            if last_same_hash_close_was_instrument_limited {
                "queued_behind_instrument_limited_review"
            } else {
                "queued_pre_close_evidence"
            }
            .to_owned(),
        );
    }
    status
}

fn threshold_trigger(
    reason: ThresholdReason,
    trigger_events: &[&EvidenceEvent],
    threshold_event: &EvidenceEvent,
    event_index: usize,
) -> ThresholdTrigger {
    ThresholdTrigger {
        reason,
        trigger_event_ids: trigger_events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        threshold_session_id: (threshold_event.top_level_session_id != "unavailable")
            .then(|| threshold_event.top_level_session_id.clone()),
        fired_at: threshold_event.recorded_at.clone(),
        event_index,
    }
}

fn cooldown_state(severe: bool, cooldown_passed: bool) -> &'static str {
    match (severe, cooldown_passed) {
        (true, true) => "quarantined_eligible",
        (true, false) => "quarantined_pending_cooldown",
        (false, true) => "eligible",
        (false, false) => "eligible_pending_cooldown",
    }
}

fn format_timestamp_milliseconds(epoch_milliseconds: i64) -> String {
    let timestamp =
        OffsetDateTime::from_unix_timestamp_nanos(i128::from(epoch_milliseconds) * 1_000_000)
            .expect("supported evidence timestamps fit OffsetDateTime");
    let format = time::format_description::parse_borrowed::<3>(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z",
    )
    .expect("static timestamp format is valid");
    timestamp
        .format(&format)
        .expect("UTC timestamp always formats")
}

fn independent_count(events: &[&EvidenceEvent]) -> usize {
    events
        .iter()
        .map(|event| {
            (
                event.top_level_session_id.as_str(),
                event
                    .use_recorded()
                    .expect("incident cluster contains use records")
                    .task_fingerprint
                    .as_str(),
            )
        })
        .collect::<HashSet<_>>()
        .len()
}

fn write_gate_status(directory: &Path, status: &GateStatus) -> Result<(), Error> {
    let destination = directory.join("gate-status.json");
    let temporary = directory.join(".gate-status.json.tmp");
    prepare_gate_status(&temporary, status)?;
    fs::rename(&temporary, &destination).map_err(|error| {
        unsafe_failure(format!(
            "Could not atomically replace gate projection {}: {error}",
            destination.display()
        ))
    })
}

fn prepare_gate_status(temporary: &Path, status: &GateStatus) -> Result<(), Error> {
    let bytes =
        serde_json::to_vec_pretty(status).expect("serializing a validated gate status cannot fail");
    let mut file = fs::File::create(temporary).map_err(|error| {
        unsafe_failure(format!(
            "Could not create temporary gate projection {}: {error}",
            temporary.display()
        ))
    })?;
    file.write_all(&bytes).map_err(|error| {
        unsafe_failure(format!(
            "Could not write temporary gate projection {}: {error}",
            temporary.display()
        ))
    })?;
    file.write_all(b"\n").map_err(|error| {
        unsafe_failure(format!(
            "Could not finish temporary gate projection {}: {error}",
            temporary.display()
        ))
    })?;
    file.sync_all().map_err(|error| {
        unsafe_failure(format!(
            "Could not synchronize temporary gate projection {}: {error}",
            temporary.display()
        ))
    })
}

fn canonical_directory(path: &Path, label: &str) -> Result<PathBuf, Error> {
    let canonical = path
        .canonicalize()
        .map_err(|_| refusal(format!("{label} does not exist: {}", path.display())))?;
    if !canonical.is_dir() {
        return Err(refusal(format!(
            "{label} is not a directory: {}",
            path.display()
        )));
    }
    Ok(canonical)
}

/// Finds the skill directory `target` names, under `root` if it can.
///
/// A relative target is tried against `root` first and the working directory
/// second. The order matters only when both exist, and then `root` is the one
/// the operator named — explicitly with `--root`, or implicitly as the git
/// toplevel that the working directory sits inside. Preferring the working
/// directory there means auditing a nested copy while writing evidence to the
/// named repository's store, which is a wrong answer that looks like a right
/// one.
///
/// The opposite order was inherited, not chosen: the JavaScript helper this was
/// ported from in GitHub #116 wrote `resolve(process.cwd(), target),
/// resolve(root, target)`, and the port preserved it along with everything else
/// under that issue's compatibility boundary. No skill package documents it —
/// all four say "from the repository root", where the two candidates agree —
/// and across 980 recorded events in the two repositories that carried this
/// code, no target ever resolved to anything but a top-level
/// `.claude/skills/<name>`.
///
/// The working-directory candidate is kept rather than dropped, so a relative
/// path that is *only* meaningful from where the operator stands still
/// resolves.
fn resolve_target(root: &Path, target: &Path) -> Result<PathBuf, Error> {
    let mut candidates = Vec::new();
    if target.is_absolute() {
        candidates.push(target.to_owned());
    } else {
        let current = std::env::current_dir().map_err(|error| {
            unsafe_failure(format!("Could not resolve current directory: {error}"))
        })?;
        candidates.push(root.join(target));
        candidates.push(current.join(target));
    }
    for candidate in candidates {
        if !candidate.is_dir() {
            continue;
        }
        let canonical = candidate.canonicalize().map_err(|error| {
            unsafe_failure(format!(
                "Could not resolve target skill directory {}: {error}",
                candidate.display()
            ))
        })?;
        if !canonical.join("SKILL.md").is_file() {
            return Err(refusal(format!(
                "Target is not a skill directory (no SKILL.md): {}",
                target.display()
            )));
        }
        return Ok(canonical);
    }
    Err(refusal(format!(
        "Target skill directory not found: {}",
        target.display()
    )))
}

pub fn snapshot_baseline(
    source: &Path,
    destination: &Path,
    expected_hash: &str,
) -> Result<(), Error> {
    copy_directory_contents(source, destination)?;
    let copied_hash = hash_target_directory(destination)?.content_hash;
    if copied_hash != expected_hash {
        let _ = fs::remove_dir_all(destination);
        return Err(unsafe_failure(
            "Baseline snapshot did not reproduce the live target hash; nothing recorded."
                .to_owned(),
        ));
    }
    Ok(())
}

pub fn diff_directories(before: &Path, after: &Path) -> Result<DirectoryDiff, Error> {
    let before_files = list_files(before)?;
    let after_files = list_files(after)?;
    let before_set = before_files.iter().cloned().collect::<HashSet<_>>();
    let after_set = after_files.iter().cloned().collect::<HashSet<_>>();
    let mut added = after_set
        .difference(&before_set)
        .map(|path| normalized_path(path))
        .collect::<Vec<_>>();
    let mut removed = before_set
        .difference(&after_set)
        .map(|path| normalized_path(path))
        .collect::<Vec<_>>();
    let mut modified = Vec::new();
    for path in before_set.intersection(&after_set) {
        let before_bytes = fs::read(before.join(path)).map_err(|error| {
            unsafe_failure(format!(
                "Could not read baseline file {} while deriving the directory diff: {error}",
                path.display()
            ))
        })?;
        let after_bytes = fs::read(after.join(path)).map_err(|error| {
            unsafe_failure(format!(
                "Could not read landed file {} while deriving the directory diff: {error}",
                path.display()
            ))
        })?;
        if before_bytes != after_bytes {
            modified.push(normalized_path(path));
        }
    }
    added.sort();
    removed.sort();
    modified.sort();
    Ok(DirectoryDiff {
        added,
        removed,
        modified,
    })
}

fn prepare_validated_landing(
    target: &TargetContext,
    events: &[EvidenceEvent],
    live_hash: &str,
    candidate: &Path,
    policy: ValidatedLandingPolicy<'_>,
) -> Result<ValidatedLandingPreparation, Error> {
    if candidate == target.target_real {
        return Err(refusal(
            "--candidate must be the isolated copy, not the live target.".to_owned(),
        ));
    }
    let candidate_hash = hash_target_directory(candidate)?.content_hash;
    if candidate_hash == live_hash {
        return Err(refusal(
            "Candidate is byte-identical to the live target; nothing to land.".to_owned(),
        ));
    }
    let latest = events
        .iter()
        .rfind(|event| {
            matches!(event.kind, EventKind::ValidationCompleted { .. })
                && event.review_id() == Some(policy.owner_id)
                && event
                    .raw
                    .pointer("/payload/decision")
                    .and_then(Value::as_str)
                    == Some("accepted")
        })
        .ok_or_else(|| {
            refusal(format!(
                "No accepted validation_completed event exists for review {}. Landing refused.",
                policy.owner_id
            ))
        })?;
    let validated_hash = latest
        .raw
        .pointer("/payload/candidate_hash")
        .and_then(Value::as_str)
        .expect("validated candidate hash");
    if validated_hash != candidate_hash {
        return Err(refusal(format!(
            "Candidate bytes are not exactly those validated (validated {}…, supplied {}…). Landing refused.",
            &validated_hash[..validated_hash.len().min(12)],
            &candidate_hash[..12]
        )));
    }
    if events.iter().any(|event| event.event_id == policy.event_id) {
        return Err(unsafe_failure(format!(
            "Constructed event failed validation — nothing appended:\n  duplicate event_id {}",
            policy.event_id
        )));
    }
    if policy.backup_directory.exists() {
        return Err(refusal(format!(
            "Backup already exists at {}; a prior land attempt ran for this review. Inspect before retrying.",
            policy.backup_directory.display()
        )));
    }
    Ok(ValidatedLandingPreparation {
        candidate_hash,
        backup_directory: policy.backup_directory,
    })
}

pub fn land_validated_candidate(
    live_target: &Path,
    candidate: &Path,
    backup: &Path,
    baseline_hash: &str,
    candidate_hash: &str,
    mirror: Option<&Path>,
) -> Result<LandingMechanicsReceipt, Error> {
    snapshot_baseline(live_target, backup, baseline_hash).map_err(|error| {
        unsafe_failure(format!(
            "Backup copy did not reproduce the live baseline hash; landing aborted before any target change. {error}"
        ))
    })?;
    let verification = sync_directory(candidate, live_target)
        .and_then(|()| hash_target_directory(live_target).map(|report| report.content_hash));
    let after_hash = match verification {
        Ok(after_hash) if after_hash == candidate_hash => after_hash,
        Ok(after_hash) => {
            return Err(landing_verification_failure(
                live_target,
                backup,
                baseline_hash,
                format!(
                    "landed hash {}… != candidate hash",
                    &after_hash[..after_hash.len().min(12)]
                ),
            ));
        }
        Err(error) => {
            return Err(landing_verification_failure(
                live_target,
                backup,
                baseline_hash,
                error.to_string(),
            ));
        }
    };
    let mirror_status = landing_mirror_status(live_target, mirror);
    let changed_files = diff_directories(backup, live_target)?;
    Ok(LandingMechanicsReceipt {
        after_hash,
        changed_files,
        mirror_status,
    })
}

fn landing_verification_failure(
    live_target: &Path,
    backup: &Path,
    baseline_hash: &str,
    reason: String,
) -> Error {
    let restoration = sync_directory(backup, live_target)
        .and_then(|()| hash_target_directory(live_target).map(|report| report.content_hash));
    match restoration {
        Ok(restored_hash) if restored_hash == baseline_hash => unsafe_failure(format!(
            "Landing verification failed: {reason}. Live baseline restored from backup."
        )),
        Ok(restored_hash) => unsafe_failure(format!(
            "Landing verification failed: {reason}. RESTORE ALSO FAILED (live hash {}…); recover from {} or Git.",
            &restored_hash[..restored_hash.len().min(12)],
            backup.display()
        )),
        Err(error) => unsafe_failure(format!(
            "Landing verification failed: {reason}. RESTORE ALSO FAILED ({error}); recover from {} or Git.",
            backup.display()
        )),
    }
}

fn expected_mirror_path(target: &TargetContext) -> Option<PathBuf> {
    if !target.repo_relative_path.starts_with(".claude/skills/") {
        return None;
    }
    Some(
        target
            .repository_root
            .join(".agents/skills")
            .join(&target.target_name),
    )
}

fn landing_mirror_status(live_target: &Path, mirror: Option<&Path>) -> &'static str {
    let Some(mirror) = mirror else {
        return "not_applicable";
    };
    if !mirror.exists() {
        return "absent";
    }
    if mirror
        .canonicalize()
        .is_ok_and(|resolved| resolved == live_target)
    {
        "ok"
    } else {
        "broken"
    }
}

fn sync_directory(source: &Path, destination: &Path) -> Result<(), Error> {
    if destination.exists() {
        for entry in fs::read_dir(destination).map_err(|error| {
            unsafe_failure(format!(
                "Could not read destination directory {}: {error}",
                destination.display()
            ))
        })? {
            let path = entry
                .map_err(|error| {
                    unsafe_failure(format!(
                        "Could not read destination entry in {}: {error}",
                        destination.display()
                    ))
                })?
                .path();
            remove_path(&path)?;
        }
    } else {
        fs::create_dir_all(destination).map_err(|error| {
            unsafe_failure(format!(
                "Could not create destination directory {}: {error}",
                destination.display()
            ))
        })?;
    }
    copy_directory_contents(source, destination)
}

fn copy_directory_contents(source: &Path, destination: &Path) -> Result<(), Error> {
    fs::create_dir_all(destination).map_err(|error| {
        unsafe_failure(format!(
            "Could not create copied directory {}: {error}",
            destination.display()
        ))
    })?;
    for entry in fs::read_dir(source).map_err(|error| {
        unsafe_failure(format!(
            "Could not read source directory {}: {error}",
            source.display()
        ))
    })? {
        let entry = entry.map_err(|error| {
            unsafe_failure(format!(
                "Could not read source entry in {}: {error}",
                source.display()
            ))
        })?;
        copy_path(&entry.path(), &destination.join(entry.file_name()))?;
    }
    Ok(())
}

fn copy_path(source: &Path, destination: &Path) -> Result<(), Error> {
    let metadata = fs::symlink_metadata(source).map_err(|error| {
        unsafe_failure(format!(
            "Could not inspect source path {}: {error}",
            source.display()
        ))
    })?;
    if metadata.file_type().is_symlink() {
        let link = fs::read_link(source).map_err(|error| {
            unsafe_failure(format!(
                "Could not read source symlink {}: {error}",
                source.display()
            ))
        })?;
        return create_symlink(source, &link, destination);
    }
    if metadata.is_dir() {
        return copy_directory_contents(source, destination);
    }
    fs::copy(source, destination).map_err(|error| {
        unsafe_failure(format!(
            "Could not copy {} to {}: {error}",
            source.display(),
            destination.display()
        ))
    })?;
    Ok(())
}

fn remove_path(path: &Path) -> Result<(), Error> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        unsafe_failure(format!(
            "Could not inspect target path {}: {error}",
            path.display()
        ))
    })?;
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
    .map_err(|error| unsafe_failure(format!("Could not remove {}: {error}", path.display())))
}

#[cfg(unix)]
fn create_symlink(_source: &Path, link: &Path, destination: &Path) -> Result<(), Error> {
    std::os::unix::fs::symlink(link, destination).map_err(|error| {
        unsafe_failure(format!(
            "Could not copy symlink to {}: {error}",
            destination.display()
        ))
    })
}

#[cfg(windows)]
fn create_symlink(source: &Path, link: &Path, destination: &Path) -> Result<(), Error> {
    let target = if link.is_absolute() {
        link.to_owned()
    } else {
        source
            .parent()
            .expect("source symlink has a parent")
            .join(link)
    };
    let result = if target.is_dir() {
        std::os::windows::fs::symlink_dir(link, destination)
    } else {
        std::os::windows::fs::symlink_file(link, destination)
    };
    result.map_err(|error| {
        unsafe_failure(format!(
            "Could not copy symlink to {}: {error}",
            destination.display()
        ))
    })
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}

fn list_files(directory: &Path) -> Result<Vec<PathBuf>, Error> {
    fn visit(base: &Path, directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
        let entries = fs::read_dir(directory).map_err(|error| {
            unsafe_failure(format!(
                "Could not read target directory {}: {error}",
                directory.display()
            ))
        })?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                unsafe_failure(format!(
                    "Could not read target directory entry in {}: {error}",
                    directory.display()
                ))
            })?;
            let path = entry.path();
            let metadata = fs::metadata(&path).map_err(|error| {
                unsafe_failure(format!(
                    "Could not inspect target path {}: {error}",
                    path.display()
                ))
            })?;
            if metadata.is_dir() {
                visit(base, &path, files)?;
            } else {
                files.push(
                    path.strip_prefix(base)
                        .expect("visited path is below base")
                        .to_owned(),
                );
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    visit(directory, directory, &mut files)?;
    files.sort();
    Ok(files)
}

#[cfg(unix)]
fn path_bytes(path: &Path) -> Cow<'_, [u8]> {
    use std::os::unix::ffi::OsStrExt;
    normalize_separator_for_hash(path.as_os_str().as_bytes(), b'/')
}

#[cfg(not(unix))]
fn path_bytes(path: &Path) -> Cow<'_, [u8]> {
    Cow::Owned(
        normalize_separator_for_hash(
            path.to_str()
                .expect("skill evidence paths must be valid UTF-8 on this platform")
                .as_bytes(),
            std::path::MAIN_SEPARATOR as u8,
        )
        .into_owned(),
    )
}

fn normalize_separator_for_hash(bytes: &[u8], separator: u8) -> Cow<'_, [u8]> {
    if separator == b'/' || !bytes.contains(&separator) {
        Cow::Borrowed(bytes)
    } else {
        Cow::Owned(
            bytes
                .iter()
                .map(|byte| if *byte == separator { b'/' } else { *byte })
                .collect(),
        )
    }
}

fn refusal(message: String) -> Error {
    Error {
        class: ErrorClass::Refusal,
        message,
        recovery: None,
    }
}

fn unsafe_failure(message: String) -> Error {
    Error {
        class: ErrorClass::UnsafeFailure,
        message,
        recovery: None,
    }
}

/// An unsafe failure the operator can still finish by hand.
///
/// Reach for this instead of [`unsafe_failure`] whenever the effect is partial
/// in a way a named command repairs. The message says what happened; the
/// [`Recovery`] says what to do, and the host says what to call the binary that
/// does it.
fn unsafe_failure_with_recovery(message: String, recovery: Recovery) -> Error {
    Error {
        class: ErrorClass::UnsafeFailure,
        message,
        recovery: Some(recovery),
    }
}
