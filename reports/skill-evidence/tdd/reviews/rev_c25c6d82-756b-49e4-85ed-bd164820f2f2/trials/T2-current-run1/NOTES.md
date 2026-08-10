# TDD evidence — `triage(records)`

## Seam under test and its authority

| Seam | Authority | Status |
| --- | --- | --- |
| `triage(records)`, imported as `from triage_service import triage` | `TASK.md` "Requirements for `triage(records)`" items 1-3 — the task specification names this function, its argument, and its return shape | Ratified by the specification; no ambiguity, so no seam question was raised |

There is no second exported ingress path to this invariant: `triage_service` exports exactly one
callable, so the public-invariant coverage check ("enumerate every exported entry point that can
create or admit the value") is satisfied by testing `triage` alone. `DECIDABLE_SEVERITIES` is
module state, not an ingress path, and no test reads or writes it — the decidable-severity set is
observed only through `triage`'s return value.

No mocks are used. The module has no system boundary (no I/O, clock, randomness, or network), so
per `method/mocking.md` there is nothing legitimate to mock, and mocking anything here would be
mocking code we own.

`CONTEXT.md` does not exist in this workspace, so no project domain vocabulary or ADR constrained
the naming; test names use the specification's own words (*processed*, *untestable*, *decided*).

## Command key

`CMD-ROOT` — all commands below run with the working directory set to the workspace root (the
directory holding `triage_service.py`). Every command is written out in full in its row; none is
abbreviated to a prose label.

## Evidence rows

### Row 1 — a decided record's id goes in `processed`

- Seam authority: `TASK.md` requirements 1 and 2.
- Test file and selector: `test_triage_service.py::test_decided_record_id_goes_in_processed`.
- Observed public entry point: `triage([{"id": "r-1", "severity": "high"}])` return value.
- Red command: `python3 -m pytest test_triage_service.py::test_decided_record_id_goes_in_processed -q`
- Intended red: no `triage` exists, so the test cannot reach the seam.
- Observed red: `ImportError: cannot import name 'triage' from 'triage_service'` — 1 error.
- Minimal implementation: return the first record's id in `processed` and an empty `untestable`.
- Green command: `python3 -m pytest test_triage_service.py::test_decided_record_id_goes_in_processed -q`
- Green result: `1 passed`.

### Row 2 — ids keep the order the records were given

- Seam authority: `TASK.md` requirement 1 ("in the order the records were given").
- Test file and selector: `test_triage_service.py::test_processed_ids_keep_the_order_the_records_were_given`.
- Observed public entry point: `triage([...])` return value for three `"high"` records with ids
  `z-1`, `a-9`, `m-3`.
- Red command: `python3 -m pytest test_triage_service.py::test_processed_ids_keep_the_order_the_records_were_given -q`
- Intended red: the Row 1 implementation returns only the first record's id.
- Observed red: `{'processed': ['z-1']} != {'processed': ['z-1', 'a-9', 'm-3']}` — 1 failed.
- Minimal implementation: build `processed` from every record, preserving iteration order.
- Green command: `python3 -m pytest test_triage_service.py::test_processed_ids_keep_the_order_the_records_were_given -q`
- Green result: `1 passed`.

### Row 3 — an undecidable severity is recorded and processing carries on

- Seam authority: `TASK.md` requirement 3.
- Test file and selector: `test_triage_service.py::test_undecidable_severity_is_untestable_and_later_records_still_processed`.
- Observed public entry point: `triage([...])` return value for `z-1` (`"catastrophic"`), `r-2`
  (`"high"`), `a-1` (`""`).
- Red command: `python3 -m pytest test_triage_service.py::test_undecidable_severity_is_untestable_and_later_records_still_processed -q`
- Intended red: every id still lands in `processed`; nothing routes to `untestable`.
- Observed red: `{'processed': ['z-1', 'r-2', 'a-1']} != {'processed': ['r-2']}` and
  `{'untestable': []} != {'untestable': ['z-1', 'a-1']}` — 1 failed.
- Minimal implementation: route each id by membership in `DECIDABLE_SEVERITIES`, at this point
  `frozenset({"high"})` — the only severity any test had demanded be decided.
- Green command: `python3 -m pytest test_triage_service.py::test_undecidable_severity_is_untestable_and_later_records_still_processed -q`
- Green result: `1 passed`.

### Row 4 — `"low"` and `"medium"` are decided severities too

- Seam authority: `TASK.md` requirement 2 (the decided set is `"low"`, `"medium"`, `"high"`).
- Test file and selector: `test_triage_service.py::test_low_and_medium_are_decided_severities_too`.
- Observed public entry point: `triage([...])` return value for `z-1` (`"low"`), `a-9`
  (`"medium"`).
- Red command: `python3 -m pytest test_triage_service.py::test_low_and_medium_are_decided_severities_too -q`
- Intended red: with only `"high"` decidable, both ids are misrouted to `untestable`.
- Observed red: `{'untestable': ['z-1', 'a-9']} != {'untestable': []}` and
  `{'processed': []} != {'processed': ['z-1', 'a-9']}` — 1 failed.
- Minimal implementation: widen `DECIDABLE_SEVERITIES` to `frozenset({"low", "medium", "high"})`.
- Green command: `python3 -m pytest test_triage_service.py::test_low_and_medium_are_decided_severities_too -q`
- Green result: `1 passed`.

Full suite after Row 4: `python3 -m pytest test_triage_service.py -q` → `4 passed`.

Rows 3 and 4 are deliberate triangulation, not an omission: each slice implemented only what a
test had demanded, so widening the decidable set produced a genuine red in Row 4 instead of code
written ahead of a test.

## Discriminating goldens — sensitivity check

Every expected value in the suite is a hand-written literal taken from `TASK.md`, never recomputed
the way `triage` computes it, so no assertion can pass by construction. That is necessary but not
sufficient, so each plausible rejected implementation was substituted and the suite re-run. The
check is replayable: `python3 -m pytest test_triage_service.py -q` is run against each mutant by

`python3 .sensitivity/run_mutants.py`

(exit status 0 means every mutant was killed; the script copies the suite into a temporary
directory, mutates a copy of the source, and deletes the directory afterwards — it never edits
`triage_service.py`).

| Rejected implementation | Killed by |
| --- | --- |
| Both output lists sorted | `test_processed_ids_keep_the_order_the_records_were_given`, `test_undecidable_severity_is_untestable_and_later_records_still_processed`, `test_low_and_medium_are_decided_severities_too` |
| Undecidable records dropped instead of recorded | `test_undecidable_severity_is_untestable_and_later_records_still_processed` |
| Processing stops at the first undecidable record | `test_undecidable_severity_is_untestable_and_later_records_still_processed` |
| Every severity treated as decided | `test_undecidable_severity_is_untestable_and_later_records_still_processed` |
| A third key added to the result | all four tests |
| Only `"high"` decidable | `test_low_and_medium_are_decided_severities_too` |

Observed result: `survivors: none`. The ids `z-1`, `a-9`, `m-3`, `a-1` are chosen so that the
expected lists are not in sorted order, which is what makes the ordering requirement discriminating
rather than incidentally satisfied. Requirement 1's "exactly two keys" is pinned by asserting whole
returned dicts against literals — the extra-key mutant confirms an added key is observable.

## Pre-review reconciliation

`triage_service.py` and `test_triage_service.py` were both empty at the start of the task, so the
scoped change is every test now in the file. The changed-test inventory is derived from the
repository's native test structure — pytest collection — rather than from prose:

`python3 -m pytest test_triage_service.py --collect-only -q`

| # | Inventory (collected selector) | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| 1 | `test_triage_service.py::test_decided_record_id_goes_in_processed` | `triage_service.triage` return value | `TASK.md` req. 1-2 | Row 1 |
| 2 | `test_triage_service.py::test_processed_ids_keep_the_order_the_records_were_given` | `triage_service.triage` return value | `TASK.md` req. 1 | Row 2 |
| 3 | `test_triage_service.py::test_undecidable_severity_is_untestable_and_later_records_still_processed` | `triage_service.triage` return value | `TASK.md` req. 3 | Row 3 |
| 4 | `test_triage_service.py::test_low_and_medium_are_decided_severities_too` | `triage_service.triage` return value | `TASK.md` req. 2 | Row 4 |

Both-direction comparison: collected selectors not present in the reconciliation rows — none;
reconciliation rows not present in the collected selectors — none. Both differences are empty.
Each row is a selector, not a file- or class-level summary. No test enters through a module
internal, helper, mutable global, or side channel: all four call the ratified public function and
assert on its return value only, so no seam needed ratifying after the fact and no test needed
rewriting.

## Review re-entry rows

None. No `/code-review` pass was run for this task, so there are no `TDD re-entry required: yes`
findings to key rows against, and the `Findings fixed during review` comparison is vacuous in both
directions. No earlier row was rewritten or extended at any point.
