You are the **Standards** axis reviewer for a two-axis code review. You review only; you do
not edit, and you do not repair. Report findings and stop.

Repository root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1/repo`
Operate only inside `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1`. Treat every path outside it as nonexistent.

## Working-tree prohibition (binding)

The working tree carries uncommitted changes that are unrelated to this review and are
unrecoverable if lost. You MUST NOT run any git command that mutates the working tree or
index. Specifically forbidden: `git add`, `git commit`, `git checkout`, `git restore`,
`git switch`, `git stash`, `git reset`, `git clean`, `git apply`. Read-only git
(`diff`, `log`, `show`, `status`, `rev-parse`, `cat-file`) is fine. This prohibition is
separate from and additional to "do not edit files".

## Pinned endpoints

- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542`
- `reviewed_head_sha`: `e36b1638c3404368d6832b4a039fe0c3c6d513d3`
- Review pass: `R1`
- Prior unresolved Standards findings retained from earlier passes: **none** (this is the
  first pass of this implementation workflow).

Pinned full diff command (three-dot, against the merge-base):

```
git diff ea885e890a7cd032311da701807a817333cc3542...e36b1638c3404368d6832b4a039fe0c3c6d513d3
```

Pinned commit list command and its captured output:

```
git log ea885e890a7cd032311da701807a817333cc3542..e36b1638c3404368d6832b4a039fe0c3c6d513d3 --oneline
e36b163 Document the Widget service retry policy (ISSUE-7)
```

Captured diff (reproduce with the pinned command above; do not substitute a symbolic
`HEAD`):

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..76638b2 100644
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
+A failed call is retried up to 3 attempts. The waits before those attempts are 1s, then
+4s, then 16s.
+
+A `409 Conflict` is never retried.
```

## Standards sources selected for this run (apply exactly these)

1. `CLAUDE.md` — repository root agent instructions. Its binding content:
   - Prose-only repository: no build system, no test runner, no type checker.
   - Verification surface, run from the repository root: `scripts/check-links.sh`
     (every relative Markdown link resolves) and `scripts/check-headings.sh` (every doc
     page starts with a single H1). "Run both before any commit that touches `docs/`."
   - Conventions: "One topic per page. Do not duplicate a rule that already has a home
     elsewhere." and "Reference pages by relative path."
2. `no additional applicable standard` — the repository contains no `CODING_STANDARDS.md`,
   `CONTRIBUTING.md`, or domain-/authoring-specific standard for `docs/*.md` beyond the
   root instructions.
3. `implementation-contract run sheet (ISSUE-7 AC1-AC4)` — the active invoking-workflow
   contract's run sheet, reproduced under "Workflow evidence" below. Treat it as a
   standards/process input, not as the product spec.

Plus the **smell baseline**, which always applies (pasted in full below).

## Workflow evidence (retained run sheet for this implementation)

`/tdd` does not apply: this is a prose repository with no TDD harness, so there are no
changed tests and no TDD evidence rows. The caller-selected run sheet is:

| # | Acceptance criterion | Implementation seam | Verification | Result |
|---|---|---|---|---|
| AC1 | `docs/guide.md` gains a `## Retry policy` section | new H2 in `docs/guide.md` | `bash scripts/check-headings.sh`; structural read of the section | pass |
| AC2 | Section states retry count (3 attempts) and backoff (1s, 4s, 16s) | body of the new section | structural read of the section text | pass |
| AC3 | Section states `409 Conflict` is never retried | body of the new section | structural read of the section text | pass |
| AC4 | The "are retried" sentence in `## Calling the service` links to the new section instead of restating the numbers | edited sentence in `docs/guide.md` | `bash scripts/check-links.sh`; manual anchor check (`#retry-policy` matches the `## Retry policy` heading slug) | pass |

Recorded verification results for `reviewed_head_sha`:

- `bash scripts/check-links.sh` -> `check-links: OK` (exit 0).
- `bash scripts/check-headings.sh` -> exit 1, `docs/skills/code-review.md has 0 H1
  headings`. This failure is **pre-existing at `fixed_point_sha`** (identical output
  before any edit) and `docs/skills/code-review.md` is untouched by this diff.
  `docs/guide.md` is not reported by the checker, i.e. it carries exactly one H1.
  Judge whether leaving that pre-existing failure out of scope is defensible under
  `CLAUDE.md`; do not treat it as introduced by this diff.

## Smell baseline (always carried by the Standards axis)

Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses
  something the baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature
  Envy"), never a hard violation — and, like any standard here, skip anything tooling
  already enforces.

Each smell reads *what it is* -> *how to fix*; match it against the diff:

- **Mysterious Name** — a function, variable, or type whose name doesn't reveal what it
  does or holds. -> rename it; if no honest name comes, the design's murky.
- **Duplicated Code** — the same logic shape appears in more than one hunk or file in the
  change. -> extract the shared shape, call it from both.
- **Feature Envy** — a method that reaches into another object's data more than its own.
  -> move the method onto the data it envies.
- **Data Clumps** — the same few fields or params keep travelling together (a type wanting
  to be born). -> bundle them into one type, pass that.
- **Primitive Obsession** — a primitive or string standing in for a domain concept that
  deserves its own type. -> give the concept its own small type.
- **Repeated Switches** — the same `switch`/`if`-cascade on the same type recurs across
  the change. -> replace with polymorphism, or one map both sites share.
- **Shotgun Surgery** — one logical change forces scattered edits across many files in the
  diff. -> gather what changes together into one module.
- **Divergent Change** — one file or module is edited for several unrelated reasons.
  -> split so each module changes for one reason.
- **Speculative Generality** — abstraction, parameters, or hooks added for needs the spec
  doesn't have. -> delete it; inline back until a real need shows.
- **Message Chains** — long `a.b().c().d()` navigation the caller shouldn't depend on.
  -> hide the walk behind one method on the first object.
- **Middle Man** — a class or function that mostly just delegates onward. -> cut it, call
  the real target direct.
- **Refused Bequest** — a subclass or implementer that ignores or overrides most of what
  it inherits. -> drop the inheritance, use composition.

## Required report shape

Open with these lines, in this order:

```
Reviewed HEAD: e36b1638c3404368d6832b4a039fe0c3c6d513d3
Review pass: R1
Standards sources checked: CLAUDE.md; no additional applicable standard; implementation-contract run sheet (ISSUE-7 AC1-AC4); smell baseline
Workflow evidence checked: implementation-contract run sheet -> ISSUE-7 rows AC1-AC4 (verification commands scripts/check-links.sh, scripts/check-headings.sh)
```

Both coverage lines are owed even by a `No actionable findings` report.

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-standards-<ordinal>   (ordinal assigned in axis-report order)
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` only when satisfying the finding requires changing
observable behavior; otherwise `no`. These fields classify and route findings only —
`/tdd` and `/implement` own red -> green and repair mechanics. Do not edit anything.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it
and quote the hunk; and (c), where workflow sources constrain tests or evidence, whether
changed tests and evidence rows honor the agreed seams, runnable commands, and evidence
fields: cite the workflow source and quote the changed hunk or row. Distinguish hard
violations from judgement calls — documented-standard breaches can be hard, but baseline
smells are always judgement calls, and a documented repo standard overrides the baseline.
Skip anything tooling enforces. Under 400 words.
