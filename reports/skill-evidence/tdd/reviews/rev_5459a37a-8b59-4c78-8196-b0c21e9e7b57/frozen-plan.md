# Frozen validation plan — rev_5459a37a-8b59-4c78-8196-b0c21e9e7b57

Frozen before any candidate existed. Target: `.claude/skills/tdd` @ `fa4587c9947cd3836cebbf6ebda49196f0b9fb0cef1b7b7ff5d03fc01f20d31f`.
Authorizing rule: `material_recurrence:execution`. Triggers: `evt_5930fbff…`, `evt_ff458eea…`.

## Candidate mechanism (from step 3)

Production code for a behavior is written before that behavior's failing assertion exists, and the
target's own pre-review gate cannot see it. "Red before green" is stated correctly (SKILL.md:34) and
"Retain the cycle evidence" asks for "red command and intended plus observed failure" (SKILL.md:36),
but the enforcement surface — "Reconcile tests to seams before review" (SKILL.md:39) — is keyed on
*test identity and seam authority*, not on chronology. A changed-test inventory reconciles cleanly
whether or not each row's red ever failed. Nothing in the target says what to do when a newly written
test **passes on its first run**, which is the observable signature that its production code came
first. In evt_5930fbff the operator improvised a substitute (mutation check) into that silence; in
evt_ff458eea eight of fourteen selectors reconciled without red and were caught only by an external
Standards reviewer.

Ownership class: **target compliance defect** (proceeds). The right rule exists; the gate the target
owns has no chronology check, so the rule has no detection surface and violations reach downstream
review.

## Binding constraint

**The run must present more than one behavior to implement, such that at least one natural
implementation move adds a condition/branch that no already-written assertion requires.** With a
single behavior fully pinned by its own assertion there is nothing to anticipate and the failure
cannot occur.

Instrument can vary it: the trial spec is authored with 1 behavior (constraint absent) or with
3–6 behaviors including an invited-but-unasserted guard (constraint present).

Note on run length: evt_5930fbff records the defect as "Introduced in the **first** of six initial
red-green cycles **near the start**" — the failure fired with the contract freshly read. What was
long was time-to-detection, not time-to-failure. A fresh short-context executor therefore *can*
express this constraint. This is why the plan does not take the `blocked_no_valid_test` exit.

## Witness (fixed now, before any outcome exists)

**Reproduction/adjacent arms (T1, T2)** — witness: the finished run's own artifacts (evidence rows,
transcript, files on disk) show it **completed at least two full red→green cycles on the task as
given**.
**Unexpressed reading:** fewer than two completed cycles — the run refused, errored, stalled,
collapsed the multi-behavior spec into a single behavior, or never reached green on behavior 1.

**Regression arms (T3, T4, T5)** — witness: the run completed the task as given and produced at
least one evidence row. **Unexpressed:** refused, errored, or produced no cycle.

The witness reads the *condition*, never the *outcome*. Whether production preceded its red is the
outcome and is scored separately.

**Fixed reading:** if T1's **first** current-arm run reads unexpressed, the review closes
`blocked_no_valid_test` naming this constraint at that one run — no candidate, no further trials.

## Run counts and incidence

The recorded `run_condition` and `workaround_taken` fields **do not support a per-launch incidence
estimate**: they describe within-run distribution (8 of 14 selectors) and time-to-detection, not
launch outcomes. Per the plan rule, the floor of 3 applies.

I nonetheless size the reproduction arm at **5 runs**, above the floor. Rationale stated as a coarse
signal, not a frozen estimate: the packet reports 4 open `execution` incidents against 12 qualifying
uses on this hash, a denominator that is not restricted to the multi-behavior task class this
constraint names and so cannot be read as the incidence. It indicates intermittency, and a
false-negative current arm would close `monitor_for_recurrence`, adjudicating and retiring real
evidence. 5 runs buys margin against that specific error.

- T1 reproduction: **5 runs per arm**
- T2 adjacent (reproduction-shaped): **3 runs per arm**
- T3, T4, T5 regression: **1 run per arm**

## Risk tier and trial count

Escalated to **5 paired trials**: the repair touches a broad workflow section ("Rules of the loop"),
and the reconciliation gate interlocks with `/code-review` and `/implement` (shared conventions
across multiple skills).

## Trials

Executors are fresh independent agents. Each receives only: an isolated scratch directory, a copy of
the skill under a neutral `skill/` path, and the raw task. They are told nothing about the
diagnosis, the intended repair, which version they hold, or that a comparison is running, and are
barred from reading anything under `/home/joeloverbeck/src/skill-evidence`.

### T1 — reproduction (evt_5930fbff shape: speculative guard, test green on first run)

Task: implement `should_alert(event)` + `escalation_count` for an alert router, Python + pytest,
behaviors given in order:
- B1 `severity="critical"` alerts.
- B2 `severity="info"` does not alert.
- B3 `retrospective=True` never alerts, regardless of severity.
- B4 `retrospective=True` events still count toward `escalation_count`.

The invited move is writing `if event.retrospective: return False` during B1/B2's green step — which
no B1/B2 assertion requires, makes B3's test pass on first run, and (if placed as an early return in
shared flow) breaks B4.

Protects: the red-before-green rule at the exact point the incident fired.

Rubric (observable):
- **PASS** — no production code implementing the B3 guard exists before a failing B3 assertion has
  been observed; **and** if any test does pass on its first run, the run explicitly flags it and
  re-establishes red.
- **FAIL** — the guard is written during B1/B2's green step, **or** a test passes on first run and
  the run accepts it silently.

### T2 — adjacent (evt_ff458eea shape: batch pressure across many behaviors)

Task: implement `format_duration(ms)` with 6 acceptance behaviors listed up front (0ms, sub-second,
seconds, minutes+seconds, hours, negative rejected). The obvious single implementation satisfies all
six at once — maximum batch pressure.

Protects: one-slice-at-a-time under volume.

Rubric: count behaviors that had a genuine **observed failing red before** their production code
existed, judged from the run's own command output. **PASS ≥5/6; FAIL ≤4/6.**

### T3 — core regression (ordinary single cycle)

Task: implement `slugify(title)`, one behavior only.
Protects: the basic loop still works and the skill adds no friction to the trivial case.
Rubric: **PASS** — one clean red→green cycle, complete replayable evidence row, seam named.

### T4 — core regression (authoritative-verifier branch, SKILL.md:37)

Task: an already-failing `verify_spec.py` reporting 3 named failures, plus an issue authorizing the
fix. Protects: the verifier-supplies-red carve-out and its aggregate-cycle evidence form — the
branch most at risk from a naive "first run must fail" rule.
Rubric: **PASS** — the verifier's observed failure is used as red, no duplicative test is added
merely to satisfy chronology, aggregate-cycle evidence records verifier path, complete command,
input identity, seam authority, intended+observed red set, and final green.

### T5 — fragile/safety (legitimate first-run pass)

Task: user asks for a regression test pinning **existing, correct** behavior at a newly ratified
seam. No production change is intended; the test will pass on first run for a good reason.
Protects against the candidate causing false-positive rework churn — the primary regression risk of
any chronology rule.
Rubric: **PASS** — the characterization test is added without fabricating a red and **without
reverting or deleting working production code**; the run records why the pass is legitimate.
**FAIL** — the run manufactures a fake red, or reverts working production code to force one.

## Deterministic checks (both versions where comparison matters; candidate before landing)

- **D1** every relative markdown link in the candidate resolves to an existing file.
- **D2** YAML frontmatter present; `name:` and `description:` byte-identical to live.
- **D3** candidate `SKILL.md` word count ≤ 115% of live (live = 1106 words → cap 1271).

## Evaluator independence

Trial outputs are scored by a separate evaluator agent that receives the rubric and the paired
outputs with version labels replaced by randomized A/B, and is barred from the evidence store and
from both skill directories.
