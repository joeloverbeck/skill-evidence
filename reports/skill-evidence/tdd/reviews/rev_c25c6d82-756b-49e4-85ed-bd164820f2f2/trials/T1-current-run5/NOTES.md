# TDD evidence — `textkit.redaction.redact`

All commands below are run from the workspace root
(`.../scratchpad/runs/T1-A-run5`), the directory that contains `packages/`.

## Seams under test, agreed before the first test

| Seam | Authority | Status |
| --- | --- | --- |
| `redact(text: str, secrets: list[str]) -> str`, imported as `from textkit.redaction import redact` | `TASK.md`, "Requirements for `redact(text: str, secrets: list[str]) -> str`" — the task statement names both the signature and the module | Ratified; no ambiguity, so no seam question was raised |

No other seam is under test. Nothing reaches into module internals: `PLACEHOLDER`,
`MIN_SECRET_LENGTH`, and the compiled pattern are never referenced by a test, so the
implementation can be rewritten (per-secret loop, trie, single regex) without touching the
suite. No mocking: `redact` is a pure function with no system boundary to stub.

`packages/textkit/conftest.py` was added as plumbing so the in-tree package imports when
the suite runs from the workspace root. It is not a test and carries no assertions.

## Evidence rows

One row per behavior, in the order the cycles ran. Observed public entry point is
`textkit.redaction.redact` for every row — no test observes anything else.

### Row 1 — every occurrence of every secret is replaced

- **Seam authority:** `TASK.md` requirement 1.
- **Test:** `packages/textkit/tests/test_redaction.py::test_redacts_every_occurrence_of_each_secret`
- **Observed public entry point:** `textkit.redaction.redact`
- **Red command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_redacts_every_occurrence_of_each_secret -q`
- **Intended failure:** no `redact` exists yet, so the module cannot supply the name.
- **Observed failure:** `ImportError: cannot import name 'redact' from 'textkit.redaction'` (1 error). Matches intent — the module itself resolved, so this is a missing-behavior red, not a broken import path.
- **Minimal implementation:** per-secret `str.replace` loop.
- **Green command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_redacts_every_occurrence_of_each_secret -q`
- **Green result:** 1 passed.

### Row 2 — matching is case-insensitive

- **Seam authority:** `TASK.md` requirement 2.
- **Test:** `packages/textkit/tests/test_redaction.py::test_matches_secrets_case_insensitively`
- **Observed public entry point:** `textkit.redaction.redact`
- **Red command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_matches_secrets_case_insensitively -q`
- **Intended failure:** the case-sensitive loop redacts only the exactly-cased occurrence.
- **Observed failure:** `AssertionError`, got `'Hunter2 then HUNTER2 then [REDACTED]'`, wanted `'[REDACTED] then [REDACTED] then [REDACTED]'` (1 failed). Matches intent.
- **Minimal implementation:** `re.sub(re.escape(secret), PLACEHOLDER, text, flags=re.IGNORECASE)` per secret.
- **Green command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_matches_secrets_case_insensitively -q`
- **Green result:** 1 passed; full file `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → 2 passed.

### Row 3 — the longer secret wins at a shared position

- **Seam authority:** `TASK.md` requirement 3.
- **Test:** `packages/textkit/tests/test_redaction.py::test_longer_secret_wins_when_both_match_at_the_same_position`
- **Observed public entry point:** `textkit.redaction.redact`
- **Red command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_longer_secret_wins_when_both_match_at_the_same_position -q`
- **Intended failure:** the per-secret loop consumes the shorter `hunter2` first and strands its tail.
- **Observed failure:** `AssertionError`, got `'the key is [REDACTED]000 today'`, wanted `'the key is [REDACTED] today'` (1 failed). Matches intent.
- **Minimal implementation:** one pass over a single alternation, branches sorted longest-first (Python alternation is leftmost-**first**, so branch order is what decides a shared position).
- **Green command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_longer_secret_wins_when_both_match_at_the_same_position -q`
- **Green result:** 1 passed; full file `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → 3 passed.

### Row 4 — a secret shorter than 4 characters is ignored

- **Seam authority:** `TASK.md` requirement 4.
- **Test:** `packages/textkit/tests/test_redaction.py::test_ignores_secrets_shorter_than_four_characters`
- **Observed public entry point:** `textkit.redaction.redact`
- **Red command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_ignores_secrets_shorter_than_four_characters -q`
- **Intended failure:** with no length rule, the 3-character `cat` redacts too.
- **Observed failure:** `AssertionError`, got `'[REDACTED] and [REDACTED]'`, wanted `'cat and [REDACTED]'` (1 failed). Matches intent.
- **Minimal implementation:** filter to `len(secret) >= MIN_SECRET_LENGTH` before building the pattern.
- **Green command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_ignores_secrets_shorter_than_four_characters -q`
- **Green result:** 1 passed; full file `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → 4 passed.

### Row 5 — "ignored entirely" still holds when nothing survives the length rule

- **Seam authority:** `TASK.md` requirement 4, "ignored **entirely**" — the all-too-short list is that requirement's degenerate case, not a new requirement.
- **Test:** `packages/textkit/tests/test_redaction.py::test_leaves_text_untouched_when_every_secret_is_too_short`
- **Observed public entry point:** `textkit.redaction.redact`
- **Red command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_leaves_text_untouched_when_every_secret_is_too_short -q`
- **Intended failure:** filtering empties the alternation, and an empty alternation matches the empty string at every position.
- **Observed failure:** `AssertionError`, got `'[REDACTED]c[REDACTED]a[REDACTED]t[REDACTED] ...[REDACTED]g[REDACTED]'`, wanted `'cat and dog'` (1 failed). Matches intent, and the damage is worse than the requirement's own warning about shredding ordinary text.
- **Minimal implementation:** return `text` unchanged when no secret survives the filter.
- **Green command:** `python3 -m pytest packages/textkit/tests/test_redaction.py::test_leaves_text_untouched_when_every_secret_is_too_short -q`
- **Green result:** 1 passed; full file `python3 -m pytest packages/textkit/tests/test_redaction.py -q` → 5 passed.

## Discriminating goldens

Every expected value is a hand-written literal read off the `TASK.md` requirement, never
recomputed the way the code computes it. Each golden was checked against the plausible
wrong implementation it has to rule out.

| Golden | Rejected implementation it rules out | Evidence it discriminates |
| --- | --- | --- |
| Row 1, `"login abcd1234 then abcd1234 again, key wxyz9876 done"` | replaces only the first occurrence | mutation probe: that variant returns `'login [REDACTED] then abcd1234 again, key wxyz9876 done'` — golden fails |
| Row 2, mixed-case `Hunter2 / HUNTER2 / hunter2` | case-sensitive matching | observed red of row 2 is exactly that variant's output |
| Row 3, secrets listed **shortest first** | "first listed wins" and "shortest wins" | observed red of row 3 is exactly that output; list order is inverted from length order, so the test cannot pass by list order |
| Row 4, `"cat"` (3) ignored **and** `"cart"` (4) redacted | (a) no length rule, (b) off-by-one threshold keeping only 5+ | (a) is the observed red; (b) mutation probe returns `'cat and cart'` — golden fails. One golden pins the threshold from both sides |
| Row 5, all-too-short list | dropping the empty-alternation guard | observed red of row 5 is exactly that variant's output |

Mutation probes for rows 1 and 4 were run as throwaway in-process variants (rows 1 and 4
were the only ones whose own red was not already the rejected behavior). Nothing was left
on disk.

## Pre-review reconciliation

Changed-test inventory, derived from the repository's native test structure with
`python3 -m pytest packages/textkit/tests/test_redaction.py -q --collect-only`. The test
file started empty, so every collected selector is a changed test.

| Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `packages/textkit/tests/test_redaction.py` | `test_redacts_every_occurrence_of_each_secret` | `textkit.redaction.redact` | `TASK.md` req. 1 | Row 1 |
| `packages/textkit/tests/test_redaction.py` | `test_matches_secrets_case_insensitively` | `textkit.redaction.redact` | `TASK.md` req. 2 | Row 2 |
| `packages/textkit/tests/test_redaction.py` | `test_longer_secret_wins_when_both_match_at_the_same_position` | `textkit.redaction.redact` | `TASK.md` req. 3 | Row 3 |
| `packages/textkit/tests/test_redaction.py` | `test_ignores_secrets_shorter_than_four_characters` | `textkit.redaction.redact` | `TASK.md` req. 4 | Row 4 |
| `packages/textkit/tests/test_redaction.py` | `test_leaves_text_untouched_when_every_secret_is_too_short` | `textkit.redaction.redact` | `TASK.md` req. 4 (degenerate case) | Row 5 |

Two-direction difference check:

- inventory selectors not in the reconciliation table: **none**
- reconciliation rows not in the inventory: **none**

Every row enters through the ratified public boundary, so no seam needed re-ratifying and
no test needed rewriting. Coverage check passes; the change is ready for `/code-review`.

## Review re-entry rows

None. `/code-review` has not been run against this change, so the
`Findings fixed during review` ledger is empty and no review-keyed evidence rows exist. If
review lands a finding marked `TDD re-entry required: yes`, it gets a new appended row —
no earlier row above may be rewritten or extended to absorb it.

## Left unratified, deliberately

`TASK.md` does not say whether the inserted `[REDACTED]` is itself re-scanned. The
single-pass implementation happens not to re-scan it, so a secret such as `"redacted"`
cannot chain off an earlier substitution — but that is an untested consequence of the
current implementation, not a pinned behavior. Untested means unprotected: a future
rewrite to a per-secret loop would silently change it. Pinning it needs a requirement
first; no test was written at an authority that does not yet exist.
