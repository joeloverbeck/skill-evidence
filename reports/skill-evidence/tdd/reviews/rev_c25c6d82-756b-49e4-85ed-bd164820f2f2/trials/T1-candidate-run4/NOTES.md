# TDD evidence — `textkit.redaction.redact`

## Seam under test and its authority

| Seam | Authority | Status |
| --- | --- | --- |
| `textkit.redaction.redact(text: str, secrets: list[str]) -> str`, imported as `from textkit.redaction import redact` | `TASK.md` — names the exact module path, the signature, the test file, and the run command | Ratified; not ambiguous, so no seam question was put to the user |

One seam only. No test reaches a module-private helper, a mutable global, or any side channel:
the whole suite's only import is `redact`, and the `[REDACTED]` placeholder is asserted as a
literal inside each expected string rather than by importing the `PLACEHOLDER` constant. No
mocks — there is no system boundary here (no I/O, clock, or randomness), so mocking anything
would have meant mocking code we own.

## Command keys

Both keys expand to a complete command that replays on its own from any working directory.

- `FOCUS(<selector>)` = `python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run4/packages/textkit/tests/test_redaction.py::<selector> -q`
- `SUITE` = `python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run4/packages/textkit/tests/test_redaction.py -q`

All test selectors live in
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run4/packages/textkit/tests/test_redaction.py`.

## Cycle evidence

One vertical slice per row: one test, then only enough production code to pass it. Each row was
run red, then green, before the next row's test was written.

### Row 1 — every occurrence of a secret is replaced (TASK.md requirement 1)

- **Seam authority:** TASK.md requirement 1.
- **Test file / selector:** `packages/textkit/tests/test_redaction.py::test_replaces_every_occurrence_of_a_secret`
- **Observed public entry point:** `redact("hunter2 then hunter2 again", ["hunter2"])`
- **Red command:** `FOCUS(test_replaces_every_occurrence_of_a_secret)`
- **Intended failure:** `redact` does not exist yet, so the import fails.
- **Observed failure:** `ImportError: cannot import name 'redact' from 'textkit.redaction'` — 1 error.
- **Green command:** `FOCUS(test_replaces_every_occurrence_of_a_secret)`
- **Result:** 1 passed. Implementation: loop over `secrets`, `str.replace` each with `[REDACTED]`.

### Row 2 — matching is case-insensitive (TASK.md requirement 2)

- **Seam authority:** TASK.md requirement 2, which fixes `Hunter2` and `HUNTER2` as required matches.
- **Test file / selector:** `packages/textkit/tests/test_redaction.py::test_matches_secrets_case_insensitively`
- **Observed public entry point:** `redact("Hunter2 and HUNTER2", ["hunter2"])`
- **Red command:** `FOCUS(test_matches_secrets_case_insensitively)`
- **Intended failure:** exact-case `str.replace` matches neither casing, so the text comes back untouched.
- **Observed failure:** `AssertionError`, actual `'Hunter2 and HUNTER2'` vs expected `'[REDACTED] and [REDACTED]'` — 1 failed.
- **Green command:** `FOCUS(test_matches_secrets_case_insensitively)`
- **Result:** 1 passed. Implementation: per-secret scan comparing lowercased slices of the original text (slice-wise, so a character whose lowercase is wider than itself cannot misalign the index).

### Row 3 — the longer secret wins at a shared position (TASK.md requirement 3)

- **Seam authority:** TASK.md requirement 3.
- **Test file / selector:** `packages/textkit/tests/test_redaction.py::test_longer_secret_wins_when_two_match_at_the_same_position`
- **Observed public entry point:** `redact("pw: hunter2000", ["hunter2", "hunter2000"])`
- **Red command:** `FOCUS(test_longer_secret_wins_when_two_match_at_the_same_position)`
- **Intended failure:** secret-at-a-time replacement consumes `hunter2` first and strands `000`.
- **Observed failure:** `AssertionError`, actual `'pw: [REDACTED]000'` vs expected `'pw: [REDACTED]'` — 1 failed.
- **Green command:** `FOCUS(test_longer_secret_wins_when_two_match_at_the_same_position)`
- **Result:** 1 passed. Implementation replaced the per-secret passes with a single left-to-right scan that takes the longest secret matching at each position.

The secrets are listed shortest-first on purpose: with the list reversed, the row-2
implementation would have passed by luck and the red would have proved nothing.

### Row 4 — secrets shorter than 4 characters are ignored (TASK.md requirement 4)

- **Seam authority:** TASK.md requirement 4.
- **Test file / selector:** `packages/textkit/tests/test_redaction.py::test_ignores_secrets_shorter_than_four_characters`
- **Observed public entry point:** `redact("cat code1234", ["cat", "code"])`
- **Red command:** `FOCUS(test_ignores_secrets_shorter_than_four_characters)`
- **Intended failure:** with no length rule, the 3-character `cat` is redacted too.
- **Observed failure:** `AssertionError`, actual `'[REDACTED] [REDACTED]1234'` vs expected `'cat [REDACTED]1234'` — 1 failed.
- **Green command:** `FOCUS(test_ignores_secrets_shorter_than_four_characters)`
- **Result:** 1 passed. Implementation: drop secrets shorter than `MIN_SECRET_LENGTH = 4` before scanning.

One expected string carries both sides of the threshold — the 3-character secret must survive and
the 4-character one must not — so the row also fails an off-by-one rule that ignored 4-character
secrets.

### Whole suite

`SUITE` → **4 passed**.

## Discriminating-golden check

Every expected value above is a literal fixed by TASK.md, never recomputed the way the code
computes it. To confirm each golden actually discriminates, the final implementation source was
mutated in memory into each plausible rejected implementation and the goldens re-run against it
(no files were modified):

| Rejected implementation | Rejected by |
| --- | --- |
| Redacts only the first match | Row 1, Row 2 |
| Case-sensitive matching | Row 2 |
| Shortest match wins at a position | Row 3 |
| Threshold off by one (`> 4`, ignoring 4-character secrets) | Row 4 |
| No length threshold at all | Row 4 |

No variant survived the suite, so no golden passes by construction. Three of the five were also
observed directly as the red of their own cycle; the "first match only" and "off-by-one threshold"
variants never occurred during the loop, so this probe is the only evidence for those two.

## Pre-review reconciliation

Inventory derived from the repository's native test structure via
`python3 -m pytest <test file> --collect-only -q`. Git is off-limits in this workspace, so the
scope was established instead by the fact that both target files measured 0 bytes at session
start (`wc -c`): every collected test in the file is therefore a changed test.

| Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `packages/textkit/tests/test_redaction.py` | `test_replaces_every_occurrence_of_a_secret` | `redact(text, secrets)` | TASK.md req. 1 | Row 1 |
| `packages/textkit/tests/test_redaction.py` | `test_matches_secrets_case_insensitively` | `redact(text, secrets)` | TASK.md req. 2 | Row 2 |
| `packages/textkit/tests/test_redaction.py` | `test_longer_secret_wins_when_two_match_at_the_same_position` | `redact(text, secrets)` | TASK.md req. 3 | Row 3 |
| `packages/textkit/tests/test_redaction.py` | `test_ignores_secrets_shorter_than_four_characters` | `redact(text, secrets)` | TASK.md req. 4 | Row 4 |

Both-direction comparison: collected selectors not in the reconciliation table — **none**;
reconciliation rows with no collected selector — **none**. Both differences empty, so the gate
passes. Every row proves public behavior through the ratified seam; none stands in for a
private-invariant test.

## Review re-entry

No `/code-review` pass has been run against this change, so there are no review-reentry rows and
nothing to reconcile against a `Findings fixed during review` ledger.

## Decisions worth flagging

- **`packages/textkit/tests/conftest.py` is new.** The layout shipped without any packaging
  metadata, so `from textkit.redaction import redact` could not resolve under the task's run
  command. The conftest puts `packages/textkit` on `sys.path` — test infrastructure only, no
  production behavior, and no seam of its own.
- **Requirements 3 and 4 interact,** and the order of the slices matters: the length filter runs
  before the scan, so a short secret cannot win a position or block a longer one that starts
  there. Filtering also removes the empty-string secret that would otherwise match at every
  position with zero width.
- **Not tested, because TASK.md does not require it:** empty `secrets`, secrets overlapping at
  *different* positions, and non-ASCII casing. Literal (non-regex) matching is structural in the
  final scan rather than pinned by a test — a future rewrite onto `re` would need a test that
  regex metacharacters in a secret stay literal.
