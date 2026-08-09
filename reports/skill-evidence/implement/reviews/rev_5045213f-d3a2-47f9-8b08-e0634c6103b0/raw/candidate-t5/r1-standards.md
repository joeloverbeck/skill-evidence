# Reviewer packet — Standards axis — review pass R1

You are the **Standards** reviewer for a two-axis code review. You review only. Another
agent reviews the Spec axis independently; do not consider spec fidelity, and do not read
that agent's report.

## Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t5`.
The repository is that directory's `repo/`. Treat every path outside the trial directory as
nonexistent, including any skill, document, or store you may believe exists elsewhere on
this machine. Do not search the filesystem outside it.

## You must not modify anything

You are a reviewer. Do not edit, create, or delete any file.

**Additionally, and separately from the no-edit rule above, you are forbidden from running
any git command that mutates the working tree or the index.** Specifically forbidden:
`git add`, `git commit`, `git checkout`, `git restore`, `git switch`, `git stash`,
`git reset`, `git clean`, `git apply`. This prohibition is not implied by "don't edit
files" — it is a separate, independent rule, and it binds even if you believe a mutation
would be harmless, would be reverted, or is needed to inspect something. Uncommitted state
in this repository is unrecoverable and no later reconciliation restores it.

Read-only git (`git show`, `git diff`, `git log`, `git status`, `git rev-parse`,
`git ls-files`) is permitted.

## Pinned review inputs

- `reviewed_head_sha`: `10e38f2351165cb93bdc6436625e87d81eafcdfa`
- `fixed_point_sha`: `ea885e890a7cd032311da701807a817333cc3542` (supplied by the invoking
  implementation workflow as the review fixed point)

Build every review input from these captured SHAs. Do not substitute a symbolic `HEAD`.

Pinned full diff command:

```
git -C <trial>/repo diff ea885e890a7cd032311da701807a817333cc3542...10e38f2351165cb93bdc6436625e87d81eafcdfa
```

Pinned commit list command, and its result:

```
git -C <trial>/repo log ea885e890a7cd032311da701807a817333cc3542..10e38f2351165cb93bdc6436625e87d81eafcdfa --oneline
10e38f2 Document the Widget service retry policy (ISSUE-7)
```

Changed paths in range: `docs/guide.md` (only).

## Standards sources selected for this run

Check the change against **all** of these:

1. `repo/CLAUDE.md` — the repository's root agent instructions. Both sections bind:
   - §Verification surface — `scripts/check-links.sh` and `scripts/check-headings.sh` are
     the native checks; both must be run "before any commit that touches `docs/`". There is
     no build system, test runner, or type checker.
   - §Conventions — "One topic per page. Do not duplicate a rule that already has a home
     elsewhere." and "Reference pages by relative path."
2. The **smell baseline**, pasted in full below.

Workflow sources that constrain this change:

3. The active invoking implementation contract (repair/review handoff; scope discipline;
   working-tree reconciliation). Relevant retained evidence: the caller-selected run sheet
   at `<trial>/run-sheet.md`, which defines the agreed seams, the verification command per
   acceptance criterion, and the recorded baseline check results at the fixed point.

`/tdd` is **not** an applicable source for this pass: the repository is prose-only with no
test harness, no tests changed in range, and no TDD evidence rows exist.

### Recorded baseline you must use when judging the verification standard

At the fixed point `ea885e8`, **before** any edit in this range:

| Check | Exit | Output |
|---|---|---|
| `scripts/check-links.sh` | 0 | `check-links: OK` |
| `scripts/check-headings.sh` | **1** | `docs/skills/code-review.md has 0 H1 headings` |

`check-headings.sh` was **already failing at the fixed point**, on a file outside this
change's scope. Judge the change on whether it introduces a *new* failure, not on the
absolute exit code of a check that never passed. If you believe the pre-existing failure
should have been repaired inside this change, say so explicitly as a judgement call and
name the scope tension — do not report it as a hard violation introduced by this diff.

### Smell baseline (applies even where the repo documents nothing)

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

These smells are written for code. This diff is prose documentation; apply the ones that
carry over honestly (Duplicated Code, Divergent Change, Speculative Generality, Mysterious
Name all have prose analogues) and do not force the object-oriented ones onto Markdown.

## Prior unresolved findings for this axis

None. `R1` is the first review pass of this implementation workflow.

## Report format — required

Open your report with exactly these lines, in this order:

```
Reviewed HEAD: 10e38f2351165cb93bdc6436625e87d81eafcdfa
Review pass: R1
Standards sources checked: repo/CLAUDE.md; invoking implementation contract; <trial>/run-sheet.md; smell baseline
Workflow evidence checked: invoking implementation contract -> run-sheet.md acceptance-criteria rows AC1-AC4 and recorded fixed-point baseline results
```

Both coverage lines are owed **even if you have no actionable findings**. The
`Standards sources checked` line must name every selected source above and must name the
smell baseline.

Every actionable finding must carry, as its first line:

```
Finding ID: R1-standards-<ordinal>
```

with the ordinal assigned in the order findings appear in your report. Every actionable
finding must end with both of these lines:

```
Repair class: observable behavior | behavior-neutral
TDD re-entry required: yes | no
```

Set `TDD re-entry required: yes` only when satisfying the finding requires changing
observable behavior; otherwise `no`. These fields route findings only — you do not perform
repairs, and you do not run red/green cycles.

If you find nothing actionable, say `No actionable findings` after the four header lines.

## Your brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether
changed tests and evidence rows honor the agreed seams, runnable commands, and evidence
fields: cite the workflow source and quote the changed hunk or row. Distinguish hard
violations from judgement calls — documented-standard breaches can be hard, but baseline
smells are always judgement calls, and a documented repo standard overrides the baseline.
Skip anything tooling enforces. Under 400 words.
