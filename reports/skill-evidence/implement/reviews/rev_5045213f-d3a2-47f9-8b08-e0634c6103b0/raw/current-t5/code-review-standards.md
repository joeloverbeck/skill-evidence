# Dispatch packet — /code-review Standards axis (review pass R1)

Dispatch target: isolated Standards reviewer sub-agent, launched in parallel with the Spec
reviewer (`packets/code-review-spec.md`). Sub-agent surface unavailable in this
environment; packet composed for dispatch and recorded here instead.

---

You are the **Standards** reviewer for a two-axis code review. You review only; you must
not edit any file, stage anything, or move `HEAD`.

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

## Applicable standards sources (the complete list selected for this run)

1. `repo/CLAUDE.md` — root agent instructions for this repository. Full text:

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

2. The active invoking-workflow `/implement` contract, at `t5/prompt.md` — governs scope
   discipline, index hygiene, and the review handoff for this change.

3. The caller-selected run sheet, at `t5/run-sheet.md` — defines the agreed implementation
   seams, the verification commands, and the pre-existing verification baseline. It is
   deliberately untracked (kept outside the repository) because `CLAUDE.md` forbids
   duplicating a rule that has a home elsewhere; judge whether that placement call is
   sound. Full text is available at that path; read it.

4. The **smell baseline** below (always applicable).

No `CODING_STANDARDS.md` or `CONTRIBUTING.md` exists in this repository. `/tdd` is not an
applicable standards source: the repository has no test runner and no TDD evidence exists
or is in scope.

## Smell baseline (applies even when the repo documents nothing)

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
- **Divergent Change** — one file or module is edited for several unrelated reasons. →
  split so each module changes for one reason.
- **Speculative Generality** — abstraction, parameters, or hooks added for needs the spec
  doesn't have. → delete it; inline back until a real need shows.
- **Message Chains** — long `a.b().c().d()` navigation the caller shouldn't depend on. →
  hide the walk behind one method on the first object.
- **Middle Man** — a class or function that mostly just delegates onward. → cut it, call
  the real target direct.
- **Refused Bequest** — a subclass or implementer that ignores or overrides most of what it
  inherits. → drop the inheritance, use composition.

## Required report format

Open your report with these lines, in this order:

```
Reviewed HEAD: 37aa66cdd2894c57e75020092b94fd19e633fe72
Review pass: R1
Standards sources checked: repo/CLAUDE.md; t5/prompt.md (active /implement contract); t5/run-sheet.md (caller-selected run sheet); smell baseline
Workflow evidence checked: t5/run-sheet.md -> requirement map rows 1-4 and the pre-existing verification baseline table
```

Both coverage lines are owed even if you report no actionable findings. Report every
selected source above, no more and no fewer.

Prior unresolved Standards findings carried into this pass: **none** (R1 is the first
pass).

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-standards-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Assign ordinals in axis-report order, starting at 1. Set `TDD re-entry required: yes`
whenever satisfying the finding requires changing observable behavior; otherwise `no`. For
this prose repository, "observable behavior" means what the published documentation states
to a reader. These fields classify and route findings only — `/tdd` and `/implement` retain
ownership of red → green and repair mechanics. Do not edit anything.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether
changed tests and evidence rows honor the agreed seams, runnable commands, and evidence
fields: cite the workflow source and quote the changed hunk or row. Distinguish hard
violations from judgement calls — documented-standard breaches can be hard, but baseline
smells are always judgement calls, and a documented repo standard overrides the baseline.
Skip anything tooling enforces. Under 400 words.

Specific things worth your attention, none of which are conclusions:

- `CLAUDE.md` requires both `scripts/check-links.sh` and `scripts/check-headings.sh` to run
  before any commit touching `docs/`. Both were run. `check-links.sh` passes.
  `check-headings.sh` exits 1 on `docs/skills/code-review.md has 0 H1 headings` — a failure
  that is present at `ea885e890a7cd032311da701807a817333cc3542`, before this change, in a
  file this change does not touch. The run sheet classifies it as pre-existing and out of
  scope. Judge whether leaving a red repository-native check red is defensible here, and
  whether the classification is honest.
- The new `## Retry policy` section names `409 Conflict`, whose meaning is documented in
  `## Errors` on the same page. Judge this against "One topic per page. Do not duplicate a
  rule that already has a home elsewhere."
- The link added in `## Calling the service` is a same-page anchor, `#retry-policy`, not a
  relative path. Judge it against "Reference pages by relative path" and note that
  `check-links.sh` skips `#`-prefixed targets, so tooling does not verify this anchor.
