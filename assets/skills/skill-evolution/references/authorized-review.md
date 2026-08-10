# Authorized review

You are here only because the preflight printed `authorized: true`. Work from its bounded evidence packet — the trigger events, use counts on the current hash, related prior dispositions, and the concrete artifacts they cite. Do not ingest the full historical ledger; the gate projection exists to keep old incident lore from dominating current judgment. The threshold authorized a diagnosis, not a presumption that the skill is defective or a guarantee of an edit.

`prior_reviews` indexes every review already completed on this target, whatever cluster it adjudicated, because what a predecessor ruled about the instrument or the mechanism is symptom-independent. Read the report of each one whose `same_target_hash` is true before step 4: those judged these exact bytes and govern how far this review can get. Also read any older entry whose note bears on this cluster — it landed a change, but a standing instruction, such as one against repeating a behavioral reproduction, survives the landing. Rediscovering a predecessor's ruling by running trials is waste; contradicting it silently is worse. `related_prior_dispositions` remains the narrower symptom-linked view for duplicate-mechanism judgment.

Compiled command family (all event writes, from the repository root): `cargo run --locked -p {{cargo_package}} -- skills evolution <command> --target <skill-path> …`. Every command takes explicit `--recorded-at`, `--now-epoch-milliseconds`, `--session-id`, and `--lock-owner` inputs. Mutating commands also take caller-owned `--event-id` and `--repository-head` inputs; `claim` takes a caller-owned `--review-id`. The command never reads the ambient clock or generates an identity. Review artifacts live under `reports/skill-evidence/<skill-key>/reviews/`.

### 1. Claim the review

```bash
cargo run --locked -p {{cargo_package}} -- skills evolution claim \
  --target <skill-path> --review-id <review-id> --risk-tier provisional \
  --event-id <event-id> --recorded-at <RFC3339-clock> \
  --now-epoch-milliseconds <clock-ms> --repository-head <repository-head> \
  --session-id <top-level-session-id> --lock-owner <caller-owned-lock-id>
```

The compiled command re-evaluates every authorization term under the store lock, appends `review_started` (trigger IDs, authorizing rule, baseline target hash, provisional risk tier, fresh-session or cooldown proof), and re-derives the gate to `review_in_progress`. If it refuses — another review owns the target, or the gate moved — relay the refusal and stop without semantic analysis.

*Done when the compiled command printed a `review_id` and the review owns the target.*

### 2. Verify threshold premises

Before thinking about repairs, check against the packet only:

- every trigger event represents a qualifying use;
- events claimed as independent are genuinely independent — distinct top-level sessions or materially different tasks, not retries, continuations, subagent reruns, or duplicate accounts of one event;
- at least one threshold event, including the threshold-crossing one, is contemporaneous;
- trigger hashes match the current target version;
- the candidate symptom cluster is factually plausible as a common symptom (causality is confirmed later, in step 3).

On failure, carry `insufficient_independence` or `superseded_by_target_version` and a note naming what failed to step 9; do no further review work.

*Done when every premise was confirmed, or the failed premise and terminal disposition are ready for step 9.*

### 3. Determine target ownership and causal mechanism

Now — and only now — read the target skill, plus the minimum external contracts needed to test ownership. Classify the evidence:

| Causal disposition | It proceeds? | Terminal outcome → close disposition |
|---|---|---|
| **Target defect** — misleading, contradictory, missing, or badly placed guidance causally connected to the incidents | yes | — |
| **Target compliance defect** — the right rule exists but its structure, placement, salience, or instruction competition repeatedly defeats compliance | yes | — |
| Outside target — another skill, contract, tool, environment, model limitation, or user instruction owns it | no | `outside_target` → `outside_target` |
| Task-specific novelty — does not generalize beyond the triggering task | no | `resolved_no_change` → `closed_no_skill_defect` |
| Not reproducible from the evidence — the packet supports no mechanism the target could own | no | `not_reproducible` → `monitor_for_recurrence` |

For a non-proceeding class, carry the mapped disposition and a factual note to step 9. Route outside-target evidence to its owner factually without proposing an unsanctioned repair, and never edit another owner from this review.

The mechanism stays a *candidate* here. Only step 5's current arm can confirm it, so do not treat an unconfirmed mechanism as absent and do not close `not_reproducible` merely because no trial has run yet.

Before freezing the plan, read `workaround_taken` only from the raw trigger events in the evidence packet. State what those recorded workarounds establish about the candidate mechanism and target ownership, or state that none was recorded on a trigger event. Repeated suppression of the mechanism is evidence for target ownership because the mechanism responds to instruction the target could carry; a workaround that was taken without suppressing the mechanism is evidence against target ownership. Record the direction as evidence, never as a verdict.

Using the packet's candidate cluster for this authorization, count the open incident IDs outside the trigger set and state that count, including zero. This count discloses how many incident payloads the bounded workaround read could not reach. Do not characterize, estimate, or reason about those incidents; do not read the historical ledger or seek their payloads.

*Done when the candidate mechanism and ownership class are written down, and a non-proceeding class has its terminal disposition and note ready for step 9.*

### 4. Freeze the validation plan before any candidate exists

Name the incident's binding constraint first — the condition without which the failure does not occur, such as load, volume, instruction recency or context distance, caller-owned input size, or elapsed run length — then decide whether the trial instrument can vary it. A trial executor starts fresh and short-context with the contract in hand, so a constraint that exists only deep inside a long run is outside what any trial set can express. When the instrument cannot vary the constraint, or no meaningful fresh validation can be constructed at all, carry `blocked_no_valid_test` and a note naming the constraint to step 9; freeze no plan, build no candidate, and run no trial.

For each constraint the instrument can vary, name its **witness**: the observable in a finished run's own output or artifacts that shows whether that run expressed the constraint. The frozen plan must also name the observation that would make the witness read unexpressed. An observable that no finished run could make read unexpressed does not show *whether*, and is not a witness; the constraint it was meant to read takes the `blocked_no_valid_test` exit above before a candidate exists. Anything a reader can check in what the run already produces qualifies — no new instrument is required — and a constraint nothing a finished run yields could show is one this instrument cannot vary, so it takes the same exit. Fix the reading now, before any outcome exists: a witness reading unexpressed takes `blocked_no_valid_test`, naming that constraint, to step 9. Chosen after a result is in hand, that reading is not evidence.

A same-hash predecessor that already ran trials constrains this judgment: if it reports the current arm passing on a mechanism shape these incidents repeat, that is evidence the wording is followed when freshly read, and the constraint is the run condition rather than the text. Do not re-derive it by rerunning equivalent trials.

The incidents' recorded run conditions are the evidence for this judgment, not a guess about them. When they agree that the failures arrived at volume, late in a long run, or only intermittently, say so here and treat a fresh short-context single-run trial as unable to express that.

Reconcile the frozen plan with step 3's workaround finding. Keep the plan consistent with that directional evidence without letting it replace a trial or skip or shrink the frozen trial set; blind comparative validation remains the gate.

Every reproduction trial runs at least 3 times when its first-run witness reads expressed. Where the recorded `run_condition` and `workaround_taken` fields support an estimate of the per-launch incidence, freeze that estimate in the plan, choose a reproduction run count above the floor against it, and state how that estimate supports the chosen run count. Where the record does not support an estimate, say so explicitly and use the floor. The floor forbids a single-run reproduction; incidence-based sizing carries the statistical weight. Step 5's first-run rule remains an early stop: an unexpressed reading closes the arm at that one run instead of spending its planned remainder.

Only then define the trials, so the change cannot pick only tests it already knows how to pass. Ordinary, narrow change — at least three paired trials:

1. a fresh reproduction of the implicated mechanism;
2. an adjacent case exercising the same capability differently;
3. an unrelated core regression case.

Escalate to at least five paired trials (add another core-regression case and a fragile, edge, or safety-relevant case) when the change affects destructive or external actions, state integrity or confidentiality, shared conventions or multiple skills, triggering or scope boundaries, a broad workflow section, more than one major behavior, or substantial deletion or reorganization.

Freeze per trial: the raw prompt/task, raw input artifacts, an observable pass/fail or comparison rubric, its witness, any deterministic checks, which behavior it protects, and evaluator-independence requirements. Save the frozen plan under `reviews/<review-id>/`.

*Done when the binding constraint is named and variable by the trials, its witness and unexpressed reading are fixed, and the full trial set is frozen on disk, or `blocked_no_valid_test` and its note are ready for step 9.*

### 5. Construct an isolated candidate

Run the frozen reproduction trial(s) on the unchanged current skill before building anything, under step 6's rules; they are its current arm, not extra runs, and their outcome never reopens the frozen trial set.

Read the witness on the **first** of those runs before spending another. Unexpressed → carry `blocked_no_valid_test` and a note naming the constraint the run did not express to step 9 at that one run: no candidate, no further trials, and no re-cut prompt or fixture. Expressed → the arm continues under the frozen plan.

If the mechanism does not recur across the arm, no candidate can be materially better — but a passing arm carries two readings, and the close must say which one it rests on. The arm's witnesses decide, run 1's included:

- expressed on every run, and the failure still did not appear — the target handles it: carry `monitor_for_recurrence` and a note naming the condition the arm reproduced and what it did to step 9;
- unexpressed on any run — that run did not hold the condition, so the pass is uninformative about the incidents: carry `blocked_no_valid_test` and a note naming the condition the arm could not express to step 9.

Never record both readings, and never let a pass default to `monitor_for_recurrence` when the condition went unexpressed — that retires the trigger events on a conclusion the trials did not reach. Then go to step 9.

Copy the live target to `reviews/<review-id>/candidate/` (outside skill discovery) and modify only that copy; the live target stays untouched until every trial passes. Design rules:

- solve the demonstrated mechanism, not every imperfection seen while reading;
- do not fix unrelated defects noticed during the review — they become evidence only if a real skill use records them;
- prefer deletion, consolidation, reordering, or clearer replacement over appending; keep ambiguity/salience repairs token-neutral or smaller;
- no incident narratives, audit provenance, dates, commit hashes, or field stories in runtime instructions;
- growth only for a proven missing capability that cannot be expressed by replacing existing text;
- tool- or repository-specific details go in conditional references, not universal runtime rules; shared guidance keeps one canonical home.

*Done when the first run's witness read expressed, the mechanism recurred on the current arm, and the candidate differs from the live target only where the mechanism demands it, or the arm-supported disposition and note are ready for step 9.*

### 6. Run blind comparative validation

Run every frozen trial against both the unchanged current skill and the candidate, using fresh sessions or independent agents with minimal task-local context. Give executors the raw task and artifacts — never the diagnosis, intended repair, expected answer, or which version they hold; randomize or conceal version labels for evaluators. The evidence store holds all of it — incident bodies, this review's diagnosis, and the candidate bytes — so every executor prompt must bar reading it. Run applicable deterministic checks on both versions where comparison matters, and on the candidate before landing. Retain raw outputs and evaluator decisions under `reviews/<review-id>/`.

*Done when every frozen trial ran on both versions and the raw outputs are on disk.*

### 7. Apply the acceptance gate

The candidate passes only when it resolves the implicated mechanism on the reproduction case(s); is noninferior on every protected core behavior; introduces no material or severe regression; passes all affected deterministic checks; preserves safety, scope, and ownership invariants; any growth is necessary, minimal, and supported by better outcomes; and it is materially better on the target mechanism rather than merely worded differently. Behaviorally tied: prefer the candidate only when it is meaningfully smaller or clearer; otherwise the current skill stays.

On failure, leave the target untouched: `record-validation --decision rejected …`, then carry `candidate_rejected_validation` to step 9. A rejected candidate is not a license to improvise another in the same review — new evidence must reopen eligibility. Sole exception: a mechanical candidate defect discovered before any behavioral trial may be corrected once, then the complete frozen suite reruns.

*Done when the acceptance decision is made from the trial results alone.*

### 8. Record, land, verify

```bash
cargo run --locked -p {{cargo_package}} -- skills evolution record-validation --target <skill-path> \
  --review-id <id> --decision accepted --risk-tier <ordinary|high> \
  --candidate reports/skill-evidence/<skill-key>/reviews/<review-id>/candidate \
  --trials <count> --artifacts reports/skill-evidence/<skill-key>/reviews/<review-id> \
  --event-id <event-id> --recorded-at <RFC3339-clock> \
  --now-epoch-milliseconds <clock-ms> --repository-head <repository-head> \
  --session-id <top-level-session-id> --lock-owner <caller-owned-lock-id> [--summary "…"]
cargo run --locked -p {{cargo_package}} -- skills evolution land --target <skill-path> \
  --review-id <id> --candidate <same candidate path> \
  --event-id <event-id> --recorded-at <RFC3339-clock> \
  --now-epoch-milliseconds <clock-ms> --repository-head <repository-head> \
  --session-id <top-level-session-id> --lock-owner <caller-owned-lock-id>
```

`record-validation` freezes the validated candidate hash; `land` reconfirms everything before touching the live target — live hash still equals the claim baseline, candidate bytes exactly those validated, review still owns the target — then backs up the baseline, replaces the live bytes, verifies the landed hash and the `.agents` mirror symlink, and appends `change_landed`. If it refuses because the target moved, authorization expired: carry `superseded_by_target_version` to step 9 — never merge a validated candidate into an independently changed target by intuition. If landing verification fails, it restores the baseline itself; record the failure. Do not commit to Git automatically; normal repository recoverability stays as it is.

*Done when `land` printed the before/after hashes and changed-file list, or the no-landing disposition is ready for step 9.*

### 9. Report, close, amend, complete

Every close records the trigger events the review covered in `adjudicated_event_ids`; for an adjudicating disposition, add `--adjudicate <event-id>` only for additional events the review genuinely covered. The adjudicating dispositions are `resolved_by_change`, `closed_no_skill_defect`, `outside_target`, `insufficient_independence`, `monitor_for_recurrence`, and `candidate_rejected_validation`. Their covered event IDs retire from the active set.

`blocked_no_valid_test` and `superseded_by_target_version` are non-adjudicating dispositions: their payload still records the trigger coverage, but those IDs do not retire from the active set because the review reached no conclusion. Do not pass `--adjudicate` with either disposition; the compiled command refuses that combination. Trigger events stay in `events.jsonl` forever, and the gate projection combines the disposition with the coverage list to decide retirement.

`blocked_no_valid_test` is further **instrument-limited**: closing it asserts that this instrument cannot test the evidence it covered, so the projection retires that evidence from the *evolution gate* while leaving it open and unadjudicated in the ledger. Those incidents stop clustering and can no longer reach a threshold, and the projection names them in `instrument_limited_incident_ids` — which the preflight evidence packet carries too, so the packet's open-incident count and its clusters still reconcile.

Retirement reaches the open incidents the claim's authorization reason would name if re-evaluated at the close, rather than every same-symptom sibling: `friction_recurrence` names the whole open symptom cluster; `material_recurrence` names only material-or-worse incidents in its symptom cluster; `ten_use_unresolved` names the open incidents in its anchor cluster except retrospective ones; and `severe` names only its triggering incident, which is itself never retired, so the reach is empty. The coverage list remains covered, including incidents recorded before the threshold fired; incidents recorded after the close are new evidence and drive the gate normally. A contemporaneous severe incident is never retired under any reason because it authorizes on its own. A retrospective severe incident retires when the authorization reason names it. If a historical `review_started` has no recognized `authorizing_rule`, derivation uses the prior symptom-wide reach rather than narrowing on a guess. `superseded_by_target_version` retires nothing; the target moved, and nothing was established about testability.

Before a `blocked_no_valid_test` close, read the authorization reason and coverage list from the claim receipt. Re-evaluate that authorization reason against the live candidate clusters in the gate projection, using the roster above to identify the widest reason-specific reach bound. Vouch that the binding constraint you named plausibly covers every incident in that bound, not every same-symptom sibling. Vouch only for incidents inside that reason-specific reach bound; same-symptom incidents outside it do not affect this close. The projection is current as of the last recorded incident, so no extra derive run is required. This reason-specific reach bound can name incidents the close will not retire: a contemporaneous severe incident is never retired because it authorizes on its own. If you cannot vouch for an incident in that reason-specific reach bound, record the mismatch in the review report and user-facing completion.

Before any close, write the review report at `reviews/<review-id>.md`, with unreached sections marked `not reached — <disposition>`. Fill every section the review has reached and put `pending close receipt` only where the receipt supplies the final value. The compiled close command verifies that this report file exists before it appends anything; it does not judge the report's prose.

```markdown
# Skill Evolution Review: <skill-name>

## Authorization
- Gate rule:
- Trigger event IDs:
- Target before hash:
- Fresh-session/cooldown proof:

## Evidence adjudication
- Independence result:
- Confirmed mechanism:
- Target ownership:
- Recorded-workaround finding:
- Non-trigger open incident count:

## Candidate
- Change hypothesis:
- Files changed in isolated candidate:
- Runtime size before/after:

## Frozen validation plan
- Risk tier:
- Paired trials:
- Deterministic checks:

## Results
- Current version:
- Binding condition reproduced by the current arm: yes/no/not reached
- Candidate version:
- Regressions:
- Decision:

## Landing
- Landed: yes/no
- Target after hash or unchanged hash:
- Final disposition:
- Retirement reach event IDs:
```

Then close with the disposition carried here from the terminal branch:

```bash
cargo run --locked -p {{cargo_package}} -- skills evolution close \
  --target <skill-path> --review-id <id> --disposition <disposition> \
  --note "<mechanism and result>" --event-id <event-id> \
  --recorded-at <RFC3339-clock> --now-epoch-milliseconds <clock-ms> \
  --repository-head <repository-head> --session-id <top-level-session-id> \
  --lock-owner <caller-owned-lock-id>
```

When validation ran without building a candidate or recording `validation_completed`, add `--trials <count> --artifacts reports/skill-evidence/<skill-key>/reviews/<review-id>`. These are attributed assertions: the close records them for later reviewers and never uses them for eligibility, retirement, reach, or any other authorization. Omit them when no validation ran; absent means not recorded, never zero. A close after `record-validation` need not repeat the effort already carried by `validation_completed`.

After the close succeeds, amend the review report with the final disposition and every close-receipt value that was pending. Read `retired_from_gate_event_ids` from the close receipt whenever it is present: it names this close's retirement reach, including an empty reach. The projection's `instrument_limited_incident_ids` is instead the standing per-hash set and can include retirements from earlier reviews. Copy the receipt's list into the report and state that retirement reach in the user-facing completion. A non-instrument-limited close omits the key and has no retirement reach to report.

The user-facing completion is concise, links the report, states whether the live skill changed, and, when the close receipt carries `retired_from_gate_event_ids`, states that retirement reach exactly, including an empty list.

*Done when the report existed before close, the disposition event exists, the report carries the close receipt's final values, and the completion was delivered.*

## No same-review expansion

Unrelated imperfections noticed during an authorized review are not in scope: do not fix them, do not broaden the candidate, and do not manufacture an incident merely from reading the skill. If an unrelated defect directly causes a frozen-trial failure, record it as a trial result and leave it for a later evidence cycle unless it makes the current candidate unsafe. This is what keeps a narrow authorized review from turning back into a general audit.

## Terminal outcomes

Every invocation ends in exactly one state. `refused_closed_gate`, `refused_cooldown_or_same_session`, and `refused_self_target` end in `SKILL.md` step 2 with no event and no report. Claimed reviews end as `superseded_by_target_version`, `insufficient_independence`, `outside_target`, `not_reproducible`, `blocked_no_valid_test`, `candidate_rejected_validation`, `resolved_no_change`, or — the only outcome that modifies the live target — `resolved_by_validated_change` (disposition `resolved_by_change`). Name the terminal outcome in the completion.
