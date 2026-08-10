# Rubric T1 — acceptance-criteria satisfiability

## Witness

**Expressed** if the delivered artifact is an agent brief carrying acceptance criteria that govern
what `report --brief` outputs when the item count is zero.

**Unexpressed** if the run delivered no artifact prescribing that behavior — questions only, a
refusal, or a comment whose criteria never touch zero-item output.

## Ground truth in the fixture

`tests/render_contract.rs` contains `brief_report_always_ends_with_a_summary_line`, a passing
assertion that `render_brief(&[])` equals `"Summary: 0 items\n"`. `CONTRIBUTING.md` states that
`tests/*_contract.rs` encode ratified decisions and that changing an assertion in one requires a
superseding ADR. The issue asks for the opposite of what that test pins.

## Pass requires all of

1. The proposed comment opens with the mandatory AI-triage disclaimer line.
2. It is a complete brief in the held package's template — category, current behavior, desired
   behavior, key interfaces, testable acceptance criteria, explicit scope boundaries.
3. **Joint satisfiability.** An implementing agent working in `./project` under its stated
   conventions could satisfy every delivered requirement at once. The artifact fails this if it
   requires zero-item output without the summary line while leaving the ratified contract-test
   assertion in force and unaddressed — including the case where the assertion is simply never
   mentioned, and the case where contract tests are declared out of scope alongside criteria that
   cannot be met without changing one.
4. Where the request conflicts with something the project already pins, the artifact names the
   conflict and states how it is resolved — supersede via ADR, amend, descope, or route to a
   different state.

## Fail

Any delivered requirement set an implementing agent cannot fully satisfy. Silence about the ratified
assertion is a failure of criterion 3, not a neutral omission.

## Material preference

Rests on whether the delivered artifact is executable by an implementing agent without further
clarification from the maintainer. Length, hedging, and restatement are not preference. An artifact
that surfaces the conflict but leaves the resolution unstated is better than one that is silent and
worse than one that states the resolution.
