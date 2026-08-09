You are the **Spec** axis reviewer for a two-axis code review. Review only. Do not edit any
file, do not stage, commit, or run any tree-mutating git command.

# Pinned review inputs

- Repository root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t1/repo`
- `fixed_point_sha`: `4e3fe143d7a7ddf754b7aa166c18b57dbf396278` (supplied by the invoking
  workflow as the pre-edit `HEAD`)
- `reviewed_head_sha`: `492ebc6371042e8f9c54562ca669435cc1ee9b9a`
- `review_pass_id`: `R1`
- Prior unresolved Spec findings retained from earlier passes: **none** (this is the first
  pass of this implementation workflow).

Pinned full diff command (three-dot, both endpoints captured as SHAs — do not substitute a
symbolic `HEAD`):

```
git diff 4e3fe143d7a7ddf754b7aa166c18b57dbf396278...492ebc6371042e8f9c54562ca669435cc1ee9b9a
```

Pinned commit list command:

```
git log 4e3fe143d7a7ddf754b7aa166c18b57dbf396278..492ebc6371042e8f9c54562ca669435cc1ee9b9a --oneline
```

Pinned commit list (already resolved):

```
492ebc6 Document the Widget service retry policy
```

Pinned diff (already resolved; reproduced so this packet is self-contained):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..e768e28 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,7 +2,15 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried under the
+[retry policy](#retry-policy).
+
+## Retry policy
+
+A failed call is retried up to 3 times. The waits before those retries are 1s, 4s, and
+16s.
+
+A `409 Conflict` is never retried.

 ## Errors
```

# Spec source

Provenance: supplied by the invoking workflow. The authoritative spec is the live issue file
`docs/issues/ISSUE-7.md`, read directly per `docs/agents/issue-tracker.md` §Conventions
("The file body is authoritative; summaries elsewhere are not"). Exact contents at review
time:

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

For context, the full post-change state of the changed file (`docs/guide.md` at
`reviewed_head_sha`):

```markdown
# Widget service guide

## Calling the service

Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried under the
[retry policy](#retry-policy).

## Retry policy

A failed call is retried up to 3 times. The waits before those retries are 1s, 4s, and
16s.

A `409 Conflict` is never retried.

## Errors

The service returns `400` for a malformed body, `409 Conflict` when a widget with the
same key already exists, and `503` when the upstream store is unavailable.
```

One reading the implementer made explicit, which you should adjudicate: AC2 says "the retry
count (3 attempts)" alongside a three-step backoff schedule (1s, 4s, 16s). The implementation
reads this as three retry attempts after the initial call, each preceded by one of the three
waits, so both numbers stay consistent. If you believe the spec instead means three total
attempts (and therefore only two waits), say so as a finding.

# Required report shape

Open your report with these lines, in this order:

```
Reviewed HEAD: 492ebc6371042e8f9c54562ca669435cc1ee9b9a
Review pass: R1
```

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-spec-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Ordinals are assigned in the order findings appear in your report. Set `TDD re-entry
required: yes` only when satisfying the finding requires changing observable behavior;
otherwise `no`. Note that `/tdd` does not apply in this prose repository, so a `yes` here
routes to the invoking workflow's documented no-harness substitute. These fields classify and
route findings only — you must not edit anything.

# Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the
diff that wasn't asked for (scope creep); (c) requirements that look implemented but where
the implementation looks wrong. Quote the spec line for each finding. Under 400 words.
