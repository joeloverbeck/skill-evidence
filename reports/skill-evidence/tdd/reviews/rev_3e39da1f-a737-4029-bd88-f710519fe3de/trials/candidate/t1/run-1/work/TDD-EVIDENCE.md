# TDD evidence

## Scope and seam authority

`TASK.md` is the raw user authority for all four public seams. The public entry points are the names exported by `routebook.__all__`:

- `normalize_destination` — behaviors 1–4.
- `group_routes` — behaviors 5–8.
- `manifest_stats` — behaviors 9–10.
- `render_manifest` — behaviors 11–12.

All tests call these exported functions. No internal collaborator is mocked or asserted.

## Retained red-green cycles

### C03 — uppercase a two-letter country code

- Authority: `TASK.md`, behavior 3.
- Test: `tests/test_normalization.py::NormalizeDestinationTests.test_uppercases_two_letter_country_code`.
- Observed public entry point: `routebook.normalize_destination`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v`.
- Intended failure: lowercase `no` was not converted to `NO`.
- Observed failure: tuple comparison reported `('Oslo', 'no') != ('Oslo', 'NO')`.
- Minimal production change: uppercase a country value when its length is two.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v`.
- Green result: one test ran and passed (`OK`).

### C04 — reject a blank city

- Authority: `TASK.md`, behavior 4.
- Test: `tests/test_normalization.py::NormalizeDestinationTests.test_rejects_blank_city`.
- Observed public entry point: `routebook.normalize_destination`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v`.
- Intended failure: an all-whitespace city was accepted.
- Observed failure: `AssertionError: ValueError not raised`.
- Minimal production change: raise `ValueError` when the normalized city is empty.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v`.
- Green result: one test ran and passed (`OK`).

### C06 — sort a group by normalized city and route ID

- Authority: `TASK.md`, behavior 6.
- Test: `tests/test_grouping.py::GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id`.
- Observed public entry point: `routebook.group_routes`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v`.
- Intended failure: input order was retained instead of normalized-city/ID order.
- Observed failure: route IDs were `['r2', 'r3', 'r1']` rather than `['r1', 'r2', 'r3']`.
- Minimal production change: sort each group by normalized city and then route ID.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v`.
- Green result: one test ran and passed (`OK`).

### C07 — reject an exact duplicate route ID

- Authority: `TASK.md`, behavior 7.
- Test: `tests/test_grouping.py::GroupRoutesTests.test_rejects_exact_duplicate_route_ids`.
- Observed public entry point: `routebook.group_routes`.
- Red command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_exact_duplicate_route_ids -v`.
- Intended failure: the same route ID was accepted in two countries.
- Observed failure: `AssertionError: ValueError not raised`.
- Minimal production change: track route IDs across the input and reject a repeated exact value.
- Green command: `python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_exact_duplicate_route_ids -v`.
- Green result: one test ran and passed (`OK`).

### C10 — report ordered per-country counts

- Authority: `TASK.md`, behavior 10.
- Test: `tests/test_stats.py::ManifestStatsTests.test_reports_per_country_counts_in_alphabetical_order`.
- Observed public entry point: `routebook.manifest_stats`.
- Red command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_per_country_counts_in_alphabetical_order -v`.
- Intended failure: no alphabetically ordered per-country counts were present.
- Observed failure: `KeyError: 'countries'`.
- Minimal production change: add a `countries` mapping constructed in sorted country order.
- Green command: `python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_per_country_counts_in_alphabetical_order -v`.
- Green result: one test ran and passed (`OK`).

### C11 — render stable country and route lines

- Authority: `TASK.md`, behavior 11.
- Test: `tests/test_rendering.py::RenderManifestTests.test_emits_countries_and_routes_in_stable_line_order`.
- Observed public entry point: `routebook.render_manifest`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_line_order -v`.
- Intended failure: the legacy dictionary representation did not emit sorted country headings followed by route lines.
- Observed failure: one debug-representation line was returned instead of the five required lines.
- Minimal production change: render sorted country headings and each country's routes as explicit lines.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_line_order -v`.
- Green result: one test ran and passed (`OK`).

### C12 — terminate cleanly

- Authority: `TASK.md`, behavior 12.
- Test: `tests/test_rendering.py::RenderManifestTests.test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace`.
- Observed public entry point: `routebook.render_manifest`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace -v`.
- Intended failure: output retained a trailing space/tab and lacked its final newline.
- Observed failure: actual `'ES:\n  r1: Madrid \t'` differed from expected `'ES:\n  r1: Madrid\n'`.
- Minimal production change: strip horizontal whitespace from each emitted line and append one newline.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace -v`.
- Green result: one test ran and passed (`OK`).

### R1-Spec-F1 — trim country whitespace before code normalization

- Review identity: self-review pass 1, Spec axis, finding F1; TDD re-entry required: yes.
- Authority: `TASK.md`, behaviors 1 and 3.
- Test: `tests/test_normalization.py::NormalizeDestinationTests.test_trims_country_whitespace_before_uppercasing`.
- Observed public entry point: `routebook.normalize_destination`.
- Red command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_country_whitespace_before_uppercasing -v`.
- Intended failure: surrounding country whitespace prevented recognition and uppercasing of a two-letter code.
- Observed failure: actual `' es '` differed from expected `'ES'`.
- Minimal production repair: strip country whitespace before the two-letter uppercase rule.
- Green command: `python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_country_whitespace_before_uppercasing -v`.
- Green result: one test ran and passed (`OK`).

### R1-Spec-F2 — clean physical lines on direct render ingress

- Review identity: self-review pass 1, Spec axis, finding F2; TDD re-entry required: yes.
- Authority: `TASK.md`, behavior 12, plus the method's direct-bypass coverage rule.
- Test: `tests/test_rendering.py::RenderManifestTests.test_cleans_physical_lines_for_direct_render_input`.
- Observed public entry point: `routebook.render_manifest`.
- Red command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_cleans_physical_lines_for_direct_render_input -v`.
- Intended failure: a direct input ending in horizontal whitespace plus a newline produced a dirty physical line and two final newlines.
- Observed failure: actual `'ES:\n  r1: Madrid \t\n\n'` differed from expected `'ES:\n  r1: Madrid\n'`.
- Minimal production repair: split logical values into physical lines, strip trailing spaces/tabs from each, then append one final newline.
- Green command: `python3 -m unittest tests.test_rendering.RenderManifestTests.test_cleans_physical_lines_for_direct_render_input -v`.
- Green result: one test ran and passed (`OK`).

## Observed history without retained TDD custody

The following focused tests genuinely ran red and then green, but later valid slices exposed an over-broad expectation and required a post-production edit to the same selector. Under the method, the final selectors are therefore reconciled as non-TDD rather than claiming custody:

- `NormalizeDestinationTests.test_trims_surrounding_city_whitespace`: red observed the untrimmed city, then green after `str.strip`; the final assertion was later narrowed so country uppercasing could vary independently.
- `NormalizeDestinationTests.test_collapses_internal_city_whitespace`: red observed the internal whitespace run, then green after whitespace collapsing; the final assertion was later narrowed so country uppercasing could vary independently.
- `GroupRoutesTests.test_groups_by_normalized_country_code`: red observed separate `es` and `ES` groups, then green after normalized key grouping; the final assertion was later made insensitive to the separately required route ordering.

## Final changed-test-to-seam reconciliation

| Change | File and exact identity | Public entry point | Seam authority | TDD evidence or disposition |
|---|---|---|---|---|
| removed | `tests/test_normalization.py::NormalizeDestinationTests.test_legacy_preserves_destination_spelling` | `normalize_destination` | Superseded by `TASK.md` behaviors 1–4 | Non-TDD: removed because its legacy expectation contradicted the new authority. |
| added | `tests/test_normalization.py::NormalizeDestinationTests.test_trims_surrounding_city_whitespace` | `normalize_destination` | `TASK.md` behavior 1 | Non-TDD: selector edited after production; observed history is disclosed above. |
| added | `tests/test_normalization.py::NormalizeDestinationTests.test_collapses_internal_city_whitespace` | `normalize_destination` | `TASK.md` behavior 2 | Non-TDD: selector edited after production; observed history is disclosed above. |
| added | `tests/test_normalization.py::NormalizeDestinationTests.test_uppercases_two_letter_country_code` | `normalize_destination` | `TASK.md` behavior 3 | TDD C03. |
| added | `tests/test_normalization.py::NormalizeDestinationTests.test_trims_country_whitespace_before_uppercasing` | `normalize_destination` | `TASK.md` behaviors 1 and 3 | TDD R1-Spec-F1. |
| added | `tests/test_normalization.py::NormalizeDestinationTests.test_rejects_blank_city` | `normalize_destination` | `TASK.md` behavior 4 | TDD C04. |
| removed | `tests/test_grouping.py::GroupRoutesTests.test_legacy_groups_by_original_country` | `group_routes` | Superseded by `TASK.md` behaviors 5–8 | Non-TDD: removed because its legacy expectation contradicted the new authority. |
| added | `tests/test_grouping.py::GroupRoutesTests.test_groups_by_normalized_country_code` | `group_routes` | `TASK.md` behavior 5 | Non-TDD: selector edited after production; observed history is disclosed above. |
| added | `tests/test_grouping.py::GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id` | `group_routes` | `TASK.md` behavior 6 | TDD C06. |
| added | `tests/test_grouping.py::GroupRoutesTests.test_rejects_exact_duplicate_route_ids` | `group_routes` | `TASK.md` behavior 7 | TDD C07. |
| added | `tests/test_grouping.py::GroupRoutesTests.test_preserves_distinct_route_ids_for_the_same_city` | `group_routes` | `TASK.md` behavior 8 | Non-TDD: first focused run passed; no production change was driven. |
| removed | `tests/test_stats.py::ManifestStatsTests.test_legacy_reports_total` | `manifest_stats` | Superseded by `TASK.md` behaviors 9–10 | Non-TDD: renamed and narrowed for the new mapping contract. |
| added | `tests/test_stats.py::ManifestStatsTests.test_reports_total_route_count` | `manifest_stats` | `TASK.md` behavior 9 | Non-TDD: first focused run passed; the legacy implementation already counted totals. |
| added | `tests/test_stats.py::ManifestStatsTests.test_reports_per_country_counts_in_alphabetical_order` | `manifest_stats` | `TASK.md` behavior 10 | TDD C10. |
| removed | `tests/test_rendering.py::RenderManifestTests.test_legacy_uses_debug_representation` | `render_manifest` | Superseded by `TASK.md` behaviors 11–12 | Non-TDD: removed because its legacy expectation contradicted the new authority. |
| added | `tests/test_rendering.py::RenderManifestTests.test_emits_countries_and_routes_in_stable_line_order` | `render_manifest` | `TASK.md` behavior 11 | TDD C11. |
| added | `tests/test_rendering.py::RenderManifestTests.test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace` | `render_manifest` | `TASK.md` behavior 12 | TDD C12. |
| added | `tests/test_rendering.py::RenderManifestTests.test_cleans_physical_lines_for_direct_render_input` | `render_manifest` | `TASK.md` behavior 12 and direct-bypass coverage | TDD R1-Spec-F2. |

### Bidirectional reconciliation checks

- Changed-selector inventory minus reconciliation identities: empty.
- Reconciliation identities minus changed-selector inventory: empty.
- Retained TDD evidence IDs minus TDD-custody reconciliation IDs: empty.
- TDD-custody reconciliation IDs minus retained TDD evidence IDs: empty.
- Retained TDD custody set on both sides: `C03`, `C04`, `C06`, `C07`, `C10`, `C11`, `C12`, `R1-Spec-F1`, `R1-Spec-F2`.

## Findings fixed during self-review

| Review identity | Finding | Resolution | Re-entry row |
|---|---|---|---|
| R1-Spec-F1 | Country surrounding whitespace was not trimmed before applying the two-letter uppercase rule. | Fixed and focused green observed. | R1-Spec-F1. |
| R1-Spec-F2 | Direct render input could retain trailing horizontal whitespace on a physical line and end with two newlines. | Fixed and focused green observed. | R1-Spec-F2. |

- Applicable self-review findings minus review-reentry rows: empty.
- Review-reentry rows minus applicable self-review findings: empty.

## Pre-review aggregate verification

- Command: `python3 -m unittest discover -s tests -v`.
- Result: 12 tests ran and passed (`OK`).

## Self-review outcome

One contract self-review covered all twelve requested behaviors through the four exported entry points. It found F1 and F2 above; both were repaired through their own focused red-green slices, and the finding/re-entry comparison is empty in both directions. No open finding remains.

## Final aggregate verification

- Command: `python3 -m unittest discover -s tests -v`.
- Result: 14 tests ran and passed (`OK`).
