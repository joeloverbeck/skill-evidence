# Validation Result

- Decision: accepted.
- Risk tier: high.
- Executor runs: 22 (T1: 3 current + 3 candidate; T2-T5: 2 current + 2 candidate each).
- T1: current recurrence 3/3; candidate recurrence 0/3; witnesses expressed on all six runs.
- T2-T5: 8/8 current and 8/8 candidate outputs passed their protected readings.
- Deterministic candidate checks: frontmatter unchanged; `tests.md` and `mocking.md` byte-identical;
  both relative links resolve; SKILL.md is 1,116 words before and after.
- Regressions: none observed or established.
- Frozen-input faults: none.
- Material regression attribution: not applicable; no adverse candidate observation met the first
  arm-discriminating condition.
- Acceptance: the candidate resolves the reproduced mechanism, is noninferior on every protected
  behavior, preserves scope and ownership, passes deterministic checks, and is exactly word-neutral.
