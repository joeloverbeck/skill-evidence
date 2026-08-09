# Triage Labels

The skills speak in terms of five canonical triage roles. This file maps those roles to the actual label strings used in this repo's issue tracker.

| Label in mattpocock/skills | Label in our tracker | Meaning                                  |
| -------------------------- | -------------------- | ---------------------------------------- |
| `needs-triage`             | `needs-triage`       | Maintainer needs to evaluate this issue  |
| `needs-info`               | `needs-info`         | Waiting on reporter for more information |
| `ready-for-agent`          | `ready-for-agent`    | Fully specified, ready for an AFK agent  |
| `ready-for-human`          | `ready-for-human`    | Requires human implementation            |
| `wontfix`                  | `wontfix`            | Will not be actioned                     |

When a skill mentions a role (e.g. "apply the AFK-ready triage label"), use the corresponding label string from this table.

All five labels exist in `joeloverbeck/skill-evidence`. `wontfix` is GitHub's stock label; the other four were created for triage. They apply to external PRs too — see `issue-tracker.md`.

These five are triage *state*. They are orthogonal to the repo's type labels (`bug`, `enhancement`, `question`, `documentation`, …), which say what a request is rather than where it stands. An issue carries one of each.

Edit the right-hand column to match whatever vocabulary you actually use.

## `coordination` is playbench's state, deliberately not adopted here

`playbench` carries a sixth state, `coordination`, for an open `PRD:` or `Program:` parent whose separately AFK-ready children hold the implementation work. It arrived in this repository's triage skill as unlocalized copy when the common skills were brought over, naming a label that does not exist in `joeloverbeck/skill-evidence` — so a parent transition into it would have failed *after* the child issues were already created.

It is not adopted because there is nothing here to coordinate. Every issue this tracker has held has been leaf work; #10 was split off from #8 during triage, but neither was kept open as a parent. This repository has no program structure to give one. `to-issues` already handles the absence — its documented fallback is to propose a truthful non-AFK state or to park the child breakdown.

Recording the absence is the point. The triage skill arrived by wholesale copy and will arrive that way again; without this note the clause comes back. If a genuine umbrella ever appears, create the label then — it is one `gh label create` — and add the row above.
