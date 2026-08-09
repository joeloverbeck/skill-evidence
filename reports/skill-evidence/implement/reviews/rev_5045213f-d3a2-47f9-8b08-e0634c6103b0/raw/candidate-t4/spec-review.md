# Dispatch packet — /code-review Spec axis (review pass R1)

Sub-agent type: general-purpose (read-only review; you must not edit any file).

---

You are the **Spec** reviewer for a two-axis code review. Review only; make no edits.

## Environment boundary

Operate only inside `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t4`. The repository is
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t4/repo`. Do not read, write, list, or run any command against any path
outside that directory. Treat every path outside it as nonexistent, including any skill,
document, or store you may believe exists elsewhere on this machine.

## Working-tree prohibition (binding, no exceptions)

The working tree carries pre-existing uncommitted changes that are unrelated to this work
and are **unrecoverable if lost** (`notes/analysis.md`, `scratch/records.jsonl`). You must
not run any git command that mutates the working tree or the index. Specifically these are
forbidden: `git add`, `git commit`, `git checkout`, `git restore`, `git switch`,
`git stash`, `git reset`, `git clean`, `git apply`. This prohibition is separate from and
additional to the instruction not to edit files — do not treat "I won't edit files" as
covering it. Read-only git (`git diff`, `git log`, `git show`, `git rev-parse`,
`git status`) is fine.

## Pinned endpoints

- `fixed_point_sha`: `ba2d824bbbfd7a55ce31f990cfbd975f0049f3aa`
- `reviewed_head_sha`: `66128e4c8c3b4df02c57aef0f3b24bd7686863f9`
- `review_pass_id`: `R1`
- Prior unresolved Spec findings retained from earlier passes: **none** (first pass).

Pinned commands — build every review input from these captured SHAs only:

```
git diff ba2d824bbbfd7a55ce31f990cfbd975f0049f3aa...66128e4c8c3b4df02c57aef0f3b24bd7686863f9
git log ba2d824bbbfd7a55ce31f990cfbd975f0049f3aa..66128e4c8c3b4df02c57aef0f3b24bd7686863f9 --oneline
```

Pinned commit list:

```
66128e4 Document the Widget service retry policy (ISSUE-7)
```

Pinned full diff (reproduced so this packet is self-contained):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index d45a51c..3fc72cb 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,13 +2,21 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried under the
+[retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.

+## Retry policy
+
+A failed call gets 3 retry attempts. The wait before each attempt follows a fixed backoff
+schedule: 1s, then 4s, then 16s.
+
+A `409 Conflict` is never retried.
+
 ## Support

 Open an issue under `docs/issues/`.
```

## Authoritative spec

Provenance: supplied by the invoking implementation workflow. Path:
`repo/docs/issues/ISSUE-7.md`. Per `repo/docs/agents/issue-tracker.md`, the issue file body
is authoritative and summaries elsewhere are not; read the file directly if you need more
than the reproduction below. Contents at dispatch time:

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

## Required report format

Open your report with these lines, in this order:

```
Reviewed HEAD: 66128e4c8c3b4df02c57aef0f3b24bd7686863f9
Review pass: R1
```

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-spec-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Assign ordinals in axis-report order. Set `TDD re-entry required: yes` only when
satisfying the finding requires changing observable behavior; otherwise `no`. These fields
classify and route findings only — you must not edit, and `/tdd` and `/implement` retain
ownership of red → green and repair mechanics. Note for this repository: it is prose-only
with no test runner, so no TDD harness exists; classify honestly anyway.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in
the diff that wasn't asked for (scope creep); (c) requirements that look implemented but
where the implementation looks wrong. Quote the spec line for each finding. Under 400
words.
