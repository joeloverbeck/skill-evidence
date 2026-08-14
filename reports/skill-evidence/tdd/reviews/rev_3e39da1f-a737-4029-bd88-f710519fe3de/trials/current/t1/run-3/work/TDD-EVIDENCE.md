# TDD evidence: Routebook manifest v2

## Agreed public seams

`TASK.md` is the authority for behaviors B01-B12. `README.md` states that the
package exposes only the names in `routebook.__all__`, so the agreed observed
seams are the four public functions imported from `routebook`:
`normalize_destination`, `group_routes`, `manifest_stats`, and
`render_manifest`. All tests exercise those public entry points; no internal
collaborator is mocked or inspected.

Commands below are complete and run from the `work/` directory.

## Retained cycles

### N01 - B01: trim surrounding city whitespace

- Seam authority: `TASK.md` behavior 1 and the public `routebook.normalize_destination` seam.
- Test: `tests/test_normalization.py`; selector `tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_city_whitespace`.
- Observed public entry point: `routebook.normalize_destination`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_city_whitespace -v`
- Intended failure: the returned city still contained its surrounding spaces.
- Observed red: exit 1; tuple mismatch `('  Madrid  ', 'ES') != ('Madrid', 'ES')`.
- Minimal implementation: strip surrounding city whitespace.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_city_whitespace -v`
- Green result: exit 0; 1 test passed.

### N02 - B02: collapse internal city whitespace

- Seam authority: `TASK.md` behavior 2 and the public `routebook.normalize_destination` seam.
- Test: `tests/test_normalization.py`; selector `tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace`.
- Observed public entry point: `routebook.normalize_destination`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace -v`
- Intended failure: a mixed internal run of spaces and a tab was retained.
- Observed red: exit 1; tuple mismatch `('New \t  York', 'US') != ('New York', 'US')`.
- Minimal implementation: form the normalized city by joining its whitespace-delimited words with one space.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace -v`
- Green result: exit 0; 1 test passed.

### N03 - B03: uppercase a two-letter country code

- Seam authority: `TASK.md` behavior 3 and the public `routebook.normalize_destination` seam.
- Test: `tests/test_normalization.py`; selector `tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code`.
- Observed public entry point: `routebook.normalize_destination`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v`
- Intended failure: the lower-case two-letter code was returned unchanged.
- Observed red: exit 1; tuple mismatch `('Boston', 'us') != ('Boston', 'US')`.
- Minimal implementation: uppercase country strings whose length is two.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v`
- Green result: exit 0; 1 test passed.

### N04 - B04: reject a blank city

- Seam authority: `TASK.md` behavior 4 and the public `routebook.normalize_destination` seam.
- Test: `tests/test_normalization.py`; selector `tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city`.
- Observed public entry point: `routebook.normalize_destination`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v`
- Intended failure: whitespace-only input was accepted as an empty normalized city.
- Observed red: exit 1; `ValueError not raised`.
- Minimal implementation: raise `ValueError("city must not be blank")` after city normalization produces an empty value.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v`
- Green result: exit 0; 1 test passed.

### G01 - B05: group by normalized country

- Seam authority: `TASK.md` behavior 5 and the public `routebook.group_routes` seam.
- Test: `tests/test_grouping.py`; selector `tests.test_grouping.GroupRoutesTests.test_groups_routes_by_normalized_country_code`.
- Observed public entry point: `routebook.group_routes`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_groups_routes_by_normalized_country_code -v`
- Intended failure: lower-case country keys and unnormalized route fields remained observable.
- Observed red: exit 1; actual groups used `es`/`pt` and retained `" Madrid "`, while the golden used `ES`/`PT` and `"Madrid"`.
- Minimal implementation: normalize each destination, copy the route with normalized fields, and group on the normalized country.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_groups_routes_by_normalized_country_code -v`
- Green result: exit 0; 1 test passed.

### G02 - B06 and B08: stable city/ID order while preserving distinct same-city IDs

- Seam authority: `TASK.md` behaviors 6 and 8 and the public `routebook.group_routes` seam.
- Test: `tests/test_grouping.py`; selector `tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id`.
- Observed public entry point: `routebook.group_routes`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v`
- Intended failure: input order was retained instead of normalized-city then route-ID order.
- Observed red: exit 1; IDs were `['r2', 'r1', 'r9']` instead of the discriminating golden `['r9', 'r1', 'r2']`. Both distinct IDs for normalized city `Rome` were already present in the red output; the green preserves that pre-existing B08 behavior.
- Minimal implementation: sort every accumulated group by `(route["city"], route["id"])` without deduplicating by city.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v`
- Green result: exit 0; 1 test passed with both `r1` and `r2` retained.

### G03 - B07: reject duplicate route IDs

- Seam authority: `TASK.md` behavior 7 and the public `routebook.group_routes` seam.
- Test: `tests/test_grouping.py`; selector `tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids_across_countries`.
- Observed public entry point: `routebook.group_routes`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids_across_countries -v`
- Intended failure: the same exact ID was accepted in different country groups.
- Observed red: exit 1; `ValueError not raised`.
- Minimal implementation: track route IDs across the full input and raise `ValueError("duplicate route id: r1")` on reuse.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids_across_countries -v`
- Green result: exit 0; 1 test passed.

### S01 - B09: report total route count

- Seam authority: `TASK.md` behavior 9 and the public `routebook.manifest_stats` seam.
- Test: `tests/test_stats.py`; selector `tests.test_stats.ManifestStatsTests.test_reports_total_route_count`.
- Observed public entry point: `routebook.manifest_stats`.
- Red command: not applicable: the supplied implementation already satisfied B09, as confirmed by the passing baseline legacy total test. No production change was made for this behavior and no false red was manufactured.
- Focused characterization command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_total_route_count -v`
- Characterization result: exit 0; 1 test passed before the per-country implementation slice.
- Green command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_total_route_count -v`
- Green result: exit 0; 1 test passed in the final implementation.

### S02 - B10: report alphabetically ordered country counts

- Seam authority: `TASK.md` behavior 10 and the public `routebook.manifest_stats` seam.
- Test: `tests/test_stats.py`; selector `tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order`.
- Observed public entry point: `routebook.manifest_stats`.
- Red command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order -v`
- Intended failure: the result omitted the ordered `countries` mapping.
- Observed red: exit 1; `{'total': 6}` differed from `{'total': 6, 'countries': {'ES': 1, 'PT': 3, 'US': 2}}`.
- Minimal implementation: build country counts by sorted country key and compute the total from those counts.
- Green command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order -v`
- Green result: exit 0; 1 test passed.

### R01 - B11: stable country and route lines

- Seam authority: `TASK.md` behavior 11 and the public `routebook.render_manifest` seam.
- Test: `tests/test_rendering.py`; selector `tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_order`.
- Observed public entry point: `routebook.render_manifest`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_order -v`
- Intended failure: debug representation did not produce ordered country headers followed by ordered route lines.
- Observed red: exit 1; one `repr` line differed from the golden lines `ES:`, its Madrid/Seville routes, then `PT:` and its Porto route.
- Minimal implementation: sort country keys, emit one header per country, then sort and emit its routes by city and ID.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_order -v`
- Green result: exit 0; 1 test passed.

### R02 - B12: exactly one final newline

- Seam authority: `TASK.md` behavior 12 and the public `routebook.render_manifest` seam.
- Test: `tests/test_rendering.py`; selector `tests.test_rendering.RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace`.
- Observed public entry point: `routebook.render_manifest`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace -v`
- Intended failure: the rendered text had no final newline.
- Observed red: exit 1; `output.endswith("\n")` was false.
- Minimal implementation: append one newline to the joined output lines.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace -v`
- Green result: exit 0; 1 test passed, including the ordinary-input horizontal-whitespace assertion.

### R03 - B12: no trailing horizontal whitespace from rendered fields

- Seam authority: `TASK.md` behavior 12 and the public `routebook.render_manifest` seam.
- Test: `tests/test_rendering.py`; selector `tests.test_rendering.RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_rendered_fields`.
- Observed public entry point: `routebook.render_manifest`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_rendered_fields -v`
- Intended failure: trailing spaces and a tab in a route field remained at the end of the rendered line.
- Observed red: exit 1; actual contained `"  r1: Madrid \t\n"` instead of `"  r1: Madrid\n"`.
- Minimal implementation: remove spaces and tabs from the end of every constructed route line.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_rendered_fields -v`
- Green result: exit 0; 1 test passed.

## Contract self-review

- B01-B04: normalization trims and collapses city whitespace, uppercases two-letter country codes, and rejects a city that normalizes to blank.
- B05-B08: grouping normalizes copied route fields, uses normalized country keys, orders routes by normalized city then ID, rejects duplicate IDs globally, and retains distinct IDs for the same city.
- B09-B10: statistics include the total and an insertion-ordered mapping built from alphabetically sorted country keys.
- B11-B12: rendering sorts countries and routes independently of input mapping/sequence order, emits a country header followed by its route lines, terminates with one newline, and strips trailing spaces/tabs from route lines.
- Finding: none. No repair cycle was required.

## Final changed-test inventory and seam reconciliation

| Test file | Exact selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `tests/test_normalization.py` | `tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_city_whitespace` | `routebook.normalize_destination` | `TASK.md` B01 | N01 |
| `tests/test_normalization.py` | `tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace` | `routebook.normalize_destination` | `TASK.md` B02 | N02 |
| `tests/test_normalization.py` | `tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code` | `routebook.normalize_destination` | `TASK.md` B03 | N03 |
| `tests/test_normalization.py` | `tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city` | `routebook.normalize_destination` | `TASK.md` B04 | N04 |
| `tests/test_grouping.py` | `tests.test_grouping.GroupRoutesTests.test_groups_routes_by_normalized_country_code` | `routebook.group_routes` | `TASK.md` B05 | G01 |
| `tests/test_grouping.py` | `tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id` | `routebook.group_routes` | `TASK.md` B06 and B08 | G02 |
| `tests/test_grouping.py` | `tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids_across_countries` | `routebook.group_routes` | `TASK.md` B07 | G03 |
| `tests/test_stats.py` | `tests.test_stats.ManifestStatsTests.test_reports_total_route_count` | `routebook.manifest_stats` | `TASK.md` B09 | S01 |
| `tests/test_stats.py` | `tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order` | `routebook.manifest_stats` | `TASK.md` B10 | S02 |
| `tests/test_rendering.py` | `tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_order` | `routebook.render_manifest` | `TASK.md` B11 | R01 |
| `tests/test_rendering.py` | `tests.test_rendering.RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace` | `routebook.render_manifest` | `TASK.md` B12 | R02 |
| `tests/test_rendering.py` | `tests.test_rendering.RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_rendered_fields` | `routebook.render_manifest` | `TASK.md` B12 | R03 |

Inventory identities minus reconciliation identities: empty.

Reconciliation identities minus inventory identities: empty.

## Aggregate verification

- Command: `python3 -m unittest discover -s tests -v`
- Result after self-review: exit 0; 12 tests passed.
