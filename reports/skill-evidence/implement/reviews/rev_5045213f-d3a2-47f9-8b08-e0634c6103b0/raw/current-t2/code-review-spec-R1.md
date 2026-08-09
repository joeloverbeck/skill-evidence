# /code-review Spec axis reviewer packet — review pass R1

Dispatch target: isolated Spec reviewer sub-agent (launched in parallel with the Standards
reviewer, `packets/code-review-standards-R1.md`).
Dispatched by: `/code-review` (`repo/docs/skills/code-review.md`) step 4, embedded in the ISSUE-7
implementation workflow.
`review_pass_id`: **R1** (first allocated pass in this workflow; never reused).
Prior unresolved Spec findings retained from earlier passes: **none — R1 is the first pass.**
Mode: **read-only**. Reviewers must not edit. Report findings only; no file writes, and no git
command that mutates the repository, index, or working tree. The working tree carries pre-existing
unrelated modifications (`notes/analysis.md`, `scratch/records.jsonl`) that must survive untouched.

---

## Prompt as dispatched

You are the **Spec** axis reviewer. Review only; change nothing.

### Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t2`.
The repository is
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t2/repo`.
Do not read, write, list, or run any command against any path outside that directory. Treat every
path outside it as nonexistent, including any skill, document, or store you may believe exists
elsewhere on this machine. Use absolute paths. Run no mutating git command
(`add`, `commit`, `checkout`, `restore`, `stash`, `clean`, `reset`, `amend`).

### Pinned endpoints

- `fixed_point_sha`: `1898fc8f729e81e4d0971cdd0875336cfee8386d` (supplied by the invoking
  workflow: the repository `HEAD` before any edit)
- `reviewed_head_sha`: `941aabbad333dfacb3fd5b495f9ac377325bd4d3`

Pinned commands (run from the repository root):

```
git diff 1898fc8f729e81e4d0971cdd0875336cfee8386d...941aabbad333dfacb3fd5b495f9ac377325bd4d3
git log 1898fc8f729e81e4d0971cdd0875336cfee8386d..941aabbad333dfacb3fd5b495f9ac377325bd4d3 --oneline
```

Pinned commit list:

```
941aabb Document the Widget service retry policy (ISSUE-7)
```

Pinned full diff:

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..923419c 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,9 +2,19 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried; see
+[Retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.
+
+## Retry policy
+
+A failed call gets up to 3 retry attempts. The wait before each attempt follows a fixed
+backoff schedule: 1s, then 4s, then 16s. A call that fails on the third attempt is
+reported as failed and is not retried again.
+
+A `409 Conflict` is never retried. The widget key already exists, so repeating the call
+cannot change the outcome.
```

### Authoritative spec

Provenance: supplied by the invoking workflow. Path: `repo/docs/issues/ISSUE-7.md`. Per
`repo/docs/agents/issue-tracker.md`, the issue **file body is authoritative** and summaries
elsewhere are not — read the file itself; the copy below is for convenience only and must not
substitute for the exact read.

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

Note for criterion 2: the spec words the count as "the retry count (3 attempts)" and gives a
three-step schedule (1s, 4s, 16s). Judge the implementation against what the spec states, and
call out any place the diff's phrasing over-specifies, under-specifies, or silently reinterprets
those numbers (for example, ambiguity about whether the initial call counts toward the three).

Note for criterion 4: this is a single-home requirement — the sentence must link rather than
restate the numbers. An independent delegated investigation of how the whole `docs/` tree
currently describes retries was run for this criterion (packet
`packets/retry-docs-investigation.md`) and returned no actionable findings, i.e. no page outside
`docs/guide.md` restates retry counts, waits, or eligibility. You are not being asked to redo that
census; judge the diff.

### Report format (required)

Open with these lines, in this order:

```
Reviewed HEAD: 941aabbad333dfacb3fd5b495f9ac377325bd4d3
Review pass: R1
```

Every actionable finding must carry:

- `Finding ID: R1-spec-<ordinal>` (ordinal assigned in axis-report order; IDs are immutable)
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` (`yes` only when satisfying the finding requires changing
  observable behavior; note this repository has no TDD harness and no tests, so `no` is expected
  unless you can name the behavior that would need a red cycle)

These fields classify and route only. Do not edit anything; `/tdd` and `/implement` retain
ownership of red → green and repair mechanics.

### Brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the diff
that wasn't asked for (scope creep); (c) requirements that look implemented but where the
implementation looks wrong. Quote the spec line for each finding. Under 400 words.
