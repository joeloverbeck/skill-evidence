# Frozen validation plan — rev_c25c6d82-756b-49e4-85ed-bd164820f2f2

Frozen before any candidate existed and before any trial ran. Target hash at freeze:
`fa4587c9947cd3836cebbf6ebda49196f0b9fb0cef1b7b7ff5d03fc01f20d31f`.

## Mechanisms under trial

**M1 — non-replayable evidence commands.** An evidence row's red or green command position is
filled with a prose label or back-reference instead of a complete runnable command or a stable
defined key. Triggers: `evt_7cff09bf` (whole), `evt_a1cbfef5` (abbreviated-command part).
Candidate ownership class: *target compliance defect*. SKILL.md:38 states the rule correctly and
even names the anti-pattern, but SKILL.md:36 asks for a "compact" evidence row in the same breath,
and the pre-review reconciliation gate (SKILL.md:39) is keyed on test identity and seam authority,
so it never reads a row's command position. The rule has no detection surface inside the skill.

**M2 — a behavior with no assertion of its own.** A requirement whose subordinate clause is a
second testable behavior gets one selector covering the headline behavior only; removing the
clause's production code leaves the retained suite green. Triggers: `evt_37543379` (whole),
`evt_a1cbfef5` (bundled-selector part). Candidate ownership class: *target defect*. SKILL.md:35
bounds a cycle to one selector but never says a compound requirement decomposes into one pinned
behavior each; the reconciliation gate (SKILL.md:39) keys rows on *changed test identity*, so a
behavior that never got a selector is structurally invisible to it; and the only "would this still
pass if the behavior were removed" text lives in tests.md under *Discriminating goldens*, reachable
only through SKILL.md:16's conditional pointer ("Before testing a public invariant or choosing a
golden fixture"), whose condition does not fire for an ordinary behavior slice.

`evt_a1cbfef5` maps to **both** mechanisms; its recorded observation contains both forms
("abbreviated commands in two evidence rows and two public scenarios bundled in one selector").
Its reading is the disjunction: it reproduces if either T1 or T2 reproduces.

## Binding constraints and expressibility

| Mechanism | Binding constraint | Instrument can vary it? |
|---|---|---|
| M1 | Several evidence rows in one run whose focused red and green commands are identical and long, so repeating the command in every row reads as redundant | Yes — 4 behaviors on a deep path vs 1 behavior |
| M2 | One requirement whose headline behavior is naturally assertable alone while a subordinate clause carries a second testable behavior | Yes — compound requirement vs two numbered requirements |

Neither constraint needs detection distance. The predecessor review
(`rev_5459a37a`) recorded a real limit — a fresh short-context executor reproduces a *decision
point* but not a *detection distance measured in review passes*. That limit does not bind here:
both defects are present in the artifacts the run itself hands over at the moment it declares
itself ready for review (the evidence rows for M1; the retained suite for M2), so each is read off
the finished run's own output rather than requiring six downstream review passes to surface.

Neither mechanism is marked unable to be expressed. `blocked_no_valid_test` is not taken.

## Witnesses, fixed before any outcome exists

### T1 (M1) — 5 runs per arm

- **Witness (expressed):** the finished run left a retained TDD evidence section with **≥3 rows**,
  each row having a red command position and a green command position, and the run reached green on
  ≥3 behaviors.
- **Unexpressed reading:** fewer than 3 evidence rows, no evidence section at all, or fewer than 3
  behaviors reaching green (refused, errored, stalled, collapsed the spec).
- **Failure reading (mechanism reproduced):** *any* row's red or green command position is filled by
  a prose label, a back-reference, or a description of the implementation, rather than by (a) a
  complete runnable command or (b) a stable key that is defined verbatim somewhere in the same
  evidence section. Non-exhaustive examples of a prose label: "same command", "same as above",
  "same as red command above", "the focused test", "same suite".
- **Pass reading:** every row's red and green command position holds a complete runnable command or
  a key defined verbatim in the same section.

### T2 (M2) — 5 runs per arm

- **Fixed seam (so the mutation is uniform):** module `triage_service.py` at the workspace root,
  exposing `triage(records)` returning `{"processed": [...], "untestable": [...]}`.
- **Witness (expressed):** the run left a runnable pytest suite that is **green before mutation**,
  and the final code implements the compound requirement (a mixed input yields a non-empty
  `untestable`).
- **Unexpressed reading:** no runnable suite, suite not green as left, the fixed seam absent, or the
  requirement never implemented — in any of these the mutation cannot be applied.
- **Frozen mutation:** a `conftest.py` is added that wraps `triage` so that **when the returned
  `untestable` list is non-empty, `processed` is replaced by `[]`**. This removes exactly the
  subordinate clause's behavior — "carry on processing the remaining records" once something was
  undecidable — and nothing else. The all-decidable path is left untouched, so a test of the
  headline behavior alone does not incidentally kill the mutant.
- **Failure reading (mechanism reproduced):** the retained suite is still **fully green** under the
  mutation — no assertion pins the second behavior.
- **Pass reading:** the mutation makes at least one retained test fail.
- **Recorded but not scored:** how many selectors the run wrote for the compound requirement.

### T3 — adjacent, authoritative-verifier branch (SKILL.md:37) — 2 runs per arm

Same capability (evidence rows and pre-review reconciliation) exercised differently: an
already-failing public verifier supplies red. **Pass:** the run uses the existing verifier as red
instead of adding a duplicative test, records the verifier's complete command and input identity,
and reaches green. **Fail:** the run writes a duplicative test only to satisfy chronology, or the
verifier branch is blocked/misrouted.

### T4 — core regression, ordinary single cycle — 2 runs per arm

**Pass:** red observed before production code, one selector for the one behavior, complete command
in the evidence row, green reached. Also recorded for bloat comparison: NOTES.md word count.

### T5 — fragile / false-positive guard — 2 runs per arm

A single behavior that legitimately needs two assertions in one test. **Pass:** the run keeps **one**
selector for that one behavior and still records complete commands. **Fail:** the run splits one
behavior across multiple selectors, or adds ceremony the behavior does not require. This trial
exists to catch a candidate that buys M2 with gratuitous selector inflation.

## Protected behaviors (noninferiority)

- **P1** red observed before production for every behavior (T1, T2, T4) — no test green on first run.
- **P2** tests at public seams; no implementation coupling.
- **P3** the authoritative-verifier branch stays available and usable (T3).
- **P4** no gratuitous selector inflation for a genuine single behavior (T5).
- **P5** runtime size stays within D3.

## Deterministic checks

- **D1** every relative link in the candidate SKILL.md resolves to an existing file.
- **D2** frontmatter `name` and `description` unchanged.
- **D3** SKILL.md word count ≤ **1271** (current 1106 + 15%, same cap the predecessor review used).

## Incidence sizing

The recorded `run_condition` and `workaround_taken` fields describe *within-run* distribution
(2 of N rows abbreviated; 3 of 5 changed-test identities; 1 of 2 slices) and time-to-detection, not
launch outcomes, so they support **no per-launch incidence estimate**. The floor of 3 therefore
applies. Both reproductions are nonetheless sized at **5 runs** per arm, for the reason the
predecessor gave: a false-negative arm adjudicates and retires real evidence. T3–T5 are comparison
trials rather than incidence measurements and are sized at 2 runs per arm.

## Unexpressed candidate-arm runs — discount or replace, frozen before results

A candidate-arm run whose witness reads unexpressed is **replaced** by one additional run of the
same trial, up to **2 replacements per trial**. If a third run in the same trial reads unexpressed,
that trial is reported as unable to be expressed on the candidate arm and is dropped from the
comparison — never counted for or against the candidate.

## Pre-registered outcome-deficit reading (frozen before results)

All three triggers are classified **conformance-only** at step 3: each `consequence` records rework
before delivery, not worse delivered work. A trigger is therefore treated as outcome-graded *by the
trials* only if its trial demonstrates a defect in an artifact the run itself hands over as final —
for T2, a retained suite that survives the frozen mutation; for T1, a retained evidence row that
cannot be replayed as written. If a trial shows only that the run's process differed from the
instruction while the artifact it delivered was sound, that trigger stays conformance-only and is
named as untestable coverage at close.

## Executor and evaluator independence

Executors are fresh independent agents with no task-local context, given only the raw task and a
`method/` directory holding an unlabelled copy of one arm's skill files. They are barred from the
evidence store, from this plan, from the diagnosis, and from any version label, and are forbidden to
run any tree-mutating git command. Arms are indistinguishable at the executor's path. T1 and T2
outputs are additionally scored by an independent evaluator over a pooled, anonymized, shuffled set
with arm labels stripped.

## Risk tier

Claimed `provisional`; escalated here to **5 paired trials**. The candidate touches a broad workflow
section — the pre-review reconciliation gate, which interlocks with `/code-review` and `/implement` —
and more than one major behavior.
