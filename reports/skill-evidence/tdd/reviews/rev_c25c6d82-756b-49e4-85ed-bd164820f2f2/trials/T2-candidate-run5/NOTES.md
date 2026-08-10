# TDD evidence — `triage(records)`

## Command key

All command positions below use one stable key, defined here and referenced with its argument:

- `PYTEST(<args>)` = `cd /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-B-run5 && python3 -m pytest <args> -q`

## Seams under test

| Seam | Authority |
| --- | --- |
| `triage(records)` imported from module `triage_service` — its returned dict is the only observed output | `TASK.md` §"Requirements for `triage(records)`" (names the function, the module file, and the pytest entry point) |

One seam only. `TASK.md` fixes it explicitly, so it is already agreed and no user question was needed. No `CONTEXT.md` or ADRs exist in this workspace, so no additional domain vocabulary or constraints applied. Nothing is mocked: the unit under test has no system boundary — no I/O, clock, or randomness.

## TDD evidence rows

### Row 1 — decided records reach `processed` in input order

- **Seam authority**: `TASK.md` requirements 1 (order) and 2 (`low`/`medium`/`high` are decided)
- **Test file / selector**: `test_triage_service.py::test_decided_records_are_processed_in_input_order`
- **Observed public entry point**: `triage(records)` from `triage_service`
- **Red command**: `PYTEST(test_triage_service.py::test_decided_records_are_processed_in_input_order)`
- **Intended failure**: no public `triage` entry point exists yet
- **Observed failure**: collection error — `ImportError: cannot import name 'triage' from 'triage_service'`
- **Minimal implementation**: `triage` returns `{"processed": [<every id>]}`
- **Green command**: `PYTEST(test_triage_service.py::test_decided_records_are_processed_in_input_order)`
- **Green result**: `1 passed`

### Row 2 — the result carries exactly the two keys

- **Seam authority**: `TASK.md` requirement 1 ("a dict with exactly two keys, `processed` and `untestable`")
- **Test file / selector**: `test_triage_service.py::test_result_carries_exactly_the_processed_and_untestable_keys`
- **Observed public entry point**: `triage(records)` from `triage_service`
- **Red command**: `PYTEST(test_triage_service.py::test_result_carries_exactly_the_processed_and_untestable_keys)`
- **Intended failure**: the returned dict omits `untestable`
- **Observed failure**: `AssertionError` — set comparison, "Extra items in the right set: 'untestable'"
- **Minimal implementation**: added `"untestable": []` to the returned dict
- **Green command**: `PYTEST(test_triage_service.py::test_result_carries_exactly_the_processed_and_untestable_keys)`
- **Green result**: `1 passed`

### Row 3 — undecidable records are recorded and processing continues

- **Seam authority**: `TASK.md` requirement 3 ("put its id in `untestable`, and carry on processing the remaining records as normal")
- **Test file / selector**: `test_triage_service.py::test_undecidable_records_are_recorded_and_processing_continues`
- **Observed public entry point**: `triage(records)` from `triage_service`
- **Red command**: `PYTEST(test_triage_service.py::test_undecidable_records_are_recorded_and_processing_continues)`
- **Intended failure**: undecidable ids are classified as processed and `untestable` stays empty
- **Observed failure**: `AssertionError` — `{'processed': ['r-3', 'r-8', 'r-1', 'r-6', 'r-4']} != {'processed': ['r-3', 'r-1', 'r-4']}` and `{'untestable': []} != {'untestable': ['r-8', 'r-6']}`
- **Minimal implementation**: partition loop over records, appending each id to `processed` when `severity` is in `DECIDABLE_SEVERITIES` and to `untestable` otherwise
- **Green command**: `PYTEST(test_triage_service.py::test_undecidable_records_are_recorded_and_processing_continues)`
- **Green result**: `1 passed`

Full suite after the last slice: `PYTEST(test_triage_service.py)` → `3 passed`.

## Discriminating goldens

Every expected value is a known-good literal read off `TASK.md`, never recomputed the way the implementation computes it. The inputs were chosen so each golden differs from the plausible rejected implementations:

- Ids are deliberately out of lexicographic order (`r-9`, `r-2`, `r-5`), so a sorting implementation yields a different list.
- Row 3 places undecidable records in the *middle* of the run (`r-8` at index 1, `r-6` at index 3) with decidable records after them, so a stop-at-first-undecidable implementation loses `r-1` and `r-4`.
- Row 3 includes `"HIGH"`, which requirement 2 does not list, so a case-insensitive match yields a different partition.

Sensitivity was verified by substitution rather than assumed: a throwaway probe swapped five rejected implementations behind the same public entry point and re-ran all three tests (probe deleted afterwards; it was not a deliverable). Results:

| Rejected implementation | Caught by |
| --- | --- |
| sorts processed ids | rows 1 and 3 |
| stops at first undecidable | row 3 |
| matches severity case-insensitively | row 3 |
| drops undecidable ids | row 3 |
| omits the `untestable` key | rows 2 and 3 |

No rejected implementation survived, so no golden needed replacing.

## Pre-review reconciliation

Changed-test inventory derived from the repository's native test structure via
`PYTEST(test_triage_service.py --collect-only)` → 3 tests collected.

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `test_triage_service.py` | `test_decided_records_are_processed_in_input_order` | `triage(records)` from `triage_service` | `TASK.md` reqs 1–2 | Row 1 |
| `test_triage_service.py` | `test_result_carries_exactly_the_processed_and_untestable_keys` | `triage(records)` from `triage_service` | `TASK.md` req 1 | Row 2 |
| `test_triage_service.py` | `test_undecidable_records_are_recorded_and_processing_continues` | `triage(records)` from `triage_service` | `TASK.md` req 3 | Row 3 |

Both-directions comparison between the collected inventory and the reconciliation rows:

- inventory − reconciliation: empty
- reconciliation − inventory: empty

Every test enters through the `triage` function ratified by `TASK.md`. No test reaches a module internal, helper, mutable global, or side channel, and no row claims public behavior that is only proven through a private invariant — `DECIDABLE_SEVERITIES` is never imported or asserted on by the tests, so replacing it with an inline literal would not break them.

## Review re-entry rows

None. `/code-review` was not part of this task, so no findings were routed and no review-reentry rows exist; the ledger comparison for review re-entry is vacuously empty on both sides.

## Deliberately not covered

A separate case-sensitivity test (e.g. `"Low "` alone) and an empty-input test would both have been **green on arrival** against the row 3 implementation. Adding them would have been horizontal slicing — tests for imagined behavior with no red — and each extra test would have needed an evidence row it could not honestly claim. Exact-match severity is instead pinned inside row 3, where it genuinely earned its red.
