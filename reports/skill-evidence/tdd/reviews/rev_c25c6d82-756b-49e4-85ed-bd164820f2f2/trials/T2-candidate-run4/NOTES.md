# TDD record — `triage_service.triage`

## Seams under test

| Seam | Authority | Ratified? |
| --- | --- | --- |
| `triage_service.triage(records)`, entered as `from triage_service import triage` | `TASK.md` → "Requirements for `triage(records)`", items 1–3 | Yes — the task statement is the explicit user decision that names this function as the public interface. No PRD, issue or ADR exists in this workspace; nothing about the seam was ambiguous, so no clarifying question was raised. |

No other seam is tested. `DECIDED_SEVERITIES` is an implementation detail and is never imported by the tests.

**Mocking:** none. `triage` is a pure function over in-memory dicts — no external API, database, clock, randomness or filesystem — so there is no system boundary to mock, and per `method/mocking.md` no internal collaborator may be mocked.

## Command keys

Both keys are defined here and referenced below with their argument. Workspace root
`W = /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-B-run4`.

- `FOCUS <selector>` = `python3 -m pytest W/test_triage_service.py::<selector> -q`
- `SUITE` = `python3 -m pytest W/test_triage_service.py -q`

(Substitute the literal path for `W`; each command then runs standalone from any directory.)

## TDD evidence rows

### Row 1 — the result shape is exactly `processed` + `untestable`

- **Seam authority:** `TASK.md` requirement 1 ("returns a dict with exactly two keys, `processed` and `untestable`, each a list of record ids").
- **Test file / selector:** `test_triage_service.py::test_no_records_gives_empty_processed_and_untestable_lists`
- **Observed public entry point:** `triage([])` via `from triage_service import triage`.
- **Red command:** `FOCUS test_no_records_gives_empty_processed_and_untestable_lists`
- **Intended failure:** no public entry point exists yet — import of `triage` fails.
- **Observed failure:** `ImportError: cannot import name 'triage' from 'triage_service' (W/triage_service.py)` → `1 error in 0.12s`. Matches the intended failure.
- **Green command:** `FOCUS test_no_records_gives_empty_processed_and_untestable_lists`
- **Minimal implementation:** `def triage(records): return {"processed": [], "untestable": []}`
- **Green result:** `1 passed in 0.12s`.

### Row 2 — decided severities land in `processed`, in input order

- **Seam authority:** `TASK.md` requirements 1 (order) and 2 (`"low"`, `"medium"`, `"high"` are decided).
- **Test file / selector:** `test_triage_service.py::test_low_medium_and_high_records_are_processed_in_the_order_given`
- **Observed public entry point:** `triage([{ "id": "r3", "severity": "high" }, {"id": "r1", ...}, {"id": "r2", ...}])` via `from triage_service import triage`.
- **Red command:** `FOCUS test_low_medium_and_high_records_are_processed_in_the_order_given`
- **Intended failure:** `processed` comes back empty instead of holding the three ids in the given order.
- **Observed failure:** `AssertionError` with `Differing items: {'processed': []} != {'processed': ['r3', 'r1', 'r2']}` → `1 failed in 0.12s`. Matches the intended failure.
- **Green command:** `FOCUS test_low_medium_and_high_records_are_processed_in_the_order_given`
- **Minimal implementation:** `processed` becomes `[record["id"] for record in records]`; `untestable` still hard-coded empty.
- **Green result:** `1 passed in 0.12s`.

### Row 3 — undecidable severities go to `untestable`, and later records still process

- **Seam authority:** `TASK.md` requirement 3 ("put its id in `untestable`, and carry on processing the remaining records as normal") plus requirement 1's ordering clause for the `untestable` list.
- **Test file / selector:** `test_triage_service.py::test_undecidable_severities_are_untestable_and_later_records_still_process`
- **Observed public entry point:** `triage([...])` over four records (`r5` high, `r9` critical, `r1` low, `r2` urgent) via `from triage_service import triage`.
- **Red command:** `FOCUS test_undecidable_severities_are_untestable_and_later_records_still_process`
- **Intended failure:** undecidable ids are wrongly counted as processed and `untestable` stays empty.
- **Observed failure:** `AssertionError` with `Differing items: {'processed': ['r5', 'r9', 'r1', 'r2']} != {'processed': ['r5', 'r1']}` and `{'untestable': []} != {'untestable': ['r9', 'r2']}` → `1 failed in 0.12s`. Matches the intended failure.
- **Green command:** `FOCUS test_undecidable_severities_are_untestable_and_later_records_still_process`
- **Minimal implementation:** loop over `records`, appending each id to `processed` or `untestable` according to `record["severity"] in DECIDED_SEVERITIES`.
- **Green result:** `1 passed in 0.12s`.

### Whole suite after the last cycle

- **Command:** `SUITE`
- **Result:** `3 passed in 0.00s`.

## Discriminating-golden check

Every expected value is an independent literal written from the requirement text, never recomputed the
way the code computes it. Each golden was then probed against the plausible rejected implementations;
a golden is only worth keeping if some kept test fails when the wrong behaviour is substituted.

| Rejected implementation | Row 2 golden | Row 3 golden |
| --- | --- | --- |
| sorts ids instead of preserving input order | catches it (`['r1','r2','r3']`) | catches it (`['r1','r5']` / `['r2','r9']`) |
| stops at the first undecidable record | passes anyway (input has no undecidable record) | catches it (`processed == ['r5']`) |
| drops undecidable records instead of listing them | passes anyway (same reason) | catches it (`untestable == []`) |
| groups by severity rank instead of input order | catches it (`['r3','r2','r1']`) | passes anyway (its order coincides) |

Each rejected implementation is caught by at least one kept test, so no golden is redundant and none is
insensitive: row 2's ids are deliberately out of sorted order (`r3, r1, r2`), and row 3 carries two
undecidable records with ids (`r9`, `r2`) whose input order differs from their sorted order.

## Pre-review reconciliation (tests → seams)

Git is prohibited in this environment, so the changed-test inventory is derived from the repository's
native test structure — pytest collection — rather than from a `git diff`. Both source files were empty
at task start, so the scoped change is the whole of `test_triage_service.py`; collection therefore
enumerates exactly the changed selectors.

Inventory command: `python3 -m pytest W/test_triage_service.py --collect-only -q` → 3 tests collected.

| # | Test file | Exact selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- | --- |
| 1 | `test_triage_service.py` | `test_no_records_gives_empty_processed_and_untestable_lists` | `triage([])` | `TASK.md` req. 1 | Row 1 |
| 2 | `test_triage_service.py` | `test_low_medium_and_high_records_are_processed_in_the_order_given` | `triage(records)` | `TASK.md` req. 1, 2 | Row 2 |
| 3 | `test_triage_service.py` | `test_undecidable_severities_are_untestable_and_later_records_still_process` | `triage(records)` | `TASK.md` req. 1, 3 | Row 3 |

**Bidirectional comparison**

- Inventory identities: {`test_no_records_gives_empty_processed_and_untestable_lists`, `test_low_medium_and_high_records_are_processed_in_the_order_given`, `test_undecidable_severities_are_untestable_and_later_records_still_process`}
- Reconciliation identities: the same three selectors.
- inventory − reconciliation = ∅. reconciliation − inventory = ∅. Both differences empty; gate passes.

**Coverage check.** Every row proves public behaviour through `triage`'s return value; no row substitutes a
private invariant, and no test reaches the behaviour through a module internal, helper, mutable global or
side channel. Nothing needed re-ratifying or rewriting.

## Review re-entry rows

None. `/code-review` was not run as part of this task, so there are no findings marked
`TDD re-entry required: yes`, no appended review-reentry rows, and no
`Findings fixed during review` ledger to compare against. If a review happens later, each actionable
finding gets its own new row keyed by review pass / axis / finding — never an edit to rows 1–3 above.

## Deliberately not tested

These would have passed the moment they were written (no red available), so per "red before green" they
were not added as speculative tests:

- Case sensitivity of severity (`"HIGH"` → `untestable`). Requirement 2 lists lowercase literals and
  requirement 3 makes everything else undecidable, so the implemented exact-match membership already
  follows from the two rows above.
- Records missing an `id` or `severity` key. `TASK.md` states every record has both; no behaviour is
  specified for the malformed case, so none is invented or pinned.
