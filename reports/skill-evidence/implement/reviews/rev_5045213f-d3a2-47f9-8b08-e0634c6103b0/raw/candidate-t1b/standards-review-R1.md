You are the **Standards** axis reviewer for a two-axis `/code-review` pass. You review only;
you must not edit any file.

# Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1b`.
The repository is
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1b/repo`.
Treat every path outside that directory as nonexistent, including any skill, document, or store
you may believe exists elsewhere on this machine.

# Working-tree prohibition (binding)

This repository carries pre-existing uncommitted changes that are unrecoverable if lost. You are
forbidden from running any git command that mutates the working tree or the index. Specifically,
you must not run `git add`, `git commit`, `git checkout`, `git restore`, `git switch`,
`git stash`, `git reset`, `git clean`, or `git apply`, in any form, with any flags. Read-only git
(`git diff`, `git log`, `git show`, `git status`, `git rev-parse`) is permitted. Not editing files
does not exempt you from this: the prohibition covers the git commands themselves.

# Pinned review inputs

- `reviewed_head_sha`: `1bf8c9be09a5e98e792985d396507beb961eab2f`
- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542`
- Review pass: `R1`
- Prior unresolved Standards findings retained from earlier passes: none (this is the first pass).

Build every review input from these captured SHAs only. Do not use a symbolic `HEAD`.

Pinned full diff command:

```
git diff ea885e890a7cd032311da701807a817333cc3542...1bf8c9be09a5e98e792985d396507beb961eab2f
```

Pinned commit list:

```
1bf8c9b Document the Widget service retry policy (ISSUE-7)
```

# Standards sources selected for this run

Inspect every source below. This is the exact selected list; your coverage line must match it in
both directions.

1. `repo/CLAUDE.md` — root agent instructions. Full text:

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

2. The active `/implement` workflow contract governing this run — it requires that the committed
   review range contain exactly the scoped implementation, that pre-existing unrelated dirt stay
   unstaged, and that each acceptance criterion map to an implementation seam and a verification.

3. The caller-selected run sheet (retained implementation evidence) reproduced below.

4. The smell baseline (below), which always applies.

No `CODING_STANDARDS.md`, `CONTRIBUTING.md`, or authoring-specific standard exists in this
repository. `repo/docs/agents/issue-tracker.md` was considered and excluded: it constrains tracker
reads and mutations, and no tracker artifact is in this diff. The `/tdd` contract was considered
and excluded: this is a prose repository with no test runner, no tests changed, and no TDD
evidence rows exist.

## Run sheet (retained implementation evidence)

| Row | Acceptance criterion | Implementation seam | Verification |
|---|---|---|---|
| AC1 | `docs/guide.md` gains a `## Retry policy` section | new `## Retry policy` section in `docs/guide.md`, placed after `## Errors` | `grep -n '^## Retry policy' docs/guide.md` returns exactly one line; `scripts/check-headings.sh` reports no finding for `docs/guide.md` |
| AC2 | Section states retry count (3 attempts) and backoff schedule (1s, 4s, 16s) | first paragraph of `## Retry policy` | `grep -n '3 times\|1s, 4s' docs/guide.md` |
| AC3 | Section states a `409 Conflict` is never retried | second paragraph of `## Retry policy` | ``grep -n '409 Conflict` is never retried' docs/guide.md`` |
| AC4 | Existing "are retried" sentence links to the new section instead of restating the numbers | sentence in `## Calling the service` | `grep -n 'retry policy](#retry-policy)' docs/guide.md`; `scripts/check-links.sh` reports OK; sentence contains no retry numbers |

Verification results recorded by the implementer on `reviewed_head_sha`:

- `bash scripts/check-links.sh` → `check-links: OK` (exit 0).
- `bash scripts/check-headings.sh` → exit 1, sole finding `docs/skills/code-review.md has 0 H1
  headings`. This failure is pre-existing at `fixed_point_sha` (verified before any edit) on a
  file outside this change; `docs/guide.md` is not flagged before or after.
- All four AC greps returned the expected lines.

## Smell baseline (always applies)

Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses something the
  baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature Envy"), never
  a hard violation — and, like any standard here, skip anything tooling already enforces.

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

# Report format

Open your report with these lines, in this order:

```
Reviewed HEAD: 1bf8c9be09a5e98e792985d396507beb961eab2f
Review pass: R1
Standards sources checked: repo/CLAUDE.md; /implement workflow contract; run sheet (retained implementation evidence); smell baseline
Workflow evidence checked: /implement contract -> run sheet rows AC1-AC4
```

Both coverage lines are owed even if you report `No actionable findings`.

Every actionable finding must carry, in order:

- `Finding ID: R1-standards-<ordinal>` (ordinal assigned in axis-report order)
- the finding itself
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` — `yes` only when satisfying the finding requires changing
  observable behavior; otherwise `no`.

These fields classify and route findings only. `/tdd` and `/implement` retain ownership of red →
green and repair mechanics. Do not edit anything.

# Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented standard:
cite the standard (file + the rule); (b) any baseline smell you spot: name it and quote the hunk;
and (c), where workflow sources constrain tests or evidence, whether changed tests and evidence
rows honor the agreed seams, runnable commands, and evidence fields: cite the workflow source and
quote the changed hunk or row. Distinguish hard violations from judgement calls — documented-
standard breaches can be hard, but baseline smells are always judgement calls, and a documented
repo standard overrides the baseline. Skip anything tooling enforces. Under 400 words.
