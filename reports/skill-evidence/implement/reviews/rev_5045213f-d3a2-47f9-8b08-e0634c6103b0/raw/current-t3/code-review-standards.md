# Standards axis reviewer — /code-review pass R1

You are the **Standards** axis reviewer for a two-axis review. You review only; you must not
edit any file, stage anything, or move `HEAD`.

## Repository

`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t3/repo`

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t3`.
Treat every path outside it as nonexistent.

## Pinned endpoints

- `fixed_point_sha`: `3e70c4486bb871d442973448aac358603097244c` (supplied by the invoking
  implementation workflow as the pre-edit `HEAD`)
- `reviewed_head_sha`: `7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc`

Build every review input from these captured SHAs only. Do not leave a symbolic `HEAD` in any
command.

Pinned full diff command:

```
git diff 3e70c4486bb871d442973448aac358603097244c...7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc
```

Pinned commit list command and its result:

```
git log 3e70c4486bb871d442973448aac358603097244c..7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc --oneline
7a82a3c Document the Widget service retry policy
```

The pinned diff, inlined so this prompt is self-contained (run the command yourself as well):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..ff64752 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,9 +2,17 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried under the
+[retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.
+
+## Retry policy
+
+A failed call is retried up to 3 times. The waits before the first, second, and third
+retry are 1s, 4s, and 16s.
+
+A `409 Conflict` is never retried.
```

## Review pass

`review_pass_id`: **R1**

Prior unresolved Standards findings retained from earlier passes: **none** (R1 is the first
pass of this implementation workflow).

## Standards sources selected for this run

These are the exact sources selected in step 3. Check all of them.

1. **`CLAUDE.md`** (root agent instructions), quoted in full:

   > # widget-docs
   >
   > Documentation repository for the Widget service. Prose only; there is no build system,
   > no test runner, and no type checker.
   >
   > ## Verification surface
   >
   > The repository's native checks are structural, run from the repository root:
   >
   > - `scripts/check-links.sh` — every relative Markdown link resolves.
   > - `scripts/check-headings.sh` — every doc page starts with a single H1.
   >
   > Run both before any commit that touches `docs/`.
   >
   > ## Conventions
   >
   > - One topic per page. Do not duplicate a rule that already has a home elsewhere.
   > - Reference pages by relative path.

2. **Implementation-workflow contract and its retained run sheet** (embedded use: the active
   invoking-workflow contract and retained implementation evidence constrain the changed
   artifact). The retained run sheet for this run:

   | # | Acceptance criterion (ISSUE-7) | Implementation seam | Verification |
   |---|---|---|---|
   | 1 | `docs/guide.md` gains a `## Retry policy` section | new `## Retry policy` H2 in `docs/guide.md`, placed after `## Errors` | `grep -nE '^#+ ' docs/guide.md`; `scripts/check-headings.sh` |
   | 2 | Section states retry count (3 attempts) and backoff schedule (1s, 4s, 16s) | body of the new `## Retry policy` section | read-back of the section text |
   | 3 | Section states a `409 Conflict` is never retried | body of the new `## Retry policy` section | read-back of the section text |
   | 4 | The `are retried` sentence in `## Calling the service` links to the new section instead of restating the numbers | in-page anchor link `[retry policy](#retry-policy)` on that sentence | read-back; `scripts/check-links.sh`; manual anchor-slug check (the checker skips `#`-only targets) |

   Recorded verification results on `reviewed_head_sha`:
   - `bash scripts/check-links.sh` → `check-links: OK`, exit 0.
   - `bash scripts/check-headings.sh` → exit 1, sole failure
     `docs/skills/code-review.md has 0 H1 headings`. This failure is **pre-existing at
     `fixed_point_sha`** (identical output before any edit) and lies outside the changed path.
     `docs/guide.md` has exactly one H1.

   `/tdd` is **not** an applicable workflow source: this is a prose repository with no test
   runner, so no changed tests and no TDD evidence rows are in scope.

3. **The smell baseline**, which always applies (pasted in full below).

## Smell baseline (always carried by the Standards axis)

A fixed set of Fowler code smells (_Refactoring_, ch.3) that applies even when a repo documents
nothing. Two rules bind it:

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

## Required report format

Open your report with these four lines, in this order:

```
Reviewed HEAD: 7a82a3c328a8a8d91bd31298a7ef445b3c3e8fbc
Review pass: R1
Standards sources checked: CLAUDE.md; implementation-workflow contract (retained run sheet); smell baseline
Workflow evidence checked: implementation-workflow retained run sheet -> ISSUE-7 run-sheet rows 1-4 and their recorded check results
```

Both coverage lines are owed even if you report no actionable findings.

Every actionable finding must carry a stable identity and both routing fields:

- `Finding ID: R1-standards-<ordinal>`, ordinal assigned in axis-report order.
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` — `yes` only when satisfying the finding requires changing
  observable behavior.

These fields classify and route findings only. You must not edit; `/tdd` and `/implement` retain
ownership of red → green and repair mechanics.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented standard:
cite the standard (file + the rule); (b) any baseline smell you spot: name it and quote the hunk;
and (c), where workflow sources constrain tests or evidence, whether changed tests and evidence
rows honor the agreed seams, runnable commands, and evidence fields: cite the workflow source and
quote the changed hunk or row. Distinguish hard violations from judgement calls —
documented-standard breaches can be hard, but baseline smells are always judgement calls, and a
documented repo standard overrides the baseline. Skip anything tooling enforces. Under 400 words.
