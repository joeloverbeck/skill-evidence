# Routebook manifest v2 TDD evidence

## Authority and public seams

`../TASK.md` is the seam authority for all twelve behaviors. The observed entry points are the four names exported by `routebook.__all__`: `normalize_destination`, `group_routes`, `manifest_stats`, and `render_manifest`. All tests call those exported functions directly; no private function, internal collaborator, mock, or side channel is used.

Focused command keys below are stable and runnable from any directory:

- `NORM <selector>`: `cd /tmp/tdd-evolution-rev-3e39-current-1/work && python3 -m unittest tests/test_normalization.py -k <selector> -v`
- `GROUP <selector>`: `cd /tmp/tdd-evolution-rev-3e39-current-1/work && python3 -m unittest tests/test_grouping.py -k <selector> -v`
- `STATS <selector>`: `cd /tmp/tdd-evolution-rev-3e39-current-1/work && python3 -m unittest tests/test_stats.py -k <selector> -v`
- `RENDER <selector>`: `cd /tmp/tdd-evolution-rev-3e39-current-1/work && python3 -m unittest tests/test_rendering.py -k <selector> -v`

Each focused run selected exactly one test.

## Retained red/green cycles

| Row | Authority | Test file and exact selector | Public entry point | Red command | Intended and observed red | Minimal green change | Green command and result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| N1 | Task behavior 1 | `tests/test_normalization.py::NormalizeDestinationTests.test_trims_surrounding_city_whitespace` | `normalize_destination` | `NORM test_trims_surrounding_city_whitespace` | Intended: surrounding city spaces removed. Observed: returned `'  Madrid  '` instead of `'Madrid'`. | Strip city whitespace. | `NORM test_trims_surrounding_city_whitespace` — `Ran 1 test`, `OK`. |
| N2 | Task behavior 2 | `tests/test_normalization.py::NormalizeDestinationTests.test_collapses_internal_city_whitespace` | `normalize_destination` | `NORM test_collapses_internal_city_whitespace` | Intended: `New   York` becomes `New York`. Observed: the three spaces were retained. | Join the city's whitespace-separated words with one space. | `NORM test_collapses_internal_city_whitespace` — `Ran 1 test`, `OK`. |
| N3 | Task behavior 3 | `tests/test_normalization.py::NormalizeDestinationTests.test_uppercases_two_letter_country_code` | `normalize_destination` | `NORM test_uppercases_two_letter_country_code` | Intended: `es` becomes `ES`. Observed: returned `es`. | Uppercase the country code. | `NORM test_uppercases_two_letter_country_code` — `Ran 1 test`, `OK`. |
| N4 | Task behavior 1, destination-wide whitespace interpretation | `tests/test_normalization.py::NormalizeDestinationTests.test_trims_surrounding_country_whitespace` | `normalize_destination` | `NORM test_trims_surrounding_country_whitespace` | Intended: surrounding country spaces removed. Observed: returned `'  ES  '`. | Strip the country code before uppercasing it. | `NORM test_trims_surrounding_country_whitespace` — `Ran 1 test`, `OK`. |
| N5 | Task behavior 4 | `tests/test_normalization.py::NormalizeDestinationTests.test_rejects_blank_city` | `normalize_destination` | `NORM test_rejects_blank_city` | Intended: whitespace-only city raises `ValueError`. Observed: no exception was raised. | Reject an empty normalized city with `ValueError`. | `NORM test_rejects_blank_city` — `Ran 1 test`, `OK`. |
| G1 | Task behavior 5 | `tests/test_grouping.py::GroupRoutesTests.test_groups_routes_by_normalized_country_code` | `group_routes` | `GROUP test_groups_routes_by_normalized_country_code` | Intended: normalized route under `ES`. Observed: unnormalized route remained under `es`. | Normalize each destination, copy the route, and group by normalized country. | `GROUP test_groups_routes_by_normalized_country_code` — `Ran 1 test`, `OK`. |
| G2 | Task behavior 6 | `tests/test_grouping.py::GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id` | `group_routes` | `GROUP test_sorts_each_group_by_normalized_city_then_route_id` | Intended ID order: `r1, r2, r3`. Observed insertion order: `r3, r2, r1`. | Sort each group by `(city, id)` after normalization. | `GROUP test_sorts_each_group_by_normalized_city_then_route_id` — `Ran 1 test`, `OK`. |
| G3 | Task behavior 7 | `tests/test_grouping.py::GroupRoutesTests.test_rejects_duplicate_route_ids` | `group_routes` | `GROUP test_rejects_duplicate_route_ids` | Intended: repeated `r1` raises `ValueError`. Observed: no exception was raised. | Track route IDs across all groups and reject a repeated ID. | `GROUP test_rejects_duplicate_route_ids` — `Ran 1 test`, `OK`. |
| G4 | Task behavior 8 | `tests/test_grouping.py::GroupRoutesTests.test_preserves_distinct_route_ids_for_same_normalized_city` | `group_routes` | `GROUP test_preserves_distinct_route_ids_for_same_normalized_city` | Intended red would show one distinct ID lost. Observed first focused execution was already green (`r1` and `r2` both present), so no artificial red was manufactured. | No production change; retain the public characterization test. | `GROUP test_preserves_distinct_route_ids_for_same_normalized_city` — first execution `Ran 1 test`, `OK`. |
| S1 | Task behavior 9 | `tests/test_stats.py::ManifestStatsTests.test_reports_total_route_count` | `manifest_stats` | `STATS test_reports_total_route_count` | Intended red would report a total other than `3`. Observed first focused execution was already green, so no artificial red was manufactured. | No production change; retain the public characterization test. | `STATS test_reports_total_route_count` — first execution `Ran 1 test`, `OK`. |
| S2 | Task behavior 10 | `tests/test_stats.py::ManifestStatsTests.test_reports_per_country_counts_in_alphabetical_order` | `manifest_stats` | `STATS test_reports_per_country_counts_in_alphabetical_order` | Intended: `ES: 2` then `FR: 1`. Observed: `KeyError: 'countries'`. | Add a `countries` mapping populated in sorted country order. | `STATS test_reports_per_country_counts_in_alphabetical_order` — `Ran 1 test`, `OK`. |
| R1 | Task behavior 11 | `tests/test_rendering.py::RenderManifestTests.test_emits_stable_country_lines_followed_by_their_routes` | `render_manifest` | `RENDER test_emits_stable_country_lines_followed_by_their_routes` | Intended: sorted country headers and sorted route lines. Observed: one legacy dictionary-representation line. | Render sorted country headers and `(city, id)`-sorted route lines. | `RENDER test_emits_stable_country_lines_followed_by_their_routes` — `Ran 1 test`, `OK`. |
| R2 | Task behavior 12 | `tests/test_rendering.py::RenderManifestTests.test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace` | `render_manifest` | `RENDER test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace` | Intended: exact `"ES:\n  r1: Madrid\n"`. Observed: terminal newline was absent. | Append one terminal newline. | `RENDER test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace` — `Ran 1 test`, `OK`. |
| REVIEW-1-SPEC-TRAILING-HSPACE | Review pass 1, Spec axis, task behavior 12 | `tests/test_rendering.py::RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_each_line` | `render_manifest` | `RENDER test_removes_trailing_horizontal_whitespace_from_each_line` | Intended: direct public input ending `Madrid \t` cannot produce trailing horizontal whitespace. Observed: output contained space and tab before the newline. | Right-strip spaces and tabs from every rendered line. | `RENDER test_removes_trailing_horizontal_whitespace_from_each_line` — `Ran 1 test`, `OK`. |

The two first-green rows document behaviors already supplied by the legacy implementation. They remain explicit rather than inventing a failing expectation or deliberately breaking working production code.

## Self-review

Review pass 1 checked each numbered behavior in `../TASK.md` against production code and public tests. One actionable Spec finding was found and repaired:

| Finding identity | Finding | TDD re-entry required | Evidence row | Status |
| --- | --- | --- | --- | --- |
| `review-1/spec/trailing-horizontal-whitespace-at-direct-render-seam` | Direct `render_manifest` input could place a space or tab before a line-ending newline. | yes | `REVIEW-1-SPEC-TRAILING-HSPACE` | Fixed and focused green. |

Findings fixed during review minus review-reentry identities: empty. Review-reentry identities minus findings fixed during review: empty.

After the repair, all twelve requested behaviors have direct public-seam coverage. The implementation remains limited to destination normalization, grouping and sorting, statistics, and deterministic text rendering.

## Final changed-test-to-seam reconciliation

The changed-test inventory below was derived from the scoped test diff. Each selector is reconciled individually.

| Test file | Exact changed selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `tests/test_normalization.py` | `NormalizeDestinationTests.test_trims_surrounding_city_whitespace` | `normalize_destination` | Task behavior 1 | `N1` |
| `tests/test_normalization.py` | `NormalizeDestinationTests.test_trims_surrounding_country_whitespace` | `normalize_destination` | Task behavior 1 | `N4` |
| `tests/test_normalization.py` | `NormalizeDestinationTests.test_collapses_internal_city_whitespace` | `normalize_destination` | Task behavior 2 | `N2` |
| `tests/test_normalization.py` | `NormalizeDestinationTests.test_uppercases_two_letter_country_code` | `normalize_destination` | Task behavior 3 | `N3` |
| `tests/test_normalization.py` | `NormalizeDestinationTests.test_rejects_blank_city` | `normalize_destination` | Task behavior 4 | `N5` |
| `tests/test_grouping.py` | `GroupRoutesTests.test_groups_routes_by_normalized_country_code` | `group_routes` | Task behavior 5 | `G1` |
| `tests/test_grouping.py` | `GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id` | `group_routes` | Task behavior 6 | `G2` |
| `tests/test_grouping.py` | `GroupRoutesTests.test_rejects_duplicate_route_ids` | `group_routes` | Task behavior 7 | `G3` |
| `tests/test_grouping.py` | `GroupRoutesTests.test_preserves_distinct_route_ids_for_same_normalized_city` | `group_routes` | Task behavior 8 | `G4` |
| `tests/test_stats.py` | `ManifestStatsTests.test_reports_total_route_count` | `manifest_stats` | Task behavior 9 | `S1` |
| `tests/test_stats.py` | `ManifestStatsTests.test_reports_per_country_counts_in_alphabetical_order` | `manifest_stats` | Task behavior 10 | `S2` |
| `tests/test_rendering.py` | `RenderManifestTests.test_emits_stable_country_lines_followed_by_their_routes` | `render_manifest` | Task behavior 11 | `R1` |
| `tests/test_rendering.py` | `RenderManifestTests.test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace` | `render_manifest` | Task behavior 12 | `R2` |
| `tests/test_rendering.py` | `RenderManifestTests.test_removes_trailing_horizontal_whitespace_from_each_line` | `render_manifest` | Task behavior 12 and review finding | `REVIEW-1-SPEC-TRAILING-HSPACE` |

Changed-test inventory identities minus reconciliation identities: empty.

Reconciliation identities minus changed-test inventory identities: empty.

All evidence-row selectors are present in reconciliation, and all reconciliation rows point to one retained evidence row.

## Aggregate verification

Command:

```bash
cd /tmp/tdd-evolution-rev-3e39-current-1/work && python3 -B -m unittest discover -s tests -v
```

Result: `Ran 14 tests in 0.000s` and `OK` (exit 0).
