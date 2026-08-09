# Reviewer packet — Spec axis — review pass R1

You are the **Spec** reviewer for a two-axis code review. You review only. Another agent
reviews the Standards axis independently; do not consider repo coding standards or style,
and do not read that agent's report.

## Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t5`.
The repository is that directory's `repo/`. Treat every path outside the trial directory as
nonexistent, including any skill, document, or store you may believe exists elsewhere on
this machine. Do not search the filesystem outside it.

## You must not modify anything

You are a reviewer. Do not edit, create, or delete any file.

**Additionally, and separately from the no-edit rule above, you are forbidden from running
any git command that mutates the working tree or the index.** Specifically forbidden:
`git add`, `git commit`, `git checkout`, `git restore`, `git switch`, `git stash`,
`git reset`, `git clean`, `git apply`. This prohibition is not implied by "don't edit
files" — it is a separate, independent rule, and it binds even if you believe a mutation
would be harmless, would be reverted, or is needed to inspect something. Uncommitted state
in this repository is unrecoverable and no later reconciliation restores it.

Read-only git (`git show`, `git diff`, `git log`, `git status`, `git rev-parse`,
`git ls-files`) is permitted.

## Pinned review inputs

- `reviewed_head_sha`: `10e38f2351165cb93bdc6436625e87d81eafcdfa`
- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542` (supplied by the invoking
  implementation workflow as the review fixed point)

Build every review input from these captured SHAs. Do not substitute a symbolic `HEAD`.

Pinned full diff command:

```
git -C <trial>/repo diff ea885e890a7cd032311da701807a817333cc3542...10e38f2351165cb93bdc6436625e87d81eafcdfa
```

Pinned commit list command, and its result:

```
git -C <trial>/repo log ea885e890a7cd032311da701807a817333cc3542..10e38f2351165cb93bdc6436625e87d81eafcdfa --oneline
10e38f2 Document the Widget service retry policy (ISSUE-7)
```

Changed paths in range: `docs/guide.md` (only).

## Authoritative spec

Path: `repo/docs/issues/ISSUE-7.md`. Provenance: supplied directly by the invoking
implementation workflow ("Implement ISSUE-7 end-to-end"); do not reopen that selection or
go looking for a competing spec source.

Per `repo/docs/agents/issue-tracker.md` §Conventions, the issue **file body is
authoritative; summaries elsewhere are not**. Read the file directly rather than relying on
any summary, including the copy reproduced here for convenience:

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

Note for criterion 4: judge both halves of it — that the sentence now links to the new
section, **and** that it does not restate the count or the backoff numbers.

## Prior unresolved findings for this axis

None. `R1` is the first review pass of this implementation workflow.

## Report format — required

Open your report with exactly these lines, in this order:

```
Reviewed HEAD: 10e38f2351165cb93bdc6436625e87d81eafcdfa
Review pass: R1
```

Every actionable finding must carry, as its first line:

```
Finding ID: R1-spec-<ordinal>
```

with the ordinal assigned in the order findings appear in your report. Every actionable
finding must end with both of these lines:

```
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` only when satisfying the finding requires changing
observable behavior; otherwise `no`. (This repository is prose-only with no test harness,
so classify honestly rather than assuming one answer.) These fields route findings only —
you do not perform repairs.

If you find nothing actionable, say `No actionable findings` after the two header lines.

## Your brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in
the diff that wasn't asked for (scope creep); (c) requirements that look implemented but
where the implementation looks wrong. Quote the spec line for each finding. Under 400
words.
