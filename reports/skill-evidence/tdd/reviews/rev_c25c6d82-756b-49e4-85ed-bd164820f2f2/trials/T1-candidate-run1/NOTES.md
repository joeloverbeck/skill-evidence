# TDD record — `textkit.redaction.redact`

## Seams under test (written down before the first test)

| Seam | Authority | Status |
| --- | --- | --- |
| `textkit.redaction.redact(text: str, secrets: list[str]) -> str` | `TASK.md` — "Requirements for `redact(text: str, secrets: list[str]) -> str`" fixes the name, signature and behaviour; "`packages/textkit/textkit/redaction.py` — empty; the implementation goes here" fixes the module | Ratified. Not ambiguous, so the user was not asked. |

No test is written at any other boundary. Nothing in these tests imports a private
helper, monkeypatches a module global, or inspects `redaction` internals; every
assertion goes through the single call above.

### Public-invariant ingress enumeration

The public invariant is *no secret of length ≥ 4 survives in the returned text*. Per
`method/tests.md` ("Public invariant coverage") every exported entry point that can create
or admit the value is enumerated, rather than assuming the preferred path is the only path:

| Candidate ingress | Present? | Evidence |
| --- | --- | --- |
| `textkit.redaction.redact` | yes | the seam above |
| `textkit.redact` (package re-export) | no | `wc -c packages/textkit/textkit/__init__.py` → `0`; `python3 -c "...; import textkit; print([n for n in dir(textkit) if not n.startswith('_')])"` → `[]` |
| parser / constructor / factory / deserializer | none exist | `packages/textkit/textkit/` contains only `__init__.py` and `redaction.py` |

There is exactly one ingress path, so there is no second path that could bypass the
length filter or the longest-match rule, and no bypass probe to write. If
`__init__.py` ever re-exports `redact`, that re-export becomes a second ingress and this
enumeration must be redone.

### Mocking

None. `redact` is pure string-in/string-out with no system boundary (`method/mocking.md`:
mock external APIs, DBs, time, randomness, filesystem — none are involved).

### Test harness

`pytest.ini` at the workspace root sets `pythonpath = packages/textkit` so the task's
documented command works from the workspace root. Verified beforehand that the bare
interpreter cannot import the package (`ModuleNotFoundError: No module named 'textkit'`),
so that a red result below is never a disguised import-path failure.

---

## TDD evidence — command keys

These keys belong to the evidence section below and are defined here so every command
position in it replays on its own. Two stable keys are used, each cited with its argument;
no row ever says "same command" or "same suite". `SUITE` takes no argument.

- `FOCUS(<selector>)` = `python3 -m pytest packages/textkit/tests/test_redaction.py::<selector> -q`
- `SUITE` = `python3 -m pytest packages/textkit/tests/test_redaction.py -q`

Both are run from the workspace root
(`.../scratchpad/runs/T1-B-run1`), the directory containing `pytest.ini`.

---

## TDD evidence

<!-- rows appended one per red -> green cycle -->

### Row 1 — every occurrence of every secret is replaced

- **Seam authority**: `TASK.md` req 1 — "Every occurrence of each secret in `text` is replaced by the literal `[REDACTED]`."
- **Test file**: `packages/textkit/tests/test_redaction.py`
- **Selector**: `test_replaces_every_occurrence_of_each_secret`
- **Observed public entry point**: `textkit.redaction.redact("alpha beta alpha", ["alpha", "beta"])`
- **Red command**: `FOCUS(test_replaces_every_occurrence_of_each_secret)`
- **Intended red**: the seam does not exist yet, so importing it fails.
- **Observed red**: `1 error` — `ImportError: cannot import name 'redact' from 'textkit.redaction' (…/packages/textkit/textkit/redaction.py). Did you mean: 'redaction'?` The module path resolved to the intended file, so this is the intended missing-seam failure and not a `sys.path` artefact.
- **Minimal implementation**: `for secret in secrets: text = text.replace(secret, REDACTION)`.
- **Green command**: `FOCUS(test_replaces_every_occurrence_of_each_secret)`
- **Green result**: `1 passed in 0.00s`
- **Discriminating golden**: required `"[REDACTED] [REDACTED] [REDACTED]"` (authority: req 1). Rejected first-occurrence-only substitution yields `"[REDACTED] [REDACTED] alpha"`, and a one-secret-only loop yields `"[REDACTED] beta [REDACTED]"`; the golden differs from both, and the second assertion states that difference. The golden is a known-good literal, not recomputed the way the code computes it.
- **Note**: `"beta"` is exactly 4 characters and is required to be redacted here, so this row also pins the inclusive edge of the req 4 length floor (`< 4` is dropped, `== 4` is kept). A `<= 4` off-by-one in row 4's filter fails *this* row.

### Row 2 — matching is case-insensitive

- **Seam authority**: `TASK.md` req 2 — "Matching is case-insensitive: the secret `\"hunter2\"` also redacts `Hunter2` and `HUNTER2`."
- **Test file**: `packages/textkit/tests/test_redaction.py`
- **Selector**: `test_matches_secrets_case_insensitively`
- **Observed public entry point**: `textkit.redaction.redact("Hunter2 and HUNTER2 and hunter2", ["hunter2"])`
- **Red command**: `FOCUS(test_matches_secrets_case_insensitively)`
- **Intended red**: row 1's `str.replace` loop is case-sensitive, so only the exact-case `hunter2` is redacted.
- **Observed red**: `1 failed` — `AssertionError`, diff `- [REDACTED] and [REDACTED] and [REDACTED]` / `+ Hunter2 and HUNTER2 and [REDACTED]`. That is exactly the intended failure, and exactly the rejected implementation's output.
- **Minimal implementation**: replaced the loop with `re.compile("|".join(secrets), re.IGNORECASE).sub(REDACTION, text)`.
- **Green command**: `FOCUS(test_matches_secrets_case_insensitively)`
- **Green result**: `1 passed in 0.00s`; `SUITE` → `2 passed in 0.00s` (row 1 did not regress).
- **Discriminating golden**: required `"[REDACTED] and [REDACTED] and [REDACTED]"` (authority: req 2, which names `Hunter2` and `HUNTER2` explicitly). The rejected case-sensitive implementation produces `"Hunter2 and HUNTER2 and [REDACTED]"`; the input carries all three casings precisely so the two outputs cannot coincide.

### Row 3 — the longer secret wins at a shared position

- **Seam authority**: `TASK.md` req 3 — "When two secrets both match at the same position, the longer one wins."
- **Test file**: `packages/textkit/tests/test_redaction.py`
- **Selector**: `test_longer_secret_wins_when_two_match_at_the_same_position`
- **Observed public entry point**: `textkit.redaction.redact("the password is set", ["pass", "password"])`
- **Red command**: `FOCUS(test_longer_secret_wins_when_two_match_at_the_same_position)`
- **Intended red**: row 2's pattern joins the secrets in list order, and Python alternation is leftmost-*first*, so the shorter `pass` wins the tie and strands `word`.
- **Observed red**: `1 failed` — `AssertionError`, diff `- the [REDACTED] is set` / `+ the [REDACTED]word is set`. Exactly the intended failure.
- **Minimal implementation**: `longest_first = sorted(secrets, key=len, reverse=True)` before the join.
- **Green command**: `FOCUS(test_longer_secret_wins_when_two_match_at_the_same_position)`
- **Green result**: `1 passed in 0.00s`; `SUITE` → `3 passed in 0.00s` (rows 1–2 did not regress).
- **Discriminating golden**: required `"the [REDACTED] is set"` (authority: req 3). The secrets are deliberately passed shortest-first, so both rejected implementations — regex alternation in given order, and a sequential per-secret replace loop — produce `"the [REDACTED]word is set"`. Had the list been passed longest-first the golden would have passed under the rejected behaviour too, and would not have been discriminating.

### Row 4 — secrets shorter than four characters are ignored entirely

- **Seam authority**: `TASK.md` req 4 — "A secret shorter than 4 characters is ignored entirely — it is too short to be a secret, and redacting it would shred ordinary text."
- **Test file**: `packages/textkit/tests/test_redaction.py`
- **Selector**: `test_ignores_secrets_shorter_than_four_characters`
- **Observed public entry point**: `textkit.redaction.redact("the cat sat on the mat", ["cat"])`
- **Red command**: `FOCUS(test_ignores_secrets_shorter_than_four_characters)`
- **Intended red**: no length floor exists yet, so the 3-character secret is redacted.
- **Observed red**: `1 failed` — `AssertionError`, diff `- the cat sat on the mat` / `+ the [REDACTED] sat on the mat`. Exactly the intended failure.
- **Minimal implementation**: filter to `len(secret) >= MIN_SECRET_LENGTH` (4), plus an early `return text` when nothing survives the filter.
- **Green command**: `FOCUS(test_ignores_secrets_shorter_than_four_characters)`
- **Green result**: `1 passed in 0.00s`; `SUITE` → `4 passed in 0.00s` (rows 1–3 did not regress).
- **Discriminating golden**: required `"the cat sat on the mat"`, unchanged (authority: req 4). The rejected no-filter implementation produces `"the [REDACTED] sat on the mat"`.
- **Note — the empty guard is not speculative**: this row's input filters down to *no* usable secrets, and `"|".join([])` is the empty pattern, which matches at every position. Verified directly: `python3 -c "import re; print(repr(re.compile('', re.IGNORECASE).sub('[REDACTED]', 'the cat')))"` → `'[REDACTED]t[REDACTED]h[REDACTED]e[REDACTED] [REDACTED]c[REDACTED]a[REDACTED]t[REDACTED]'`. Without the guard this row cannot go green, so the guard is the minimum this cycle needs rather than anticipation of a future test.
- **Boundary**: the `>= 4` / `> 4` off-by-one is pinned by row 1, whose `"beta"` is exactly 4 characters and required to be redacted. No separate cycle was spent on it.

### Row 5 — punctuation inside a secret matches literally

- **Seam authority**: `TASK.md` req 1 — "Every occurrence of *each secret* in `text` is replaced". Read strictly: `abcd` is not an occurrence of the secret `a.cd`, so it must survive. This is a derived reading of req 1 rather than a requirement of its own; it is called out here because the derivation, not a quoted line, is the authority. The hazard is created by the regex implementation chosen in row 2, and real secrets do contain punctuation.
- **Test file**: `packages/textkit/tests/test_redaction.py`
- **Selector**: `test_matches_punctuation_in_a_secret_literally`
- **Observed public entry point**: `textkit.redaction.redact("a.cd and abcd", ["a.cd"])`
- **Red command**: `FOCUS(test_matches_punctuation_in_a_secret_literally)`
- **Intended red**: the secret is interpolated into the pattern unescaped, so `.` acts as a wildcard and `abcd` is over-redacted.
- **Observed red**: `1 failed` — `AssertionError`, diff `- [REDACTED] and abcd` / `+ [REDACTED] and [REDACTED]`. Exactly the intended failure.
- **Minimal implementation**: `re.escape(secret)` for each alternative before the join.
- **Green command**: `FOCUS(test_matches_punctuation_in_a_secret_literally)`
- **Green result**: `1 passed in 0.00s`; `SUITE` → `5 passed in 0.00s` (rows 1–4 did not regress).
- **Discriminating golden**: required `"[REDACTED] and abcd"` (authority: req 1). The rejected unescaped implementation produces `"[REDACTED] and [REDACTED]"`. The input deliberately contains both the literal secret and a string that only the wildcard reading matches, so the two outputs cannot coincide.

#### Judgement call recorded for review: the crash class

Before the fix, the same unescaped interpolation also made `redact` *raise* on ordinary
secrets rather than merely over-redact. Verified pre-fix, not assumed:

```
'a**b'  -> RAISES PatternError: multiple repeat at position 2
'(abc'  -> RAISES PatternError: missing ), unterminated subpattern at position 0
'a[bc'  -> RAISES PatternError: unterminated character set at position 1
'pw*(1' -> RAISES PatternError: missing ), unterminated subpattern at position 3
```

and post-fix all three of `'a**b'`, `'(abc'`, `'pw*(1'` return `'literal [REDACTED] here'`.

I did **not** spend a sixth cycle pinning this with its own test, and the reason belongs in
the record rather than in silence. Writing that test after the `re.escape` fix would have
been green on arrival, so recording it as a red → green cycle would have been false; and
folding it into row 5 as a second case would have made the row's red precede two greens.
The residual exposure is narrow: only an implementation that escaped `.` alone would pass
row 5 and still raise. If a future change touches the pattern construction, that is the
regression test to add first. Note also that `a++b` does *not* raise — it compiles as a
possessive quantifier — so the crash set is smaller than it first looks; this was checked
rather than presumed.

---

## Pre-review reconciliation (tests → seams)

**How the changed-test inventory was derived.** Running `git` is forbidden in this
environment, so the inventory does not come from a `git diff`. It is derived instead from
the repository's native test structure — pytest's own collection over the changed file:

`python3 -m pytest packages/textkit/tests/test_redaction.py --collect-only -q`

This is a sound substitute *here* specifically because
`packages/textkit/tests/test_redaction.py` measured 0 bytes before the first cycle
(`wc -c` at the start of the task), so every collected test is a new test and collection
and diff necessarily agree. On a file with pre-existing tests this substitution would not
hold and a real scoped diff would be required.

**Inventory** (5 collected selectors): `test_replaces_every_occurrence_of_each_secret`,
`test_matches_secrets_case_insensitively`,
`test_longer_secret_wins_when_two_match_at_the_same_position`,
`test_ignores_secrets_shorter_than_four_characters`,
`test_matches_punctuation_in_a_secret_literally`.

**Reconciliation rows** — one per changed selector, not a file- or class-level summary:

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `packages/textkit/tests/test_redaction.py` | `test_replaces_every_occurrence_of_each_secret` | `redact("alpha beta alpha", ["alpha", "beta"])` | TASK.md req 1 | Row 1 |
| `packages/textkit/tests/test_redaction.py` | `test_matches_secrets_case_insensitively` | `redact("Hunter2 and HUNTER2 and hunter2", ["hunter2"])` | TASK.md req 2 | Row 2 |
| `packages/textkit/tests/test_redaction.py` | `test_longer_secret_wins_when_two_match_at_the_same_position` | `redact("the password is set", ["pass", "password"])` | TASK.md req 3 | Row 3 |
| `packages/textkit/tests/test_redaction.py` | `test_ignores_secrets_shorter_than_four_characters` | `redact("the cat sat on the mat", ["cat"])` | TASK.md req 4 | Row 4 |
| `packages/textkit/tests/test_redaction.py` | `test_matches_punctuation_in_a_secret_literally` | `redact("a.cd and abcd", ["a.cd"])` | TASK.md req 1 (derived; see Row 5) | Row 5 |

**Both-direction difference check**, computed rather than eyeballed — the script parses the
selectors out of this file and compares them with pytest's collection:

```
inventory  - reconciled: EMPTY
reconciled - inventory:  EMPTY
GATE PASSES: True
```

**Coverage check.** Every row above claims public behaviour and proves it through the
ratified seam: each test's only import is `from textkit.redaction import redact`, and each
assertion is on that function's return value. Verified mechanically —
`grep -nE "^(import|from)|monkeypatch|_[a-z]+\(|\bre\.|sys\.|patch"` over the test file
returns the single `redact` import and the five `def test_…` lines, and nothing else. No
test reaches through a module global, helper, mutable global, side channel, or private
invariant, so no seam needs ratifying or rewriting, and no private-invariant test is
standing in for a public-behaviour proof.

---

## Final state

`SUITE` → `5 passed in 0.00s`.

Files left in place:

- `packages/textkit/textkit/redaction.py` — `redact`, plus `REDACTION` and `MIN_SECRET_LENGTH`
- `packages/textkit/tests/test_redaction.py` — the five tests above
- `pytest.ini` — test harness only (`pythonpath`, `testpaths`); no production behaviour

`packages/textkit/textkit/__init__.py` was left empty, so the ingress enumeration above
still holds: `textkit.redaction.redact` remains the single public entry point.

### Requirement-to-row map

| TASK.md requirement | Row |
| --- | --- |
| 1 — every occurrence of each secret replaced by `[REDACTED]` | Row 1 (and Row 5 for literal matching) |
| 2 — case-insensitive matching | Row 2 |
| 3 — longer secret wins at a shared position | Row 3 |
| 4 — secrets shorter than 4 characters ignored entirely | Row 4 (inclusive edge pinned by Row 1) |

### Not done, deliberately

- No `/code-review` was run, so there are no review-repair re-entry rows. The method's
  review-repair section applies from the first `TDD re-entry required: yes` finding; there
  are none yet, and an empty ledger is recorded here as empty rather than omitted.
