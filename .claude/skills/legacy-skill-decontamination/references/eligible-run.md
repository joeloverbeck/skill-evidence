# Eligible decontamination run

You are here only because the preflight printed `eligible: true`. Eligibility established provenance, nothing more: the run's job is to prove which accretion can go, and the baseline may turn out to be worth keeping wholesale. Work from the target and its real usage history; old audit reports and witness statements are provenance clues to reproduce or disregard, never an answer key.

Compiled command family (all event writes, from the repository root): `cargo run --locked -p skill-evidence -- skills decontamination <command> --target <skill-path> …`. Every command takes explicit `--recorded-at`, `--now-epoch-milliseconds`, `--session-id`, and `--lock-owner` inputs. Mutating commands also take caller-owned `--event-id` and `--repository-head` inputs; `claim` takes a caller-owned `--run-id`. The command never reads the ambient clock or generates an identity. Run artifacts live under `reports/skill-evidence/<skill-key>/decontamination/<run-id>/`.

### 1. Claim and snapshot the run

```bash
cargo run --locked -p skill-evidence -- skills decontamination claim \
  --target <skill-path> --run-id <run-id> --basis <basis> \
  [--basis-note "<provenance>"] [--basis-ref <routing-event-id>] \
  [--trials <n>] [--risk-rationale "…"] --event-id <event-id> \
  --recorded-at <RFC3339-clock> --now-epoch-milliseconds <clock-ms> \
  --repository-head <repository-head> --session-id <top-level-session-id> \
  --lock-owner <caller-owned-lock-id>
```

The compiled command re-runs the eligibility gate under the store lock, snapshots a hash-verified baseline copy to `decontamination/<run-id>/baseline/`, and appends `decontamination_started`; the run then owns the target. Raise `--trials` above five when the target governs destructive or externally visible actions, data migrations or state integrity, security, privacy, credentials, or publication, complex multi-skill coordination, or several distinct core branches — and say why in `--risk-rationale`.

*Done when the compiled command printed a `run_id` and the baseline copy exists.*

### 2. Freeze the representative behavior corpus

Before reading for removals, freeze at least the claimed trial count of raw paired-trial tasks under `decontamination/<run-id>/corpus/`, covering: the skill's most common core use; a second materially different core use; a known fragile or high-risk branch; an adjacent or unusual but valid case; and a regression case protecting the behavior most likely to be lost through simplification. Prefer real historical tasks and raw artifacts, stripped of audit diagnoses and expected answers. Freeze per task: the raw prompt and input artifacts, observable success criteria or a comparison rubric, and any deterministic checks.

If the available history cannot support a representative corpus: `complete --run-id <id> --outcome blocked_no_valid_test --note "<what was missing>"`, make no edit, and go to step 9's report.

*Done when the full corpus is frozen on disk before any candidate exists, or the run was completed as blocked.*

### 3. Establish the current behavioral baseline

Run every corpus task against the unchanged current skill in fresh sessions or independent agents with minimal task-local context, retaining raw outputs, check results, and evaluator notes under `decontamination/<run-id>/trials/`. A bloated skill may be behaviorally effective; this baseline is the capability the candidate must preserve.

*Done when every corpus task has a retained current-skill result.*

### 4. Classify structure and provenance

Now read the complete target and available provenance. Classify each substantive instruction or coherent group into exactly one category:

| Category | Default treatment |
|---|---|
| Core trigger or output contract | Preserve unless demonstrably inaccurate. |
| Domain knowledge unavailable to a general agent | Preserve or clarify concisely. |
| Necessary safety or state-integrity invariant | Preserve and test. |
| Current repository convention with a canonical owner | Preserve as a concise pointer or conditional rule. |
| Current tool-specific requirement | Preserve only with a current applicability test or narrow condition. |
| Duplicated instruction | Merge into one canonical home. |
| Incident narrative, dated witness, commit anecdote, or audit provenance | Remove from runtime; evidence history keeps it if useful. |
| One-off defensive exception | Remove unless trials or current contracts establish transferability. |
| Audit bookkeeping, report conformance, custody ceremony, self-audit scaffolding | Remove unless it directly protects the skill's actual domain operation. |
| Stale empirical quantity or environment assertion | Replace with a durable applicability check, or remove. |
| Contradictory or instruction-competing clause | Resolve toward the demonstrated core behavior. |
| Correct but disproportionately costly rare-case rule | Move to conditional reference, reduce, or remove after regression testing. |

A witnessed incident proves the event occurred, not that every future invocation must load its story or obey the exact repair chosen at the time.

*Done when every substantive instruction carries exactly one category.*

### 5. Mark audit-induced risk patterns

Candidate risks, not automatic deletions — tie every proposed simplification to structure, provenance, context cost, or a trial-protected behavior: qualifications layered on earlier qualifications; taxonomies minted to classify prior audit mistakes; instructions existing only to satisfy another audit check; dates, commit hashes, counts, or tool versions embedded in runtime prose; self-audit scaffolding unrelated to the skill's purpose; several rules solving one hazard in different places; repository-specific behavior stated as universal; hard procedures from a single uncorroborated incident; large examples teaching provenance rather than transferable action; rare-case defenses crowding the common path; frontmatter widened to match accumulated behavior; references every normal use must load, making their progressive disclosure nominal.

*Done when each marked pattern carries its justification.*

### 6. Construct one isolated candidate

Copy the baseline to `decontamination/<run-id>/candidate/` and modify only that copy, applying priorities in order: preserve the core trigger, domain contract, outputs, and safety invariants; remove audit and incident narration from runtime context; collapse duplicated definitions into one home; replace exact historical measurements with durable conditional checks where still necessary; move genuinely rare branches behind explicit progressive-disclosure triggers; resolve contradictions and instruction competition; shorten the common path; delete rules justified only by one uncorroborated incident; keep uncertain domain knowledge until trials adjudicate it.

One hypothesis-driven candidate — never several competing candidates keeping whichever wins. A mechanical defect found before trials may be corrected once; substantive failure ends the run without landing.

*Done when the candidate differs from the baseline only where step 4–5 analysis justifies it.*

### 7. Measure both versions

Record in the run artifacts, for current and candidate: runtime-loaded word or token estimate; mandatory normal-path references; steps and hard gates; incident or provenance passages; duplicated definitions where derivable; executable helpers added or removed. Comparison measures, not scores — smaller wins only while behavior stays adequate.

*Done when the comparison table is in the run artifacts.*

### 8. Run blind paired validation and the noninferiority gate

Run the frozen corpus against the candidate exactly as the baseline ran, in fresh sessions or independent agents. Evaluators compare unlabeled or randomized outputs and never receive the decontamination diagnosis, the removed clauses, the expected winner, or old audit findings as an answer key. The evidence store holds all of it — run artifacts, the diagnosis, and the candidate bytes — so every executor and evaluator prompt, on the baseline run and the candidate run alike, must bar reading it. Run deterministic checks on the candidate and on any current baseline needed for comparison.

The candidate may land only when it is noninferior on every core and safety-relevant task; introduces no material or severe regression; passes all applicable deterministic checks; retains necessary domain knowledge and ownership boundaries; is measurably smaller, clearer, less mandatory, or less internally conflicting; and every retained unusual rule has a transferable justification rather than audit provenance alone. If a removed clause protected a behavior the candidate loses, restore the smallest transferable invariant — never the incident story — and rerun the entire corpus. If the candidate is not meaningfully simpler, retain the current skill even on a behavioral tie; if uncertainty remains on a safety-relevant behavior, retain the current clause or stop without landing.

```bash
cargo run --locked -p skill-evidence -- skills decontamination record-validation \
  --target <skill-path> --run-id <id> --decision accepted|rejected \
  --candidate reports/skill-evidence/<skill-key>/decontamination/<run-id>/candidate \
  --trials <count> --artifacts reports/skill-evidence/<skill-key>/decontamination/<run-id> \
  [--summary "…"] --event-id <event-id> --recorded-at <RFC3339-clock> \
  --now-epoch-milliseconds <clock-ms> --repository-head <repository-head> \
  --session-id <top-level-session-id> --lock-owner <caller-owned-lock-id>
```

*Done when the decision is recorded from the trial results alone.*

### 9. Land when accepted, complete, report

```bash
cargo run --locked -p skill-evidence -- skills decontamination land \
  --target <skill-path> --run-id <id> --candidate <same candidate path> \
  --event-id <event-id> --recorded-at <RFC3339-clock> \
  --now-epoch-milliseconds <clock-ms> --repository-head <repository-head> \
  --session-id <top-level-session-id> --lock-owner <caller-owned-lock-id>
```

`land` reconfirms everything before touching the live target — live hash still equals the claim baseline, candidate bytes exactly those validated, run still owns the target — then backs up the baseline, replaces the live bytes, verifies the landed hash and the `.agents` mirror symlink, and appends `change_landed`. If it refuses because the target moved: complete with `superseded_by_target_version`; never merge a validated candidate into an independently changed target by intuition. If landing verification fails, it restores the baseline itself; record the failure. Do not commit to Git automatically.

Then complete — the compiled command enforces outcome consistency with what actually happened:

```bash
cargo run --locked -p skill-evidence -- skills decontamination complete \
  --target <skill-path> --run-id <id> --outcome <outcome> --note "<rationale>" \
  --event-id <event-id> --recorded-at <RFC3339-clock> \
  --now-epoch-milliseconds <clock-ms> --repository-head <repository-head> \
  --session-id <top-level-session-id> --lock-owner <caller-owned-lock-id>
```

Write the report at `decontamination/<run-id>.md`, with unreached sections marked `not reached — <outcome>`:

```markdown
# Legacy Skill Decontamination: <skill-name>

## Eligibility
- Legacy basis:
- Target before hash:
- Prior decontamination status:

## Current-skill baseline
- Core behaviors:
- Runtime size and mandatory references:
- Paired-trial corpus:

## Accretion analysis
- Preserved invariants:
- Removed or distilled audit-induced material:
- Uncertain material retained:

## Candidate
- Files changed:
- Runtime size before/after:
- Structural simplifications:

## Comparative validation
- Trial results:
- Deterministic checks:
- Regressions or uncertainty:

## Decision
- Outcome:
- Landed: yes/no
- Target after hash or unchanged hash:
```

The user-facing completion is concise, links the report, and states whether the live skill changed.

*Done when the completion event exists, the report is written, and the completion was delivered.*

## No same-run expansion

The eligibility basis covers audit accretion, not everything imperfect. A functional defect noticed while reading is evidence for Skill Evidence Capture and a later evolution cycle, not extra scope for this candidate: decontamination simplifies, it does not repair. If a defect directly fails a frozen trial or makes the candidate unsafe, record it as a trial result and stop short of landing rather than fixing it here.

## Terminal outcomes

Every invocation ends in exactly one state. `refused_not_legacy_eligible`, `refused_already_completed`, and `refused_self_target` end in `SKILL.md` step 2 with no event and no report. Claimed runs end as `blocked_no_valid_test`, `superseded_by_target_version`, `candidate_rejected_validation`, `healthy_no_change`, or — the only outcome that modifies the live target — `validated_simplification_landed`. Name the terminal outcome in the completion.
