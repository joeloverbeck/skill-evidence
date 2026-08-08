---
name: legacy-skill-decontamination
description: One-time behavior-preserving simplification of a skill accreted by the retired skill-audit — refuses unless legacy-eligible, and lands only a blind-validated noninferior candidate.
argument-hint: "<path of the target skill directory> [legacy basis when not the standing owner confirmation]"
disable-model-invocation: true
---

# Legacy Skill Decontamination

Determine, once per legacy baseline, whether a skill repeatedly mutated by the retired `skill-audit` can shed its accretion — incident lore, one-off defenses, duplicated safeguards, audit bookkeeping, stale environment claims, instruction collisions — without losing demonstrated capability. Eligibility is provenance, not a verdict: the target is not presumed damaged, every removal must survive blind paired trials, and `healthy_no_change` is as successful a terminal as a landed simplification. Invoking it is authorization to land a validated candidate — no second "implement now" prompt — and equally authorization for nothing at all when eligibility or validation says no.

Arguments: the path of the target skill directory (required). Optional: a legacy basis when the standing owner confirmation (every skill active in the old audit era underwent dozens of `skill-audit` rounds) does not apply — `audit-history` or `imported` with a provenance note, or `routed-review` with the routing review's event id.

## Hard boundaries

- One run per legacy baseline, ever. Never a recurring cleanup mode: not after ordinary Skill Evolution changes, not after clean or problematic uses, not for reassurance that a previous run was correct, and never justified by a skill merely looking long or inelegant.
- Until the preflight passes: never read the target semantically, inspect it for removal opportunities, propose anything, or edit anything. Repeated invocations do not create eligibility.
- Provenance is a clue, not a fact. Old audit findings, witness statements, and "field-witnessed" defenses prove an event occurred, not that its rule is necessary — reproduce or disregard them, never inherit them as trusted facts.
- The live target stays unchanged until validation passes; all analysis and edits happen on the isolated snapshot and candidate. Every decontamination is a broad change: at least five blind paired trials, always.
- Evidence is immutable: every event write goes through the compiled command; never hand-edit `events.jsonl` or `gate-status.json`, and never delete or rewrite earlier events.
- Never target this skill's own live definition, the retired `skill-audit` (archive it; never decontaminate it for continued use), or a newly written skill: no legacy audit history, no eligibility.
- One target at a time: never batch several skills into one run; each target gets its own corpus, candidate, and evidence.

## Workflow

### 1. Run the mechanical preflight

From the repository root:

```bash
cargo run --locked -p skill-evidence -- skills decontamination preflight \
  --target <skill-path> --basis owner-confirmed \
  --recorded-at <RFC3339-clock> --now-epoch-milliseconds <clock-ms> \
  --session-id <top-level-session-id> --lock-owner <caller-owned-lock-id>
```

One run is the whole check: the compiled command validates event-stream integrity, hashes the live target, confirms no other review or pending Skill Evolution authorization owns it, applies the one-time gate, and validates the basis (`--basis-note` for `audit-history`/`imported`, `--basis-ref` for `routed-review`). Clock, session, and lock identities are explicit caller inputs; the command never reads the ambient clock or generates an identity. The command cannot verify provenance itself: before claiming the standing owner confirmation, confirm the target actually predates the audit retirement — a skill created after it has no legacy history.

*Done when the compiled command printed either a refusal block or an eligibility packet.*

### 2. On refusal: relay and stop

Relay the compiled command's refusal verbatim — gate state, failed condition, terminal outcome — and end the run with no analysis, no suggestions, and no softening ("we could still look at…"). Safe refusal is success.

*Done when the refusal was relayed unchanged and nothing else was done to or about the target.*

### 3. On eligibility: run the decontamination

Read `references/eligible-run.md` and execute it end-to-end, starting by claiming the run.

*Done when exactly one terminal outcome was reached and recorded through the compiled command, the run report exists under `reports/skill-evidence/<skill-key>/decontamination/`, and the user-facing completion states whether the live skill changed.*

Campaign note: when the user asks for the next target instead of naming one, prefer frequently used, high-impact skills, then the largest or most audit-accreted, then those governing destructive or stateful actions; skip inactive skills.

Design contract for maintainers, never loaded at runtime: `archive/workflows/00_shared-skill-evolution-contract.md`; source workflow archived at `archive/workflows/03_legacy-skill-decontamination-workflow.md`.
