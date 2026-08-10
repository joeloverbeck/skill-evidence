# TDD record — `triage_service.triage`

## Seams under test, and their authority

| Seam | Authority |
| --- | --- |
| `triage_service.triage(records)`, imported as `from triage_service import triage` — the only public entry point; the returned dict is the only observation channel | `TASK.md` §"Requirements for `triage(records)`" names the function and fixes its result. The seam is ratified by that spec, so no seam question was put to the user. |

No seam other than `triage` is exercised. `DECIDABLE_SEVERITIES` is an implementation
detail of the module and is deliberately not asserted against by any test — behavior is
observed only through the value `triage` returns. No mocks are used: there is no system
boundary here (no I/O, clock, or randomness), so per `method/mocking.md` there is nothing
to mock.

There is no `CONTEXT.md` and no ADR directory in this workspace, so test names follow the
vocabulary of `TASK.md` itself: *record*, *severity*, *decided*, *processed*, *untestable*.

## Command keys

Every command position below replays on its own via one of these two keys. Both are
absolute, so neither depends on the working directory.

- `FOCUS(<selector>)` = `python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-B-run2/test_triage_service.py::<selector> -q`
- `SUITE` = `python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-B-run2/test_triage_service.py -q`

`SUITE` is the form `TASK.md` prescribes (`python3 -m pytest test_triage_service.py -q`),
with the path made absolute.

## TDD evidence

### Row 1 — the result always reports exactly `processed` and `untestable`

- **Seam authority**: `TASK.md` requirement 1 (a dict with exactly two keys).
- **Test file / selector**: `test_triage_service.py` :: `test_triage_reports_exactly_processed_and_untestable`
- **Observed public entry point**: `triage([])`; the whole returned dict is asserted, so the
  key set is pinned exactly.
- **Red command**: `FOCUS(test_triage_reports_exactly_processed_and_untestable)`
- **Intended red**: no `triage` exists yet, so the import fails.
- **Observed red**: collection `ImportError: cannot import name 'triage' from 'triage_service'`
  → `1 error in 0.12s`. Matches the intended failure.
- **Minimal implementation**: `triage` returns the literal `{"processed": [], "untestable": []}`.
- **Green command**: `FOCUS(test_triage_reports_exactly_processed_and_untestable)`
- **Green result**: `1 passed in 0.12s`.

### Row 2 — decided records land in `processed`, in the order given

- **Seam authority**: `TASK.md` requirements 1 (order the records were given) and 2
  (`low` / `medium` / `high` are decided).
- **Test file / selector**: `test_triage_service.py` :: `test_decided_records_are_processed_in_the_order_given`
- **Observed public entry point**: `triage([r3:high, r1:low, r2:medium])`.
- **Red command**: `FOCUS(test_decided_records_are_processed_in_the_order_given)`
- **Intended red**: the constant from row 1 reports no ids, so `processed` is empty where
  `["r3", "r1", "r2"]` is required.
- **Observed red**: `AssertionError` — `{'processed': []} != {'processed': ['r3', 'r1', 'r2']}`
  → `1 failed in 0.12s`. Matches the intended failure.
- **Minimal implementation**: `processed` becomes `[record["id"] for record in records]`;
  `untestable` still the empty literal, since no test yet demands otherwise.
- **Green command**: `FOCUS(test_decided_records_are_processed_in_the_order_given)`
- **Green result**: `1 passed in 0.12s`.

### Row 3 — an undecidable record is recorded and processing carries on

- **Seam authority**: `TASK.md` requirement 3 (any other severity goes to `untestable`, and
  the remaining records are processed as normal).
- **Test file / selector**: `test_triage_service.py` :: `test_undecidable_record_is_reported_and_the_rest_still_processed`
- **Observed public entry point**: `triage([a:low, b:critical, c:high])`.
- **Red command**: `FOCUS(test_undecidable_record_is_reported_and_the_rest_still_processed)`
- **Intended red**: row 2's implementation treats every record as decided, so `b` is wrongly
  in `processed` and `untestable` is empty.
- **Observed red**: `AssertionError` — `{'processed': ['a', 'b', 'c']} != {'processed': ['a', 'c']}`
  and `{'untestable': []} != {'untestable': ['b']}` → `1 failed in 0.13s`. Matches the
  intended failure on both keys.
- **Minimal implementation**: classify each record by membership in `DECIDABLE_SEVERITIES`,
  appending to `processed` or `untestable`, with no early exit.
- **Green command**: `FOCUS(test_undecidable_record_is_reported_and_the_rest_still_processed)`
- **Green result**: `1 passed in 0.12s`.

Full suite after row 3: `SUITE` → `3 passed in 0.12s`.

## Are the goldens discriminating?

Every expected value is a literal read off `TASK.md`, never recomputed the way the code
computes it, so no assertion passes by construction. Each golden was then checked against
the implementations it is meant to reject, by running `SUITE` against a substituted module
in a scratch copy (since deleted). A golden that still passed under the forbidden behavior
would not be discriminating.

| Substituted behavior | Test that must catch it | Caught? |
| --- | --- | --- |
| `processed` sorted by id rather than input order | row 2 | yes — `1 failed, 2 passed` |
| stop processing at the first undecidable record | row 3 | yes — `1 failed, 2 passed` |
| unknown severity treated as decided | row 3 | yes — `1 failed, 2 passed` |
| result carries a third key alongside the two | row 1 | yes — `3 failed` (row 1 is the one that pins the key set) |

Row 2's input order `r3, r1, r2` is chosen so that the required answer differs from a sort
by id (`r1, r2, r3`) and from a sort by severity rank in either direction (`r1, r2, r3` or
`r3, r2, r1`). Row 3's undecidable record sits in the middle, so a short-circuiting
implementation loses `c` and is caught.

## Reconciliation of tests to seams

Inventory derivation: `git` is unavailable to me for this task, so the changed-test
inventory is derived from pytest's own collection rather than from a diff. This is sound
here because both `triage_service.py` and `test_triage_service.py` were empty at the start
of the task — every collected test is therefore a changed (new) test, and collection and
diff scope coincide.

Inventory command:
`python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-B-run2/test_triage_service.py --collect-only -q`
→ 3 tests collected.

| # | Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `test_triage_service.py` | `test_triage_reports_exactly_processed_and_untestable` | `triage([])` | `TASK.md` req 1 | Row 1 |
| 2 | `test_triage_service.py` | `test_decided_records_are_processed_in_the_order_given` | `triage([...])` | `TASK.md` reqs 1, 2 | Row 2 |
| 3 | `test_triage_service.py` | `test_undecidable_record_is_reported_and_the_rest_still_processed` | `triage([...])` | `TASK.md` req 3 | Row 3 |

Bidirectional comparison of the collected inventory against the reconciliation rows:

- in inventory, missing from reconciliation: none
- in reconciliation, missing from inventory: none

Both differences are empty. No test enters through a module-private name, helper, mutable
global, or side channel; all three enter through `triage`, the seam `TASK.md` ratifies, so
no seam needed ratifying after the fact and no test needed rewriting. Every requirement in
`TASK.md` is claimed by a public-behavior row above — none rests on a private-invariant
test.

## Not applicable

`/code-review` was not run as part of this task, so there are no review-reentry rows and no
`Findings fixed during review` ledger to compare against. Should a review follow, each
actionable finding marked `TDD re-entry required: yes` gets its own appended row keyed by
review pass, axis, and finding — rows 1–3 above must not be rewritten to absorb it.
