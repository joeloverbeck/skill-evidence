# TDD evidence: Routebook manifest v2

## Authority and public seams

`../TASK.md` is the explicit user authority for all twelve behaviors. The tests observe only the
four names exported by `routebook.__all__`:

- `normalize_destination(city, country)` for destination normalization and blank-city refusal;
- `group_routes(routes)` for normalized grouping, deterministic route ordering, duplicate-ID
  refusal, and preservation of distinct IDs;
- `manifest_stats(groups)` for total and alphabetized per-country counts; and
- `render_manifest(groups)` for stable text rendering and line-ending hygiene.

No mocks or internal seams are used. Expected values are literal examples derived directly from
the task contract.

## Baseline

Command:

```text
cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest discover -s tests -v
```

Observed before changes: `Ran 4 tests`; `OK`.

## Retained red-green cycles

### C01 - surrounding city whitespace

- Authority: `../TASK.md`, behavior 1.
- Test: `tests/test_normalization.py` —
  `NormalizeDestinationTests.test_trims_surrounding_city_whitespace`.
- Public entry point: `routebook.normalize_destination`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_city_whitespace -v
  ```

- Intended failure: the legacy result retains surrounding spaces and a tab.
- Observed red: assertion failure; `('  New York \t', 'US') != ('New York', 'US')`.
- Minimal production change: strip the city at the normalization seam.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_trims_surrounding_city_whitespace -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C02 - internal city whitespace

- Authority: `../TASK.md`, behavior 2.
- Test: `tests/test_normalization.py` —
  `NormalizeDestinationTests.test_collapses_internal_city_whitespace`.
- Public entry point: `routebook.normalize_destination`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace -v
  ```

- Intended failure: stripping alone leaves the internal tab-and-space run intact.
- Observed red: assertion failure; `('New \t  York', 'US') != ('New York', 'US')`.
- Minimal production change: rebuild the city from whitespace-delimited words with one space.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_collapses_internal_city_whitespace -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C03 - two-letter country code casing

- Authority: `../TASK.md`, behavior 3.
- Test: `tests/test_normalization.py` —
  `NormalizeDestinationTests.test_uppercases_two_letter_country_code`.
- Public entry point: `routebook.normalize_destination`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v
  ```

- Intended failure: the legacy result returns lowercase `es`.
- Observed red: assertion failure; `('Madrid', 'es') != ('Madrid', 'ES')`.
- Minimal production change: uppercase the country code in the normalized tuple.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_uppercases_two_letter_country_code -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C04 - blank city refusal

- Authority: `../TASK.md`, behavior 4.
- Test: `tests/test_normalization.py` —
  `NormalizeDestinationTests.test_rejects_blank_city`.
- Public entry point: `routebook.normalize_destination`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v
  ```

- Intended failure: a whitespace-only city normalizes to an empty string without refusal.
- Observed red: assertion failure; `ValueError not raised`.
- Minimal production change: raise `ValueError` when the normalized city is empty.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_normalization.NormalizeDestinationTests.test_rejects_blank_city -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C05 - normalized country grouping

- Authority: `../TASK.md`, behavior 5.
- Test: `tests/test_grouping.py` —
  `GroupRoutesTests.test_groups_by_normalized_country_code`.
- Public entry point: `routebook.group_routes`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_grouping.GroupRoutesTests.test_groups_by_normalized_country_code -v
  ```

- Intended failure: the legacy group key and copied route retain lowercase `es`.
- Observed red: assertion failure; the result used key/country `es` instead of `ES`.
- Minimal production change: normalize each destination and place a normalized route copy under
  the normalized country key.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_grouping.GroupRoutesTests.test_groups_by_normalized_country_code -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C06 - route ordering

- Authority: `../TASK.md`, behavior 6.
- Test: `tests/test_grouping.py` —
  `GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id`.
- Public entry point: `routebook.group_routes`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v
  ```

- Intended failure: insertion order differs from normalized-city/route-ID order.
- Observed red: assertion failure; `['r2', 'r9', 'r1'] != ['r1', 'r9', 'r2']`.
- Minimal production change: sort each finished group by `(route["city"], route["id"])`.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_grouping.GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C07 - duplicate route IDs

- Authority: `../TASK.md`, behavior 7.
- Test: `tests/test_grouping.py` — `GroupRoutesTests.test_rejects_duplicate_route_ids`.
- Public entry point: `routebook.group_routes`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids -v
  ```

- Intended failure: identical IDs in different countries are both admitted.
- Observed red: assertion failure; `ValueError not raised`.
- Minimal production change: retain seen IDs for the invocation and reject a repeated exact ID.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_grouping.GroupRoutesTests.test_rejects_duplicate_route_ids -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C08 - alphabetized country counts

- Authority: `../TASK.md`, behavior 10.
- Test: `tests/test_stats.py` —
  `ManifestStatsTests.test_reports_country_counts_in_alphabetical_order`.
- Public entry point: `routebook.manifest_stats`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order -v
  ```

- Intended failure: the legacy statistics lack per-country counts.
- Observed red: error; `KeyError: 'countries'`.
- Minimal production change: add a `countries` mapping built by iterating sorted country keys.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_country_counts_in_alphabetical_order -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C09 - stable country and route lines

- Authority: `../TASK.md`, behavior 11.
- Test: `tests/test_rendering.py` —
  `RenderManifestTests.test_emits_countries_and_routes_in_stable_order`.
- Public entry point: `routebook.render_manifest`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_order -v
  ```

- Intended failure: the legacy debug representation is one unstable mapping line.
- Observed red: assertion failure; one `repr` line differed from the five required manifest lines.
- Minimal production change: emit sorted country headers and route lines sorted by city then ID.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_rendering.RenderManifestTests.test_emits_countries_and_routes_in_stable_order -v
  ```

- Observed green: `Ran 1 test`; `OK`.

### C10 - terminal newline and horizontal whitespace hygiene

- Authority: `../TASK.md`, behavior 12.
- Test: `tests/test_rendering.py` —
  `RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace`.
- Public entry point: `routebook.render_manifest`.
- Red command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace -v
  ```

- Intended failure: rendered route data retains trailing spaces/tabs and has no terminal newline.
- Observed red: assertion failure; `'ES:\n  r1: Madrid \t' != 'ES:\n  r1: Madrid\n'`.
- Minimal production change: strip trailing horizontal whitespace from route lines and append one
  terminal newline.
- Green command:

  ```text
  cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_rendering.RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace -v
  ```

- Observed green: `Ran 1 test`; `OK`.

## Final changed-test-to-seam reconciliation

The inventory below is derived at exact unittest method identity. A replaced legacy method appears
once as a removed identity and once as an added identity.

| Change | File and exact identity | Public entry point | Seam authority | Custody or disposition |
|---|---|---|---|---|
| Removed | `tests/test_normalization.py::NormalizeDestinationTests.test_legacy_preserves_destination_spelling` | `normalize_destination` | `../TASK.md`, behaviors 1-3 | Non-TDD: removed obsolete legacy expectation that directly contradicted the v2 contract. |
| Added | `tests/test_normalization.py::NormalizeDestinationTests.test_trims_surrounding_city_whitespace` | `normalize_destination` | `../TASK.md`, behavior 1 | TDD C01. |
| Added | `tests/test_normalization.py::NormalizeDestinationTests.test_collapses_internal_city_whitespace` | `normalize_destination` | `../TASK.md`, behavior 2 | TDD C02. |
| Added | `tests/test_normalization.py::NormalizeDestinationTests.test_uppercases_two_letter_country_code` | `normalize_destination` | `../TASK.md`, behavior 3 | TDD C03. |
| Added | `tests/test_normalization.py::NormalizeDestinationTests.test_rejects_blank_city` | `normalize_destination` | `../TASK.md`, behavior 4 | TDD C04. |
| Removed | `tests/test_grouping.py::GroupRoutesTests.test_legacy_groups_by_original_country` | `group_routes` | `../TASK.md`, behavior 5 | Non-TDD: removed obsolete legacy expectation that directly contradicted normalized grouping. |
| Added | `tests/test_grouping.py::GroupRoutesTests.test_groups_by_normalized_country_code` | `group_routes` | `../TASK.md`, behavior 5 | TDD C05. |
| Added | `tests/test_grouping.py::GroupRoutesTests.test_sorts_each_group_by_normalized_city_then_route_id` | `group_routes` | `../TASK.md`, behavior 6 | TDD C06. |
| Added | `tests/test_grouping.py::GroupRoutesTests.test_rejects_duplicate_route_ids` | `group_routes` | `../TASK.md`, behavior 7 | TDD C07. |
| Added | `tests/test_grouping.py::GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city` | `group_routes` | `../TASK.md`, behavior 8 | Non-TDD: focused first run passed; retained as public regression coverage. Command: `cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_grouping.GroupRoutesTests.test_preserves_distinct_route_ids_for_same_city -v`; observed `Ran 1 test`; `OK`. |
| Removed | `tests/test_stats.py::ManifestStatsTests.test_legacy_reports_total` | `manifest_stats` | `../TASK.md`, behavior 9 | Non-TDD: renamed and narrowed from an obsolete exact legacy mapping assertion. |
| Added | `tests/test_stats.py::ManifestStatsTests.test_reports_total_route_count` | `manifest_stats` | `../TASK.md`, behavior 9 | Non-TDD: focused first run passed; retained existing total behavior without claiming red. Command: `cd /tmp/tdd-evolution-rev-3e39-candidate-2/work && python3 -m unittest tests.test_stats.ManifestStatsTests.test_reports_total_route_count -v`; observed `Ran 1 test`; `OK`. |
| Added | `tests/test_stats.py::ManifestStatsTests.test_reports_country_counts_in_alphabetical_order` | `manifest_stats` | `../TASK.md`, behavior 10 | TDD C08. |
| Removed | `tests/test_rendering.py::RenderManifestTests.test_legacy_uses_debug_representation` | `render_manifest` | `../TASK.md`, behavior 11 | Non-TDD: removed obsolete legacy expectation that directly contradicted stable manifest lines. |
| Added | `tests/test_rendering.py::RenderManifestTests.test_emits_countries_and_routes_in_stable_order` | `render_manifest` | `../TASK.md`, behavior 11 | TDD C09. |
| Added | `tests/test_rendering.py::RenderManifestTests.test_ends_in_one_newline_without_trailing_horizontal_whitespace` | `render_manifest` | `../TASK.md`, behavior 12 | TDD C10. |

### Bidirectional reconciliation audit

- Changed-selector inventory minus reconciliation identities: empty.
- Reconciliation identities minus changed-selector inventory: empty.
- TDD evidence IDs minus TDD-custody links: empty (`C01` through `C10` all linked once).
- TDD-custody links minus TDD evidence IDs: empty.
- Every TDD row contains authority, exact selector, public entry point, distinct complete red and
  green command positions, intended and observed red, minimal production delta, and observed green.

## Self-review

One post-green review checked each of the twelve task behaviors against its public implementation
and test. All twelve were present. The review found four stale production docstrings that still
described legacy behavior; those docstrings were corrected. This repair changes no observable
behavior or test identity, so TDD re-entry was not applicable. The selector reconciliation above
remains unchanged.
