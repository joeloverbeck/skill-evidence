# Dispatch packet — /code-review Standards axis (review pass R1)

Sub-agent type: general-purpose (read-only review; you must not edit any file).

---

You are the **Standards** reviewer for a two-axis code review. Review only; make no edits.

## Environment boundary

Operate only inside `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t4`. The repository is
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t4/repo`. Do not read, write, list, or run any command against any path
outside that directory. Treat every path outside it as nonexistent, including any skill,
document, or store you may believe exists elsewhere on this machine.

## Working-tree prohibition (binding, no exceptions)

The working tree carries pre-existing uncommitted changes that are unrelated to this work
and are **unrecoverable if lost** (`notes/analysis.md`, `scratch/records.jsonl`). You must
not run any git command that mutates the working tree or the index. Specifically these are
forbidden: `git add`, `git commit`, `git checkout`, `git restore`, `git switch`,
`git stash`, `git reset`, `git clean`, `git apply`. This prohibition is separate from and
additional to the instruction not to edit files — do not treat "I won't edit files" as
covering it. Read-only git (`git diff`, `git log`, `git show`, `git rev-parse`,
`git status`) is fine.

## Pinned endpoints

- `fixed_point_sha`: `ba2d824bbbfd7a55ce31f990cfbd975f0049f3aa` (supplied by the invoking
  implementation workflow as the repository `HEAD` before any edit)
- `reviewed_head_sha`: `66128e4c8c3b4df02c57aef0f3b24bd7686863f9`
- `review_pass_id`: `R1`
- Prior unresolved Standards findings retained from earlier passes: **none** (this is the
  first pass).

Build every review input from these captured SHAs only. Pinned commands:

```
git diff ba2d824bbbfd7a55ce31f990cfbd975f0049f3aa...66128e4c8c3b4df02c57aef0f3b24bd7686863f9
git log ba2d824bbbfd7a55ce31f990cfbd975f0049f3aa..66128e4c8c3b4df02c57aef0f3b24bd7686863f9 --oneline
```

Pinned commit list:

```
66128e4 Document the Widget service retry policy (ISSUE-7)
```

Pinned full diff (reproduced so this packet is self-contained):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index d45a51c..3fc72cb 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,13 +2,21 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried under the
+[retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.

+## Retry policy
+
+A failed call gets 3 retry attempts. The wait before each attempt follows a fixed backoff
+schedule: 1s, then 4s, then 16s.
+
+A `409 Conflict` is never retried.
+
 ## Support

 Open an issue under `docs/issues/`.
```

## Standards sources selected for this run (the complete list)

1. `repo/CLAUDE.md` — the repository's root agent instructions. Reproduced in full:

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

2. The active `/implement` workflow contract and its retained implementation evidence: the
   external run sheet at `.../t4/run-sheet.md`, which fixes the agreed seam
   (`docs/guide.md` only), the verification commands (`bash scripts/check-links.sh`,
   `bash scripts/check-headings.sh`), the pre-existing working-tree classification, and
   the acceptance-criteria → seam → verification rows. Judge the changed artifact against
   those agreed seams, commands, and evidence fields; do not restate the contract's schema.

3. General repository standards (`CODING_STANDARDS.md`, `CONTRIBUTING.md`, or equivalent):
   **no additional applicable standard** — none exist in this repository.

`repo/docs/agents/issue-tracker.md` was considered and **not** selected: it constrains
tracker artifacts (`docs/issues/*.md`), and no tracker artifact is in this diff.

## Smell baseline (always applies, pasted in full)

On top of whatever the repo documents, the Standards axis always carries this fixed set of
Fowler code smells (_Refactoring_, ch.3), which applies even when a repo documents nothing.
Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses
  something the baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature
  Envy"), never a hard violation — and, like any standard here, skip anything tooling
  already enforces.

Each smell reads *what it is* → *how to fix*; match it against the diff:

- **Mysterious Name** — a function, variable, or type whose name doesn't reveal what it
  does or holds. → rename it; if no honest name comes, the design's murky.
- **Duplicated Code** — the same logic shape appears in more than one hunk or file in the
  change. → extract the shared shape, call it from both.
- **Feature Envy** — a method that reaches into another object's data more than its own.
  → move the method onto the data it envies.
- **Data Clumps** — the same few fields or params keep travelling together (a type wanting
  to be born). → bundle them into one type, pass that.
- **Primitive Obsession** — a primitive or string standing in for a domain concept that
  deserves its own type. → give the concept its own small type.
- **Repeated Switches** — the same `switch`/`if`-cascade on the same type recurs across the
  change. → replace with polymorphism, or one map both sites share.
- **Shotgun Surgery** — one logical change forces scattered edits across many files in the
  diff. → gather what changes together into one module.
- **Divergent Change** — one file or module is edited for several unrelated reasons.
  → split so each module changes for one reason.
- **Speculative Generality** — abstraction, parameters, or hooks added for needs the spec
  doesn't have. → delete it; inline back until a real need shows.
- **Message Chains** — long `a.b().c().d()` navigation the caller shouldn't depend on.
  → hide the walk behind one method on the first object.
- **Middle Man** — a class or function that mostly just delegates onward. → cut it, call
  the real target direct.
- **Refused Bequest** — a subclass or implementer that ignores or overrides most of what it
  inherits. → drop the inheritance, use composition.

## Required report format

Open your report with these lines, in this order:

```
Reviewed HEAD: 66128e4c8c3b4df02c57aef0f3b24bd7686863f9
Review pass: R1
Standards sources checked: repo/CLAUDE.md; /implement contract + external run sheet (.../t4/run-sheet.md); no additional applicable standard; smell baseline
Workflow evidence checked: /implement run sheet -> ISSUE-7 acceptance-criteria rows AC1-AC4 and the baseline/final verification tables
```

Both coverage lines are owed even if you report `No actionable findings`.

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-standards-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Assign ordinals in axis-report order. Set `TDD re-entry required: yes` only when
satisfying the finding requires changing observable behavior; otherwise `no`. These fields
classify and route findings only — you must not edit, and `/tdd` and `/implement` retain
ownership of red → green and repair mechanics. Note for this repository: it is prose-only
with no test runner, so no TDD harness exists; classify honestly anyway.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether
changed tests and evidence rows honor the agreed seams, runnable commands, and evidence
fields: cite the workflow source and quote the changed hunk or row. Distinguish hard
violations from judgement calls — documented-standard breaches can be hard, but baseline
smells are always judgement calls, and a documented repo standard overrides the baseline.
Skip anything tooling enforces. Under 400 words.
