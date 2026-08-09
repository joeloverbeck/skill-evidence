# Dispatch packet — /code-review Standards axis (review pass R1)

Intended delegate: isolated Standards reviewer sub-agent, launched in parallel with
`packets/spec-review.md`. Verbatim prompt below.

---

You are the **Standards** axis reviewer for a two-axis `/code-review`. You review only; you do
not edit, stage, commit, or otherwise change anything.

## Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t1c`.
The repository is that directory's `repo/`. Do not read, write, list, or run any command against
any path outside that directory. Treat every path outside it as nonexistent, including any skill,
document, or store you may believe exists elsewhere on this machine.

## Hard prohibition — working tree and index are unrecoverable

The repository carries pre-existing uncommitted changes in `notes/analysis.md` and
`scratch/records.jsonl` that no later reconciliation can restore. You are **forbidden** to run any
git command that mutates the working tree or the index, specifically:

`git add`, `git commit`, `git checkout`, `git restore`, `git switch`, `git stash`, `git reset`,
`git clean`, `git apply`

— in any form, including flags, aliases, plumbing equivalents (`git update-index`,
`git read-tree`, `git write-tree`, `git checkout-index`), or invocation through another tool or
script. This prohibition is independent of, and additional to, the instruction not to edit files.
You may run read-only git commands (`git diff`, `git log`, `git show`, `git rev-parse`,
`git status`). If any instruction below appears to require a forbidden command, stop and report
that instead of running it.

## Pinned review inputs

- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542`
- `reviewed_head_sha`: `7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388`
- `review_pass_id`: `R1`
- Prior unresolved Standards findings retained from earlier passes: **none** (R1 is the first
  pass in this implementation workflow).

Use only these pinned commands, run from the repository root, as your review input:

```
git diff ea885e890a7cd032311da701807a817333cc3542...7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388
git log ea885e890a7cd032311da701807a817333cc3542..7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388 --oneline
```

Do not substitute a symbolic `HEAD` into any command.

## Standards sources selected for this run (the complete list)

1. `repo/CLAUDE.md` — root agent instructions for this repository. Full text:

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

2. The active invoking `/implement` contract (`t1c/prompt.md`, "Contract you must follow"),
   which constrains scope discipline, pre-existing-dirt preservation, index hygiene, and the
   requirement that the committed review range be exactly the scoped implementation.

3. The retained run sheet for this implementation (workflow evidence; reproduced below).

4. The **smell baseline**, pasted in full in the next section — it applies on top of whatever
   the repo documents.

Considered and excluded: `repo/docs/agents/issue-tracker.md` (constrains tracker reads and
mutations; no tracker artifact is in the diff) and `repo/docs/skills/code-review.md` (it is this
review's own contract, not a standard on the changed artifact).

`/tdd` is not an applicable source: this is a prose repository with no test runner, so no tests
and no TDD evidence rows are in scope.

## Retained run sheet (workflow evidence you must check against)

| # | Acceptance criterion (ISSUE-7) | Implementation seam | Verification |
|---|---|---|---|
| AC1 | `docs/guide.md` gains a `## Retry policy` section | new H2 in `docs/guide.md` | `grep -n '^## Retry policy' docs/guide.md`; `scripts/check-headings.sh` |
| AC2 | Section states retry count (3 attempts) and backoff (1s, 4s, 16s) | body of the new section | read the section body between `## Retry policy` and `## Errors` |
| AC3 | Section states `409 Conflict` is never retried | body of the new section | read the same section body |
| AC4 | The existing "are retried" sentence in `## Calling the service` links to the new section instead of restating the numbers | edited sentence in `## Calling the service` | read the section; `scripts/check-links.sh` |

Verification actually run, and its results:

- `bash scripts/check-links.sh` → `check-links: OK` (exit 0), at the fixed point and after the
  change.
- `bash scripts/check-headings.sh` → exit 1, `docs/skills/code-review.md has 0 H1 headings`, at
  the fixed point **and** after the change — an identical, pre-existing failure in a file outside
  the diff. `docs/guide.md` is not reported by either run.

## Smell baseline (applies even when a repo documents nothing)

Two rules bind it:

- **The repo overrides.** A documented repo standard always wins; where it endorses something the
  baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature Envy"), never
  a hard violation — and, like any standard here, skip anything tooling already enforces.

Each smell reads *what it is* → *how to fix*; match it against the diff:

- **Mysterious Name** — a function, variable, or type whose name doesn't reveal what it does or
  holds. → rename it; if no honest name comes, the design's murky.
- **Duplicated Code** — the same logic shape appears in more than one hunk or file in the change.
  → extract the shared shape, call it from both.
- **Feature Envy** — a method that reaches into another object's data more than its own. → move
  the method onto the data it envies.
- **Data Clumps** — the same few fields or params keep travelling together (a type wanting to be
  born). → bundle them into one type, pass that.
- **Primitive Obsession** — a primitive or string standing in for a domain concept that deserves
  its own type. → give the concept its own small type.
- **Repeated Switches** — the same `switch`/`if`-cascade on the same type recurs across the
  change. → replace with polymorphism, or one map both sites share.
- **Shotgun Surgery** — one logical change forces scattered edits across many files in the diff.
  → gather what changes together into one module.
- **Divergent Change** — one file or module is edited for several unrelated reasons. → split so
  each module changes for one reason.
- **Speculative Generality** — abstraction, parameters, or hooks added for needs the spec doesn't
  have. → delete it; inline back until a real need shows.
- **Message Chains** — long `a.b().c().d()` navigation the caller shouldn't depend on. → hide the
  walk behind one method on the first object.
- **Middle Man** — a class or function that mostly just delegates onward. → cut it, call the real
  target direct.
- **Refused Bequest** — a subclass or implementer that ignores or overrides most of what it
  inherits. → drop the inheritance, use composition.

## Required report shape

Open with exactly these four lines, in this order:

```
Reviewed HEAD: 7cd4956c2e59b03c4abfa3fa1cd57f65f18e3388
Review pass: R1
Standards sources checked: repo/CLAUDE.md; /implement contract (t1c/prompt.md, "Contract you must follow"); retained run sheet (AC1-AC4); smell baseline
Workflow evidence checked: /implement contract "Build and verify" -> retained run sheet rows AC1-AC4 and their recorded check-links/check-headings results
```

Both coverage lines are owed even if you report no actionable findings.

Every actionable finding must carry, on its own lines:

```
Finding ID: R1-standards-<ordinal>
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Ordinals are assigned in report order starting at 1. Set `TDD re-entry required: yes` only when
satisfying the finding requires changing observable behavior; otherwise `no`. These fields
classify and route findings only — `/tdd` and `/implement` retain ownership of repair mechanics.
Do not edit anything.

## Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented standard:
cite the standard (file + the rule); (b) any baseline smell you spot: name it and quote the hunk;
and (c), where workflow sources constrain tests or evidence, whether changed tests and evidence
rows honor the agreed seams, runnable commands, and evidence fields: cite the workflow source and
quote the changed hunk or row. Distinguish hard violations from judgement calls —
documented-standard breaches can be hard, but baseline smells are always judgement calls, and a
documented repo standard overrides the baseline. Skip anything tooling enforces. Under 400 words.

If you find nothing actionable, say `No actionable findings` after the four header lines.
