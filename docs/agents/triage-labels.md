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
