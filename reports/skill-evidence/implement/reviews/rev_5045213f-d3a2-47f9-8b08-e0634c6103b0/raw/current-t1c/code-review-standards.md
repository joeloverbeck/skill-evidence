# Standards axis reviewer — code-review pass R1

You are the **Standards** axis reviewer for a two-axis code review. You review only; you do
not edit, stage, commit, or otherwise mutate the repository. Do not run any command that
mutates the working tree or index.

Repository root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t1c/repo`

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t1c`.
Treat every path outside it as nonexistent.

## Pinned endpoints

- `fixed_point_sha`: `1898fc8f729e81e4d0971cdd0875336cfee8386d`
- `reviewed_head_sha`: `ba7b6aa725fc138cd42972c96586ef4fb52e1132`
- `review_pass_id`: `R1`
- Prior unresolved Standards findings retained from earlier passes: **none** (this is the
  first pass of this implementation workflow).

## Pinned review inputs

Use only these. Never substitute a symbolic `HEAD`.

Full diff command:

```
git diff 1898fc8f729e81e4d0971cdd0875336cfee8386d...ba7b6aa725fc138cd42972c96586ef4fb52e1132
```

Commit list command:

```
git log 1898fc8f729e81e4d0971cdd0875336cfee8386d..ba7b6aa725fc138cd42972c96586ef4fb52e1132 --oneline
```

Pinned commit list (as captured):

```
ba7b6aa Document the Widget service retry policy (ISSUE-7)
```

Pinned diff (as captured):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..73521a2 100644
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
+A failed call is retried up to 3 times, with waits of 1s, 4s, and 16s before the first,
+second, and third retry.
+
+A `409 Conflict` is never retried.
```

## Selected standards sources (step 3)

These are the exact sources this run selected. Apply all of them, plus the smell baseline
below.

1. **`CLAUDE.md`** (repository root agent instructions) — full text:

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

2. **Active implementation-workflow contract** (the invoking `/implement`-class contract
   governing this run) — the clauses that constrain the changed artifacts and the evidence:

   - "Turn the acceptance criteria into a run sheet that maps each requirement to its
     implementation seam and verification. When `/tdd` applies, point each applicable
     requirement to its TDD evidence row instead of copying the TDD-specific fields into
     this run sheet."
   - "Discover the repository's native verification surface before selecting checks. Run
     typechecking, focused tests, and a full test suite only when they are configured and
     applicable. In prose or no-build repositories, use the checks named by the repository
     instructions, such as single-home, cross-reference, and release-metadata checks."
   - "Run the smallest applicable checks regularly, and run the full applicable
     verification set once at the end. If no automated harness exists, record the exact
     structural or manual checks and their results."
   - "Put only the scoped implementation into `HEAD` (a provisional commit is acceptable)
     before invoking `/code-review`…" plus the unscoped `git status --short` /
     `git diff --cached --name-only` / `git diff --cached --check` reconciliation gate and
     the pre-review `git diff --name-only <fixed-point>...HEAD` /
     `git diff --check <fixed-point>...HEAD` gate.
   - "Classify all pre-existing changes, preserve unrelated dirt, and keep the
     implementation scope explicit."

   Note for your assessment: `/tdd` does **not** apply in this run — this prose repository
   has no test harness, so there are no TDD evidence rows to point at.

3. **Caller-selected run sheet** (retained implementation evidence for this run):

   | # | Acceptance criterion | Implementation seam | Verification | Result |
   |---|---|---|---|---|
   | AC1 | `docs/guide.md` gains a `## Retry policy` section | `docs/guide.md`, new H2 appended after `## Errors` | `scripts/check-headings.sh` (page keeps exactly one H1) + exact read of `docs/guide.md` | pass |
   | AC2 | Section states retry count (3 attempts) and backoff schedule (1s, 4s, 16s) | body of `## Retry policy` | exact read of `docs/guide.md` | pass |
   | AC3 | Section states a `409 Conflict` is never retried | body of `## Retry policy` | exact read of `docs/guide.md` | pass |
   | AC4 | The existing "are retried" sentence in `## Calling the service` links to the new section instead of restating the numbers | `docs/guide.md`, `## Calling the service` paragraph | `scripts/check-links.sh` + exact read confirming no counts/waits restated at that site | pass |

   Recorded verification results for `reviewed_head_sha`:

   - `bash scripts/check-links.sh` → `check-links: OK`, exit 0.
   - `bash scripts/check-headings.sh` → exit 1, sole output
     `docs/skills/code-review.md has 0 H1 headings`. This same failure reproduces at
     `fixed_point_sha` (it is a pre-existing repository condition, out of ISSUE-7 scope and
     deliberately not repaired). `docs/guide.md` is not reported by this check at either
     endpoint.

   Pre-existing working-tree dirt classified before editing and deliberately preserved
   unstaged (not in the review range): `notes/analysis.md` (appended re-measured latency
   line) and `scratch/records.jsonl` (appended `run 2` record). Both are unrelated to
   ISSUE-7.

4. **Smell baseline** (always applies — see below).

No `CODING_STANDARDS.md` or `CONTRIBUTING.md` exists in this repository.
`docs/agents/issue-tracker.md` was considered and excluded: no tracker artifact appears in
the review range.

## Smell baseline (always carried by the Standards axis)

A fixed set of Fowler code smells (_Refactoring_, ch.3) that applies even when a repo
documents nothing. Two rules bind it:

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

Open your report with exactly these lines, in order:

```
Reviewed HEAD: ba7b6aa725fc138cd42972c96586ef4fb52e1132
Review pass: R1
Standards sources checked: CLAUDE.md; active implementation-workflow contract; caller-selected run sheet; smell baseline
Workflow evidence checked: active implementation-workflow contract -> run sheet rows AC1-AC4 and the recorded check-links.sh / check-headings.sh results for ba7b6aa
```

Both coverage lines are owed even if you report no actionable findings.

Every actionable finding must carry, as its first line,
`Finding ID: R1-standards-<ordinal>` with the ordinal assigned in report order, and must
end with both routing fields:

```
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` whenever satisfying the finding requires changing
observable behavior; otherwise `no`. These fields classify and route only — `/tdd` and
`/implement` retain ownership of red → green and repair mechanics. Do not edit anything.

If you find nothing actionable, still emit the four header lines and then
`No actionable findings`.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether
changed tests and evidence rows honor the agreed seams, runnable commands, and evidence
fields: cite the workflow source and quote the changed hunk or row. Distinguish hard
violations from judgement calls — documented-standard breaches can be hard, but baseline
smells are always judgement calls, and a documented repo standard overrides the baseline.
Skip anything tooling enforces. Under 400 words.
