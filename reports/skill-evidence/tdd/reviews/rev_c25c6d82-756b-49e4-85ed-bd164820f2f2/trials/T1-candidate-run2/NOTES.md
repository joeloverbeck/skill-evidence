# TDD record — `textkit.redaction.redact`

## Seams under test

Written down before the first test, as the loop requires.

| Seam | Authority | Status |
| --- | --- | --- |
| `textkit.redaction.redact(text, secrets) -> str` | `TASK.md` §"Requirements for `redact(text: str, secrets: list[str]) -> str`", items 1–4 | Ratified by the task document; no ask needed |

There is no `CONTEXT.md` and no `docs/adr/` in this workspace, so `TASK.md` is the
only authority available, and it names this function as the deliverable interface.

**Ingress enumeration** (tests.md, "Public invariant coverage"): the module exports
exactly one callable, `redact`. `packages/textkit/textkit/__init__.py` is empty, so
there is no re-export, alias, factory, or deserializer that could admit a value by a
second path. The single seam above is therefore the whole public ingress surface, and
every test enters through it. No test touches a private helper.

## Stable command keys

Every command position below replays on its own via these keys. `<W>` is the workspace
root, `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T1-B-run2`.

- `FOCUS(<selector>)` := `cd <W> && python3 -m pytest packages/textkit/tests/test_redaction.py::<selector> -q`
- `SUITE` := `cd <W> && python3 -m pytest packages/textkit/tests/test_redaction.py -q`

## Harness note

`packages/textkit/conftest.py` was added (empty of fixtures) so that pytest places
`packages/textkit` on `sys.path` and `TASK.md`'s prescribed command imports `textkit`
without extra environment. This is harness setup, not production behavior, so it sits
outside the red → green loop.

## TDD cycle evidence

One row per behavior, in the order the slices were run.

### Cycle 1 — every occurrence of a secret is replaced

- **Seam authority**: `TASK.md` requirement 1 ("Every occurrence of each secret in
  `text` is replaced by the literal `[REDACTED]`").
- **Test**: `packages/textkit/tests/test_redaction.py::test_every_occurrence_of_a_secret_is_replaced`
- **Observed public entry point**: `redact("token swordfish, backup swordfish", ["swordfish"])`
- **Discriminating golden**: two occurrences of the same secret. The required output
  `"token [REDACTED], backup [REDACTED]"` differs from the rejected
  replace-the-first-match-only reading, which yields `"token [REDACTED], backup swordfish"`.
  The expected string is a hand-written literal from requirement 1, not recomputed.
- **Red**: `FOCUS(test_every_occurrence_of_a_secret_is_replaced)`.
  Intended failure — the seam does not exist yet, so the import fails.
  Observed: `ImportError: cannot import name 'redact' from 'textkit.redaction'`;
  `1 error in 0.01s`. (The module itself resolved, so this is the seam's absence, not a
  path problem.)
- **Green**: `FOCUS(test_every_occurrence_of_a_secret_is_replaced)` → `1 passed in 0.00s`.
- **Implementation**: `redact` loops the secrets and applies `str.replace` for each.

### Cycle 2 — matching is case-insensitive

- **Seam authority**: `TASK.md` requirement 2 ("Matching is case-insensitive: the secret
  `"hunter2"` also redacts `Hunter2` and `HUNTER2`").
- **Test**: `packages/textkit/tests/test_redaction.py::test_matching_is_case_insensitive`
- **Observed public entry point**: `redact("Hunter2 and HUNTER2 and hunter2", ["hunter2"])`
- **Discriminating golden**: the two capitalization variants named in the requirement sit
  beside the exact-case one. The required
  `"[REDACTED] and [REDACTED] and [REDACTED]"` differs from the rejected case-sensitive
  reading, which leaves `"Hunter2 and HUNTER2 and [REDACTED]"`.
- **Red**: `FOCUS(test_matching_is_case_insensitive)`.
  Intended failure — the cycle-1 `str.replace` is case-sensitive, so only the lowercase
  occurrence should go. Observed exactly that:
  `AssertionError: ... + Hunter2 and HUNTER2 and [REDACTED]`; `1 failed in 0.01s`.
- **Green**: `FOCUS(test_matching_is_case_insensitive)` → `1 passed in 0.00s`.
- **Implementation**: swapped `str.replace` for `re.sub(re.escape(secret), ..., flags=re.IGNORECASE)`.

### Cycle 3 — the longer secret wins at the same position

- **Seam authority**: `TASK.md` requirement 3 ("When two secrets both match at the same
  position, the longer one wins").
- **Test**: `packages/textkit/tests/test_redaction.py::test_longer_secret_wins_at_the_same_position`
- **Observed public entry point**: `redact("my password is set", ["pass", "password"])`
- **Discriminating golden**: `"pass"` and `"password"` match at the same index, and the
  shorter one is listed first, so the required `"my [REDACTED] is set"` differs from the
  rejected in-list-order reading (`"my [REDACTED]word is set"`). Both secrets are four
  characters or longer, so requirement 4 cannot decide the case and the golden isolates
  requirement 3.
- **Red**: `FOCUS(test_longer_secret_wins_at_the_same_position)`.
  Intended failure — cycle 2 substitutes in list order, so `"pass"` should be consumed
  first and strand `"word"`. Observed exactly that: `+ my [REDACTED]word is set`;
  `1 failed in 0.01s`.
- **Green**: `FOCUS(test_longer_secret_wins_at_the_same_position)` → `1 passed in 0.00s`.
- **Implementation**: iterate `sorted(secrets, key=len, reverse=True)` so longer secrets
  are consumed first.

### Cycle 4 — a secret shorter than four characters is ignored

- **Seam authority**: `TASK.md` requirement 4 ("A secret shorter than 4 characters is
  ignored entirely").
- **Test**: `packages/textkit/tests/test_redaction.py::test_secret_shorter_than_four_characters_is_ignored`
- **Observed public entry point**: `redact("cat catalog and cathedral", ["cat"])`
- **Discriminating golden**: the three-character secret is embedded in ordinary words, so
  the required unchanged `"cat catalog and cathedral"` differs from the rejected
  honour-every-secret reading, which shreds the sentence into
  `"[REDACTED] [REDACTED]alog and [REDACTED]hedral"` — the exact harm the requirement
  names.
- **Red**: `FOCUS(test_secret_shorter_than_four_characters_is_ignored)`.
  Intended failure — cycle 3 honours every secret regardless of length, so the ordinary
  words should be shredded. Observed exactly that:
  `+ [REDACTED] [REDACTED]alog and [REDACTED]hedral`; `1 failed in 0.01s`.
- **Green**: `FOCUS(test_secret_shorter_than_four_characters_is_ignored)` → `1 passed in 0.00s`.
- **Implementation**: filter the secrets to `len(secret) >= MIN_SECRET_LENGTH` before
  substituting.

### Cycle 5 — the marker is not itself redacted

- **Seam authority**: `TASK.md` requirement 1. Read closely, it fixes two things: the
  occurrences that count are the ones *in `text`*, and what replaces them is *the literal*
  `[REDACTED]`. This is a derivation from requirement 1's wording, not a separate clause,
  and it is flagged as such because the requirement list never names the case.
- **Test**: `packages/textkit/tests/test_redaction.py::test_marker_text_is_not_itself_redacted`
- **Observed public entry point**: `redact("my password is set", ["password", "acted"])`
- **Discriminating golden**: `"acted"` does not occur in the input at all — it only appears
  inside the `[REDACTED]` that redacting `"password"` writes. The required
  `"my [REDACTED] is set"` differs from the rejected redact-the-output reading, which
  yields `"my [RED[REDACTED]] is set"`: an occurrence that was never in `text`, and a
  replacement that is no longer the literal marker.
- **Red**: `FOCUS(test_marker_text_is_not_itself_redacted)`.
  Intended failure — cycles 1–4 substitute one secret at a time over the *output* of the
  previous substitution, so the second secret should match inside the marker. Observed
  exactly that: `+ my [RED[REDACTED]] is set`; `1 failed in 0.01s`.
- **Green**: `FOCUS(test_marker_text_is_not_itself_redacted)` → `1 passed in 0.00s`;
  `SUITE` → `5 passed in 0.00s`.
- **Implementation**: replaced the per-secret loop with a single `re.sub` pass over the
  original text, using one alternation of the escaped secrets ordered longest-first.
  Python's leftmost-first alternation is what now carries requirement 3, so cycle 3's test
  guards the ordering. The empty-`usable` early return is required too: an empty
  alternation matches the empty string everywhere, and cycle 4's test catches that.

### Regression guard outside the loop — the four-character boundary

This one is **not** a red → green cycle, and it is recorded separately rather than dressed
up as one.

- **Test**: `packages/textkit/tests/test_redaction.py::test_secret_of_exactly_four_characters_is_still_redacted`
- **Seam authority**: `TASK.md` requirement 4 — "*shorter than* 4" honours a secret of
  exactly 4.
- **Observed public entry point**: `redact("key 4f9c rotated", ["4f9c"])`
- **Why no red**: cycle 4's minimal implementation had to pick a threshold, and
  `len(secret) >= 4` is already the correct one. Once that was green, no honest red for the
  boundary existed. Manufacturing one by first shipping `> 4` on purpose would be a
  fabricated red, so I did not.
- **Why it is here anyway**: a sensitivity check showed the boundary was genuinely
  unpinned. Running the rejected off-by-one variant (`MIN_SECRET_LENGTH = 5`, patched on
  the imported module in a throwaway process — the production file was never edited)
  against all five cycle goldens, **all five still passed**. The suite would have accepted
  the off-by-one. The boundary golden fails under that variant
  (`'key 4f9c rotated'` unredacted), so it discriminates, which is what
  `tests.md` §"Discriminating goldens" asks of it.
- **Sensitivity probe, replayable**:

  ```
  cd <W> && python3 -c 'import sys; sys.path.insert(0, "packages/textkit"); from textkit import redaction; redaction.MIN_SECRET_LENGTH = 5; print(redaction.redact("key 4f9c rotated", ["4f9c"]))'
  ```

  Observed: prints `key 4f9c rotated` — unredacted, so the boundary golden does fail under
  the mutant. The five-golden sweep was the same construction over a heredoc, evaluating
  each cycle's input/expected pair against the mutated module and printing PASS/FAIL;
  observed output was `PASS` for cycles 1–5 and `FAIL` for the boundary candidate.
- **First run**: `FOCUS(test_secret_of_exactly_four_characters_is_still_redacted)` →
  `1 passed in 0.00s`. Passed on first run, as expected for a guard.

**Deviation, stated plainly**: the loop's "red before green" rule is not satisfied by this
test, and the reconciliation row below therefore has no TDD evidence row to name. The
methodology has no slot for a guard with no available red; I would rather leave the gap
visible than either drop real coverage or fake a cycle. If the boundary had been folded
into cycle 4 as a second explicitly named parameterized case, both cases would have gone
red together and this deviation would not exist — that is the sequencing mistake, and it
was mine.

## Pre-review reconciliation

Run before handing the change to `/code-review`.

**Changed-test inventory.** Derived from the repository's native test structure via
`cd <W> && python3 -m pytest packages/textkit/tests/test_redaction.py --collect-only -q`
(`6 tests collected`). `packages/textkit/tests/test_redaction.py` started empty, so every
collected node is a changed test; no git command was run to produce this.

| # | Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `packages/textkit/tests/test_redaction.py` | `test_every_occurrence_of_a_secret_is_replaced` | `redact(text, secrets)` | `TASK.md` req. 1 | Cycle 1 |
| 2 | `packages/textkit/tests/test_redaction.py` | `test_matching_is_case_insensitive` | `redact(text, secrets)` | `TASK.md` req. 2 | Cycle 2 |
| 3 | `packages/textkit/tests/test_redaction.py` | `test_longer_secret_wins_at_the_same_position` | `redact(text, secrets)` | `TASK.md` req. 3 | Cycle 3 |
| 4 | `packages/textkit/tests/test_redaction.py` | `test_secret_shorter_than_four_characters_is_ignored` | `redact(text, secrets)` | `TASK.md` req. 4 | Cycle 4 |
| 5 | `packages/textkit/tests/test_redaction.py` | `test_marker_text_is_not_itself_redacted` | `redact(text, secrets)` | `TASK.md` req. 1 (derived — see cycle 5) | Cycle 5 |
| 6 | `packages/textkit/tests/test_redaction.py` | `test_secret_of_exactly_four_characters_is_still_redacted` | `redact(text, secrets)` | `TASK.md` req. 4 (boundary) | **none** — see the deviation above |

**Both-directions comparison.**

- Inventory identities not present in the reconciliation rows: none.
- Reconciliation identities not present in the inventory: none.

Both differences are empty, so the inventory gate passes. The separate cycle-to-row check
does not: row 6 names no TDD evidence row. That is the one open deviation, disclosed
rather than closed.

**Seam check.** All six rows enter through the ratified public seam `redact(text, secrets)`.
No test imports a private helper, reads a module global, patches a collaborator, or asserts
through a side channel. `REDACTION_MARKER` and `MIN_SECRET_LENGTH` exist for the
implementation's own readability and are deliberately not asserted on — the tests spell the
literal `[REDACTED]` out, so renaming or inlining either constant leaves them all green.
Nothing is mocked; there is no system boundary here to mock (`mocking.md`).

## Final state

`SUITE` → `6 passed in 0.00s`.
