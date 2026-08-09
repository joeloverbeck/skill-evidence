# Trial t02: exact durable artifacts are the work's subject

The maintainer has already directed that confirmed enhancement issue #13 move to
`ready-for-agent`. Work offline: draft the comment that the held triage workflow would post, but do
not mutate a tracker.

The ratified change has exactly three required subjects:

1. Amend the adopted decision identified as **ADR-0007, Evidence Reader Compatibility**, in place.
2. Revise the shipped **Skill Evolution / Authorized Review** reference so its review protocol
   matches that amended decision.
3. Revise the **Skill Evolution / Review Report Template** that the Authorized Review reference
   defines.

Those stable artifact identities are part of the accepted scope. Their current checkout happens to
place them at `docs/adr/0007-evidence-reader-compatibility.md`,
`.claude/skills/skill-evolution/references/authorized-review.md`, and a template section inside
that reference, but an implementation agent must rediscover current locations. Verification is
already confirmed, the category is `enhancement`, and the outcome is `ready-for-agent`.

Produce the complete proposed tracker comment, including every invariant the held workflow
requires. The executor record must make clear which artifact identities were included and whether
any current location was prescribed as an implementation step.
