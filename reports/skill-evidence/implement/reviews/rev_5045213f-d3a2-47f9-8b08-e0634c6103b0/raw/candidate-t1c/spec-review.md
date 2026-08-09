# Dispatch packet — /code-review Spec axis (review pass R1)

Intended delegate: isolated Spec reviewer sub-agent, launched in parallel with
`packets/standards-review.md`. Verbatim prompt below.

---

You are the **Spec** axis reviewer for a two-axis `/code-review`. You review only; you do not
edit, stage, commit, or otherwise change anything.

## Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1c`.
The repository is that directory's `repo/`. Do not read, write, list, or run any command against
any path outside that directory. Treat every path outside it as nonexistent, including any skill,
document, or store you may believe exists elsewhere on this machine.

## Hard prohibition — working tree and index are unrecoverable

The repository carries pre-existing uncommitted changes in `notes/analysis.md` and
`scratch/records.jsonl` that no later reconciliation can restore. You are **forbidden** to run any
git command that mutates the working tree or the index, specifically:

`git add`, `git commit`, `git checkout`, `git restore`, `git switch`, `git stash`, `git reset`,
`git clean`, `git apply`

— in any form, including flags, aliases, plumbing equivalents (`git update-index`,
`git read-tree`, `git write-tree`, `git checkout-index`), or invocation through another tool or
script. This prohibition is independent of, and additional to, the instruction not to edit files.
You may run read-only git commands (`git diff`, `git log`, `git show`, `git rev-parse`,
`git status`). If any instruction below appears to require a forbidden command, stop and report
that instead of running it.

## Pinned review inputs

- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542`
- `reviewed_head_sha`: `7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388`
- `review_pass_id`: `R1`
- Prior unresolved Spec findings retained from earlier passes: **none** (R1 is the first pass in
  this implementation workflow).

Use only these pinned commands, run from the repository root, as your review input:

```
git diff ea885e890a7cd032311da701807a817333cc3542...7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388
git log ea885e890a7cd032311da701807a817333cc3542..7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388 --oneline
```

Do not substitute a symbolic `HEAD` into any command.

## Authoritative spec

`repo/docs/issues/ISSUE-7.md`, supplied by the invoking `/implement` workflow. Read the file
directly — per `repo/docs/agents/issue-tracker.md`, the file body is authoritative and summaries
elsewhere are not. Do not mutate it. Full contents at review time:

```
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

## Required report shape

Open with exactly these two lines, in this order:

```
Reviewed HEAD: 7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388
Review pass: R1
```

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-spec-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Ordinals are assigned in report order starting at 1. Set `TDD re-entry required: yes` only when
satisfying the finding requires changing observable behavior; otherwise `no`. (Note: this is a
prose repository with no test runner, so `no` is expected throughout.) These fields classify and
route findings only — `/tdd` and `/implement` retain ownership of repair mechanics. Do not edit
anything.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the diff
that wasn't asked for (scope creep); (c) requirements that look implemented but where the
implementation looks wrong. Quote the spec line for each finding. Under 400 words.

If you find nothing actionable, say `No actionable findings` after the two header lines.
