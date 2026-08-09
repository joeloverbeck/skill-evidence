# Dispatch packet — /code-review Spec axis (review pass R1)

Dispatch target: isolated Spec reviewer sub-agent, launched in parallel with the Standards
reviewer (`packets/code-review-standards.md`). Sub-agent surface unavailable in this
environment; packet composed for dispatch and recorded here instead.

---

You are the **Spec** reviewer for a two-axis code review. You review only; you must not
edit any file, stage anything, or move `HEAD`.

## Pinned endpoints

- `fixed_point_sha` = `ea885e890a7cd032311da701807a817333cc3542`
- `reviewed_head_sha` = `37aa66cdd2894c57e75020092b94fd19e633fe72`

Repository root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t5/repo`

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t5`.
Treat every path outside it as nonexistent.

Build every review input from the captured SHAs above. Do not use a symbolic `HEAD` in any
command.

Pinned full diff command:

```
git diff ea885e890a7cd032311da701807a817333cc3542...37aa66cdd2894c57e75020092b94fd19e633fe72
```

Pinned commit list command:

```
git log ea885e890a7cd032311da701807a817333cc3542..37aa66cdd2894c57e75020092b94fd19e633fe72 --oneline
```

Pinned commit list (captured):

```
37aa66c Document the Widget service retry policy
```

Pinned full diff (captured — this is the entire change under review):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..a270b21 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,9 +2,17 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried — see
+[Retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.
+
+## Retry policy
+
+A failed call is retried up to 3 times. The waits before those retries are 1s, 4s, and
+16s.
+
+A `409 Conflict` is never retried.
```

## Spec source

Provenance: the commit message says `Refs ISSUE-7`. Fetched per
`repo/docs/agents/issue-tracker.md` §Conventions, which states that issues are Markdown
files under `docs/issues/`, that the file body is authoritative, that summaries elsewhere
are not, and that an issue is read by reading its file directly.

Path: `repo/docs/issues/ISSUE-7.md`. Read it directly; the copy below is provided for
self-containment, and the file at that path governs if they ever differ.

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
Reviewed HEAD: 37aa66cdd2894c57e75020092b94fd19e633fe72
Review pass: R1
```

Prior unresolved Spec findings carried into this pass: **none** (R1 is the first pass).

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-spec-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Assign ordinals in axis-report order, starting at 1. Set `TDD re-entry required: yes`
whenever satisfying the finding requires changing observable behavior; otherwise `no`. For
this prose repository, "observable behavior" means what the published documentation states
to a reader. These fields classify and route findings only — `/tdd` and `/implement` retain
ownership of red → green and repair mechanics. Do not edit anything.

## Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in
the diff that wasn't asked for (scope creep); (c) requirements that look implemented but
where the implementation looks wrong. Quote the spec line for each finding. Under 400
words.

Specific things worth your attention, none of which are conclusions:

- Criterion 2 says "the retry count (3 attempts)". The diff writes "retried up to 3 times"
  and lists three waits. Judge whether "3 attempts" in the issue means three retries after
  the initial call (which is what the diff states) or three calls in total including the
  initial one (which the diff would then contradict), and whether the wording as shipped
  could mislead the operators the issue says have been guessing.
- Criterion 4 says the sentence must link "instead of restating the numbers". Judge whether
  the shipped sentence restates anything it shouldn't, and whether the anchor
  `#retry-policy` actually resolves to the `## Retry policy` heading.
- The issue's front matter carries `status: open`. No tracker mutation is in this diff. The
  implementer has not been authorized to close the issue, so absence of a closeout change
  is intended, not an omission — do not raise it as a missing requirement.
