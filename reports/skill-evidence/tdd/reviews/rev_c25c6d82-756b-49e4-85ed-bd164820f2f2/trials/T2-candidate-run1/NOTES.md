# TDD record — `triage(records)`

Methodology: `method/SKILL.md` (tdd), with `method/tests.md` and `method/mocking.md`.

## Seams under test (written down before the first test)

| Seam | Authority |
| --- | --- |
| `triage(records)` exported from `triage_service.py`, called with a list of record dicts, observed through its returned dict | `TASK.md` → "Requirements for `triage(records)`", items 1–3. The task statement is the explicit user decision that ratifies this seam. |

No other seam is under test. Nothing internal is exercised: `DECIDABLE_SEVERITIES` and the loop
are reachable only through `triage`, and every assertion reads the returned dict.

No mocking. `method/mocking.md` limits mocks to system boundaries (external APIs, DB, time,
randomness, filesystem); `triage` touches none of them, so there is nothing to mock.

## Command keys

Every command position below replays on its own via these keys. `W` is the workspace root:

- `W` := `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-B-run1`
- `FOCUS(<selector>)` := `python3 -m pytest W/test_triage_service.py::<selector> -q`
- `SUITE` := `python3 -m pytest W/test_triage_service.py -q`
- `INVENTORY` := `python3 -m pytest W/test_triage_service.py --collect-only -q`
- `SENSITIVITY` := `python3 W/sensitivity_check.py`

(Substitute the literal path for `W`. The commands are cwd-independent.)

## TDD evidence rows

### Row 1 — decided record ids are reported in the order given

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` req. 1 ("in the order the records were given") and req. 2 (`low`/`medium`/`high` → `processed`) |
| Test file | `test_triage_service.py` |
| Test selector | `test_decided_records_are_reported_in_the_order_given` |
| Observed public entry point | `triage(records)` → `result["processed"]` |
| Red command | `FOCUS(test_decided_records_are_reported_in_the_order_given)` |
| Intended failure | No public entry point yet — `triage` is not importable from the empty `triage_service.py` |
| Observed failure | `ImportError: cannot import name 'triage' from 'triage_service'` (1 error) — matches the intended failure |
| Minimal implementation | `triage` returns `{"processed": [record["id"] for record in records]}` |
| Green command | `FOCUS(test_decided_records_are_reported_in_the_order_given)` |
| Green result | `1 passed` |

### Row 2 — the result carries exactly the two documented keys

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` req. 1 ("returns a dict with exactly two keys, `processed` and `untestable`") |
| Test file | `test_triage_service.py` |
| Test selector | `test_result_has_exactly_the_processed_and_untestable_keys` |
| Observed public entry point | `triage(records)` → `sorted(result.keys())` |
| Red command | `FOCUS(test_result_has_exactly_the_processed_and_untestable_keys)` |
| Intended failure | The returned dict carries only `processed`, so the key set is short of `untestable` |
| Observed failure | `AssertionError: assert ['processed'] == ['processed', 'untestable']` — "Right contains one more item: 'untestable'" (1 failed) — matches the intended failure |
| Minimal implementation | The returned dict gains `"untestable": []` |
| Green command | `FOCUS(test_result_has_exactly_the_processed_and_untestable_keys)` |
| Green result | `1 passed` |

### Row 3 — an undecidable severity is recorded and processing continues

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` req. 3 ("any other `severity` … put its id in `untestable`, and carry on processing the remaining records as normal") |
| Test file | `test_triage_service.py` |
| Test selector | `test_undecidable_severity_is_recorded_and_processing_continues` |
| Observed public entry point | `triage(records)` → the whole returned dict |
| Red command | `FOCUS(test_undecidable_severity_is_recorded_and_processing_continues)` |
| Intended failure | Every id still lands in `processed` and `untestable` is the hard-coded empty list, so both keys differ from the required split |
| Observed failure | `AssertionError` with differing items `{'untestable': []} != {'untestable': ['r-2', 'r-4']}` and `{'processed': ['r-1', 'r-2', 'r-3', 'r-4', 'r-5']} != {'processed': ['r-1', 'r-3', 'r-5']}` (1 failed) — matches the intended failure |
| Minimal implementation | Loop over records, routing each id by `record["severity"] in DECIDABLE_SEVERITIES` |
| Green command | `FOCUS(test_undecidable_severity_is_recorded_and_processing_continues)` |
| Green result | `1 passed` |

Whole suite after row 3: `SUITE` → `3 passed`.

## Discriminating goldens

Each expected value is an independent literal taken from `TASK.md`, never recomputed the way the
code computes it. Each golden's input is chosen so that its required result differs from the
plausible rejected implementations, and that was checked by substituting the rejected behavior and
re-running the golden — `SENSITIVITY`, which writes each rejected implementation into a throwaway
directory next to a copy of the real test file, runs the named golden against it, and asserts the
golden fails.

| Golden | Rejected implementation | Why the input discriminates | `SENSITIVITY` |
| --- | --- | --- | --- |
| `test_decided_records_are_reported_in_the_order_given` | M1: sorted by id | Ids are given as `b-2`, `a-1`, `c-3`; sorting yields `a-1, b-2, c-3` | CAUGHT |
| " | M2: grouped by severity | Grouping low→medium→high yields `a-1, c-3, b-2` | CAUGHT |
| `test_result_has_exactly_the_processed_and_untestable_keys` | M3: `untestable` omitted when empty | The input is all-decided, so the key is only present if it is unconditional | CAUGHT |
| " | M4: an extra diagnostic key | The assertion is on the exact key set, not on key presence | CAUGHT |
| `test_undecidable_severity_is_recorded_and_processing_continues` | M5: stops at the first undecidable record | The undecidable records sit at positions 2 and 4 of 5, with decided records after both | CAUGHT |
| " | M6: undecidable records dropped silently | `untestable` is asserted to be `["r-2", "r-4"]`, not merely absent from `processed` | CAUGHT |
| " | M7: case-insensitive severity match | `r-4` carries `"HIGH"`, which req. 2 does not list and req. 3 therefore sends to `untestable` | CAUGHT |

`SENSITIVITY` → `All 7 rejected implementations were caught.` (exit 0). No golden survives the
behavior it rejects.

## Pre-review reconciliation

Constraint: git is forbidden in this run, so the scoped diff is derived instead from the
starting state — `triage_service.py` and `test_triage_service.py` were both 0 bytes at the start
(confirmed by `ls -la` before the first cycle), so every collected test is a changed (added) test.
The inventory is the repository's native test structure, taken from pytest collection.

Changed-test inventory (`INVENTORY`, 3 tests collected):

1. `test_triage_service.py::test_decided_records_are_reported_in_the_order_given`
2. `test_triage_service.py::test_result_has_exactly_the_processed_and_untestable_keys`
3. `test_triage_service.py::test_undecidable_severity_is_recorded_and_processing_continues`

Reconciliation rows:

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `test_triage_service.py` | `test_decided_records_are_reported_in_the_order_given` | `triage(records)` → `result["processed"]` | `TASK.md` req. 1 (order) + req. 2 | Row 1 |
| `test_triage_service.py` | `test_result_has_exactly_the_processed_and_untestable_keys` | `triage(records)` → `sorted(result.keys())` | `TASK.md` req. 1 (exactly two keys) | Row 2 |
| `test_triage_service.py` | `test_undecidable_severity_is_recorded_and_processing_continues` | `triage(records)` → the whole returned dict | `TASK.md` req. 3 | Row 3 |

Both-direction comparison of identities:

- Inventory minus reconciliation: empty.
- Reconciliation minus inventory: empty.

Coverage check: every row claims public behavior and proves it through `triage`'s return value.
No row enters through a module internal, helper, mutable global, or side channel, so no seam
needs ratifying or rewriting. No private-invariant test stands in for a public-behavior proof.

## Review re-entry rows

None. `/code-review` has not been run for this change, so there are no
`TDD re-entry required: yes` findings and no `Findings fixed during review` ledger to reconcile
against. If a review happens later, each actionable finding gets its own appended row keyed by
review pass, axis, and finding — no earlier row is rewritten to absorb it.

## Files

- `triage_service.py` — the implementation (public entry point: `triage`).
- `test_triage_service.py` — the three tests above.
- `sensitivity_check.py` — the discriminating-golden harness; throwaway mutants only, it cleans
  up after itself and is not collected by pytest.
