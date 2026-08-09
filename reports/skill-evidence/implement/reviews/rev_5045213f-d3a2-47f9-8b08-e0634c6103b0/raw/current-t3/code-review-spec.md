# Spec axis reviewer — /code-review pass R1

You are the **Spec** axis reviewer for a two-axis review. You review only; you must not edit any
file, stage anything, or move `HEAD`.

## Repository

`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t3/repo`

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t3`.
Treat every path outside it as nonexistent.

## Pinned endpoints

- `fixed_point_sha`: `3e70c4486bb871d442973448aac358603097244c` (supplied by the invoking
  implementation workflow as the pre-edit `HEAD`)
- `reviewed_head_sha`: `7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc`

Build every review input from these captured SHAs only. Do not leave a symbolic `HEAD` in any
command.

Pinned full diff command:

```
git diff 3e70c4486bb871d442973448aac358603097244c...7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc
```

Pinned commit list command and its result:

```
git log 3e70c4486bb871d442973448aac358603097244c..7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc --oneline
7a82a3c Document the Widget service retry policy
```

The pinned diff, inlined so this prompt is self-contained (run the command yourself as well):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..ff64752 100644
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
+A failed call is retried up to 3 times. The waits before the first, second, and third
+retry are 1s, 4s, and 16s.
+
+A `409 Conflict` is never retried.
```

## Review pass

`review_pass_id`: **R1**

Prior unresolved Spec findings retained from earlier passes: **none** (R1 is the first pass of
this implementation workflow).

## Spec source

Provenance: supplied by the invoking implementation workflow (the task names ISSUE-7), and
corroborated by the commit message trailer `Refs ISSUE-7`. Fetched per
`docs/agents/issue-tracker.md` §Conventions — issues are Markdown files under `docs/issues/`,
read directly, and the file body is authoritative.

Path: `docs/issues/ISSUE-7.md`. Contents at `reviewed_head_sha`:

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

Open your report with these two lines, in this order:

```
Reviewed HEAD: 7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc
Review pass: R1
```

Every actionable finding must carry a stable identity and both routing fields:

- `Finding ID: R1-spec-<ordinal>`, ordinal assigned in axis-report order.
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` — `yes` only when satisfying the finding requires changing
  observable behavior.

These fields classify and route findings only. You must not edit; `/tdd` and `/implement` retain
ownership of red → green and repair mechanics.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the
diff that wasn't asked for (scope creep); (c) requirements that look implemented but where the
implementation looks wrong. Quote the spec line for each finding. Under 400 words.
