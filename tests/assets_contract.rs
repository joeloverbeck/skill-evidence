use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

use skill_evidence::{Host, assets};

const RETIRED_PACKAGE: &str = "legacy-skill-decontamination";
const RETIRED_FILES: &[(&str, &str)] = &[
    (
        "SKILL.md",
        include_str!("../assets/retired-skills/legacy-skill-decontamination/SKILL.md"),
    ),
    (
        "agents/openai.yaml",
        include_str!("../assets/retired-skills/legacy-skill-decontamination/agents/openai.yaml"),
    ),
    (
        "references/eligible-run.md",
        include_str!(
            "../assets/retired-skills/legacy-skill-decontamination/references/eligible-run.md"
        ),
    ),
];

fn host() -> Host {
    Host {
        namespace: "demo".to_owned(),
        command: "demo".to_owned(),
        cargo_package: "demo-cli".to_owned(),
        skills_directory: PathBuf::from("/nonexistent/.claude/skills"),
    }
}

fn rendered_for(template: &str, command: &str, cargo_package: &str, namespace: &str) -> String {
    template
        .replace("{{cargo_package}}", cargo_package)
        .replace("{{command}}", command)
        .replace("{{namespace}}", namespace)
}

fn write_pristine_retired_package(root: &std::path::Path) {
    write_retired_package_for(root, "demo", "demo-cli", "demo");
}

fn write_reference_host_retired_package(root: &std::path::Path) {
    write_retired_package_for(root, "skill-evidence", "skill-evidence", "skill-evidence");
}

fn write_retired_package_for(
    root: &std::path::Path,
    command: &str,
    cargo_package: &str,
    namespace: &str,
) {
    let package = root.join(".claude/skills").join(RETIRED_PACKAGE);
    for (relative_path, template) in RETIRED_FILES {
        let destination = package.join(relative_path);
        fs::create_dir_all(destination.parent().expect("retired file parent"))
            .expect("create retired file parent");
        fs::write(
            destination,
            rendered_for(template, command, cargo_package, namespace),
        )
        .expect("write retired file");
    }

    #[cfg(unix)]
    {
        let links = root.join(".agents/skills");
        fs::create_dir_all(&links).expect("create discovery directory");
        std::os::unix::fs::symlink(
            format!("../../.claude/skills/{RETIRED_PACKAGE}"),
            links.join(RETIRED_PACKAGE),
        )
        .expect("create retired discovery link");
    }
}

fn snapshot_tree(root: &std::path::Path) -> BTreeMap<String, Vec<u8>> {
    let mut snapshot = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let mut entries = fs::read_dir(&directory)
            .expect("read snapshot directory")
            .collect::<Result<Vec<_>, _>>()
            .expect("read snapshot entries");
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let relative = path
                .strip_prefix(root)
                .expect("snapshot path below root")
                .to_string_lossy()
                .replace('\\', "/");
            let metadata = fs::symlink_metadata(&path).expect("snapshot metadata");
            if metadata.file_type().is_dir() {
                snapshot.insert(relative, b"directory".to_vec());
                pending.push(path);
            } else if metadata.file_type().is_symlink() {
                snapshot.insert(
                    relative,
                    format!(
                        "link:{}",
                        fs::read_link(&path).expect("snapshot link").display()
                    )
                    .into_bytes(),
                );
            } else {
                snapshot.insert(relative, fs::read(path).expect("snapshot file"));
            }
        }
    }
    snapshot
}

#[test]
fn installed_live_packages_do_not_point_to_nonexistent_archive_paths() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");

    let capture = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evidence-capture/SKILL.md"),
    )
    .expect("read installed capture package");
    assert!(
        capture.contains(
            "No report file is produced for ordinary capture; markdown reports belong to evolution runs."
        ),
        "removing the maintainer-only pointer must retain the capture package's reporting guidance"
    );

    let mut offenders = Vec::new();
    for package in ["skill-evidence-capture", "skill-evolution"] {
        let installed = fs::read_to_string(
            root.path()
                .join(".claude/skills")
                .join(package)
                .join("SKILL.md"),
        )
        .expect("read installed live package");
        if installed.contains("archive/") {
            offenders.push(package);
        }
    }

    assert_eq!(
        offenders,
        Vec::<&str>::new(),
        "live installed packages must not point to archive paths that do not exist in the authoring repository"
    );
}

/// Capture is where a run's deviations become addressable evidence or stop being
/// addressable at all. A recorder told to fix exactly one outcome and one symptom key, with
/// nothing said about a run that deviated several ways, compresses them into one record and
/// the reviewer inherits a trigger it can only retire whole.
///
/// The package has to say this without becoming a diagnoser. Recording deviations apart is
/// a description of what the session shows — it asserts nothing about their causes in
/// either direction, and the boundary that forbids claiming a shared cause has to forbid
/// the converse just as plainly, or the guidance itself becomes the diagnosis.
#[test]
fn installed_capture_package_records_one_incident_per_deviation_without_diagnosing() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evidence-capture/SKILL.md"),
    )
    .expect("read installed capture package");

    assert!(
        package.contains("one record per deviation"),
        "the package must say when a run yields more than one record"
    );
    assert!(
        package.contains("--further-incident"),
        "the package must name the flag that declares a sibling"
    );
    assert!(
        package.contains("same `--task-label`"),
        "a sibling shares its run's task label, which is what puts it in the run's group"
    );
    assert!(
        package.contains(
            "never asserts that two deviations share a cause, and never asserts that they do not"
        ),
        "recording deviations apart must stay description rather than causal judgment"
    );
    assert!(
        package.contains("one qualifying use"),
        "the package must say that siblings do not multiply the use they came from"
    );
    assert!(
        package.contains("never combines with `--same-run-group`"),
        "the two flags declare different things and the package must not blur them"
    );
    assert!(
        !package.contains("Cost ceiling: one compiled command plus"),
        "the cost ceiling must account for a run that records more than once"
    );
}

/// Once a run can take several commands to record, a refusal on the third has no terminal
/// state in a package whose states are "everything was appended" or "nothing was written".
/// The operator following it reports that nothing was written over records already in an
/// append-only stream — the one claim the substrate can never let a report make, because
/// nothing will remove them and the next reader takes the stream as it stands.
#[test]
fn installed_capture_package_states_the_partial_append_terminal_state() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evidence-capture/SKILL.md"),
    )
    .expect("read installed capture package");

    assert!(
        package.contains("records already appended stand"),
        "a later refusal must not be reported as though the earlier records were not written"
    );
    assert!(
        package.contains("which deviations were recorded and which were not"),
        "the report must let the operator tell the appended records from the missing ones"
    );
}

/// The census publishes `qualifying_uses` and `outcome_counts` side by side in one object,
/// and since #27 they count different things: runs and records. A run that deviated twice
/// makes the outcome counts sum above the use count, which reads as a miscount to anyone who
/// has not been told. The terminal reply took this same debt on and paid it; a report a
/// commissioning decision rests on cannot carry it silently.
#[test]
fn installed_method_gap_package_separates_run_counts_from_record_counts() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/method-gap-research-status/SKILL.md"),
    )
    .expect("read installed method-gap package");

    assert!(
        package.contains("`qualifying_uses` counts runs"),
        "the census must say which of the two numbers counts runs"
    );
    assert!(
        package.contains("outcome counts"),
        "and which counts records"
    );
    assert!(
        package.contains("more outcomes than uses"),
        "the reader must be told the sum can exceed the use count without either being wrong"
    );
}

/// The command refuses a clean further incident and allows the reverse — a run first
/// recorded clean can still gain an incident. That asymmetry is deliberate, because refusing
/// it would leave a deviation noticed after the receipt with nowhere to go, but an operator
/// who is never told the path exists will not use it, and the evidence is simply lost.
#[test]
fn installed_capture_package_states_a_clean_run_can_still_gain_an_incident() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evidence-capture/SKILL.md"),
    )
    .expect("read installed capture package");

    assert!(
        package.contains("a run already recorded clean can still gain an incident"),
        "the operator must be told the deviation noticed after a clean receipt has a home"
    );
    assert!(
        package.contains("never the other way round"),
        "and that the reverse — a clean receipt for a run already recorded as deviating — is not available"
    );
}

/// The package sends cross-session continuations to `--same-run-group`, which a further
/// incident may not use. An operator reading only that would try the route, be refused, and
/// have no instruction covering where they actually are — the state that ends with one run
/// recorded as two uses.
#[test]
fn installed_capture_package_bounds_further_incidents_to_one_session() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evidence-capture/SKILL.md"),
    )
    .expect("read installed capture package");

    assert!(
        package.contains("only within the top-level session that recorded the run"),
        "the package must state the limit before the operator meets it as a refusal"
    );
    assert!(
        package.contains("would count that one run twice"),
        "and must say why recording it as a fresh use is not the way out"
    );
}

/// A caller-supplied group can carry a continuation only while that group is absent from the
/// current target hash. Once an earlier session has recorded it on the unchanged target, the
/// operator must stop rather than mint a second qualifying use for the same run.
#[test]
fn installed_capture_package_bounds_cross_session_continuations_on_the_current_hash() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evidence-capture/SKILL.md"),
    )
    .expect("read installed capture package");

    assert!(
        package.contains(
            "a caller-supplied run group cannot join a group the current target content hash already holds"
        ),
        "the package must state the machinery's current-hash limit"
    );
    assert!(
        package.contains("do not record the same run again as a new use"),
        "the package must give the safe outcome when an unchanged target already holds the group"
    );
}

#[test]
fn installed_skill_evolution_reference_writes_the_report_before_close_and_amends_it_after() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let write = reference
        .find("Before any close, write the review report")
        .expect("the report is written before close");
    let close = reference
        .find("skills evolution close")
        .expect("the compiled close command remains explicit");
    let amend = reference
        .find("After the close succeeds, amend the review report")
        .expect("the report is amended from the close receipt");
    assert!(write < close && close < amend, "reference={reference}");
    assert!(
        reference.contains(
            "--trials <count> --artifacts reports/skill-evidence/<skill-key>/reviews/<review-id>"
        ),
        "a no-candidate validation arm must carry its asserted effort into close"
    );
}

#[test]
fn installed_skill_evolution_reference_uses_operating_identity_for_precedent() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains("--record-operating-skill-hash"),
        "every real Skill Evolution claim must opt into the compiled identity receipt"
    );
    assert!(
        reference.contains("evidence to weigh rather than a ruling that governs"),
        "a predecessor decided under different operating-package bytes is not current precedent"
    );
    assert!(
        reference.contains("unknown rather than equal")
            && reference.contains("read exactly as it is read today"),
        "an absent historical identity must remain unknown without rewriting prior semantics"
    );
    assert!(
        reference.contains("operating_skill_hash"),
        "the rule must name the compiled receipt key it consumes"
    );
}

#[test]
fn installed_skill_evolution_reference_weighs_trigger_workarounds_before_freezing_the_plan() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let ownership = reference
        .find("### 3. Determine target ownership and causal mechanism")
        .expect("the ownership step remains explicit");
    let workaround = reference
        .find("read `workaround_taken` only from the raw trigger events in the evidence packet")
        .expect("the ownership step reads only the workaround evidence the packet carries");
    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    assert!(
        ownership < workaround && workaround < freeze,
        "the workaround finding belongs to ownership determination before plan freezing: reference={reference}"
    );

    for required in [
        "state that none was recorded on a trigger event",
        "Repeated suppression of the mechanism is evidence for target ownership",
        "a workaround that was taken without suppressing the mechanism is evidence against target ownership",
        "Record the direction as evidence, never as a verdict",
        "count the open incident IDs outside the trigger set and state that count, including zero",
        "Do not characterize, estimate, or reason about those incidents",
        "do not read the historical ledger or seek their payloads",
        "Reconcile the frozen plan with step 3's workaround finding",
        "without letting it replace a trial or skip or shrink the frozen trial set",
        "- Recorded-workaround finding:",
        "- Non-trigger open incident count:",
        "acceptance decision is made from the trial results alone",
    ] {
        assert!(
            reference.contains(required),
            "installed reference must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_requires_per_mechanism_trials() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let ownership = reference
        .find("### 3. Determine target ownership and causal mechanism")
        .expect("the ownership step remains explicit");
    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let validation = reference
        .find("### 6. Run blind comparative validation")
        .expect("the candidate-arm step remains explicit");
    let report = reference
        .find("# Skill Evolution Review: <skill-name>")
        .expect("the review report template remains explicit");

    for (required, section) in [
        (
            "Name one candidate mechanism for each trigger event, or explicitly group several trigger events under one shared mechanism and state why they share it.",
            &reference[ownership..freeze],
        ),
        (
            "Freeze one reproduction trial per distinct mechanism, each with its own trigger event IDs, witness, unexpressed reading, and incidence-sized run count.",
            &reference[freeze..current_arm],
        ),
        (
            "The current arm is the union of the reproduction trials; if their results disagree, proceed only on each mechanism that reproduced and report every mechanism as reproduced, not reproduced with witnesses expressed, or unable to be expressed.",
            &reference[current_arm..validation],
        ),
        (
            "Before any candidate output exists, freeze whether a candidate-arm run whose witness reads unexpressed is discounted from the comparison or replaced.",
            &reference[freeze..current_arm],
        ),
    ] {
        assert!(
            section.contains(required),
            "installed reference must preserve `{required}` in its governing step: reference={reference}"
        );
    }

    assert_eq!(
        reference[report..]
            .matches("- Trigger event → reproduction trial → witness reading:")
            .count(),
        2,
        "both evidence adjudication and results must carry the per-mechanism mapping: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reconciles_every_mechanism_clause_with_its_reproduction_oracle() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let validation = reference
        .find("### 6. Run blind comparative validation")
        .expect("the candidate-arm step remains explicit");
    let plan = &reference[freeze..current_arm];
    let reproduction = &reference[current_arm..validation];

    for required in [
        "Break each candidate mechanism into named observable clauses",
        "including its triggering condition and every behavior the mechanism says is wrong",
        "Freeze the recurrence rule that combines those clauses",
        "every mechanism clause maps to at least one frozen reading",
        "every reproduction criterion maps back to a mechanism clause or protected behavior",
        "Both unmatched lists must be empty before a runnable trial is frozen",
    ] {
        assert!(
            plan.contains(required),
            "the frozen plan must preserve `{required}`: reference={reference}"
        );
    }
    assert!(
        reproduction.contains(
            "Classify recurrence from the frozen recurrence rule, not from the broader trial verdict"
        ) && reproduction.contains(
            "A retained artifact that satisfies that rule reproduced the mechanism even when the trial otherwise passed"
        ),
        "a broad outcome rubric must not erase a mechanism failure present in the retained artifact: reference={reference}"
    );
    assert!(
        reference.contains("- Mechanism clause → observable reading:")
            && reference.contains("- Unmatched mechanism clauses / reproduction criteria:"),
        "the durable report must retain the reconciliation that made the reproduction oracle complete: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_reserves_blocked_for_the_whole_review() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "When only some mechanisms are untestable, mark each as unable to be expressed in the plan and proceed with the runnable reproduction trials.",
        "`blocked_no_valid_test` remains a whole-review disposition; do not assign it to an individual mechanism.",
    ] {
        assert!(
            plan.contains(required),
            "installed reference must preserve `{required}` in the frozen-plan contract: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_sources_each_binding_constraint_in_step_four() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "For each binding constraint, identify what in the evidence packet establishes it",
        "`run_condition`, `observed`, `consequence`, or `workaround_taken`",
        "a same-target predecessor's ruling",
        "record it as unestablished rather than asserting it",
        "An effect recorded as undetermined establishes nothing about that effect and does not establish its opposite",
    ] {
        assert!(
            plan.contains(required),
            "installed step 4 must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_tests_witnesses_against_a_clean_compliant_run() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");
    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "would a compliant run that finds nothing still emit it?",
        "If no, it is not a witness",
        "before any executor runs",
    ] {
        assert!(
            plan.contains(required),
            "installed step 4 must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_keeps_refuted_constraints_in_trial_per_trigger() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");
    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "placing the failure at first use refutes an accumulation, volume, or late-run constraint for that trigger",
        "That trigger keeps its trial slot regardless of how the mechanism grouped it",
        "re-examine the grouping before any unable-to-be-expressed marking",
    ] {
        assert!(
            plan.contains(required),
            "installed step 4 must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_keeps_unestablished_constraints_in_trial() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    assert!(
        plan.contains(
            "Only a constraint the evidence packet establishes can support an **unable to be expressed** marking"
        ),
        "an unestablished constraint must not retire a trial slot: reference={reference}"
    );
    assert!(
        plan.contains(
            "An unestablished constraint keeps its trial slot and proceeds to an ordinary reproduction trial"
        ),
        "the installed reference must name the safe fallback: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_routes_established_run_conditions_by_kind() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    assert!(
        plan.contains(
            "When recorded run conditions agree that failures arrived at volume or late in a long run, say so here and freeze a long-course reproduction trial sized to the scale they establish"
        ),
        "established run-condition routing must reach the long-course instrument rather than the exit: reference={reference}"
    );
    assert!(
        plan.contains(
            "When they agree that failures arrived only intermittently, say so here and treat a fresh single-run trial as unable to express that"
        ),
        "intermittency keeps the routing it had, on the single-run ground rather than the retired short-context one: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_freezes_a_long_course_trial_for_accumulation_constraints() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "A trial executor starts fresh, which is what makes the arms independent; it does not have to stay short.",
        "A constraint of accumulated context, volume, or run length is varied by a **long-course reproduction trial**: one raw task whose own work carries a fresh executor to the scale the packet establishes before it reaches the mechanism's failure boundary.",
        "What remains inexpressible is the residue one executor session cannot reach at all — accumulation across separate sessions, or elapsed wall-clock a run cannot produce — and only that residue makes a binding constraint itself unreachable.",
        "The witness rules below, the intermittency routing below, and step 5's first reading keep their own separate grounds for marking a mechanism unable to be expressed; this narrows the reachability ground alone.",
    ] {
        assert!(
            plan.contains(required),
            "installed step 4 must freeze a long-course trial for an accumulation constraint: missing `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_denies_cost_as_an_instrument_limit() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    assert!(
        plan.contains(
            "Trial cost is not that residue: a long-course trial is expensive, and expense is the maintainer's judgment about whether to spend the session, never a recorded claim that this instrument cannot test the evidence."
        ),
        "expense must not be recordable as an instrument limit: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_reaches_long_course_scale_by_working_not_instruction() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "A long-course trial reaches its scale by working, never by instruction.",
        "Telling an executor to accumulate context, to work for a stated number of steps, or to expect a late failure is behavioral scope under the logistics rule below, and it simulates the constraint instead of expressing it.",
    ] {
        assert!(
            plan.contains(required),
            "a long-course trial must express its constraint rather than announce it: missing `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_separates_frozen_scale_from_reached_reading() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    // The dead-end section is named in step 9's prose before the template repeats it, so each
    // anchor searches forward from the previous one rather than from the start of the file.
    let template_plan = reference
        .find("## Frozen validation plan")
        .expect("the report template keeps its frozen-plan section");
    let template_results = reference[template_plan..]
        .find("## Results")
        .map(|offset| template_plan + offset)
        .expect("the report template keeps its results section");
    let template_unable = reference[template_results..]
        .find("## Unable to be expressed")
        .map(|offset| template_results + offset)
        .expect("the report template keeps its dead-end section");
    let frozen = &reference[template_plan..template_results];
    let results = &reference[template_results..template_unable];

    assert!(
        frozen.contains("- Long-course scale → established source, or not applicable:")
            && !frozen.contains("- Long-course scale reached:"),
        "the frozen plan must carry the scale and its established source, and never the observed reading: reference={reference}"
    );
    assert!(
        results.contains("- Long-course scale reached: yes/no/not applicable")
            && !results.contains("- Long-course scale → established source"),
        "whether the runs reached the scale must be reported with the other results: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_frees_long_course_trials_from_short_context_predecessors() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    assert!(
        plan.contains(
            "A predecessor's short-context pass is not an equivalent trial to a long-course reproduction, and a same-target ruling that a mechanism was unable to be expressed under the short-context premise does not bind this review."
        ),
        "the no-equivalent-rerun rule must not foreclose the instrument that supersedes it: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_keeps_trial_prompts_behavior_neutral_in_step_four() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "an observable the raw task naturally produces",
        "The harness must not compel the executor to emit it",
        "may only make the held package and raw artifacts findable",
        "changes what a compliant run would do or output",
        "behavioral scope, output requirements, or search directives",
        "expose the witness or make the mechanism under test salient",
    ] {
        assert!(
            plan.contains(required),
            "installed step 4 must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_distinguishes_location_from_behavioral_scope() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    assert!(
        plan.contains("naming where one is located is logistics"),
        "installed step 4 must permit location-only logistics: reference={reference}"
    );
    assert!(
        plan.contains(
            "Declaring which repositories or artifacts form the complete task scope is behavioral"
        ),
        "installed step 4 must reject a completeness declaration that supplies behavioral scope: reference={reference}"
    );
    assert!(
        plan.contains("even when it also makes inputs findable"),
        "findability must not excuse answer-shaping scope: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_separates_raw_tasks_from_executor_logistics_in_step_six() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let validation = reference
        .find("### 6. Run blind comparative validation")
        .expect("the blind-validation step remains explicit");
    let acceptance = reference
        .find("### 7. Apply the acceptance gate")
        .expect("the acceptance step remains explicit");
    let executor_custody = &reference[validation..acceptance];

    for required in [
        "the original raw task and artifacts plus only the frozen executor logistics",
        "may locate an opaque held package or raw artifact",
        "must not change what a compliant run would do or output",
        "Retain the original raw task separately from executor logistics",
        "distinct artifacts or marked sections",
        "under `reviews/<review-id>/`",
    ] {
        assert!(
            executor_custody.contains(required),
            "installed step 6 must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_discloses_evaluable_failure_readings_in_step_six() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");
    let validation = reference
        .find("### 6. Run blind comparative validation")
        .expect("the blind-validation step remains explicit");
    let acceptance = reference
        .find("### 7. Apply the acceptance gate")
        .expect("the acceptance step remains explicit");
    let step_six = &reference[validation..acceptance];

    for required in [
        "the frozen failure reading remains evaluable on the returned artifact",
        "state what that failure reading read in the review report",
    ] {
        assert!(
            step_six.contains(required),
            "installed step 6 must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_freezes_artifact_identity_before_outcomes() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let freeze = reference
        .find("### 4. Freeze the validation plan before any candidate exists")
        .expect("the plan-freezing step remains explicit");
    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let plan = &reference[freeze..current_arm];

    for required in [
        "Freeze the **artifact identity relation** each such reading uses: exact bytes, or one named deterministic canonicalization, with the exact command that computes it and the difference it is allowed to absorb.",
        "Exact bytes is the default and needs no argument: a frozen reading that compares artifacts without naming a relation is governed by exact bytes.",
        "A comparison the plan did not freeze at all cannot carry an adverse claim at step 7, where it is recorded and decides nothing by itself. No relation may be chosen, widened, or narrowed once results exist.",
    ] {
        assert!(
            plan.contains(required),
            "step 4 must fix artifact sameness before any outcome exists: missing `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_compares_under_the_frozen_relation_in_step_six() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let validation = reference
        .find("### 6. Run blind comparative validation")
        .expect("the blind-validation step remains explicit");
    let acceptance = reference
        .find("### 7. Apply the acceptance gate")
        .expect("the acceptance step remains explicit");
    let step_six = &reference[validation..acceptance];

    assert!(
        step_six.contains(
            "Compare artifacts only under the artifact identity relation step 4 froze for that reading, and retain both sides of every comparison, so a claim made at step 7 rests on what is on disk rather than on what a run reported."
        ),
        "the run step must apply the frozen relation and retain what a step-7 claim will have to be established from: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_voids_a_symmetric_frozen_input_fault() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let acceptance = reference
        .find("### 7. Apply the acceptance gate")
        .expect("the acceptance step remains explicit");
    let record = reference
        .find("### 8. Record, land, verify")
        .expect("the landing step remains explicit");
    let gate = &reference[acceptance..record];

    for required in [
        "that is a **frozen-input fault** in the plan rather than a result about either arm",
        "Establish it by reading the retained artifact and quoting the refuting bytes, never by re-running and never by re-cutting the fixture",
        "The refuted criterion is void for this gate, because a criterion both arms fail on a premise the input itself refutes discriminates nothing between them.",
        "A symmetric reading with no such fault established is not void: on a protected-behavior or regression criterion it is noninferior, which the gate already accounts for, and on a reproduction case it means the candidate resolved nothing, which the gate's first term already refuses.",
        "Voiding takes the refuting bytes and never a shared result alone.",
        "Record the fault, the refuting bytes, and the voided criterion in the review report.",
        "A void criterion is a defect in the plan and never a limit of the instrument, so it supports no unable-to-be-expressed marking, no `--instrument-limited` naming, and no `blocked_no_valid_test`.",
    ] {
        assert!(
            gate.contains(required),
            "a symmetric input fault must not be spent against the candidate: missing `{required}`: reference={reference}"
        );
    }
    assert!(
        gate.contains(
            "Voiding reaches every criterion this gate reads by comparing the arms, the reproduction reading included: the bars stated absolutely above — safety, scope, and ownership invariants, and the deterministic checks the candidate must pass before landing — are not satisfied by the current skill failing them too."
        ),
        "voiding must reach the reproduction reading and must not reach the bars this gate states absolutely: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_attributes_every_material_regression_claim() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let acceptance = reference
        .find("### 7. Apply the acceptance gate")
        .expect("the acceptance step remains explicit");
    let record = reference
        .find("### 8. Record, land, verify")
        .expect("the landing step remains explicit");
    let gate = &reference[acceptance..record];

    assert!(
        gate.contains(
            "Whatever reading it comes from, calling an observation a **material candidate regression** requires all four of the following, each established from the retained artifacts."
        ),
        "the standard must bind every material-regression claim, so that a frozen criterion both arms fail is not exempt from arm-discrimination: reference={reference}"
    );
    assert!(
        gate.contains(
            "The bar stays open to an observation no frozen criterion covers, because a severe regression nobody anticipated is what it exists to catch; being frozen is not itself one of the four and does not stand in for them."
        ),
        "the open regression bar must survive the attribution standard rather than be closed by it: reference={reference}"
    );
    for required in [
        "**arm-discriminating** — present on the candidate arm and absent from the current arm under the same frozen input;",
        "**not variance** — not produced by the frozen input's own bytes, by the harness or executor logistics, or by a difference the reading's frozen artifact identity relation absorbs; where the comparison was never frozen at all, this part is unmet, because sameness settled after the fact is not evidence;",
        "**attributable** — the candidate's text produces it, named as the clause it adds, the clause it removes, or the exact difference from the current skill responsible where no single clause is;",
        "**baselined** — the current arm produced a comparable behavior for it to regress from.",
    ] {
        assert!(
            gate.contains(required),
            "a material candidate regression must be established on all four parts: missing `{required}`: reference={reference}"
        );
    }
    assert!(
        gate.contains(
            "A severe regression is established the same way: the four parts settle whether the candidate caused it, never how bad it is."
        ),
        "the gate bars material or severe regression, so calling one severe must not route around the four parts: reference={reference}"
    );
    assert!(
        gate.contains(
            "Where the candidate supplies a capability the current arm never had, the fourth fails by construction and the observation is not a regression."
        ) && gate.contains(
            "a defect read there can still reject the candidate"
        ),
        "a novel-capability defect must be regraded rather than dropped: reference={reference}"
    );
    assert!(
        gate.contains(
            "An observation failing any of the four is recorded in the review report with which part it failed, and cannot by itself reject the candidate."
        ),
        "an unattributed observation must be recorded and must not decide the gate on its own: reference={reference}"
    );
    assert!(
        gate.contains(
            "This narrows the open bar in one place, deliberately. An unanticipated regression that turns on two artifacts being the same cannot be established by the review that first notices it, because the relation deciding sameness would have to be chosen after the result it decides."
        ),
        "the reference an agent actually reads must disclose the one narrowing, rather than leaving it to the decision record: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_reports_gate_attribution_in_its_own_section() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    // `## Unable to be expressed` is named in step 9's prose before the template repeats it, so the
    // anchors chain forward from the previous one rather than each searching from the start.
    let template_plan = reference
        .find("## Frozen validation plan")
        .expect("the report template keeps its frozen-plan section");
    let template_results = reference[template_plan..]
        .find("## Results")
        .map(|offset| template_plan + offset)
        .expect("the report template keeps its results section");
    let template_unable = reference[template_results..]
        .find("## Unable to be expressed")
        .map(|offset| template_results + offset)
        .expect("the report template keeps its dead-end section");
    let frozen = &reference[template_plan..template_results];
    let results = &reference[template_results..template_unable];

    assert!(
        frozen.contains("- Artifact identity relation → comparisons it governs:")
            && !results.contains("- Artifact identity relation"),
        "the identity relation belongs to the frozen plan, because choosing one after results is what it forbids: reference={reference}"
    );
    for required in [
        "- Frozen-input fault → refuting bytes → voided criterion:",
        "- Material regression claim → attribution result:",
    ] {
        assert!(
            results.contains(required),
            "the results section must carry what the gate did with an unfrozen or refuted reading: missing `{required}`: reference={reference}"
        );
    }
    assert!(
        !frozen.contains("- Frozen-input fault") && !frozen.contains("- Material regression claim"),
        "a fault found only once results exist must never be written back into the frozen plan: reference={reference}"
    );
}

#[test]
fn installed_event_schema_declares_close_validation_effort_as_optional() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let schema: serde_json::Value = serde_json::from_slice(
        &fs::read(
            root.path()
                .join("schemas/skill-evidence/event.v1.schema.json"),
        )
        .expect("read installed event schema"),
    )
    .expect("installed event schema JSON");
    let payload = schema
        .pointer("/allOf/1/then/properties/payload")
        .expect("review_disposition payload schema");
    assert_eq!(payload["properties"]["trial_count"]["type"], "integer");
    assert_eq!(payload["properties"]["artifacts_path"]["type"], "string");
    let required = payload["required"]
        .as_array()
        .expect("required review_disposition properties");
    assert!(!required.iter().any(|field| field == "trial_count"));
    assert!(!required.iter().any(|field| field == "artifacts_path"));
}

#[test]
fn installed_status_package_does_not_tie_retirement_to_one_disposition() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution-status/SKILL.md"),
    )
    .expect("read installed skill-evolution-status package");

    assert!(
        package.contains(
            "`Retired as untestable` names stores whose open incidents left the gate because a review could not decide them"
        ),
        "an adjudicating close naming untestable coverage retires evidence too, so the relay may not attribute the section to one disposition"
    );
    assert!(
        package.contains(
            "An adjudicating close contributes the coverage it named as untestable, by name and never wider, less anything that still drives the gate on its own"
        ),
        "the relay explains how the standing retired set is composed, so it must carry the second contribution path too"
    );
}

#[test]
fn installed_status_package_treats_reporter_stdout_as_the_portable_payload() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution-status/SKILL.md"),
    )
    .expect("read installed skill-evolution-status package");
    let prose = package.split_whitespace().collect::<Vec<_>>().join(" ");

    for required in [
        "cargo run --locked --quiet -p demo-cli -- skills evolution-status",
        "The authoritative census payload is the compiled reporter's stdout",
        "Cargo diagnostics on stderr are not part of the census",
        "without omission, duplication, reordering, or paraphrase",
        "Host-required framing around that intact payload and normalization of its terminal newline are permitted",
        "Cargo may write only its ordinary build and cache artifacts",
    ] {
        assert!(
            prose.contains(required),
            "installed status package must preserve `{required}`: package={package}"
        );
    }

    assert!(
        !package.contains("Relay the compiled command output verbatim."),
        "byte equality with an agent's whole response is not a portable output contract"
    );
    assert!(
        !package.contains("_Done when the exact census"),
        "completion must be judged on the intact reporter payload rather than host framing"
    );
}

#[test]
fn installed_event_schema_keeps_untestable_coverage_optional_and_open() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let schema: serde_json::Value = serde_json::from_slice(
        &fs::read(
            root.path()
                .join("schemas/skill-evidence/event.v1.schema.json"),
        )
        .expect("read installed event schema"),
    )
    .expect("installed event schema JSON");
    let payload = schema
        .pointer("/allOf/1/then/properties/payload")
        .expect("review_disposition payload schema");

    assert_eq!(
        payload["properties"]["instrument_limited_event_ids"]["type"],
        "array"
    );
    assert!(
        !payload["required"]
            .as_array()
            .expect("required review_disposition properties")
            .iter()
            .any(|field| field == "instrument_limited_event_ids"),
        "absent means the review concluded about everything it covered, which is what every close written before this key existed asserts"
    );
    assert!(
        payload.get("additionalProperties").is_none(),
        "a consumer still on a stale installed schema must keep validating streams that carry this key"
    );
}

#[test]
fn installed_event_schema_keeps_constraint_provenance_optional_and_open() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let schema: serde_json::Value = serde_json::from_slice(
        &fs::read(
            root.path()
                .join("schemas/skill-evidence/event.v1.schema.json"),
        )
        .expect("read installed event schema"),
    )
    .expect("installed event schema JSON");
    let payload = schema
        .pointer("/allOf/1/then/properties/payload")
        .expect("review_disposition payload schema");
    let provenance = &payload["properties"]["constraint_provenance"];

    assert_eq!(provenance["type"], "array");
    assert_eq!(provenance["minItems"], 1);
    assert_eq!(
        provenance["items"]["required"],
        serde_json::json!(["constraint_label", "event_id", "field", "field_value"])
    );
    assert_eq!(
        provenance["items"]["properties"]["field"]["enum"],
        serde_json::json!(skill_evidence::ConstraintProvenanceField::roster())
    );
    for field in ["constraint_label", "event_id", "field_value"] {
        assert_eq!(provenance["items"]["properties"][field]["type"], "string");
        assert_eq!(provenance["items"]["properties"][field]["minLength"], 1);
    }
    assert!(
        provenance["description"]
            .as_str()
            .expect("provenance absent-meaning description")
            .contains("Omitted means no checked constraint provenance was recorded"),
        "the optional key must define what every historical absence means"
    );
    assert!(
        provenance["description"]
            .as_str()
            .expect("provenance absent-meaning description")
            .contains("does not negate an instrument-limited claim"),
        "historical blocked closes must keep their instrument-limited meaning"
    );
    assert!(
        !payload["required"]
            .as_array()
            .expect("required review_disposition properties")
            .iter()
            .any(|field| field == "constraint_provenance"),
        "historical closes omit constraint provenance and remain valid"
    );
    assert!(
        payload.get("additionalProperties").is_none(),
        "a stale installed schema must keep accepting the optional payload key"
    );
}

#[test]
fn installed_outside_target_owner_contract_matches_the_published_schema() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let schema: serde_json::Value = serde_json::from_slice(
        &fs::read(
            root.path()
                .join("schemas/skill-evidence/event.v1.schema.json"),
        )
        .expect("read installed event schema"),
    )
    .expect("installed event schema JSON");
    let payload = schema
        .pointer("/allOf/1/then/properties/payload")
        .expect("review_disposition payload schema");
    let owners = &payload["properties"]["external_owners"];
    assert_eq!(owners["type"], "array");
    assert_eq!(
        owners["items"]["required"],
        serde_json::json!(["event_id", "kind", "reference"])
    );
    let kinds = owners["items"]["properties"]["kind"]["enum"]
        .as_array()
        .expect("external owner kind enum")
        .iter()
        .map(|kind| kind.as_str().expect("external owner kind"))
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        vec![
            "skill",
            "contract",
            "tool",
            "environment",
            "model_limitation",
            "user_instruction"
        ]
    );
    assert_eq!(
        kinds,
        skill_evidence::ExternalOwnerKind::roster(),
        "compiled close and installed schema must share one owner-kind roster"
    );
    assert!(!kinds.contains(&"caller"));
    assert!(!kinds.contains(&"session"));
    assert!(
        !payload["required"]
            .as_array()
            .expect("required review_disposition properties")
            .iter()
            .any(|field| field == "external_owners"),
        "absent means no owner was recorded; historical outside_target events stay readable"
    );
    assert!(
        payload.get("additionalProperties").is_none(),
        "a stale installed schema must keep accepting the new optional payload key"
    );

    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");
    for kind in kinds {
        assert!(
            reference.contains(&format!("| `{kind}` |")),
            "installed ownership table must expose schema kind `{kind}`: {reference}"
        );
    }
}

#[test]
fn installed_event_schema_declares_operating_skill_hash_as_optional_and_open() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let schema: serde_json::Value = serde_json::from_slice(
        &fs::read(
            root.path()
                .join("schemas/skill-evidence/event.v1.schema.json"),
        )
        .expect("read installed event schema"),
    )
    .expect("installed event schema JSON");
    let identity_branch = schema["allOf"]
        .as_array()
        .expect("event conditionals")
        .iter()
        .find(|branch| {
            branch.pointer("/then/properties/payload/properties/operating_skill_hash/type")
                == Some(&serde_json::json!("string"))
        })
        .expect("operating identity payload schema");
    let event_type_schema = identity_branch
        .pointer("/if/properties/event_type")
        .expect("operating identity event types");
    let identity_event_types = event_type_schema["enum"].as_array().map_or_else(
        || {
            vec![
                event_type_schema["const"]
                    .as_str()
                    .expect("identity event type"),
            ]
        },
        |event_types| {
            event_types
                .iter()
                .map(|event_type| event_type.as_str().expect("identity event type"))
                .collect::<Vec<_>>()
        },
    );
    assert_eq!(
        identity_event_types,
        vec![
            "review_started",
            "validation_completed",
            "change_landed",
            "review_disposition"
        ],
        "every Skill Evolution lifecycle writer must declare the computed identity"
    );
    let payload = identity_branch
        .pointer("/then/properties/payload")
        .expect("operating identity payload");

    assert_eq!(
        payload["properties"]["operating_skill_hash"]["type"],
        "string"
    );
    assert_eq!(
        payload["properties"]["operating_skill_hash"]["minLength"],
        1
    );
    assert_eq!(
        payload["properties"]["operating_package_matches_shipped"]["type"],
        "boolean"
    );
    assert!(
        !payload["required"]
            .as_array()
            .expect("required identity-bearing properties")
            .iter()
            .any(|field| field == "operating_skill_hash"),
        "absent means the event predates unconditional identity recording"
    );
    assert!(
        !payload["required"]
            .as_array()
            .expect("required identity-bearing properties")
            .iter()
            .any(|field| field == "operating_package_matches_shipped"),
        "absent means the event predates operating-package comparison, not mismatch"
    );
    assert!(
        payload.get("additionalProperties").is_none(),
        "a consumer with the previous installed schema must keep validating the additive key"
    );

    let validator = jsonschema::validator_for(&schema).expect("compile installed event schema");
    for (event_type, event_payload) in [
        (
            "review_started",
            serde_json::json!({ "review_id": "review-fixture" }),
        ),
        (
            "validation_completed",
            serde_json::json!({ "review_id": "review-fixture" }),
        ),
        (
            "change_landed",
            serde_json::json!({ "review_id": "review-fixture" }),
        ),
        (
            "review_disposition",
            serde_json::json!({
                "review_id": "review-fixture",
                "disposition": "monitor_for_recurrence",
                "adjudicated_event_ids": ["evt-trigger"],
                "note": "fixture close"
            }),
        ),
    ] {
        let mut event = serde_json::json!({
            "schema_version": 1,
            "event_id": format!("evt_{event_type}"),
            "event_type": event_type,
            "recorded_at": "2026-01-02T03:04:05Z",
            "operator_workflow": "skill-evolution",
            "target": {
                "name": "demo-skill",
                "repo_relative_path": ".claude/skills/demo-skill",
                "content_hash": "target-hash",
                "repo_head": "fixture-head"
            },
            "top_level_session_id": "review-session",
            "payload": event_payload
        });
        assert!(
            validator.is_valid(&event),
            "the historical {event_type} shape without identity stays valid"
        );
        event["payload"]["operating_skill_hash"] = serde_json::json!("operating-hash");
        assert!(
            validator.is_valid(&event),
            "the additive identity is valid on {event_type}"
        );
        event["payload"]["operating_package_matches_shipped"] = serde_json::json!(true);
        assert!(
            validator.is_valid(&event),
            "a computed package match is valid on {event_type}"
        );
        event["payload"]["operating_package_matches_shipped"] = serde_json::json!(false);
        assert!(
            validator.is_valid(&event),
            "a computed package mismatch is valid on {event_type}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_reports_the_close_retirement_reach() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains("Read `retired_from_gate_event_ids` from the close receipt"),
        "the reviewer must read the per-close retirement reach from the irreversible action's receipt"
    );
    assert!(
        reference.contains("state that retirement reach in the user-facing completion"),
        "the receipt needs an operator-visible reader"
    );
    assert!(
        reference.contains("- Retirement reach event IDs:"),
        "the durable review report needs a home for the close's retirement reach"
    );
    assert!(
        reference
            .contains("reaches exactly the events an event-level `--instrument-limited` naming"),
        "an event-level naming retires only the events it names, which the roster below does not cover"
    );
    assert!(
        reference.contains("never the adjudication retirement"),
        "gate retirement and adjudication retirement must not read as one reach"
    );
}

#[test]
fn installed_skill_evolution_reference_reports_external_owners_to_the_maintainer() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        !reference.contains("Route outside-target evidence to its owner factually"),
        "the installed workflow must not instruct a routing act it cannot perform"
    );
    assert!(
        reference.contains(
            "Naming each outside-target owner in the close reports that attribution in the close receipt and user-facing completion for the maintainer to act on"
        ),
        "step 3 must state what the recorded owner actually accomplishes"
    );
    assert!(
        reference.contains("without proposing an unsanctioned repair")
            && reference.contains("never edit another owner from this review"),
        "the replacement must preserve both outside-target prohibitions"
    );
    assert!(
        reference.contains("Read `external_owners` from the close receipt whenever it is present"),
        "the irreversible close receipt must be the completion's owner source"
    );
    assert!(
        reference.contains("states each attributed owner kind and reference"),
        "the user-facing completion must report every attributed owner"
    );
    assert!(
        reference.contains("no owner clause and no empty-owner placeholder"),
        "an ownerless close must not render a placeholder"
    );
}

#[test]
fn installed_skill_evolution_reference_records_the_unexpressible_mechanism_dead_end() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "When any mechanism was marked **unable to be expressed**, record a dead-end note for each one"
        ),
        "the terminal reporting step must make the dead-end record conditional on the review reaching that reading"
    );
    assert!(
        reference.contains(
            "the mechanism, its binding constraint, and that this workflow has no further instrument for it"
        ),
        "the next reader needs the mechanism, the constraint that defeated the instrument, and the workflow limit"
    );
    assert!(
        reference
            .contains("The decision to pursue it belongs to the maintainer, not a later review"),
        "the note must report the dead end without scheduling another workflow"
    );
}

#[test]
fn installed_skill_evolution_reference_carries_forward_a_repeated_untestable_mechanism() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "Use the same-target `prior_reviews` reports read before step 4 to check whether an earlier review ruled a mechanism of the same shape unable to be expressed on these target bytes"
        ),
        "the evidence packet and prior report must supply the repeated same-bytes ruling"
    );
    assert!(
        reference.contains("reaching this exit twice on one target is the signal the note carries"),
        "the report must make the repeated dead end legible to the maintainer and next reviewer"
    );
}

#[test]
fn installed_skill_evolution_reference_provides_a_fixed_report_home_for_dead_ends() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    for required in [
        "## Unable to be expressed",
        "- Mechanism:",
        "- Binding constraint:",
        "- Earlier same-shape ruling on these target bytes:",
        "- Further instrument in this workflow: none",
    ] {
        assert!(
            reference.contains(required),
            "the installed report template must give `{required}` a fixed home: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_completes_with_the_unexpressible_mechanism_clause() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "When any mechanism was marked unable to be expressed, add one clause stating that fact alongside the retirement reach"
        ),
        "the user-facing completion must disclose the dead end beside the reach it already reports"
    );
}

#[test]
fn installed_skill_evolution_reference_leaves_reviews_without_unexpressible_mechanisms_unchanged() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "When no mechanism has that reading, omit the `## Unable to be expressed` section and add no completion clause; the report and completion otherwise stay unchanged"
        ),
        "a review that can express every mechanism must retain its existing report and completion"
    );
}

#[test]
fn installed_skill_evolution_reference_routes_untestable_coverage_out_of_adjudication() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains("--instrument-limited <event-id>"),
        "the close surface must expose the per-trigger channel the operator has to reach for"
    );
    assert!(
        reference.contains("stays open in the ledger and stops clustering"),
        "the operator must know what naming a trigger untestable claims about it"
    );
    assert!(
        reference
            .contains("so an event this review could not decide can never again reach a threshold"),
        "ground 2 names mechanisms step 7 did grade, so the paragraph must not claim the instrument failed to test them"
    );
    assert!(
        reference.contains("- Coverage named untestable:"),
        "the durable review report needs a home for the triggers the instrument could not test"
    );
    assert!(
        reference
            .contains("retire from the active set, except any the close names as untestable below"),
        "the sentence introducing adjudicating dispositions must not still promise that all covered ids retire"
    );
    assert!(
        reference.contains(
            "Naming a contemporaneous severe incident stops it being adjudicated without retiring it"
        ),
        "the naming paragraph promises retirement unconditionally; the carve-out sentences elsewhere are scoped to the blocked disposition and do not reach it"
    );
}

#[test]
fn installed_skill_evolution_reference_makes_every_close_route_explicit_and_inspectable() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    for required in [
        "For every event in the coverage list, add exactly one `--concluded <event-id>` or `--instrument-limited <event-id>`",
        "Missing, duplicate, conflicting, unknown, and out-of-coverage routes refuse before any write",
        "For every concluded `outside_target` event, add `--external-owner <event-id> <kind> <stable-reference>`",
        "- Trigger event → ownership class → owning source → discriminating evidence:",
        "- Trigger event → binding constraint → terminal route:",
        "- Undecidable ground: reproduction instrument/acceptance gate/not applicable",
    ] {
        assert!(
            reference.contains(required),
            "installed close workflow must preserve `{required}`: {reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_supplies_checked_whole_field_provenance() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");
    let report = reference
        .find("### 9. Report, close, amend, complete")
        .expect("the close step remains explicit");
    let close = &reference[report..];

    for required in [
        "`--constraint-provenance <constraint-label> <event-id> <field>`",
        "`run_condition`, `observed`, `consequence`, or `workaround_taken`",
        "every event named `--instrument-limited`",
        "every event in the coverage list for `blocked_no_valid_test`",
        "copies the complete field verbatim into the disposition event and close receipt",
        "refuses before writing when a pointer is missing, outside coverage, or names an absent, null, or empty field",
        "- Constraint provenance copied:",
    ] {
        assert!(
            close.contains(required),
            "installed close step must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_preserves_the_complete_close_receipt_in_completion() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");
    let report = reference
        .find("### 9. Report, close, amend, complete")
        .expect("the close step remains explicit");
    let close = &reference[report..];

    for required in [
        "preserve the compiled close command's complete stdout payload exactly once",
        "every line and emitted command in order without paraphrase",
        "Cargo stderr is not part of that reporter payload",
    ] {
        assert!(
            close.contains(required),
            "installed completion must preserve `{required}`: reference={reference}"
        );
    }
}

#[test]
fn installed_skill_evolution_reference_bars_a_verdict_conformance_only_evidence_cannot_bear() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains("conformance-only") && reference.contains("outcome-graded"),
        "the reviewer must classify what claim each trigger's evidence actually bears"
    );
    assert!(
        reference.contains("Read `consequence` from each raw trigger event"),
        "the classification reads a recorded field rather than the reviewer's impression"
    );
    assert!(
        reference.contains(
            "A trigger step 3 classified conformance-only, whose mechanism step 7 graded, is adjudicated only when those trials demonstrated an outcome deficit for it"
        ),
        "a gate graded on outcome may not reach a verdict on evidence bearing no outcome claim"
    );
    assert!(
        reference.contains("- Trigger event → evidence class:"),
        "the close requires the class to be on record beforehand, so the durable report needs a home for it"
    );
    assert!(
        reference.contains("A trigger this gate never graded is untouched by that bar"),
        "step 7 states the bar before step 9 qualifies it, so an ungraded trigger must not read as routed away from its verdict"
    );
}

#[test]
fn installed_skill_evolution_reference_names_which_instrument_limit_a_conformance_only_trigger_hits()
 {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains("Naming has two grounds, and they are different limits"),
        "the two grounds carry different limits, so one unqualified sentence cannot describe both"
    );
    assert!(
        reference.contains(
            "A candidate target or target-compliance defect whose ownership the packet cannot decide proceeds to step 4"
        ),
        "an unresolved ownership candidate must reach the existing reproduction-instrument ground rather than becoming an outside_target conclusion by omission"
    );
    assert!(
        reference.contains(
            "Expect it to re-authorize every session until a later review adjudicates it"
        ),
        "naming a contemporaneous severe trigger keeps the gate open indefinitely, and the operator must be told before choosing it"
    );
    assert!(
        reference.contains(
            "Choose `blocked_no_valid_test` only when the review has concluded about no covered trigger and no trial could express any mechanism"
        ),
        "an adjudicating sibling conclusion must keep the review on its honest mixed close even when another trigger is instrument-limited"
    );
    assert!(
        reference.contains("Both grounds must already be on record before the close"),
        "naming has two grounds, so a pre-recording rule written for only the first would forbid the second"
    );
}

#[test]
fn installed_skill_evolution_reference_keeps_its_no_candidate_route_consistent_with_close() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "Otherwise keep the disposition for a conclusion already reached, or carry `monitor_for_recurrence`, and route every unable-to-be-expressed mechanism's triggers undecidable at step 9"
        ),
        "a review with one concluded trigger and one unexpressible mechanism must retain its conclusion instead of falling onto the whole-cluster disposition"
    );
}

#[test]
fn installed_skill_evolution_names_a_wholly_not_reproduced_review() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let current_arm = reference
        .find("### 5. Construct an isolated candidate")
        .expect("the current-arm step remains explicit");
    let validation = reference
        .find("### 6. Run blind comparative validation")
        .expect("the candidate-arm step remains explicit");
    let arm = &reference[current_arm..validation];

    assert!(
        arm.contains(
            "When the review carries `monitor_for_recurrence` and step 3 reached no conclusion of its own that names the whole review — `outside_target` or `resolved_no_change` — name its terminal outcome from the readings:"
        ),
        "the shared precondition must be stated once, and must not be blocked by a per-trigger conclusion that names no review: reference={reference}"
    );
    assert!(
        arm.contains(
            "`not_reproduced_witnesses_expressed` when none was unable to be expressed and at least one was not reproduced with witnesses expressed"
        ),
        "step 5 must name the wholly not-reproduced outcome, and guard it existentially so a zero-mechanism review cannot satisfy it vacuously: reference={reference}"
    );
    assert!(
        arm.contains(
            "A review that did reach such a conclusion names its outcome instead, whatever its siblings' readings"
        ),
        "the guard must reach a tested sibling's reading, not only an untested one: reference={reference}"
    );
    assert!(
        arm.contains(
            "a review that also tested a mechanism takes its outcome from the branch above"
        ),
        "a per-trigger not_reproducible sibling must leave the review nameable rather than stranding it: reference={reference}"
    );
    assert!(
        reference.contains(
            "one of the two no-candidate outcomes step 5 branches between — `mixed_no_candidate` and `not_reproduced_witnesses_expressed` —"
        ),
        "the roster must defer to step 5's branch rather than restate its conditions and drop its precondition: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reports_a_mixed_no_candidate_close_truthfully() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let package = fs::read_to_string(root.path().join(".claude/skills/skill-evolution/SKILL.md"))
        .expect("read installed Skill Evolution package");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "`mixed_no_candidate` when at least one mechanism was unable to be expressed"
        ),
        "the no-candidate branch must name the mixed result instead of borrowing the narrower ownership conclusion: reference={reference}"
    );
    assert!(
        reference.contains("Do not label the whole review `not_reproducible`"),
        "the mixed route must explicitly reject the previous false terminal label: reference={reference}"
    );
    assert!(
        !reference.contains(
            "`mixed_no_candidate` when `monitor_for_recurrence` closes a review in which no mechanism reproduced"
        ),
        "the roster must not restate step 5's branch conditions, which is how the two drifted apart: reference={reference}"
    );
    assert!(
        reference.contains("- Terminal outcome:"),
        "the durable report must keep the human-facing terminal label distinct from the close disposition: reference={reference}"
    );
    assert!(
        !package.contains(
            "exactly one terminal outcome was reached and recorded through the compiled command"
        ),
        "the compiled event records a close disposition, not the report's human-facing terminal label: package={package}"
    );
    assert!(
        package.contains("exactly one close disposition was recorded through the compiled command")
            && package.contains("the review report names its truthful terminal outcome"),
        "the installed entrypoint must distinguish the persisted disposition from the reported terminal label: package={package}"
    );
}

#[test]
fn installed_skill_evolution_reference_requires_pre_close_reach_bound_review() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "Before a `blocked_no_valid_test` close, read the authorization reason and coverage list from the claim receipt"
        ),
        "the immutable claim receipt must supply both inputs to the reason-specific reach bound"
    );
    assert!(
        reference.contains(
            "Re-evaluate that authorization reason against the live candidate clusters in the gate projection"
        ),
        "the live projection must supply the reason-specific reach bound"
    );
    assert!(
        reference.contains(
            "`material_recurrence` names only material-or-worse incidents in its symptom cluster"
        ),
        "the vouch must not silently widen a material authorization to friction siblings"
    );
    assert!(
        reference.contains(
            "The projection is current as of the last recorded incident, so no extra derive run is required"
        ),
        "the reviewer must not invent an unnecessary pre-close command"
    );
    assert!(
        reference.contains(
            "This reason-specific reach bound can name incidents the close will not retire: a contemporaneous severe incident is never retired because it authorizes on its own"
        ),
        "the safely over-stated bound must not be presented as an exact preview"
    );
}

#[test]
fn installed_skill_evolution_reference_freezes_coverage_at_claim() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "The coverage list freezes when the review is claimed, so it includes every incident the authorization reason names at that point"
        ),
        "the installed operator must use the claim as the coverage freeze point"
    );
    assert!(
        reference.contains(
            "Incidents recorded after the claim and before the close remain outside coverage and inside the reason-specific reach"
        ),
        "the installed operator must disclose the residual claim-to-close span"
    );
    assert!(
        !reference.contains(
            "including incidents recorded before the threshold fired; incidents recorded after the close"
        ),
        "the installed operator must not partition coverage at first eligibility"
    );
}

#[test]
fn installed_skill_evolution_reference_ties_mismatch_disclosure_to_unvouched_sibling() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    assert!(
        reference.contains(
            "Vouch only for incidents inside that reason-specific reach bound; same-symptom incidents outside it do not affect this close"
        ),
        "the reviewer must not import symptom-wide vouching into the narrowed rule"
    );
}

#[test]
fn installed_skill_evolution_reference_vouches_for_constraint_provenance_before_close() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let report_and_close = reference
        .find("### 9. Report, close, amend, complete")
        .expect("the close step remains explicit");
    let report = reference
        .find("Before any close, write the review report")
        .expect("the pre-close report instruction remains explicit");
    let pre_close = &reference[report_and_close..report];

    assert!(
        pre_close.contains(
            "Vouch for every named binding constraint's provenance as well as its coverage"
        ),
        "the pre-close vouch must cover provenance: reference={reference}"
    );
    assert!(
        pre_close.contains("the evidence-packet artifact establishing it is on record"),
        "the vouch must name its observable: reference={reference}"
    );
    assert!(
        pre_close.contains(
            "If a binding constraint is unestablished, record that mismatch in the review report and user-facing completion"
        ),
        "an unestablished constraint must use the existing mismatch disclosure: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_reference_vouches_for_every_constraint_before_any_close() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let reference = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution/references/authorized-review.md"),
    )
    .expect("read installed authorized-review reference");

    let report_and_close = reference
        .find("### 9. Report, close, amend, complete")
        .expect("the close step remains explicit");
    let report = reference
        .find("Before any close, write the review report")
        .expect("the pre-close report instruction remains explicit");
    let pre_close = &reference[report_and_close..report];

    assert!(
        pre_close.contains("Before any close, perform the provenance vouch"),
        "the provenance vouch must not depend on the eventual close route: reference={reference}"
    );
    assert!(
        pre_close.contains(
            "Vouch for every named binding constraint's provenance as well as its coverage"
        ),
        "the unconditional vouch must still cover every named constraint: reference={reference}"
    );
}

#[test]
fn installed_skill_evolution_status_describes_reason_scoped_retirement() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let status_skill = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution-status/SKILL.md"),
    )
    .expect("read installed skill-evolution-status package");

    assert!(
        status_skill.contains(
            "Each instrument-limited close contributes only the open incidents its recorded authorization reason named at the close"
        ),
        "the standing set must be traced to reason-scoped per-close reaches"
    );
    assert!(
        status_skill
            .contains("`material_recurrence` leaves friction siblings outside the retired set"),
        "status readers must not infer the former symptom-wide material reach"
    );
    assert!(
        status_skill.contains("`severe` contributes no retired incidents"),
        "an empty severe reach must be legible at the standing-set surface"
    );
    assert!(
        status_skill.contains(
            "a missing or unrecognized authorizing rule uses the prior symptom-wide reach"
        ),
        "historical fallback must be visible to status readers"
    );
}

#[test]
fn installed_skill_evolution_status_describes_deferred_review_bases_without_claiming_coverage() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("install current assets");
    let status_skill = fs::read_to_string(
        root.path()
            .join(".claude/skills/skill-evolution-status/SKILL.md"),
    )
    .expect("read installed skill-evolution-status package");

    assert!(
        status_skill.contains(
            "distinguishing evidence queued behind an instrument-limited close that reached no conclusion and could test nothing from evidence behind a concluded close where no threshold-supporting incident was recorded afterward; neither basis claims that the close covered the deferred evidence"
        ),
        "the queued bases must describe disposition and post-close evidence without claiming coverage"
    );
    assert!(
        !status_skill.contains("evidence a review accounted for"),
        "the deferred section must not describe either queued basis as accounted-for evidence"
    );
}

#[test]
fn install_reports_a_retired_package_without_writing_or_removing_it() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let retired_skill = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/SKILL.md");
    fs::create_dir_all(retired_skill.parent().expect("retired package directory"))
        .expect("create retired package directory");
    fs::write(&retired_skill, "consumer-owned sentinel\n").expect("write sentinel");

    let receipt = assets::install(root.path(), &host(), false).expect("install current assets");

    assert_eq!(
        receipt.orphaned_packages,
        vec!["legacy-skill-decontamination".to_owned()]
    );
    assert_eq!(
        fs::read_to_string(&retired_skill).expect("read sentinel after install"),
        "consumer-owned sentinel\n",
        "install must neither write nor remove a retired template"
    );

    let forced = assets::install(root.path(), &host(), true).expect("force install current assets");
    assert_eq!(
        forced.orphaned_packages,
        vec!["legacy-skill-decontamination".to_owned()]
    );
    assert_eq!(
        fs::read_to_string(&retired_skill).expect("read sentinel after forced install"),
        "consumer-owned sentinel\n",
        "install --force must not authorize removal"
    );
}

#[cfg(unix)]
#[test]
fn install_reports_a_retired_package_when_only_its_discovery_link_remains() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let links = root.path().join(".agents/skills");
    fs::create_dir_all(&links).expect("create discovery directory");
    let link = links.join(RETIRED_PACKAGE);
    std::os::unix::fs::symlink(format!("../../.claude/skills/{RETIRED_PACKAGE}"), &link)
        .expect("create retired discovery link");

    let receipt = assets::install(root.path(), &host(), false).expect("install current assets");

    assert_eq!(receipt.orphaned_packages, vec![RETIRED_PACKAGE.to_owned()]);
    assert_eq!(
        fs::read_link(&link).expect("install retained retired link"),
        PathBuf::from(format!("../../.claude/skills/{RETIRED_PACKAGE}"))
    );
}

#[test]
fn withdraw_removes_a_pristine_retired_package_and_reports_every_effect() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_pristine_retired_package(root.path());
    let empty_subdirectory = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/scripts/nested");
    fs::create_dir_all(&empty_subdirectory).expect("create empty retired-package subdirectory");

    let receipt = assets::withdraw(root.path(), &host(), false).expect("withdraw retired package");

    assert_eq!(
        receipt.removed_files,
        vec![
            ".claude/skills/legacy-skill-decontamination/SKILL.md".to_owned(),
            ".claude/skills/legacy-skill-decontamination/agents/openai.yaml".to_owned(),
            ".claude/skills/legacy-skill-decontamination/references/eligible-run.md".to_owned(),
        ]
    );
    assert!(receipt.forced_files.is_empty());
    assert!(receipt.retained.is_empty());
    assert!(
        receipt
            .removed_directories
            .contains(&".claude/skills/legacy-skill-decontamination".to_owned())
    );
    assert!(
        receipt
            .removed_directories
            .contains(&".claude/skills/legacy-skill-decontamination/scripts/nested".to_owned())
    );
    assert!(
        !root
            .path()
            .join(".claude/skills")
            .join(RETIRED_PACKAGE)
            .exists()
    );
    assert!(root.path().join(".claude/skills").is_dir());

    #[cfg(unix)]
    {
        assert_eq!(
            receipt.removed_links,
            vec![".agents/skills/legacy-skill-decontamination".to_owned()]
        );
        assert!(root.path().join(".agents/skills").is_dir());
    }
}

#[test]
fn withdraw_cli_refuses_an_edited_retired_file_without_changing_the_tree() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let edited = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/SKILL.md");
    fs::write(&edited, "consumer edit\n").expect("edit retired file");
    let evidence = root.path().join("reports/skill-evidence/demo/events.jsonl");
    fs::create_dir_all(evidence.parent().expect("evidence directory"))
        .expect("create evidence directory");
    fs::write(&evidence, b"immutable evidence sentinel\n").expect("write evidence sentinel");
    let before = snapshot_tree(root.path());

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("run compiled withdraw command");

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 refusal");
    assert!(stderr.contains(".claude/skills/legacy-skill-decontamination/SKILL.md"));
    assert!(stderr.contains("--force"));
    assert_eq!(snapshot_tree(root.path()), before);
}

#[cfg(unix)]
#[test]
fn withdrawal_unsafe_failure_reports_every_effect_already_applied() {
    use std::os::unix::fs::PermissionsExt;

    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let package = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination");
    let blocked_directory = package.join("agents");
    fs::set_permissions(&blocked_directory, fs::Permissions::from_mode(0o500))
        .expect("make the later removal fail");

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("run compiled withdrawal with a later I/O failure");

    fs::set_permissions(&blocked_directory, fs::Permissions::from_mode(0o700))
        .expect("restore directory permissions for cleanup");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(
        !package.join("SKILL.md").exists(),
        "the first removal happened"
    );
    assert!(
        package.join("agents/openai.yaml").is_file(),
        "the failing removal did not happen"
    );
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 unsafe failure");
    let (_, receipt) = stderr
        .trim_end()
        .split_once(" Partial withdrawal receipt: ")
        .expect("unsafe failure carries an inspectable partial receipt");
    let receipt: serde_json::Value =
        serde_json::from_str(receipt).expect("partial withdrawal receipt JSON");
    assert_eq!(
        receipt["removed_files"],
        serde_json::json!([".claude/skills/legacy-skill-decontamination/SKILL.md"])
    );
    assert_eq!(receipt["removed_links"], serde_json::json!([]));
    assert_eq!(receipt["removed_directories"], serde_json::json!([]));
}

#[test]
fn install_refusal_leaves_the_whole_tree_byte_identical() {
    let root = tempfile::tempdir().expect("temporary repository root");
    assets::install(root.path(), &host(), false).expect("initial install");
    let edited = root.path().join(".claude/skills/skill-evolution/SKILL.md");
    fs::write(&edited, "consumer edit\n").expect("edit installed file");
    fs::create_dir_all(root.path().join("unrelated/empty")).expect("create unrelated directory");
    let before = snapshot_tree(root.path());

    let error = assets::install(root.path(), &host(), false).expect_err("refuse edited file");

    assert_eq!(error.class(), skill_evidence::ErrorClass::Refusal);
    assert_eq!(snapshot_tree(root.path()), before);
}

#[test]
fn install_help_explains_atomic_exit_3_refusal_without_force() {
    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "install", "--help"])
        .output()
        .expect("run compiled install help");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let help = String::from_utf8(output.stdout).expect("UTF-8 install help");
    assert!(
        help.contains("Without --force, differing installed files cause an atomic refusal"),
        "install help must identify the no-force refusal: {help}"
    );
    assert!(
        help.contains("nothing is written") && help.contains("exits 3"),
        "install help must state the refusal's write and exit semantics: {help}"
    );
}

#[test]
fn withdraw_force_removes_an_edited_retired_file_and_names_it() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let edited_path = ".claude/skills/legacy-skill-decontamination/SKILL.md";
    fs::write(root.path().join(edited_path), "consumer edit\n").expect("edit retired file");

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .arg("--force")
        .output()
        .expect("run compiled forced withdrawal");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let receipt: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("withdrawal receipt JSON");
    assert_eq!(receipt["forced_files"], serde_json::json!([edited_path]));
    assert!(
        receipt["removed_files"]
            .as_array()
            .expect("removed files")
            .contains(&serde_json::json!(edited_path))
    );
    assert!(!root.path().join(edited_path).exists());
}

fn assert_foreign_file_is_retained(force: bool) {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_pristine_retired_package(root.path());
    let foreign_path = ".claude/skills/legacy-skill-decontamination/consumer-owned/notes.txt";
    let foreign = root.path().join(foreign_path);
    fs::create_dir_all(foreign.parent().expect("foreign file parent"))
        .expect("create foreign file parent");
    fs::write(&foreign, "do not remove\n").expect("write foreign file");

    let receipt = assets::withdraw(root.path(), &host(), force).expect("withdraw shipped files");

    assert_eq!(
        fs::read_to_string(&foreign).expect("read foreign file"),
        "do not remove\n"
    );
    assert!(
        root.path()
            .join(".claude/skills/legacy-skill-decontamination")
            .is_dir()
    );
    assert_eq!(
        receipt.retained,
        vec![assets::RetainedPath {
            path: foreign_path.to_owned(),
            reason: "path was not shipped by this crate".to_owned(),
        }]
    );
}

#[test]
fn withdraw_retains_a_foreign_file_without_force() {
    assert_foreign_file_is_retained(false);
}

#[test]
fn withdraw_force_does_not_remove_a_foreign_file() {
    assert_foreign_file_is_retained(true);
}

#[cfg(unix)]
#[test]
fn withdraw_retains_a_discovery_link_that_points_elsewhere() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_pristine_retired_package(root.path());
    let link = root
        .path()
        .join(".agents/skills/legacy-skill-decontamination");
    fs::remove_file(&link).expect("remove expected link");
    std::os::unix::fs::symlink("../../somewhere-else", &link).expect("create foreign link");

    let receipt = assets::withdraw(root.path(), &host(), false).expect("withdraw package files");

    assert_eq!(
        fs::read_link(&link).expect("retained alternate link"),
        PathBuf::from("../../somewhere-else")
    );
    assert_eq!(
        receipt.retained,
        vec![assets::RetainedPath {
            path: ".agents/skills/legacy-skill-decontamination".to_owned(),
            reason: "discovery link points to ../../somewhere-else instead of ../../.claude/skills/legacy-skill-decontamination".to_owned(),
        }]
    );
}

#[cfg(unix)]
#[test]
fn withdraw_does_not_follow_a_retired_package_symlink() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let outside = root.path().join("consumer-owned-package");
    for (relative_path, template) in RETIRED_FILES {
        let destination = outside.join(relative_path);
        fs::create_dir_all(destination.parent().expect("outside file parent"))
            .expect("create outside file parent");
        fs::write(
            destination,
            rendered_for(template, "demo", "demo-cli", "demo"),
        )
        .expect("write outside file");
    }
    let package_parent = root.path().join(".claude/skills");
    fs::create_dir_all(&package_parent).expect("create package parent");
    let package_link = package_parent.join(RETIRED_PACKAGE);
    std::os::unix::fs::symlink("../../consumer-owned-package", &package_link)
        .expect("create package symlink");
    let before = snapshot_tree(&outside);

    let receipt = assets::withdraw(root.path(), &host(), false).expect("retain package symlink");

    assert_eq!(snapshot_tree(&outside), before);
    assert!(receipt.removed_files.is_empty());
    assert_eq!(
        receipt.retained,
        vec![assets::RetainedPath {
            path: ".claude/skills/legacy-skill-decontamination".to_owned(),
            reason: "retired package path is not a directory created by this crate".to_owned(),
        }]
    );
}

#[test]
fn withdraw_is_idempotent() {
    let root = tempfile::tempdir().expect("temporary repository root");
    write_reference_host_retired_package(root.path());
    let first = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("first compiled withdrawal");
    assert_eq!(first.status.code(), Some(0));

    let second = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("second compiled withdrawal");

    assert_eq!(second.status.code(), Some(0));
    assert!(second.stderr.is_empty());
    let second: serde_json::Value =
        serde_json::from_slice(&second.stdout).expect("second withdrawal receipt JSON");
    assert_eq!(
        second,
        serde_json::json!({
            "schema_version": 1,
            "removed_files": [],
            "forced_files": [],
            "removed_directories": [],
            "removed_links": [],
            "retained": [],
        })
    );
}

#[test]
fn withdraw_is_a_no_op_when_no_retired_package_was_ever_installed() {
    let root = tempfile::tempdir().expect("temporary repository root");

    let output = Command::new(env!("CARGO_BIN_EXE_skill-evidence"))
        .args(["skills", "evidence", "withdraw", "--root"])
        .arg(root.path())
        .output()
        .expect("compiled empty withdrawal");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let receipt: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("empty withdrawal receipt JSON");
    assert_eq!(receipt["removed_files"], serde_json::json!([]));
    assert_eq!(receipt["forced_files"], serde_json::json!([]));
    assert_eq!(receipt["removed_directories"], serde_json::json!([]));
    assert_eq!(receipt["removed_links"], serde_json::json!([]));
    assert_eq!(receipt["retained"], serde_json::json!([]));
}

#[test]
fn asset_operations_never_change_recorded_evidence() {
    let root = tempfile::tempdir().expect("temporary repository root");
    let evidence = root.path().join("reports/skill-evidence/demo/events.jsonl");
    fs::create_dir_all(evidence.parent().expect("evidence directory"))
        .expect("create evidence directory");
    fs::write(&evidence, b"immutable evidence sentinel\n").expect("write evidence sentinel");
    let expected = fs::read(&evidence).expect("read evidence before asset operations");

    assets::install(root.path(), &host(), false).expect("install");
    assets::install(root.path(), &host(), true).expect("force install");
    write_pristine_retired_package(root.path());
    assets::withdraw(root.path(), &host(), false).expect("withdraw");
    write_pristine_retired_package(root.path());
    let edited = root
        .path()
        .join(".claude/skills/legacy-skill-decontamination/SKILL.md");
    fs::write(&edited, "consumer edit\n").expect("edit retired file");
    assets::withdraw(root.path(), &host(), true).expect("force withdraw");

    assert_eq!(
        fs::read(&evidence).expect("read evidence afterward"),
        expected
    );
}
