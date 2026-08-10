# TDD record — `parse_duration`

## Seams under test (written down before any test was authored)

| # | Seam | Authority | Ratified? |
|---|------|-----------|-----------|
| S1 | `duration.parse_duration(text: str) -> int`, reached as `from duration import parse_duration` | `TASK.md` § "Requirement for `parse_duration(text: str) -> int`" — the task statement is the explicit user decision in force here. It fixes the module file (`duration.py`), the function name, the signature, and a worked example (`parse_duration("90s")` is `90`). | Yes — authority names the seam outright, so no seam question was put to the user. |

No other seam is under test. `CONTEXT.md` does not exist in this workspace and I am scoped to the
workspace, so no project domain vocabulary was available to align names against; test names use the
requirement's own wording ("duration", "seconds", "suffix").

Scope note: the authority says "Only the `s` suffix is in scope for this task." It fixes no behavior
for any other suffix, for malformed input, or for a missing suffix. Under *Red before green* ("Don't
anticipate future tests or add speculative features") no test and no production branch was written
for those; they are unspecified, not merely untested.

## Slice plan

One behavior is specified, so one vertical slice. Tracer bullet, not a bulk test-first batch.

- Slice 1 — a duration written with a trailing `s` parses to that whole number of seconds.

---

## Choosing the golden (discrimination check)

The golden input is `"90s"` → `90`, taken from the authority's worked example rather than recomputed
the way the code computes it, so it is an independent literal and not tautological.

`"90s"` was kept rather than a shorter input such as `"5s"` because it discriminates against the
plausible rejected implementations:

| Rejected implementation | Result on `"90s"` | Caught? | Would `"5s"` have caught it? |
|---|---|---|---|
| Return the string with the suffix removed (`text[:-1]`) | `"90"` | Yes — `"90" != 90` in Python, so the assertion fails on type as well as value | Yes |
| Parse only the first digit | `9` | Yes — `9 != 90` | **No** — `"5s"` → `5`, passes by accident |
| Return the input's length (`len(text)`) | `3` | Yes — `3 != 90` | **No** — `"5s"` → `2`... but a 1-digit golden makes length/value collisions far likelier; `"90s"` separates them cleanly |
| Return a hardcoded small constant | any | Yes unless the constant is exactly `90` | Weaker — small constants collide with small goldens |
| Parse the whole string (`int(text)`) | `ValueError` | Yes | Yes |

A 2-digit golden is therefore the discriminating choice: value (90), first digit (9), and length (3)
are three distinct numbers, so the assertion can disagree with each wrong rule. A 1-digit golden
collapses value and first digit into the same number and cannot.

The assertion is `== 90` against the int, which also rejects the string-returning variant, since
Python's `"90" == 90` is `False`. No separate `isinstance` assertion was added — it would be a second
logical assertion covering a case the value assertion already fails.

### Substitution check (the table above, actually executed — not reasoned)

The red for slice 1 was a collection-time `ImportError`, which never executed the assertion. So the
golden had never been *observed* to fail. `tests.md` requires that a golden fail once the forbidden
behavior is substituted, so each rejected implementation was substituted into a throwaway copy of
`duration.py` (in a scratch dir, since deleted) and the suite re-run against the unchanged test:

| Substituted body of `parse_duration` | Result | Discriminates? |
|---|---|---|
| `return text[:-1]` | `1 failed` | Yes |
| `return int(text[0])` | `1 failed` | Yes |
| `return len(text)` | `1 failed` | Yes |
| `return int(text)` | `1 failed` | Yes |
| `return 90` | **`1 passed`** | **No** |

Replay: for each body above, write it into a `duration.py` beside a copy of `test_duration.py` in an
empty directory and run `python3 -m pytest test_duration.py -q` from that directory.

**Known limitation, stated plainly:** the golden does not discriminate against a stub that hardcodes
the golden's own answer. That is inherent to a single worked example, and it traces to a deviation
in this run: at green I wrote the general one-liner rather than the strictly minimal `return 90`. Had
I taken strict minimality, `return 90` would have been the implementation and a second, triangulating
example would then have been a genuine red that forced generalization. Because I generalized in one
step, no second example can ever go red here, and adding a green-on-arrival second case would be a
test that never demonstrated it can fail — so none was added. The constant stub is excluded by
inspection of the delivered `duration.py`, not by the test. This is the one point where the run
departs from the strictest reading of *Red before green*; it is recorded rather than papered over.

---

## TDD cycle evidence

One row per behavior, per *Retain the cycle evidence*. Both command positions below are complete
runnable commands; each replays on its own from the workspace root. Neither is a back-reference.

### Row 1 — slice 1

| Field | Value |
|---|---|
| **Seam authority** | `TASK.md` § "Requirement for `parse_duration(text: str) -> int`" (seam S1 above) — explicit user decision fixing module, name, signature, and the worked example `parse_duration("90s")` is `90` |
| **Test file** | `test_duration.py` |
| **Test selector** | `test_duration.py::test_duration_with_seconds_suffix_parses_to_that_many_seconds` |
| **Observed public entry point** | `duration.parse_duration`, reached as `from duration import parse_duration` — the module-level function, called with a `str` and asserted on its returned value. No internals touched, no mocks (nothing here is a system boundary, per `mocking.md`). |
| **Red command** | `python3 -m pytest test_duration.py::test_duration_with_seconds_suffix_parses_to_that_many_seconds -q` |
| **Intended red failure** | `duration.py` is empty, so the seam does not exist: import of `parse_duration` fails at collection with `ImportError: cannot import name 'parse_duration'`. |
| **Observed red failure** | `ImportError: cannot import name 'parse_duration' from 'duration' (…/duration.py)`, reported as `ERROR collecting test_duration.py` → `1 error in 0.01s`, with `ERROR: found no collectors for …::test_duration_with_seconds_suffix_parses_to_that_many_seconds`. **Matches the intended failure.** |
| **Minimal implementation** | `duration.py`: `return int(text.removesuffix("s"))` (plus module/function docstrings). No suffix validation, no error branches — the authority fixes no behavior for those. |
| **Green command** | `python3 -m pytest test_duration.py::test_duration_with_seconds_suffix_parses_to_that_many_seconds -q` |
| **Green result** | `1 passed in 0.00s`. Full suite `python3 -m pytest test_duration.py -q` also `1 passed in 0.00s`. |

---

## Pre-review reconciliation (tests → seams)

*Reconcile tests to seams before review* requires a changed-test inventory derived from the scoped
diff using the repository's native test structure.

**How the scope was derived without a diff tool:** running any git command is forbidden in this
environment, and this workspace is not a checkout in any case. The scope is nonetheless exact:
`test_duration.py` was 0 bytes when the task began (confirmed by the initial listing), so every
collected test in it is a changed test. The inventory is therefore pytest's own collection, which is
this project's native test structure:

```
$ python3 -m pytest test_duration.py --collect-only -q
test_duration.py::test_duration_with_seconds_suffix_parses_to_that_many_seconds
1 test collected in 0.00s
```

**Changed-test inventory (A)** — 1 identity:

1. `test_duration.py::test_duration_with_seconds_suffix_parses_to_that_many_seconds`

**Reconciliation rows (B)** — one row per changed selector, not per file or class:

| # | Test file | Exact selector | Observed public entry point | Seam authority | TDD evidence row |
|---|---|---|---|---|---|
| 1 | `test_duration.py` | `test_duration_with_seconds_suffix_parses_to_that_many_seconds` | `duration.parse_duration` via `from duration import parse_duration` | S1 — `TASK.md` § "Requirement for `parse_duration(text: str) -> int`" | Row 1 |

**Both-direction comparison:**

- In inventory A but not in reconciliation B: *(empty)*
- In reconciliation B but not in inventory A: *(empty)*

Both differences are empty, so the gate passes.

**Coverage check:** the row claims public behavior and proves it through the public entry point —
the test calls the exported function and asserts on its return value. It is not a private-invariant
test standing in for a public-behavior proof, and it reaches the seam through no module-internal
helper, mutable global, or side channel. `tests.md`'s multi-ingress rule is satisfied trivially:
`parse_duration` is the only exported entry point that admits this value; `duration.py` exports no
other parser, constructor, factory, or deserializer, so there is no alternative path that could
bypass it. There is no refusal path in scope, so the refusal/non-observation rule does not apply.

**Review re-entry ledger:** not applicable. `/code-review` was not run in this task, so there are no
findings marked `TDD re-entry required: yes` and no review-reentry rows to compare. If a review is
run later, each actionable finding gets its own appended row keyed by review pass, axis, and
finding — Row 1 must not be rewritten or extended to absorb one.

---

## Final state

- `duration.py` — `parse_duration`, one expression, in scope only for the `s` suffix.
- `test_duration.py` — 1 test at seam S1.
- Suite: `python3 -m pytest test_duration.py -q` → `1 passed`.
