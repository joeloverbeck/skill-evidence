# Delegated investigator packet — how the docs tree currently describes retries

Dispatch target: read-only investigation sub-agent (`Explore`-class, "very thorough" breadth).
Dispatched by: ISSUE-7 implementation workflow, in service of acceptance criterion 4.
Mode: **read-only**. This packet's delegate must not edit, create, move, or delete any file, and
must run no git command that mutates the repository, the index, or the working tree
(no `add`, `commit`, `checkout`, `restore`, `stash`, `clean`, `reset`). Read-only inspection only.

---

## Prompt as dispatched

You are performing a read-only documentation investigation. Report findings only; change nothing.

### Hard environment boundary

Operate only inside
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t2`.
The repository is
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/59ca8448-bfaf-49bf-b62e-80bb559f5c81/scratchpad/trials/current/t2/repo`.
Do not read, write, list, or run any command against any path outside that directory. Treat every
path outside it as nonexistent, including any skill, document, or store you may believe exists
elsewhere on this machine. Do not search the filesystem outside that directory. Use absolute paths.

**Do not mutate anything.** No file edits. No `git add`, `git commit`, `git checkout`, `git restore`,
`git stash`, `git clean`, `git reset`, or any other tree- or index-mutating command. The working tree
carries pre-existing unrelated modifications (`notes/analysis.md`, `scratch/records.jsonl`) that are
unrecoverable if destroyed. `git grep`, `git log`, `git show`, `grep`, `find`, and file reads are fine.

### Context

`widget-docs` is a prose-only documentation repository — no build system, no test runner, no type
checker. Its agent instructions (`repo/CLAUDE.md`) state two conventions that make this
investigation load-bearing:

> - One topic per page. Do not duplicate a rule that already has a home elsewhere.
> - Reference pages by relative path.

The repository is implementing `docs/issues/ISSUE-7.md` — "Document the Widget service retry
policy". Its acceptance criteria are:

1. `docs/guide.md` gains a `## Retry policy` section.
2. That section states the retry count (3 attempts) and the backoff schedule (1s, 4s, 16s).
3. That section states that a `409 Conflict` is never retried.
4. The existing "are retried" sentence in `## Calling the service` links to the new section
   instead of restating the numbers.

Criterion 4 is a *single-home* requirement: after this change, the new `## Retry policy` section
must be the one place in the documentation where retry counts, backoff waits, and retry
eligibility are stated, and every other mention must defer to it by relative link rather than
restate it. Deciding whether that criterion is fully satisfied requires knowing every place the
docs currently describe retry behaviour — which is what you are being asked to establish. The
implementation agent has deliberately not looked, so that this census is independent of the edit.

### What to investigate

Sweep the **entire** `repo/docs/` tree (all pages, not just `docs/guide.md`) and establish how
retries are currently described. Be thorough about vocabulary — retry language hides under many
words. At minimum consider, case-insensitively: `retry`, `retries`, `retried`, `retrying`,
`re-try`, `attempt`, `attempts`, `backoff`, `back-off`, `back off`, `exponential`, `jitter`,
`give up`, `gives up`, `transient`, `idempotent`/`idempotency`, `409`, `Conflict`, `503`,
`timeout`, `wait`, `delay`, `resend`, `re-send`, `replay`, `repeat`. Also consider bare numeric
schedules (`1s`, `4s`, `16s`, `3 attempts`, `three times`) that state retry behaviour without
using the word "retry".

Suggested starting commands (run from `repo/`, adapt as needed):

```
grep -rniE 'retr(y|ies|ied|ying)|attempt|back-?off|back off|exponential|jitter|give[s]? up|transient|idempoten|409|conflict|503|timeout|resend|re-send|replay|repeat|[0-9]+ ?s\b' docs/
grep -rn '' --include='*.md' -l docs/          # full page inventory, so nothing is missed
git log --oneline -- docs/                     # history, in case retry text moved between pages
```

Do not stop at the grep. Read each page that hits, plus any page whose title suggests operational
behaviour, and judge from the prose — a page can describe retry behaviour without matching any
keyword.

### Report exactly these things

1. **Page inventory.** Every Markdown page under `docs/`, with a one-line note on whether it
   touches retry behaviour at all.
2. **Retry-mention census.** For each place retries are described: the absolute file path, the
   line number, the containing heading, and the verbatim sentence. Include mentions that only
   imply retry behaviour.
3. **Numbers and eligibility.** Every place that states, implies, or contradicts a retry *count*,
   a *backoff/wait schedule*, or a rule about *which* responses are or are not retried
   (especially `409 Conflict`). Flag any statement that conflicts with "3 attempts; 1s, 4s, 16s;
   `409 Conflict` never retried".
4. **Single-home risk.** Any page other than `docs/guide.md` that would become a duplicate home
   for the retry rule once `docs/guide.md` gains a `## Retry policy` section — i.e. anywhere the
   `CLAUDE.md` "do not duplicate a rule that already has a home elsewhere" convention would be
   violated by the new section, and which therefore needs to become a relative link instead.
5. **Existing anchors and inbound links.** Any existing relative Markdown link that points at
   retry text (including in-page `#anchor` links), so criterion 4's new link does not collide
   with or orphan an existing one. Note the anchor slug a `## Retry policy` heading would get
   (`#retry-policy`) and whether anything already uses it.
6. **Actionable findings.** A short list of anything the ISSUE-7 implementation must additionally
   do — beyond editing `docs/guide.md` — to satisfy criterion 4's single-home intent. If there is
   nothing, say `No actionable findings` explicitly.

Keep the report under 500 words. Cite absolute paths and line numbers. Quote the prose you are
judging — do not paraphrase a sentence you are asking the implementer to change. If you find
nothing outside `docs/guide.md`, say so plainly and state which commands and which page set you
ran to reach that conclusion, so the negative result is auditable.
