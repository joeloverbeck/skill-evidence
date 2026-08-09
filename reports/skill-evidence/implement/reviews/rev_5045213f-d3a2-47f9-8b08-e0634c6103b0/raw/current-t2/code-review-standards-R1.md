# /code-review Standards axis reviewer packet — review pass R1

Dispatch target: isolated Standards reviewer sub-agent (launched in parallel with the Spec
reviewer, `packets/code-review-spec-R1.md`).
Dispatched by: `/code-review` (`repo/docs/skills/code-review.md`) step 4, embedded in the ISSUE-7
implementation workflow.
`review_pass_id`: **R1** (first allocated pass in this workflow; never reused).
Prior unresolved Standards findings retained from earlier passes: **none — R1 is the first pass.**
Mode: **read-only**. Reviewers must not edit. Report findings only; no file writes, and no git
command that mutates the repository, index, or working tree. The working tree carries pre-existing
unrelated modifications (`notes/analysis.md`, `scratch/records.jsonl`) that must survive untouched.

---

## Prompt as dispatched

You are the **Standards** axis reviewer. Review only; change nothing.

### Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t2`.
The repository is
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t2/repo`.
Do not read, write, list, or run any command against any path outside that directory. Treat every
path outside it as nonexistent, including any skill, document, or store you may believe exists
elsewhere on this machine. Use absolute paths. Run no mutating git command
(`add`, `commit`, `checkout`, `restore`, `stash`, `clean`, `reset`, `amend`).

### Pinned endpoints

- `fixed_point_sha`: `1898fc8f729e81e4d0971cdd0875336cfee8386d` (supplied by the invoking
  workflow: the repository `HEAD` before any edit)
- `reviewed_head_sha`: `941aabbad333dfacb3fd5b495f9ac377325bd4d3`

Build every review input from those captured SHAs. Pinned commands (run from the repository root):

```
git diff 1898fc8f729e81e4d0971cdd0875336cfee8386d...941aabbad333dfacb3fd5b495f9ac377325bd4d3
git log 1898fc8f729e81e4d0971cdd0875336cfee8386d..941aabbad333dfacb3fd5b495f9ac377325bd4d3 --oneline
```

Pinned commit list:

```
941aabb Document the Widget service retry policy (ISSUE-7)
```

Pinned full diff:

```diff
diff --git a/docs/guide.md b/docs/guide.md
index 6d482a1..923419c 100644
--- a/docs/guide.md
+++ b/docs/guide.md
@@ -2,9 +2,19 @@

 ## Calling the service

-Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.
+Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried; see
+[Retry policy](#retry-policy).

 ## Errors

 The service returns `400` for a malformed body, `409 Conflict` when a widget with the
 same key already exists, and `503` when the upstream store is unavailable.
+
+## Retry policy
+
+A failed call gets up to 3 retry attempts. The wait before each attempt follows a fixed
+backoff schedule: 1s, then 4s, then 16s. A call that fails on the third attempt is
+reported as failed and is not retried again.
+
+A `409 Conflict` is never retried. The widget key already exists, so repeating the call
+cannot change the outcome.
```

### Applicable standards sources selected for this run (step 3)

Check the diff against **every** entry below. Read each one before reporting; do not assume its
content from this summary.

1. `repo/CLAUDE.md` — root agent instructions for `widget-docs`. Verbatim, in full:

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

2. `repo/docs/agents/issue-tracker.md` — tracker conventions (issues are Markdown files under
   `docs/issues/`; the file body is authoritative; read by reading the file; comment by appending
   a `## Comments` entry; close by setting `status:` in front matter and appending a closing
   comment naming the implementation SHA; never infer state from anything but an exact read).
   Applicable because this workflow read `docs/issues/ISSUE-7.md` and because any tracker artifact
   appearing in the diff would be bound by it.

3. The active invoking `/implement` workflow contract (embedded-use standards source). The clauses
   that bind this change: read agent instructions and the authoritative issue body first; record
   the user-supplied review fixed point; run unscoped `git status --short` and classify and
   preserve all pre-existing changes; keep implementation scope explicit; discover the
   repository's native verification surface rather than assuming one; in prose or no-build
   repositories use the checks named by the repository instructions and record exact structural
   or manual checks and results when no automated harness exists; put only the scoped
   implementation into `HEAD` before review; reconcile unscoped `git status --short`,
   `git diff --cached --name-only`, and `git diff --cached --check` immediately before every
   commit or amend; require an empty index and inspect the pinned range immediately before review.

4. Caller-selected run sheet: `<trial-root>/run-sheet.md` (ISSUE-7 run sheet — criterion-to-seam
   -to-verification table, pre-existing working-tree classification, and the recorded verification
   surface). Treat it as the agreed-seams/agreed-commands evidence artifact for this workflow.

5. `/tdd` contract — **not applicable**: no TDD harness exists in this prose repository, no tests
   changed, and no TDD evidence row is in scope.

6. The **smell baseline** below (always carried, even where the repo documents nothing).

### Smell baseline (pasted in full)

A fixed set of Fowler code smells (_Refactoring_, ch.3). Two rules bind it:

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

In a prose repository these translate to their documentation analogues where they translate at
all (duplicated prose rule, page edited for several unrelated reasons, section added for a need
the spec doesn't have, a heading whose name doesn't reveal its content). Say so explicitly when a
smell does not translate rather than forcing a match.

### Report format (required)

Open with these lines, in this order:

```
Reviewed HEAD: 941aabbad333dfacb3fd5b495f9ac377325bd4d3
Review pass: R1
Standards sources checked: repo/CLAUDE.md; repo/docs/agents/issue-tracker.md; active /implement workflow contract; <trial-root>/run-sheet.md; smell baseline
Workflow evidence checked: <applicable workflow source -> retained evidence row or identity>
```

Both coverage lines are owed even by a `No actionable findings` report. `Workflow evidence
checked: N/A` is valid only if no selected workflow source constrains changed tests or evidence;
here the `/implement` contract and the run sheet do constrain the recorded structural checks, so
name the run-sheet row or evidence identity you actually checked.

Every actionable finding must carry:

- `Finding ID: R1-standards-<ordinal>` (ordinal assigned in axis-report order; IDs are immutable)
- `Repair class: observable behavior | behavior-neutral`
- `TDD re-entry required: yes | no` (`yes` only when satisfying the finding requires changing
  observable behavior)

These fields classify and route only. Do not edit anything; `/tdd` and `/implement` retain
ownership of red → green and repair mechanics.

### Brief

Report — per file/hunk where relevant — (a) every place the diff violates a documented standard:
cite the standard (file + the rule); (b) any baseline smell you spot: name it and quote the hunk;
and (c), where workflow sources constrain tests or evidence, whether changed tests and evidence
rows honor the agreed seams, runnable commands, and evidence fields: cite the workflow source and
quote the changed hunk or row. Distinguish hard violations from judgement calls — documented-
standard breaches can be hard, but baseline smells are always judgement calls, and a documented
repo standard overrides the baseline. Skip anything tooling enforces. Under 400 words.

### Context the reviewer should have about tooling results

`scripts/check-links.sh` and `scripts/check-headings.sh` were run from the repository root against
this change. `check-links.sh` printed `check-links: OK` (exit 0). `check-headings.sh` exits 1
solely on `docs/skills/code-review.md has 0 H1 headings`; that file is untouched by this diff and
had 0 H1 headings at `1898fc8f729e81e4d0971cdd0875336cfee8386d`, so the failure is a pre-existing
baseline condition, not a regression. `docs/guide.md` has exactly one H1. Skip anything these two
scripts already enforce.
