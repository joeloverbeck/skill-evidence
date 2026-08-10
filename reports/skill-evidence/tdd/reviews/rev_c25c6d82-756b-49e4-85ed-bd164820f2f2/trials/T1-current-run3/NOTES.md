# TDD record — `redact()` in `packages/textkit/textkit/redaction.py`

All commands below are complete and runnable from the workspace root
(`.../runs/T1-A-run3`). They are repeated verbatim in each row so every row replays
on its own.

## Seams under test, and their authority

| Seam | Authority |
| --- | --- |
| `redact(text: str, secrets: list[str]) -> str`, exported from module `textkit.redaction` | `TASK.md` — "Requirements for `redact(text: str, secrets: list[str]) -> str`" plus the project-layout line naming `packages/textkit/textkit/redaction.py` as where the implementation goes. This is an explicit user decision, so the seam was already agreed and no seam question was needed. |

Every one of the four requirements is stated as a property of that one function's
return value, so the whole task sits at a single seam. No test reaches into module
internals: `PLACEHOLDER`, `MIN_SECRET_LENGTH`, the length filter, and the regex
alternation are all implementation detail, and none of them is named by any test.
The implementation was rewritten twice (per-secret `str.replace` loop → per-secret
`re.sub` loop → single alternation pass) and no already-green test needed editing —
that is the refactor-survival property the seam is supposed to buy.

### Public invariant coverage — ingress enumeration

The invariant "text returned by `redact` contains no occurrence of a usable secret"
has exactly one exported ingress path. `packages/textkit/textkit/__init__.py` is
empty and was left empty, so it re-exports nothing; there is no parser, constructor,
factory, or deserializer that can produce or admit a redacted string by another
route. The tests therefore enter through the only path that exists, which is also
the direct/bypass path: `from textkit.redaction import redact`. No refusal path
exists in this API (the function has no error branch), so the refusal-and-
non-observation check does not apply.

## TDD evidence

One row per behavior, in cycle order. Observed public entry point for every row is
`textkit.redaction.redact(text, secrets)`, reached via
`from textkit.redaction import redact` at `packages/textkit/tests/test_redaction.py:1`.
Seam authority for every row is the single seam above (`TASK.md`), with the
requirement number that fixes the expected value called out per row.

### Row 1 — every occurrence of a secret is replaced (TASK.md requirement 1)

- Test: `packages/textkit/tests/test_redaction.py::test_every_occurrence_of_a_secret_is_replaced`
- Red: `python3 -m pytest "packages/textkit/tests/test_redaction.py::test_every_occurrence_of_a_secret_is_replaced" -q`
  - Intended failure: the seam does not exist yet, so the import of `redact` fails.
  - Observed failure: `ImportError: cannot import name 'redact' from 'textkit.redaction' ... Did you mean: 'redaction'?` → `1 error in 0.01s` (collection error).
- Green: same command. Result: `1 passed in 0.00s`. Implementation: per-secret
  `str.replace(secret, "[REDACTED]")` loop.
- Suite after: `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → `1 passed in 0.00s`.

### Row 2 — matching is case-insensitive (TASK.md requirement 2)

- Test: `packages/textkit/tests/test_redaction.py::test_matching_is_case_insensitive`
- Red: `python3 -m pytest "packages/textkit/tests/test_redaction.py::test_matching_is_case_insensitive" -q`
  - Intended failure: exact-case matching redacts only the lowercase occurrence and leaves `Hunter2` and `HUNTER2` standing.
  - Observed failure: `AssertionError`, `- [REDACTED] [REDACTED] [REDACTED]` / `+ Hunter2 HUNTER2 [REDACTED]` → `1 failed in 0.01s`.
- Green: same command. Result: `1 passed in 0.00s`. Implementation: swapped the
  loop body to `re.sub(re.escape(secret), ..., flags=re.IGNORECASE)`.
- Suite after: `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → `2 passed in 0.00s`.

### Row 3 — the longer secret wins at the same position (TASK.md requirement 3)

- Test: `packages/textkit/tests/test_redaction.py::test_the_longer_secret_wins_when_two_match_at_the_same_position`
- Red: `python3 -m pytest "packages/textkit/tests/test_redaction.py::test_the_longer_secret_wins_when_two_match_at_the_same_position" -q`
  - Intended failure: replacing secrets one at a time in list order consumes `pass` first and strands the tail `word`.
  - Observed failure: `AssertionError`, `- my [REDACTED] here` / `+ my [REDACTED]word here` → `1 failed in 0.01s`.
- Green: same command. Result: `1 passed in 0.00s`. Implementation: replaced the
  per-secret loop with a single left-to-right `re.sub` over one alternation whose
  branches are sorted longest-first (Python alternation is leftmost-first, so the
  longest branch wins at a given start position).
- Suite after: `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → `3 passed in 0.00s`.

### Row 4 — a secret shorter than four characters is ignored (TASK.md requirement 4)

- Test: `packages/textkit/tests/test_redaction.py::test_a_secret_shorter_than_four_characters_is_ignored`
- Red: `python3 -m pytest "packages/textkit/tests/test_redaction.py::test_a_secret_shorter_than_four_characters_is_ignored" -q`
  - Intended failure: with no length rule in the code, the 3-character `cat` is redacted alongside `lion`.
  - Observed failure: `AssertionError`, `- cat and [REDACTED]` / `+ [REDACTED] and [REDACTED]` → `1 failed in 0.01s`.
- Green: same command. Result: `1 passed in 0.00s`. Implementation: added
  `MIN_SECRET_LENGTH = 4` and filtered `secrets` to `len(secret) >= MIN_SECRET_LENGTH`.
- Suite after: `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → `4 passed in 0.00s`.

### Row 5 — text is untouched when every secret is too short (TASK.md requirement 4, "ignored **entirely**")

- Test: `packages/textkit/tests/test_redaction.py::test_text_is_untouched_when_every_secret_is_too_short`
- Red: `python3 -m pytest "packages/textkit/tests/test_redaction.py::test_text_is_untouched_when_every_secret_is_too_short" -q`
  - Intended failure: once the length filter discards every secret, the alternation
    is the empty pattern, which matches at every position instead of nowhere.
  - Observed failure: `AssertionError`, `- the cat sat on the mat` /
    `+ [REDACTED]t[REDACTED]h[REDACTED]e[REDACTED] [REDACTED]c[REDACTED]a[REDACTED]t...`
    (placeholder inserted between every character) → `1 failed in 0.01s`.
- Green: same command. Result: `1 passed in 0.00s`. Implementation: `if not usable: return text`.
- Suite after: `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → `5 passed in 0.00s`.

This row is the one that earned its keep. Row 4 left a real defect behind — an
all-too-short secret list shredded the text into per-character placeholders, which
is exactly the outcome requirement 4 exists to prevent. Nothing in rows 1-4 could
have caught it, because they all pass at least one usable secret.

## Discriminating-golden checks

Every expected value is a hand-written literal taken from the requirement, never
recomputed the way the code computes it. Rows 2-5 each observed their rejected
alternative directly as the red output, which is the strongest form of this check.
Two goldens needed a separate probe, because their red did not exercise the rejected
behavior; both probes mutated the finished implementation, confirmed the test
failed, and were then reverted.

| Golden | Rejected alternative | Probe | Outcome |
| --- | --- | --- | --- |
| Row 1 (`"hunter2 then hunter2 again"`) | Replace only the first occurrence. Row 1's red was an `ImportError`, so it never disagreed with a working-but-wrong implementation. | Added `count=1` to the `re.sub` call, then `python3 -m pytest "packages/textkit/tests/test_redaction.py::test_every_occurrence_of_a_secret_is_replaced" -q` | `1 failed in 0.01s` — mutant killed. Reverted. |
| Row 4 (`"cat and lion"`) | Off-by-one threshold that also drops 4-character secrets. Row 4's red showed the no-rule side only, not the too-strict side. | Set `MIN_SECRET_LENGTH = 5`, then `python3 -m pytest "packages/textkit/tests/test_redaction.py::test_a_secret_shorter_than_four_characters_is_ignored" -q` | `1 failed in 0.01s`, observed `+ cat and lion` — mutant killed. Reverted. |

Final state after reverting both probes:
`python3 -m pytest packages/textkit/tests/test_redaction.py -q` → `5 passed in 0.00s`.

Row 3's input is deliberately chosen so list order opposes length order
(`["pass", "password"]`): with the secrets listed longest-first, the rejected
sequential implementation would have passed by luck and the golden would not
discriminate.

## Pre-review reconciliation

Inventory derivation: git is unavailable in this environment, so the scoped diff
could not be taken from version control. The baseline is not in doubt —
`packages/textkit/tests/test_redaction.py` was 0 bytes at task start, so every
currently collected test in it is a changed test. The inventory is derived from
pytest's own collection, which is this project's native test structure:

`python3 -m pytest packages/textkit/tests/test_redaction.py --collect-only -q` → 5 tests collected:

1. `packages/textkit/tests/test_redaction.py::test_every_occurrence_of_a_secret_is_replaced`
2. `packages/textkit/tests/test_redaction.py::test_matching_is_case_insensitive`
3. `packages/textkit/tests/test_redaction.py::test_the_longer_secret_wins_when_two_match_at_the_same_position`
4. `packages/textkit/tests/test_redaction.py::test_a_secret_shorter_than_four_characters_is_ignored`
5. `packages/textkit/tests/test_redaction.py::test_text_is_untouched_when_every_secret_is_too_short`

Reconciliation rows — one per changed test selector:

| # | Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `packages/textkit/tests/test_redaction.py` | `test_every_occurrence_of_a_secret_is_replaced` | `textkit.redaction.redact` | `TASK.md` req. 1 | Row 1 |
| 2 | `packages/textkit/tests/test_redaction.py` | `test_matching_is_case_insensitive` | `textkit.redaction.redact` | `TASK.md` req. 2 | Row 2 |
| 3 | `packages/textkit/tests/test_redaction.py` | `test_the_longer_secret_wins_when_two_match_at_the_same_position` | `textkit.redaction.redact` | `TASK.md` req. 3 | Row 3 |
| 4 | `packages/textkit/tests/test_redaction.py` | `test_a_secret_shorter_than_four_characters_is_ignored` | `textkit.redaction.redact` | `TASK.md` req. 4 | Row 4 |
| 5 | `packages/textkit/tests/test_redaction.py` | `test_text_is_untouched_when_every_secret_is_too_short` | `textkit.redaction.redact` | `TASK.md` req. 4 | Row 5 |

Two-directional comparison:

- Inventory selectors not present in the reconciliation rows: **none**.
- Reconciliation rows not present in the inventory: **none**.

Both differences are empty. Every row enters through the ratified public boundary —
no test enters via a module-private helper, a mutable global, or a side channel, and
no row claims a public behavior that is actually proved only against a private
invariant, so the coverage check passes.

## Changes with no evidence row

- `packages/textkit/tests/conftest.py` (new). Harness plumbing only: it puts
  `packages/textkit` on `sys.path`, because the workspace has no packaging config
  and `import textkit` otherwise fails at collection (verified before writing any
  test: `ModuleNotFoundError: No module named 'textkit'`). It contains no test
  selector and asserts no behavior, so it appears in no inventory or evidence row.

## Review re-entry rows

None. `/code-review` has not been run against this change, so there are no
`TDD re-entry required: yes` findings to key rows against, and the
`Findings fixed during review` comparison is vacuous. If a review follows, each
actionable finding gets its own appended row keyed by review pass, axis, and
finding — no existing row above may be rewritten or extended to absorb one.
