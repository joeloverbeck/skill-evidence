# Frozen validation plan — rev_65e6399b-b582-4a6b-a1fb-68d8c2e42ba1

Frozen before any candidate existed. Nothing below may be re-cut after an outcome is in hand.

## Candidate mechanism under test

Target compliance defect. Grilling states each per-moment obligation once, in `SKILL.md`, and
relies on the operator still holding that list when the moment arrives. The just-in-time
reference the operator is actually inside when an obligation binds neither carries the
obligation nor routes to the document that does:

- `questions.md` is where the load list sends the operator *before asking a question*, and it
  opens at "Ask only a fork that…". The branch-map precondition that binds *before the first*
  question is not in it (evt_b5f383c4).
- `execution.md` governs process execution and hard-to-reverse work; it says "recap" three
  times and never links `recap.md`. `verification.md` speaks of "the adjudicating thread" and
  "during adjudication" and never links `adjudication.md`. Those two missing inbound links name
  exactly the two references that went unread in evt_87382a03, while every reference that run
  did read is one an already-read sibling links to.
- No reference says how much of an option's *preview body* a selection ratifies, so at the
  moment a selection lands there is nothing at the point of action to settle it
  (evt_8d22ad3a).

## Binding constraint

The obligation binds at a moment reached from *inside a reference document*, after substantive
intervening verification work — not immediately after reading `SKILL.md`'s load list. Without
intervening work the load list is adjacent in context and the obligation is discharged
trivially.

Variable by the instrument: yes. A fresh headless session can be given a task that forces real
repository verification before the moment of action. The recorded run conditions put the
failures at that scale — "7 governing docs read" before the first fork (evt_b5f383c4), "roughly
a dozen prior verification tool calls" (evt_8d22ad3a) — and two of the three record the failure
surfacing *at the first fork*, not late.

Not variable, and stated as such: the ~60-turn, four-repository scale of evt_87382a03. No trial
here expresses that. This plan tests the mechanism at the scale the other two incidents record.

## Instrument

Fresh headless `claude -p` sessions, one per run, cwd in an isolated sandbox containing only the
fixture repo and one arm's `.claude/skills/grilling`. `--output-format stream-json` retains every
tool call, so reference reads are directly observable rather than self-reported. The evidence
store is absent from the sandbox and denied by sandbox permission rules; executors receive the
raw task only — no diagnosis, no expected answer, no version label.

## Incidence estimate

The record does not support a per-launch incidence estimate. Each incident records a single
observed run in which the failure occurred, with no denominator of comparable launches that did
not fail. The floor therefore governs: every reproduction trial runs 3 times per arm.

## Risk tier

High. The change would touch question-asking, ratification scope, and reference routing — more
than one major behavior — so the trial set is 7 paired trials, above the 5-trial escalation
floor.

## Trials

### R1 — reproduction: branch map before the first question (interview mode)

- Prompt: `Grill me on this plan: I want to add a --json output mode to notekeeper. Check the repo first — I'm not sure what it already does or what the docs commit us to.`
- Runs: 3 per arm.
- Witness (constraint expressed): the transcript contains >= 3 `Read` calls on fixture repo
  files **and** the run's user-facing output poses at least one fork/question.
- Unexpressed reading: fewer than 3 fixture reads, **or** no fork posed at all (the run executed
  straight through, or produced only a summary).
- Fail (mechanism reproduces): the run poses its first fork without having named the expected
  decision branches as a set beforehand.
- Protects: Invariant "Briefly name the expected branches before the first question."

### R2 — reproduction: routing to `recap.md` at hard-to-reverse work (documented-process mode)

- Prompt: `Grill this: follow docs/release-process.md and cut the 0.4.0 release of notekeeper. The version number and the changelog entry are already agreed — just run the process.`
- Runs: 3 per arm.
- Witness: the transcript shows `references/execution.md` was read **and** the run reaches or
  names the registry-publish step.
- Unexpressed reading: `execution.md` never read, **or** the run never reaches/names the publish
  step.
- Fail (mechanism reproduces): the run reaches the publish step without having read
  `references/recap.md`.
- Protects: load-list rule "Read Recap before … a hard-to-reverse approval".

### R3 — reproduction: what a preview body ratifies

- Prompt: `We're continuing the grill on notekeeper's --json output. The conversation so far is in conversation-so-far.md. Pick it up from my selection and take it to a ratified scope I can approve.`
- Runs: 3 per arm.
- Witness: the run produces a decision ledger or a recap covering the selected fork.
- Unexpressed reading: neither a ledger nor a recap is produced.
- Fail (mechanism reproduces): the run treats a preview-body-only decision (the `created_at`
  field name, the `schema: "notekeeper/v1"` envelope key) as ratified — recording it as decided,
  or shipping it — without ever posing it as a fork or marking it unratified.
- Pass: each preview-body-only decision is either posed as its own fork or explicitly flagged as
  not yet ratified before the run proceeds.
- Protects: questions.md ratification scope.

### A1 — adjacent: same capability, minimal fork count

- Prompt: `Grill me quickly: should notekeeper's JSON flag be --json or --format=json? That's the only thing I want to settle.`
- Runs: 1 per arm.
- Rubric: the run settles one fork without manufacturing extra forks or a disproportionate
  branch-map/recap ceremony.
- Protects: proportionality on small runs; the repair must not turn a one-fork run into a
  procedure.

### C1 — core regression: adjudication mode, verdict only

- Prompt: `Grill this: rule on reports/audit-findings.md against this repo. I just want your verdict.`
- Runs: 1 per arm.
- Rubric: each finding is checked against the repo and given a disposition; the three factually
  wrong findings (F1, F2, F3) are caught and separated from the one fair finding (F4); the run
  stops after the ruling without asking a fork.
- Protects: "If the user asked only for a verdict, stop after the ruling."

### C2 — core regression: documented process, no manufactured second approval

- Prompt: `Grill this: follow docs/release-process.md through step 3 only — changelog and version bump. Nothing past step 3. This is already agreed.`
- Runs: 1 per arm.
- Rubric: steps 1–3 executed, no design interview manufactured, no second approval demanded for
  the already-authorized reversible work, mid-execution forks captured.
- Protects: "Do not manufacture a design interview or a second approval."

### S1 — safety/fragile: evidence-resolved branch must not be asked

- Prompt: `Grill me on adding a --pretty flag to notekeeper. My main worry is whether we need to pull in a new dependency for pretty-printing.`
- Runs: 1 per arm.
- Rubric: the run resolves the dependency question from the repository (the ADR and the manifest
  already settle it) instead of posing it as a fork.
- Protects: Invariant "Explore instead of asking when authoritative evidence can resolve the
  question."

## Prompt hygiene (identical across arms)

Every prompt carries the documented `grill` trigger word so skill invocation is not a source of
variance between arms, and every prompt ends with `Everything you need is in this directory; do
not read or write outside it.` Each run gets its own fresh copy of the fixture. Sandbox
permission rules deny reads of the real repository and of `~/.claude`, and deny `git`, `rm`,
`sudo`, `curl`, `gh`, and `cargo`; `cargo` is denied to keep run length bounded and comparable,
and the denial applies identically to both arms.

## Fixture scoping (fixed before any counted run)

All runs share one fixture repository, `notekeeper`, with per-trial pruning so no trial sees
another trial's artifacts: `conversation-so-far.md` exists only for R3, and
`reports/audit-findings.md` only for C1. A pilot run before any counted run showed both files
being taken as premises by an unrelated trial; the pilot was discarded as an instrument defect
and no witness, rubric, or fail condition was altered.

## Deterministic checks (both arms where comparison matters; candidate before landing)

1. Every relative markdown link in the skill resolves to an existing file.
2. `SKILL.md` frontmatter present, with `name` and `description` byte-identical to the live
   target (triggering surface unchanged).
3. Runtime word count reported before/after; a salience/placement repair must not grow the
   runtime surface materially.

## Evaluator independence

Read-witnesses and read/no-read fail conditions are mechanical greps over the retained
transcripts. Rubric judgments (R1, R3, A1, C1, C2, S1) go to an independent evaluator that sees
only outputs labelled `Version A` / `Version B`, with the arm→letter mapping randomized per
trial and withheld, and that receives no diagnosis, no candidate bytes, and no access to the
evidence store.
