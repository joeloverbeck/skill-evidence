# TDD record — `triage(records)`

## Seams under test, and their authority

| Seam | Authority | Status |
| --- | --- | --- |
| `triage(records)`, imported from the `triage_service` module | `TASK.md` — names the function, the module file, and the two-key return shape | Ratified up front; no seam was invented during the loop |

`TASK.md` is the sole authority in this workspace: there is no `CONTEXT.md` and no `docs/adr/`,
so no domain vocabulary or ADR constrains the naming beyond the task's own terms
(`triage`, `processed`, `untestable`, "decided", "record id", "severity").

All three tests enter through that one public seam. Nothing reaches into module internals:
`DECIDABLE_SEVERITIES` is an implementation detail and no test names it, so the set can be
re-expressed (tuple, regex, per-severity policy objects) without touching a test. No mocks are
used — there is no system boundary here to mock, only a pure function.

## Command keys

Commands are given relative to `$ROOT`, the workspace root:

```
ROOT=/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-A-run5
```

| Key | Complete runnable command |
| --- | --- |
| `K-FULL` | `python3 -m pytest $ROOT/test_triage_service.py -q` |
| `K-F1` | `python3 -m pytest $ROOT/test_triage_service.py::test_triage_of_no_records_reports_two_empty_buckets -q` |
| `K-F2` | `python3 -m pytest $ROOT/test_triage_service.py::test_decided_records_are_processed_in_the_order_given -q` |
| `K-F3` | `python3 -m pytest $ROOT/test_triage_service.py::test_undecidable_severities_are_set_aside_and_the_rest_still_process -q` |
| `K-COLLECT` | `python3 -m pytest $ROOT/test_triage_service.py -q --collect-only` |
| `K-MUTANTS` | `python3 $ROOT/.sensitivity/run_mutants.py` |

## TDD evidence

One row per behavior, one vertical slice per row, in the order the cycles were run.

### Row 1 — the result is exactly two buckets

- **Seam authority**: `TASK.md` requirement 1 (exactly two keys, `processed` and `untestable`)
- **Test file / selector**: `test_triage_service.py::test_triage_of_no_records_reports_two_empty_buckets`
- **Observed public entry point**: `triage([])` via `from triage_service import triage`
- **Red**: `K-F1`. Intended failure: no `triage` symbol at the seam. Observed:
  `ImportError: cannot import name 'triage' from 'triage_service'` — 1 error during collection.
- **Green**: `K-F1` after adding `triage` returning `{"processed": [], "untestable": []}` — `1 passed`.
- **Minimal implementation**: returned the two empty buckets outright; nothing observed a filter yet.

### Row 2 — decided records keep the caller's order

- **Seam authority**: `TASK.md` requirements 1 and 2 (ids "in the order the records were given";
  `low`/`medium`/`high` are decided)
- **Test file / selector**: `test_triage_service.py::test_decided_records_are_processed_in_the_order_given`
- **Observed public entry point**: `triage([...])` with three decided records, severities in the
  order high, low, medium
- **Red**: `K-F2`. Intended failure: `processed` empty instead of the three ids. Observed:
  `AssertionError` — `{'processed': []} != {'processed': ['r-1', 'r-2', 'r-3']}` — `1 failed`.
- **Green**: `K-FULL` after collecting each record's id in iteration order — `2 passed`.
- **Minimal implementation**: `[record["id"] for record in records]`, no severity filter — the
  filter was not yet observable, so it was left for row 3 to force.

### Row 3 — undecidable severities are set aside and processing carries on

- **Seam authority**: `TASK.md` requirement 3 ("any other severity" → `untestable`, "carry on
  processing the remaining records as normal")
- **Test file / selector**: `test_triage_service.py::test_undecidable_severities_are_set_aside_and_the_rest_still_process`
- **Observed public entry point**: `triage([...])` with undecidable records (`"critical"`, `"HIGH"`)
  sitting mid-list, followed by a further decided record
- **Red**: `K-F3`. Intended failure: all four ids in `processed`, `untestable` empty. Observed:
  `AssertionError` — `{'processed': ['r-1', 'r-2', 'r-3', 'r-4']} != {'processed': ['r-1', 'r-4']}`
  and `{'untestable': []} != {'untestable': ['r-2', 'r-3']}` — `1 failed`.
- **Green**: `K-FULL` after routing each id to a bucket by severity membership — `3 passed`.
- **Minimal implementation**: `DECIDABLE_SEVERITIES` membership decides the bucket; the loop
  never short-circuits, which is what "carry on" means observably.

## Discriminating goldens

Every expected value is an independent literal written from `TASK.md`, never recomputed the way
the code computes it. Inputs were chosen so the required result differs from each plausible
rejected implementation. `K-MUTANTS` substitutes each rejected implementation in a throwaway
sandbox and asserts the suite rejects it:

| Rejected implementation | Would produce | Caught by |
| --- | --- | --- |
| Sorts output by severity rank | `processed` `["r-2", "r-3", "r-1"]` | Row 2 |
| Stops at the first undecidable record | `processed` `["r-1"]` | Row 3 |
| Drops undecidable records silently | `untestable` `[]` | Row 3 |
| Case-folds severity before matching | `"HIGH"` lands in `processed` | Row 3 |
| Adds a third key to the result | extra `count` key | Rows 1–3 |
| Returns whole records instead of ids | dicts, not id strings | Rows 2–3 |

Last run of `K-MUTANTS`: all six caught, exit status 0. Had any escaped, the golden would not be
discriminating and would have to be replaced.

Row 2's input is deliberately given in an order (high, low, medium) that no severity ranking
reproduces; had the records been given in rank order, the golden would pass under a sorting
implementation and prove nothing about order preservation.

## Pre-review reconciliation

Changed-test inventory derived from the suite's native structure via `K-COLLECT` (both files
started empty, so every collected test is new; no git command was run).

| Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `test_triage_service.py` | `test_triage_of_no_records_reports_two_empty_buckets` | `triage([])` | `TASK.md` req. 1 | Row 1 |
| `test_triage_service.py` | `test_decided_records_are_processed_in_the_order_given` | `triage([...])` | `TASK.md` reqs. 1–2 | Row 2 |
| `test_triage_service.py` | `test_undecidable_severities_are_set_aside_and_the_rest_still_process` | `triage([...])` | `TASK.md` req. 3 | Row 3 |

Both-direction comparison:

- In the inventory but not reconciled: none.
- Reconciled but not in the inventory: none.

Both differences are empty. Every row enters through the ratified public seam — no module
internal, helper, mutable global, or side channel — so no seam needs further ratification.

## Coverage of the authority, and what is deliberately not tested

Each of the three numbered requirements in `TASK.md` has a row. Requirement 1 splits across rows
1 and 2: row 1 pins "exactly two keys" (whole-dict equality fails on an extra or renamed key) and
row 2 pins the ordering clause.

Not tested, and why:

- **Record shapes the authority does not define** — missing `id`, missing `severity`, non-string
  severity, duplicate ids, `None` as the record list. `TASK.md` states no required behavior for
  these, so any assertion would invent a contract rather than verify one. `triage` currently
  raises `KeyError` on a malformed record; that is unspecified fallout, not a ratified behavior,
  and a test asserting it would freeze an accident. This is the open question worth putting to
  the author before the module meets real input.
- **Input is not mutated** — plausible to assert, but `TASK.md` says nothing about it, so it is
  listed here as an unratified question rather than pinned by a test.

## Loop discipline

Three cycles, each red → green at one seam with one selector and one minimal implementation.
No test was written ahead of its implementation, and no cycle's test was rewritten to absorb a
later behavior. `.sensitivity/run_mutants.py` is a verification harness, not a deliverable test:
it is excluded from collection (hidden directory, no `test_` prefix) and asserts nothing about
`triage` beyond re-running the real suite.
