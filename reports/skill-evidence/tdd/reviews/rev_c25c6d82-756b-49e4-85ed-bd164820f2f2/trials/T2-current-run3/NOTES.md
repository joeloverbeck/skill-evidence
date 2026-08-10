# TDD record — `triage(records)`

## Seams under test (agreed before the first test was written)

| Seam | Authority | Status |
| --- | --- | --- |
| `triage(records)` exported from `triage_service.py` | `TASK.md` "Requirements for `triage(records)`" — names the function, its argument, and its return contract | Ratified by the task document; unambiguous, so no user question needed |

`triage_service` has exactly two public names: `triage` and `DECIDABLE_SEVERITIES`.
Per the public-invariant coverage check, I enumerated the ingress paths that can admit a
record: `triage` is the only one. `DECIDABLE_SEVERITIES` is a module constant, not a
parser/constructor/deserializer, and it is a tuple, so it is not a mutable-global side
channel a caller could use to bypass classification. No test reaches through it.

No test double is used anywhere. Per `mocking.md` there is no system boundary here —
no I/O, clock, randomness, or external service — so mocking would only couple tests to
internals.

## Replayable command keys

Every command below runs from the workspace root
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-A-run3`.

- **`FOCUS <selector>`** = `python3 -m pytest test_triage_service.py::<selector> -q`
- **`SUITE`** = `python3 -m pytest test_triage_service.py -q`
- **`INVENTORY`** = `python3 -m pytest test_triage_service.py --collect-only -q`

## TDD evidence rows

One seam, one test, one minimal implementation per cycle; each red was observed before
the implementation that turned it green.

### Cycle 1 — the result reports exactly the two contract keys

- **Seam authority**: `TASK.md` requirement 1 ("returns a dict with exactly two keys").
- **Test**: `test_triage_service.py::test_triage_reports_exactly_processed_and_untestable`
- **Observed public entry point**: `triage([])`
- **Red**: `FOCUS test_triage_reports_exactly_processed_and_untestable`.
  Intended failure — no `triage` to import yet. Observed: collection error,
  `ImportError: cannot import name 'triage' from 'triage_service'`. Matches intent.
- **Green**: `FOCUS test_triage_reports_exactly_processed_and_untestable` → `1 passed`.
  Implementation: `triage` returns `{"processed": [], "untestable": []}` and nothing more.

### Cycle 2 — decided ids land in `processed`, in the order given

- **Seam authority**: `TASK.md` requirements 1 (source order) and 2 (`low`/`medium`/`high` are decided).
- **Test**: `test_triage_service.py::test_decided_records_are_processed_in_the_order_given`
- **Observed public entry point**: `triage([...])` returning the whole result dict.
- **Red**: `FOCUS test_decided_records_are_processed_in_the_order_given`.
  Intended failure — ids are dropped by the stub. Observed:
  `{'processed': []} != {'processed': ['T-3', 'T-1', 'T-2']}`. Matches intent.
- **Green**: `FOCUS test_decided_records_are_processed_in_the_order_given` → `1 passed`;
  `SUITE` → `2 passed`. Implementation: collect every id in iteration order.

### Cycle 3 — an unrecognized severity is undecidable

- **Seam authority**: `TASK.md` requirement 3 ("any other `severity` ... put its id in `untestable`").
- **Test**: `test_triage_service.py::test_record_with_unrecognized_severity_is_untestable`
- **Observed public entry point**: `triage([{"id": "T-9", "severity": "critical"}])`
- **Red**: `FOCUS test_record_with_unrecognized_severity_is_untestable`.
  Intended failure — cycle 2's code processes everything. Observed:
  `{'processed': ['T-9']} != {'processed': []}` and
  `{'untestable': []} != {'untestable': ['T-9']}`. Matches intent.
- **Green**: `FOCUS test_record_with_unrecognized_severity_is_untestable` → `1 passed`;
  `SUITE` → `3 passed`. Implementation: on an unrecognized severity, return immediately
  with that id in `untestable`. This test demands only the classification, so the
  short-circuit is the minimal code that passes it; requirement 3's "carry on" clause is
  deliberately not anticipated here and is driven out by cycle 4.

### Cycle 4 — processing carries on past an undecidable record

- **Seam authority**: `TASK.md` requirement 3 ("carry on processing the remaining records as normal") plus requirement 1 (source order in both lists).
- **Test**: `test_triage_service.py::test_later_records_are_still_triaged_after_an_undecidable_one`
- **Observed public entry point**: `triage([...])` over a five-record mixed sequence with
  two undecidable records interleaved between decided ones.
- **Red**: `FOCUS test_later_records_are_still_triaged_after_an_undecidable_one`.
  Intended failure — cycle 3's short-circuit abandons the tail. Observed:
  `{'processed': ['T-1']} != {'processed': ['T-1', 'T-3', 'T-5']}` and
  `{'untestable': ['T-2']} != {'untestable': ['T-2', 'T-4']}`. Matches intent.
- **Green**: `FOCUS test_later_records_are_still_triaged_after_an_undecidable_one` →
  `1 passed`; `SUITE` → `4 passed`. Implementation: classify each record into `processed`
  or `untestable` and always continue the loop.

### Cycle 5 — severity matching is exact, not case-insensitive

- **Seam authority**: `TASK.md` requirement 2 quotes the three literals and requirement 3
  routes "any other `severity`" to `untestable`; `"HIGH"` is a different string, so the
  task fixes the required result.
- **Test**: `test_triage_service.py::test_severity_matching_is_case_sensitive`
- **Observed public entry point**: `triage([{"id": "T-7", "severity": "HIGH"}])`
- **Red**: this cycle had no missing feature to supply red — exact matching fell out of
  cycle 3's `not in` check incidentally, never demanded by a test. Rather than fake a red
  or claim coverage I had not proved, I produced red by substituting the rejected
  implementation for real: `record["severity"].lower() in DECIDABLE_SEVERITIES`.
  Under that substitution, `FOCUS test_severity_matching_is_case_sensitive` observed
  `{'processed': ['T-7']} != {'processed': []}` /
  `{'untestable': []} != {'untestable': ['T-7']}` — the intended failure. `SUITE` under the
  same substitution reported `1 failed, 4 passed`, i.e. the new test was the only one that
  caught it.
- **Green**: reverted to `record["severity"] in DECIDABLE_SEVERITIES`;
  `FOCUS test_severity_matching_is_case_sensitive` → `1 passed`; `SUITE` → `5 passed`.

## Discriminating-golden check

Every expected value is an independent literal written from the requirements, never
recomputed the way the implementation computes it. I then checked each golden against the
plausible rejected implementations, since an independent literal alone is not sufficient:

| Rejected implementation | Caught by |
| --- | --- |
| Sorts `processed` by id instead of keeping source order | cycle 2 |
| Drops undecidable records instead of listing them | cycles 3 and 4 |
| Stops at the first undecidable record | cycle 4 |
| Matches severity case-insensitively | cycle 5 |

The last row is the reason cycle 5 exists. The first four tests all passed against the
case-insensitive implementation, so that golden set was not discriminating and had to be
extended before review rather than after.

## Pre-review reconciliation

Changed-test inventory, derived from the repository's native test structure via
`INVENTORY` (both files started empty, so every collected test is a changed test):

1. `test_triage_reports_exactly_processed_and_untestable`
2. `test_decided_records_are_processed_in_the_order_given`
3. `test_record_with_unrecognized_severity_is_untestable`
4. `test_later_records_are_still_triaged_after_an_undecidable_one`
5. `test_severity_matching_is_case_sensitive`

| Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `test_triage_service.py` | `test_triage_reports_exactly_processed_and_untestable` | `triage([])` | TASK.md req. 1 | Cycle 1 |
| `test_triage_service.py` | `test_decided_records_are_processed_in_the_order_given` | `triage(records)` | TASK.md req. 1, 2 | Cycle 2 |
| `test_triage_service.py` | `test_record_with_unrecognized_severity_is_untestable` | `triage(records)` | TASK.md req. 3 | Cycle 3 |
| `test_triage_service.py` | `test_later_records_are_still_triaged_after_an_undecidable_one` | `triage(records)` | TASK.md req. 1, 3 | Cycle 4 |
| `test_triage_service.py` | `test_severity_matching_is_case_sensitive` | `triage(records)` | TASK.md req. 2, 3 | Cycle 5 |

Two-way comparison: inventory minus reconciliation = empty; reconciliation minus
inventory = empty. Both differences empty, so the gate passes. Every row observes the
ratified public seam `triage(...)` and asserts on its returned value only — no private
invariant stands in for a public-behavior proof, and no test enters through a module
internal, helper, mutable global, or side channel.

Final state: `SUITE` → `5 passed`.

## Deliberately not covered

- **Malformed records** (missing `id`/`severity`, non-`str` severity). `TASK.md` states each
  record *is* a dict with a `str` id and a `str` severity, so behavior outside that contract
  is unspecified; `triage` would raise `KeyError`/`AttributeError`. Worth pinning only once an
  authority decides what it should do — I did not invent a requirement to test against.
- **Whitespace-padded severities** such as `" high"`. Same reasoning as case: strictly "any
  other severity", and the current exact match already routes them to `untestable`, but no
  authority calls it out and cycle 5 already pins exact-string matching as the rule.
