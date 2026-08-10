# TDD record — `parse_duration`

Methodology: `method/SKILL.md` (+ `method/tests.md`, `method/mocking.md`).

## Seams under test, written down before any test

| Seam | Authority | Status |
| --- | --- | --- |
| `duration.parse_duration(text: str) -> int` — module-level exported function | `TASK.md`, "Requirement for `parse_duration(text: str) -> int`", which names the function, its signature, and the worked example `parse_duration("90s")` is `90` | Ratified by the written requirement supplied with the task. Not ambiguous, so no question was raised. |

This is the only seam. `parse_duration` is the single exported ingress path that
can admit a duration value, so the public-invariant enumeration in `tests.md`
(every parser, constructor, factory, deserializer) is satisfied by testing it
directly; there is no alternative path that could bypass it and nothing to
probe around.

No mocking. There is no system boundary here — no I/O, clock, randomness, or
external service — so per `mocking.md` nothing is mocked.

**Command key `SUITE`** = `python3 -m pytest test_duration.py -q`
(run from the workspace root; focus a case by appending `::<selector>` to
`test_duration.py`). Every command below is written out in full.

## TDD evidence rows

### Row 1 — a duration with the `s` suffix reads as whole seconds

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` requirement + its worked example `parse_duration("90s")` is `90` |
| Test file | `test_duration.py` |
| Test selector | `test_duration.py::test_seconds_suffix_gives_whole_seconds` |
| Observed public entry point | `parse_duration("90s")`, imported as `from duration import parse_duration` |
| Red command | `python3 -m pytest test_duration.py::test_seconds_suffix_gives_whole_seconds -q` |
| Intended red | The seam does not exist yet, so importing it fails |
| Observed red | `1 error` — `ImportError: cannot import name 'parse_duration' from 'duration'`. Matches the intended failure. |
| Green command | `python3 -m pytest test_duration.py::test_seconds_suffix_gives_whole_seconds -q` |
| Implementation | `def parse_duration(text: str) -> int: return 90` — faked constant, the least code that passes (see note below) |
| Observed green | `1 passed` |

Note on the faked constant: it is deliberate, not an oversight. `tests.md`
requires a golden to differ from each plausible rejected implementation, and
`parse_duration("90s") == 90` on its own would still pass under `return 90`.
Generalising in this cycle would have made Row 2 pass on arrival, i.e. a test
that never went red. Faking here keeps Row 2's red a genuine assertion failure
and is what turns the constant into a *rejected* implementation rather than an
untested one.

### Row 2 — the seconds value comes from the text, not a fixed value

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` requirement: "converts a duration written with a trailing `s` into a whole number of seconds" — a general rule over the value, not a single case |
| Test file | `test_duration.py` |
| Test selector | `test_duration.py::test_seconds_come_from_the_text_not_a_fixed_value` |
| Observed public entry point | `parse_duration("7s")`, imported as `from duration import parse_duration` |
| Red command | `python3 -m pytest test_duration.py::test_seconds_come_from_the_text_not_a_fixed_value -q` |
| Intended red | The faked constant returns 90 regardless of input, so a second value must disagree |
| Observed red | `1 failed` — `AssertionError: assert 90 == 7` / `where 90 = parse_duration('7s')`. Matches the intended failure. |
| Green command | `python3 -m pytest test_duration.py::test_seconds_come_from_the_text_not_a_fixed_value -q` |
| Implementation | `return int(text.removesuffix("s"))` |
| Observed green | `1 passed`; full suite `python3 -m pytest test_duration.py -q` → `2 passed` |

## Pre-review reconciliation

Changed-test inventory derived from the repository's native test structure
(`python3 -m pytest test_duration.py --collect-only -q`), not from recollection.
Both source files started empty, so every collected test is a changed test.

| Test file | Exact selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `test_duration.py` | `test_seconds_suffix_gives_whole_seconds` | `duration.parse_duration` | `TASK.md` requirement (worked example) | Row 1 |
| `test_duration.py` | `test_seconds_come_from_the_text_not_a_fixed_value` | `duration.parse_duration` | `TASK.md` requirement (general rule) | Row 2 |

Both-direction comparison:

- Inventory selectors not present in the reconciliation rows: **none**.
- Reconciliation rows not present in the inventory: **none**.

Both differences are empty. Every test enters through the ratified public
function; none reaches in through a module internal, helper, mutable global, or
side channel, so no seam needed re-ratifying or rewriting.

`/code-review` was not part of this task, so there are no review-reentry rows.
If review later raises findings marked `TDD re-entry required: yes`, each one
gets a **new** appended row keyed by review pass, axis, and finding — the two
rows above are not to be rewritten or extended to absorb it.

## Discriminating-golden check (`tests.md`)

Expected values are independent of the implementation: 90 is the literal from
the requirement's worked example, 7 is that same rule applied to a value the
code had never seen. Neither is recomputed the way the code computes it, so
neither test is tautological.

Sensitivity was verified rather than asserted. `sensitivity_probe.py`
substitutes each plausible rejected implementation and checks the goldens still
disagree with it — `python3 sensitivity_probe.py`, exit 0:

| Rejected implementation | Killed by |
| --- | --- |
| constant from the worked example (`return 90`) | `"7s"` → 90, required 7 |
| first character only | `"90s"` → 9, required 90 |
| length of the text | both goldens |
| treats the value as minutes (`× 60`) | both goldens |
| drops every digit after the first | `"90s"` → 9, required 90 |

No rejected implementation survives. This is why the second golden uses a
different magnitude (7, not another two-digit number) and the first uses a
multi-digit value: the pair, not either one alone, does the discriminating.

## Anti-pattern self-check

- **Implementation-coupled** — no. Both tests call the exported function and
  assert on its return value only; nothing inspects internals or verifies
  through a side channel. Refactor survival was checked by running, not
  assumed: with `text.removesuffix("s")` temporarily swapped for `text[:-1]`,
  `python3 -m pytest test_duration.py -q` still reported `2 passed`; the
  original implementation was then restored and re-ran `2 passed`.
- **Tautological** — no; see the golden check above.
- **Horizontal slicing** — no. Two vertical slices, each one test → one
  implementation, and slice 2 responded to what slice 1 left unpinned.

## Scope boundaries and one open question

`TASK.md` limits scope to the `s` suffix, so no test was written for input that
the requirement does not define, and no unrequested feature (minutes, hours,
fractional values, whitespace) was added.

One consequence is worth flagging rather than quietly deciding, because the
requirement fixes no behavior for it and the tests therefore do not pin it:
**malformed and suffix-less input has undefined behavior.** Observed, not
predicted: `parse_duration("90")` returns `90`, and `parse_duration("abc")`
raises `ValueError: invalid literal for int() with base 10: 'abc'`. Both are
accidents of the minimal
implementation, not ratified behavior. Two related choices were unconstrained
by any in-scope test, since `removesuffix("s")` and `text[:-1]` behave
identically on every input the requirement defines; `removesuffix` was chosen
because it names the requirement's suffix in the code. If callers need
validation or a specific error, that needs a seam authority first — then a new
red → green slice and a new evidence row.
