# Dispatch packet — /code-review Standards axis, pass R1

Target repository: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t3/repo`

---

You are the **Standards** reviewer for a two-axis code review. You review only; you do not
edit, and you do not implement repairs.

## Hard prohibitions

Do not modify the working tree or the index. Specifically, you must **not** run any of:
`git add`, `git commit`, `git checkout`, `git restore`, `git switch`, `git stash`,
`git reset`, `git clean`, `git apply`. This repository carries uncommitted changes that
are unrecoverable if disturbed — no later reconciliation restores them. Read-only git
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
- Prior unresolved Standards findings carried into this pass: **none** (this is the first pass).

## Applicable standards sources (selected for this run — apply exactly these)

### 1. `CLAUDE.md` — root agent instructions (full text)

```markdown
# widget-docs

Documentation repository for the Widget service. Prose only; there is no build system,
no test runner, and no type checker.

## Verification surface

The repository's native checks are structural, run from the repository root:

- `scripts/check-links.sh` — every relative Markdown link resolves.
- `scripts/check-headings.sh` — every doc page starts with a single H1.

Run both before any commit that touches `docs/`.

## Conventions

- One topic per page. Do not duplicate a rule that already has a home elsewhere.
- Reference pages by relative path.
```

### 2. Active invoking-workflow contract (`/implement`) — the clauses that bind this diff

- Only the scoped implementation may be in `HEAD`; unrelated pre-existing dirt must remain
  unstaged and unaltered.
- The repository's native verification surface must be discovered and used rather than a
  generic build/test assumption. In prose repositories, that means the repository-named
  structural checks.
- `/tdd` does not apply here: this repository has no TDD harness, so no requirement is
  expected to point at a TDD evidence row.

### 3. Caller-selected run sheet (external-only evidence, not committed) — agreed seams and verification

| # | Acceptance criterion | Agreed implementation seam | Agreed verification |
|---|---|---|---|
| 1 | `docs/guide.md` gains a `## Retry policy` section. | New `## Retry policy` section in `docs/guide.md`, between `## Calling the service` and `## Errors`. | `grep -n '^## Retry policy' docs/guide.md`; `check-headings.sh` shows no new failure. |
| 2 | States the retry count (3 attempts) and backoff (1s, 4s, 16s). | Body of the new section. | Read-back; `3`, `1s`, `4s`, `16s` present and mapped to attempts. |
| 3 | States that a `409 Conflict` is never retried. | Body of the new section; kept out of `## Errors` for single-home. | Read-back; `grep -n '409' docs/guide.md`. |
| 4 | The "are retried" sentence links to the new section instead of restating numbers. | Rewrite that sentence with `[Retry policy](#retry-policy)` and no numbers. | Read-back; `check-links.sh` stays OK. |

Recorded verification results on `reviewed_head_sha`:

- `bash scripts/check-links.sh` → `check-links: OK`, exit 0.
- `bash scripts/check-headings.sh` → `docs/skills/code-review.md has 0 H1 headings`, exit 1.
  This failure is **pre-existing at `fixed_point_sha`** (recorded identically before any
  edit) and concerns a file outside this diff. The gate applied was "no new failure line".
  Judge whether that disposition is defensible under `CLAUDE.md` §Verification surface
  ("Run both before any commit that touches `docs/`"); do not treat repairing the unrelated
  file as an expected part of this change.

### 4. Smell baseline (always applies — reproduced in full)

A fixed set of Fowler code smells (_Refactoring_, ch.3) that applies even when a repo
documents nothing. Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses
  something the baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature Envy"),
  never a hard violation — and, like any standard here, skip anything tooling already enforces.

Each smell reads *what it is* → *how to fix*; match it against the diff:

- **Mysterious Name** — a function, variable, or type whose name doesn't reveal what it does or holds. → rename it; if no honest name comes, the design's murky.
- **Duplicated Code** — the same logic shape appears in more than one hunk or file in the change. → extract the shared shape, call it from both.
- **Feature Envy** — a method that reaches into another object's data more than its own. → move the method onto the data it envies.
- **Data Clumps** — the same few fields or params keep travelling together (a type wanting to be born). → bundle them into one type, pass that.
- **Primitive Obsession** — a primitive or string standing in for a domain concept that deserves its own type. → give the concept its own small type.
- **Repeated Switches** — the same `switch`/`if`-cascade on the same type recurs across the change. → replace with polymorphism, or one map both sites share.
- **Shotgun Surgery** — one logical change forces scattered edits across many files in the diff. → gather what changes together into one module.
- **Divergent Change** — one file or module is edited for several unrelated reasons. → split so each module changes for one reason.
- **Speculative Generality** — abstraction, parameters, or hooks added for needs the spec doesn't have. → delete it; inline back until a real need shows.
- **Message Chains** — long `a.b().c().d()` navigation the caller shouldn't depend on. → hide the walk behind one method on the first object.
- **Middle Man** — a class or function that mostly just delegates onward. → cut it, call the real target direct.
- **Refused Bequest** — a subclass or implementer that ignores or overrides most of what it inherits. → drop the inheritance, use composition.

Note: this diff is prose Markdown, so several structural smells will not apply. Say so
rather than forcing a match. Duplicated Code (a rule restated in two places) and
Divergent Change are the live ones here, and they interact with `CLAUDE.md`'s
"Do not duplicate a rule that already has a home elsewhere".

## Required report format

Open your report with these lines, in this order:

```
Reviewed HEAD: a331777956137ff947f48de893152db1c550d2a4
Review pass: R1
Standards sources checked: CLAUDE.md; /implement contract (invoking workflow); ISSUE-7 run sheet (caller-selected, external); smell baseline
Workflow evidence checked: ISSUE-7 run sheet -> requirement rows 1-4 (agreed seams and structural check commands); /tdd -> N/A (no TDD harness in this repository)
```

Both coverage lines are owed even if you have no actionable findings.

Every actionable finding must end with these two lines:

```
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` only when satisfying the finding requires changing
observable behavior; otherwise `no`. In this prose repository, "observable behavior" means
what the published documentation states to an operator, not runtime behavior. These fields
route findings only — you do not repair, and you do not edit.

Every new actionable finding must carry `Finding ID: R1-standards-<ordinal>`, with the
ordinal assigned in the order the findings appear in your report.

## The brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether changed
tests and evidence rows honor the agreed seams, runnable commands, and evidence fields: cite
the workflow source and quote the changed hunk or row. Distinguish hard violations from
judgement calls — documented-standard breaches can be hard, but baseline smells are always
judgement calls, and a documented repo standard overrides the baseline. Skip anything tooling
enforces. Under 400 words.
