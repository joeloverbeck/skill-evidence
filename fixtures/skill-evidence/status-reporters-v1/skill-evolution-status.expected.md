# Skill Evolution Status

Scanned 2 evidence stores read-only. Ready: 1; deferred after review: 0; blocked after eligibility: 1; indeterminate: 0; omitted as not eligible: 0.

## Ready to evolve

### .claude/skills/game-alpha

- Eligibility: three independent friction-or-worse incidents in the `execution` symptom cluster (`friction_recurrence:execution`).
- Gate provenance: First eligibility on this target hash; no completed same-hash review precedes this threshold.
- Destination proof: Paste into a session-ID-capable fresh session (Claude Code or Codex) whose top-level-session identity differs from the threshold session. A no-ID destination will be refused; waiting will not help.

Paste into another top-level session:

```text
$skill-evolution ".claude/skills/game-alpha"
```

## Eligible but blocked

### .claude/skills/game-beta — QUARANTINED

- Eligibility: one contemporaneous severe incident (`severe`).
- Gate provenance: First eligibility on this target hash; no completed same-hash review precedes this threshold.
- Blocker: 12-hour clock fallback; 11h 00m remaining.
- Eligible at: 25/07/2026, 23:00:00 UTC (UTC); 2026-07-25T23:00:00.000Z.
- Session effect: Changing sessions will not bypass this timer.
- Quarantine: Stop using this target. Immediate containment is allowed; permanent edits still require the authorized review.
