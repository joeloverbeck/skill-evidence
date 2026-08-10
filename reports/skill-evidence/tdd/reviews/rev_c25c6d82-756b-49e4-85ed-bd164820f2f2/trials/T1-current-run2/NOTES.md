# `redact` — TDD record

All commands below run with the workspace root as the working directory.

Stable command keys (referenced by the evidence rows):

- `SUITE` = `python3 -m pytest packages/textkit/tests/test_redaction.py -q`
- `FOCUS <selector>` = `python3 -m pytest "packages/textkit/tests/test_redaction.py::<selector>" -q`

Final state: `SUITE` → `6 passed`.

## Seams under test (written before any test)

| Seam | Authority |
| --- | --- |
| `redact(text: str, secrets: list[str]) -> str`, imported as `from textkit.redaction import redact` | `TASK.md` — names the exact signature and fixes the implementation file as `packages/textkit/textkit/redaction.py`. The seam is therefore pre-agreed; no seam question needed to be put to the user. |
| Runner: `python3 -m pytest packages/textkit/tests/test_redaction.py -q` | `TASK.md` |

No other seam is touched. Nothing internal is tested: `PLACEHOLDER`, `MIN_SECRET_LENGTH`,
the sorting, and the regex are all implementation details that no test names or imports.
No mocking anywhere — `redact` is a pure function with no system boundary to mock
(`method/mocking.md`).

**Ingress-path enumeration** (`method/tests.md`, "Public invariant coverage"):
`packages/textkit/textkit/__init__.py` is empty and re-exports nothing, so
`textkit.redaction.redact` is the *only* exported path that can admit a value. There is no
second constructor, parser, or deserializer that could bypass the length rule, so one
ingress path is the complete enumeration. I did not add an `__init__` re-export — TASK.md
did not ask for one, and adding one would create a second ingress path needing its own
coverage.

## TDD evidence

Every row: one seam (above), one selector, red before green, minimal implementation.

| # | Behavior (TASK.md req.) | Test file / selector | Public entry point observed | Red command → intended & observed failure | Green command → result |
| --- | --- | --- | --- | --- | --- |
| 1 | Every occurrence of every secret is replaced (req. 1) | `packages/textkit/tests/test_redaction.py` / `test_replaces_every_occurrence_of_every_secret` | `redact(text, ["hunter2", "alice"])` return value | `FOCUS test_replaces_every_occurrence_of_every_secret`. Intended: seam does not exist. Observed: `ImportError: cannot import name 'redact' from 'textkit.redaction'` | same `FOCUS` → `1 passed` |
| 2 | Case-insensitive matching (req. 2) | …/ `test_matches_secrets_regardless_of_case_and_keeps_surrounding_case` | `redact(text, ["hunter2"])` return value | `FOCUS test_matches_secrets_regardless_of_case_and_keeps_surrounding_case`. Intended: mixed-case occurrences survive. Observed: `AssertionError` — got `Login: Hunter2 then HUNTER2 and [REDACTED] (Case Kept)` | `SUITE` → `2 passed` |
| 3 | Longer secret wins at the same position (req. 3) | …/ `test_longer_secret_wins_when_both_match_at_the_same_position` | `redact(text, ["password", "password123"])` return value | `FOCUS test_longer_secret_wins_when_both_match_at_the_same_position`. Intended: the shorter secret wins and leaves a tail. Observed: `AssertionError` — got `pw=[REDACTED]123!` | `SUITE` → `3 passed` |
| 4 | Occurrences are those in `text`, so an emitted placeholder is not itself redacted (req. 1, see caveat below) | …/ `test_secret_occurring_only_inside_the_placeholder_is_not_redacted_again` | `redact(text, ["password123", "acted"])` return value | `FOCUS test_secret_occurring_only_inside_the_placeholder_is_not_redacted_again`. Intended: the second secret matches inside the placeholder the first one emitted. Observed: `AssertionError` — got `pw=[RED[REDACTED]] stays clean` | `SUITE` → `4 passed` |
| 5 | Secrets shorter than 4 characters are ignored (req. 4) | …/ `test_secrets_shorter_than_four_characters_are_ignored[three-chars-ignored]` | `redact("the cats sat, cat naps", ["cat"])` return value | `FOCUS test_secrets_shorter_than_four_characters_are_ignored`. Intended: the 3-char secret is redacted anyway. Observed: `AssertionError` — got `the [REDACTED]s sat, [REDACTED] naps` (`1 failed, 1 passed`) | `SUITE` → `6 passed` |
| 5b | Boundary partner: a 4-character secret *is* redacted (req. 4) | …/ `test_secrets_shorter_than_four_characters_are_ignored[four-chars-redacted]` | `redact("the cats sat, cat naps", ["cats"])` return value | **No red — green on arrival.** See "Deviations" | covered by row 5's `SUITE` run |

Implementation steps, one per row: (1) `str.replace` per secret; (2) case-insensitive
`re.sub` per secret; (3) sort secrets longest-first; (4) collapse to a single alternation
pass so the output is never rescanned; (5) drop secrets shorter than
`MIN_SECRET_LENGTH`, returning `text` unchanged when nothing survives the filter.

## Discriminating goldens

`method/tests.md` requires each golden to differ from the plausible rejected
implementation. I checked this by executing each rejected implementation against the
golden input rather than by reasoning about it (throwaway script, run once, then deleted).
Every golden killed its mutant:

| Golden | Rejected implementation | Its output (≠ expected) |
| --- | --- | --- |
| every-occurrence | replace first occurrence only | `user=[REDACTED] pass=[REDACTED]; retry user=alice pass=hunter2` |
| case-insensitive | lowercase the whole text before replacing | `login: [REDACTED] … (case kept)` |
| longest-wins | replace sequentially in list order | `pw=[REDACTED]123!` |
| placeholder-not-rescanned | multi-pass replace, longest secret first | `pw=[RED[REDACTED]] stays clean` |
| three-chars-ignored | threshold `len < 3` | `the [REDACTED]s sat, [REDACTED] naps` |
| four-chars-redacted | threshold `len <= 4` (off-by-one) | `the cats sat, cat naps` (unredacted) |

Two consequences worth stating plainly:

- The `longest-wins` golden lists the secrets **shortest-first** on purpose. Listed
  longest-first, the golden would also pass under the rejected sequential implementation,
  making it non-discriminating — `method/tests.md` says such a golden must be replaced, so
  I deliberately did not add the longest-first ordering as a second case.
- Row 5b exists **only** because of this rule. The 3-char case alone passes under the
  `len <= 4` off-by-one, so it does not pin the boundary; the pair does.

All expected values are independent literals written from TASK.md's stated rules, not
recomputed the way the code computes them, so none of the tests is tautological.

## Pre-review reconciliation

Changed-test inventory derived from pytest's own collection
(`python3 -m pytest packages/textkit/tests/test_redaction.py --collect-only -q`), which is
the repository's native test structure. `packages/textkit/tests/test_redaction.py` was
0 bytes before this work (verified with `wc -c` at the start), so the whole file is the
scoped change and the collected identities *are* the changed-test inventory. I did not use
git for this: running git commands was prohibited for this task.

All rows are in `packages/textkit/tests/test_redaction.py`, all at the seam
`textkit.redaction.redact` whose authority is TASK.md.

| Collected identity | Public entry point observed | Evidence row |
| --- | --- | --- |
| `test_replaces_every_occurrence_of_every_secret` | `redact(...)` return value | 1 |
| `test_matches_secrets_regardless_of_case_and_keeps_surrounding_case` | `redact(...)` return value | 2 |
| `test_longer_secret_wins_when_both_match_at_the_same_position` | `redact(...)` return value | 3 |
| `test_secret_occurring_only_inside_the_placeholder_is_not_redacted_again` | `redact(...)` return value | 4 |
| `test_secrets_shorter_than_four_characters_are_ignored[three-chars-ignored]` | `redact(...)` return value | 5 |
| `test_secrets_shorter_than_four_characters_are_ignored[four-chars-redacted]` | `redact(...)` return value | 5b |

Both-direction comparison: inventory (6 identities) minus reconciliation (6 identities) is
empty; reconciliation minus inventory is empty. No test enters through a module, helper,
mutable global, or side channel — every row calls the ratified public function and asserts
on its return value, so no unratified seam needs ratifying.

`/code-review` was not part of this task, so no review-reentry rows exist; that section of
the method applies only once a review pass produces findings.

## Deviations and judgment calls, for challenge

1. **Row 5b never went red.** Once `len < 4` is implemented, no honest change makes a
   4-character secret fail; the only way to manufacture red would be to write a threshold
   I already knew to be wrong. So the method's "red before green" and `tests.md`'s
   "discriminating goldens" pull in opposite directions here. I kept the case and labelled
   it green-on-arrival rather than dropping it or dressing it up as a cycle — it is a
   boundary guard, not a TDD cycle. It does earn its place: it is the only test that fails
   under the `len <= 4` off-by-one.
2. **Row 4 tests a behavior TASK.md only implies.** The requirement says occurrences "in
   `text`" are replaced; it never says the placeholder must not be re-redacted. I read
   "in `text`" as meaning the input, so redacting part of `[REDACTED]` would be a defect.
   A reviewer could reasonably call this out of scope — but note that without it,
   requirement 3's golden is satisfied by a multi-pass implementation that corrupts real
   output, and that corruption (`pw=[RED[REDACTED]]`) was observed, not hypothesized.
3. **`packages/textkit/conftest.py` is new.** TASK.md's stated pytest command cannot import
   `textkit` otherwise — there is no installed distribution and no packaging config. The
   conftest only puts the package root on `sys.path`; it defines no fixtures and changes no
   behavior under test.
4. **Deliberately untested, to avoid gold-plating.** The empty-`secrets` early return is
   exercised by the `three-chars-ignored` case (every secret is filtered out, leaving the
   usable list empty), so it is covered without a test of its own. Not covered, because
   TASK.md does not specify them: overlapping secrets that start at *different* positions
   (current behavior: leftmost match wins, scanning resumes after it, so matches never
   overlap), and duplicate secrets in the list (harmless).
5. **Unicode.** Case-insensitivity is delegated to `re.IGNORECASE` rather than to manual
   `str.lower()` index arithmetic, which would desynchronize for characters whose lowercase
   form has a different length. This was a free correctness win from the structure, not a
   tested claim — there is no test pinning it, and I am not claiming it is verified.
