# TDD evidence — `redact(text, secrets)`

Retained per the `tdd` skill: seams and their authority, one evidence row per behavior,
replayable commands, the discriminating-golden check, and the pre-review reconciliation.

## Seams under test (written down before the first test)

| Seam | Authority | Status |
| --- | --- | --- |
| `textkit.redaction.redact(text: str, secrets: list[str]) -> str` | `TASK.md` → "Requirements for `redact(text: str, secrets: list[str]) -> str`", which fixes the module path, the signature, and all four behaviors | Ratified — no ambiguity, so no seam question was raised |

`TASK.md` is the authoritative spec for this task and plays the PRD role. It names exactly one
public entry point, so all five tests enter through `from textkit.redaction import redact`.
Nothing is tested through the private helper `_longest_match_at`, through module globals, or
through any side channel.

## Command keys

Both run from the workspace root
(`.../scratchpad/runs/T1-A-run4`).

- **`CMD_ONE(<selector>)`** = `python3 -m pytest "packages/textkit/tests/test_redaction.py::<selector>" -q`
- **`CMD_ALL`** = `python3 -m pytest packages/textkit/tests/test_redaction.py -q`

## TDD evidence rows

All rows: test file `packages/textkit/tests/test_redaction.py`; observed public entry point
`textkit.redaction.redact`.

### Cycle 1 — every occurrence of a secret is replaced

- **Seam authority**: `TASK.md` requirement 1 (first clause, "Every occurrence").
- **Selector**: `test_redacts_every_occurrence_of_a_secret`
- **Red**: `CMD_ONE(test_redacts_every_occurrence_of_a_secret)`.
  Intended failure: the seam does not exist yet, so importing it fails.
  Observed: collection error — `ImportError: cannot import name 'redact' from 'textkit.redaction'`; `1 error`.
- **Green**: same command → `1 passed`. Implementation: `str.replace` per secret.
- **Note**: the golden was strengthened from one occurrence to two *while still red* (see
  "Deviations", item 1), and red was re-observed on the final golden text before any
  implementation was written.

### Cycle 2 — matching is case-insensitive, surrounding text keeps its case

- **Seam authority**: `TASK.md` requirement 2.
- **Selector**: `test_matches_secrets_case_insensitively_and_preserves_surrounding_case`
- **Red**: `CMD_ONE(test_matches_secrets_case_insensitively_and_preserves_surrounding_case)`.
  Intended failure: case-sensitive replacement leaves `Hunter2` and `HUNTER2` in the clear.
  Observed: `1 failed` —
  expected `'Login: [REDACTED], backup [REDACTED], old [REDACTED]'`,
  got `'Login: Hunter2, backup HUNTER2, old [REDACTED]'`.
- **Green**: same command → `1 passed`; `CMD_ALL` → `2 passed`.
  Implementation: `re.sub` with `re.IGNORECASE` per secret.

### Cycle 3 — the longer secret wins at the same position

- **Seam authority**: `TASK.md` requirement 3.
- **Selector**: `test_longer_secret_wins_when_two_secrets_match_at_the_same_position`
- **Red**: `CMD_ONE(test_longer_secret_wins_when_two_secrets_match_at_the_same_position)`.
  Intended failure: sequential per-secret replacement consumes `pass` first and leaves the
  `word` tail of `password` exposed.
  Observed: `1 failed` — expected `'my [REDACTED] here'`, got `'my [REDACTED]word here'`.
- **Green**: same command → `1 passed`; `CMD_ALL` → `3 passed`.
  Implementation: replaced the per-secret loop with a single left-to-right scan that takes the
  longest candidate matching at each position.

### Cycle 4 — secrets shorter than 4 characters are ignored

- **Seam authority**: `TASK.md` requirement 4.
- **Selector**: `test_ignores_secrets_shorter_than_four_characters_but_redacts_four`
- **Red**: `CMD_ONE(test_ignores_secrets_shorter_than_four_characters_but_redacts_four)`.
  Intended failure: with no length threshold the 3-character `cat` is redacted out of ordinary text.
  Observed: `1 failed` — expected `'the cat sat by the [REDACTED]'`,
  got `'the [REDACTED] sat by the [REDACTED]'`.
- **Green**: same command → `1 passed`; `CMD_ALL` → `4 passed`.
  Implementation: `MIN_SECRET_LENGTH = 4` filter applied to the candidate list.

### Final state

`CMD_ALL` → `5 passed`.

## Coverage test with no available red (not a TDD cycle)

- **Selector**: `test_redacts_each_secret_in_the_list`
- **Seam authority**: `TASK.md` requirement 1 (second clause, "each secret").
- **Status**: green on arrival. Verified, not assumed —
  `CMD_ONE(test_redacts_each_secret_in_the_list)` → `1 passed` against the cycle-4 implementation
  with no production change.
- **Why no red exists**: the ratified signature takes `secrets: list[str]`, so cycle 1's minimal
  implementation already had to iterate the list. Manufacturing red would have meant writing
  production code I believed to be wrong (e.g. redacting only `secrets[0]`) purely to satisfy
  loop chronology. The test is retained as coverage for the clause and is recorded here as a
  non-cycle rather than dressed up as one.

## Discriminating-golden sensitivity check

`tests.md` requires a golden to fail if the forbidden behavior is substituted. Each rejected
implementation was executed against its golden rather than reasoned about. All 11 rejects
produce output differing from the expected value (`ALL GOLDENS DISCRIMINATING`):

| Golden (selector) | Rejected implementation | Its output |
| --- | --- | --- |
| every occurrence | replace first occurrence only | `'[REDACTED] logs in, then hunter2 logs out'` |
| each secret | redact `secrets[0]` only | `'user [REDACTED] key swordfish'` |
| case-insensitive | case-sensitive `str.replace` | `'Login: Hunter2, backup HUNTER2, old [REDACTED]'` |
| case-insensitive | lowercase the whole text first | `'login: [REDACTED], backup [REDACTED], old [REDACTED]'` |
| longest wins | list-order sequential replace | `'my [REDACTED]word here'` |
| longest wins | shortest match at position | `'my [REDACTED]word here'` |
| 4-char threshold | no threshold | `'the [REDACTED] sat by the [REDACTED]'` |
| 4-char threshold | ignore `len <= 4` | `'the cat sat by the gate'` |
| 4-char threshold | ignore `len < 3` | `'the [REDACTED] sat by the [REDACTED]'` |

The threshold golden carries both a 3-character and a 4-character secret in one input, so it
pins the boundary from both sides: `<= 4` and `< 3` variants each disagree with it. No golden
recomputes its expected value the way the code does; every expected value is a literal read off
the requirement.

## Pre-review reconciliation (tests ↔ seams)

Inventory source: `python3 -m pytest packages/textkit/tests/test_redaction.py -q --collect-only`
(pytest node IDs are this project's native test structure). See "Deviations", item 3, for why
collection rather than a diff supplies the inventory.

| # | Test file | Selector (node ID) | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `packages/textkit/tests/test_redaction.py` | `test_redacts_every_occurrence_of_a_secret` | `textkit.redaction.redact` | `TASK.md` req 1 (first clause) | Cycle 1 |
| 2 | `packages/textkit/tests/test_redaction.py` | `test_redacts_each_secret_in_the_list` | `textkit.redaction.redact` | `TASK.md` req 1 (second clause) | Coverage test, no available red |
| 3 | `packages/textkit/tests/test_redaction.py` | `test_matches_secrets_case_insensitively_and_preserves_surrounding_case` | `textkit.redaction.redact` | `TASK.md` req 2 | Cycle 2 |
| 4 | `packages/textkit/tests/test_redaction.py` | `test_longer_secret_wins_when_two_secrets_match_at_the_same_position` | `textkit.redaction.redact` | `TASK.md` req 3 | Cycle 3 |
| 5 | `packages/textkit/tests/test_redaction.py` | `test_ignores_secrets_shorter_than_four_characters_but_redacts_four` | `textkit.redaction.redact` | `TASK.md` req 4 | Cycle 4 |

**Both-directions comparison**: inventory = 5 node IDs; reconciliation rows = 5 node IDs.
Inventory − reconciliation = empty. Reconciliation − inventory = empty.

**Coverage check**: every row claims public behavior observed through `redact`'s return value.
No row rests on a private invariant, and no test reaches through a module, helper, mutable
global, or side channel that `TASK.md` did not ratify.

## Deviations and things worth knowing

1. **Cycle 1's golden was strengthened mid-red.** The first draft asserted a single occurrence.
   That would have made a later "every occurrence" test green on arrival against the natural
   `str.replace` implementation, so the golden was widened to two occurrences before any
   production code existed, and red was re-observed. No implementation was written against the
   weaker golden.
2. **`packages/textkit/tests/conftest.py` was added** (not listed in `TASK.md`). Without it the
   prescribed command cannot import the package: `python3 -m pytest` puts the repo root on
   `sys.path`, but `textkit` lives at `packages/textkit/textkit`. The conftest prepends
   `packages/textkit`. This was done *before* cycle 1's red so that the observed red was the
   intended `ImportError: cannot import name 'redact'` (a missing seam) rather than a
   `ModuleNotFoundError` path artifact.
3. **The changed-test inventory comes from pytest collection, not a diff.** Running git was
   forbidden for this task. The substitution is sound here because `TASK.md` declares both
   `redaction.py` and `test_redaction.py` empty at the start, which was verified (`wc -c` → 0
   bytes for both), so every collected test is a changed test.
4. **`/code-review` was not part of this task**, so the skill's review-repair rules produced no
   review-reentry rows. If a review is run later, findings marked `TDD re-entry required: yes`
   must be appended as new rows keyed by pass/axis/finding — the rows above must not be
   rewritten to absorb them.
5. **Known limit, recorded rather than claimed away.** Matching compares `slice.lower()` to
   `secret.lower()`. Advancing by `len(secret)` in the original text keeps indices safe, but a
   secret whose lowercase form has a different length than itself only matches text written in
   the same form. Verified: with secret `"İKEY"`, `"the İKEY here"` redacts, while the
   already-decomposed `"the i̇key here"` does not. No crash and no index desync; outside the
   four stated requirements, so no test pins it.
6. **Behaviors probed but deliberately left untested** (not in `TASK.md`; recorded so the gap is
   visible rather than silent): empty secret string → no infinite loop, ignored by the length
   filter; empty secrets list and empty text → returned unchanged; regex metacharacters in a
   secret → treated literally; the emitted `[REDACTED]` is not re-scanned, so the secret
   `"REDACTED"` does not cascade; adjacent and duplicate occurrences behave.
