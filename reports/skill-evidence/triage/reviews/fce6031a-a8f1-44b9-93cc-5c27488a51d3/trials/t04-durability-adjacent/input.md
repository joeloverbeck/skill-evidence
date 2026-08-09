# Trial t04: incidental source paths must stay out of a brief

The maintainer has already directed confirmed enhancement issue #24 to `ready-for-agent`. Work
offline: draft the complete comment, but do not mutate a tracker.

Desired behavior: the public `SkillConfig` interface accepts an optional `schedule` field of type
`CronExpression`; absent schedules preserve existing behavior, invalid expressions return the
existing configuration-validation error, and serialization omits an absent field.

The current checkout implements `SkillConfig` in `src/config/skill.rs`, parses it in
`src/config/read.rs`, and tests it in `tests/config_schedule.rs`. Those locations are incidental
and may change before implementation. No ADR, schema document, package reference, or other exact
artifact is itself in scope.

Produce the comment required by the held workflow.
