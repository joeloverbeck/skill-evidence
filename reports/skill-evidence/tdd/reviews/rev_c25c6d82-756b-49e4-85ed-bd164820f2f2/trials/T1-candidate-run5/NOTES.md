# TDD record — `textkit.redaction.redact`

## Seam under test and its authority

| Seam | Authority | Status |
| --- | --- | --- |
| `redact(text: str, secrets: list[str]) -> str`, imported as `from textkit.redaction import redact` | `TASK.md` § "Requirements for `redact(text: str, secrets: list[str]) -> str`", which fixes both the module path (`packages/textkit/textkit/redaction.py`) and the signature | Pre-agreed; no seam needed asking about |

Every test below enters through that one function. Nothing reaches into module
internals: `REDACTION_MARKER`, `MINIMUM_SECRET_LENGTH`, the ordering rule and the
regex are all unobserved by the suite, so the implementation can be rewritten
(scanner instead of regex, say) without touching a test.

**Public invariant coverage (ingress enumeration).** The redaction invariant has
exactly one exported ingress path. `packages/textkit/textkit/__init__.py` is
empty, so there is no re-export, no alternative constructor, parser, factory or
deserializer, and no bypass path that could admit a secret list without passing
through `redact`. No direct bypass probe is therefore possible or needed.
`redact` has no refusal path — it never raises and returns only the redacted
string — so the "assert refusal *and* non-observation of the protected payload"
rule has nothing to bind to here.

## Command key

`RUN(<target>)` = `python3 -m pytest <target> -q`, run from the workspace root
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run5`.

`TEST` = `packages/textkit/tests/test_redaction.py`.

## TDD evidence

One row per behaviour, one vertical slice each: test written first, run red, then
the minimum production change, then green.

### Row 1 — every occurrence of every secret is replaced

- **Seam authority:** `TASK.md` requirement 1.
- **Test:** `TEST::test_replaces_every_occurrence_of_every_secret_with_the_redaction_marker`
- **Observed public entry point:** `redact("swordfish here, swordfish there, hunter2 too", ["swordfish", "hunter2"])`
- **Red command:** `RUN(TEST::test_replaces_every_occurrence_of_every_secret_with_the_redaction_marker)`
- **Intended failure:** no `redact` exists yet, so the import fails.
- **Observed failure:** `ImportError: cannot import name 'redact' from 'textkit.redaction' (…/packages/textkit/textkit/redaction.py). Did you mean: 'redaction'?` → `1 error in 0.01s`.
- **Implementation:** per-secret `str.replace` loop onto `REDACTION_MARKER`.
- **Green command:** `RUN(TEST::test_replaces_every_occurrence_of_every_secret_with_the_redaction_marker)`
- **Green result:** `1 passed in 0.00s`.

### Row 2 — matching is case-insensitive, surrounding text keeps its case

- **Seam authority:** `TASK.md` requirement 2.
- **Test:** `TEST::test_matches_secrets_case_insensitively_and_leaves_other_text_cased`
- **Observed public entry point:** `redact("Login used Hunter2 and backup HUNTER2", ["hunter2"])`
- **Red command:** `RUN(TEST::test_matches_secrets_case_insensitively_and_leaves_other_text_cased)`
- **Intended failure:** the literal `str.replace` from row 1 leaves `Hunter2` and `HUNTER2` intact.
- **Observed failure:** `AssertionError`, diff `- Login used [REDACTED] and backup [REDACTED]` / `+ Login used Hunter2 and backup HUNTER2` → `1 failed in 0.01s`.
- **Implementation:** per-secret `re.sub(secret, MARKER, text, flags=re.IGNORECASE)`.
- **Green command:** `RUN(TEST::test_matches_secrets_case_insensitively_and_leaves_other_text_cased)`
- **Green result:** `1 passed in 0.00s`.

### Row 3 — the longer secret wins at the same position

- **Seam authority:** `TASK.md` requirement 3.
- **Test:** `TEST::test_longest_secret_wins_when_two_secrets_match_at_the_same_position`
- **Observed public entry point:** `redact("leaked hunter2000 today", ["hunter", "hunter2000"])`
- **Red command:** `RUN(TEST::test_longest_secret_wins_when_two_secrets_match_at_the_same_position)`
- **Intended failure:** the row-2 loop applies secrets in list order, so the shorter `hunter` is consumed first and `2000` survives.
- **Observed failure:** `AssertionError`, diff `- leaked [REDACTED] today` / `+ leaked [REDACTED]2000 today` → `1 failed in 0.01s`.
- **Implementation:** replaced the loop with a single left-to-right pass over one alternation, secrets sorted longest-first (`key=(-len, secret)`), so the longest alternative matching at a position is the one taken.
- **Green command:** `RUN(TEST::test_longest_secret_wins_when_two_secrets_match_at_the_same_position)`
- **Green result:** `1 passed in 0.00s`; full file `RUN(TEST)` → `3 passed in 0.00s`.

### Row 4 — secrets under four characters are ignored, four-character ones are not

- **Seam authority:** `TASK.md` requirement 4.
- **Test:** `TEST::test_ignores_secrets_shorter_than_four_characters_but_redacts_four`
- **Observed public entry point:** `redact("the cat sat on the sofa", ["cat", "sofa"])`
- **Red command:** `RUN(TEST::test_ignores_secrets_shorter_than_four_characters_but_redacts_four)`
- **Intended failure:** with no length rule, the three-character `cat` is redacted too.
- **Observed failure:** `AssertionError`, diff `- the cat sat on the [REDACTED]` / `+ the [REDACTED] sat on the [REDACTED]` → `1 failed in 0.01s`.
- **Implementation:** filter `len(secret) >= MINIMUM_SECRET_LENGTH` (4) before building the pattern.
- **Green command:** `RUN(TEST::test_ignores_secrets_shorter_than_four_characters_but_redacts_four)`
- **Green result:** `1 passed in 0.00s`.

### Row 5 — when every secret is too short the text comes back unchanged

- **Seam authority:** `TASK.md` requirement 4 ("ignored **entirely**") — the case where the filter empties the list.
- **Test:** `TEST::test_returns_the_text_unchanged_when_every_secret_is_too_short`
- **Observed public entry point:** `redact("the cat sat on the mat", ["cat", "mat"])`
- **Red command:** `RUN(TEST::test_returns_the_text_unchanged_when_every_secret_is_too_short)`
- **Intended failure:** row 4's filter empties the list, the joined alternation becomes the empty pattern, and the empty pattern matches at every position.
- **Observed failure:** `AssertionError`, diff `- the cat sat on the mat` / `+ [REDACTED]t[REDACTED]h[REDACTED]e[REDACTED] [REDACTED]c[REDACTED]a[REDACTED]t…` → `1 failed in 0.01s`.
- **Implementation:** `if not usable: return text` before building the pattern.
- **Green command:** `RUN(TEST::test_returns_the_text_unchanged_when_every_secret_is_too_short)`
- **Green result:** `1 passed in 0.00s`.

### Row 6 — secrets match literally, not as patterns

- **Seam authority:** `TASK.md` requirement 1 — "every occurrence of *each secret*", i.e. of the literal string. A secret is caller data (tokens carry `.`, `+`, `/`), not a pattern language.
- **Test:** `TEST::test_matches_secrets_literally_rather_than_as_patterns`
- **Observed public entry point:** `redact("n.ll is redacted but null is not", ["n.ll"])`
- **Red command:** `RUN(TEST::test_matches_secrets_literally_rather_than_as_patterns)`
- **Intended failure:** the rows 2–3 implementation feeds secrets to `re` unescaped, so `.` in `n.ll` also matches `null`.
- **Observed failure:** `AssertionError`, diff `- [REDACTED] is redacted but null is not` / `+ [REDACTED] is redacted but [REDACTED] is not` → `1 failed in 0.01s`.
- **Implementation:** `re.escape` each secret when joining the alternation.
- **Green command:** `RUN(TEST::test_matches_secrets_literally_rather_than_as_patterns)`
- **Green result:** `1 passed in 0.00s`; full file `RUN(TEST)` → `6 passed in 0.00s`.

## Discriminating goldens — sensitivity check

Every expected value above is an independent literal written from the
requirement, never recomputed the way the code computes it. To show each golden
actually discriminates rather than merely passing, `sensitivity_check.py`
substitutes each plausible rejected implementation into `redaction.py` and runs
the retained suite against it (`python3 sensitivity_check.py` from the workspace
root). Observed result — every rejected implementation is killed:

| Rejected implementation | Tests that die |
| --- | --- |
| case-sensitive matching | row 2 |
| lowercase the whole text | row 2 |
| declared list order wins, not longest | row 3 |
| no minimum-length filter | rows 4, 5 |
| minimum length is `> 4` rather than `>= 4` | rows 4, 6 |
| secrets compiled as patterns, not escaped | row 6 |
| only the first occurrence replaced | rows 1, 2 |
| only the first secret applied | row 1 |

No mutant survived, so no golden needed replacing. (Row 6's golden `n.ll` is
itself exactly four characters, which is why the `> 4` mutant also kills it; the
intended killer for that mutant is row 4.)

## Pre-review reconciliation (changed tests → seams)

Both `packages/textkit/tests/test_redaction.py` and
`packages/textkit/textkit/redaction.py` were empty at the start of this task, so
the scoped change is the whole of each file and every collected test is a changed
test. Inventory derived from pytest's own collection — the repository's native
test structure — with `python3 -m pytest packages/textkit/tests/test_redaction.py --collect-only -q`
(6 tests collected, no classes, no parameterisation):

| # | Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `packages/textkit/tests/test_redaction.py` | `test_replaces_every_occurrence_of_every_secret_with_the_redaction_marker` | `redact(text, secrets)` | `TASK.md` req 1 | Row 1 |
| 2 | `packages/textkit/tests/test_redaction.py` | `test_matches_secrets_case_insensitively_and_leaves_other_text_cased` | `redact(text, secrets)` | `TASK.md` req 2 | Row 2 |
| 3 | `packages/textkit/tests/test_redaction.py` | `test_longest_secret_wins_when_two_secrets_match_at_the_same_position` | `redact(text, secrets)` | `TASK.md` req 3 | Row 3 |
| 4 | `packages/textkit/tests/test_redaction.py` | `test_ignores_secrets_shorter_than_four_characters_but_redacts_four` | `redact(text, secrets)` | `TASK.md` req 4 | Row 4 |
| 5 | `packages/textkit/tests/test_redaction.py` | `test_returns_the_text_unchanged_when_every_secret_is_too_short` | `redact(text, secrets)` | `TASK.md` req 4 | Row 5 |
| 6 | `packages/textkit/tests/test_redaction.py` | `test_matches_secrets_literally_rather_than_as_patterns` | `redact(text, secrets)` | `TASK.md` req 1 | Row 6 |

**Both-direction comparison.** Inventory selectors \ reconciliation selectors =
∅. Reconciliation selectors \ inventory selectors = ∅. Every row's entry point is
the ratified public function, so no row needed a seam ratified or a test rewritten
at a different boundary, and no row claims public behaviour it proves only through
a private invariant.

## Non-test, non-behaviour changes in scope

- `pytest.ini` (new, workspace root): `pythonpath = packages/textkit` so the
  documented command resolves `textkit` from the workspace root. Import plumbing,
  no observable behaviour, so no cycle.
- `sensitivity_check.py` (new, workspace root): the throwaway mutation harness
  behind the table above. Not collected by pytest (not `test_*.py`), imports
  nothing from the suite, and writes only a `.mutants/` scratch tree that it
  deletes on the way out.

## What is deliberately not pinned

- Ties between equal-length secrets (both would yield the same marker anyway).
- Text that already contains `[REDACTED]`, and secrets that overlap a span the
  left-to-right pass has already consumed.
- Empty `secrets`, `None` inputs, non-string members: outside the stated
  requirements, so no speculative test or defence was added.

## Review re-entry

`/code-review` has not been run on this change, so there are no review findings,
no `TDD re-entry required: yes` routing, and no appended review-reentry rows. The
ledger comparison that gate calls for is vacuous in both directions here; if the
change goes to review, each actionable finding gets its own appended row rather
than an edit to any row above.
