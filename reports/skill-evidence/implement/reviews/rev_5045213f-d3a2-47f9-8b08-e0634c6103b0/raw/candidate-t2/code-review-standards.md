# /code-review — Standards axis reviewer packet (review pass R1)

Dispatch target: isolated Standards review sub-agent, launched in parallel with the Spec axis.
Dispatched by: ISSUE-7 implementation workflow, per `repo/docs/skills/code-review.md` step 4.

---

You are the **Standards** axis reviewer. You review only; you do not edit.

## Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t2`.
The repository is that directory's `repo/`. Do not read, write, list, or run any command
against any path outside it; treat every outside path as nonexistent, including any skill or
document you may believe exists elsewhere on this machine.

## Prohibited commands

The working tree carries **uncommitted, unrecoverable changes** (`notes/analysis.md`,
`scratch/records.jsonl`) that no reconciliation can restore. You must not run any git command
that mutates the working tree or index. Forbidden in any form or with any flags: `git add`,
`git commit`, `git checkout`, `git restore`, `git switch`, `git stash`, `git reset`,
`git clean`, `git apply`. This is separate from, and additional to, "do not edit files" — it
binds even when a command looks merely inspective. Read-only git is fine. Create, modify, and
delete no files anywhere.

## Pinned review inputs — use these exact SHAs, never a symbolic `HEAD`

- `reviewed_head_sha` = `14d3e8b615c7e9b10fb6483631238816fe3db412`
- fixed point = `ea885e890a7cd032311da701807a817333cc3542`
- Full diff: `git diff ea885e890a7cd032311da701807a817333cc3542...14d3e8b615c7e9b10fb6483631238816fe3db412`
- Commit list: `git log ea885e890a7cd032311da701807a817333cc3542..14d3e8b615c7e9b10fb6483631238816fe3db412 --oneline`

The change is documentation prose: one file, `docs/guide.md`.

## Applicable standards sources selected for this run

Apply exactly these, plus the smell baseline pasted below. The list is self-contained on
purpose — apply these inputs even if you can read more of the filesystem.

1. **`repo/CLAUDE.md`** — the repository's root agent instructions. Its §Verification surface
   names the only native checks (`scripts/check-links.sh`, `scripts/check-headings.sh`, both
   run from the repository root before any commit touching `docs/`). Its §Conventions bind the
   prose: "One topic per page. Do not duplicate a rule that already has a home elsewhere." and
   "Reference pages by relative path."
2. **The active invoking implementation contract**, at
   `trials/candidate/t2/prompt.md` §"Contract you must follow". The clauses that constrain this
   diff: the implementation put into `HEAD` must be *only* the scoped implementation, with
   unrelated pre-existing dirt preserved unstaged; verification must use the repository's
   discovered native surface rather than invented checks.
3. **The retained run sheet**, at `trials/candidate/t2/run-sheet.md` — the agreed
   requirement-to-seam-to-verification map for ISSUE-7, the recorded fixed point, and the
   pre-existing working-tree classification.
4. **The smell baseline** below.

No `CODING_STANDARDS.md` or `CONTRIBUTING.md` exists in this repository.
`repo/docs/agents/issue-tracker.md` was considered and **not** selected: it governs tracker
reads and mutations, and this diff changes no tracker artifact.
The `/tdd` contract was considered and **not** selected: this is a prose repository with no
test runner and no TDD harness, and the diff changes no tests.

Two rules bind the baseline:

- **The repo overrides.** A documented repo standard always wins; where it endorses something
  the baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature Envy"),
  never a hard violation — and, like any standard here, skip anything tooling already enforces.
  In this repository, `scripts/check-links.sh` and `scripts/check-headings.sh` are the tooling:
  do not hand-flag link resolution or H1 counts.

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

Translate the baseline to prose where it lands (duplicated rules across pages, a section edited
for two unrelated reasons, wording added for a need the spec doesn't have); say so plainly when
a smell has no prose analogue rather than forcing one.

## Report format

Open with these lines, in this order:

```
Reviewed HEAD: 14d3e8b615c7e9b10fb6483631238816fe3db412
Review pass: R1
Standards sources checked: repo/CLAUDE.md; trials/candidate/t2/prompt.md §Contract you must follow; trials/candidate/t2/run-sheet.md; smell baseline
Workflow evidence checked: trials/candidate/t2/prompt.md §Contract you must follow -> trials/candidate/t2/run-sheet.md requirement map rows AC1-AC4, fixed point record, and pre-existing dirt classification
```

Both coverage lines are owed even if you report no actionable findings; reproduce the source
list exactly as given above.

Prior unresolved Standards findings carried into this pass: **none** (R1 is the first pass).

## The brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented
standard: cite the standard (file + the rule); (b) any baseline smell you spot: name it and
quote the hunk; and (c), where workflow sources constrain tests or evidence, whether changed
tests and evidence rows honor the agreed seams, runnable commands, and evidence fields: cite
the workflow source and quote the changed hunk or row. Distinguish hard violations from
judgement calls — documented-standard breaches can be hard, but baseline smells are always
judgement calls, and a documented repo standard overrides the baseline. Skip anything tooling
enforces. Under 400 words.

Give every actionable finding, in axis-report order:

- `Finding ID: R1-standards-<ordinal>`
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` — `yes` only when satisfying the finding requires changing
  observable behavior; otherwise `no`.

These fields route findings only. `/tdd` and `/implement` own red → green and repair mechanics.
Do not edit, and do not propose replacement prose beyond what is needed to make a finding
actionable. If you find nothing actionable, say `No actionable findings` and still emit both
coverage lines.
