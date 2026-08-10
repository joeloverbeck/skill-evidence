# TDD record — `triage_service.triage`

Methodology: `method/SKILL.md` (+ `method/tests.md`, `method/mocking.md`).
Workspace root (`<WS>` below): `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T2-B-run3`

## Seams under test (written down before any test was written)

| Seam | Authority |
| --- | --- |
| `triage_service.triage(records)` — module-level function, imported as `from triage_service import triage`, observed only through its returned dict | `TASK.md` § "Requirements for `triage(records)`" (task specification supplied by the user), requirements 1–3 |

`TASK.md` names the function, its argument, its return shape and its three behaviors, so the seam is
ratified up front and no seam question needed to go back to the user. It is the only seam used; no
test reaches through a module internal, helper, mutable global or side channel.

Mocking: none. `triage` is pure and crosses no system boundary (`method/mocking.md` — mock only at
external APIs, DBs, time/randomness, filesystem), so mocking anything here would be mocking code we
own.

Not applicable — the "authoritative verifier already supplies red" branch of the loop rules:
`test_triage_service.py` started empty, so no pre-existing failing public verifier existed and every
cycle below had to author its own red.

## Command keys

Both keys are complete as written and replay from any working directory.

- `RUN <selector>` ≡ `python3 -m pytest <WS>/test_triage_service.py::<selector> -q`
- `SUITE` ≡ `python3 -m pytest <WS>/test_triage_service.py -q`

## TDD evidence rows

One row per behavior, one vertical slice each: test → run red → minimal implementation → run green.
No test was written ahead of its slice.

### Row 1 — result carries exactly the two documented keys

- **Seam authority**: `TASK.md` requirement 1 ("returns a dict with exactly two keys, `processed` and `untestable`")
- **Test file / selector**: `<WS>/test_triage_service.py` :: `test_returns_exactly_processed_and_untestable_keys`
- **Observed public entry point**: `triage([])` → returned dict's key set
- **Red command**: `RUN test_returns_exactly_processed_and_untestable_keys`
- **Intended failure**: no `triage` exists yet in the empty module, so the import fails
- **Observed failure**: `ImportError: cannot import name 'triage' from 'triage_service' (<WS>/triage_service.py)` — collection error, `1 error in 0.12s`. Matches the intended failure.
- **Minimal implementation**: `triage` returns `{"processed": [], "untestable": []}` — constant, no record handling yet
- **Green command**: `RUN test_returns_exactly_processed_and_untestable_keys`
- **Green result**: `1 passed in 0.12s`

### Row 2 — `low` / `medium` / `high` are processed, in the given order

- **Seam authority**: `TASK.md` requirements 1 and 2 (decided severities go to `processed`; ids appear "in the order the records were given")
- **Test file / selector**: `<WS>/test_triage_service.py` :: `test_low_medium_and_high_are_processed_in_given_order`
- **Observed public entry point**: `triage([{id r3, low}, {id r1, high}, {id r2, medium}])` → `result["processed"]`, `result["untestable"]`
- **Red command**: `RUN test_low_medium_and_high_are_processed_in_given_order`
- **Intended failure**: the constant stub returns an empty `processed`, so the expected id list cannot match
- **Observed failure**: `AssertionError: assert [] == ['r3', 'r1', 'r2']` at `test_triage_service.py:19` — `1 failed in 0.12s`. Matches the intended failure.
- **Minimal implementation**: iterate `records`, append `record["id"]` to `processed` when `record["severity"]` is in `DECIDED_SEVERITIES`; `untestable` still returned empty
- **Green command**: `RUN test_low_medium_and_high_are_processed_in_given_order`
- **Green result**: `1 passed in 0.12s`

### Row 3 — an undecidable severity becomes untestable and processing carries on

- **Seam authority**: `TASK.md` requirement 3 ("put its id in `untestable`, and carry on processing the remaining records as normal")
- **Test file / selector**: `<WS>/test_triage_service.py` :: `test_undecidable_severity_is_untestable_and_later_records_still_processed`
- **Observed public entry point**: `triage([{id r1, high}, {id r2, critical}, {id r3, low}, {id r4, HIGH}])` → `result["untestable"]`, `result["processed"]`
- **Red command**: `RUN test_undecidable_severity_is_untestable_and_later_records_still_processed`
- **Intended failure**: `untestable` is still hard-coded empty, so the two undecidable ids cannot appear
- **Observed failure**: `AssertionError: assert [] == ['r2', 'r4']` at `test_triage_service.py:33` — `1 failed in 0.12s`. Matches the intended failure.
- **Minimal implementation**: add the `else` branch appending `record["id"]` to `untestable`, and return the accumulated list
- **Green command**: `RUN test_undecidable_severity_is_untestable_and_later_records_still_processed`
- **Green result**: `1 passed in 0.12s`

Whole suite after row 3: `SUITE` → `3 passed in 0.12s`.

## Public invariant coverage

Exported names in `triage_service`: `triage` and the `DECIDED_SEVERITIES` constant. `triage` is the
only ingress path that admits a record or creates a result — there is no second parser, constructor,
factory or deserializer that could bypass it — so the single ratified seam covers every path to the
invariant. `DECIDED_SEVERITIES` is public and could be rebound by a caller to change the outcome;
no test touches it, because asserting through it would be a side channel rather than the ratified
seam. If that constant is ever meant to be a supported customization point, that is a new seam
needing its own authority, not something these tests should quietly pin.

## Discriminating goldens — sensitivity check

Expected values are hand-written literals taken from `TASK.md`, never recomputed the way the code
computes them, so no assertion passes by construction. Inputs were chosen to separate the required
result from plausible wrong implementations: record ids run `r3, r1, r2` (not alphabetical, so
sorting is visible), the undecidable record sits *between* two decided ones (so early exit and
silent dropping are visible), and `"HIGH"` is a deliberate near miss of `"high"`.

Replay: `python3 <WS>/.sensitivity/run_mutations.py` — it copies the tests against each rejected
implementation in a temp directory and leaves the real module untouched. Every mutation must fail
the suite. Observed:

| Rejected implementation | Result |
| --- | --- |
| sorted output (ignores given order) | CAUGHT |
| stops at first undecidable record | CAUGHT |
| drops undecidable records entirely | CAUGHT |
| case-insensitive severity match | CAUGHT |
| extra third key in result | CAUGHT |
| returns whole records instead of ids | CAUGHT |

No golden survives the behavior it is supposed to forbid, so none needs replacing.

## Pre-review reconciliation (tests ↔ seams)

**How the changed-test inventory was derived.** The scoped change is the whole of
`test_triage_service.py`: the file began empty, so every collected test is new in this change. Git is
forbidden in this workspace, so the inventory comes from pytest's own collection — the repository's
native test structure — rather than from a diff command:
`python3 -m pytest <WS>/test_triage_service.py --collect-only -q` → `3 tests collected`. Because the
prior file was empty, collection and a diff of the same file give the same identity set here.

Inventory (A):

1. `test_triage_service.py::test_returns_exactly_processed_and_untestable_keys`
2. `test_triage_service.py::test_low_medium_and_high_are_processed_in_given_order`
3. `test_triage_service.py::test_undecidable_severity_is_untestable_and_later_records_still_processed`

Reconciliation rows (B) — one per changed selector, not per file or class:

| Test file | Selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `<WS>/test_triage_service.py` | `test_returns_exactly_processed_and_untestable_keys` | `triage([])` → key set of returned dict | `TASK.md` req. 1 | Row 1 |
| `<WS>/test_triage_service.py` | `test_low_medium_and_high_are_processed_in_given_order` | `triage(records)` → `result["processed"]` / `result["untestable"]` | `TASK.md` req. 1–2 | Row 2 |
| `<WS>/test_triage_service.py` | `test_undecidable_severity_is_untestable_and_later_records_still_processed` | `triage(records)` → `result["untestable"]` / `result["processed"]` | `TASK.md` req. 3 | Row 3 |

**Both-direction comparison**: A \ B = empty; B \ A = empty. Gate passes.

**Coverage check**: every row above proves the public behavior it claims, through the ratified
`triage` seam only — no row substitutes a private-invariant test for a public-behavior proof, and no
row enters through a module internal, helper, mutable global or side channel. Requirements 1, 2 and
3 of `TASK.md` map onto rows 1, 2 and 3 respectively with none unclaimed.

## Review re-entry rows

None. `/code-review` has not been run on this change, so no finding is marked
`TDD re-entry required: yes` and no review-reentry row exists yet. If a review runs later, each
actionable finding gets its own appended row keyed by review pass, axis and finding — earlier rows
above must not be rewritten or extended to absorb it.
