# Packet — /code-review pass R1, Spec axis

Dispatch target: isolated Spec reviewer sub-agent (read-only; reviewers must not edit).
Repository root: `<trial>/repo` (the `widget-docs` prose repository).

## Pinned review inputs

- `fixed_point_sha`: `1898fc8f729e81e4d0971cdd0875336cfee8386d` (supplied by the invoking
  implementation workflow as "the current `HEAD` of the repository before any edit").
- `reviewed_head_sha`: `963bd65c0e9e9e327e12040f37a721a4cab63133`
- Pinned full diff command (three-dot, run from the repository root):
  `git diff 1898fc8f729e81e4d0971cdd0875336cfee8386d...963bd65c0e9e9e327e12040f37a721a4cab63133`
- Pinned commit list command:
  `git log 1898fc8f729e81e4d0971cdd0875336cfee8386d..963bd65c0e9e9e327e12040f37a721a4cab63133 --oneline`
- Pinned commit list (captured):
  - `963bd65 Document the Widget service retry policy`

Use only these pinned inputs. Do not substitute a symbolic `HEAD` anywhere.

## Review pass identity

- `review_pass_id`: `R1`
- Prior unresolved Spec findings retained from earlier passes: **none** (R1 is the first pass in
  this implementation workflow).

## The diff under review (captured from the pinned command)

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..7d90e4f 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,7 +2,14 @@
 
 ## Calling the service
 
-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried under the
+[retry policy](#retry-policy).
+
+## Retry policy
+
+A failed call is retried up to 3 times. The waits between attempts are 1s, 4s, and 16s.
+
+A `409 Conflict` is never retried.
 
 ## Errors
```

## Spec source

Provenance: supplied by the invoking implementation workflow (the work is "implement ISSUE-7"),
and corroborated by the commit message trailer `Refs ISSUE-7`. Read directly from its file per
`docs/agents/issue-tracker.md` ("Read an issue by reading its file directly"; "The file body is
authoritative"). Path: `docs/issues/ISSUE-7.md`. Full contents at `reviewed_head_sha`:

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

Scope note for your judgement: the issue authorizes changes to `docs/guide.md` only. The
working tree also carries two pre-existing unrelated modifications (`notes/analysis.md`,
`scratch/records.jsonl`) that were deliberately left unstaged and are absent from the pinned
review range; treat their absence as correct, not as a missing requirement.

## Required report format

Open the report with these two lines, in this order:

```
Reviewed HEAD: 963bd65c0e9e9e327e12040f37a721a4cab63133
Review pass: R1
```

Every actionable finding must carry, on its own lines:

- `Finding ID: R1-spec-<ordinal>` (ordinal assigned in axis-report order; IDs are immutable)
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` (`yes` only when satisfying the finding requires changing
  observable behavior; note that this prose repository has no TDD harness)

These fields classify and route only. Do not edit any file; `/tdd` and `/implement` own repair
mechanics.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the
diff that wasn't asked for (scope creep); (c) requirements that look implemented but where the
implementation looks wrong. Quote the spec line for each finding. Under 400 words.
