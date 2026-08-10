//! The command surface, written once and mounted under whatever the host calls
//! its binary.
//!
//! A host declares its own top-level command tree and hangs [`SkillsArgs`] off
//! it wherever it likes — `playbench skills …`, `mundifold skills …` — then
//! hands [`run`] the [`Host`] it built. Nothing here knows which repository it
//! is running in beyond what that value says.
//!
//! The split between this module and the library is the split between an
//! operator's request and the lifecycle itself. Argument shapes, missing-input
//! diagnostics, clock and identifier generation, and the two `git` calls that
//! discover a repository live here. Everything that reads or writes evidence
//! lives in the library and takes its inputs as data.

use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Command as ProcessCommand,
};

use clap::{Args, Subcommand};

use crate::Host;

/// Every status this surface reports.
///
/// A host maps these onto its own process exit codes. The numbers in the
/// comments are what both current hosts use; this crate states the meaning and
/// lets the host own the number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exit {
    /// The command completed. (`0`)
    Success,
    /// The command could not complete safely; its effect may be incomplete. (`1`)
    UnsafeFailure,
    /// The command refused: an input or precondition was not admissible. (`3`)
    Refusal,
}

/// A request this surface rejected before the lifecycle saw it.
enum CliError {
    /// A required argument was absent. The message names what is missing.
    MissingInput(String),
    /// The lifecycle refused the request or could not complete it.
    Lifecycle(crate::Error),
}

impl From<crate::Error> for CliError {
    fn from(error: crate::Error) -> Self {
        Self::Lifecycle(error)
    }
}

impl CliError {
    /// Writes the diagnostic and reports the status it carries.
    ///
    /// A lifecycle error that names a [`Recovery`](crate::Recovery) gets the host's sentence
    /// appended here, which is the one place the two halves of that message
    /// meet.
    fn report(&self, err: &mut impl Write, host: &Host) -> Exit {
        match self {
            Self::MissingInput(message) => {
                write_line(err, message);
                Exit::Refusal
            }
            Self::Lifecycle(error) => {
                let mut line = error.to_string();
                if let Some(recovery) = error.recovery() {
                    line.push(' ');
                    line.push_str(&host.recovery_instruction(recovery));
                }
                write_line(err, &line);
                match error.class() {
                    crate::ErrorClass::Refusal => Exit::Refusal,
                    crate::ErrorClass::UnsafeFailure => Exit::UnsafeFailure,
                }
            }
        }
    }
}

/// The `skills` command group, mountable by a host.
#[derive(Debug, Args)]
pub struct SkillsArgs {
    #[command(subcommand)]
    command: Box<SkillsCommand>,
}

#[derive(Debug, Subcommand)]
pub enum SkillsCommand {
    Evidence {
        #[command(subcommand)]
        command: Box<SkillEvidenceCommand>,
    },
    Evolution {
        #[command(subcommand)]
        command: Box<SkillEvolutionCommand>,
    },
    MethodGapResearchStatus(MethodGapResearchStatusArgs),
    EvolutionStatus(EvolutionStatusArgs),
}

#[derive(Debug, Args)]
pub struct MethodGapResearchStatusArgs {
    family: String,
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(long)]
    now_epoch_milliseconds: i64,
}

#[derive(Debug, Args)]
pub struct EvolutionStatusArgs {
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(long)]
    now_epoch_milliseconds: i64,
    #[arg(long)]
    session_id: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum SkillEvidenceCommand {
    Derive {
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long)]
        session_id: Option<String>,
    },
    Hash {
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        root: Option<PathBuf>,
    },
    Record(Box<SkillEvidenceRecordArgs>),
    /// Writes this crate's skill packages and schemas into a repository.
    Install(InstallArgs),
    /// Removes packages this crate shipped and has since retired.
    Withdraw(WithdrawArgs),
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[arg(long)]
    root: Option<PathBuf>,
    /// Overwrite packages that already differ from the ones shipped here.
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Args)]
pub struct WithdrawArgs {
    #[arg(long)]
    root: Option<PathBuf>,
    /// Remove retired files that differ from their last shipped contents.
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Subcommand)]
pub enum SkillEvolutionCommand {
    Preflight(LifecycleContextArgs),
    Claim(EvolutionClaimArgs),
    RecordValidation(EvolutionRecordValidationArgs),
    Land(EvolutionLandArgs),
    Close(EvolutionCloseArgs),
}

#[derive(Debug, Args)]
pub struct LifecycleContextArgs {
    #[arg(long)]
    target: Option<PathBuf>,
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(long)]
    recorded_at: Option<String>,
    #[arg(long)]
    now_epoch_milliseconds: Option<i64>,
    #[arg(long)]
    session_id: Option<String>,
    #[arg(long)]
    lock_owner: Option<String>,
}

#[derive(Debug, Args)]
pub struct LifecycleEventArgs {
    #[command(flatten)]
    context: LifecycleContextArgs,
    #[arg(long)]
    event_id: Option<String>,
    #[arg(long)]
    repository_head: Option<String>,
}

#[derive(Debug, Args)]
pub struct EvolutionClaimArgs {
    #[command(flatten)]
    event: LifecycleEventArgs,
    #[arg(long)]
    review_id: Option<String>,
    #[arg(long, default_value = "provisional")]
    risk_tier: String,
    /// Record the computed content hash of the operating Skill Evolution package.
    #[arg(long)]
    record_operating_skill_hash: bool,
}

#[derive(Debug, Args)]
pub struct EvolutionRecordValidationArgs {
    #[command(flatten)]
    event: LifecycleEventArgs,
    #[arg(long)]
    review_id: Option<String>,
    #[arg(long)]
    decision: Option<String>,
    #[arg(long)]
    risk_tier: Option<String>,
    #[arg(long)]
    candidate: Option<PathBuf>,
    #[arg(long)]
    trials: Option<String>,
    #[arg(long)]
    artifacts: Option<String>,
    #[arg(long)]
    summary: Option<String>,
}

#[derive(Debug, Args)]
pub struct EvolutionLandArgs {
    #[command(flatten)]
    event: LifecycleEventArgs,
    #[arg(long)]
    review_id: Option<String>,
    #[arg(long)]
    candidate: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct EvolutionCloseArgs {
    #[command(flatten)]
    event: LifecycleEventArgs,
    #[arg(long)]
    review_id: Option<String>,
    #[arg(long)]
    disposition: Option<String>,
    #[arg(long)]
    note: Option<String>,
    #[arg(long)]
    adjudicate: Vec<String>,
    #[arg(long)]
    instrument_limited: Vec<String>,
    #[arg(long)]
    trials: Option<String>,
    #[arg(long)]
    artifacts: Option<String>,
}

#[derive(Debug, Args)]
pub struct SkillEvidenceRecordArgs {
    #[arg(long)]
    target: PathBuf,
    #[arg(long)]
    outcome: String,
    #[arg(long)]
    task_label: String,
    #[arg(long)]
    symptom_key: Option<String>,
    #[arg(long)]
    expected: Option<String>,
    #[arg(long)]
    observed: Option<String>,
    #[arg(long)]
    consequence: Option<String>,
    #[arg(long)]
    workaround: Option<String>,
    #[arg(long)]
    run_condition: Option<String>,
    #[arg(long)]
    retrospective: bool,
    #[arg(long)]
    evidence_ref: Vec<String>,
    #[arg(long)]
    session_id: Option<String>,
    #[arg(long)]
    same_run_group: Option<String>,
    #[arg(long)]
    root: Option<PathBuf>,
    #[arg(long)]
    human: bool,
}

/// Runs one parsed `skills` invocation and reports its status.
///
/// For a host whose whole `skills` group is this one — see
/// `src/bin/skill-evidence.rs`:
///
/// ```ignore
/// enum Command {
///     Skills(cli::SkillsArgs),
/// }
/// # let exit = cli::run(args, &host(), &mut out, &mut err);
/// ```
///
/// A host with `skills` subcommands of its own flattens [`SkillsCommand`] into
/// its own enum and calls [`run_command`] instead.
pub fn run(args: SkillsArgs, host: &Host, out: &mut impl Write, err: &mut impl Write) -> Exit {
    run_command(*args.command, host, out, err)
}

/// Runs one `skills` subcommand this crate owns, for a host that mixes them
/// with its own.
///
/// ```text
/// #[derive(clap::Subcommand)]
/// enum SkillsCommand {
///     Inspect { … },              // the host's own
///     #[command(flatten)]
///     Shared(cli::SkillsCommand), // every subcommand this crate owns
/// }
/// ```
///
/// Flattening is why the argument types here are public. They are a command
/// surface, not an API to build on: a host names [`SkillsCommand`] and nothing
/// below it.
pub fn run_command(
    command: SkillsCommand,
    host: &Host,
    out: &mut impl Write,
    err: &mut impl Write,
) -> Exit {
    match dispatch(command, host, out) {
        Ok(exit) => exit,
        Err(error) => error.report(err, host),
    }
}

fn dispatch(command: SkillsCommand, host: &Host, out: &mut impl Write) -> Result<Exit, CliError> {
    match command {
        SkillsCommand::Evidence { command } => run_skill_evidence(*command, host, out),
        SkillsCommand::Evolution { command } => run_skill_evolution(*command, host, out),
        SkillsCommand::MethodGapResearchStatus(args) => {
            run_method_gap_research_status(args, host, out)
        }
        SkillsCommand::EvolutionStatus(args) => run_evolution_status(args, out),
    }
}

fn run_evolution_status(args: EvolutionStatusArgs, out: &mut impl Write) -> Result<Exit, CliError> {
    let session_id = resolve_session_id(args.session_id.as_deref())?;
    let report = crate::skill_evolution_status(
        &selected_root(args.root),
        args.now_epoch_milliseconds,
        &session_id,
    )?;
    // Already a rendered terminal reply, not a report to serialize.
    write!(out, "{report}").expect("writing to the output stream cannot fail");
    Ok(Exit::Success)
}

fn run_method_gap_research_status(
    args: MethodGapResearchStatusArgs,
    host: &Host,
    out: &mut impl Write,
) -> Result<Exit, CliError> {
    let report = crate::method_gap_research_inventory(
        &selected_root(args.root),
        &args.family,
        args.now_epoch_milliseconds,
        host,
    )?;
    // The one indented report. Its bytes are pinned by
    // `fixtures/skill-evidence/status-reporters-v1/method-gap-research-status.expected.json`,
    // a golden carried over from the superseded JavaScript reporter, so the
    // indentation is part of the published inventory rather than a rendering
    // accident. Compacting it would move emitted bytes.
    write_report(&report, out, ReportRendering::Indented);
    Ok(Exit::Success)
}

fn run_skill_evolution(
    command: SkillEvolutionCommand,
    host: &Host,
    out: &mut impl Write,
) -> Result<Exit, CliError> {
    match command {
        SkillEvolutionCommand::Preflight(args) => {
            let (root, target, inputs) = lifecycle_context_inputs(args, "skill-evolution", host)?;
            let receipt = crate::evolution_preflight(&root, &target, &inputs)?;
            Ok(print_successful_report(&receipt, out))
        }
        SkillEvolutionCommand::Claim(args) => {
            let EvolutionClaimArgs {
                event,
                review_id,
                risk_tier,
                record_operating_skill_hash,
            } = args;
            let (root, target, inputs) = lifecycle_event_inputs(event, "skill-evolution", host)?;
            let request = crate::EvolutionClaimRequest {
                review_id: review_id.unwrap_or_default(),
                risk_tier,
                record_operating_skill_hash,
            };
            let receipt = crate::evolution_claim(&root, &target, &request, &inputs)?;
            Ok(print_successful_report(&receipt, out))
        }
        SkillEvolutionCommand::RecordValidation(args) => {
            let EvolutionRecordValidationArgs {
                event,
                review_id,
                decision,
                risk_tier,
                candidate,
                trials,
                artifacts,
                summary,
            } = args;
            let (root, target, inputs) = lifecycle_event_inputs(event, "skill-evolution", host)?;
            let candidate = required_lifecycle_skill_path(candidate)?;
            let request = crate::EvolutionValidationRequest {
                review_id: review_id.unwrap_or_default(),
                decision: decision.unwrap_or_default(),
                risk_tier: risk_tier.unwrap_or_default(),
                candidate,
                trials: trials.unwrap_or_default(),
                artifacts: artifacts.unwrap_or_default(),
                summary,
            };
            let receipt = crate::evolution_record_validation(&root, &target, &request, &inputs)?;
            Ok(print_successful_report(&receipt, out))
        }
        SkillEvolutionCommand::Land(args) => {
            let EvolutionLandArgs {
                event,
                review_id,
                candidate,
            } = args;
            let (root, target, inputs) = lifecycle_event_inputs(event, "skill-evolution", host)?;
            let candidate = required_lifecycle_skill_path(candidate)?;
            let request = crate::EvolutionLandRequest {
                review_id: review_id.unwrap_or_default(),
                candidate,
            };
            let receipt = crate::evolution_land(&root, &target, &request, &inputs)?;
            Ok(print_successful_report(&receipt, out))
        }
        SkillEvolutionCommand::Close(args) => {
            let EvolutionCloseArgs {
                event,
                review_id,
                disposition,
                note,
                adjudicate,
                instrument_limited,
                trials,
                artifacts,
            } = args;
            let (root, target, inputs) = lifecycle_event_inputs(event, "skill-evolution", host)?;
            let request = crate::EvolutionCloseRequest {
                review_id: review_id.unwrap_or_default(),
                disposition: disposition.unwrap_or_default(),
                note: note.unwrap_or_default(),
                adjudicate,
                instrument_limited,
                trials,
                artifacts,
            };
            let receipt = crate::evolution_close(&root, &target, &request, &inputs)?;
            Ok(print_successful_report(&receipt, out))
        }
    }
}

fn required_lifecycle_skill_path(path: Option<PathBuf>) -> Result<PathBuf, CliError> {
    path.ok_or_else(|| CliError::MissingInput("Missing required --target <skill-dir>.".to_owned()))
}

fn required_lifecycle_input<T>(value: Option<T>, flag: &str) -> Result<T, CliError> {
    value.ok_or_else(|| CliError::MissingInput(format!("Missing required {flag}.")))
}

fn lifecycle_context_inputs(
    args: LifecycleContextArgs,
    operator_skill_name: &str,
    host: &Host,
) -> Result<(PathBuf, PathBuf, crate::LifecycleInputs), CliError> {
    let LifecycleContextArgs {
        target,
        root,
        recorded_at,
        now_epoch_milliseconds,
        session_id,
        lock_owner,
    } = args;
    let root = selected_root(root);
    let target = required_lifecycle_skill_path(target)?;
    Ok((
        root,
        target,
        crate::LifecycleInputs {
            recorded_at: required_lifecycle_input(recorded_at, "--recorded-at")?,
            now_epoch_milliseconds: required_lifecycle_input(
                now_epoch_milliseconds,
                "--now-epoch-milliseconds",
            )?,
            session_id: required_lifecycle_input(session_id, "--session-id")?,
            lock_owner: required_lifecycle_input(lock_owner, "--lock-owner")?,
            operator_skill: host.operator_skill(operator_skill_name),
        },
    ))
}

fn lifecycle_event_inputs(
    args: LifecycleEventArgs,
    operator_skill_name: &str,
    host: &Host,
) -> Result<(PathBuf, PathBuf, crate::LifecycleEventInputs), CliError> {
    let LifecycleEventArgs {
        context,
        event_id,
        repository_head,
    } = args;
    let (root, target, context) = lifecycle_context_inputs(context, operator_skill_name, host)?;
    Ok((
        root,
        target,
        crate::LifecycleEventInputs {
            event_id: required_lifecycle_input(event_id, "--event-id")?,
            recorded_at: context.recorded_at,
            now_epoch_milliseconds: context.now_epoch_milliseconds,
            repository_head: required_lifecycle_input(repository_head, "--repository-head")?,
            session_id: context.session_id,
            lock_owner: context.lock_owner,
            operator_skill: context.operator_skill,
        },
    ))
}

fn run_skill_evidence(
    command: SkillEvidenceCommand,
    host: &Host,
    out: &mut impl Write,
) -> Result<Exit, CliError> {
    match command {
        SkillEvidenceCommand::Derive {
            target,
            root,
            session_id,
        } => {
            let root = selected_root(root);
            let now = time::OffsetDateTime::now_utc();
            let generated_at = now
                .format(&time::format_description::well_known::Rfc3339)
                .expect("the current UTC time always formats as RFC 3339");
            let session_id = resolve_session_id(session_id.as_deref())?;
            let inputs = crate::DerivationInputs {
                generated_at,
                now_epoch_milliseconds: i64::try_from(now.unix_timestamp_nanos() / 1_000_000)
                    .expect("current timestamp milliseconds fit in i64"),
                session_id,
                lock_owner: format!("lock_{}", uuid::Uuid::new_v4()),
            };
            let status = crate::derive_store(&root, &target, &inputs)?;
            Ok(print_successful_report(&status, out))
        }
        SkillEvidenceCommand::Hash { target, root } => {
            let report = crate::hash_skill(&selected_root(root), &target, host)?;
            Ok(print_successful_report(&report, out))
        }
        SkillEvidenceCommand::Install(args) => {
            let root = selected_root(args.root);
            let receipt = crate::assets::install(&root, host, args.force)?;
            Ok(print_successful_report(&receipt, out))
        }
        SkillEvidenceCommand::Withdraw(args) => {
            let root = selected_root(args.root);
            let receipt = crate::assets::withdraw(&root, host, args.force)?;
            Ok(print_successful_report(&receipt, out))
        }
        SkillEvidenceCommand::Record(args) => {
            let SkillEvidenceRecordArgs {
                target,
                outcome,
                task_label,
                symptom_key,
                expected,
                observed,
                consequence,
                workaround,
                run_condition,
                retrospective,
                evidence_ref,
                session_id,
                same_run_group,
                root,
                human,
            } = *args;
            let root = selected_root(root);
            let now = time::OffsetDateTime::now_utc();
            let recorded_at = now
                .format(&time::format_description::well_known::Rfc3339)
                .expect("the current UTC time always formats as RFC 3339");
            let request = crate::RecordUseRequest {
                outcome,
                task_label,
                symptom_key,
                expected,
                observed,
                consequence,
                workaround,
                run_condition,
                retrospective,
                evidence_refs: evidence_ref,
                same_run_group,
            };
            let session_id = resolve_session_id(session_id.as_deref())?;
            let inputs = crate::RecordInputs {
                event_id: format!("evt_{}", uuid::Uuid::new_v4()),
                recorded_at,
                now_epoch_milliseconds: i64::try_from(now.unix_timestamp_nanos() / 1_000_000)
                    .expect("current timestamp milliseconds fit in i64"),
                repository_head: repository_head(&root),
                session_id,
                lock_owner: format!("lock_{}", uuid::Uuid::new_v4()),
            };
            let receipt = crate::record_use(&root, &target, &request, &inputs, host)?;
            if human {
                // A rendered terminal reply, not a report to serialize.
                write_line(out, &receipt.terminal_reply);
                return Ok(Exit::Success);
            }
            Ok(print_successful_report(&receipt, out))
        }
    }
}

fn repository_head(root: &Path) -> String {
    ProcessCommand::new("git")
        .args(["-C"])
        .arg(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|head| head.trim().to_owned())
        .filter(|head| !head.is_empty())
        .unwrap_or_else(|| "unavailable".to_owned())
}

fn resolve_session_id(explicit: Option<&str>) -> Result<String, crate::Error> {
    let claude = std::env::var("CLAUDE_CODE_SESSION_ID").ok();
    let codex = std::env::var("CODEX_THREAD_ID").ok();
    crate::resolve_top_level_session_id(explicit, claude.as_deref(), codex.as_deref())
}

/// The repository the operator meant, defaulting to the enclosing checkout.
fn selected_root(root: Option<PathBuf>) -> PathBuf {
    if let Some(root) = root {
        return root;
    }
    ProcessCommand::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|root| PathBuf::from(root.trim()))
        .filter(|root| !root.as_os_str().is_empty())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// How a report is serialized.
enum ReportRendering {
    /// One compact line. Every report but the Method-Gap inventory.
    Compact,
    /// Indented. See [`run_method_gap_research_status`] for why one report is.
    Indented,
}

/// The one place a report becomes bytes.
fn write_report(report: &impl serde::Serialize, out: &mut impl Write, rendering: ReportRendering) {
    let rendered = match rendering {
        ReportRendering::Compact => serde_json::to_string(report),
        ReportRendering::Indented => serde_json::to_string_pretty(report),
    }
    .expect("serializing a report cannot fail");
    write_line(out, &rendered);
}

/// Emits a report that exists only when its command succeeded.
///
/// These calls return a receipt or a [`CliError`]; there is no
/// failed-but-renderable report on this path, so the status is always success.
fn print_successful_report(report: &impl serde::Serialize, out: &mut impl Write) -> Exit {
    write_report(report, out, ReportRendering::Compact);
    Exit::Success
}

/// Writes one line, panicking as `println!` does when the stream rejects it.
fn write_line(sink: &mut impl Write, line: &str) {
    writeln!(sink, "{line}").expect("writing a line to the stream cannot fail");
}
