# TDD record — `triage_service.triage`

All commands below are complete and replayable, run from the workspace root
(`.../runs/T2-A-run4`), the directory holding `triage_service.py` and
`test_triage_service.py`. Python 3.14, pytest.

## Seams under test (written down before the first test)

| Seam | Authority |
| --- | --- |
| `triage_service.triage(records)` — module-level function, imported as `from triage_service import triage`, observed through its returned dict | `TASK.md` § "Requirements for `triage(records)`", requirements 1–3. The task spec is the explicit user decision that names both the function and its public result shape, so the seam is already agreed and no further ratification was needed. |

No other seam is used. Tests never import a private helper, never touch
`DECIDABLE_SEVERITIES`, and never assert through a side channel; every assertion
goes through the returned dict.

**Public-invariant ingress enumeration.** The invariants (key set, membership,
order) can only be created through one exported path: `triage`. The module
exports no other parser, constructor, factory, or deserializer that can build or
admit a triage result, so there is no bypass path to probe. There are no refusal
paths in this spec — an undecidable severity is reported in `untestable`, not
raised — so the non-observation-of-payload check does not apply.

## TDD evidence rows

### Row 1 — decided records reach `processed` in the order given (req 1, 2)

- Seam authority: `TASK.md` reqs 1–2 (seam table above).
- Test file / selector: `test_triage_service.py::test_decided_records_are_processed_in_the_order_given`
- Observed public entry point: `triage(records)["processed"]`
- Red command: `python3 -m pytest test_triage_service.py::test_decided_records_are_processed_in_the_order_given -q`
  - Intended failure: no `triage` exists yet, so the import at the seam fails.
  - Observed failure: collection error — `ImportError: cannot import name 'triage' from 'triage_service'`; `1 error in 0.01s`. Matches the intended failure.
- Minimal implementation: `triage` returns `{"processed": [record["id"] for record in records]}`.
- Green command: `python3 -m pytest test_triage_service.py::test_decided_records_are_processed_in_the_order_given -q` → `1 passed in 0.00s`.

### Row 2 — the result has exactly the two keys (req 1)

- Seam authority: `TASK.md` req 1 ("returns a dict with exactly two keys").
- Test file / selector: `test_triage_service.py::test_result_has_exactly_the_processed_and_untestable_keys`
- Observed public entry point: `sorted(triage(records).keys())`
- Red command: `python3 -m pytest test_triage_service.py::test_result_has_exactly_the_processed_and_untestable_keys -q`
  - Intended failure: the row-1 result carries only `processed`, so the key set is short one entry.
  - Observed failure: `AssertionError: assert ['processed'] == ['processed', 'untestable']` — "Right contains one more item: 'untestable'"; `1 failed in 0.01s`. Matches.
- Minimal implementation: add `"untestable": []` to the returned dict.
- Green command: `python3 -m pytest test_triage_service.py::test_result_has_exactly_the_processed_and_untestable_keys -q` → `1 passed in 0.00s`.

### Row 3 — an undecidable severity is listed in `untestable` (req 3, first half)

- Seam authority: `TASK.md` req 3 ("any other `severity` … put its id in `untestable`").
- Test file / selector: `test_triage_service.py::test_record_with_an_undecidable_severity_is_listed_untestable`
- Observed public entry point: `triage(records)["untestable"]`
- Red command: `python3 -m pytest test_triage_service.py::test_record_with_an_undecidable_severity_is_listed_untestable -q`
  - Intended failure: `untestable` is still the hardcoded empty list from row 2.
  - Observed failure: `AssertionError: assert [] == ['r-9']` — "Right contains one more item: 'r-9'"; `1 failed in 0.01s`. Matches.
- Minimal implementation: introduce `DECIDABLE_SEVERITIES = ("low", "medium", "high")` and populate `untestable` with the ids whose severity is outside it. `processed` deliberately left untouched — only enough code to pass this test.
- Green command: `python3 -m pytest test_triage_service.py::test_record_with_an_undecidable_severity_is_listed_untestable -q` → `1 passed in 0.00s`.

### Row 4 — processing carries on past an undecidable record (req 3, second half)

- Seam authority: `TASK.md` req 3 ("carry on processing the remaining records as normal").
- Test file / selector: `test_triage_service.py::test_processing_carries_on_past_an_undecidable_record`
- Observed public entry point: `triage(records)["processed"]`
- Red command: `python3 -m pytest test_triage_service.py::test_processing_carries_on_past_an_undecidable_record -q`
  - Intended failure: after row 3 the undecidable id is in *both* lists, because `processed` still takes every record.
  - Observed failure: `AssertionError: assert ['r-1', 'r-2'] == ['r-2']` — "At index 0 diff: 'r-1' != 'r-2'"; `1 failed in 0.01s`. Matches.
- Minimal implementation: filter `processed` to the decidable severities as well.
- Green command: `python3 -m pytest test_triage_service.py::test_processing_carries_on_past_an_undecidable_record -q` → `1 passed in 0.00s`.
- Full suite after the last slice: `python3 -m pytest test_triage_service.py -q` → `4 passed in 0.00s`.

Each row is one vertical slice: one seam, one selector, one minimal
implementation, run in the order above. No test was written ahead of its
implementation, and no implementation anticipated a later row.

## Discriminating goldens

Every expected value is an independent literal taken from `TASK.md`, not
recomputed the way the implementation computes it.

| Golden | Authority | Rejected alternative it discriminates against |
| --- | --- | --- |
| `processed == ["r-3", "r-1", "r-2"]` from input ordered high, low, medium | req 1 "in the order the records were given" | sorting by id, and ranking by severity — both of those produce `["r-1", "r-2", "r-3"]`, so the ids were chosen to disagree with the input order |
| key set `["processed", "untestable"]` | req 1 "exactly two keys" | a result carrying only the populated key |
| `untestable == ["r-9"]` for `severity="critical"` | req 3 "any other `severity`" | treating `critical` as a decidable (top-of-scale) severity, the most plausible misreading of "low/medium/high" |
| `processed == ["r-2"]` for input `[critical, low]` | req 3 "carry on processing the remaining records" | stopping or raising at the first undecidable record (`[]`), and letting undecidable ids fall into `processed` too (`["r-1", "r-2"]`) |

**Sensitivity check.** Each rejected implementation above was substituted for
the real one in a throwaway copy and the suite re-run (probe directory deleted
afterwards; it is not part of the deliverable). No golden survived its own
rejected alternative:

| Substituted implementation | Tests that failed |
| --- | --- |
| ids sorted instead of kept in input order | `test_decided_records_are_processed_in_the_order_given` |
| `"critical"` added to the decidable severities | `test_record_with_an_undecidable_severity_is_listed_untestable`, `test_processing_carries_on_past_an_undecidable_record` |
| loop breaks at the first undecidable record | `test_processing_carries_on_past_an_undecidable_record` |
| `untestable` key omitted from the result | `test_result_has_exactly_the_processed_and_untestable_keys`, `test_record_with_an_undecidable_severity_is_listed_untestable` |

Every mutant is killed, and every test kills at least one mutant, so no test is
insensitive dead weight.

## Pre-review reconciliation (tests → seams)

Changed-test inventory derived from the repository's native test structure —
pytest collection, not a hand-written list:
`python3 -m pytest test_triage_service.py --collect-only -q` → `4 tests collected`.
Both `triage_service.py` and `test_triage_service.py` started empty, so every
collected selector is a changed test.

| # | Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `test_triage_service.py` | `test_decided_records_are_processed_in_the_order_given` | `triage(records)["processed"]` | `TASK.md` reqs 1–2 | Row 1 |
| 2 | `test_triage_service.py` | `test_result_has_exactly_the_processed_and_untestable_keys` | `sorted(triage(records).keys())` | `TASK.md` req 1 | Row 2 |
| 3 | `test_triage_service.py` | `test_record_with_an_undecidable_severity_is_listed_untestable` | `triage(records)["untestable"]` | `TASK.md` req 3 | Row 3 |
| 4 | `test_triage_service.py` | `test_processing_carries_on_past_an_undecidable_record` | `triage(records)["processed"]` | `TASK.md` req 3 | Row 4 |

Two-direction comparison of the collected inventory against the reconciliation
rows:

- in the inventory but not reconciled: **none**
- reconciled but not in the inventory: **none**

Both differences are empty. Every row enters through `triage`'s return value,
the ratified public boundary; none enters through a module internal, helper,
mutable global, or side channel, so no seam needed re-ratifying and no test
needed rewriting. No row claims public behavior via a private invariant.

## Notes and deliberate omissions

- No mock is used anywhere. `triage` is a pure function over its argument with
  no system boundary — no clock, no I/O, no external service — so there is
  nothing that mocking guidance would authorize stubbing.
- Cases deliberately **not** added: empty input, several undecidable records in
  a row, and mixed inputs longer than the ones above. Each of them passes
  against the row-4 implementation without a genuine red, so writing them would
  be bulk verification of imagined behavior rather than a slice. The order
  invariant for `untestable` is a `TASK.md` req-1 clause with no failing case
  available at this point; it is flagged here rather than covered by a test that
  could never have gone red.
- `/code-review` was not part of this task, so there are no review re-entry
  rows. If review later returns findings marked `TDD re-entry required: yes`,
  each one gets a new appended row keyed by review pass, axis, and finding —
  the four rows above must not be rewritten to absorb it.
