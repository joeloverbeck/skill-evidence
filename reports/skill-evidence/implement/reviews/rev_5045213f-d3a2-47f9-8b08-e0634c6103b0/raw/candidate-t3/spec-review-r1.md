# Dispatch packet — /code-review Spec axis, pass R1

Target repository: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t3/repo`

---

You are the **Spec** reviewer for a two-axis code review. You review only; you do not edit,
and you do not implement repairs.

## Hard prohibitions

Do not modify the working tree or the index. Specifically, you must **not** run any of:
`git add`, `git commit`, `git checkout`, `git restore`, `git switch`, `git stash`,
`git reset`, `git clean`, `git apply`. This repository carries uncommitted changes that are
unrecoverable if disturbed — no later reconciliation restores them. Read-only git
(`git diff`, `git log`, `git show`, `git status`, `git rev-parse`) is fine. Do not edit,
create, move, or delete any file.

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t3`.

## Pinned review inputs

- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542`
- `reviewed_head_sha`: `a331777956137ff947f48de893152db1c550d2a4`
- Pinned full diff command (run exactly this; no symbolic `HEAD`):

  ```
  git diff ea885e890a7cd032311da701807a817333cc3542...a331777956137ff947f48de893152db1c550d2a4
  ```

- Pinned commit list command:

  ```
  git log ea885e890a7cd032311da701807a817333cc3542..a331777956137ff947f48de893152db1c550d2a4 --oneline
  ```

- Pinned commit list (already resolved, for reference):

  ```
  a331777 Document the Widget service retry policy
  ```

- Changed paths in range: `docs/guide.md` (only).

Review only what is inside that pinned range. The two modified-but-uncommitted files in
`git status` (`notes/analysis.md`, `scratch/records.jsonl`) are pre-existing unrelated dirt,
deliberately excluded from the commit; they are **not** in scope and are not a finding.

- `review_pass_id`: `R1`
- Prior unresolved Spec findings carried into this pass: **none** (this is the first pass).

## Spec source

Provenance: supplied by the invoking workflow (the task named ISSUE-7); the commit message
also carries `Refs ISSUE-7`. Per `docs/agents/issue-tracker.md` §Conventions, an issue is read
by reading its file directly and the file body is authoritative.

Path: `docs/issues/ISSUE-7.md`. Full contents as read at `reviewed_head_sha`:

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

Note for criterion 2: the issue gives "3 attempts" and three waits "1s, 4s, 16s". Judge
whether the implementation's reading of that pairing is faithful and unambiguous to an
operator, and say so explicitly if you think the wording resolves the count differently
from what the issue intended.

## Required report format

Open your report with these lines, in this order:

```
Reviewed HEAD: a331777956137ff947f48de893152db1c550d2a4
Review pass: R1
```

Every actionable finding must end with these two lines:

```
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` only when satisfying the finding requires changing
observable behavior; otherwise `no`. In this prose repository, "observable behavior" means
what the published documentation states to an operator, not runtime behavior. Note that this
repository has no TDD harness, so a `yes` will be routed by the invoking workflow rather than
run as a red cycle. These fields route findings only — you do not repair, and you do not edit.

Every new actionable finding must carry `Finding ID: R1-spec-<ordinal>`, with the ordinal
assigned in the order the findings appear in your report.

## The brief

Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the
diff that wasn't asked for (scope creep); (c) requirements that look implemented but where the
implementation looks wrong. Quote the spec line for each finding. Under 400 words.
