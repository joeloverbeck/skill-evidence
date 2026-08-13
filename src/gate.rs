//! Deriving a gate projection from a target's recorded events.
//!
//! The event model, the validity rules a reader applies to a recorded line, and
//! the clustering, threshold and retirement ladder that turns an accumulation into
//! a gate all live here. None of it touches the filesystem. The evidence store on
//! disk is one adapter over this module; the status reporter is another, and it
//! derives without writing anything.
//!
//! What does not live here are the rosters these rules check against — symptom
//! keys, dispositions, external-owner kinds, constraint-provenance fields. The
//! command surface validates against the same lists, and two of them back public
//! types, so they stay in [`crate`] and are imported below. Adding a value to one
//! is an edit there, not here. This module mostly checks membership against them,
//! though not only: the external-owner rule in [`validate_event`] names the
//! `outside_target` disposition outright, because that is the one close the rule is
//! about. That the core takes its vocabulary from the crate that adapts it is the
//! wrong way round, and moving the rosters down here is a change worth making on
//! its own rather than inside a move.
//!
//! The currency is [`ValidatedStream`]. It carries the events a reader admitted
//! together with the integrity errors it found, so a caller cannot hand the
//! derivation one without the other and cannot forget to pass the errors it
//! already knows about. That is what makes a clean gate over a corrupt stream
//! unrepresentable rather than merely avoided by convention.

use std::collections::{HashMap, HashSet};

use serde_json::Value;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    CONSTRAINT_PROVENANCE_FIELDS, CandidateCluster, DISPOSITIONS,
    EVOLUTION_ADJUDICATING_DISPOSITIONS, EVOLUTION_INSTRUMENT_LIMITED_DISPOSITIONS,
    EXTERNAL_OWNER_KINDS, GateStatus, SYMPTOM_KEYS,
};

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
pub(crate) enum Outcome {
    Clean,
    Friction,
    MaterialFailure,
    SevereIncident,
}

impl Outcome {
    pub(crate) const ALLOWED: &'static str = "clean|friction|material_failure|severe_incident";

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "clean" => Some(Self::Clean),
            "friction" => Some(Self::Friction),
            "material_failure" => Some(Self::MaterialFailure),
            "severe_incident" => Some(Self::SevereIncident),
            _ => None,
        }
    }

    pub(crate) fn severity(self) -> u8 {
        match self {
            Self::Clean => 0,
            Self::Friction => 1,
            Self::MaterialFailure => 2,
            Self::SevereIncident => 3,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Friction => "friction",
            Self::MaterialFailure => "material_failure",
            Self::SevereIncident => "severe_incident",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UseRecordedEvent {
    pub(crate) retrospective: bool,
    pub(crate) task_fingerprint: String,
    pub(crate) same_run_group: String,
    pub(crate) outcome: Outcome,
    pub(crate) symptom_key: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum EventKind {
    UseRecorded(UseRecordedEvent),
    ReviewStarted {
        review_id: String,
    },
    ReviewDisposition {
        review_id: String,
        disposition: String,
        adjudicated_event_ids: Vec<String>,
        /// The subset of the coverage list this close could not decide — no trial could
        /// express the mechanism, or the acceptance gate grades outcome while the evidence
        /// bears no outcome claim. Empty for every close written before the key existed,
        /// which is the same claim those closes were already making: everything covered
        /// was decided.
        instrument_limited_event_ids: Vec<String>,
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
pub(crate) struct EvidenceEvent {
    pub(crate) event_id: String,
    pub(crate) recorded_at: String,
    pub(crate) target_content_hash: String,
    pub(crate) top_level_session_id: String,
    pub(crate) kind: EventKind,
    pub(crate) raw: Value,
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
                instrument_limited_event_ids: payload
                    .get("instrument_limited_event_ids")
                    .and_then(Value::as_array)
                    .map(|identities| {
                        identities
                            .iter()
                            .map(|identity| {
                                identity
                                    .as_str()
                                    .expect("validated instrument-limited event identity")
                                    .to_owned()
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
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

    pub(crate) fn review_id(&self) -> Option<&str> {
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

    pub(crate) fn use_recorded(&self) -> Option<&UseRecordedEvent> {
        match &self.kind {
            EventKind::UseRecorded(recorded) => Some(recorded),
            _ => None,
        }
    }

    pub(crate) fn is_review_start(&self) -> bool {
        matches!(self.kind, EventKind::ReviewStarted { .. })
    }

    pub(crate) fn starts_ownership(&self) -> bool {
        matches!(
            self.kind,
            EventKind::ReviewStarted { .. } | EventKind::DecontaminationStarted { .. }
        )
    }

    pub(crate) fn terminates_ownership(&self) -> bool {
        matches!(
            self.kind,
            EventKind::ReviewDisposition { .. }
                | EventKind::ChangeLanded { .. }
                | EventKind::DecontaminationCompleted { .. }
        )
    }
}

#[derive(Debug)]
pub(crate) enum ThresholdReason {
    Severe,
    MaterialRecurrence(String),
    FrictionRecurrence(String),
    TenUseUnresolved,
}

impl ThresholdReason {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "severe" => Some(Self::Severe),
            "ten_use_unresolved" => Some(Self::TenUseUnresolved),
            _ => value
                .strip_prefix("material_recurrence:")
                .filter(|symptom| !symptom.is_empty())
                .map(|symptom| Self::MaterialRecurrence(symptom.to_owned()))
                .or_else(|| {
                    value
                        .strip_prefix("friction_recurrence:")
                        .filter(|symptom| !symptom.is_empty())
                        .map(|symptom| Self::FrictionRecurrence(symptom.to_owned()))
                }),
        }
    }

    pub(crate) fn as_string(&self) -> String {
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

    pub(crate) fn is_severe(&self) -> bool {
        matches!(self, Self::Severe)
    }
}

pub(crate) fn authorization_reason_names_incident(
    reason: Option<&ThresholdReason>,
    anchor_symptoms: &HashSet<&str>,
    recorded: &UseRecordedEvent,
) -> bool {
    match reason {
        Some(ThresholdReason::Severe) => false,
        Some(ThresholdReason::TenUseUnresolved) => {
            !recorded.retrospective
                && recorded
                    .symptom_key
                    .as_deref()
                    .is_some_and(|symptom| anchor_symptoms.contains(symptom))
        }
        Some(ThresholdReason::MaterialRecurrence(symptom)) => {
            recorded.symptom_key.as_deref() == Some(symptom.as_str())
                && recorded.outcome.severity() >= Outcome::MaterialFailure.severity()
        }
        Some(ThresholdReason::FrictionRecurrence(symptom)) => {
            recorded.symptom_key.as_deref() == Some(symptom.as_str())
        }
        None => recorded
            .symptom_key
            .as_deref()
            .is_some_and(|symptom| anchor_symptoms.contains(symptom)),
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

/// One target's recorded events as the reader admitted them, together with the
/// integrity errors it found.
///
/// The only constructors are [`ValidatedStream::parse`], [`ValidatedStream::empty`]
/// and [`ValidatedStream::accept_candidate`], so an [`EvidenceEvent`] cannot exist
/// unless [`validate_event`] passed the line it came from. That is what makes
/// [`EvidenceEvent::from_validated`]'s `expect` calls unreachable rather than merely
/// unreached, and it is why deriving takes a stream rather than a list of events and
/// a separate list of errors a caller could forget.
#[derive(Debug, Clone)]
pub(crate) struct ValidatedStream {
    events: Vec<EvidenceEvent>,
    integrity_errors: Vec<String>,
}

impl ValidatedStream {
    /// Admits the lines that validate, recording an integrity error for each line
    /// that does not.
    ///
    /// `source` names the stream in the one error that is about the whole stream
    /// rather than about a line within it.
    pub(crate) fn parse(bytes: &[u8], source: &str) -> Self {
        let Ok(text) = std::str::from_utf8(bytes) else {
            return Self {
                events: Vec::new(),
                integrity_errors: vec![format!("event stream is not valid UTF-8: {source}")],
            };
        };
        let mut events = Vec::new();
        let mut integrity_errors = Vec::new();
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
                        integrity_errors.extend(
                            event_errors
                                .into_iter()
                                .map(|message| format!("line {}: {message}", line_index + 1)),
                        );
                    }
                }
                Err(_) => {
                    integrity_errors.push(format!("line {}: not valid JSON", line_index + 1));
                }
            }
        }
        Self {
            events,
            integrity_errors,
        }
    }

    /// What a target whose evidence directory holds no stream yet has recorded.
    ///
    /// Distinct from a stream that failed to read: an absent stream is not an
    /// integrity problem, it is a target nobody has recorded a use against.
    pub(crate) fn empty() -> Self {
        Self {
            events: Vec::new(),
            integrity_errors: Vec::new(),
        }
    }

    pub(crate) fn events(&self) -> &[EvidenceEvent] {
        &self.events
    }

    pub(crate) fn integrity_errors(&self) -> &[String] {
        &self.integrity_errors
    }

    /// Whether every recorded line validated.
    pub(crate) fn is_intact(&self) -> bool {
        self.integrity_errors.is_empty()
    }

    /// Records an integrity finding about the store this stream came from, rather
    /// than about a line within it — a stream whose events disagree with the target
    /// they are filed under, for instance.
    ///
    /// It joins the stream's own errors because it means the same thing to the
    /// derivation: this evidence cannot be read as it stands, so the gate blocks.
    pub(crate) fn with_integrity_error(mut self, message: String) -> Self {
        self.integrity_errors.push(message);
        self
    }

    /// The stream this one would be if `event_id` had never been recorded.
    ///
    /// Deriving from it answers what the gate would say without that event. No
    /// projection records that answer, so a reporter that needs it has to ask —
    /// and asking here keeps the question inside the module that owns it rather
    /// than rebuilding an event list outside.
    pub(crate) fn without_event(&self, event_id: &str) -> Self {
        Self {
            events: self
                .events
                .iter()
                .filter(|event| event.event_id != event_id)
                .cloned()
                .collect(),
            integrity_errors: self.integrity_errors.clone(),
        }
    }

    /// The stream this one would become with `event` appended, or the reasons that
    /// event cannot join it.
    ///
    /// The rejection comes back as bare messages because callers grade it
    /// differently — recording refuses, a lifecycle write fails unsafely — and
    /// because a candidate this stream never accepted is not one of its integrity
    /// errors. Nothing is written here; a caller stages the projection this returns
    /// and only then appends, so a failed append leaves nothing half-applied.
    pub(crate) fn accept_candidate(&self, event: &Value) -> Result<Self, Vec<String>> {
        let seen_ids = self
            .events
            .iter()
            .map(|existing| existing.event_id.clone())
            .collect::<HashSet<_>>();
        let errors = validate_event(event, &seen_ids);
        if !errors.is_empty() {
            return Err(errors);
        }
        let mut events = self.events.clone();
        events.push(EvidenceEvent::from_validated(event));
        Ok(Self {
            events,
            integrity_errors: self.integrity_errors.clone(),
        })
    }
}

/// The two labels a projection copies from the target it is about.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GateTarget<'a> {
    pub(crate) name: &'a str,
    pub(crate) repo_relative_path: &'a str,
}

/// When the derivation is happening and who is asking.
///
/// Narrower than [`crate::DerivationInputs`] on purpose: deriving never locks, so
/// it has no business being handed a lock owner.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GateClock<'a> {
    pub(crate) generated_at: &'a str,
    pub(crate) now_epoch_milliseconds: i64,
    pub(crate) session_id: &'a str,
}

fn is_valid_external_owner_entry(owner: &Value) -> bool {
    owner.as_object().is_some_and(|entry| {
        entry.len() == 3
            && ["event_id", "kind", "reference"]
                .iter()
                .all(|field| entry.contains_key(*field))
    }) && non_empty_string(owner.get("event_id")).is_some()
        && non_empty_string(owner.get("kind"))
            .is_some_and(|kind| EXTERNAL_OWNER_KINDS.contains(&kind))
        && non_empty_string(owner.get("reference")).is_some()
}

fn is_valid_constraint_provenance_entry(citation: &Value) -> bool {
    citation.as_object().is_some_and(|entry| {
        ["constraint_label", "event_id", "field", "field_value"]
            .iter()
            .all(|field| entry.contains_key(*field))
    }) && non_empty_string(citation.get("constraint_label")).is_some()
        && non_empty_string(citation.get("event_id")).is_some()
        && non_empty_string(citation.get("field"))
            .is_some_and(|field| CONSTRAINT_PROVENANCE_FIELDS.contains(&field))
        && non_empty_string(citation.get("field_value")).is_some()
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
    if matches!(
        event_type,
        Some(
            EventType::ReviewStarted
                | EventType::ValidationCompleted
                | EventType::ChangeLanded
                | EventType::ReviewDisposition
        )
    ) && payload
        .get("operating_skill_hash")
        .is_some_and(|value| non_empty_string(Some(value)).is_none())
    {
        errors.push("operating_skill_hash must be a non-empty string when present".to_owned());
    }
    if matches!(
        event_type,
        Some(
            EventType::ReviewStarted
                | EventType::ValidationCompleted
                | EventType::ChangeLanded
                | EventType::ReviewDisposition
        )
    ) && payload
        .get("operating_package_matches_shipped")
        .is_some_and(|value| !value.is_boolean())
    {
        errors.push("operating_package_matches_shipped must be boolean when present".to_owned());
    }

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
    } else if event_type == Some(EventType::ReviewStarted) {
        if non_empty_string(payload.get("review_id")).is_none() {
            errors.push("review_id missing".to_owned());
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
        // Absent means the close concluded about everything it covered. A shape the reader
        // cannot trust must not collapse into that claim, so it is an integrity error
        // rather than a narrowing silently read as empty.
        if payload
            .get("instrument_limited_event_ids")
            .is_some_and(|value| {
                value.as_array().is_none_or(|identities| {
                    identities.is_empty()
                        || identities
                            .iter()
                            .any(|identity| non_empty_string(Some(identity)).is_none())
                })
            })
        {
            errors.push(
                "instrument_limited_event_ids must be a non-empty array of event ids when present"
                    .to_owned(),
            );
        }
        if payload.get("constraint_provenance").is_some_and(|value| {
            value.as_array().is_none_or(|citations| {
                citations.is_empty()
                    || citations
                        .iter()
                        .any(|citation| !is_valid_constraint_provenance_entry(citation))
            })
        }) {
            errors.push(format!(
                "constraint_provenance must be a non-empty array of citations containing non-empty constraint_label, event_id, field ({}), and field_value when present",
                CONSTRAINT_PROVENANCE_FIELDS.join("|")
            ));
        }
        if payload.get("external_owners").is_some_and(|value| {
            value.as_array().is_none_or(|owners| {
                owners.is_empty()
                    || owners
                        .iter()
                        .any(|owner| !is_valid_external_owner_entry(owner))
            })
        }) {
            errors.push(format!(
                "external_owners must be a non-empty array of objects containing exactly event_id, kind ({}), and reference when present",
                EXTERNAL_OWNER_KINDS.join("|")
            ));
        }
        if let Some(owners) = payload.get("external_owners").and_then(Value::as_array)
            && !owners.is_empty()
            && owners.iter().all(is_valid_external_owner_entry)
        {
            let coverage = payload
                .get("adjudicated_event_ids")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>();
            let undecidable = payload
                .get("instrument_limited_event_ids")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect::<HashSet<_>>();
            let concluded = coverage
                .iter()
                .copied()
                .filter(|identity| !undecidable.contains(identity))
                .collect::<HashSet<_>>();
            let owner_ids = owners
                .iter()
                .filter_map(|owner| owner["event_id"].as_str())
                .collect::<Vec<_>>();
            let owner_id_set = owner_ids.iter().copied().collect::<HashSet<_>>();
            if payload.get("disposition").and_then(Value::as_str) != Some("outside_target")
                || owner_ids.len() != owner_id_set.len()
                || owner_id_set != concluded
            {
                errors.push(
                    "external_owners, when present, must name each concluded coverage event exactly once on an outside_target disposition"
                        .to_owned(),
                );
            }
        }
        if payload
            .get("trial_count")
            .is_some_and(|value| value.as_u64().is_none_or(|count| count == 0))
        {
            errors.push("trial_count must be a positive integer when present".to_owned());
        }
        if payload
            .get("artifacts_path")
            .is_some_and(|value| non_empty_string(Some(value)).is_none())
        {
            errors.push("artifacts_path must be a non-empty string when present".to_owned());
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

/// What a target's recorded evidence currently supports.
///
/// A mechanical function of the stream: it reports, it does not judge, and it never
/// touches the events it read. A stream carrying integrity errors derives a blocked
/// gate, which is why the errors travel with the events rather than beside them.
pub(crate) fn derive(
    stream: &ValidatedStream,
    target: GateTarget<'_>,
    current_hash: &str,
    clock: GateClock<'_>,
) -> GateStatus {
    let events = stream.events();
    let mut status = GateStatus {
        schema_version: 1,
        generated_at: clock.generated_at.to_owned(),
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
        target_name: target.name.to_owned(),
        target_repo_relative_path: target.repo_relative_path.to_owned(),
        derivation_session_id: (clock.session_id != crate::SESSION_UNAVAILABLE)
            .then(|| clock.session_id.to_owned()),
        instrument_limited_incident_ids: Vec::new(),
        integrity_errors: stream.integrity_errors().to_vec(),
    };
    if !status.integrity_errors.is_empty() {
        status.state = "blocked".to_owned();
        return status;
    }
    // The coverage list records what a close accounted for; the conclusion set is that
    // list minus the events the close named as untestable. A close that recorded reaching
    // no conclusion about an event never adjudicated it, whatever the target hash has done
    // since, so this subtraction is unconditional exactly like the union it narrows.
    // Whether such an event also leaves the gate is a separate question, answered below
    // under ADR 0002's same-hash rule.
    let mut adjudicated = HashSet::new();
    for event in events {
        let EventKind::ReviewDisposition {
            disposition,
            adjudicated_event_ids,
            instrument_limited_event_ids,
            ..
        } = &event.kind
        else {
            continue;
        };
        if !EVOLUTION_ADJUDICATING_DISPOSITIONS.contains(&disposition.as_str()) {
            continue;
        }
        for identity in adjudicated_event_ids {
            if !instrument_limited_event_ids.contains(identity) {
                adjudicated.insert(identity.as_str());
            }
        }
    }
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
    // Current claims freeze coverage at the claim. Historical claims retain the coverage
    // they recorded; neither shape is reinterpreted. Retirement re-evaluates the review's
    // own authorization reason at the close: for current claims it can reach reason-scoped
    // incidents that arrived between claim and close, but not incidents that could never
    // have contributed to that authorization.
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
            let authorizing_reason = review_starts
                .iter()
                .find(|(started_id, _)| {
                    *started_id == event.review_id().expect("review disposition identity")
                })
                .and_then(|(_, start)| {
                    start
                        .raw
                        .pointer("/payload/authorizing_rule")
                        .and_then(Value::as_str)
                })
                .and_then(ThresholdReason::parse);
            Some((index, covered, symptoms, authorizing_reason))
        })
        .collect::<Vec<_>>();
    // Coverage an adjudicating close named as untestable leaves the gate on the same
    // warrant ADR 0002 gives an instrument-limited close: the review established that it could
    // not decide them, so they stop clustering while staying open in the ledger.
    //
    // This reach never widens past the names. A close that examined its coverage list
    // mechanism by mechanism said exactly which mechanisms it could not decide, so there is
    // nothing left for a symptom-wide or reason-scoped rule to infer — and inferring one
    // would retire evidence the review did not examine, which is what #16 narrowed away
    // from. The names may rest on either limit: no trial could express the mechanism, or
    // the acceptance gate grades outcome while the evidence bears no outcome claim.
    //
    // Same-hash only, for ADR 0002's reason: what this review could not decide about these
    // bytes says nothing about a target that has since changed.
    let named_untestable_coverage = events
        .iter()
        .filter_map(|event| {
            let EventKind::ReviewDisposition {
                disposition,
                adjudicated_event_ids,
                instrument_limited_event_ids,
                ..
            } = &event.kind
            else {
                return None;
            };
            if !EVOLUTION_ADJUDICATING_DISPOSITIONS.contains(&disposition.as_str()) {
                return None;
            }
            if !ran_on_current_hash(event.review_id().expect("review disposition identity")) {
                return None;
            }
            // Naming is a narrowing of the coverage list, so a name outside that list
            // establishes nothing. The close refuses to write one; honouring it on read
            // would let a hand-edited stream retire an incident no review accounted for.
            Some(
                instrument_limited_event_ids
                    .iter()
                    .filter(|identity| adjudicated_event_ids.contains(identity))
                    .map(String::as_str),
            )
        })
        .flatten()
        .collect::<HashSet<_>>();
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
    // The denominator counts runs, not records. One run can record several incidents —
    // distinct deviations, each with its own identity so a close can name exactly the one
    // its instrument could not test — and they share the run group that says they are one
    // use. Counting records instead would let an honestly recorded run advance
    // `ten_use_unresolved` faster than a compressed one, which is recording manufacturing
    // its own authorization.
    //
    // Counted as the loop walks, so the running total the threshold below reads is the
    // number of runs seen so far. Where a hash carries one record per run group this counts
    // exactly what counting records counted, at every point in the walk and not only at the
    // end — so no such stream re-derives to a different denominator than it already reports.
    //
    // Two separate grounds say existing streams are that shape, and only the first is
    // structural. Every record written before this change met a write path that refused any
    // repeat of a derived run group — the declared further incident that may now repeat one
    // did not exist to be written. That argument does not reach a legacy group, whose check
    // is scoped to one session, so a pre-session-scoped stream was never excluded by
    // construction. Measured instead — 1251 use records across the three consumer
    // repositories and this one, every `(hash, run group)` pair distinct.
    let mut counted_run_groups = HashSet::new();
    for (event_index, event) in events.iter().enumerate() {
        let Some(recorded) = event.use_recorded() else {
            continue;
        };
        if event.target_content_hash != current_hash {
            continue;
        }
        if counted_run_groups.insert(recorded.same_run_group.as_str()) {
            status.qualifying_uses_on_current_hash += 1;
        }
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
            && (named_untestable_coverage.contains(event.event_id.as_str())
                || instrument_limited_closes.iter().any(
                    |(close, covered, symptoms, authorizing_reason)| {
                        covered.contains(event.event_id.as_str())
                            || (event_index < *close
                                && authorization_reason_names_incident(
                                    authorizing_reason.as_ref(),
                                    symptoms,
                                    recorded,
                                ))
                    },
                ));
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
            clock.session_id != crate::SESSION_UNAVAILABLE
                && clock.session_id != threshold_session_id
        } else {
            let fired_at = OffsetDateTime::parse(&trigger.fired_at, &Rfc3339)
                .expect("validated event timestamp parses");
            let not_before_milliseconds =
                i64::try_from(fired_at.unix_timestamp_nanos() / 1_000_000)
                    .expect("event timestamp milliseconds fit in i64")
                    + 12 * 60 * 60 * 1_000;
            status.not_before = Some(format_timestamp_milliseconds(not_before_milliseconds));
            clock.now_epoch_milliseconds >= not_before_milliseconds
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
        threshold_session_id: (threshold_event.top_level_session_id != crate::SESSION_UNAVAILABLE)
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

pub(crate) fn format_timestamp_milliseconds(epoch_milliseconds: i64) -> String {
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use serde_json::json;

    use super::*;

    /// The evidence a derivation reads, with no store around it.
    ///
    /// Deriving wants a stream, a hash and a clock — not a repository, a lock, a
    /// directory to hash or a projection to write — so the fixture supplies exactly
    /// those. The target hash is a fixed string because nothing here hashes anything;
    /// what matters is only that the events agree with what the derivation is told
    /// the current hash is.
    struct Fixture {
        target_relative: &'static str,
        target_hash: String,
        recorded: RefCell<Vec<Value>>,
    }

    impl Fixture {
        fn new() -> Self {
            Self {
                target_relative: ".claude/skills/demo-skill",
                target_hash: "fixture-target-content-hash".to_owned(),
                recorded: RefCell::new(Vec::new()),
            }
        }

        fn write_events(&self, events: &[Value]) {
            *self.recorded.borrow_mut() = events.to_vec();
        }

        /// The events this fixture last recorded, for a test that appends to them.
        fn recorded_events(&self) -> Vec<Value> {
            self.recorded.borrow().clone()
        }

        fn derive(&self, session_id: &str, now_millis: i64) -> GateStatus {
            self.derive_against(&self.target_hash, session_id, now_millis)
        }

        /// Derives against a hash the recorded events were not recorded against —
        /// what the store does once the target's own content has changed.
        fn derive_against(
            &self,
            current_hash: &str,
            session_id: &str,
            now_millis: i64,
        ) -> GateStatus {
            let mut text = self
                .recorded
                .borrow()
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join("\n");
            text.push('\n');
            let stream = ValidatedStream::parse(text.as_bytes(), "fixture events.jsonl");
            super::derive(
                &stream,
                GateTarget {
                    name: "demo-skill",
                    repo_relative_path: self.target_relative,
                },
                current_hash,
                GateClock {
                    generated_at: "2026-01-03T00:00:00Z",
                    now_epoch_milliseconds: now_millis,
                    session_id,
                },
            )
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

        fn review_with_authorizing_rule(
            &self,
            review_id: &str,
            disposition: &str,
            covered: &[&str],
            authorizing_rule: &str,
        ) -> [Value; 2] {
            let mut review = self.review(review_id, disposition, covered);
            review[0]["payload"]["authorizing_rule"] = json!(authorizing_rule);
            review
        }

        fn review_naming_untestable_coverage(
            &self,
            review_id: &str,
            disposition: &str,
            covered: &[&str],
            untestable: &[&str],
        ) -> [Value; 2] {
            let mut review = self.review(review_id, disposition, covered);
            review[1]["payload"]["instrument_limited_event_ids"] = json!(untestable);
            review
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

        /// Another incident from the run `of_run` already recorded — what a
        /// `--further-incident` record writes.
        ///
        /// Two such records are siblings *of each other*, in the sense CONTEXT.md's *Further
        /// incident* entry defines. Every bare "sibling" in this file means that relation — a
        /// run-mate — never the cluster-mate sense ADR 0002 uses when it says a friction sibling
        /// cannot lower a `material_recurrence` bar.
        ///
        /// Siblings carry their own event identity and their own symptom, and share the run's
        /// group, task label, and task fingerprint. The caller passes the run's session, since
        /// one run is one session.
        fn further_incident_event(
            &self,
            serial: usize,
            of_run: usize,
            outcome: &str,
            symptom_key: Option<&str>,
            session_id: &str,
        ) -> Value {
            let mut event = self.use_event(serial, outcome, symptom_key, session_id);
            event["payload"]["task_label"] = json!(format!("task {of_run}"));
            event["payload"]["task_fingerprint"] = json!(format!("fingerprint-{of_run}"));
            event["payload"]["same_run_group"] = json!(format!("run-{of_run}"));
            event
        }
    }

    /// The denominator measures how much use this exact target version has seen, and one run
    /// is one use however many ways it deviated. Counting records instead would let an
    /// operator who recorded a run honestly advance it toward `ten_use_unresolved` faster than
    /// one who compressed the same run into a single receipt — recording would manufacture its
    /// own authorization.
    #[test]
    fn qualifying_uses_count_run_groups_rather_than_records() {
        let fixture = Fixture::new();
        fixture.write_events(&[
            fixture.use_event(1, "friction", Some("execution"), "session-a"),
            fixture.further_incident_event(2, 1, "friction", Some("output"), "session-a"),
            fixture.further_incident_event(3, 1, "friction", Some("state"), "session-a"),
            fixture.use_event(4, "clean", None, "session-b"),
        ]);

        let status = fixture.derive("session-c", 1_767_398_400_000);
        assert_eq!(
            status.qualifying_uses_on_current_hash, 2,
            "three siblings record one run, so two runs are two uses"
        );
        assert_eq!(
            status.open_incident_ids,
            ["evt_1", "evt_2", "evt_3"],
            "every sibling stays separately addressable in the ledger"
        );
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

    /// The whole point of recording a run's deviations separately is that a close can name
    /// exactly the one no trial could express. That is worth nothing if the name reaches the
    /// run: the sibling that a same-hash predecessor already confirmed as a real target defect
    /// would be gate-retired along with it, which is the cost `evt_41dfe1f4` actually paid.
    ///
    /// Sharing a run group is a statement about how the evidence was recorded, never about what
    /// a review could decide. Retirement follows the names and nothing else.
    #[test]
    fn naming_one_sibling_untestable_leaves_its_siblings_clustering() {
        let fixture = Fixture::new();
        let mut events = vec![
            fixture.use_event(1, "friction", Some("execution"), "session-a"),
            fixture.further_incident_event(2, 1, "friction", Some("execution"), "session-a"),
            fixture.use_event(3, "friction", Some("execution"), "session-b"),
        ];
        events.extend(fixture.review_naming_untestable_coverage(
            "rev_1",
            "closed_no_skill_defect",
            &["evt_2"],
            &["evt_2"],
        ));
        fixture.write_events(&events);

        let status = fixture.derive("session-c", 1_767_398_400_000);
        assert_eq!(
            status.open_incident_ids,
            ["evt_1", "evt_2", "evt_3"],
            "a named event was never adjudicated, so it stays open in the ledger"
        );
        assert_eq!(
            status.instrument_limited_incident_ids,
            ["evt_2"],
            "retirement reaches the name, not the run group it belongs to"
        );
        let cluster = status
            .candidate_clusters
            .iter()
            .find(|cluster| cluster.symptom_key == "execution")
            .expect("the unnamed incidents still cluster");
        assert_eq!(cluster.open_event_ids, ["evt_1", "evt_3"]);
        assert_eq!(cluster.independent_incidents, 2);
        assert_eq!(status.qualifying_uses_on_current_hash, 2);
    }

    /// Independence is what makes a recurrence claim mean anything, and it is derived from the
    /// top-level session and the task fingerprint — both of which siblings share, because they
    /// record one run. Letting a run reach a threshold by deviating twice would say two
    /// independent incidents recurred when one run misbehaved once.
    ///
    /// The contrast is
    /// `two_independent_material_failures_fire_the_material_recurrence_gate`: the same two
    /// records from distinct sessions do fire.
    #[test]
    fn siblings_of_one_run_cannot_reach_material_recurrence() {
        let fixture = Fixture::new();
        fixture.write_events(&[
            fixture.use_event(1, "material_failure", Some("output"), "session-a"),
            fixture.further_incident_event(2, 1, "material_failure", Some("output"), "session-a"),
        ]);

        let status = fixture.derive("session-b", 1_767_398_400_000);
        assert_eq!(status.state, "collecting");
        assert_eq!(status.authorization_reason, None);
        assert_eq!(status.trigger_event_ids, [] as [String; 0]);
        let cluster = status
            .candidate_clusters
            .iter()
            .find(|cluster| cluster.symptom_key == "output")
            .expect("the siblings cluster on their shared symptom");
        assert_eq!(cluster.open_event_ids, ["evt_1", "evt_2"]);
        assert_eq!(
            cluster.independent_incidents, 1,
            "one run contributes one independent incident however many ways it deviated"
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

    #[test]
    fn a_ten_use_blocked_close_leaves_retrospective_incidents_in_the_anchor_cluster() {
        let fixture = Fixture::new();
        let mut events = vec![fixture.use_event(1, "friction", Some("cost"), "session-1")];
        let mut retrospective = fixture.use_event(2, "friction", Some("cost"), "session-2");
        retrospective["payload"]["retrospective"] = json!(true);
        retrospective["payload"]["evidence_refs"] = json!(["logs/retrospective-cost.txt"]);
        events.push(retrospective);
        events.extend(
            (3..=10).map(|serial| {
                fixture.use_event(serial, "clean", None, &format!("session-{serial}"))
            }),
        );
        events.extend(fixture.review_with_authorizing_rule(
            "review-blocked",
            "blocked_no_valid_test",
            &["evt_1"],
            "ten_use_unresolved",
        ));
        fixture.write_events(&events);

        let status = fixture.derive("fresh-session", 1_767_398_400_000);
        assert_eq!(status.instrument_limited_incident_ids, ["evt_1"]);
        assert_eq!(
            status.candidate_clusters[0].open_event_ids,
            ["evt_2"],
            "ten-use retirement names only contemporaneous incidents in the anchor cluster"
        );
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
        let mut retrospective =
            fixture.use_event(1, "severe_incident", Some("execution"), "session-1");
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
        let mut events =
            vec![fixture.use_event(1, "severe_incident", Some("execution"), "session-1")];
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
    fn an_adjudicating_close_leaves_coverage_it_named_untestable_open_and_unclustered() {
        let fixture = Fixture::new();
        let mut events = vec![
            fixture.use_event(1, "friction", Some("execution"), "session-1"),
            fixture.use_event(2, "friction", Some("execution"), "session-2"),
        ];
        events.extend(fixture.review_naming_untestable_coverage(
            "review-mixed",
            "candidate_rejected_validation",
            &["evt_1", "evt_2"],
            &["evt_1"],
        ));
        fixture.write_events(&events);

        let status = fixture.derive("fresh-session", 1_767_398_400_000);

        assert_eq!(
            status.open_incident_ids,
            vec!["evt_1".to_owned()],
            "the review reached no conclusion about evt_1, so it stays open"
        );
        assert_eq!(
            status.instrument_limited_incident_ids,
            vec!["evt_1".to_owned()],
            "evt_1 retires from the gate as untestable rather than as adjudicated"
        );
        assert!(
            status.candidate_clusters.is_empty(),
            "an incident retired as untestable can never reach a threshold again"
        );
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
        let mut retrospective =
            fixture.use_event(3, "friction", Some("cost"), "retrospective-session");
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

        let mut events = fixture.recorded_events();
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

    /// Evidence accumulates against one exact version of a target, so uses recorded
    /// against a hash the target no longer carries say nothing about the target as it
    /// stands — editing a skill starts a fresh accumulation rather than inheriting the
    /// old one.
    ///
    /// `tests/gate_contract.rs` reaches the same rule through a real store, where the
    /// hash is computed from the target's own content rather than supplied. What only
    /// that test can pin is the computed identity; what this one pins is the rule, with
    /// nothing computed and nothing on disk.
    #[test]
    fn evidence_recorded_against_another_hash_does_not_count_toward_this_one() {
        let fixture = Fixture::new();
        fixture.write_events(&[
            fixture.use_event(1, "friction", Some("execution"), "session-1"),
            fixture.use_event(2, "friction", Some("execution"), "session-2"),
        ]);

        let status =
            fixture.derive_against("a-hash-the-events-do-not-carry", "fresh", 1_767_398_400_000);
        assert_eq!(status.qualifying_uses_on_current_hash, 0);
        assert_eq!(status.open_incident_ids, Vec::<String>::new());
        assert_eq!(status.state, "closed");
        assert_eq!(status.authorization_reason, None);
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

    /// ADR 0002's carve-out reaches the naming channel too: a contemporaneous severe incident
    /// authorizes on its own, so listing it as retired would have the projection claim it
    /// stopped driving the gate while it demonstrably still does. Naming it therefore stops it
    /// being adjudicated and nothing more — and the gate keeps re-authorizing, which is the
    /// safety claim rather than a defect.
    #[test]
    fn naming_a_contemporaneous_severe_incident_stops_adjudication_without_retiring_it() {
        let fixture = Fixture::new();
        let mut events =
            vec![fixture.use_event(1, "severe_incident", Some("execution"), "session-1")];
        events.extend(fixture.review_naming_untestable_coverage(
            "review-severe",
            "monitor_for_recurrence",
            &["evt_1"],
            &["evt_1"],
        ));
        fixture.write_events(&events);

        let status = fixture.derive("fresh-session", 1_767_398_400_000);

        assert_eq!(status.open_incident_ids, vec!["evt_1".to_owned()]);
        assert_eq!(
            status.instrument_limited_incident_ids,
            Vec::<String>::new(),
            "it never left the gate, so the projection must not say it did"
        );
        assert_eq!(
            status.authorized_workflow.as_deref(),
            Some("skill-evolution"),
            "and it still authorizes on its own"
        );
    }

    /// The close refuses to name coverage it does not hold. Derivation must not honour it
    /// either, or a hand-edited stream retires an incident the review never accounted for —
    /// the widening the command exists to prevent.
    #[test]
    fn untestable_coverage_outside_the_coverage_list_retires_nothing() {
        let fixture = Fixture::new();
        let mut events = vec![
            fixture.use_event(1, "friction", Some("execution"), "session-1"),
            fixture.use_event(2, "friction", Some("execution"), "session-2"),
        ];
        events.extend(fixture.review_naming_untestable_coverage(
            "review-overreaching",
            "monitor_for_recurrence",
            &["evt_1"],
            &["evt_2"],
        ));
        fixture.write_events(&events);

        let status = fixture.derive("fresh-session", 1_767_398_400_000);

        assert_eq!(
            status.instrument_limited_incident_ids,
            Vec::<String>::new(),
            "evt_2 is outside the coverage list, so this close established nothing about it"
        );
        assert_eq!(
            status.open_incident_ids,
            vec!["evt_2".to_owned()],
            "and it stays open, driving the gate as before"
        );
        assert_eq!(
            status.candidate_clusters.len(),
            1,
            "and stays in its cluster"
        );
    }

    #[test]
    fn untestable_coverage_refuses_a_malformed_shape_on_read() {
        for (case, invalid) in [
            ("not an array", json!("evt_1")),
            ("non-string member", json!([42])),
            ("empty array", json!([])),
            ("empty member", json!([""])),
        ] {
            let fixture = Fixture::new();
            let mut events = vec![fixture.use_event(1, "friction", Some("execution"), "session-1")];
            let mut review =
                fixture.review("review-malformed", "monitor_for_recurrence", &["evt_1"]);
            review[1]["payload"]["instrument_limited_event_ids"] = invalid;
            events.extend(review);
            fixture.write_events(&events);

            let status = fixture.derive("fresh-session", 1_767_398_400_000);

            assert_eq!(status.state, "blocked", "case {case}");
            assert!(
                status
                    .integrity_errors
                    .iter()
                    .any(|error| error.contains("instrument_limited_event_ids")),
                "case {case}: a shape the reader cannot trust must not be silently read as an empty narrowing: {:?}",
                status.integrity_errors
            );
        }
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
}
