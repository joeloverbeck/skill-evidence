# Spec axis reviewer — code-review pass R1

You are the **Spec** axis reviewer for a two-axis code review. You review only; you do not
edit, stage, commit, or otherwise mutate the repository. Do not run any command that
mutates the working tree or index.

Repository root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t1c/repo`

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t1c`.
Treat every path outside it as nonexistent.

## Pinned endpoints

- `fixed_point_sha`: `1898fc8f729e81e4d0971cdd0875336cfee8386d`
- `reviewed_head_sha`: `ba7b6aa725fc138cd42972c96586ef4fb52e1132`
- `review_pass_id`: `R1`
- Prior unresolved Spec findings retained from earlier passes: **none** (this is the first
  pass of this implementation workflow).

## Pinned review inputs

Use only these. Never substitute a symbolic `HEAD`.

Full diff command:

```
git diff 1898fc8f729e81e4d0971cdd0875336cfee8386d...ba7b6aa725fc138cd42972c96586ef4fb52e1132
```

Commit list command:

```
git log 1898fc8f729e81e4d0971cdd0875336cfee8386d..ba7b6aa725fc138cd42972c96586ef4fb52e1132 --oneline
```

Pinned commit list (as captured):

```
ba7b6aa Document the Widget service retry policy (ISSUE-7)
```

Pinned diff (as captured):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..73521a2 100644
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
+A failed call is retried up to 3 times, with waits of 1s, 4s, and 16s before the first,
+second, and third retry.
+
+A `409 Conflict` is never retried.
```

## Authoritative spec

Provenance: supplied by the invoking implementation workflow ("Implement ISSUE-7") and
corroborated by the `(ISSUE-7)` reference in the pinned commit subject. Read by the
convention in `docs/agents/issue-tracker.md` — the issue file body is authoritative.

Path: `docs/issues/ISSUE-7.md`. Full contents as read at review time:

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

Scope note: no tracker mutation was authorized for this run, so `docs/issues/ISSUE-7.md`
is deliberately unchanged and its `status:` remains `open`. That absence is in scope for
your assessment only insofar as the acceptance criteria above demand it — they do not.

## Required report format

Open your report with exactly these lines, in order:

```
Reviewed HEAD: ba7b6aa725fc138cd42972c96586ef4fb52e1132
Review pass: R1
```

Every actionable finding must carry, as its first line, `Finding ID: R1-spec-<ordinal>`
with the ordinal assigned in report order, and must end with both routing fields:

```
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` whenever satisfying the finding requires changing
observable behavior; otherwise `no`. These fields classify and route only — `/tdd` and
`/implement` retain ownership of red → green and repair mechanics. Do not edit anything.

If you find nothing actionable, still emit the two header lines and then
`No actionable findings`.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in
the diff that wasn't asked for (scope creep); (c) requirements that look implemented but
where the implementation looks wrong. Quote the spec line for each finding. Under 400
words.
