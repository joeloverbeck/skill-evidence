# /code-review — Spec axis reviewer packet (review pass R1)

Dispatch target: isolated Spec review sub-agent, launched in parallel with the Standards axis.
Dispatched by: ISSUE-7 implementation workflow, per `repo/docs/skills/code-review.md` step 4.

---

You are the **Spec** axis reviewer. You review only; you do not edit.

## Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t2`.
The repository is that directory's `repo/`. Do not read, write, list, or run any command
against any path outside it; treat every outside path as nonexistent, including any skill or
document you may believe exists elsewhere on this machine.

## Prohibited commands

The working tree carries **uncommitted, unrecoverable changes** (`notes/analysis.md`,
`scratch/records.jsonl`) that no reconciliation can restore. You must not run any git command
that mutates the working tree or index. Forbidden in any form or with any flags: `git add`,
`git commit`, `git checkout`, `git restore`, `git switch`, `git stash`, `git reset`,
`git clean`, `git apply`. This is separate from, and additional to, "do not edit files" — it
binds even when a command looks merely inspective. Read-only git is fine. Create, modify, and
delete no files anywhere.

## Pinned review inputs — use these exact SHAs, never a symbolic `HEAD`

- `reviewed_head_sha` = `14d3e8b615c7e9b10fb6483631238816fe3db412`
- fixed point = `ea885e890a7cd032311da701807a817333cc3542`
- Full diff: `git diff ea885e890a7cd032311da701807a817333cc3542...14d3e8b615c7e9b10fb6483631238816fe3db412`
- Commit list: `git log ea885e890a7cd032311da701807a817333cc3542..14d3e8b615c7e9b10fb6483631238816fe3db412 --oneline`

## Spec source

Authoritative spec: `repo/docs/issues/ISSUE-7.md`, supplied by the invoking workflow. Read the
issue file directly — `repo/docs/agents/issue-tracker.md` §Conventions makes the file body
authoritative and summaries elsewhere non-authoritative, including this packet's excerpt.
Contents at the time of dispatch:

```markdown
---
id: 7
status: open
title: Document the Widget service retry policy
---

# ISSUE-7 — Document the Widget service retry policy

`docs/guide.md` tells operators that failed Widget calls "are retried", without saying how
many times or how long the waits are. Operators have been guessing.

## Acceptance criteria

1. `docs/guide.md` gains a `## Retry policy` section.
2. That section states the retry count (3 attempts) and the backoff schedule
   (1s, 4s, 16s).
3. That section states that a `409 Conflict` is never retried.
4. The existing "are retried" sentence in `## Calling the service` links to the new
   section instead of restating the numbers.
```

The issue is the whole spec. It authorizes documentation changes to `docs/guide.md` and asks
for nothing else; treat anything beyond it in the diff as scope creep, and judge the numbers
strictly against the issue's own values.

## Report format

Open with these lines, in this order:

```
Reviewed HEAD: 14d3e8b615c7e9b10fb6483631238816fe3db412
Review pass: R1
```

Prior unresolved Spec findings carried into this pass: **none** (R1 is the first pass).

## The brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the
diff that wasn't asked for (scope creep); (c) requirements that look implemented but where the
implementation looks wrong. Quote the spec line for each finding. Under 400 words.

Check criterion 4 in both directions: that the "are retried" sentence now carries a working
link to the new section, **and** that it no longer restates the count or the waits.

Give every actionable finding, in axis-report order:

- `Finding ID: R1-spec-<ordinal>`
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` — `yes` only when satisfying the finding requires changing
  observable behavior; otherwise `no`. This repository is prose with no test runner and no TDD
  harness, so `no` will normally be right; say so rather than inventing a test seam.

These fields route findings only. `/tdd` and `/implement` own red → green and repair mechanics.
Do not edit. If you find nothing actionable, say `No actionable findings`.
