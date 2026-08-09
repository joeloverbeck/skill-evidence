You are the **Spec** axis reviewer for `/code-review`, pass `R1`. Review only; never edit any
file, never move `HEAD`, never run a mutating git command.

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t4`.
The repository is that directory's `repo/`. Treat every path outside the trial directory as
nonexistent.

## Pinned endpoints

- `fixed_point_sha` = `86b9a1eb58edaa3726926c197b87570167f6084b` (supplied by the invoking
  workflow: the user named the pre-edit `HEAD` of `repo` as the review fixed point)
- `reviewed_head_sha` = `1975f2f1613582a22f45bf34688643e1bda0c11d`

Build every review input from those captured SHAs, run from the repository root:

```
git diff 86b9a1eb58edaa3726926c197b87570167f6084b...1975f2f1613582a22f45bf34688643e1bda0c11d
git log 86b9a1eb58edaa3726926c197b87570167f6084b..1975f2f1613582a22f45bf34688643e1bda0c11d --oneline
```

Pinned commit list (the entire review range):

```
1975f2f Document the Widget service retry policy
```

Pinned full diff (the entire review range; `docs/guide.md` is the only changed path):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index d45a51c..d49ce52 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,13 +2,20 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried; see
+[Retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.

+## Retry policy
+
+A failed call is retried 3 times. The waits before those retries are 1s, 4s, and 16s.
+
+A `409 Conflict` is never retried.
+
 ## Support

 Open an issue under `docs/issues/`.
```

## Spec

Provenance: supplied by the invoking workflow. The authoritative spec is the live issue file
`repo/docs/issues/ISSUE-7.md`, read directly per `repo/docs/agents/issue-tracker.md`
("The file body is authoritative; summaries elsewhere are not."). Full contents as read at
`reviewed_head_sha`:

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

The issue carries no `## Comments` section at `reviewed_head_sha`.

## Report format

Open your report with exactly these lines, in this order:

```
Reviewed HEAD: 1975f2f1613582a22f45bf34688643e1bda0c11d
Review pass: R1
```

Prior unresolved Spec findings carried into this pass: none (R1 is the first pass).

Every actionable finding must carry, in this order:

- `Finding ID: R1-spec-<ordinal>` (ordinal assigned in axis-report order; IDs are immutable
  once emitted)
- the finding text
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` (`yes` only when satisfying the finding requires changing
  observable behavior)

These fields classify and route findings only. You must not edit anything; `/tdd` and
`/implement` retain ownership of red → green and repair mechanics.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the
diff that wasn't asked for (scope creep); (c) requirements that look implemented but where the
implementation looks wrong. Quote the spec line for each finding. Under 400 words.
