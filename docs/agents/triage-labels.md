# Triage Labels

The skills speak in terms of five canonical triage roles. This file maps those roles to the actual label strings used in this repo's issue tracker.

| Label in mattpocock/skills | Label in our tracker | Meaning                                  |
| -------------------------- | -------------------- | ---------------------------------------- |
| `needs-triage`             | `needs-triage`       | Maintainer needs to evaluate this issue  |
| `needs-info`               | `needs-info`         | Waiting on reporter for more information |
| `ready-for-agent`          | `ready-for-agent`    | Fully specified, ready for an AFK agent  |
| `ready-for-human`          | `ready-for-human`    | Requires human implementation            |
| `wontfix`                  | `wontfix`            | Will not be actioned                     |
| `coordination`             | `coordination`       | Open parent holding sequencing for separately AFK-ready children |

When a skill mentions a role (e.g. "apply the AFK-ready triage label"), use the corresponding label string from this table.

All six labels exist in `joeloverbeck/skill-evidence`. `wontfix` is GitHub's stock label; the other five were created for triage. They apply to external PRs too — see `issue-tracker.md`.

These six are triage *state*. They are orthogonal to the repo's type labels (`bug`, `enhancement`, `question`, `documentation`, …), which say what a request is rather than where it stands. An issue carries one of each.

Edit the right-hand column to match whatever vocabulary you actually use.

## `coordination` was playbench's state, adopted here on 2026-08-11

`playbench` carries a sixth state, `coordination`, for an open `PRD:` or `Program:` parent whose separately AFK-ready children hold the implementation work. It arrived in this repository's triage skill as unlocalized copy when the common skills were brought over, naming a label that did not then exist in `joeloverbeck/skill-evidence` — so a parent transition into it would have failed *after* the child issues were already created.

**It was deliberately not adopted until now**, because there was nothing here to coordinate. Every issue this tracker had held was leaf work; #10 was split off from #8 during triage, but neither was kept open as a parent. This repository had no program structure to give one. `to-issues` handled the absence — its documented fallback is to propose a truthful non-AFK state or to park the child breakdown. That note also carried a standing condition: if a genuine umbrella ever appeared, create the label then and add the row above.

**That condition fired on 2026-08-11.** Six open issues arrived from one skill-evolution run cluster carrying dependencies no child owns. A three-deep chain: #34 creates external-owner identity, #37 routes evidence with it, and #36's gate can only authorize once #37 lands. A decision dependency: #33's remaining scope is whatever #32's claim-time freeze leaves behind. And a collision: #32 and #34 mutate the same command surface *and* the same installed reference file, which two parallel agents would meet head-on. Release bundling sits on top — both are minor bumps requiring `skills evidence install --force`, and [`../releasing.md`](../releasing.md) §6 makes each such release a seven-step ritual run once per consumer. The label was created and the row added at that point, per the condition above.

Recording the transition is the point, exactly as recording the absence was. The triage skill arrived by wholesale copy and will arrive that way again. What flips is which claim goes stale: a re-copied `SKILL.md` asserting this repo has no umbrella state is now the error to catch, because the tracker has one and `to-issues`'s never-invent-a-label fallback no longer applies to a parent that needs it.
