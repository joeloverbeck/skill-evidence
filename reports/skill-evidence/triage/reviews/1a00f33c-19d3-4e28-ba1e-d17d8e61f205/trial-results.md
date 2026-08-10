# Trial results — triage review 1a00f33c-19d3-4e28-ba1e-d17d8e61f205

## What ran

Only the current arm of T1, the M1 reproduction, as the frozen plan directs: the current arm runs
before anything is built, and the first witness is read before another run is spent.

| Run | Arm | Witness | Rubric verdict |
| --- | --- | --- | --- |
| `wr4821` | current (unchanged skill) | expressed | PASS |
| `wr9317` | current (unchanged skill) | expressed | PASS |
| `wr2064` | current (unchanged skill) | expressed | PASS |

Run 1 (`wr4821`) read **expressed**, so the trial continued to its planned three runs.

Raw outputs: `trials/t01-criteria-satisfiability/current-arm/`.
Blind verdicts: `trials/t01-criteria-satisfiability/verdicts/`.

## Blind grading

Two independent graders received the three submissions under neutral labels, the frozen rubric, the
task, and a copy of the fixture checkout. Neither saw the diagnosis, the mechanism, the incident
evidence, the skill under review, or the fact that a single arm produced all three. Both were
instructed to be adversarial about pass criterion 3, joint satisfiability, and to trace whether an
implementing agent could satisfy every delivered requirement at once.

Both returned **witness expressed, PASS** for all three submissions — unanimous, 6/6 readings.

What both graders found independently: every run located
`tests/render_contract.rs::brief_report_always_ends_with_a_summary_line`, quoted the `CONTRIBUTING.md`
rule that a contract assertion changes only under a superseding ADR and never as ordinary bug work,
and stated a resolution. The rubric's failure mode — criteria that require the pinned behavior to
change while the pin stands unaddressed, or contract tests declared out of scope alongside criteria
that cannot be met without editing one — never occurred. Each run carved in the single empty-brief
assertion, fenced every other contract assertion out, and supplied the ADR authority the convention
requires. Their resolutions differed in shape (maintainer-authored ADR as a gate with the issue
rerouted to `ready-for-human`; agent-authored ADR in the same change with sign-off at merge; a
pre-drafted ADR for verbatim transcription), and both graders ordered material preference the same
way, but every one passed.

Grader beta flagged the sloppiest passage in the set — `wr4821` forbids renaming the assertion "to
get to a green bar" while an acceptance criterion requires its name to be updated — and traced that
the artifact states the permission explicitly under Key interfaces, so the set remains jointly
satisfiable. Recorded because it is the closest any run came to the recorded failure shape, and it
did not reach it.

## Mechanism readings

- **M1 — unchecked acceptance-criteria satisfiability** (`evt_22e0abe5`): **not reproduced with
  witnesses expressed.** Three current-arm runs, witness expressed on every one, failure on none.
- **M2 — durability drift discovered only after publication** (`evt_41dfe1f4`): **unable to be
  expressed.** Frozen as such at plan time; its binding constraint is elapsed run length and context
  distance, which a fresh short-context executor cannot vary, and the same-hash predecessor's t02
  already read the current arm passing on this mechanism shape.

## Consequences for the review

No mechanism reproduced, so no candidate was built. T2–T5 were frozen as paired comparison trials;
with no candidate there is no second arm to pair against, so they did not run. The acceptance gate
was never reached and graded nothing. The deterministic checks compare a candidate against the live
target and were therefore not applicable; the live target was never touched.
