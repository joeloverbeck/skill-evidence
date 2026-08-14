# TDD Evidence

## Agreed public seams

The raw Routebook manifest-v2 task is the authority for every seam below. The package README
states that only the names in `routebook.__all__` are public, and all four named entry points are
exported there.

| Contract behaviors | Public seam | Authority |
| --- | --- | --- |
| 1-4 | `routebook.normalize_destination(city, country)` | Raw user task, manifest-v2 behaviors 1-4 |
| 5-8 | `routebook.group_routes(routes)` | Raw user task, manifest-v2 behaviors 5-8 |
| 9-10 | `routebook.manifest_stats(groups)` | Raw user task, manifest-v2 behaviors 9-10 |
| 11-12 | `routebook.render_manifest(groups)` | Raw user task, manifest-v2 behaviors 11-12 |

## Baseline

Command: `python3 -m unittest discover -s tests -v`

Result before changes: `Ran 4 tests ... OK`.

## Observed cycles

_Rows are appended only after their focused red and green have both been observed._

### Cycle N1 - trim surrounding whitespace

- Seam authority: raw task behavior 1; public `routebook.normalize_destination`.
- Test: `tests/test_normalization.py` — `NormalizeDestinationTests.test_trims_surrounding_whitespace`.
- Observed public entry point: `normalize_destination(city, country)`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_whitespace -v`.
- Intended failure: legacy values retain surrounding city and country whitespace.
- Observed failure: expected `('Madrid', 'ES')`, got `('  Madrid  ', '  ES  ')`; `FAILED (failures=1)`.
- Minimal production change: strip surrounding whitespace from both values.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_whitespace -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle N2 - collapse internal city whitespace

- Seam authority: raw task behavior 2; public `routebook.normalize_destination`.
- Test: `tests/test_normalization.py` — `NormalizeDestinationTests.test_collapses_internal_city_whitespace`.
- Observed public entry point: `normalize_destination(city, country)`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace -v`.
- Intended failure: stripping alone leaves the tab and repeated spaces within the city.
- Observed failure: expected `('New York', 'US')`, got `('New \t  York', 'US')`; `FAILED (failures=1)`.
- Minimal production change: split and rejoin city whitespace with one space.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle N3 - uppercase a two-letter country code

- Seam authority: raw task behavior 3; public `routebook.normalize_destination`.
- Test: `tests/test_normalization.py` — `NormalizeDestinationTests.test_uppercases_two_letter_country_code`.
- Observed public entry point: `normalize_destination(city, country)`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v`.
- Intended failure: the lower-case two-letter code remains lower-case.
- Observed failure: expected `('Madrid', 'ES')`, got `('Madrid', 'es')`; `FAILED (failures=1)`.
- Minimal production change: uppercase a stripped country value when its length is two.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle N4 - reject a blank city

- Seam authority: raw task behavior 4; public `routebook.normalize_destination`.
- Test: `tests/test_normalization.py` — `NormalizeDestinationTests.test_rejects_blank_city`.
- Observed public entry point: `normalize_destination(city, country)`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v`.
- Intended failure: a whitespace-only city normalizes to an empty value without refusal.
- Observed failure: `AssertionError: ValueError not raised`; `FAILED (failures=1)`.
- Minimal production change: reject an empty normalized city with `ValueError`.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle G1 - group by normalized country code

- Seam authority: raw task behavior 5; public `routebook.group_routes`.
- Test: `tests/test_grouping.py` — `GroupRoutesTests.test_groups_routes_by_normalized_country_code`.
- Observed public entry point: `group_routes(routes)`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_groups_routes_by_normalized_country_code -v`.
- Intended failure: legacy grouping keeps `es` and `ES` separate and leaves route destinations unnormalized.
- Observed failure: output had separate `es` and `ES` keys and retained `" Madrid "`; `FAILED (failures=1)`.
- Minimal production change: normalize each public route destination, update its copied values, and group by the normalized country.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_groups_routes_by_normalized_country_code -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle G2 - sort by normalized city then route ID

- Seam authority: raw task behavior 6; public `routebook.group_routes`.
- Test: `tests/test_grouping.py` — `GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id`.
- Observed public entry point: `group_routes(routes)`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v`.
- Intended failure: grouped routes retain input order rather than normalized-city/ID order.
- Observed failure: expected IDs `['b', 'c', 'a']`, got `['a', 'c', 'b']`; `FAILED (failures=1)`.
- Minimal production change: sort every completed group by `(route["city"], route["id"])` after normalization.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle G3 - reject duplicate route IDs

- Seam authority: raw task behavior 7; public `routebook.group_routes`.
- Test: `tests/test_grouping.py` — `GroupRoutesTests.test_rejects_duplicate_route_ids`.
- Observed public entry point: `group_routes(routes)`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids -v`.
- Intended failure: equal route IDs in different destinations are both accepted.
- Observed failure: `AssertionError: ValueError not raised`; `FAILED (failures=1)`.
- Minimal production change: track IDs across the input and raise `ValueError` on a repeated ID.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle S2 - alphabetical per-country counts

- Seam authority: raw task behavior 10; public `routebook.manifest_stats`.
- Test: `tests/test_stats.py` — `ManifestStatsTests.test_reports_country_counts_in_alphabetical_order`.
- Observed public entry point: `manifest_stats(groups)`.
- Red command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order -v`.
- Intended failure: total-only legacy statistics have no per-country counts.
- Observed failure: `KeyError: 'countries'`; `FAILED (errors=1)`.
- Minimal production change: build an insertion-ordered `countries` mapping by iterating sorted country keys.
- Green command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle R1 - stable country and route lines

- Seam authority: raw task behavior 11; public `routebook.render_manifest`.
- Test: `tests/test_rendering.py` — `RenderManifestTests.test_emits_stable_country_and_route_lines`.
- Observed public entry point: `render_manifest(groups)`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_stable_country_and_route_lines -v`.
- Intended failure: legacy rendering emits one debug-representation line in insertion order.
- Observed failure: one `repr` line differed from the six expected stable country/route lines; `FAILED (failures=1)`.
- Minimal production change: emit alphabetized `CC:` headers followed by routes sorted by city and ID as `  id: city`.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_stable_country_and_route_lines -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle R2 - one final newline without trailing horizontal whitespace

- Seam authority: raw task behavior 12; public `routebook.render_manifest`.
- Test: `tests/test_rendering.py` — `RenderManifestTests.test_ends_with_exactly_one_newline_and_no_trailing_horizontal_whitespace`.
- Observed public entry point: `render_manifest(groups)`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_with_exactly_one_newline_and_no_trailing_horizontal_whitespace -v`.
- Intended failure: the stable renderer has no final newline.
- Observed failure: `AssertionError: False is not true` for `rendered.endswith("\n")`; `FAILED (failures=1)`.
- Minimal production change: append one newline after joining lines; generated line templates already end in non-whitespace values.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_with_exactly_one_newline_and_no_trailing_horizontal_whitespace -v`.
- Green result: `Ran 1 test ... OK`.

### Cycle SR1 - self-review repair for direct rendering input

- Review identity: self-review, contract behavior 12, finding SR1.
- Finding: the first behavior-12 probe used already-normalized input, so a direct caller could supply a city ending in spaces or tabs and receive trailing horizontal whitespace.
- Seam authority: raw task behavior 12; public `routebook.render_manifest`.
- Test: `tests/test_rendering.py` — `RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_direct_input`.
- Observed public entry point: `render_manifest(groups)`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_direct_input -v`.
- Intended failure: a direct route city ending in space and tab leaves its rendered route line ending in horizontal whitespace.
- Observed failure: the per-line `rstrip(" \t")` invariant was false; `FAILED (failures=1)`.
- Minimal production repair: strip only trailing spaces and tabs from each completed route line.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_direct_input -v`.
- Green result: `Ran 1 test ... OK`.

## First-run passes (non-TDD)

### Grouping coverage G4 - preserve distinct IDs for the same city

- Seam authority: raw task behavior 8; public `routebook.group_routes`.
- Test: `tests/test_grouping.py` — `GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city`.
- Observed public entry point: `group_routes(routes)`.
- First-run command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city -v`.
- Result: `Ran 1 test ... OK` before any production change for this selector.
- Disposition: non-TDD first-run pass. The legacy collection behavior already preserved distinct records, and the preceding duplicate-ID implementation rejects only repeated IDs.

### Statistics coverage S1 - total route count

- Seam authority: raw task behavior 9; public `routebook.manifest_stats`.
- Test: `tests/test_stats.py` — `ManifestStatsTests.test_reports_total_route_count`.
- Observed public entry point: `manifest_stats(groups)`.
- First-run command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_total_route_count -v`.
- Result: `Ran 1 test ... OK` before any production change for this selector.
- Disposition: non-TDD first-run pass. The legacy implementation already summed the lengths of all country groups.

## Contract self-review

One self-review was performed after the first aggregate green run against all twelve numbered
behaviors. Behaviors 1-11 matched their public observations. Behavior 12 had one sensitivity gap:
the public rendering seam could receive a direct, non-normalized city value ending in horizontal
whitespace. Finding SR1 was repaired through the distinct red-green cycle above. A second pass over
the same checklist after that repair found no remaining mismatch; this was completion of the one
self-review, not a separate review pass.

## Final changed-test-to-seam reconciliation

The changed-selector inventory was derived at the native `unittest` method boundary. It contains
every selector added to the final test tree and the four legacy selectors removed from the baseline.

| Change | File and exact selector | Public entry point | Seam authority | Custody or disposition |
| --- | --- | --- | --- | --- |
| Added | `tests/test_normalization.py` — `NormalizeDestinationTests.test_trims_surrounding_whitespace` | `normalize_destination` | Raw task behavior 1 | TDD cycle N1 |
| Added | `tests/test_normalization.py` — `NormalizeDestinationTests.test_collapses_internal_city_whitespace` | `normalize_destination` | Raw task behavior 2 | TDD cycle N2 |
| Added | `tests/test_normalization.py` — `NormalizeDestinationTests.test_uppercases_two_letter_country_code` | `normalize_destination` | Raw task behavior 3 | TDD cycle N3 |
| Added | `tests/test_normalization.py` — `NormalizeDestinationTests.test_rejects_blank_city` | `normalize_destination` | Raw task behavior 4 | TDD cycle N4 |
| Added | `tests/test_grouping.py` — `GroupRoutesTests.test_groups_routes_by_normalized_country_code` | `group_routes` | Raw task behavior 5 | TDD cycle G1 |
| Added | `tests/test_grouping.py` — `GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id` | `group_routes` | Raw task behavior 6 | TDD cycle G2 |
| Added | `tests/test_grouping.py` — `GroupRoutesTests.test_rejects_duplicate_route_ids` | `group_routes` | Raw task behavior 7 | TDD cycle G3 |
| Added | `tests/test_grouping.py` — `GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city` | `group_routes` | Raw task behavior 8 | Non-TDD first-run pass G4 |
| Added | `tests/test_stats.py` — `ManifestStatsTests.test_reports_total_route_count` | `manifest_stats` | Raw task behavior 9 | Non-TDD first-run pass S1 |
| Added | `tests/test_stats.py` — `ManifestStatsTests.test_reports_country_counts_in_alphabetical_order` | `manifest_stats` | Raw task behavior 10 | TDD cycle S2 |
| Added | `tests/test_rendering.py` — `RenderManifestTests.test_emits_stable_country_and_route_lines` | `render_manifest` | Raw task behavior 11 | TDD cycle R1 |
| Added | `tests/test_rendering.py` — `RenderManifestTests.test_ends_with_exactly_one_newline_and_no_trailing_horizontal_whitespace` | `render_manifest` | Raw task behavior 12 | TDD cycle R2 |
| Added | `tests/test_rendering.py` — `RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_direct_input` | `render_manifest` | Raw task behavior 12 | TDD self-review cycle SR1 |
| Removed | `tests/test_normalization.py` — `NormalizeDestinationTests.test_legacy_preserves_destination_spelling` | `normalize_destination` | Legacy fixture, superseded by raw task behaviors 1-3 | Non-TDD removal: asserted forbidden legacy spelling preservation |
| Removed | `tests/test_grouping.py` — `GroupRoutesTests.test_legacy_groups_by_original_country` | `group_routes` | Legacy fixture, superseded by raw task behavior 5 | Non-TDD removal: asserted forbidden original-country grouping |
| Removed | `tests/test_stats.py` — `ManifestStatsTests.test_legacy_reports_total` | `manifest_stats` | Legacy fixture, expanded by raw task behaviors 9-10 | Non-TDD removal: exact total-only mapping contradicted required country counts |
| Removed | `tests/test_rendering.py` — `RenderManifestTests.test_legacy_uses_debug_representation` | `render_manifest` | Legacy fixture, superseded by raw task behaviors 11-12 | Non-TDD removal: asserted forbidden debug rendering |

Reconciliation checks:

- Changed selector inventory minus reconciliation identities: empty.
- Reconciliation identities minus changed selector inventory: empty.
- TDD evidence identities (N1, N2, N3, N4, G1, G2, G3, S2, R1, R2, SR1) minus TDD-custody reconciliation identities: empty.
- TDD-custody reconciliation identities minus TDD evidence identities: empty.
- Every TDD row has seam authority, file and exact selector, public entry point, intended and observed red, a complete red command, minimal change, a separate complete green command, and green result.

## Aggregate verification

Command: `python3 -m unittest discover -s tests -v`

Final result: `Ran 13 tests ... OK`.
