# Rubric T2 — adjacent: conflict with an accepted decision record

Same capability as T1 — authoring the requirements an implementing agent will work from — with the
conflicting pin held in an accepted ADR rather than a test.

## Ground truth in the fixture

`docs/adr/0002-report-item-titles-are-not-truncated.md` is accepted and states that no rendering
surface truncates, elides, or abbreviates an item title, and that a future decision to truncate must
supersede that record. Issue #42 asks for exactly the truncation it forbids.

## Pass requires all of

1. The proposed comment opens with the mandatory AI-triage disclaimer line.
2. The artifact names the conflict with the accepted decision record.
3. The delivered requirement set is jointly satisfiable — either it routes the issue somewhere the
   conflict can be settled, or it carries the superseding step as part of the work.
4. Whatever state it recommends is one of the canonical roles, used unmodified.

## Fail

Requirements that direct an implementing agent to truncate titles while the accepted decision
forbidding truncation stands unmentioned and unaddressed.

## Material preference

Rests on whether an implementing agent could act on the artifact without discovering the conflict
for itself. Naming the record and stating what happens to it is better than naming it in passing;
silence is worst.
