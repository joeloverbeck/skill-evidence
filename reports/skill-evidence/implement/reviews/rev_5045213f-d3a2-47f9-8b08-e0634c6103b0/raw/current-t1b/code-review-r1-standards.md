# Packet — /code-review pass R1, Standards axis

Dispatch target: isolated Standards reviewer sub-agent (read-only; reviewers must not edit).
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

## Review pass identity

- `review_pass_id`: `R1`
- Prior unresolved Standards findings retained from earlier passes: **none** (R1 is the first
  pass in this implementation workflow).

## Standards sources selected for this run (apply exactly these, plus the baseline)

1. **`CLAUDE.md`** — the repository's root agent instructions. Full applicable text:

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

2. **Active invoking implementation-workflow contract** (embedded use; constrains the changed
   artifact's scope, verification, and commit hygiene). Applicable clauses verbatim:

   > Run `git status --short` without a path filter. Classify all pre-existing changes,
   > preserve unrelated dirt, and keep the implementation scope explicit.
   >
   > Turn the acceptance criteria into a run sheet that maps each requirement to its
   > implementation seam and verification.
   >
   > Discover the repository's native verification surface before selecting checks. Run
   > typechecking, focused tests, and a full test suite only when they are configured and
   > applicable. In prose or no-build repositories, use the checks named by the repository
   > instructions, such as single-home, cross-reference, and release-metadata checks.
   >
   > Run the smallest applicable checks regularly, and run the full applicable verification
   > set once at the end. If no automated harness exists, record the exact structural or
   > manual checks and their results.
   >
   > Put only the scoped implementation into `HEAD` ... proceed only when the index is
   > exactly the scoped implementation and unrelated dirt remains unstaged.

   `/tdd` is **not applicable**: this prose repository has no TDD harness, no test runner, and
   no type checker, so no test or TDD-evidence artifact is in scope for this diff.

3. **Caller-selected run sheet** (retained implementation evidence for this workflow; the
   agreed seam and verification per acceptance criterion):

   | AC | Requirement | Implementation seam | Verification | Result |
   |---|---|---|---|---|
   | 1 | `docs/guide.md` gains a `## Retry policy` section | new H2 between `## Calling the service` and `## Errors` in `docs/guide.md` | `scripts/check-headings.sh` (guide.md keeps exactly one H1); read-back of the section heading | H1 count for `docs/guide.md` = 1; heading present |
   | 2 | Section states retry count (3 attempts) and backoff (1s, 4s, 16s) | body of the new section | read-back of the section body | present: "up to 3 times", "1s, 4s, and 16s" |
   | 3 | Section states a `409 Conflict` is never retried | body of the new section | read-back of the section body | present |
   | 4 | The existing "are retried" sentence in `## Calling the service` links to the new section instead of restating the numbers | first paragraph of `## Calling the service` | `scripts/check-links.sh`; read-back confirming no numbers restated | `check-links: OK`; sentence links, states no numbers |

   Full applicable verification set run at `reviewed_head_sha`:
   `bash scripts/check-links.sh` → `check-links: OK` (exit 0);
   `bash scripts/check-headings.sh` → exit 1, sole failure
   `docs/skills/code-review.md has 0 H1 headings`, which is **pre-existing at
   `fixed_point_sha`** (identical baseline output before any edit) and outside this
   implementation's scope; `docs/guide.md` is not among the failures.

4. No additional applicable standard: the repository has no `AGENTS.md`,
   `CODING_STANDARDS.md`, or `CONTRIBUTING.md`. `docs/agents/issue-tracker.md` governs tracker
   reads/mutations only and no tracker file is in this diff.

## Smell baseline (always applies, pasted in full)

On top of whatever the repo documents, the Standards axis always carries the **smell
baseline** — a fixed set of Fowler code smells (_Refactoring_, ch.3) that applies even when a
repo documents nothing. Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses something
  the baseline would flag, suppress the smell.
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

## Required report format

Open the report with these four lines, in this order:

```
Reviewed HEAD: 963bd65c0e9e9e327e12040f37a721a4cab63133
Review pass: R1
Standards sources checked: CLAUDE.md; active invoking implementation-workflow contract; caller-selected run sheet (AC1–AC4 evidence rows); smell baseline
Workflow evidence checked: invoking implementation-workflow contract -> run sheet rows AC1–AC4 and their recorded structural-check results (/tdd: N/A, no TDD harness in this prose repository)
```

Both coverage lines are owed even if you report `No actionable findings`.

Every actionable finding must carry, on its own lines:

- `Finding ID: R1-standards-<ordinal>` (ordinal assigned in axis-report order; IDs are immutable)
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` (`yes` only when satisfying the finding requires changing
  observable behavior)

These fields classify and route only. Do not edit any file; `/tdd` and `/implement` own repair
mechanics.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether changed
tests and evidence rows honor the agreed seams, runnable commands, and evidence fields: cite
the workflow source and quote the changed hunk or row. Distinguish hard violations from
judgement calls — documented-standard breaches can be hard, but baseline smells are always
judgement calls, and a documented repo standard overrides the baseline. Skip anything tooling
enforces. Under 400 words.
