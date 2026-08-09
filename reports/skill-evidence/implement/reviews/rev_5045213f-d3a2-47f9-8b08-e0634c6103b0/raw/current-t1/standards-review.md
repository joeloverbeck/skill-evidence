You are the **Standards** axis reviewer for a two-axis code review. Review only. Do not edit
any file, do not stage, commit, or run any tree-mutating git command.

# Pinned review inputs

- Repository root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t1/repo`
- `fixed_point_sha`: `4e3fe143d7a7ddf754b7aa166c18b57dbf396278` (supplied by the invoking
  workflow as the pre-edit `HEAD`)
- `reviewed_head_sha`: `492ebc6371042e8f9c54562ca669435cc1ee9b9a`
- `review_pass_id`: `R1`
- Prior unresolved Standards findings retained from earlier passes: **none** (this is the
  first pass of this implementation workflow).

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

# Selected standards sources for this run

Apply exactly these sources, plus the smell baseline pasted in full below. Do not add or
drop sources; the aggregation gate compares your coverage line against this list in both
directions.

## 1. `CLAUDE.md` (repository root agent instructions) — full text

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

## 2. `docs/agents/issue-tracker.md` — full text

```markdown
# Issue tracker

## Conventions

Issues are Markdown files under `docs/issues/`, named `ISSUE-<n>.md`. The file body is
authoritative; summaries elsewhere are not.

- **Read** an issue by reading its file directly.
- **Comment** by appending a `## Comments` entry to the issue file.
- **Close** by setting the `status:` field in the issue front matter to `closed` and
  appending a closing comment naming the implementation commit SHA.

Never infer an issue's state from anything but an exact read of its file.
```

## 3. The active `/implement` workflow contract (embedded-use standards source)

The invoking workflow binds this implementation. The clauses that constrain the changed
artifacts and the retained evidence:

- Read the repository agent instructions and the authoritative issue body before editing;
  for tracker reads and mutations follow `docs/agents/issue-tracker.md` §Conventions rather
  than copied summaries.
- Record the review fixed point; run unscoped `git status --short`, classify all
  pre-existing changes, preserve unrelated dirt, and keep implementation scope explicit.
- Turn the acceptance criteria into a run sheet mapping each requirement to its
  implementation seam and verification. `/tdd` does not apply here: this is a prose
  repository with no test harness, so each run-sheet row carries its own structural check
  instead of a TDD evidence row.
- Discover the repository's native verification surface before selecting checks; in prose
  or no-build repositories use the checks named by the repository instructions. Record the
  exact structural or manual checks and their results.
- Put only the scoped implementation into `HEAD` before review; unrelated dirt stays
  unstaged.

### Retained evidence for this pass — the external run sheet

This run sheet is deliberately **untracked** (external-only evidence; the contract's
default closeout path forbids a tracked run-sheet or ledger change). It is reproduced here
so you can check it without filesystem access.

| # | Acceptance criterion | Implementation seam | Verification | Result |
|---|---|---|---|---|
| AC1 | `docs/guide.md` gains a `## Retry policy` section | new H2 in `docs/guide.md` between `## Calling the service` and `## Errors` | structural read of the committed file; `scripts/check-headings.sh` (guide keeps exactly one H1) | section present; `grep -cE '^# ' docs/guide.md` = 1 |
| AC2 | Section states retry count (3 attempts) and backoff schedule (1s, 4s, 16s) | body of `## Retry policy` | structural read of the committed file | "retried up to 3 times", "waits before those retries are 1s, 4s, and 16s" |
| AC3 | Section states `409 Conflict` is never retried | body of `## Retry policy` | structural read of the committed file | "A `409 Conflict` is never retried." |
| AC4 | The existing "are retried" sentence in `## Calling the service` links to the new section instead of restating the numbers | in-page anchor link `[retry policy](#retry-policy)` | `scripts/check-links.sh`; structural read confirming no numbers restated at the call site | `check-links: OK`; call-site sentence carries no retry count or waits |

Baseline and post-change verification runs (recorded by the implementer):

- At `fixed_point_sha`: `scripts/check-links.sh` → `check-links: OK` (exit 0);
  `scripts/check-headings.sh` → exit 1, `docs/skills/code-review.md has 0 H1 headings`.
  This heading failure is **pre-existing** and is not touched by the diff.
- At `reviewed_head_sha`: `scripts/check-links.sh` → `check-links: OK` (exit 0);
  `scripts/check-headings.sh` → exit 1 with the identical single pre-existing failure on
  `docs/skills/code-review.md`. No new failure, and no change in the failure set.

## 4. Smell baseline (always applies; pasted in full)

On top of whatever the repo documents, the Standards axis always carries the **smell
baseline** below — a fixed set of Fowler code smells (_Refactoring_, ch.3) that applies even
when a repo documents nothing. Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses
  something the baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature Envy"),
  never a hard violation — and, like any standard here, skip anything tooling already
  enforces.

Each smell reads *what it is* → *how to fix*; match it against the diff:

- **Mysterious Name** — a function, variable, or type whose name doesn't reveal what it does
  or holds. → rename it; if no honest name comes, the design's murky.
- **Duplicated Code** — the same logic shape appears in more than one hunk or file in the
  change. → extract the shared shape, call it from both.
- **Feature Envy** — a method that reaches into another object's data more than its own. →
  move the method onto the data it envies.
- **Data Clumps** — the same few fields or params keep travelling together (a type wanting
  to be born). → bundle them into one type, pass that.
- **Primitive Obsession** — a primitive or string standing in for a domain concept that
  deserves its own type. → give the concept its own small type.
- **Repeated Switches** — the same `switch`/`if`-cascade on the same type recurs across the
  change. → replace with polymorphism, or one map both sites share.
- **Shotgun Surgery** — one logical change forces scattered edits across many files in the
  diff. → gather what changes together into one module.
- **Divergent Change** — one file or module is edited for several unrelated reasons. → split
  so each module changes for one reason.
- **Speculative Generality** — abstraction, parameters, or hooks added for needs the spec
  doesn't have. → delete it; inline back until a real need shows.
- **Message Chains** — long `a.b().c().d()` navigation the caller shouldn't depend on. →
  hide the walk behind one method on the first object.
- **Middle Man** — a class or function that mostly just delegates onward. → cut it, call the
  real target direct.
- **Refused Bequest** — a subclass or implementer that ignores or overrides most of what it
  inherits. → drop the inheritance, use composition.

# Required report shape

Open your report with these lines, in this order:

```
Reviewed HEAD: 492ebc6371042e8f9c54562ca669435cc1ee9b9a
Review pass: R1
Standards sources checked: CLAUDE.md; docs/agents/issue-tracker.md; /implement contract (invoking workflow); smell baseline
Workflow evidence checked: /implement contract -> external run sheet rows AC1-AC4 and the baseline/post-change structural check results
```

Both coverage lines are owed even if you have no actionable findings.

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-standards-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Ordinals are assigned in the order findings appear in your report. Set `TDD re-entry
required: yes` only when satisfying the finding requires changing observable behavior;
otherwise `no`. Note that `/tdd` does not apply in this prose repository, so a `yes` here
routes to the invoking workflow's documented no-harness substitute. These fields classify
and route findings only — you must not edit anything.

# Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether changed
tests and evidence rows honor the agreed seams, runnable commands, and evidence fields: cite
the workflow source and quote the changed hunk or row. Distinguish hard violations from
judgement calls — documented-standard breaches can be hard, but baseline smells are always
judgement calls, and a documented repo standard overrides the baseline. Skip anything tooling
enforces. Under 400 words.
