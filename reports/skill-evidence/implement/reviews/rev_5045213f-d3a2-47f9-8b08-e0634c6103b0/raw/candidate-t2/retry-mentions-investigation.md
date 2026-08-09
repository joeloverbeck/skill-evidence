# Delegated investigator packet — how the docs tree currently describes retries

Dispatch target: read-only investigation sub-agent (`Explore`-class).
Dispatched by: ISSUE-7 implementation workflow, before any edit to `docs/guide.md`.

---

You are performing a read-only investigation for acceptance criterion 4 of ISSUE-7 in a
prose documentation repository. Report only; do not edit anything.

## Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t2`.
The repository is
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/candidate/t2/repo`.
Do not read, write, list, or run any command against any path outside that directory. Treat
every path outside it as nonexistent, including any skill, document, or store you may believe
exists elsewhere on this machine. Do not search the filesystem outside it.

## Prohibited commands — read this before you run anything

This working tree carries **uncommitted, unrecoverable changes** (`notes/analysis.md`,
`scratch/records.jsonl`) that no later reconciliation can restore.

You must not run any git command that mutates the working tree or the index. Specifically
forbidden, in any form or with any flags: `git add`, `git commit`, `git checkout`,
`git restore`, `git switch`, `git stash`, `git reset`, `git clean`, `git apply`.

This prohibition is separate from, and additional to, the instruction not to edit files: it
binds even when a command looks like it would only inspect or "safely" refresh state. Read-only
git (`git status`, `git log`, `git show`, `git diff` without `--cached` writes, `git grep`,
`git rev-parse`) is permitted. If you believe the task needs a forbidden command, stop and say
so in your report instead of running it.

Do not create, modify, or delete files anywhere, including scratch files.

## Context

`docs/guide.md` currently tells operators, in `## Calling the service`:

> Send a `POST` to `/v1/widgets` with a JSON body. Failed calls are retried.

ISSUE-7 (`docs/issues/ISSUE-7.md`) adds a `## Retry policy` section to `docs/guide.md` stating
the retry count (3 attempts), the backoff schedule (1s, 4s, 16s), and that a `409 Conflict` is
never retried; its criterion 4 requires that existing "are retried" sentence to **link** to the
new section instead of restating the numbers.

`CLAUDE.md` §Conventions binds this repository: "One topic per page. Do not duplicate a rule
that already has a home elsewhere." and "Reference pages by relative path." The new section is
intended to become the single home for the retry rule, so I need to know independently whether
any other page in the tree already claims part of that home or would contradict it.

## What to investigate

Sweep the **whole `docs/` tree** — every `.md` file under `docs/`, not just `docs/guide.md`,
and including `docs/issues/`, `docs/agents/`, and `docs/skills/`. Search broadly for retry
vocabulary rather than one keyword: `retry`, `retries`, `retried`, `retrying`, `re-try`,
`attempt`, `attempts`, `backoff`, `back-off`, `back off`, `exponential`, `wait`, `delay`,
`redeliver`, `resend`, `re-send`, `idempotent`, `idempotency`, `at-least-once`, `409`,
`Conflict`, `503`, `transient`, `give up`, `fail fast`. Case-insensitive.

Then answer, with a file path and quoted line for every claim:

1. **Inventory.** Every place in `docs/` that describes, mentions, or implies retry behavior
   for the Widget service. Quote the sentence and give `path:line`.
2. **Competing home.** Does any page other than `docs/guide.md` already state a retry rule
   (count, backoff, or which statuses are or are not retried)? If so, adding the section to
   `docs/guide.md` would duplicate a rule that already has a home — say so explicitly, and name
   the page that owns it today.
3. **Contradiction.** Does any existing sentence anywhere in `docs/` contradict "3 attempts",
   "1s, 4s, 16s", or "`409 Conflict` is never retried"? Quote it. Include statements that only
   imply a conflict (for example, a page implying every error status is retried, or that `409`
   is retried).
4. **Inbound references.** Does anything in `docs/` link to, or textually cross-reference, the
   `## Calling the service` section or its "are retried" sentence, such that rewriting that
   sentence would break or strand a reference? Include anchor links (`#calling-the-service`)
   and prose references alike.
5. **Anchor/link convention in force.** How do existing pages in this tree actually write
   cross-references — same-page anchors (`[x](#anchor)`), relative page paths
   (`[x](../agents/issue-tracker.md)`), path-plus-anchor, or bare prose? Quote real examples.
   Note whether `scripts/check-links.sh` would validate or skip each form (read the script;
   do not run it — I run the checks myself).
6. **Anchor collision.** Would the anchor `#retry-policy` collide with, or be ambiguous
   against, any existing heading in `docs/guide.md` or any heading another page already links
   to?

## Report format

Open with `Investigation: docs-tree retry descriptions`. Then one short section per numbered
question above, each finding as `path:line` plus the quoted text plus one sentence of
consequence for ISSUE-7 criterion 4.

End with an explicit verdict line, exactly one of:

- `No actionable findings — docs/guide.md ## Retry policy can be the single home; the "are
  retried" sentence can be rewritten to link to it.`
- `Actionable findings — <count>` followed by a numbered list of what must change before the
  edit proceeds.

State the search commands you ran so the sweep is reproducible. Under 400 words. Do not
propose wording for the new section — that is my job, not yours.
