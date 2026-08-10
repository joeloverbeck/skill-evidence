# TDD record — `triage(records)`

All commands are run from the workspace root (the directory holding this file), which is where
`triage_service.py` and `test_triage_service.py` live and what makes `import triage_service`
resolve.

## Seams under test (agreed before any test was written)

| Seam | Authority |
| --- | --- |
| `triage_service.triage(records)` — module-level function, called with a list of record dicts, observed through its returned dict | `TASK.md` names the entry point, the module path, and requirements 1–3. Ratified up front, so no seam question was put to the user. |

No other seam is under test. There is no `CONTEXT.md` and no `docs/adr/` in this workspace, so
domain vocabulary is taken from `TASK.md`: a record is *decided* (id → `processed`) or it *cannot be
decided* (id → `untestable`).

Mocking: none. `triage` crosses no system boundary — no I/O, clock, randomness, or network — so per
`method/mocking.md` there is nothing legitimate to mock, and mocking anything here would be mocking
code we own.

## TDD evidence

Three vertical slices, one test → one implementation each. Every red was observed before the
implementation that answers it.

### Row 1 — result shape is exactly the two documented keys (requirement 1, first half)

- **Seam authority**: `TASK.md` requirement 1 ("returns a dict with exactly two keys, `processed`
  and `untestable`").
- **Test file / selector**: `test_triage_service.py::test_triage_returns_exactly_the_processed_and_untestable_keys`
- **Observed public entry point**: `triage_service.triage(...)`, imported as `from triage_service import triage`; asserted on the returned dict's key set.
- **Red command**: `python3 -m pytest test_triage_service.py::test_triage_returns_exactly_the_processed_and_untestable_keys -q`
- **Intended red**: no `triage` exists yet, so the test cannot even import its subject.
- **Observed red**: collection error —
  `ImportError: cannot import name 'triage' from 'triage_service'` (`1 error in 0.01s`). Matches the intent.
- **Minimal implementation**: `def triage(records): return {"processed": [], "untestable": []}` — enough for the key set and nothing more.
- **Green command**: `python3 -m pytest test_triage_service.py::test_triage_returns_exactly_the_processed_and_untestable_keys -q`
- **Green result**: `1 passed in 0.00s`.

### Row 2 — decided severities are processed in the given order (requirements 1 and 2)

- **Seam authority**: `TASK.md` requirements 2 (`low`/`medium`/`high` → `processed`) and 1 ("in the
  order the records were given").
- **Test file / selector**: `test_triage_service.py::test_low_medium_and_high_records_are_processed_in_the_given_order`
- **Observed public entry point**: `triage_service.triage(records)`; asserted on `result["processed"]`.
- **Red command**: `python3 -m pytest test_triage_service.py::test_low_medium_and_high_records_are_processed_in_the_given_order -q`
- **Intended red**: row 1's implementation returns an empty `processed`, so no decided id appears.
- **Observed red**: `AssertionError: assert [] == ['r2', 'r3', 'r1']` (`1 failed in 0.01s`). Matches the intent.
- **Minimal implementation**: every record id collected into `processed`, in input order; `untestable` still fixed empty.
- **Green command**: `python3 -m pytest test_triage_service.py::test_low_medium_and_high_records_are_processed_in_the_given_order -q`
- **Green result**: `1 passed in 0.00s`.
- **Golden choice**: ids are deliberately `r2, r3, r1` against severities `medium, high, low`, so the
  expected `["r2", "r3", "r1"]` is input order and nothing else — it disagrees with sorting by id and
  with sorting by severity rank, which would both give `["r1", "r2", "r3"]`. The literal is written
  out, not recomputed from `records`, so the test cannot pass by construction.

### Row 3 — undecidable records are recorded and processing carries on (requirements 2 and 3)

- **Seam authority**: `TASK.md` requirement 3 ("put its id in `untestable`, and carry on processing
  the remaining records as normal").
- **Test file / selector**: `test_triage_service.py::test_undecidable_records_are_untestable_and_later_records_still_process`
- **Observed public entry point**: `triage_service.triage(records)`; asserted on `result["untestable"]` and `result["processed"]`.
- **Red command**: `python3 -m pytest test_triage_service.py::test_undecidable_records_are_untestable_and_later_records_still_process -q`
- **Intended red**: row 2's implementation puts every id in `processed` and leaves `untestable` empty.
- **Observed red**: `AssertionError: assert [] == ['r2', 'r4']` (`1 failed in 0.01s`). Matches the intent.
- **Minimal implementation**: the loop in `triage_service.py` — decidable severities append to `processed`, everything else appends to `untestable`, no early exit.
- **Green command**: `python3 -m pytest test_triage_service.py::test_undecidable_records_are_untestable_and_later_records_still_process -q`
- **Green result**: `1 passed in 0.00s`.
- **Golden choice**: the undecidable records sit *between* decidable ones (`r2` at index 1, `r4` at
  index 3, with `r5` still to come), so the expected `processed == ["r1", "r3", "r5"]` is only
  reachable if processing continues past both. An implementation that stops at the first undecidable
  record — the reading requirement 3 exists to forbid — yields `["r1"]` and fails.

Whole-suite command: `python3 -m pytest test_triage_service.py -q` → `3 passed in 0.00s`.

## Sensitivity check on the goldens

An independent literal is necessary but not sufficient, so each rejected implementation was
substituted for the real one and the suite re-run, to confirm some test actually objects. Every
mutant was caught; `triage_service.py` was restored afterwards and the suite re-run to `3 passed`.

| Substituted (rejected) behavior | Suite result | Caught by |
| --- | --- | --- |
| Stop at the first undecidable record | 1 failed, 2 passed | row 3 |
| `processed` sorted by severity rank rather than input order | 2 failed, 1 passed | rows 2 and 3 |
| Undecidable ids also counted as processed | 1 failed, 2 passed | row 3 |
| A third key (`skipped`) added to the result | 1 failed, 2 passed | row 1 |
| Empty-string severity treated as decidable (truthiness slip) | 1 failed, 2 passed | row 3 |

The empty-severity mutant is why `r4` carries `""` rather than another word: it makes the test
sensitive to a plausible `if severity in ... or not severity` slip that a word-only fixture would
miss.

## Pre-review reconciliation (changed tests → seams)

Git is forbidden in this workspace, so the changed-test inventory could not be derived from a git
diff. Both source files were empty when the task began, so every test the suite now collects is a
changed (new) test, and the inventory below is the collection output —
`python3 -m pytest test_triage_service.py --collect-only -q`, run from the workspace root — which is
pytest's own native test structure rather than a hand-written list.

| # | Test file | Exact selector (node id) | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `test_triage_service.py` | `test_triage_returns_exactly_the_processed_and_untestable_keys` | `triage_service.triage(...)` return value | `TASK.md` req. 1 | Row 1 |
| 2 | `test_triage_service.py` | `test_low_medium_and_high_records_are_processed_in_the_given_order` | `triage_service.triage(...)` return value | `TASK.md` reqs. 1, 2 | Row 2 |
| 3 | `test_triage_service.py` | `test_undecidable_records_are_untestable_and_later_records_still_process` | `triage_service.triage(...)` return value | `TASK.md` reqs. 2, 3 | Row 3 |

Two-way difference check:

- Inventory selectors not present in the reconciliation rows: **none**.
- Reconciliation rows not present in the inventory: **none**.

Both differences are empty, so the gate passes. No test enters through a module-private name, helper,
mutable global, or side channel: all three call the exported `triage` and assert only on its returned
dict, which is the sole ratified seam. `DECIDABLE_SEVERITIES` is module state the tests deliberately
do not import or assert against — restating it in a test would make the assertions tautological.

Coverage of the authority: requirement 1 is covered by rows 1 (key set) and 2 (order), requirement 2
by rows 2 and 3, requirement 3 by row 3. No requirement is left resting on a private-invariant test.

## Deliberately not written

Two candidate cases were left out because they would have arrived green — there is no red to observe,
so they are assertions about an implementation that already exists rather than TDD slices:

- `triage([])` returning two empty lists — already satisfied from row 1 onward.
- A case-mismatched severity such as `"High"` going to `untestable` — the same code path as any other
  unrecognized string, already pinned by row 3.

Both are cheap to add if a future change makes them discriminating; noting them here so the omission
is a decision on the record rather than an oversight.

## Review re-entry

None. `/code-review` has not been run against this change, so there are no review-pass/axis/finding
rows to append and no `Findings fixed during review` ledger to reconcile against. If a review runs
later, each finding marked `TDD re-entry required: yes` gets its own appended row and its own focused
red — no rewriting of rows 1–3 to absorb it.
