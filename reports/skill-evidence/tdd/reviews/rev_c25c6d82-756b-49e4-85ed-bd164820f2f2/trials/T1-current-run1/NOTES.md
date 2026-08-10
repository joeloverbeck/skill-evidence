# `redact` — TDD record

Method: `method/SKILL.md` (tdd) plus `method/tests.md` and `method/mocking.md`.

## Seam record (written before the first test)

| | |
|---|---|
| Seam | `textkit.redaction.redact(text: str, secrets: list[str]) -> str` |
| Authority | `TASK.md`, "Requirements for `redact(text: str, secrets: list[str]) -> str`", which fixes the signature and names `packages/textkit/textkit/redaction.py` as the implementation site and `packages/textkit/tests/test_redaction.py` as the test site. An explicit user decision, so the seam was already agreed and needed no confirming question. |
| Ingress paths | `method/tests.md` requires enumerating every exported path that can admit the value. `packages/textkit/textkit/__init__.py` is empty and the task asks for no re-export, so the module-level function is the only exported entry point. There is no second path to bypass it, so no bypass probe applies. |
| Mocking | None. `method/mocking.md` permits mocks only at system boundaries; `redact` is a pure string function with no boundary to cross. |

No test in this run reaches past that seam — no private helper, module global, or side channel is touched by any assertion.

## Command keys

All keys below are complete runnable commands, given this one assignment:

```
WS=/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-A-run1
```

| Key | Command |
|---|---|
| `K-SUITE` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py -q` |
| `K-COLLECT` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py --collect-only -q` |
| `K-SENS` | `python3 $WS/.sensitivity/check_goldens.py` |
| `K-1` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py::test_every_occurrence_of_a_secret_is_replaced -q` |
| `K-2` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py::test_matching_is_case_insensitive -q` |
| `K-3` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py::test_secret_is_matched_literally_not_as_a_pattern -q` |
| `K-4` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py::test_longer_secret_wins_when_both_match_at_the_same_position -q` |
| `K-5` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py::test_only_occurrences_in_the_input_text_are_redacted -q` |
| `K-6` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py::test_secret_shorter_than_four_characters_is_ignored -q` |
| `K-7` | `python3 -m pytest $WS/packages/textkit/tests/test_redaction.py::test_text_is_unchanged_when_every_secret_is_too_short -q` |

## TDD evidence rows

Every row: one seam, one test selector, one minimal implementation. Test file for
all rows is `packages/textkit/tests/test_redaction.py`; observed public entry point
for all rows is `textkit.redaction.redact(text, secrets)` called directly from the
test body.

### Row 1 — every occurrence is replaced

- **Seam authority**: TASK.md requirement 1.
- **Selector**: `test_every_occurrence_of_a_secret_is_replaced`
- **Red**: `K-1`. Intended failure: `redact` does not exist yet. Observed: collection error, `ImportError: cannot import name 'redact' from 'textkit.redaction'`. Matches.
- **Green**: `K-1` → `1 passed`. Implementation: `str.replace` per secret.

### Row 2 — matching is case-insensitive

- **Seam authority**: TASK.md requirement 2.
- **Selector**: `test_matching_is_case_insensitive`
- **Red**: `K-2`. Intended failure: mixed-case occurrences left alone. Observed: `Login with Hunter2 or HUNTER2 now` where `Login with [REDACTED] or [REDACTED] now` was required. Matches.
- **Green**: `K-2` → `1 passed`. Implementation: `re.sub(..., flags=re.IGNORECASE)`.

### Row 3 — the secret is matched literally, not as a pattern

- **Seam authority**: TASK.md requirement 1 — "every occurrence of each *secret*"; a secret is a literal string, so regex interpretation would redact text that is not the secret.
- **Selector**: `test_secret_is_matched_literally_not_as_a_pattern`
- **Red**: `K-3`. Intended failure: row 2's minimal step made the secret a live regex, so `.` matches any character. Observed: `use [REDACTED] here, not [REDACTED]` — the non-secret `p@ssXword` was redacted too. Matches.
- **Green**: `K-3` → `1 passed`. Implementation: `re.escape(secret)`.

### Row 4 — the longer secret wins at the same position

- **Seam authority**: TASK.md requirement 3.
- **Selector**: `test_longer_secret_wins_when_both_match_at_the_same_position`
- **Red**: `K-4`. Intended failure: secrets applied in the order given, so the shorter `pass` consumes the position first. Observed: `my [REDACTED]word is safe`. Matches.
- **Green**: `K-4` → `1 passed`. Implementation: iterate `sorted(secrets, key=len, reverse=True)`.

### Row 5 — only occurrences in the input text are redacted

- **Seam authority**: TASK.md requirement 1 — "every occurrence of each secret **in `text`**". A placeholder is output, not input, so a secret that never appears in `text` must match nothing.
- **Selector**: `test_only_occurrences_in_the_input_text_are_redacted`
- **Red**: `K-5`. Intended failure: substituting secrets one after another lets a later secret match inside the `[REDACTED]` an earlier one wrote. Observed: `pw [[REDACTED]ED]` — the secret `redact` matched the `REDACT` inside the placeholder, though it never occurs in the input. Matches.
- **Green**: `K-5` → `1 passed`; `K-SUITE` → `5 passed`. Implementation: single pass over the input via one alternation, branches ordered longest-first (alternation is leftmost-first, so this also keeps row 4 green).

### Row 6 — a secret shorter than four characters is ignored

- **Seam authority**: TASK.md requirement 4.
- **Selector**: `test_secret_shorter_than_four_characters_is_ignored`
- **Red**: `K-6`. Intended failure: no length rule exists, so the 3-character `cat` is shredded. Observed: `the [REDACTED] sat at [REDACTED] corp` where `the cat sat at [REDACTED] corp` was required. Matches.
- **Green**: `K-6` → `1 passed`; `K-SUITE` → `6 passed`. Implementation: filter to `len(secret) >= MIN_SECRET_LENGTH`.

### Row 7 — text is unchanged when every secret is too short

- **Seam authority**: TASK.md requirement 4 — ignored *entirely*.
- **Selector**: `test_text_is_unchanged_when_every_secret_is_too_short`
- **Red**: `K-7`. Intended failure: with every secret filtered out the alternation is the empty pattern, which matches at every position. Observed: `[REDACTED]t[REDACTED]h[REDACTED]e[REDACTED] [REDACTED]c...` — a placeholder inserted between every character. Matches.
- **Green**: `K-7` → `1 passed`; `K-SUITE` → `7 passed`. Implementation: return `text` unchanged when no secret survives the filter.

Rows 3, 5 and 7 exist because each preceding minimal step opened a real defect —
that is the tracer-bullet loop doing its job, and each red above is the defect
observed at the public seam, not a hypothetical.

## Discriminating-golden check

`method/tests.md` requires that a golden fail once the rejected behavior is
substituted. I did not assert this — I ran it. `.sensitivity/check_goldens.py`
swaps eight rejected implementations into the seam and requires the matching
golden to fail against each. Replay with `K-SENS`.

| Variant | Rejected behavior | Caught by |
|---|---|---|
| M1 | replaces only the first occurrence | row 1 |
| M2 | lowercases the whole text | row 2 |
| M3 | uses the secret as a regex | row 3 |
| M4 | applies secrets in the order given | row 4 |
| M5 | sequential substitution, longest first | row 5 |
| M6 | no minimum-length rule | row 6 |
| M7 | threshold `> 4` instead of `>= 4` | row 6 |
| M8 | no empty-secret-set guard | row 7 |

Result: `All 8 rejected variants are caught; all 7 tests pass against the real
implementation.` Both off-by-one directions on the length rule (M6, M7) are caught
by the single row-6 golden, which is why that golden carries a 3-character and a
4-character secret in one input.

No golden is tautological: every expected value is a literal string fixed by the
task requirements, never recomputed the way the implementation computes it.

## Pre-review reconciliation

Deviation, stated plainly: the method derives the changed-test inventory from the
scoped diff, and I was instructed never to run git. Substitute used —
`packages/textkit/textkit/redaction.py` and `packages/textkit/tests/test_redaction.py`
were both 0 bytes at the start of this run, so every collected test is a changed
test and the collected set is the changed set. Inventory derived with `K-COLLECT`
(pytest's own collection, the native test structure here).

| # | Selector | Observed public entry point | Seam authority | Evidence row |
|---|---|---|---|---|
| 1 | `test_every_occurrence_of_a_secret_is_replaced` | `textkit.redaction.redact` | TASK.md req 1 | Row 1 |
| 2 | `test_matching_is_case_insensitive` | `textkit.redaction.redact` | TASK.md req 2 | Row 2 |
| 3 | `test_secret_is_matched_literally_not_as_a_pattern` | `textkit.redaction.redact` | TASK.md req 1 | Row 3 |
| 4 | `test_longer_secret_wins_when_both_match_at_the_same_position` | `textkit.redaction.redact` | TASK.md req 3 | Row 4 |
| 5 | `test_only_occurrences_in_the_input_text_are_redacted` | `textkit.redaction.redact` | TASK.md req 1 | Row 5 |
| 6 | `test_secret_shorter_than_four_characters_is_ignored` | `textkit.redaction.redact` | TASK.md req 4 | Row 6 |
| 7 | `test_text_is_unchanged_when_every_secret_is_too_short` | `textkit.redaction.redact` | TASK.md req 4 | Row 7 |

Two-directional comparison: inventory (7 selectors from `K-COLLECT`) minus
reconciliation rows = empty; reconciliation rows minus inventory = empty. Coverage
check passes — every row claims public behavior and proves it through the agreed
seam, and no row substitutes a private invariant for a public one.

`/code-review` was not part of this task, so there are no review findings and no
review-reentry rows.

## Changes outside the red → green loop

- `packages/textkit/conftest.py` — **added**. Without it `import textkit.redaction`
  fails (`ModuleNotFoundError: No module named 'textkit'`): there is no packaging
  metadata in this workspace, and pytest puts the *tests* directory on `sys.path`,
  not the package root. I settled this before cycle 1 so that the first red would
  be the missing function rather than an import artifact. Test harness only, no
  product behavior.
- `.sensitivity/check_goldens.py` — scratch harness for the golden check above.
  Kept so the claim is replayable; not part of the `textkit` package and not
  collected by `K-SUITE`.

## Deliberately not covered

Flagged rather than silently pinned by a test:

- **Overlap that is not at the same position.** Requirement 3 fixes the tie-break
  only for secrets matching at the *same* position. `redact("abcdef", ["abcd",
  "bcdef"])` currently returns `"[REDACTED]ef"` — leftmost wins over the longer
  secret that starts later. That is a consequence of the single-pass scan, not
  something the task ratifies, so no test pins it. **This is the one open question
  worth a decision**; if leftmost-wins is not intended, requirement 3 needs
  extending and a new cycle follows.
- **Empty `secrets` list.** `redact("hello world", [])` returns the text unchanged,
  but it travels the exact code path row 7 already covers, so a second test would
  add no discrimination.
- **Non-ASCII case folding.** `re.IGNORECASE` applies Python's Unicode case rules.
  The task says nothing about non-ASCII secrets, so nothing is pinned.

## Final state

`K-SUITE` → `7 passed`. `K-SENS` → exit 0.
