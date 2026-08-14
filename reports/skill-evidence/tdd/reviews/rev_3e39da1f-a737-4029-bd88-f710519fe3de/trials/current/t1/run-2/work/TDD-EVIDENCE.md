# TDD evidence: Routebook manifest v2

## Authority and public seams

`../TASK.md` is the explicit user authority for all twelve behaviors. The observed public
entry points are the four names exported by `routebook.__all__`:
`normalize_destination`, `group_routes`, `manifest_stats`, and `render_manifest`.
Every test enters through `from routebook import ...`; no test imports an implementation
module or observes a private collaborator.

Focused-command key used below:

`F[SELECTOR]` = `python3 -m unittest -v SELECTOR`, run from the `work/` directory.

The expected values are literals derived directly from `../TASK.md`. No mocks are used.

## Retained cycle evidence

| ID | Behavior and seam authority | Test selector; public entry point | Red command and intended plus observed failure | Green command and result |
|---|---|---|---|---|
| E01 | TASK behavior 1: trim surrounding whitespace | `tests/test_normalization.py`; `tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_whitespace`; `normalize_destination` | `F[tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_whitespace]`; intended: surrounding city and country whitespace must disappear; observed: returned tuple retained both | `F[tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_whitespace]`; `OK` |
| E02 | TASK behavior 2: collapse internal city whitespace | `tests/test_normalization.py`; `tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace`; `normalize_destination` | `F[tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace]`; intended: tab-plus-spaces becomes one space; observed: `New\t  York` was retained | `F[tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace]`; `OK` |
| E03 | TASK behavior 3: uppercase a two-letter country code | `tests/test_normalization.py`; `tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code`; `normalize_destination` | `F[tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code]`; intended: `es` becomes `ES`; observed: `es` was returned | `F[tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code]`; `OK` |
| E04 | TASK behavior 4: reject a blank city | `tests/test_normalization.py`; `tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city`; `normalize_destination` | `F[tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city]`; intended: whitespace-only city raises `ValueError`; observed: no exception | `F[tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city]`; `OK` |
| E05 | TASK behavior 5: group by normalized country code | `tests/test_grouping.py`; `tests.test_grouping.GroupRoutesTests.test_groups_by_normalized_country_code`; `group_routes` | `F[tests.test_grouping.GroupRoutesTests.test_groups_by_normalized_country_code]`; intended: input country `es` produces key/route country `ES`; observed: key and route retained `es` | `F[tests.test_grouping.GroupRoutesTests.test_groups_by_normalized_country_code]`; `OK` |
| E06 | TASK behavior 6: sort by normalized city then route ID | `tests/test_grouping.py`; `tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id`; `group_routes` | `F[tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id]`; intended: normalized `A Coruna` first and equal-city IDs `r1`, `r2`; observed: input order `r2`, `r3`, `r1` | `F[tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id]`; `OK` |
| E07 | TASK behavior 7: reject duplicate route IDs | `tests/test_grouping.py`; `tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids`; `group_routes` | `F[tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids]`; intended: repeated `r1` across different destinations raises `ValueError`; observed: no exception | `F[tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids]`; `OK` |
| E08 | TASK behavior 8: preserve distinct IDs for the same normalized city | `tests/test_grouping.py`; `tests.test_grouping.GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city`; `group_routes` | No truthful red existed: the supplied grouping behavior already preserved both records, and the focused test returned `OK` before further production changes. No artificial regression was introduced. | `F[tests.test_grouping.GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city]`; `OK` |
| E09 | TASK behavior 9: report total route count | `tests/test_stats.py`; `tests.test_stats.ManifestStatsTests.test_reports_total_route_count`; `manifest_stats` | No truthful red existed: the supplied public implementation already returned total `2`, and the focused v2-named regression test returned `OK` before the per-country slice. No artificial regression was introduced. | `F[tests.test_stats.ManifestStatsTests.test_reports_total_route_count]`; `OK` |
| E10 | TASK behavior 10: alphabetical per-country counts | `tests/test_stats.py`; `tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order`; `manifest_stats` | `F[tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order]`; intended: ordered `ES: 2`, `FR: 1`; observed: `KeyError: 'countries'` | `F[tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order]`; `OK` |
| E11 | TASK behavior 11: stable country lines followed by routes | `tests/test_rendering.py`; `tests.test_rendering.RenderManifestTests.test_emits_stable_country_lines_followed_by_routes`; `render_manifest` | `F[tests.test_rendering.RenderManifestTests.test_emits_stable_country_lines_followed_by_routes]`; intended: ordered country/route lines; observed: one legacy dictionary-representation line | `F[tests.test_rendering.RenderManifestTests.test_emits_stable_country_lines_followed_by_routes]`; `OK` |
| E12 | TASK behavior 12: exactly one final newline and no trailing horizontal whitespace | `tests/test_rendering.py`; `tests.test_rendering.RenderManifestTests.test_ends_in_exactly_one_newline_without_trailing_horizontal_whitespace`; `render_manifest` | `F[tests.test_rendering.RenderManifestTests.test_ends_in_exactly_one_newline_without_trailing_horizontal_whitespace]`; intended: output ends with newline; observed: `endswith("\n")` was false | `F[tests.test_rendering.RenderManifestTests.test_ends_in_exactly_one_newline_without_trailing_horizontal_whitespace]`; `OK` |
| SR1 | Self-review pass 1, contract finding `trailing-horizontal-whitespace-direct-render`; TASK behavior 12; TDD re-entry required: yes | `tests/test_rendering.py`; `tests.test_rendering.RenderManifestTests.test_strips_trailing_horizontal_whitespace_from_rendered_fields`; `render_manifest` | `F[tests.test_rendering.RenderManifestTests.test_strips_trailing_horizontal_whitespace_from_rendered_fields]`; intended: a direct public call cannot emit field-supplied terminal spaces/tabs; observed: at least one rendered line differed from its horizontal `rstrip` | `F[tests.test_rendering.RenderManifestTests.test_strips_trailing_horizontal_whitespace_from_rendered_fields]`; `OK` |

The aggregate run after the initial slices exposed that E01 and E02's first fixtures also
froze values owned by later authorized slices. Their fixtures were narrowed to remain
sensitive to only their claimed behaviors; both focused commands above were replayed green.

## Contract self-review

One review pass checked each of the twelve numbered requirements against the exported public
entry point and its focused test. It found one bypass: `render_manifest` can be called directly,
without `group_routes`, so terminal spaces/tabs supplied in a country, route ID, or city could
become trailing horizontal output. Finding `trailing-horizontal-whitespace-direct-render` was
repaired through SR1. No other contract mismatch was found.

Findings fixed during review: `{SR1/trailing-horizontal-whitespace-direct-render}`.
Review-reentry evidence identities: `{SR1/trailing-horizontal-whitespace-direct-render}`.
Both set differences are empty.

## Changed-test-to-seam reconciliation

The inventory below was derived selector-by-selector from the four changed test files relative
to their supplied contents.

| Changed test identity | Observed public entry point | Seam authority | Evidence row |
|---|---|---|---|
| `tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_whitespace` | `normalize_destination` | TASK behavior 1 | E01 |
| `tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace` | `normalize_destination` | TASK behavior 2 | E02 |
| `tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code` | `normalize_destination` | TASK behavior 3 | E03 |
| `tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city` | `normalize_destination` | TASK behavior 4 | E04 |
| `tests.test_grouping.GroupRoutesTests.test_groups_by_normalized_country_code` | `group_routes` | TASK behavior 5 | E05 |
| `tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id` | `group_routes` | TASK behavior 6 | E06 |
| `tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids` | `group_routes` | TASK behavior 7 | E07 |
| `tests.test_grouping.GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city` | `group_routes` | TASK behavior 8 | E08 |
| `tests.test_stats.ManifestStatsTests.test_reports_total_route_count` | `manifest_stats` | TASK behavior 9 | E09 |
| `tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order` | `manifest_stats` | TASK behavior 10 | E10 |
| `tests.test_rendering.RenderManifestTests.test_emits_stable_country_lines_followed_by_routes` | `render_manifest` | TASK behavior 11 | E11 |
| `tests.test_rendering.RenderManifestTests.test_ends_in_exactly_one_newline_without_trailing_horizontal_whitespace` | `render_manifest` | TASK behavior 12 | E12 |
| `tests.test_rendering.RenderManifestTests.test_strips_trailing_horizontal_whitespace_from_rendered_fields` | `render_manifest` | TASK behavior 12 direct-ingress review repair | SR1 |

Changed-test inventory identities minus reconciliation identities: empty set.

Reconciliation identities minus changed-test inventory identities: empty set.

## Aggregate verification

Command: `python3 -m unittest discover -s tests -v`

Result: `Ran 13 tests`; `OK`.
