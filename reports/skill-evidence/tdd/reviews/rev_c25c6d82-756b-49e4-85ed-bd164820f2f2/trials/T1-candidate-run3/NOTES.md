# TDD evidence — `textkit.redaction.redact`

## Seams under test

| Seam | Authority |
| --- | --- |
| `redact(text: str, secrets: list[str]) -> str`, imported as `from textkit.redaction import redact` | `TASK.md` — names the function, its signature, and the file it lives in (`packages/textkit/textkit/redaction.py`), and fixes the four required behaviours |

**Ingress enumeration (public invariant coverage).** The invariant "no secret survives in the
returned text" has exactly one exported ingress path: the module-level function
`textkit.redaction.redact`. There is no parser, constructor, factory, or deserializer that can
admit a value by another route. `packages/textkit/textkit/__init__.py` is empty and `TASK.md` does
not ratify a package-level re-export, so none was added — adding one would create a second,
unratified seam. The two module constants are named `_PLACEHOLDER` and `_MIN_SECRET_LENGTH` so the
public surface stays exactly what the authority ratified. Every test below enters through the
ratified function; none reaches into module internals, so no test verifies through a side channel.

No mocks are used: `redact` is a pure string function with no system boundary (no I/O, clock, or
randomness) to stub.

## Commands

All commands run with the workspace root as the working directory. Two stable keys, used with their
argument in the rows below:

- `FOCUS <selector>` = `cd /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run3 && python3 -m pytest packages/textkit/tests/test_redaction.py::<selector> -q`
- `SUITE` = `cd /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run3 && python3 -m pytest packages/textkit/tests/test_redaction.py -q`

Scaffolding, not a behaviour change: `pytest.ini` at the workspace root sets
`pythonpath = packages/textkit` so the test module can import the package under test. It was added
before cycle 1 so that cycle 1's red would be the intended missing-name failure rather than a
harness-level `ModuleNotFoundError`.

## TDD evidence rows

All test selectors below live in `packages/textkit/tests/test_redaction.py`, and every row's
observed public entry point is `textkit.redaction.redact`.

### Row 1 — every occurrence of every secret is replaced

- **Seam authority:** `TASK.md` requirement 1.
- **Test selector:** `test_replaces_every_occurrence_of_every_secret`.
- **Red command:** `FOCUS test_replaces_every_occurrence_of_every_secret`.
- **Intended failure:** `redact` cannot be imported — `redaction.py` is empty, so the ratified entry
  point does not exist yet.
- **Observed failure:** `ImportError: cannot import name 'redact' from 'textkit.redaction'` (1 error).
- **Implementation:** `redact` loops over the secrets and calls `str.replace` for each.
- **Green command:** `FOCUS test_replaces_every_occurrence_of_every_secret`.
- **Result:** `1 passed`.

### Row 2 — matching is case-insensitive

- **Seam authority:** `TASK.md` requirement 2.
- **Test selector:** `test_matches_secrets_case_insensitively`.
- **Red command:** `FOCUS test_matches_secrets_case_insensitively`.
- **Intended failure:** exact matching leaves the differently-cased occurrences alone, so only the
  literal `hunter2` is redacted.
- **Observed failure:** `AssertionError` — got `'Hunter2 HUNTER2 [REDACTED] swordfish'`, wanted
  `'[REDACTED] [REDACTED] [REDACTED] [REDACTED]'` (1 failed).
- **Implementation:** per-secret `re.sub(re.escape(secret), ..., flags=re.IGNORECASE)`.
- **Green command:** `FOCUS test_matches_secrets_case_insensitively`.
- **Result:** `1 passed`.

The case in this row varies in both directions — a lower-case secret against mixed- and upper-case
text, and an upper-case secret against lower-case text — because an implementation that lowers only
one side passes the first direction and fails the second. Both directions are one behaviour
("matching ignores case") under one equality assertion, so they stay in one cycle.

### Row 3 — the longer secret wins a tie at the same position

- **Seam authority:** `TASK.md` requirement 3.
- **Test selector:** `test_longest_secret_wins_when_two_match_at_the_same_position`.
- **Red command:** `FOCUS test_longest_secret_wins_when_two_match_at_the_same_position`.
- **Intended failure:** substituting secrets one at a time in list order redacts `password` first
  and strands the `123` tail.
- **Observed failure:** `AssertionError` — got `'my [REDACTED]123 here'`, wanted
  `'my [REDACTED] here'` (1 failed).
- **Implementation:** a single `re.sub` pass over one alternation built from the secrets sorted
  longest-first, so the longest alternative wins at any given position.
- **Green command:** `FOCUS test_longest_secret_wins_when_two_match_at_the_same_position`.
- **Result:** `1 passed`; `SUITE` → `3 passed` (rows 1–2 unaffected by the rewrite to one pass).

### Row 4 — secrets shorter than four characters are ignored

- **Seam authority:** `TASK.md` requirement 4.
- **Test selector:** `test_ignores_secrets_shorter_than_four_characters`.
- **Red command:** `FOCUS test_ignores_secrets_shorter_than_four_characters`.
- **Intended failure:** with no length rule, the three-character secret `cat` is redacted out of
  ordinary text.
- **Observed failure:** `AssertionError` — got `'the [REDACTED] sat on [REDACTED] with [REDACTED]'`,
  wanted `'the cat sat on [REDACTED] with [REDACTED]'` (1 failed).
- **Implementation:** filter the secrets to those of length `>= 4` before building the alternation.
- **Green command:** `FOCUS test_ignores_secrets_shorter_than_four_characters`.
- **Result:** `1 passed`.

### Row 5 — "ignored entirely" holds when nothing survives the filter

- **Seam authority:** `TASK.md` requirement 4 ("ignored **entirely**"), applied to the input where
  every supplied secret is below the threshold.
- **Test selector:** `test_returns_text_unchanged_when_every_secret_is_too_short`.
- **Red command:** `FOCUS test_returns_text_unchanged_when_every_secret_is_too_short`.
- **Intended failure:** after filtering, the alternation is the empty pattern, which matches at every
  position instead of nowhere.
- **Observed failure:** `AssertionError` — got
  `'[REDACTED]t[REDACTED]h[REDACTED]e[REDACTED] [REDACTED]c[REDACTED]a[REDACTED]t[REDACTED] [REDACTED]s[REDACTED]a[REDACTED]t[REDACTED]'`,
  wanted `'the cat sat'` (1 failed). This is a real defect the first four rows could not have caught:
  each of them leaves at least one usable secret behind.
- **Implementation:** return `text` unchanged when no secret survives the length filter.
- **Green command:** `FOCUS test_returns_text_unchanged_when_every_secret_is_too_short`.
- **Result:** `1 passed`; `SUITE` → `5 passed`.

`redact(text, [])` reaches green through this same guard. It has no row of its own: `TASK.md` does
not ratify it as a distinct criterion, and after row 5's green it cannot produce red, so a row for it
would be a green-on-arrival test dressed up as a cycle.

## Discriminating-golden check

Every expected value is a hand-written literal read off the `TASK.md` requirement, never recomputed
the way `redact` computes it.

Rows 2–5 each observed their rejected implementation directly: the red run *is* the rejected
behaviour executing against the golden, and the diff in each `AssertionError` is the required
value's difference from it. Two rejected alternatives were never executed as a red, so they were
probed separately (inline `python3` heredoc, no file retained):

| Golden | Rejected alternative | Its output | Differs from required |
| --- | --- | --- | --- |
| Row 1 | replace only the first occurrence of each secret | `'user [REDACTED] logged in with hunter2 and key [REDACTED]'` | yes |
| Row 4 | threshold off by one (`len > 4`, so four-character secrets are ignored) | `'the cat sat on mat1 with [REDACTED]'` | yes |

Row 4's input therefore discriminates in both directions at once: `cat` (3 chars) must survive and
`mat1` (exactly 4) must be redacted, so the golden fails against a missing threshold *and* against
an off-by-one threshold.

## Pre-review reconciliation

Git is unavailable in this workspace, so the scoped change is stated directly: one added test file,
`packages/textkit/tests/test_redaction.py`, previously empty — every test in it is a changed test.
The inventory is derived from the repository's native test structure with
`cd /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run3 && python3 -m pytest packages/textkit/tests/test_redaction.py --collect-only -q`
(`5 tests collected`).

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `packages/textkit/tests/test_redaction.py` | `test_replaces_every_occurrence_of_every_secret` | `textkit.redaction.redact` | `TASK.md` req. 1 | Row 1 |
| `packages/textkit/tests/test_redaction.py` | `test_matches_secrets_case_insensitively` | `textkit.redaction.redact` | `TASK.md` req. 2 | Row 2 |
| `packages/textkit/tests/test_redaction.py` | `test_longest_secret_wins_when_two_match_at_the_same_position` | `textkit.redaction.redact` | `TASK.md` req. 3 | Row 3 |
| `packages/textkit/tests/test_redaction.py` | `test_ignores_secrets_shorter_than_four_characters` | `textkit.redaction.redact` | `TASK.md` req. 4 | Row 4 |
| `packages/textkit/tests/test_redaction.py` | `test_returns_text_unchanged_when_every_secret_is_too_short` | `textkit.redaction.redact` | `TASK.md` req. 4 | Row 5 |

**Two-way difference.** Inventory minus reconciliation: empty. Reconciliation minus inventory:
empty. The five collected selectors are exactly the five reconciliation rows, which are exactly the
five TDD evidence rows.

**Coverage check.** Every row claims public behaviour observed through the ratified entry point; no
row substitutes a private-invariant test for a public-behaviour proof. Gate passes.

## Review re-entry

No `/code-review` pass was run for this task, so there are no `TDD re-entry required: yes` findings
and no review-reentry rows. The ledger comparison that rule requires is vacuous here — both sides
are empty.

## Final state

`SUITE` → `5 passed`.
