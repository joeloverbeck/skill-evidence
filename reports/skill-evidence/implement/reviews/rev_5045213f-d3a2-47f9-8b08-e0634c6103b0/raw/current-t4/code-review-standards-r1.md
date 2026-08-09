You are the **Standards** axis reviewer for `/code-review`, pass `R1`. Review only; never edit
any file, never move `HEAD`, never run a mutating git command.

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

## Standards sources selected for this run

1. `repo/CLAUDE.md` — the repository's root agent instructions. Full text:

   ```
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

2. The active invoking-workflow contract (`/implement`, as supplied in this run's task
   prompt). The clauses that constrain the changed artifact and its evidence:

   - "Discover the repository's native verification surface before selecting checks. Run
     typechecking, focused tests, and a full test suite only when they are configured and
     applicable. In prose or no-build repositories, use the checks named by the repository
     instructions, such as single-home, cross-reference, and release-metadata checks."
   - "If no automated harness exists, record the exact structural or manual checks and their
     results."
   - "Turn the acceptance criteria into a run sheet that maps each requirement to its
     implementation seam and verification."
   - "Put only the scoped implementation into `HEAD` ... proceed only when the index is exactly
     the scoped implementation and unrelated dirt remains unstaged."
   - `/tdd` is not applicable: this prose repository has no TDD harness, so no red→green
     evidence rows exist and no test files are in range.

3. The smell baseline below (always applicable).

`repo/docs/agents/issue-tracker.md` was considered and **not** selected: it constrains tracker
reads and mutations, and no tracker file is in the review range.

Retained implementation evidence for this pass (the caller's run sheet — this is the
`Workflow evidence checked` target):

| Row | Acceptance criterion (ISSUE-7) | Implementation seam | Verification |
|---|---|---|---|
| AC1 | `docs/guide.md` gains a `## Retry policy` section | new H2 in `docs/guide.md`, placed after `## Errors` | `scripts/check-headings.sh`; read-back of `docs/guide.md` |
| AC2 | states retry count (3 attempts) and backoff (1s, 4s, 16s) | body of the new `## Retry policy` section | read-back of `docs/guide.md` |
| AC3 | states a `409 Conflict` is never retried | body of the new `## Retry policy` section | read-back of `docs/guide.md` |
| AC4 | the existing "are retried" sentence links to the new section instead of restating numbers | `## Calling the service` paragraph, `[Retry policy](#retry-policy)` | `scripts/check-links.sh`; read-back of `docs/guide.md` |

Recorded check results at `reviewed_head_sha` (no automated test harness exists):
`bash scripts/check-links.sh` → `check-links: OK` (rc 0);
`bash scripts/check-headings.sh` → rc 1, sole failure `docs/skills/code-review.md has 0 H1
headings`, which reproduces identically at `fixed_point_sha` and is therefore a pre-existing
failure outside this change's scope. `docs/guide.md` is not flagged by either check.

## Smell baseline (applies even when a repo documents nothing)

Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses something
  the baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature Envy"),
  never a hard violation — and, like any standard here, skip anything tooling already enforces.

Each smell reads *what it is* → *how to fix*; match it against the diff:

- **Mysterious Name** — a function, variable, or type whose name doesn't reveal what it does or
  holds. → rename it; if no honest name comes, the design's murky.
- **Duplicated Code** — the same logic shape appears in more than one hunk or file in the
  change. → extract the shared shape, call it from both.
- **Feature Envy** — a method that reaches into another object's data more than its own. → move
  the method onto the data it envies.
- **Data Clumps** — the same few fields or params keep travelling together (a type wanting to be
  born). → bundle them into one type, pass that.
- **Primitive Obsession** — a primitive or string standing in for a domain concept that deserves
  its own type. → give the concept its own small type.
- **Repeated Switches** — the same `switch`/`if`-cascade on the same type recurs across the
  change. → replace with polymorphism, or one map both sites share.
- **Shotgun Surgery** — one logical change forces scattered edits across many files in the
  diff. → gather what changes together into one module.
- **Divergent Change** — one file or module is edited for several unrelated reasons. → split so
  each module changes for one reason.
- **Speculative Generality** — abstraction, parameters, or hooks added for needs the spec
  doesn't have. → delete it; inline back until a real need shows.
- **Message Chains** — long `a.b().c().d()` navigation the caller shouldn't depend on. → hide
  the walk behind one method on the first object.
- **Middle Man** — a class or function that mostly just delegates onward. → cut it, call the
  real target direct.
- **Refused Bequest** — a subclass or implementer that ignores or overrides most of what it
  inherits. → drop the inheritance, use composition.

## Report format

Open your report with exactly these lines, in this order:

```
Reviewed HEAD: 1975f2f1613582a22f45bf34688643e1bda0c11d
Review pass: R1
Standards sources checked: repo/CLAUDE.md; active /implement contract (task prompt, "Contract you must follow"); smell baseline
Workflow evidence checked: active /implement contract -> run sheet rows AC1-AC4 and the recorded structural-check results above
```

Both coverage lines are owed even by a `No actionable findings` report.

Prior unresolved Standards findings carried into this pass: none (R1 is the first pass).

Every actionable finding must carry, in this order:

- `Finding ID: R1-standards-<ordinal>` (ordinal assigned in axis-report order; IDs are
  immutable once emitted)
- the finding text
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` (`yes` only when satisfying the finding requires changing
  observable behavior)

These fields classify and route findings only. You must not edit anything; `/tdd` and
`/implement` retain ownership of red → green and repair mechanics.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether changed
tests and evidence rows honor the agreed seams, runnable commands, and evidence fields: cite the
workflow source and quote the changed hunk or row. Distinguish hard violations from judgement
calls — documented-standard breaches can be hard, but baseline smells are always judgement
calls, and a documented repo standard overrides the baseline. Skip anything tooling enforces.
Under 400 words.
