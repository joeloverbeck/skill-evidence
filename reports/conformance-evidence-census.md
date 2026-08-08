# Conformance-evidence census

Decision-bound research for [#2](https://github.com/joeloverbeck/skill-evidence/issues/2), which
in turn gates [#1](https://github.com/joeloverbeck/skill-evidence/issues/1).

**Named decision:** should compliance-graded evidence be allowed to open a gate whose acceptance
test is an outcome improvement, and if not, what should happen to it instead?

**Population:** every recorded event in the three consumer evidence stores as of 2026-08-08.
1,083 events → 330 incidents → 94 graded `material_failure` or `severe_incident`. Read-only;
no consumer tree was modified.

> **Revision note.** The first version of this census classified incidents on the `consequence`
> field alone and reported 14 as "conformance-only, no established harm." That was wrong. Reading
> `expected` and `observed` alongside `consequence` shows most of those are not *absence* of harm
> but *inability to observe* it — and two record harm outright. Section 3 replaces the original
> finding; sections 1 and 4 are unchanged and were unaffected.

---

## 1. The acceptance gate is not broken

52 concluded reviews:

| Disposition | playbench | mundifold |
|---|---:|---:|
| `candidate_rejected_validation` | 14 | — |
| `monitor_for_recurrence` | 12 | — |
| `outside_target` | 10 | 1 |
| **`resolved_by_change`** | **8** | — |
| `blocked_no_valid_test` | 3 | 2 |
| `closed_no_skill_defect` | 2 | — |

The outcome-graded gate has **accepted eight changes** and built-then-rejected fourteen more
candidates. Whatever #2 is, it is not "no candidate can satisfy the acceptance gate." It is a
mismatch that bites in **5 of 52 concluded reviews (10%)**.

## 2. Where the deviations concentrate

14 of 94 material-or-worse incidents (15%) record a deviation whose `consequence` field does not
assert harm. They are not evenly spread:

| Skill | Such incidents | All material+ | Share |
|---|---:|---:|---:|
| `skill-evolution-status` | 1 | 1 | 100% |
| `code-review` | 5 | 6 | 83% |
| `grilling` | 4 | 6 | 67% |
| `writing-great-skills` | 3 | 32 | 9% |
| `tdd` | 1 | 21 | 5% |
| *12 other skills* | 0 | 28 | 0% |

The skills at the top share a property: **their deliverable is an assurance.** `code-review`'s
product is "both axes were checked and reconciled"; `writing-great-skills`' is a completion proof;
`grilling`'s is a ratified recap; `skill-evolution-status`' is an exact relay. For those,
conformance *is* the outcome — there is no separate artifact left to be worse.

## 3. "No harm" mostly means "no way to tell"

Reading `expected` and `observed` rather than `consequence` alone changes the picture completely.
**In eleven of the fourteen, the step that was skipped is the very instrument that would have
detected the harm.**

| | Count | What the records actually show |
|---|---:|---|
| **Harm established** | 2 | `consequence` reads clean, `observed` does not |
| **Harm unobservable — the detector was the skipped step** | 11 | No one looked, because looking was the omitted step |
| **Harm genuinely absent and independently checkable** | 1 | |

**Harm established (misclassified in the first pass).**

`tdd` — the skipped step was a both-directions comparison of finding identities. Its `observed`
field records what the comparison would have found:

> The session's own record shows the differences would not have been empty in either direction:
> findings R1-spec-1 and R2-spec-1 were supplied carrying re-entry required and have no
> corresponding evidence row, while rows E3 and E4 correspond to no re-entry-flagged finding.

The gate would have failed. Its `consequence` field says only that the change was "reviewed clean
on both axes at pass 7 … with the review-reentry reconciliation gate unexecuted."

`code-review` — seventeen findings emitted and fixed, no ledger row for any of them, the aggregate
gate never run, and:

> The review was nonetheless handed off as green at 96ef1ae, which the gate exists to prevent when
> it has not been satisfied.

**Harm unobservable by construction.** In each of the eleven, the omitted step *is* the detector:

- `code-review` ×4 — the omitted step was the aggregate-conformance gate, which reconciles emitted
  finding IDs against resolved ones in both directions. The consequence field then asserts *"All 16
  emitted finding IDs were in fact accounted for … so no finding was lost."* That is the
  conclusion the gate exists to establish, asserted from conversation memory by the run that
  skipped it.
- `writing-great-skills` ×3 — the omitted step was the fresh independent-context forward test, the
  only instrument that detects whether a revised skill works cold. Whether it would have passed is
  unknowable.
- `grilling` ×4 — required references were never loaded, then edits, recaps and final summaries
  were produced. One record says so plainly: *"whether they met those contracts is unestablished."*

**Genuinely absent.** One: `skill-evolution-status`, where an extra block was appended to a relayed
census. The relayed content is independently comparable to the helper's output, so "the census
content itself was unchanged" is an observation rather than an assertion. The relay contract was
still violated, and for that skill the relay *is* the product.

### Why this matters more than the count

The 6 records that read most reassuringly — *"no finding was lost"*, *"substantively accurate"*,
*"remains accurate"* — are the ones where the verification was most completely skipped. That is not
coincidence. **The less of the check that ran, the less evidence of harm survives to be recorded.**

Two consequences:

1. **Option (a) would install a perverse gradient.** Grading conformance-only deviations below
   threshold means the failure mode that most thoroughly destroys evidence of itself receives the
   lowest grade. Skip half a check and the discrepancy surfaces; skip all of it and the record
   reads clean.
2. **The substrate has a live instance of something the adopted prohibitions forbid.**
   `evidence-substrate-integrity.md`: *"no caller-supplied assertion silently promoted to a derived
   fact"*, and *"Where the caller must assert something the system cannot compute, that assertion is
   recorded verbatim as an assertion, attributed, and never silently promoted."* A `consequence`
   field asserting "no finding was lost", written by the run that skipped the reconciliation, is
   exactly such an assertion — and the gate reads it back as fact.

This points at a defect in **capture**, not in evolution's acceptance gate: the `consequence` field
conflates *observed no harm* with *did not look*. Those want different tokens. That is a separate
issue from #1 and #2.

## 4. The instrument for option (b) cannot be built for this population

Every one of these incidents that records a `run_condition` names accumulated volume or elapsed run
length as the binding condition:

| | Recorded run condition |
|---|---|
| `code-review` ×5 | five to seven two-axis passes, 10–14 reviewer sub-agents, 18–30 file diffs, up to ~6,350 insertions; "the omission was present from pass R1 and persisted" |
| `grilling` ×4 | full-session adjudications, ~570 inserted lines across 4 files, one 3,500-line brief plus a 212-row index, ~40 tool calls |
| `tdd` ×1 | one re-entry spanning seven code-review passes and 35 findings over an 18-file change |

Not one is a discrete short step. Every one is a **late-in-a-long-run omission**, which a trial
executor starting fresh and short-context cannot express regardless of what it measures. Three
records carry no `run_condition` and neither support nor weaken this.

## 5. What the evidence supports

- **Against (a).** Two reasons now. It would disarm the assurance-producing skills specifically
  (83% of `code-review`'s material evidence), and it would reward concealment: the more completely
  a check is skipped, the cleaner the record reads.
- **Against (b) as conceived.** The binding constraint in every measured case is accumulated
  context, which no fresh-executor trial can hold.
- **For (c), as necessary but not sufficient.** It stops the projection reporting retained
  inconclusive evidence as handled. It supplies no exit.

The evolution instrument has no way to test late-in-long-run conformance failures. That is a
property of the instrument, not a defect in the evidence — and adopted text then decides the rest:
a gate satisfiable only by evidence its own structure prevents from existing is a trap, and the
answer is an honest exit. That exit is #1's option **(B)**, paired with **(c)** here.

## 6. What this census does not settle

The values question, correctly stated. It is **not** "do you want a system that acts on deviations
absent demonstrated harm" — that framing was an artifact of the classification error above. It is:

> **Do you want a system that acts when a deviation destroyed the evidence of its own
> consequences?**

That is a different question and a much easier one, because the alternative rewards exactly the
runs that leave the least trace.

## Method and limits

- Classification requires reading `expected`, `observed` and `consequence` together. The first
  version of this document read `consequence` alone and got two cases wrong in the direction of
  under-counting harm. All fourteen determinations are quoted above or in the source data.
- Borderline cases are resolved toward *harm established* (e.g. two `grilling` incidents requiring
  re-adjudication are counted as harm, since rework occurred, even though no wrong artifact
  shipped).
- Snapshot as of 2026-08-08. Other sessions were appending to `mundifold` during extraction.
- `what-we-bring-home` contributed 8 events, 4 incidents, 0 material failures, 0 dispositions — too
  young to inform this.
