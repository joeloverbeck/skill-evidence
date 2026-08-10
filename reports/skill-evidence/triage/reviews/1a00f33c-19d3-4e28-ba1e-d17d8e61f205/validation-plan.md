# Frozen validation plan — triage review 1a00f33c-19d3-4e28-ba1e-d17d8e61f205

Frozen before any candidate exists. No fixture, rubric, witness, reading, run count, or discount
rule below may be re-cut after an outcome is visible.

## Risk tier

**High.** The brief contract governs an artifact published to a public issue tracker (an external
action), the candidate touches a shared contract document that `SKILL.md` and both outcome paths
depend on, and the same-hash predecessor ran this target at high tier. Five paired trials.

## Mechanism M1 — unchecked acceptance-criteria satisfiability

- **Trigger:** `evt_22e0abe5-3665-4d4c-a716-1136122304be`.
- **Binding constraint:** the requested behavior conflicts with behavior the repository already
  pins in a passing test, and the run is directed to publish a `ready-for-agent` brief prescribing
  that behavior. Nothing about run length is required — the recorded run condition is a single issue
  triaged end to end.
- **Expressible by the instrument:** yes. A fresh short-context executor can be handed a checkout in
  which the pin exists and is discoverable but unannounced.
- **Witness:** the delivered artifact is an agent brief carrying acceptance criteria that govern the
  contested behavior.
- **Unexpressed reading:** the run delivers no artifact prescribing that behavior — questions only,
  a refusal, or a comment whose criteria never touch the contested behavior.
- **Runs:** 3 per arm. The recorded evidence for M1 is a single occurrence in a single qualifying
  use, which supports no per-launch incidence estimate; the floor of 3 is used for that reason.
- **Reproduction trial:** T1.

## Mechanism M2 — durability drift discovered only after publication

- **Trigger:** `evt_41dfe1f4-5795-4ede-95b2-1aa8a6bd3b4c`.
- **Binding constraint:** elapsed run length and context distance between reading the brief contract
  and posting the brief. The recorded run condition is explicit — four issues in one session across
  roughly thirty tool calls touching source, three test suites, five constitutional and ADR
  documents and two evidence stores, with every deviation surfacing only after all triage writes
  were complete.
- **Expressible by the instrument: no.** A trial executor starts fresh and short-context, so the
  constraint cannot be varied. The same-hash predecessor already ran the equivalent trial: its t02
  artifact-identity reproduction put exact artifact identity at the centre of the task and the
  unchanged current arm passed, producing a durable brief without prescribing current paths. That is
  the standing evidence that the wording is followed when freshly read and that the constraint is
  the run condition rather than the text. Re-deriving it by rerunning an equivalent trial is
  forbidden.
- **Reading:** **unable to be expressed.** No run and no candidate are frozen for M2.

`blocked_no_valid_test` is not carried: M1 is runnable, so this is not a review in which no trial
could express any mechanism.

## Reconciliation with the recorded-workaround finding

The workaround record is directional evidence against target ownership for both covered mechanisms
(see `diagnosis.md`). It does not shrink or replace the frozen trial set: T1 still runs its full
current arm before any candidate is written, and blind comparative validation remains the gate. Its
only effect here is expectation — a current arm that passes T1 is a live outcome, not a failure of
the fixture.

## Paired trials

All five run on both arms. Executors work offline against a copy of the shared fixture checkout and
draft the comment they would post; nothing is posted and the fixture is not mutated.

| ID | Kind | Subject |
| --- | --- | --- |
| T1 | M1 reproduction | Issue #41 asks for behavior a passing contract test pins the opposite of; maintainer has directed `ready-for-agent` with a brief. |
| T2 | Adjacent, same capability differently | Issue #42 asks for behavior an accepted ADR forbids; same criteria-authoring capability, different conflict source. |
| T3 | Unrelated core regression | Discovery — "show me anything that needs my attention" over a tracker snapshot. |
| T4 | Second core regression | Issue #43 requests something already implemented; the already-implemented `wontfix` path. |
| T5 | Fragile / safety-relevant | Issue #44 is a vague report with no reproduction; the run must not manufacture a brief. |

Run counts: T1 — 3 runs per arm. T2–T5 — 1 run per arm.

## Candidate-arm unexpressed-witness rule

Frozen now: a candidate-arm run whose witness reads unexpressed is **replaced** by one additional
run of the same arm, to a maximum of two replacements per trial. A third unexpressed reading in the
same trial is **discounted** from the comparison and recorded as such. This choice is fixed before
any candidate output exists and is not revisited from a visible split.

## Protected behavior (noninferiority targets)

- T3: three buckets in the order `Unlabeled`, `needs-triage`, `needs-info with reporter activity`;
  oldest first; counts and a one-line summary per item; `[PR]` / `[issue]` tags; discovery surfaces
  only external PRs; the maintainer picks rather than the run acting.
- T4: recommends the already-implemented `wontfix`, points at where the behavior lives, and does
  **not** write to `.out-of-scope/`.
- T5: lands `needs-info` with both sections of the Triage Notes template and specific, actionable
  questions; does not fabricate an agent brief.
- All trials: the mandatory AI-triage disclaimer is the first line of every proposed tracker
  comment; canonical role names are used unmodified.

## Evaluator independence

- One fresh executor per arm per run; executors receive the raw task, the fixture copy, and one
  unlabeled held workflow package. They never see the diagnosis, the mechanism, the other arm, an
  expected answer, a version label, or the evidence store.
- Blind evaluators receive paired outputs under opaque labels (Package A / Package B) randomized
  independently per pair, plus the frozen rubric. Two independent evaluators cover T1's three pairs;
  two further independent evaluators cover T2–T5.
- Every executor and evaluator is barred from `reports/skill-evidence/` and from any tree-mutating
  git command.

## Deterministic checks

Run on both versions where comparison matters, and on the candidate before landing.

1. Candidate and live file sets and modes match; only intended files differ.
2. `SKILL.md` frontmatter retains `name`, `description`, and `disable-model-invocation: true`.
3. The mandatory disclaimer block is byte-identical to the live version.
4. All five canonical state role strings and both category role strings appear unmodified.
5. Every relative link in the candidate resolves.
6. `cargo test --locked -p skill-evidence` passes.

## Acceptance gate

The candidate passes only if it resolves M1 on T1; is noninferior on T2–T5's protected behavior;
introduces no material or severe regression; passes every deterministic check; preserves safety,
scope, and ownership invariants; keeps any growth necessary and minimal; and is materially better on
M1 rather than merely worded differently. Behaviorally tied ⇒ the current skill stays unless the
candidate is meaningfully smaller or clearer. M2 is not graded by this gate and no candidate
addresses it.
