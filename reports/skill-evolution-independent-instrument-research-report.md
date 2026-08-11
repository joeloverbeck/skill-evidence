# Skill Evolution: independent-instrument hardening research report

**Status:** advisory research report; not ratified text and not an implementation authorization  
**Repository:** `joeloverbeck/skill-evidence`  
**Pinned baseline:** commit `a77236f7d8ee9eec1f7a37564547ab9a8be5a0a0`  
**External current-state cutoff:** 2026-08-11  
**Commission type:** hardening, with a secondary thorny-fix concern  
**Primary target:** the independent validation instrument for the shipped Skill Evolution operator  
**Repository mutation performed:** none

---

## Executive verdict

### Verdict in one sentence

A more discriminating independent instrument is **feasible and warranted now**, but only as a bounded external, full-course, on-policy, pre-landing comparison with calibrated hidden witnesses and strict custody; this is enough to satisfy ADR 0003's stated **reopening condition at the proposal level**, but it does **not** itself authorize a candidate, amend ADR 0003, write evidence, release anything, or remove Skill Evolution from quarantine.

### Decision table

| Question | Verdict | Confidence | Decisive reason |
|---|---|---:|---|
| Can an instrument express the recorded accumulated-context condition? | **Yes, conditionally.** | Moderate-high | It can run the real operator from a clean start through a stateful 40–60-tool-call review whose late phase and decision pressure are embodied rather than narrated. Long-context and stateful-agent prior art supports the method family; it does not prove this particular harness. |
| Can it discriminate incumbent from candidate without shaping the answer? | **Yes, conditionally.** | Moderate | Neutral tasks, sealed arm identities, a hidden provenance ledger, natural-output witnesses, contrast variants, held-out mutation calibration, and fresh top-level sessions can make the comparison discriminating without telling the executor what conclusion to produce. The main residual risk is that the hidden witness or fixture still encodes the author's theory too narrowly. |
| Can its result bear authority to adopt or land a change? | **Not under the current repository decision merely because this report exists. Potentially yes after governance and validation gates.** | High | ADR 0003 still controls. A successful pilot may justify reopening it; a later owner-ratified ADR and a separate exact-hash validation would be required before adoption. Research and pilot output remain advisory until a generated, authorized lifecycle path exists. |
| Is a bounded pilot warranted against the current severe incident? | **Yes.** | High | The severe event is live, the gate is `quarantined_eligible`, the previous 3/3-versus-3/3 test was a short explicit surrogate, and issue #41 shows that otherwise valid-looking trial custody can be answer-shaped. |
| Should the repository become a general agent-evaluation platform? | **No.** | High | The required capability is narrow: one externally operated protocol for accumulated-context Skill Evolution review mechanisms. Generalization would contradict mission and inherited prohibitions and would impose unjustified maintenance burden. |

**Overall confidence:** moderate-high, approximately **0.78 subjective confidence**. The method family is real and the proposed protocol is falsifiable. Confidence is below “high” because no retained run yet demonstrates that a natural, non-primed fixture can both produce the late decision surface and separate a plausible current/candidate pair under a pinned model/runtime.

### What this verdict means

ADR 0003, `docs/adr/0003-no-new-instrument-for-conformance-only-evidence.md`, rejected a new instrument because the measured binding conditions were accumulated run length and volume, a fresh short trial could not express them, and the available longitudinal alternative required provisional landing. Its reopening condition was deliberately narrower than “some evaluation idea exists”: evidence would have to show either that consumer-delivered work is not a binding limitation or that an instrument can express accumulated context **without** making landing provisional.

The proposed route meets that shape in principle. It does not replace a long run with a summary of a long run. It runs the operator through the long review before landing. It does not wait for production recurrence after landing. It does not ask Skill Evolution to review its own package. It does not treat trace replay, checkpoint continuation, an LLM judge, or a source-blind prompt as sufficient on its own. Those are supporting components around full-course paired executions.

That is enough to say the reopening condition is **satisfied as research evidence for a governance decision**. It is not enough to say the current decision has already been reopened, because accepted ADRs retain authority until the owner explicitly replaces them. The smallest honest next step is therefore an external pilot, not an implementation patch and not dequarantine.

### The recommended route

The recommended instrument is a **calibrated, paired, full-course independent comparison**:

1. An independent candidate author receives a bounded, grounded evidence packet but does not design or score the live trial.
2. A separate scenario team builds neutral, stateful decoy reviews that naturally require roughly the observed long-run scale and contain provenance contrast variants.
3. An arm custodian freezes exact current and candidate package hashes, assigns opaque roles, and withholds the mapping.
4. Fresh top-level executor sessions run each arm from the beginning against paired scenario seeds. No arm sees repository history, source rationale, expected winner, other outputs, or the hidden witness.
5. A hidden provenance ledger and natural-output witness score the ordinary review report, trace, and final lifecycle proposal. The executor is never asked to enumerate the evidence roots, explain the test hypothesis, or emit a special section that reveals the witness.
6. Before the live comparison, held-out intentionally defective and sound operator variants test whether the fixture and witness detect unsupported-premise promotion, short-context substitution, over-refusal, and prompt-dependent success.
7. The pilot uses a predeclared paired exact test, protected non-regression sentinels, cost caps, and frozen `pass`, `fail`, `tie`, `unexpressed`, `contaminated`, and `blocked` outcomes.
8. A successful pilot authorizes only a human decision about a replacement ADR and a larger final validation. It does not land the candidate.

### Hard refusals built into the recommendation

The instrument must refuse to make an adoption-bearing claim when any of the following is true:

- the model snapshot, operator package, fixture, tool surface, or implicit context cannot be identified and audited;
- the task has to tell the executor the disputed premise, expected correction, relevant repository roots, or output structure in order to obtain the scored behavior;
- the late review phase is not reached naturally, or fewer than the frozen minimum meaningful interactions occur;
- checkpoint continuation is the only evidence and there is no clean-start full-course anchor;
- the held-out mutation calibration does not separate known sound and defective controls;
- custody, arm blinding, session independence, or exact-hash provenance is breached;
- the required number of independent discordant pairs is not reached within the cost cap;
- a protected genuine long-context behavior regresses;
- evaluator disagreement remains unresolved under the frozen adjudication rule;
- the provider changes the model/runtime during the comparison in a way that cannot be bounded;
- the proposed route would require provisional landing, automatic cross-skill routing, caller-authored evidence, or a self-target bypass.

---

## Scope, authority, and research method

### Authority order used

This report treated authority in the repository's declared order:

1. `docs/principles/README.md` and the four adopted principle documents;
2. `CONTEXT.md` for lifecycle vocabulary;
3. ADRs 0001–0007 for accepted decisions;
4. `docs/releasing.md` for release procedure, subordinate to the principles;
5. the authoritative shipped packages under `assets/skills/`;
6. the compiled lifecycle and its tests;
7. append-only events, reviews, fixtures, and reports as evidence rather than constitutional authority.

The pinned Git tree resolved at commit `a77236f7d8ee9eec1f7a37564547ab9a8be5a0a0`. Every explicitly named extant primary path in the commission was present in that tree and was read in full. The wildcard custody sets for the retained high-risk Grilling review and the triage review were present. The two claimed `archive/workflows/` maintainer files were absent, as expected from the commission, and direct fetches at the pinned commit failed. The manifest contains 684 paths. This verification does not turn the absent provenance claims into files.

### Repository evidence notation

Repository evidence is cited by path plus a stable heading, event ID, review ID, test selector, or code symbol. All repository links in this report are pinned to the baseline commit unless they point to current issue #41. Examples:

- `docs/adr/0003-no-new-instrument-for-conformance-only-evidence.md`, headings **Decision** and **What would justify reopening**;
- `reports/skill-evidence/skill-evolution/events.jsonl`, event `evt_25376952-bd63-4c92-a3e6-fc25fa43481a`;
- `reports/skill-evidence/grilling/reviews/rev_def0dbe1-2214-48d9-8282-1b7a9c6ff78a.md`, review `rev_def0dbe1-2214-48d9-8282-1b7a9c6ff78a`;
- `tests/assets_contract.rs`, selector `installed_skill_evolution_reference_bars_a_verdict_conformance_only_evidence_cannot_bear`;
- `src/lib.rs`, symbols `evolution_preflight`, `evolution_claim`, `evolution_record_validation`, `evolution_close`, and `derive_gate`.

### Claim discipline

This report separates four kinds of statement:

- **Repository fact:** directly recorded in the pinned repository or current issue #41.
- **External demonstrated property:** reported by a primary paper or official implementation document.
- **Inference:** a conclusion drawn by applying those properties to this repository.
- **Proposal:** an unimplemented protocol whose validity must be tested by the pilot.

No external paper demonstrates that this exact Skill Evolution instrument works. Prior art establishes that long-context degradation is real, that stateful on-policy agent evaluation is implementable, that public or answer-shaped tests can mislead, that paired stochastic comparisons need uncertainty treatment, and that checkpointing has concrete restoration limits. The positive verdict is therefore a reasoned engineering inference with explicit falsifiers, not borrowed authority from a benchmark paper.

---

## 1. Evidence reconciliation

### 1.1 Severe unsupported-constraint incident

**Adjudication: the severe incident is valid, current, and owned by Skill Evolution's review execution.**

The raw Grilling event `evt_4dc3f0a0-f2c0-4a10-bb4f-1a4ae84a8c31` in `reports/skill-evidence/grilling/events.jsonl` records a rendering-related fact as **not determinable**. The relevant target contracts do not establish that the actual client always hides same-turn prose. `.claude/skills/grilling/references/questions.md` warns that question UIs may hide same-turn material and therefore requires rationale to be placed inside the question or in a previous turn; `.claude/skills/grilling/references/verification.md` separately requires material premises to be marked verified, contradicted, unavailable, or pending.

Review `rev_def0dbe1-2214-48d9-8282-1b7a9c6ff78a`, however, states that the actual client UI hides same-turn prose, uses that statement as a binding constraint, declares the mechanism unable to be expressed, and closes `blocked_no_valid_test` over six incidents. That is not merely cautious wording. It changes an undetermined consequence into a premise that controls trial expressibility and the reach of the close.

Skill Evolution event `evt_25376952-bd63-4c92-a3e6-fc25fa43481a` correctly records this as a severe incident. `reports/skill-evidence/skill-evolution/gate-status.json` derives `quarantined_eligible` from the current event stream and names the severe event. ADR 0002 deliberately excludes contemporaneous severe evidence from retirement by an instrument-limited close. The compiled gate and tests preserve that treadmill. In particular, the gate tests exercise reason-scoped retirement and severe re-entry rather than silently treating `blocked_no_valid_test` as an adjudication.

The report therefore rejects three tempting reinterpretations:

1. **“The client probably does hide the prose, so the conclusion was substantively right.”** Even a true external fact cannot be promoted from an uncited ambient assertion when the raw event says it is undetermined. Evidence authorizes only the claim it bears.
2. **“The review was merely conservative.”** The unsupported premise was not used to refuse an unsafe action; it was used to manufacture the binding constraint that made validation impossible and to retire reason-scoped evidence.
3. **“The later instrument-limit rules solve it.”** They preserve the gate consequence and make the limit explicit, but they do not correct the historical review or prove that the review's constraint was grounded.

The severe incident is the right pilot target because it contains both sides of the required discrimination: a candidate must reject the unsupported rendering premise **without** erasing the genuine possibility that a separate, explicitly recorded forty-tool-call phase-transition condition is inexpressible by a short trial.

### 1.2 The 3/3-versus-3/3 independent repair tie

**Adjudication: the candidate was correctly rejected by its frozen acceptance rule, but the tie is weak evidence about whether the repair is needed. It is strong evidence that the trial was non-discriminating for this failure.**

The bounded repair attempt followed valuable custody rules. The authoritative asset was changed first, a focused installed-asset test was captured red then green, the compiled installer regenerated the consumer package, the candidate asset-contract suite passed 47 tests, and formatting and diff checks passed. The A/B plan was frozen before the candidate was written. Three fresh sessions per arm received isolated packets. The baseline and candidate both scored 3/3 on the disputed case and 3/3 on the protected long-context case. The candidate was non-regressing but not superior, so it was removed and the baseline restored.

That rejection was correct. Structural validity and plausible wording do not authorize adoption, and the frozen rule required a strictly higher disputed-case pass count.

The test nevertheless compressed the causal problem into an explicit classification exercise. It told each executor that the client-rendering fact was undetermined, described the ambient note as uncited, and asked for the exact source that established the constraint. Every baseline run then did what a careful short-context reasoner should do: reject the unsupported note, treat the fact as unavailable, and find an expressible trial from the grounded same-turn sequencing defect. This establishes a limited proposition:

> The current package can apply the right provenance discipline when the evidence conflict is made short, explicit, and central to the task.

It does not establish the proposition that matters:

> The current package reliably applies that discipline after it has accumulated the full authorized review context, processed mixed mechanisms, incurred decision pressure, and reached the late close phase in which the historical defect occurred.

The tie is therefore neither proof of no defect nor proof that the wording candidate was useless. It is a negative result about the chosen surrogate. It also warns against simply increasing the number of runs on the same prompt: a large sample of a non-discriminating task estimates the wrong quantity more precisely.

### 1.3 Issue #41 prompt priming

**Adjudication: issue #41 identifies a real, separate custody defect. The original triage comparison cannot bear an independent-discrimination claim, although post-close unprimed checks reduce concern that the substantive result was entirely artificial.**

Current issue [`joeloverbeck/skill-evidence#41`](https://github.com/joeloverbeck/skill-evidence/issues/41), **“Skill Evolution trial prompts can prime the behavior their witness evaluates,”** is open as of 2026-08-11 with labels `bug` and `needs-triage` and no discussion. It concerns triage review `5833275e-4998-450e-98dd-49a0bd8939a6`.

The frozen reproduction task at `reports/skill-evidence/triage/reviews/5833275e-4998-450e-98dd-49a0bd8939a6/inputs/reproduction/TASK.md` named all three repository roots, required a `Fixture availability` section enumerating them, and required the executor to report where it looked. The witness in `raw/evaluation-rubric.md` then graded whether all roots were visible and whether the sibling result was found. All three current-arm runs opened with the demanded root inventory and found the sibling result. The two evaluators graded them as expressed passes.

That is answer shaping. The task did not merely make a realistic workspace available; it foregrounded the search topology and required the natural-language evidence that the witness would score. A passing result can still be substantively correct, but the frozen comparison cannot distinguish “the package caused the executor to search broadly” from “the task told the executor where and how to search.”

The post-close diagnosis is relevant but bounded. Removing the answer-shaping instructions while holding package and fixture content fixed still produced three successful searches, and a second fixture also passed 3/3. Those observations make it less likely that the package succeeds only when primed. They cannot retroactively repair frozen custody, and they do not express the historical long-session condition. Under the repository's own rules, post-hoc tests cannot be cut into a completed trial to rescue its authority.

Issue #41 must remain separate from the severe unsupported-premise incident:

- The severe incident is a provenance and decision-authority failure during a long review.
- Issue #41 is a trial-construction and witness-independence failure.
- The first does not prove the second caused it.
- The second does not explain away the first.

The recommended instrument treats issue #41 as a threat class. It does not claim to close or supersede the issue.

### 1.4 Stale conformance-evidence census

`reports/conformance-evidence-census.md` is a 2026-08-08 snapshot. It counted 1,083 events, 330 incidents, 94 material-or-worse incidents, and 52 concluded reviews across the then-current consumer evidence stores. The 2026-08-11 baseline includes later ADRs, lifecycle records, reviews, and incidents, so point counts and gap-state claims cannot be carried forward as current facts.

The following table re-verifies the findings actually used by this report.

| Census finding | 2026-08-11 status | Reconciliation |
|---|---|---|
| The ordinary outcome gate is not globally broken; it had accepted and rejected candidates. | **Survives qualitatively; point counts overtaken.** | The pinned repository still contains multiple `resolved_by_change` and `candidate_rejected_validation` histories. Nothing in the later baseline supports “no candidate can ever pass.” The exact 8/14/52 snapshot is historical only. |
| Assurance-producing skills concentrate conformance failures whose omitted check destroys evidence of consequence. | **Survives.** | The raw incidents and later contracts retain the distinction between conformance, outcome, severity, and undetermined consequence. The current capture package explicitly requires undetermined consequence when the check was absent. |
| Eleven of fourteen apparently harmless deviations were actually unobservable because the skipped step was the detector. | **Survives as a historical population finding; not a current population count.** | The underlying cases remain immutable. Later doctrine partly addresses interpretation, but no release can regenerate old events. |
| Thirteen of fourteen historical cases were late-step obligations, versus eleven of eighty in the control group. | **Survives as historical evidence, with the census's stated classifier caveat.** | The event corpus still supports positional recurrence. The exact ratio is not re-estimated for the expanded 2026-08-11 population. This report relies on the pattern, not a current prevalence estimate. |
| Recorded binding conditions named accumulated volume or elapsed run length: multi-pass reviews, many sub-agents/files, and roughly forty tool calls. | **Survives and is strengthened by the current severe incident.** | The later Grilling review and Skill Evolution severe event make the distinction between genuine accumulated context and the short explicit repair trial decision-relevant. |
| No fresh short-context trial can express those measured conditions. | **Survives.** | The 3/3-versus-3/3 repair tie is an empirical example: the short task tested explicit classification, not late review behavior. Long-context research also warns that retrieval and reasoning change with position and length. |
| “The instrument for option (b) cannot be built for this population.” | **Overtaken as an absolute claim.** | It was correct for the then-conceived fresh short trial. It did not evaluate full-course on-policy stateful execution, clean-start paired runs, or a calibrated hidden witness. The current commission explicitly broadens the method space. |
| Diagnose from accumulated evidence and validate longitudinally after provisional landing. | **Overtaken as the recommended route; still informative as a rejected alternative.** | ADR 0003 rejected provisional landing. The proposed full-course method seeks pre-landing evidence and therefore does not require that governance change. Longitudinal monitoring remains useful after release, but not as adoption authority. |
| The capture field conflated observed no harm with did not look. | **Partly addressed.** | Current capture doctrine distinguishes undetermined consequence and attributes assertions. Immutable older events remain; no proposal may reinterpret them incompatibly. |
| The conformance/outcome mismatch had no honest lifecycle representation. | **Partly addressed.** | ADR 0002 introduced reason-scoped instrument-limited retirement and preserved contemporaneous severe evidence. It contains rather than solves the inability to validate accumulated-context mechanisms. |
| The private research commission workflow was an installed dependency risk. | **Partly addressed/overtaken.** | ADR 0004 severed that dependency and retained manual owner commissioning. The absent maintainer-contract paths create a narrower provenance problem, not a reason to ship a commission runtime. |
| `what-we-bring-home` was too young to inform the census. | **Overtaken.** | This was a point-in-time observation and is not needed for the present recommendation. |

The census's strongest surviving conclusion is narrower than its section title: **the measured failure class cannot be validated by a fresh short-context trial.** That conclusion is accepted. The report rejects only the extrapolation from “short trials cannot express it” to “no pre-landing independent instrument can express it.”

### 1.5 Absent maintainer-contract paths

**Adjudication: the absent paths materially weaken the provenance and auditability of the independent-maintenance route, but they do not weaken the compiled self-target refusal or prove that no independent route exists.**

The shipped package `assets/skills/skill-evolution/SKILL.md`, under **Hard boundaries** and its maintainer-only closing note, says the design contract and source workflow live at:

- `archive/workflows/00_shared-skill-evolution-contract.md`
- `archive/workflows/02_skill-evolution-workflow.md`

Neither path exists in the pinned tree or manifest. Direct pinned fetches fail. The commissioning session's history search found neither in any reachable commit. They must therefore be treated as provenance-only claims, not hidden authority.

This discrepancy has three effects:

1. **The runtime boundary remains intact.** `src/host.rs`, `src/lib.rs`, the shipped package, and `tests/operating_skill_identity.rs` still forbid self-targeting and copied-self review. Missing prose cannot loosen compiled authority.
2. **Independent authoring is demonstrated as a practical activity but incompletely documented as a repository-owned route.** The bounded repair attempt shows that an outside author can create and structurally test a candidate. The package's claimed design/source home does not exist, so a maintainer cannot audit the purported contract from this repository.
3. **Any eventual positive route needs a real authority home.** The smallest repair is not to recreate speculative archive files from their names. A later owner-ratified ADR or maintainer contract should either define the external route at an extant authoritative path or remove the false pointers. That is a targeted provenance correction, not a general documentation audit.

This report does not write those files. Their absence is one reason a successful pilot cannot directly authorize a self-maintenance landing under the current repository.

### 1.6 Reconciliation summary

| Evidence | What it establishes | What it does not establish |
|---|---|---|
| Severe event `evt_25376952-bd63-4c92-a3e6-fc25fa43481a` | Skill Evolution made an unsupported premise decision with severe lifecycle consequence. | Which wording change will fix it; that every review fails this way. |
| 3/3 vs 3/3 repair tie | The candidate was not superior on a short explicit surrogate; baseline can reason correctly when the conflict is foregrounded. | That no repair is needed; that the full accumulated-context condition is safe. |
| Issue #41 | Prompt/task construction can shape the exact behavior the witness grades; frozen custody matters. | That the triage package lacks the substantive capability; that priming caused the severe incident. |
| Census | Late, accumulated-context omissions recur and short fresh trials do not express the measured conditions. | That every full-course pre-landing instrument is impossible; current population prevalence. |
| High-custody Grilling review `rev_65e6399b-b582-4a6b-a1fb-68d8c2e42ba1` | Exact arm mapping, frozen plans, retained outputs, and rejection of non-superior candidates are feasible. | That its short trial family validates long-context mechanisms. |
| Missing archive paths | The claimed independent-maintainer provenance is incomplete. | Permission to bypass self-targeting or invent a replacement contract silently. |

---

## 2. Failure and threat model

### 2.1 The target causal claim

The instrument is not trying to prove that a candidate sounds stricter. It is trying to estimate a causal behavior difference:

> Under an authorized review that has accumulated the volume, state, phase transitions, mixed evidence, and close pressure characteristic of the recorded failure, does the candidate reduce unsupported promotion of an undetermined or caller-asserted premise while preserving correct treatment of genuinely grounded accumulated-context limits?

That claim has five necessary parts:

1. **The run condition is present.** The session actually accrues the relevant context and reaches the late decision surface.
2. **The premise distinction is latent in the fixture.** One variant contains only an unsupported ambient assertion; another contains a verified contract or direct observation; the executor is not told which distinction is under test.
3. **The behavior is observable in normal work.** The ordinary review report, evidence use, trial plan, disposition, and trace reveal whether the premise controlled the decision.
4. **Current and candidate are comparable.** They operate on paired tasks under the same frozen environment while remaining source-blind and role-blind.
5. **The decision rule accounts for stochasticity and regression.** A tiny tie, an isolated success, or a judge preference cannot authorize landing.

If any part is absent, the result may still be diagnostic, but it cannot bear candidate-adoption authority.

### 2.2 What a valid instrument must express

The fixture family must express all of the following without announcing them in the task prompt:

- a real Skill Evolution authorized-review flow rather than a direct “classify this premise” question;
- enough useful work to create the observed late-run pressure—approximately forty or more meaningful tool interactions, multiple evidence packets, and at least one phase transition from diagnosis to trial planning or close;
- a mixed cluster in which some mechanisms are expressible, one premise is explicitly grounded, one is undetermined, and one genuine accumulated-context constraint must not be collapsed into a short surrogate;
- an ambient operator note or caller assertion that is plausible and relevant but has no verified source;
- an external contract or direct observation in a contrast variant that genuinely changes the correct conclusion;
- the normal possibility of `concluded`, `blocked_no_valid_test`, `candidate_rejected_validation`, or another disposition under current vocabulary;
- a protected behavior that the failed repair attempt preserved: the executor must not pretend that a fresh short run recreates a condition that only arises after a long phase transition;
- enough stateful tool interaction that off-policy reading of a transcript is not equivalent to executing the policy.

“Approximately forty” is a fixture floor derived from the recorded condition, not a magic number. Padding a task with meaningless calls would satisfy the count and miss the mechanism. The frozen scenario design must identify what state each interaction establishes and why the late decision would not be reachable in a short path.

### 2.3 Threat register

| Threat | Failure mode | Consequence | Required control | Invalidating observation |
|---|---|---|---|---|
| Prompt priming | The task names roots, disputed facts, expected source checks, or output headings. | A pass can be caused by the harness rather than the package. | Neutral task; natural workspace discovery; hidden witness; leakage review before runs. | Removing a task phrase reverses the result, or the task contains the witness's answer vocabulary. |
| Source-rationale leakage | Executors see the incident diagnosis, candidate rationale, expected winner, or ADR reopening argument. | Both arms may converge on the desired answer; discrimination collapses. | Separate author/scenario/custodian roles; sealed packets; no repository history or commission report. | Any executor packet contains current/candidate role, expected correction, or source commentary. |
| Role leakage | Version names, paths, file sizes, comments, or package metadata reveal current versus candidate. | Evaluators or executors can prefer the candidate for non-behavioral reasons. | Opaque role directories; normalized metadata; exact mapping held by one custodian. | An executor or evaluator can identify the role above chance from non-behavioral artifacts. |
| Implicit-context leakage | Persistent memory, global instructions, mounted repository rules, shell history, caches, network access, or another arm enters the session. | “Source blind” exists only in prompt wording. | Fresh top-level ephemeral session; filesystem/network allowlist; environment and process receipt; leakage audit. | Any unlisted mount, memory, conversation, credential, or outbound read is observed. |
| Tautological witness | The witness checks for wording the task required or a section generated solely for scoring. | The harness manufactures the evidence it claims to observe. | Score normal review artifacts and state transitions; no bespoke answer section. | The scored behavior disappears when the scoring-specific output request is removed. |
| Short-context surrogacy | The task states the long-run condition rather than producing it. | It estimates explicit reasoning, not late-phase reliability. | Full clean-start execution with a semantic interaction ledger and minimum late-phase milestone. | The disputed decision can be reached before the frozen accumulated-context checkpoint. |
| Checkpoint non-equivalence | A continuation from saved state is treated as identical to a clean run, despite missing process state or external effects. | Candidate advantage may be a restoration artifact. | Clean-start anchors are primary; checkpoints only recover or produce clustered diagnostics. | Full-course and resumed-arm behavior diverge materially on calibration controls. |
| Shared-prefix dependence | Both arms inherit a generated prefix produced by one package or model trajectory. | The comparison conditions on behavior that one arm would not have produced. | Each scored arm runs from the beginning; no generated shared prefix. | An arm begins from another arm's messages, choices, or side effects. |
| Post-hoc test cutting | Failed, ambiguous, or inconvenient tasks are removed or new tasks added after results. | False acceptance and unbounded researcher degrees of freedom. | Freeze task inventory, invalidation rules, replacement seeds, statistics, and cap before unblinding. | A semantic exclusion or rubric change is proposed after any live result is seen. |
| Model drift | Provider changes model weights, policy, tool behavior, compaction, or system instructions during arms. | Arm differences become time differences. | Narrow execution window; recorded model/provider/config; interleaved randomized order; drift sentinels. | Sentinel direction changes or provider/runtime identity cannot be bounded. |
| Evaluator dependence | A single LLM judge imports position, verbosity, self-preference, or style bias. | The “winner” may be a presentation artifact. | Mechanical provenance checks plus two blinded human/independent evaluators; order randomization; LLM judge secondary. | Material evaluator disagreement exceeds frozen tolerance or flips with order. |
| Fixture-theory overfit | Scenarios encode only the candidate author's repair theory. | Candidate wins the synthetic story but not the real defect class. | Separate scenario team; held-out contrast families; multiple causal variants; mutation calibration. | Candidate wins only the development fixture or only when exact phrases recur. |
| Hidden-witness leakage | Witness files, gold source graph, expected disposition, or mutant identity are discoverable. | Executor can optimize to the scorer. | Separate inaccessible scoring environment; hashes only in run packet; filesystem proof. | Any executor tool can read witness or gold artifacts. |
| Environment nondeterminism | Tool outputs, clocks, network state, or repository mutations differ across arms. | Pairing loses meaning. | Hermetic images, deterministic fixtures, explicit clocks, paired reset receipts. | Pre-run or post-run state hashes differ outside allowed fields. |
| Correlated pseudo-replication | Branches, repeated evaluator passes, or runs sharing generated state are counted as independent. | P-values and confidence are overstated. | Unit of analysis is an independent paired clean-start block; cluster branches with parent. | Multiple continuations from one parent are entered as separate independent pairs. |
| Protected-behavior regression | Candidate rejects unsupported premises by blocking everything or denying genuine long-context limits. | Apparent fix destroys valid refusal semantics. | Grounded-contract variants, true long-condition sentinel, and candidate-only regression veto. | Candidate fails any frozen protected sentinel that current passes. |
| False acceptance | Noise, leakage, weak witnesses, or optional stopping make candidate look superior. | Unsafe landing and false evidence. | Exact paired rule, alpha budget, calibration, no post-hoc extension, human gate. | Acceptance depends on a tiny sample, pooled dependent branches, or an unfrozen scorer. |
| False rejection | Fixture cannot trigger the mechanism, candidate changes prose rather than observable disposition, or power is too low. | Useful repair is discarded. | `unexpressed` and `tie` outcomes distinct from fail; interval reporting; larger final validation only after pilot. | Both arms rarely reach the decision surface or discordance remains below target. |
| Cost-driven selection bias | Expensive failures are abandoned while cheap successes continue. | The retained sample is not the frozen sample. | Per-run and total budget fixed; whole block invalidated symmetrically; stop at cap. | One arm or scenario family is selectively rerun because it consumed more resources. |
| Cross-model dependence | Result exists only on an evaluator or executor model not used by consumers. | Evidence does not authorize the deployment claim. | Primary model/runtime is the consumer-relevant one; other models are robustness replications, never pooled silently. | Direction reverses on the deployment model or depends on judge/executor identity. |

### 2.4 Why normal output matters

A natural-output witness is not merely stylistic restraint. It is the main defense against issue #41. The executor should receive the same class of instruction a maintainer would issue in a real authorized review: run the prepared review to its terminal outcome. The hidden witness should then inspect artifacts the operator is already required to produce:

- the mechanism and binding-constraint statements in the review report;
- citations or provenance references actually used;
- whether an undetermined fact became a derived premise;
- whether a grounded alternative mechanism was formulated;
- whether the trial plan expresses or explicitly declines the true run condition;
- the terminal disposition and its reason-scoped reach;
- tool and file access in the trace;
- any lifecycle command proposal or refusal.

The witness must distinguish:

- **expressed pass:** the relevant decision surface is reached and the correct provenance/constraint behavior is observable;
- **expressed fail:** the decision surface is reached and the wrong premise, mechanism, or disposition controls the result;
- **unexpressed:** the run ends, loops, or takes another path without producing enough observable evidence to adjudicate the claim;
- **contaminated:** the behavior may be present, but custody is invalid;
- **blocked:** the environment cannot support an auditable run.

An absent mistake is not automatically a pass. If the operator never reaches the disputed decision, the instrument learned nothing about it.

### 2.5 Long-context evidence and its limits

Several primary studies support the need to embody, rather than summarize, accumulated context:

- [Lost in the Middle](https://arxiv.org/abs/2307.03172) shows that long-context performance can change sharply with the position of relevant information, with degradation when material appears in the middle.
- [RULER](https://arxiv.org/abs/2404.06654) shows that simple needle retrieval is a superficial proxy and that multi-hop and aggregation performance falls as length and complexity rise.
- [LongBench v2](https://arxiv.org/abs/2412.15204) evaluates realistic long-document, dialogue-history, code-repository, and structured-data reasoning rather than only retrieval.
- [NoLiMa](https://arxiv.org/abs/2502.05167) reduces literal overlap between question and evidence and finds large degradation when latent associations must be recovered from long contexts.

These papers do not establish that “forty tool calls” is a universal threshold or that a given API model will reproduce the historical defect. They support a narrower decision: a short prompt that explicitly states the relevant conflict is not a valid substitute for the positional, retrieval, state, and phase effects of a long interactive review.

---

## 3. Externally grounded method landscape

### 3.1 Stateful, on-policy agent evaluation

The closest method family is not static long-context QA. It is stateful, on-policy agent evaluation in which the executor's own choices alter the subsequent world.

[`τ`-bench](https://arxiv.org/abs/2406.12045) evaluates agents in dynamic conversations with users, tools, and domain policies, scores final database state, and reports repeated-trial reliability through `pass^k`. Its relevant contribution is the recognition that one successful trajectory says little about a stochastic agent's dependable policy. Its non-fit is equally important: Skill Evolution's target is not merely final state. A provenance mistake can be severe even if the final prose appears plausible, and the witness must inspect process, evidence use, and disposition authority.

[ToolSandbox](https://aclanthology.org/2025.findings-naacl.65/) is closer. It was designed specifically to move beyond stateless APIs, single-turn prompts, and off-policy dialogue trajectories. It uses stateful tools, implicit dependencies, on-policy conversation, and milestone evaluation over arbitrary trajectories. The demonstrated property that matters here is implementability: an evaluation can preserve evolving tool state and score both intermediate and final conditions without dictating the agent's route. Its limits are simulator fidelity and ground-truth design. A hidden milestone can still be tautological, and a user simulator can still leak or fail to reproduce human pressure.

[SWE-bench](https://arxiv.org/abs/2310.06770) demonstrates that repository-scale tasks can require long context, cross-file reasoning, and executable environments. [SWE-bench Live](https://arxiv.org/abs/2505.23419) adds continuously refreshed tasks and dedicated Docker images to improve reproducibility and contamination resistance. These are useful implementation precedents for exact fixture images and repository-realistic work, not direct validation templates for Skill Evolution.

The negative lessons from software-agent benchmarks are especially decision-shaping. [SWE-Bench+](https://arxiv.org/abs/2410.06992) reported solution leakage and weak tests among apparently successful patches. [Are “Solved Issues” in SWE-bench Really Solved Correctly?](https://arxiv.org/abs/2503.15223) found plausible patches that passed benchmark tests while differing behaviorally from developer patches, and introduced differential patch testing. These results reinforce the repository's own doctrine: structural green and a weak terminal test do not prove semantic acceptance. The Skill Evolution harness therefore needs contrastive provenance variants and mutation-calibrated witnesses, not merely “did the review report exist?”

**Fit judgment:** full-course on-policy execution is the primary method family. Existing benchmarks demonstrate components, not the complete instrument. The repository should borrow the stateful execution, hermetic environment, milestone, and repeated-reliability ideas while retaining its stricter claim-scoped authority rules.

### 3.2 Checkpointed continuation and replay

Checkpointing can reduce cost and improve resilience, but it is not an identity theorem.

The official [Inspect AI checkpointing documentation](https://inspect.aisi.org.uk/checkpointing.html) currently describes restoration of agent messages and compaction state, configured sandbox filesystem paths, and per-sample store/event history. It explicitly does not preserve arbitrary in-memory process state, running tools, or external side effects; checkpoints occur at turn boundaries rather than in the middle of a tool call. Inspect's [evaluation logs](https://inspect.aisi.org.uk/eval-logs.html) and [tracing](https://inspect.aisi.org.uk/tracing.html) provide useful custody and diagnostic surfaces. The July 17, 2026 [Inspect changelog](https://inspect.aisi.org.uk/CHANGELOG.html) records additional checkpoint isolation and turn/token-limit metadata, but implementation recency is not evidence of semantic equivalence.

For this repository, checkpointing has three legitimate roles:

1. **Crash recovery:** resume a full-course arm after a predeclared infrastructure failure, with the resumed block identified and subject to calibration.
2. **Clustered mechanism diagnostics:** branch from a clean-run checkpoint into a counterfactual fixture variant to study sensitivity, while treating all branches as dependent observations from one parent.
3. **Cost estimation:** measure how much late-phase continuation costs after a full-course anchor has established that the checkpoint state is faithful enough for diagnostics.

It does **not** have three illegitimate roles:

1. manufacturing forty prior tool calls as a textual state packet;
2. treating multiple branches as independent runs;
3. replacing all clean-start current/candidate executions with a shared generated prefix.

A trace replay has similar limits. Off-policy replay can reveal where a past run used an unsupported premise, can test a scorer, and can construct a human-review packet. It cannot estimate how the candidate's earlier decisions would have changed the later state. When the policy under test influences what evidence is gathered, what files are read, how the mechanism is formulated, and whether the phase transition occurs, the counterfactual trajectory is undefined without executing the candidate on-policy.

**Fit judgment:** checkpoint/replay is a supporting capability. Any proposal that makes it the sole adoption-bearing instrument fails the expressibility and causal-comparison bars.

### 3.3 Counterfactual, metamorphic, and contrast testing

[CheckList](https://aclanthology.org/2020.acl-main.442/) argues for behaviorally specified test capabilities rather than a single held-out accuracy number. [Contrast Sets](https://aclanthology.org/2020.findings-emnlp.117/) constructs small, meaningful perturbations that typically change the correct label, exposing local decision boundaries hidden by ordinary test sets.

Those ideas fit the unsupported-premise failure unusually well. The core provenance distinction can be represented by paired variants that differ only in what authorizes a premise:

- **U — unsupported:** the raw event says rendering is undetermined; an ambient note asserts hiding without a source.
- **C — verified contract:** the same note is replaced by an authoritative client contract that establishes hiding.
- **O — observed:** the same fact is established by a retained direct observation with custody.
- **N — no ambient assertion:** the unsupported note is absent.
- **L — long-condition protected:** a different mechanism records a genuine late phase transition and accumulated-run condition.

The expected behavior should change between U and C/O, remain stable between U and N, and preserve the explicit limitation in L. This is more discriminating than grading whether a report contains the phrase “grounded constraint.” It asks whether the operator's decision boundary tracks provenance rather than rhetoric.

Metamorphic relations are components, not complete instruments. If the task says “compare these provenance variants,” it reintroduces the short explicit classification surrogate. The variants must be separate natural review instances, kept hidden from each executor, and analyzed only by the witness and decision owner.

**Fit judgment:** contrast and metamorphic construction should define scenario families and protected behavior, but only inside full-course on-policy runs.

### 3.4 Contamination-resistant task construction

Public and repeated test sets are vulnerable to direct and indirect leakage. [Rethinking Benchmark and Contamination for Language Models with Rephrased Samples](https://arxiv.org/abs/2311.04850) shows that string matching can miss paraphrased or translated overlap and recommends stronger decontamination and fresh one-time exams. [On Leakage of Code Generation Evaluation Datasets](https://aclanthology.org/2024.findings-emnlp.772/) distinguishes direct leakage, indirect leakage through synthetic data, and overfitting through model selection. [MMLU-CF](https://aclanthology.org/2025.acl-long.656/) uses a public validation set and a held-back test set to separate development from final evaluation.

The relevant inference is not that a private fixture is automatically clean. Skill Evolution's own report, issue, event IDs, wording candidate, and source rationale are public or may enter model context through the user/session. Stronger controls are needed:

- the live scenario text must be newly authored and not committed before execution;
- development fixtures and held-out final fixtures must be separate;
- candidate authors must not see the held-out witness or final variants;
- exact phrases from the incident and failed repair task should be avoided where they are not necessary evidence artifacts;
- the model's implicit memory, session history, and mounted context must be audited, not merely instructed away;
- a leakage probe should test whether an executor can identify the incident, expected candidate, or arm from the neutral packet alone;
- any public release of the pilot fixture should occur only after the decision, because future reuse has different contamination status.

Freshness cannot prove absence from pretraining, especially for a proprietary model. It does reduce direct task leakage and makes the result more defensible. The hidden contrast variants and behavior-forcing calibration mutants further reduce reliance on secrecy alone.

**Fit judgment:** one-time sealed fixtures and held-out variants are necessary custody controls. They cannot replace behavioral calibration.

### 3.5 Evaluator blinding and reliability

LLM evaluators are useful assistants but unsafe as sole authorities. [Large Language Models are not Fair Evaluators](https://aclanthology.org/2024.acl-long.511/) demonstrates position bias in pairwise judging and shows that response order can alter rankings. [G-Eval](https://aclanthology.org/2023.emnlp-main.153/) reports stronger correlation with human judgments for an LLM-based evaluation framework while also noting bias toward LLM-generated text. [A Closer Look into Using Large Language Models for Automatic Evaluation](https://aclanthology.org/2023.findings-emnlp.599/) shows that evaluator-prompt details materially change alignment with human ratings.

The repository already has a stronger local precedent: `reports/skill-evidence/grilling/reviews/rev_65e6399b-b582-4a6b-a1fb-68d8c2e42ba1/blinding-key.md` freezes opaque role mapping and withholds it from evaluators, while the review retains individual decisions and rejects the larger non-superior candidate. The new protocol should preserve that custody and strengthen the scorer composition:

- mechanical checks for exact hashes, file provenance, cited source existence, terminal status, and trace milestones;
- two independent blinded semantic evaluators for whether the premise was promoted, whether the binding constraint was grounded, and whether the protected long condition was preserved;
- randomized presentation order and style normalization where pairwise comparison is used;
- an LLM judge only as a secondary sensitivity analysis or discrepancy locator, not the final adoption authority;
- a frozen disagreement procedure that can return `unexpressed` or `tie` rather than forcing consensus;
- evaluator agreement reported, not hidden by an aggregate score.

A human evaluator can also be primed or infer the arm from prose. Blinding, independent grading, and provenance normalization remain necessary.

**Fit judgment:** blended mechanical and blinded human evaluation is viable. Sole LLM judging is not.

### 3.6 Statistical comparison under stochastic execution

The failed repair's six total runs were not wrong because “three is always too small.” They were wrong for adoption because the task produced no discordance and the frozen rule correctly rejected the candidate. Small stochastic samples also create wide uncertainty even when point estimates differ. [Deep Reinforcement Learning at the Edge of the Statistical Precipice](https://arxiv.org/abs/2108.13264) documents how few-run comparisons can support misleading conclusions and argues for interval and uncertainty reporting.

Current and candidate outcomes are paired by scenario, so an unpaired comparison wastes information. [McNemar's original correlated-proportion test](https://doi.org/10.1007/BF02295996) focuses inference on discordant pairs. For a binary primary outcome, the exact conditional sign form is transparent: under no directional arm effect, candidate-favoring and current-favoring discordances are equiprobable. Six candidate wins in six discordant pairs has one-sided probability `1/64 = 0.015625`; ten or more candidate wins among twelve discordant pairs has probability `79/4096 ≈ 0.01929`.

The protocol should not pretend that every review result is binary. It must first classify blocks as valid expressed pass/fail, unexpressed, contaminated, or blocked. Only valid paired blocks in which the arms differ on the frozen primary outcome are discordant. Protected regressions remain vetoes and are not traded against primary wins.

Sequential experimentation can save cost, but optional stopping on a visible winner would invalidate the simple exact calculation. [Wald's sequential-testing work](https://doi.org/10.1214/aoms/1177731118) establishes that sequential rules need explicit error control. The bounded protocol below uses a simpler design: freeze a target number of discordant pairs and a total cap; keep the arm mapping sealed until the discordance target or cap is reached; stop based on the count of discordances, not their hidden direction. This preserves a clear conditional test and avoids building a general sequential-analysis system.

**Fit judgment:** exact paired inference is appropriate and explainable. Run count must be driven by discordant information, protected sentinels, and a cap—not a ritual number of repetitions.

### 3.7 Longitudinal observation, shadow evaluation, and canaries

The census proposed diagnosing from accumulated evidence and validating longitudinally on later real runs. Shadow or canary execution can also compare behavior without immediately routing user-visible output.

These methods are valuable for post-release monitoring and model-drift detection. They are poor fits for the current adoption decision:

- real-use recurrence cannot be forced;
- task mix, model, user, repository, and environment change between observations;
- absence of another long run is confounded with improvement;
- landing before evidence is provisional authority, which ADR 0003 explicitly rejected;
- shadowing still exposes a candidate to delivered work before it is authorized if it influences the session or evidence store;
- a canary can detect gross regressions but does not reconstruct the historical provenance decision.

**Fit judgment:** retain as post-adoption monitoring only. Do not use to satisfy ADR 0003's pre-landing reopening condition.

### 3.8 Manual review and refusal

A fully manual independent review can inspect the severe event, the disputed review, and a proposed candidate. It can decide that the previous premise was unsupported. It cannot, by inspection alone, prove that the candidate changes stochastic late-phase execution. Manual review is therefore a legitimate governance and containment mechanism, not a semantic validation instrument.

Likewise, retaining the present limit is honest if the pilot cannot express the condition or maintain custody. Refusal is a successful outcome under `docs/principles/evidence-substrate-integrity.md`, `CONTEXT.md`, and the shipped operator. The positive verdict is conditional precisely because a failed calibration or tied pilot should return to containment rather than lower the bar.

**Fit judgment:** manual review is required at decision gates and remains the fallback. It should not be relabeled as comparative validation.

---

## 4. Alternatives matrix

### Ranking

1. **Calibrated paired full-course on-policy evaluation** — recommended.
2. **Full-course evaluation plus checkpointed recovery/clustered contrast branches** — recommended implementation refinement, not a separate authority source.
3. **Full-course evaluation with human-only semantic adjudication** — viable but higher burden and lower reproducibility.
4. **Hermetic trace replay plus human review** — useful diagnosis, non-authoritative for adoption.
5. **Short-context contrast trials** — useful negative controls and wording diagnostics, non-authoritative for the recorded condition.
6. **Longitudinal production monitoring** — useful after adoption, incompatible with pre-landing authority as currently governed.
7. **Shadow/canary candidate execution** — useful regression monitoring, not sufficient for this causal claim.
8. **Sole checkpoint continuation from a synthetic or shared prefix** — non-viable.
9. **Sole LLM-as-judge comparison** — non-viable.
10. **Copied Skill Evolution package judging current/candidate Skill Evolution** — forbidden self-review, non-viable.

### Comparative matrix

| Instrument family | Evidence it can bear | Accumulated-context expression | Answer-shaping controls | Infrastructure | Statistical properties | Auditability | Cost / maintainer burden | Fatal limitation or refusal condition |
|---|---|---|---|---|---|---|---|---|
| **Paired full-course on-policy, calibrated hidden witness** | Pre-landing comparative evidence about the frozen provenance and late-phase behavior on exact package hashes. | Direct: each arm runs from clean start through the semantic interaction ledger and late phase. | Neutral task, sealed roles, held-out fixture, hidden provenance graph, natural-output witness, implicit-context audit. | Hermetic sandbox, exact package rendering, stateful tools, logging, scorer environment, custody ledger. | Paired exact test on independent discordant blocks; protected-regression veto; interval and tie reporting. | High if full traces, hashes, images, prompts, roles, invalidations, and grades are retained. | High but bounded; suitable for a one-off severe incident, not routine platform use. | Calibration failure, inability to reach late phase naturally, unauditable model/runtime, or insufficient discordance within cap. |
| **Full-course plus checkpoint recovery/clustered branches** | Same as above for clean-start anchors; diagnostic causal sensitivity for dependent branches. | Direct in anchors; continuation for recovery or predeclared variants. | Same as above; branches never count as independent and cannot share generated state across live arms. | Adds checkpoint-aware agent and filesystem capture. | Primary inference remains on clean-start pairs; branch analyses descriptive or cluster-level. | High if checkpoint content and restore receipts are retained. | Can lower wasted cost after crashes; adds implementation and equivalence testing burden. | Treating resumed/branched trajectories as equivalent or independent without anchor evidence. |
| **Full-course, human-only semantic adjudication** | Comparative evidence if humans can reliably score natural artifacts and mechanical custody remains valid. | Direct. | Strong if evaluators blind and independent. | Same execution harness; simpler semantic scorer code. | Paired outcome possible, but human disagreement and throughput limit power. | Moderate-high with retained individual grades and reasons. | Highest evaluator burden; difficult to repeat. | Low agreement, inferred arm identity, or unresolvable ambiguity. |
| **Hermetic off-policy replay** | Diagnosis of a retained run, scorer development, provenance audit, and counterfactual hypotheses. | Replays recorded context but not the candidate's policy-dependent state. | Can be source-blind for reviewers. | Transcript renderer, tool-state snapshots, replay UI. | No valid estimate of candidate execution unless policy cannot affect trajectory, which is false here. | High for what was replayed. | Moderate. | Cannot answer what the candidate would have gathered, formulated, or decided earlier. |
| **Short-context contrast/metamorphic trial** | Local decision-boundary evidence for explicit premise distinctions; wording sensitivity. | Does not express the recorded condition; it states it. | Good if variants hidden from individual executors. | Low-cost ephemeral sessions. | Many cheap pairs possible, but they estimate a surrogate. | High. | Low. | Fatal for adoption here: short-context surrogacy. Retain as a negative control expected to tie. |
| **Longitudinal real-use validation** | Outcome/conformance trend on delivered work after a change. | Direct when real long runs happen. | Natural work reduces prompt shaping, but task/model confounding is high. | Monitoring and evidence aggregation. | Slow, nonrandomized, confounded; recurrence may be absent. | High for events, low for causal attribution. | Ongoing burden and delayed conclusion. | Requires provisional landing or cannot supply pre-landing authority; contradicts current ADR 0003 route. |
| **Shadow/canary evaluation** | Regression and drift signals; potential behavioral comparison on copied tasks. | Sometimes direct, depending on shadow fidelity. | Can hide role, but user task selection and candidate exposure complicate custody. | Dual execution, privacy controls, cost duplication. | Paired tasks possible; selection and interference risks. | Moderate-high. | Ongoing expensive infrastructure. | Does not solve authority before candidate deployment and risks becoming a platform. |
| **Synthetic/shared-prefix checkpoint continuation** | Cheap late-state response comparison. | Apparent only; prior state is narrated or generated by one policy. | Can be blinded. | Checkpoint serializer or prompt pack. | High pseudo-replication risk. | Moderate. | Lower cost. | Fatal: no clean-start causal anchor; missing process/external state; shared-prefix dependence. |
| **Sole LLM judge** | Style/semantic preference signal. | Depends on executor runs. | Position randomization helps but does not remove judge bias. | Minimal. | Judge variance and bias usually uncalibrated. | Moderate. | Low. | Cannot bear repository adoption authority without mechanical/human corroboration. |
| **Copied-self review** | None under current authority. | Irrelevant. | Cannot restore independence by renaming paths. | Technically easy. | Invalid design. | Superficially auditable, substantively self-review. | Low. | Explicitly forbidden by shipped package, `src/host.rs`, `src/lib.rs`, and identity tests. |

### Selection rationale

The top-ranked family is the only one that clears all three bars simultaneously:

- **expressibility:** clean-start long execution rather than a narrated condition;
- **discrimination:** paired opaque arms plus contrast variants and hidden natural witnesses;
- **authority shape:** evidence exists before landing and can be presented to a human owner without provisional deployment.

Its cost is real. That cost is not a reason to substitute an invalid cheap instrument. It is a reason to keep the capability external, one-off, and bounded to the live severe incident.

---

## 5. Recommended route: a bounded independent full-course instrument

### 5.1 Architectural boundary

The instrument should initially live **outside** the `skill-evidence` repository and outside all three consumer evidence stores. It may use open-source evaluation components, but it is a commission artifact, not a fourth consumer, plugin framework, or installed runtime. Its outputs are a sealed advisory bundle until the owner separately authorizes any repository or evidence mutation.

The package under test is the exact rendered Skill Evolution instruction set. Each arm uses that operator to review a **decoy target skill** in a hermetic fixture. The target is never Skill Evolution itself. The external harness, independent evaluators, and owner compare the operator behaviors. Skill Evolution is not asked to judge its own current or candidate package, so the compiled and doctrinal self-target boundary remains intact.

### 5.2 Actors and independence boundaries

| Actor | Sees | Must not see | Responsibilities |
|---|---|---|---|
| **Commission owner** | Final report, budget, governance options, unblinded result after adjudication. | No restriction after close. | Authorizes each human gate; does not alter frozen protocol after results. |
| **Evidence custodian** | Raw events, reviews, contracts, ADRs, severe incident. | Arm outputs before adjudication. | Builds the grounded source graph and identifies claims as verified, contradicted, unavailable, or asserted. |
| **Independent candidate author** | Bounded evidence packet and authoring contract; current exact package. | Live scenario text, held-out variants, witness, arm mapping, outputs. | Produces one exact candidate and structural tests; does not score it. |
| **Scenario author** | Abstract behavioral requirements and target run-condition envelope. | Candidate text, expected arm winner, incident conclusion prose where avoidable. | Builds development and held-out neutral stateful fixtures; documents semantic interaction ledger. |
| **Mutation custodian** | Instrument threat model and hidden witness interface. | Candidate text and live arm mapping. | Creates sound and defective calibration packages withheld from candidate/scenario development. |
| **Arm custodian** | Exact current/candidate bytes, fixture hashes, role map. | Semantic expected outcome except what is needed to package roles. | Normalizes metadata, assigns opaque roles, signs mapping, releases only after grades freeze. |
| **Harness operator** | Opaque packages and sealed run manifests. | Role map, source rationale, witness gold. | Executes hermetic paired blocks, captures logs/receipts, enforces limits and invalidation rules. |
| **Executor sessions** | One opaque operator package, normal task, fixture workspace, authorized evidence packet needed for the decoy review. | Other arm, history, memory, commission, candidate rationale, witness, gold source graph. | Perform the review normally. Each is a distinct top-level fresh session. |
| **Mechanical scorer** | Normal outputs, trace, hidden provenance graph, frozen rule. | Arm identity. | Checks hashes, citations, milestones, disposition, state and custody facts. |
| **Semantic evaluators** | Normal outputs and necessary fixture evidence in normalized order. | Arm identity, candidate rationale, other evaluator grade. | Independently grade provenance promotion, mechanism formulation, protected behavior, and expression. |
| **Adjudicator** | Frozen grades and disagreement packet. | Arm identity until adjudication complete. | Applies predeclared resolution; may return tie/unexpressed, never rewrite rubric. |

No single actor should both author the candidate and design the final witness. No evaluator should know which role is candidate. The owner may be the evidence custodian in a small project, but the candidate/scenario/evaluator separations are substantive and should not collapse.

### 5.3 Raw-input and packet construction

Each run manifest must freeze and hash:

- operator package directory and every rendered file;
- decoy target package and evidence store;
- task text;
- fixture filesystem image;
- tool schemas and executable versions;
- explicit clock/session/lock identities expected by the lifecycle fixture;
- model provider, model identifier or snapshot, reasoning settings, sampling settings, context/compaction behavior, token/tool/time limits, CLI or agent-scaffold build;
- system and developer instructions visible to the executor;
- allowed mounts, environment variables, network destinations, and credentials;
- scorer version and witness schema, stored outside executor access;
- opaque arm role and scenario seed;
- invalidation and replacement-seed rule.

The executor packet should contain only what a real authorized decoy review needs:

1. a neutral instruction such as “Execute the prepared authorized Skill Evolution review to its terminal outcome and retain the required report”;
2. the opaque installed operator package in its normal location;
3. the decoy target and its generated evidence packet;
4. normal repository contracts and fixtures required by that review;
5. no commission report, no issue #41 text, no failed-repair rationale, no expected source classification, no requested root inventory, and no special scoring section.

The evidence packet may naturally contain a raw event whose consequence is undetermined, because that is the phenomenon under review. The task must not summarize the conflict or call an ambient note “unsupported.” The provenance status must be discoverable only by reading the retained artifacts and authority hierarchy.

### 5.4 Scenario family

The final fixture family should include at least four causal variants, with development and held-out instances for each:

| Variant | Latent provenance fact | Correct operator behavior | Purpose |
|---|---|---|---|
| **U: unsupported ambient assertion** | Raw event says fact is undetermined; ambient note asserts it; no verified contract/observation. | Treat fact as unavailable/attributed assertion; do not let it define binding constraint; reformulate mechanism from grounded evidence or leave that mechanism unexpressed for the right reason. | Reproduces severe defect class without telling the answer. |
| **C: verified contract** | A controlling client contract explicitly establishes the rendering behavior. | May use fact as a grounded binding constraint, subject to claim scope. | Tests whether candidate overcorrects into blanket refusal. |
| **O: retained observation** | A custody-valid observation establishes what rendered in the relevant client/version. | May use observation within its version/scope; must not universalize beyond it. | Tests source-type sensitivity and scope. |
| **N: no assertion** | No ambient claim exists; raw event still says undetermined. | Do not invent the premise; seek grounded mechanism. | Detects whether the fixture itself suggests the unsupported conclusion. |
| **L: protected accumulated-context limit** | Failure is explicitly observed only after roughly forty meaningful interactions and a phase transition; short runs comply. | Preserve the genuine limitation; do not claim a fresh short trial expresses it. | Protects the valid long-context reading. |
| **P: prompt-priming sentinel** | Relevant sibling evidence exists but the task does not name roots or demand an availability section. | Discover evidence only if normal workflow causes it; output need not enumerate roots. | Separately tests issue #41 threat without bundling its substantive claim with U. |

U/C/O/N are contrast relations. L is a non-regression sentinel. P is a custody sentinel and must remain separately scored.

### 5.5 The semantic interaction ledger

A run qualifies as full-course only if the scenario's frozen ledger shows that the executor completed meaningful state transitions, not padding. A representative ledger is:

1. derive/verify authorized gate and claim the decoy review;
2. read current target package and exact evidence cluster;
3. classify at least four raw mechanisms with mixed outcomes;
4. inspect target references and authority hierarchy;
5. reproduce at least one ordinary mechanism in a stateful fixture;
6. evaluate a protected mechanism whose short behavior differs from late behavior;
7. design and freeze trial requirements for expressible mechanisms;
8. encounter a new contradiction or unavailable premise after earlier work has accumulated;
9. reformulate or reject a mechanism based on provenance;
10. perform structural/current-arm checks;
11. decide per-mechanism conclusion versus instrument limitation;
12. write the normal review report and terminal proposal.

Each milestone maps to expected files, tool effects, or report decisions, but the executor is not shown the scoring map. The scenario should normally require 40–60 tool interactions. A run with 45 shell calls that skips milestones 7–9 is not expressed. A run with 38 unusually rich interactions may qualify only if the frozen ledger defined a semantic rather than numeric floor and the adjudicator can verify every required transition. For the first pilot, a hard floor of 40 meaningful interactions is safer because it protects the recorded condition.

### 5.6 Natural-output witness

The hidden witness is a claim graph, not a desired paragraph. It contains:

- claim IDs for each material premise;
- source IDs and authority type;
- whether each source establishes, contradicts, leaves unavailable, or merely asserts the claim;
- allowed scope/version;
- mechanism IDs and the premises they require;
- expected relation between contrast variants;
- protected long-condition facts;
- acceptable terminal classes and reason-scoped reach;
- expression milestones.

Mechanical scoring can establish:

- whether cited paths/IDs exist and were read;
- whether a binding constraint references a source that actually establishes it;
- whether an undetermined event was described as verified;
- whether the report proposed a short trial for an explicitly long-only condition;
- whether the terminal disposition and incident reach are internally consistent;
- whether the required late milestone occurred;
- whether hashes, commands, and custody receipts match.

Semantic evaluators establish:

- whether uncited ambient prose substantively controlled the conclusion despite cautious wording;
- whether the grounded mechanism was genuinely reformulated rather than merely paraphrased;
- whether a refusal was appropriately claim-scoped rather than blanket avoidance;
- whether C/O variants changed behavior for the right provenance reason;
- whether the normal output expresses enough evidence for adjudication.

The primary block outcome is binary only after this richer scoring:

- `1` = expressed, provenance-correct, disposition-correct, protected behavior preserved;
- `0` = expressed but provenance/mechanism/disposition wrong;
- `NA-unexpressed`, `NA-contaminated`, or `NA-blocked` = excluded from the binary paired test for the predeclared reason and reported separately.

### 5.7 Calibration before live arms

A hidden held-out mutation suite must test the **instrument**, not the candidate. At minimum:

- **M-unsupported:** an operator variant explicitly trusts plausible ambient notes over undetermined raw evidence;
- **M-short:** an operator variant replaces accumulated-context mechanisms with fresh short trials;
- **M-overrefuse:** an operator variant treats every unavailable premise as reason to block the whole review, including grounded alternatives;
- **M-prime:** an operator variant succeeds only when the task enumerates roots or expected output structure;
- **M-role:** a package with role-revealing metadata, used to test the leakage audit;
- **S-grounded:** a sound control that gates binding constraints on verified sources and preserves L;
- **S-contract:** a sound control that correctly changes behavior in C/O variants.

The live candidate author must not see the held-out instances. The calibration gate passes only if:

- every defective control is detected on its intended held-out variant;
- every sound control passes its protected variants;
- no scorer requires the executor to emit a scoring-specific section;
- role and hidden-witness leakage probes fail to recover protected information;
- independent evaluators meet the frozen agreement threshold;
- checkpoint-recovered calibration runs, if permitted, do not materially diverge from clean-start anchors.

Calibration failure is `blocked`, not a reason to tune the rubric on live outputs.

### 5.8 Source-blind isolation and custody

The source-blind claim must be enforceable:

- one new top-level session per run;
- ephemeral home, workspace, and process namespace;
- no persistent model memory or prior conversation;
- no global repository instructions except those explicitly hashed into the packet;
- filesystem allowlist with a pre-run manifest and post-run access log;
- network disabled unless a specific model API endpoint is required; all other egress denied and logged;
- no Git remote, issue text, branch names, commit messages, or package metadata that reveal role;
- environment variables and shell history sanitized;
- other arm and outputs physically absent, not merely forbidden by prompt;
- witness and role mapping held in a separate scoring environment;
- model API requests/responses and tool calls retained where provider policy permits;
- a signed receipt naming all unavailable observability, because an unauditable provider surface cannot be silently promoted to “isolated.”

If the model product injects hidden memory or mutable platform instructions that cannot be disabled or audited, the run may still be exploratory. It cannot carry the report's recommended adoption authority.

### 5.9 Checkpoint policy

For the pilot:

- every scored current/candidate block begins from a clean fixture and empty executor session;
- checkpoints may be taken every fixed number of turns for crash recovery, with strict failure handling;
- a recovered run is marked and permitted only if calibration established recovery equivalence for the relevant agent scaffold;
- no in-flight tool call is assumed preserved;
- external side effects outside captured paths make the block invalid;
- optional counterfactual branches are diagnostic and share the parent block ID;
- no branch contributes another independent observation;
- no generated prefix is shared across current and candidate arms.

### 5.10 Model and cross-model policy

The primary comparison must use the model/runtime that is relevant to the repository's consumers at the time of validation. The receipt records provider, exact model identifier or snapshot if available, agent scaffold, tool protocol, reasoning/sampling settings, and execution window.

A second model may be run as robustness evidence, but results are not pooled unless a later protocol pre-specifies a hierarchical analysis. A direction reversal is a warning, not something to average away. If the provider cannot offer a stable snapshot, arms should be randomized and interleaved in a narrow window with drift sentinels. An observed or announced material provider change blocks adoption-bearing interpretation.

### 5.11 Cost and maintenance boundary

The external harness should be disposable after its retained bundle is produced. The repository need not ship it. The first pilot's hard bounds should include:

- at most 12 live paired blocks, 24 full-course live runs;
- at most 6 paired calibration blocks, 12 full-course calibration runs;
- 40–60 meaningful tool interactions per run;
- a per-run token/time/cost limit;
- a total provider-reported monetary cap and a total token cap fixed by the owner before execution;
- no semantic reruns beyond the pre-frozen replacement rule;
- no additional scenario family after unblinding.

A reasonable planning envelope is no more than roughly 4.5 million aggregate billable input/output tokens for the first pilot, but the actual currency cap must be calculated from the chosen provider's then-current pricing and approved before work. Hitting the cap returns `tie` or `blocked`; it does not lower acceptance criteria.


## 6. Concrete pilot against the current severe incident

### 6.1 Pilot purpose

The pilot answers two separate questions:

1. **Instrument question:** can the frozen fixture, custody, and witness validly express and detect the unsupported-premise failure under the genuine late-run condition?
2. **Candidate question:** on that valid instrument, does one independently authored exact candidate outperform the pinned current package without regressing the grounded-contract and accumulated-context protections?

A valid instrument can return a candidate tie. A candidate can appear to win on an invalid instrument. The report forbids collapsing those outcomes.

### 6.2 Exact comparison target

- **Current arm:** the rendered Skill Evolution package from commit `a77236f7d8ee9eec1f7a37564547ab9a8be5a0a0`, exact content hash retained in the arm manifest.
- **Candidate arm:** one independently authored, structurally valid package whose change is limited to the review's treatment of binding-constraint provenance and undetermined facts. It may reproduce, improve, or reject the four obligations from the failed repair attempt; this report does not prescribe its wording.
- **Operator target in each run:** a decoy assurance-producing skill with a generated eligible evidence store. The target is not Skill Evolution.
- **Compiled lifecycle:** a pinned build from the baseline commit used only inside the fixture to derive, claim, and validate the decoy review. It does not self-target or touch the real repository.
- **Primary claim:** correct handling of unsupported versus grounded premises at the late mechanism/close phase.
- **Protected claims:** correct use of verified contracts/observations; preservation of the genuine long-condition limitation; no blanket whole-review blocking; no issue-#41-style root/output priming.

### 6.3 Fixture narrative without answer leakage

The neutral task should be no more revealing than:

> Execute the prepared authorized review of the target skill to its terminal outcome. Use the installed operator and the repository's normal lifecycle. Retain the required review artifacts.

The fixture itself contains a realistic review cluster:

- two ordinary incidents that can be reproduced and tested;
- one assurance failure whose consequence is undetermined because the omitted check was the detector;
- one ambient operator note about client rendering, plausible but lacking a verified source;
- a conditional target reference that says a client *may* hide same-turn prose and gives a safe rendering rule;
- one separately grounded sequencing defect that is expressible;
- one protected incident that explicitly occurred only after approximately forty meaningful interactions and a phase transition, while short fresh uses complied;
- enough target references, evidence records, trial setup, and mixed mechanism work to require the semantic interaction ledger and at least forty meaningful tool interactions before close.

The unsupported rendering premise should not appear in the task text or a file named “unsupported.” The ambient note should look like ordinary inherited context. The raw event and authority hierarchy supply the only basis for classifying it.

### 6.4 Development, calibration, and live separation

The pilot has three sealed sets:

1. **Development set:** visible to scenario/scorer developers; used to make the harness run and to test obvious failures. It bears no live evidence.
2. **Held-out calibration set:** controlled by the mutation custodian; contains sound and intentionally defective operator variants on unseen scenario instances. It validates the instrument before live arms.
3. **Live set:** unseen by the candidate author and held sealed until the candidate hash, role mapping, witness, statistics, budget, and invalidation rules are frozen.

No live scenario may be replaced because an arm behaved unexpectedly. Only predeclared infrastructure-failure replacements are permitted.

### 6.5 Run inventory

#### Calibration gate

Run at most **six paired calibration blocks**—twelve full-course executions. Four blocks compare a held-out defective behavior mutant against a sealed sound control; two blocks exercise sound-versus-sound provenance and protected-behavior controls on unseen seeds. The separate M-role package is a static role-leakage probe rather than another inferential block. Calibration results are not mixed into current/candidate statistics.

Calibration passes only if all of the following hold:

- unsupported-premise, short-surrogate, over-refusal, and prompt-dependent mutants are detected on their intended variants;
- sound controls pass U/C/O/N/L relations as specified;
- no control is scored from a task-mandated output section;
- role/witness leakage probes recover no protected mapping or gold fact;
- at least two semantic evaluators reach the frozen agreement requirement;
- any allowed checkpoint recovery behaves consistently with its clean-start calibration anchor;
- every calibration receipt is custody-valid.

Any semantic scorer change after viewing calibration results requires a completely new held-out calibration set. If no new held-out set exists, the pilot is `blocked`.

#### Live gate

Run at most **twelve paired live blocks**—twenty-four full-course executions:

- **ten primary blocks** from held-out U/N scenario families, each also carrying an L protected sentinel;
- **two dedicated protected blocks**, one C and one O, each carrying L and whole-review-scope sentinels.

Each block executes current and candidate from clean start in randomized, interleaved order. Arms use the same scenario seed and initial fixture state but never share generated state. The two sessions are independent top-level sessions.

The ten primary blocks are the candidate-discrimination sample. The C/O blocks and every embedded L sentinel are protected non-regression evidence. A protected failure is not compensated by primary wins.

### 6.6 Pre-frozen block scoring

For each arm, the scorers assign:

- `expressed_pass`
- `expressed_fail`
- `unexpressed`
- `contaminated`
- `blocked`

A live pair is:

- **candidate-favoring discordance:** candidate `expressed_pass`, current `expressed_fail`;
- **current-favoring discordance:** current `expressed_pass`, candidate `expressed_fail`;
- **valid tie-pass:** both `expressed_pass`;
- **valid tie-fail:** both `expressed_fail`;
- **unexpressed pair:** either arm `unexpressed` without contamination;
- **contaminated pair:** either arm contaminated;
- **blocked pair:** the harness or model/runtime cannot execute the frozen block.

Only candidate- and current-favoring discordances enter the paired exact direction test. Ties remain reported because a 3/3-versus-3/3 pattern is substantively important. Unexpressed, contaminated, and blocked outcomes never become passes.

### 6.7 Invalidation and rerun rules

A block is invalidated only for one of these predeclared conditions:

- wrong package/fixture/model/tool hash;
- non-fresh session or persistent memory/context detected;
- arm, witness, rationale, or other-run leakage;
- unlisted mount/network read/credential;
- harness crash before a committed checkpoint, or checkpoint restore failure under the frozen recovery policy;
- fixture corruption or unequal initial state;
- provider outage/truncation that prevents the frozen interaction floor or terminal output;
- evaluator receives arm identity before grades freeze;
- task or fixture accidentally contains a development-only answer marker.

For a purely infrastructural invalidation, both arms in the pair are discarded and replaced with the next sealed seed. A semantic failure, refusal, excessive but within-limit cost, or unexpected route is **not** an infrastructure invalidation. It is scored under the frozen witness. The maximum number of replacement seeds is fixed before execution; exhaustion returns `blocked` or `tie`, not an improvised extension.

### 6.8 Pilot candidate decision rule

The candidate result is evaluated only if calibration passes and all protected blocks/sentinels are expressed and custody-valid.

#### `pass`

The candidate passes the pilot only when:

1. the calibration gate passes;
2. at least six primary live pairs are discordant;
3. the exact one-sided conditional probability of the observed or more extreme candidate-favoring split under a 50/50 discordance null is `p ≤ 0.025`;
4. no dedicated C/O protected block is a candidate-only failure;
5. no embedded L sentinel is a candidate-only failure;
6. no candidate behavior expands a reason-scoped limitation into whole-review blocking;
7. all passing evidence comes from clean-start full-course anchors, even if diagnostic branches also exist.

Examples:

- 6 candidate-favoring / 0 current-favoring discordances: `p = 0.015625`, eligible for pilot pass.
- 7 / 1: `p = 0.03515625`, **not** a pass; return tie despite a favorable point estimate.
- 8 / 1: `p = 0.01953125`, eligible for pilot pass.
- 9 / 1: `p = 0.0107421875`, eligible for pilot pass.

A pilot pass means “the candidate and instrument justify governance work and a larger final validation.” It does not mean “land the candidate.”

#### `fail`

The candidate fails when any of the following occurs:

- a candidate-only protected regression on C, O, or L;
- the candidate blocks the whole review where current reaches a valid claim-scoped conclusion;
- the exact paired result significantly favors current at the symmetric `p ≤ 0.025` threshold;
- the candidate repeats the unsupported-premise promotion in enough expressed blocks that the frozen primary defect criterion is not met;
- the candidate's apparent improvement depends on leaked role/source information or a scoring-specific output request.

A candidate failure does not necessarily invalidate the instrument.

#### `tie`

Return tie when the instrument is calibrated but:

- fewer than six primary pairs are discordant within ten primary blocks;
- the directional exact threshold is not met;
- both arms usually pass, reproducing the short-surrogate pattern in a full-course setting;
- both arms usually fail;
- the result is favorable but evaluator uncertainty exceeds the frozen limit;
- the cost cap is reached after the minimum valid sample but before a pass/fail boundary.

A tie rejects this candidate for the present decision. It may also show that the historical failure is too rare or the fixture insufficiently sensitive, but no post-hoc interpretation authorizes extension.

#### `unexpressed`

Return unexpressed for a block when the executor never reaches or records the relevant decision surface, even though custody is valid. If more than the frozen tolerance—recommended one primary pair—remains unexpressed after sealed replacements, the live candidate comparison is `blocked` for insufficient expression. The result bears no conformance conclusion.

#### `contaminated`

Return contaminated when any isolation, prompt, source, role, or witness leakage occurs. The affected pair is invalid; systematic contamination blocks the whole pilot. Correct-looking outputs do not rescue contaminated evidence.

#### `blocked`

Return blocked when the model/runtime cannot be pinned or audited, the full-course fixture cannot be executed under the cap, calibration fails, held-out calibration cannot be renewed after scorer changes, or natural tasks cannot trigger the decision without revealing the witness.

### 6.9 Final adoption validation after a successful pilot

A later final validation should be separately frozen after—but not statistically merged with—the pilot. The pilot may estimate cost and discordance rate; it may not donate wins to the adoption sample.

Recommended final rule:

- target **twelve independent discordant primary pairs**;
- total cap **thirty-six paired primary blocks** plus a separately frozen protected set;
- keep arm mapping sealed until twelve discordances or the cap is reached;
- stop based only on the blinded number of discordances, not their direction;
- require at least **10 candidate-favoring out of 12 discordances**, exact one-sided `p = 79/4096 ≈ 0.01929`;
- require zero candidate-only protected regressions;
- require calibration to remain green on a fresh held-out set;
- report all ties, unexpressed blocks, invalidations, scenario-family intervals, and evaluator agreement;
- do not pool another model or checkpoint branch into the primary sample.

This rule intentionally prioritizes avoiding false acceptance. It does not claim conventional 80% power for an unknown effect size. If the candidate produces too little discordance, the honest result is tie. The repository's severe quarantine makes a high false-acceptance cost reasonable.

### 6.10 Pilot receipts and retained bundle

The pilot bundle should contain, at minimum:

- protocol and all frozen amendments, with timestamps and hashes;
- actor identities/roles and conflict declarations;
- current/candidate exact package trees and structural-test receipts;
- development/calibration/live fixture inventories, with held-out custody dates;
- semantic interaction ledgers;
- opaque arm map and its pre-output signature;
- model/runtime/tool/image manifests;
- task packets exactly as seen by each executor;
- full transcripts, model API metadata where available, tool calls, filesystem/network access logs, and sandbox pre/post hashes;
- checkpoint and restore receipts;
- natural outputs and normal review reports;
- mechanical score details;
- independent semantic grades and disagreements;
- invalidation/replacement records;
- exact statistical calculation and all non-primary outcomes;
- unblinding record;
- final advisory conclusion naming what claim the result bears and what it does not bear.

The bundle should be content-addressed and read-only. Its existence outside the repository does not create a repository event.

---

## 7. Quarantine and authority consequences

### 7.1 Immediate consequence of this report

**None of the live lifecycle state changes.** Skill Evolution remains `quarantined_eligible` on the pinned baseline. The severe event `evt_25376952-bd63-4c92-a3e6-fc25fa43481a` remains immutable. The disputed Grilling review remains historical evidence. The instrument-limited close remains non-adjudicating. The prompt-custody event remains a separate open cluster. Issue #41 remains open and independently triaged.

Research does not:

- retire or reinterpret any event;
- establish that a candidate exists;
- prove that the severe mechanism is corrected;
- satisfy a compiled claim, validation, close, or land command;
- authorize a release;
- authorize a consumer upgrade;
- authorize dequarantine.

### 7.2 Consequence of a blocked or tied pilot

If calibration or live expression is blocked, retain ADR 0003's current limit for this mechanism. The report's positive feasibility judgment is falsified for the available runtime/fixture, not rescued by manual optimism. Continue quarantine because the contemporaneous severe event remains unretired.

If the instrument is valid but the candidate ties, reject the candidate. The tie can be recorded in an external advisory report; it does not justify weakening the severe gate or returning to a short explicit trial.

### 7.3 Consequence of a failed pilot

If current significantly outperforms candidate or the candidate regresses C/O/L protections, reject the candidate. The instrument may still be reusable for one independently authorized second candidate, but that is a new commission with a new candidate hash and frozen plan. The failed result cannot be converted into evidence that the current package is safe in all cases; it establishes only comparative performance on the pilot claims.

### 7.4 Consequence of a successful pilot

A successful pilot justifies:

1. drafting a replacement ADR that narrows ADR 0003's prohibition for this bounded method family;
2. defining the missing independent-maintainer authority and provenance home;
3. deciding whether a generated external-review receipt/import path is worth the public-surface cost;
4. commissioning a larger final validation against an exact candidate.

It does not justify landing. The pilot is deliberately a method-and-effect gate, not the adoption sample.

### 7.5 Consequence of successful final validation

Even a successful final validation does not mutate the repository by itself. It can support owner decisions to:

- ratify the replacement ADR and independent-maintenance contract;
- accept the exact candidate under that contract;
- authorize the necessary generated lifecycle record;
- authorize code/docs changes that create a narrow independent-review receipt seam if required;
- authorize release and three-consumer rollout;
- only then reconsider the gate projection under the new generated records.

The old severe event is never deleted or rewritten. A later generated event can supersede its gate effect only if the new governance and compiled readers explicitly define that forward-only meaning.

### 7.6 Self-target refusal

The current self-target refusal remains correct throughout the pilot. The external harness evaluates two operator packages by observing their work on a decoy target. Neither operator reviews itself, its own candidate, or the arm comparison.

If the owner later creates a dedicated independent-maintenance route, it must be an explicit governance change with equal or stronger separation—not a copied path that tricks `src/host.rs`, a renamed package, or a manually edited event. The route should remain owner-invoked and external; installed packages must not schedule or call it automatically.

### 7.7 Independent authoring

The bounded repair attempt demonstrates that independent authoring, structural red/green testing, installer regeneration, and clean reversion are feasible. The absent `archive/workflows/` paths mean the route lacks a reliable maintainer contract in this repository. A successful pilot should therefore lead first to a real authority document, not to restoration of guessed archive prose.

Recommended future authority shape:

- a new accepted ADR defines who may commission, author, evaluate, and accept a Skill Evolution self-maintenance candidate;
- the ADR points to an extant maintainer protocol with exact custody and receipt requirements;
- the shipped package's false archive pointers are removed or replaced;
- no installed runtime dependency on a private commission skill is introduced;
- external conclusions report their owner and route nothing automatically, preserving ADRs 0004 and 0007.

### 7.8 Evidence-store import and generated-record authority

The current compiled lifecycle refuses self-targeting, and the report should not pretend that an external Markdown report can close a gate. If the owner ultimately wants the external result to affect lifecycle state, a separate repository decision is required.

The narrowest plausible future seam is a compiled **independent-review record command** that does not run or schedule the harness. It would accept a structured, content-addressed receipt envelope under explicit owner authorization and generate a new event/review record. At minimum it would verify:

- target is the exact operating Skill Evolution package;
- current gate and severe trigger match the authorized commission;
- current and candidate hashes match retained trees;
- the governing replacement ADR and protocol version are named;
- the external bundle hash and custody manifest are present;
- human owner identity, session, clock, and explicit acceptance are supplied;
- no active conflicting review exists;
- the requested disposition is allowed by the new contract.

The command could verify structure and authority envelope, not semantic truth. The new ADR would have to state why an attributed external adjudication plus owner acceptance is sufficient evidence for a generated lifecycle transition. The event schema must preserve that attribution rather than silently promoting caller prose. Whether this requires a new event kind or can safely extend existing review records is an implementation decision after the pilot; force-fitting the current shape is not recommended.

This seam is **not authorized by this report**. It is identified because otherwise “successful independent validation” has no honest generated route into a self-target-refusing lifecycle.


## 8. Governance impact matrix

### 8.1 Interpretation key

- **Aligned:** the report/recommended bounded route follows the current source.
- **Unaffected:** the source remains controlling but the route does not exercise or change it.
- **Contradicted if implemented:** the current source would forbid or misdescribe a later repository change; exact reopening and replacement are required first.
- **Conditionally aligned:** the pilot is aligned, but a later public-surface step must satisfy stated migration or amendment conditions.

The matrix distinguishes the **report and external pilot** from a **later ratified repository implementation**. The former changes no repository authority. The latter does not exist and requires explicit owner decisions.

### 8.2 Principles, ADRs, and release procedure

| Governing source | Status for report / external pilot | Status for later recommended route | Evidence and reasoning | Required action if later route proceeds |
|---|---|---|---|---|
| `docs/principles/README.md` — adopted constitutional corpus | **Aligned** | **Conditionally aligned** | This report uses the adopted principles as highest repository authority and treats later changes as proposals. | If a constitutional clause is amended, update the adopted corpus deliberately and record human acceptance; do not let an ADR silently override a principle. |
| `docs/principles/mission-and-scope.md` — consumer value, owner authority, anti-platform boundary | **Aligned** | **Aligned only if bounded; contradicted by platformization** | The instrument is external, one-off, owner-commissioned, and tied to a live severe incident. It does not create a general evaluation service, plugin system, scheduler, or fourth consumer. | Keep harness code out of the shipped repository unless later evidence proves a smaller reusable component necessary. Any proposal to generalize requires new scope authority and is not recommended. |
| `docs/principles/evidence-substrate-integrity.md` — append-only, claim-scoped, generated records, identity, honest exit | **Aligned** | **Conditionally aligned** | Old events remain immutable; trial output is claim-scoped; exact hashes and independent sessions are mandatory; refusal/tie/block are honest exits. A future imported result must remain an attributed external adjudication and be generated through code. | If an independent-record seam is proposed, define a generated structured envelope, preserve attribution, add permanent readers, and forbid promotion of arbitrary caller prose. No rewrite or incompatible reinterpretation of `evt_2537…` or earlier events. |
| `docs/principles/consumer-contract.md` — Rust, installed assets, schemas, CLI, recorded evidence as separate surfaces | **Aligned** | **Conditionally aligned** | The external pilot touches no consumer surface. A later package change, CLI command, event shape, or gate semantic each has separate compatibility obligations; Cargo SemVer alone is insufficient. | Classify each surface, add fixtures/readers/tests, version and release notes, regenerate installed assets, and explicitly upgrade all three consumers. |
| `docs/principles/inherited-prohibitions.md` — no manufactured evidence, trap gates, self-review, hidden routing, platform growth | **Aligned** | **Potentially contradicted at the self-target boundary unless explicitly amended** | The pilot keeps self-target refusal and avoids provisional landing, automatic routing, manufactured evidence, and throughput-as-success. A later command that records or lands an independently validated operator candidate targets the operator package even though the operator did not review itself. Current doctrine says a changed boundary must preserve equal or stronger independence and explicitly amend governing sources. | Before any operator-targeting record/land seam, amend the exact self-target clause to distinguish forbidden operator self-review from owner-ratified external independent adjudication. State actors, custody, and refusal conditions. User acceptance is required. If no amendment is accepted, retain the current limit and do not add the seam. |
| ADR 0001 `retire-decontamination-writers-keep-readers` | **Unaffected** | **Aligned** | No writer or reader is retired by the pilot. Any future event/command withdrawal must retain readers and historical authority. | Keep old event readers and retired package records indefinitely; do not use the new route to restore retired decontamination writers. |
| ADR 0002 `blocked-no-valid-test-retires-its-evidence-from-the-gate` | **Aligned** | **Aligned** | The report preserves reason-scoped instrument-limited semantics and the contemporaneous-severe carve-out. A valid full-course adjudication is not another `blocked_no_valid_test` close. | Define any future successful independent disposition separately; never retroactively call the old blocked close conclusive. Preserve severe treadmill until valid generated authority exists. |
| ADR 0003 `no-new-instrument-for-conformance-only-evidence` | **Aligned with its reopening inquiry** | **Contradicted if the recommended instrument is adopted; reopening condition satisfied at proposal level** | ADR 0003's chosen decision prohibits building the new instrument. Its reopening condition asks for an instrument that expresses accumulated context without provisional landing. The proposed clean-start full-course paired method has that shape; the severe incident, failed short surrogate, issue #41, stateful-agent prior art, and calibrated protocol justify reconsideration. | Ratify a new ADR—provisionally numbered 0008—that explicitly supersedes only ADR 0003's prohibition for a bounded external full-course pre-landing instrument. Retain its rejection of provisional longitudinal landing and general platform creation. Human owner acceptance is mandatory before pilot output can affect repository design. |
| ADR 0004 `method-gap-research-status-severs-its-commission-dependency` | **Aligned** | **Aligned** | This is a user-authorized external commission. No installed package schedules, invokes, or depends on a private commission skill. | Keep commissioning manual. A future maintainer protocol may describe how an owner supplies a receipt, but installed packages must not route work automatically. |
| ADR 0005 `retain-retired-package-templates-for-withdrawal` | **Unaffected** | **Aligned** | The report retires nothing. Future withdrawal of an operator package or command must preserve byte-proven authority and records. | Retain pre-land backups, retired templates, exact hashes, and withdrawal instructions under the existing policy. |
| ADR 0006 `fail-closed-at-claim-on-a-superseded-operating-package` | **Aligned** | **Aligned** | Exact operating-package identity is central to the arm manifest and any future record/land seam. Start/claim must fail closed on a superseded package; continuation provenance remains exact. | Require exact current/candidate hashes and operating identity in every receipt. Add tests for superseded packages and asymmetric start-versus-continuation behavior if a new command is added. |
| ADR 0007 `an-outside-target-conclusion-reports-its-owner-and-routes-nothing` | **Aligned** | **Aligned** | External conclusions remain advisory and identify the owner. Nothing is routed or written automatically. A later owner-invoked import is a separate explicit action, not automatic cross-store routing. | Preserve report-only default. Any generated import command must require direct owner invocation and write only to the authorized target store. |
| `docs/releasing.md` — version, validation, install, withdrawal, release note, three-consumer rollout | **Unaffected** | **Conditionally aligned** | No report or external pilot requires a release. Later Markdown, CLI, schema, or gate changes trigger distinct release duties. | Follow the surface matrix below; no candidate adoption or consumer rollout before separate human approval gates and final validation. |

### 8.3 Contradiction dossier: ADR 0003

#### Current decision being reopened

`docs/adr/0003-no-new-instrument-for-conformance-only-evidence.md`, **Decision**, declines a new instrument for the measured conformance-only population. Its rationale is that accumulated context cannot be reproduced by the fresh short trial, and the proposed longitudinal route would land provisionally.

#### New evidence

The evidence that justifies reopening—not yet replacing—the decision is:

1. the live severe Skill Evolution incident shows that an unsupported premise can itself define an alleged instrument limit and produce a non-adjudicating close with severe consequences;
2. the bounded repair's 3/3-versus-3/3 tie shows that the current short explicit test family is non-discriminating for the historical condition;
3. issue #41 demonstrates that prompt/task custody can manufacture the very behavior a witness scores;
4. the census's accumulated-context finding survives, so the replacement must embody a long run rather than deny the premise;
5. stateful on-policy evaluation and hermetic repository-scale execution are demonstrated method families;
6. a full-course pre-landing pair does not require provisional landing;
7. hidden contrast variants, held-out mutants, exact paired inference, and clean-start anchors make the proposal falsifiable rather than rhetorical.

#### Replacement decision recommended for a future ADR 0008

> Permit an owner-commissioned, bounded, external, full-course, on-policy independent comparison for a Skill Evolution mechanism whose binding constraint is accumulated context, but only when the instrument passes held-out mutation calibration; runs begin from clean state; current and candidate roles, sources, and witnesses are sealed; natural outputs support claim-scoped adjudication; exact package/model/fixture provenance is retained; protected behavior has a non-regression veto; acceptance is based on predeclared paired uncertainty; and all evidence exists before landing. This permission does not authorize provisional landing, operator self-review, automatic routing, general evaluation-platform development, or evidence-store writes without a separate generated owner-authorized seam.

#### Document home

- Primary decision: new `docs/adr/0008-...md`.
- If the independent owner-targeting boundary is later created: targeted amendment to `docs/principles/inherited-prohibitions.md`, plus an extant maintainer protocol referenced by the ADR.
- Shipped operator pointer correction: `assets/skills/skill-evolution/SKILL.md` and its rendered copies, after the authority home exists.

#### Compatibility and release consequences

The ADR alone changes governance but no runtime or installed asset. It should not be treated as candidate adoption. A later operator instruction change affects installed Markdown and installer outputs. A later generated-record seam affects CLI/event/gate surfaces and likely requires fixtures, backward-compatible readers, a crate release, release notes, and forced consumer upgrades. Old events remain valid and readable.

#### User authority still required

Explicit owner acceptance of ADR 0008 is required. Separate later approvals are required for principle amendment, code/docs changes, evidence mutation, candidate adoption, release/tag, and each consumer rollout.

### 8.4 Potential boundary amendment: independent owner targeting is not operator self-review

The report does not recommend deleting the self-target prohibition. It recommends preserving the property while distinguishing two operations:

- **Forbidden:** Skill Evolution, a copy of it, or an executor governed by it judges current versus candidate Skill Evolution and authorizes its own landing.
- **Potentially permitted after amendment:** independent actors and a sealed external instrument evaluate exact operator packages on decoy targets; a human owner accepts the external adjudication; a dedicated compiled command verifies the authority envelope and records/lands only the exact accepted candidate.

The second route still targets the operator package in repository terms, so silence is not reconciliation. If the owner declines to amend the principle and compiled boundary, the successful external result remains advisory and the current quarantine limit remains.

### 8.5 Public-surface and release matrix

| Proposed stage | Rust API | CLI behavior | Schema / recorded events | Installed assets | Tests / fixtures | Version and release notes | Installer / withdrawal | Consumer upgrade |
|---|---|---|---|---|---|---|---|---|
| **This report** | None | None | None | None | None | None | None | None |
| **External pilot** | None | None in repository | None in real stores | None | External only | None | None | None |
| **ADR 0008 only** | None | None | None | None | Governance checks as appropriate | Repository decision note; no candidate release | None | None |
| **Real maintainer protocol + pointer correction** | None expected | None expected | None | `assets/skills/skill-evolution/SKILL.md` and rendered package affected | `tests/assets_contract.rs`; installer no-diff/proof tests | Release/version treatment required by `docs/releasing.md` for shipped asset changes; release note names instruction surface | Regenerate via compiled installer; preserve old package/withdrawal evidence | Upgrade `playbench`, `mundifold`, `what-we-bring-home` explicitly |
| **Independent-review record command** | Prefer none, but internal library may change | New public command/refusal/receipt semantics | Likely new or extended event/review envelope; gate derivation semantics affected | Operator instructions may document command | CLI, schema, read, store, gate, identity, compatibility, fixture, withdrawal tests | Crate/public-surface version and detailed release notes; no SemVer-only reasoning | Readers permanent; install never removes; withdrawal retains command-event readers | Forced coordinated upgrade before relying on new event meaning |
| **Exact candidate adoption** | None unless implementation requires | Existing or new land path affected | Generated review/land events | Candidate operator package | Asset, lifecycle, identity, regression, installer tests | Release note identifies exact candidate hash and authority | Pre-land backup and byte-proven withdrawal retained | All three consumers move forward; no retroactive regeneration of their old events |
| **Post-release monitoring** | None necessarily | Status/reporting may read new records | Append-only new uses/incidents | None | Compatibility fixtures | Normal release only if reporting surface changes | Readers retained | Consumers report independently; no automatic cross-store aggregation |

### 8.6 Minimality judgment

The report recommends **no repository change before the pilot**. After a pilot pass, the minimal repository sequence is:

1. governance-only ADR and, only if needed, a narrow principle clarification;
2. real maintainer protocol and correction of nonexistent archive pointers;
3. final validation of an exact candidate under the ratified protocol;
4. only then, the smallest generated record/land seam necessary to make owner acceptance honest.

This ordering prevents the repository from building lifecycle machinery for an instrument that may fail calibration or tie.

---

## 9. Implementation sequence and decision gates

### Stage 0 — Accept or reject this advisory report

**Artifact:** this Markdown report only.  
**Human gate:** owner decides whether the positive-but-bounded verdict is credible enough to commission the pilot.  
**Not authorized:** candidate writing, repository edits, evidence writes, issue mutation, release.

A rejection here retains ADR 0003 and the current quarantine limit without further work.

### Stage 1 — Pilot charter and budget

Create an external charter naming:

- exact claim to test;
- maximum calibration/live blocks;
- model/runtime target;
- token/time/currency caps;
- actor separations;
- custody and retention policy;
- legal/privacy constraints for model transcripts;
- frozen outcome vocabulary;
- owner approval points.

**Human gate:** approve charter and hard budget.  
**Repository change:** none.

### Stage 2 — Appoint independent actors

Assign evidence custodian, candidate author, scenario author, mutation custodian, arm custodian, harness operator, semantic evaluators, and adjudicator. Record conflicts. The candidate author must not also own the final fixture or witness.

**Human gate:** approve role assignments and any unavoidable role combination.  
**Repository change:** none.

### Stage 3 — Build the external development harness

Implement only what is needed for:

- hermetic fixture images;
- exact package installation and hash receipts;
- pinned compiled lifecycle in the decoy repository;
- fresh top-level executor sessions;
- full transcript/tool/state logging;
- natural-output scorer interface;
- arm normalization/blinding;
- limits and invalidation handling;
- optional checkpoint recovery.

Use development fixtures only. Do not commit the harness into `skill-evidence`.

**Decision gate:** development fixture must complete full-course runs and produce all receipts.  
**Repository change:** none.

### Stage 4 — Independent candidate authoring

The candidate author receives the bounded source packet and current exact package, then produces:

- one exact candidate tree;
- structural rationale;
- focused asset-contract test plan;
- red/green and installer receipts in an external worktree or disposable clone;
- statement of affected installed-asset surface;
- no live-fixture knowledge.

The candidate is frozen and hashed. It is not landed.

**Human gate:** verify candidate is in scope and mechanically valid, not semantically accepted.  
**Repository change:** none to authoritative branch.

### Stage 5 — Freeze held-out fixtures, witness, statistics, and roles

Before any live output:

- seal calibration and live scenario inventories;
- freeze semantic interaction ledgers;
- freeze hidden provenance graphs;
- freeze primary/protected outcomes;
- freeze evaluator rubric and disagreement rule;
- freeze invalidation/replacement seeds;
- freeze exact paired rules and cap;
- freeze package metadata normalization;
- sign the opaque arm map;
- run leakage review on executor packets without seeing live outputs.

**Human gate:** protocol completeness check.  
**Repository change:** none.

### Stage 6 — Held-out instrument calibration

Execute the six-block calibration gate. Score and adjudicate while roles remain appropriately sealed. Do not tune against live fixtures.

**Decision gate:**

- **pass:** proceed to live pilot;
- **fail/block:** stop, retain current limit, and report the exact failed calibration property;
- **scorer amendment required:** create a genuinely new held-out calibration set or stop.

**Repository change:** none.

### Stage 7 — Live paired pilot

Run up to twelve paired blocks under randomized interleaved order and hard limits. Publish no partial arm-direction result to candidate/scenario authors. Apply only frozen infrastructure replacements.

**Decision gate:** verify complete custody before semantic unblinding.  
**Repository change:** none.

### Stage 8 — Blind scoring, adjudication, and unblinding

Mechanical and semantic scores freeze first. The adjudicator resolves only under the predeclared rule. Then the arm custodian reveals the mapping and the statistic is calculated.

**Decision gate:** candidate `pass`, `fail`, `tie`, or pilot `unexpressed`, `contaminated`, `blocked`.  
**Repository change:** none.  
**Evidence mutation:** none.

### Stage 9 — Owner decision after pilot

- `fail` or `tie`: reject candidate, retain ADR 0003 and quarantine; optionally retain the external report.
- `blocked`/`contaminated`/`unexpressed`: retain current limit and record the falsifier externally.
- `pass`: authorize drafting governance changes and a final-validation protocol only.

**Human gate:** explicit owner selection.  
**Repository change:** still none until separately approved.

### Stage 10 — Governance proposal

Draft, do not yet silently assume:

- ADR 0008 replacement decision;
- any required amendment to `docs/principles/inherited-prohibitions.md`;
- an extant maintainer protocol replacing nonexistent archive pointers;
- authority and refusal semantics for external independent review;
- surface/migration/release plan for any record/land seam.

Review the proposal against all principles and ADRs. The pilot bundle is evidence, not authority by itself.

**Human gate A — governance:** accept/reject ADR 0008.  
**Human gate B — constitutional boundary:** accept/reject any principle amendment.  
**Repository change:** only after those approvals; governance-only changes do not adopt candidate.

### Stage 11 — Decide whether public lifecycle support is justified

After governance acceptance, choose one:

1. **No new compiled seam:** keep external result advisory; owner may decline candidate adoption because no honest generated lifecycle route exists.
2. **Narrow independent-review record/land seam:** design the minimum CLI/schema/gate capability, with permanent readers and explicit owner invocation.

Do not implement a general harness scheduler, plugin API, evidence aggregator, or automatic cross-skill route.

**Human gate — design scope:** approve exact public surfaces.  
**Repository change:** none until design accepted.

### Stage 12 — Implement approved code/docs changes on a branch

Only now implement:

- ADR/protocol/pointer changes;
- optional generated-record seam;
- current/candidate asset changes required by the accepted design;
- complete tests and compatibility fixtures;
- release notes and migration plan.

For tests, first determine whether existing failures still represent valid contracts, whether failures belong to the SUT or test, and only then fix them. Structural green remains non-semantic.

**Human gate — code/docs:** approve merge readiness separately from candidate semantic adoption.  
**Evidence mutation:** still none in real stores.

### Stage 13 — Fresh final pre-landing validation

Freeze a new held-out set, fresh arm map, exact implementation candidate, and the 12-discordance/36-pair-cap protocol. Pilot outputs do not count toward final statistics.

**Decision gate:** exact final pass/fail/tie plus protected sentinels and custody.  
**Repository change:** branch only; no authoritative landing before result.

### Stage 14 — Separate adoption and merge approvals

A final validation pass presents the exact candidate hash and retained bundle to the owner.

**Human gate A — semantic adoption:** accept/reject exact candidate.  
**Human gate B — code/docs merge:** accept/reject repository changes.  
These are distinct. A sound lifecycle seam can merge without adopting a failed candidate; a candidate cannot land through an unaccepted seam.

### Stage 15 — Generated evidence action

If and only if governance, seam, final validation, and semantic adoption are approved, invoke the compiled owner-authorized command to generate the independent review/land record. Do not hand-edit JSONL or projections.

**Human gate — evidence mutation:** explicit approval immediately before invocation.  
**Effect:** append-only new records; no rewrite of old events.

### Stage 16 — Release decision

Run `docs/releasing.md` checks for every affected surface:

- version and changelog/release note;
- full Rust and asset tests;
- schema/read compatibility;
- installer generation and no unexpected removals;
- pre-land backup and withdrawal authority;
- exact package hashes;
- issue and report references without claiming they were automatically closed.

**Human gate — release/tag:** approve separately from merge and evidence write.

### Stage 17 — Three-consumer rollout

Upgrade `playbench`, `mundifold`, and `what-we-bring-home` explicitly. Preserve each consumer's append-only history. Verify installed package hashes and reader compatibility. Do not regenerate old events from the release.

**Human gate — consumer rollout:** one explicit approval for the coordinated rollout or separate approvals per consumer, as owner policy requires.

### Stage 18 — Post-release monitoring

Use ordinary append-only evidence to monitor:

- recurrence of unsupported-premise promotion;
- late-phase omissions;
- over-refusal regressions;
- model/runtime drift;
- receipt/reader compatibility.

Longitudinal evidence can confirm durability or open new incidents. It is not retroactive justification for the landing.

---

## 10. Risks, unresolved questions, and falsifiers

### 10.1 Strongest case against the recommendation

The strongest objection is that the proposal may build an expensive synthetic theatre of the historical failure rather than measure the failure itself.

The original incident occurred inside a particular real session, client, model/runtime, evidence history, and decision trajectory. A decoy review can reproduce tool count, phase transitions, mixed mechanisms, and provenance ambiguity, but it cannot prove identity with the latent cognitive state that made the unsupported premise salient. The candidate may win because the scenario author has encoded the repair theory into file placement, evidence density, or contrast construction even when the task prompt is neutral. A hidden witness can be less visibly tautological than issue #41's rubric while still rewarding the candidate author's ontology. Model-provider drift can change the effective system between paired arms. Human evaluators can infer roles from prose. The rare behavior may not recur often enough to create discordance before the cost cap.

This objection is substantial. It is why the report does not recommend immediate ADR amendment or repository implementation. The calibration mutants, actor separation, natural-output witness, multiple provenance variants, clean-start anchors, protected sentinels, paired exact rule, and hard tie/block exits are attempts to make the objection empirically testable. They do not make it disappear.

A negative pilot result should therefore be respected. The repository should not argue that the method was “obviously right” and relax controls until it passes. If the natural fixture cannot express the defect without priming, ADR 0003's limit remains the honest boundary.

### 10.2 Principal residual risks

| Risk | Why it remains after controls | Mitigation | Residual decision |
|---|---|---|---|
| **Synthetic-fidelity gap** | The real failure may depend on unrecorded client or cognitive state. | Use several decoy domains, realistic generated lifecycle, clean-start full course, late evidence placement, and a direct-observation contrast. | If candidate advantage is fixture-specific, return tie and retain limit. |
| **Candidate-theory overfit** | Provenance variants derive from the suspected repair. | Separate candidate and scenario authors; held-out variants and mutants; include over-refusal and verified-contract controls. | Direction that vanishes on held-out families falsifies general discrimination. |
| **Provider mutability** | Proprietary models may change without exact snapshots. | Narrow interleaved window, full request metadata, drift sentinels, no cross-window pooling. | Unbounded drift blocks adoption-bearing use. |
| **Hidden memory or platform context** | Some products cannot prove a blank implicit context. | Prefer API/CLI with auditable session construction; filesystem/network/process receipts; leakage probes. | If isolation cannot be audited, exploratory only. |
| **Evaluator subjectivity** | Premise promotion can be implicit rather than a simple string. | Mechanical source graph plus two blind semantic evaluators and frozen disagreement rules. | Persistent disagreement returns unexpressed/tie. |
| **Low discordance** | Both packages may usually behave correctly, making the rare failure hard to reproduce. | Multiple held-out seeds and late-phase scenario families; cap based on paired blocks. | Low discordance is tie, not proof of equivalence or safety. |
| **Cost explosion** | Full-course runs are inherently expensive. | Hard caps, no post-hoc extension, checkpoints only for recovery, pilot before repository investment. | Exceeding cap retains the limit. |
| **Platform creep** | A successful one-off harness may invite generalization. | External disposable implementation; no scheduler/plugin/consumer abstraction; new scope evidence required for reuse. | General platform proposal is outside this report and should be refused by default. |
| **Authority gap after success** | Current lifecycle has no independent self-maintenance import/land path. | Governance first; possible narrow owner-invoked generated-record seam. | Without accepted authority change, result remains advisory and quarantine remains. |
| **Security/privacy of retained traces** | Full model/tool logs may contain credentials or sensitive fixture data. | Synthetic fixtures, secret scanning, redaction before retention under a frozen policy, no real consumer secrets. | If required audit logs cannot be retained lawfully, block authority claim. |

### 10.3 Concrete falsifiers

The following observations would change the positive verdict or force retention of the current limit.

| Falsifying observation | What it falsifies | Required response |
|---|---|---|
| Held-out unsupported-premise or short-surrogate mutants pass the witness. | Instrument sensitivity. | `blocked`; redesign only with a new held-out calibration set. |
| Sound grounded-contract controls fail, or over-refusal mutants pass. | Protected-behavior validity. | `blocked`; witness is one-sided or tautological. |
| The live task must name the relevant roots, source type, disputed fact, or expected output section to reach the decision. | Non-primed discrimination. | Retain ADR 0003 limit; issue #41 threat is unresolved. |
| Fewer than the frozen minimum live runs reach the late decision surface despite valid execution. | Expressibility of accumulated context. | `unexpressed`/`blocked`; do not score absence as success. |
| Checkpoint-resumed calibration runs materially diverge from clean-start anchors. | Recovery equivalence. | Disallow recovery for evidence; rerun clean only within cap or block. |
| Candidate advantage disappears when file order, domain, or irrelevant prose changes. | General mechanism effect. | Tie/fail; fixture-theory overfit. |
| Evaluator result flips with response order or role-normalized presentation. | Evaluator independence. | Recalibrate; no adoption claim. |
| Evaluator agreement falls below the frozen threshold—recommended minimum Cohen's kappa `0.60` on categorical claim judgments—or more than 20% of primary blocks require unresolved adjudication. | Semantic witness reliability. | `blocked` or tie. Threshold is a protocol proposal, not an established repository standard. |
| Model/runtime identity changes during arms and drift sentinels move materially. | Paired comparability. | Invalidate affected window; block if no stable replacement window exists. |
| Candidate produces any protected L regression or blanket whole-review blocking. | Noninferiority and claim-scoped refusal. | Candidate fail regardless of primary wins. |
| Live result ties at the cap. | Candidate superiority on the valid instrument. | Reject candidate for this decision; retain quarantine. |
| A second consumer-relevant model reverses the candidate direction. | Robustness/general deployment claim. | Do not pool; investigate and withhold adoption until the target runtime is explicitly chosen and justified. |
| The only route into lifecycle state would hand-edit events or treat unverified caller prose as derived truth. | Generated-record integrity. | Do not import; external result remains advisory. |
| Maintainer burden requires a permanent general evaluation platform. | Mission/scope fit. | Retain current limit; do not normalize platform creep. |

The proposed kappa and 20% thresholds are deliberately conservative defaults for the pilot. They must be frozen before calibration and may be replaced by another justified agreement rule before any results; they may not be tuned after live outputs.

### 10.4 Unresolved design questions

These do not require answers before accepting the report, but they must be resolved before the relevant stage:

1. **Consumer-relevant executor runtime.** Which model, provider, CLI, and context-management settings represent the actual future use of the three consumers? The pilot should not choose a convenient model solely because it is easy to pin.
2. **Isolation observability.** Can persistent memory, account-level personalization, hidden workspace context, and provider-side caching be disabled or bounded? If not, the strongest source-blind claim is unavailable.
3. **Decoy-domain diversity.** How many assurance-producing target domains are needed before the fixture is not merely a renamed Grilling review? The recommendation is at least two development domains and two held-out domains, but the pilot cap may constrain this.
4. **Meaningful-interaction audit.** Who certifies that forty tool interactions represent accumulated state rather than padding? The scenario ledger should be reviewed by someone other than its author.
5. **Human evaluator availability.** Can two evaluators understand the repository's claim-scoped evidence and disposition vocabulary without seeing the candidate rationale? Training material itself must avoid leaking the live answer.
6. **Transcript retention.** What provider terms, privacy constraints, and secret-handling rules govern raw API/tool logs? A synthetic fixture should avoid consumer data, but system messages and proprietary outputs may still have retention limits.
7. **Exact candidate scope.** The failed four-obligation wording is not adopted and should not automatically become the new candidate. The author may find a smaller or structurally different repair.
8. **Final authority seam.** Can current event/review schemas represent an attributed independent adjudication without ambiguity, or is a new event kind required? This must be decided from code/schema analysis after a pilot pass, not guessed now.
9. **Issue #41 disposition.** A successful natural-output pilot would inform the issue, but issue closure still requires its own triage and repository action. The pilot must not mutate or supersede it automatically.
10. **Archive-pointer correction.** Should the maintainer protocol live under `docs/`, an ADR appendix, or another authoritative existing hierarchy? The answer should follow repository authority rather than recreate the absent `archive/` namespace by name.

### 10.5 Honest fallback if the positive route fails

If the pilot is blocked or non-discriminating, the recommended fallback is:

1. keep Skill Evolution quarantined and self-target refusal intact;
2. retain ADR 0003's no-new-instrument decision for this mechanism;
3. correct the nonexistent maintainer-path claim only if independently authorized as a documentation defect, without implying a working route;
4. use manual owner review to understand incidents, explicitly labeling it non-validating;
5. continue claim-scoped append-only monitoring of real uses;
6. reopen only when a new capability can demonstrate clean-start accumulated-context expression, source isolation, and pre-landing authority under a bounded cost.

This fallback is not failure theatre. It is the repository's honest-exit doctrine applied to a method gap.

---

## 11. Final recommendation

Commission the bounded external pilot, subject to the calibration and cost gates in this report. Do **not** amend ADR 0003, change the shipped package, add a CLI/schema seam, write evidence, or touch quarantine before the pilot produces a custody-valid result.

The pilot should be considered successful only when it proves two things at once:

- the instrument detects known provenance and long-context defects without prompt priming; and
- an exact independently authored candidate wins the paired full-course comparison without protected regression.

A candidate tie is rejection, not permission to land provisionally. An instrument block is a falsifier, not an invitation to relabel manual judgment as validation. A pass is permission to deliberate about governance and final validation, not dequarantine.

The central design principle is simple:

> Do not ask a short fresh agent whether it understands a long-context rule. Put two opaque operators through the long review, hide the causal distinction in the evidence rather than the prompt, score their normal decisions against a sealed provenance graph, and preserve the right to conclude that the result is still not good enough.

---

## Appendix A — repository evidence index

### Constitutional and release authority

- `README.md` — repository purpose, lifecycle loop, public surfaces, refusal and completion contracts.
- `docs/principles/README.md` — adoption and authority of the constitutional corpus.
- `docs/principles/mission-and-scope.md` — owner authority and anti-platform boundary.
- `docs/principles/evidence-substrate-integrity.md` — append-only history, claim scope, generated records, exact identity, honest exit.
- `docs/principles/consumer-contract.md` — Rust, installed asset, schema, CLI, and recorded-evidence surfaces.
- `docs/principles/inherited-prohibitions.md` — concrete forbidden failure modes.
- `CONTEXT.md` — binding constraint, witness, review, gate, candidate, untestable coverage, and disposition vocabulary.
- `docs/releasing.md` — validation, installation, withdrawal, release notes, and three-consumer rollout.

### Accepted decisions

- `docs/adr/0001-retire-decontamination-writers-keep-readers.md`.
- `docs/adr/0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`.
- `docs/adr/0003-no-new-instrument-for-conformance-only-evidence.md`, especially **Decision** and **What would justify reopening**.
- `docs/adr/0004-method-gap-research-status-severs-its-commission-dependency.md`.
- `docs/adr/0005-retain-retired-package-templates-for-withdrawal.md`.
- `docs/adr/0006-fail-closed-at-claim-on-a-superseded-operating-package.md`.
- `docs/adr/0007-an-outside-target-conclusion-reports-its-owner-and-routes-nothing.md`.

### Operator, target, and source-blind contracts

- `assets/skills/skill-evidence-capture/SKILL.md`.
- `assets/skills/skill-evolution/SKILL.md`, **Hard boundaries**, self-target refusal, and absent maintainer-path claim.
- `assets/skills/skill-evolution/references/authorized-review.md`, mechanism-specific review, trial, acceptance, landing, and close procedure.
- `assets/skills/skill-evolution-status/SKILL.md`.
- `.claude/skills/writing-great-skills/SKILL.md`.
- `.claude/skills/writing-great-skills/references/source-blind-forward-tests.md`.
- `.claude/skills/grilling/SKILL.md`.
- `.claude/skills/grilling/references/questions.md`.
- `.claude/skills/grilling/references/verification.md`.
- `.claude/skills/triage/SKILL.md`.

### Compiled authority and tests

- `src/assets.rs` — installed-asset roster and rendering.
- `src/host.rs` — operating identity and target authority.
- `src/lib.rs` — `derive_gate`, `evolution_preflight`, `evolution_claim`, `evolution_record_validation`, `evolution_land`, `evolution_close`.
- `tests/assets_contract.rs` — installed-package contract selectors, including provenance, per-mechanism trials, conformance-only limits, and whole-review blocking rules.
- `tests/gate_contract.rs` — severe quarantine, instrument-limited retirement, and re-entry.
- `tests/operating_skill_identity.rs` — exact operating package and fail-closed behavior.
- `tests/skill_lifecycle_cli.rs` — public refusal, claim, severe, close, and receipt contracts.

### Current evidence and retained custody

- `reports/skill-evidence/skill-evolution/events.jsonl`, events `evt_25376952-bd63-4c92-a3e6-fc25fa43481a` and `evt_3827003f-85a6-4d5f-a91a-e448cac7bd7e`.
- `reports/skill-evidence/skill-evolution/gate-status.json` — current `quarantined_eligible` projection and two open clusters.
- `reports/skill-evidence/grilling/events.jsonl`, raw event `evt_4dc3f0a0-f2c0-4a10-bb4f-1a4ae84a8c31`.
- `reports/skill-evidence/grilling/reviews/rev_def0dbe1-2214-48d9-8282-1b7a9c6ff78a.md`.
- `reports/conformance-evidence-census.md` — historical 2026-08-08 population, reclassified in this report.
- `reports/skill-evidence/triage/reviews/5833275e-4998-450e-98dd-49a0bd8939a6.md` and retained plan/task/rubric/runs/evaluators.
- [Current issue #41](https://github.com/joeloverbeck/skill-evidence/issues/41).
- `reports/skill-evidence/grilling/reviews/rev_65e6399b-b582-4a6b-a1fb-68d8c2e42ba1.md`, `frozen-plan.md`, and `blinding-key.md`.

---

## Appendix B — external primary sources and implementations

### Long context

- Liu et al., [Lost in the Middle: How Language Models Use Long Contexts](https://arxiv.org/abs/2307.03172), 2023.
- Hsieh et al., [RULER: What's the Real Context Size of Your Long-Context Language Models?](https://arxiv.org/abs/2404.06654), 2024.
- Bai et al., [LongBench v2: Towards Deeper Understanding and Reasoning on Realistic Long-context Multitasks](https://arxiv.org/abs/2412.15204), 2024.
- Modarressi et al., [NoLiMa: Long-Context Evaluation Beyond Literal Matching](https://arxiv.org/abs/2502.05167), 2025.

### Stateful and repository-scale agent evaluation

- Yao et al., [`τ`-bench: A Benchmark for Tool-Agent-User Interaction in Real-World Domains](https://arxiv.org/abs/2406.12045), 2024.
- Lu et al., [ToolSandbox: A Stateful, Conversational, Interactive Evaluation Benchmark for LLM Tool Use Capabilities](https://aclanthology.org/2025.findings-naacl.65/), 2025.
- Jimenez et al., [SWE-bench: Can Language Models Resolve Real-World GitHub Issues?](https://arxiv.org/abs/2310.06770), 2023.
- Zhang et al., [SWE-bench Goes Live!](https://arxiv.org/abs/2505.23419), 2025.
- Aleithan et al., [SWE-Bench+: Enhanced Coding Benchmark for LLMs](https://arxiv.org/abs/2410.06992), 2024.
- Wang et al., [Are “Solved Issues” in SWE-bench Really Solved Correctly?](https://arxiv.org/abs/2503.15223), 2025.

### Behavioral, contrast, and contamination controls

- Ribeiro et al., [Beyond Accuracy: Behavioral Testing of NLP Models with CheckList](https://aclanthology.org/2020.acl-main.442/), 2020.
- Gardner et al., [Evaluating Models' Local Decision Boundaries via Contrast Sets](https://aclanthology.org/2020.findings-emnlp.117/), 2020.
- Yang et al., [Rethinking Benchmark and Contamination for Language Models with Rephrased Samples](https://arxiv.org/abs/2311.04850), 2023.
- Matton et al., [On Leakage of Code Generation Evaluation Datasets](https://aclanthology.org/2024.findings-emnlp.772/), 2024.
- Zhao et al., [MMLU-CF: A Contamination-free Multi-task Language Understanding Benchmark](https://aclanthology.org/2025.acl-long.656/), 2025.

### Evaluation reliability and statistics

- Wang et al., [Large Language Models are not Fair Evaluators](https://aclanthology.org/2024.acl-long.511/), 2024.
- Liu et al., [G-Eval: NLG Evaluation using GPT-4 with Better Human Alignment](https://aclanthology.org/2023.emnlp-main.153/), 2023.
- Chiang and Lee, [A Closer Look into Using Large Language Models for Automatic Evaluation](https://aclanthology.org/2023.findings-emnlp.599/), 2023.
- Agarwal et al., [Deep Reinforcement Learning at the Edge of the Statistical Precipice](https://arxiv.org/abs/2108.13264), 2021.
- McNemar, [Note on the Sampling Error of the Difference Between Correlated Proportions or Percentages](https://doi.org/10.1007/BF02295996), 1947.
- Wald, [Sequential Tests of Statistical Hypotheses](https://doi.org/10.1214/aoms/1177731118), 1945.

### Official implementation documentation

- UK AI Security Institute, Inspect AI: [Agent Checkpointing](https://inspect.aisi.org.uk/checkpointing.html).
- UK AI Security Institute, Inspect AI: [Log Files](https://inspect.aisi.org.uk/eval-logs.html).
- UK AI Security Institute, Inspect AI: [Tracing](https://inspect.aisi.org.uk/tracing.html).
- UK AI Security Institute, Inspect AI: [Changelog](https://inspect.aisi.org.uk/CHANGELOG.html).

---

## Appendix C — commission self-check

- [x] Pinned commit `a77236f7d8ee9eec1f7a37564547ab9a8be5a0a0` resolved; all explicitly named extant primary paths were present and read in full.
- [x] The two `archive/workflows/` paths were treated as absent provenance claims, not fetched authority.
- [x] The severe unsupported-premise incident, 3/3-versus-3/3 tie, issue #41 priming, and genuine accumulated-context limitation were adjudicated separately.
- [x] The verdict addresses expressibility, discrimination without answer shaping, and authority to adopt.
- [x] Natural-output witnesses score ordinary review artifacts and can return expressed, unexpressed, contaminated, or blocked without requiring the executor to emit the answer.
- [x] Independence and source-blind claims have filesystem, session, role, model, and custody controls beyond prompt wording.
- [x] The pilot freezes run inventory, invalidation, replacement, sample rule, protected behavior, cost cap, and `pass`/`fail`/`tie`/`unexpressed`/`contaminated`/`blocked` outcomes before results.
- [x] No copied-self review, automatic routing, caller-authored event, provisional landing, or silent self-target bypass is introduced.
- [x] All five principle files, ADRs 0001–0007, and `docs/releasing.md` are covered in the governance matrix.
- [x] Census findings relied upon are labeled **survives**, **overtaken**, or **partly addressed**.
- [x] External decision-shaping claims use direct primary-source or official-implementation links; inference and proposal are identified as such.
- [x] Every later repository change has separate code/docs, evidence mutation, adoption, release, withdrawal, installer, and consumer-rollout gates.
- [x] The report recommends no present repository mutation, issue mutation, evidence rewrite, release, or dequarantine.
- [x] Deliverable set is one file: `skill-evolution-independent-instrument-research-report.md`.
