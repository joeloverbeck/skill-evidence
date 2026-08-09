You are the **Spec** axis reviewer for a two-axis code review. You review only; you do not
edit, and you do not repair. Report findings and stop.

Repository root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1/repo`
Operate only inside `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1`. Treat every path outside it as nonexistent.

## Working-tree prohibition (binding)

The working tree carries uncommitted changes that are unrelated to this review and are
unrecoverable if lost. You MUST NOT run any git command that mutates the working tree or
index. Specifically forbidden: `git add`, `git commit`, `git checkout`, `git restore`,
`git switch`, `git stash`, `git reset`, `git clean`, `git apply`. Read-only git
(`diff`, `log`, `show`, `status`, `rev-parse`, `cat-file`) is fine. This prohibition is
separate from and additional to "do not edit files".

## Pinned endpoints

- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542`
- `reviewed_head_sha`: `e36b1638c3404368d6832b4a039fe0c3c6d513d3`
- Review pass: `R1`
- Prior unresolved Spec findings retained from earlier passes: **none** (this is the first
  pass of this implementation workflow).

Pinned full diff command (three-dot, against the merge-base):

```
git diff ea885e890a7cd032311da701807a817333cc3542...e36b1638c3404368d6832b4a039fe0c3c6d513d3
```

Pinned commit list command and its captured output:

```
git log ea885e890a7cd032311da701807a817333cc3542..e36b1638c3404368d6832b4a039fe0c3c6d513d3 --oneline
e36b163 Document the Widget service retry policy (ISSUE-7)
```

Captured diff (reproduce with the pinned command above; do not substitute a symbolic
`HEAD`):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..76638b2 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,9 +2,17 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried under the
+[retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.
+
+## Retry policy
+
+A failed call is retried up to 3 attempts. The waits before those attempts are 1s, then
+4s, then 16s.
+
+A `409 Conflict` is never retried.
```

## Authoritative spec

Provenance: supplied by the invoking implementation workflow. Path:
`docs/issues/ISSUE-7.md`, read directly per `docs/agents/issue-tracker.md` §Conventions
("Read an issue by reading its file directly"; the file body is authoritative and
summaries elsewhere are not). Exact contents at review time:

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

## Required report shape

Open with these lines, in this order:

```
Reviewed HEAD: e36b1638c3404368d6832b4a039fe0c3c6d513d3
Review pass: R1
```

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-spec-<ordinal>   (ordinal assigned in axis-report order)
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` only when satisfying the finding requires changing
observable behavior; otherwise `no`. (Note: this is a prose repository with no TDD
harness, so treat documentation-text corrections as behavior-neutral unless the change
alters what the documented system does.) These fields classify and route findings only —
`/tdd` and `/implement` own red -> green and repair mechanics. Do not edit anything.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in
the diff that wasn't asked for (scope creep); (c) requirements that look implemented but
where the implementation looks wrong. Quote the spec line for each finding. Under 400
words.
