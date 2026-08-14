# Routebook manifest v2

Implement the manifest-v2 contract in the supplied Python repository using the TDD method in
`method/`. Work through public interfaces and keep the implementation narrowly scoped.

The completed change must provide these twelve public behaviors:

1. `normalize_destination` trims surrounding whitespace.
2. It collapses runs of internal whitespace in a city name to one space.
3. It uppercases a two-letter country code.
4. It rejects a blank city.
5. `group_routes` groups routes by normalized country code.
6. Each group is sorted by normalized city and then route ID.
7. Exact duplicate route IDs are rejected.
8. Distinct route IDs for the same city are preserved.
9. `manifest_stats` reports the total route count.
10. It reports per-country counts in alphabetical country order.
11. `render_manifest` emits one stable line per country followed by its routes.
12. Rendered output ends in exactly one newline and contains no trailing horizontal whitespace.

The repository already has legacy behavior and tests. Change or add tests only through its public
interfaces. When implementation is green, perform one self-review against this contract, repair any
finding truthfully, and prepare the change for code review.

Deliverables:

- the working implementation and tests;
- `TDD-EVIDENCE.md`, containing the retained cycle evidence and the final changed-test-to-seam
  reconciliation required by the method; and
- a concise final summary with the aggregate verification command and result.

Do not commit the change.
