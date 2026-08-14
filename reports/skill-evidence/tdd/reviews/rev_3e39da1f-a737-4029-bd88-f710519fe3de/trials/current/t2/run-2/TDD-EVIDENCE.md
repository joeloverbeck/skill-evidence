# TDD evidence

## Agreed seam and authority

The owner-accepted public seam is `Slug.parse(str) -> Slug` (`work/authority.md:3`). The accepted contract requires surrounding whitespace to be ignored, ASCII letters to be lowercased, and an empty normalized result to be refused with `ValueError("empty slug")` (`work/authority.md:5-7`).

## Red-green cycles

| Cycle | Behavior | Test file and exact selector | Observed public entry point | Red command | Intended and observed red | Green command | Green result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Surrounding whitespace is ignored | `tests/test_slug.py` — `tests.test_slug.SlugTests.test_parse_trims_whitespace` | `Slug.parse("  hello  ")` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v` | Intended `"hello"`; observed `"  hello  "` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v` | `OK` |
| 2 | ASCII letters are lowercased | `tests/test_slug.py` — `tests.test_slug.SlugTests.test_parse_lowercases_ascii` | `Slug.parse("HELLO")` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v` | Intended `"hello"`; observed `"HELLO"` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v` | `OK` |
| 3 | An empty normalized result is refused | `tests/test_slug.py` — `tests.test_slug.SlugTests.test_parse_refuses_empty` | `Slug.parse("   ")` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v` | Intended `ValueError("empty slug")`; observed no exception | `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v` | `OK` |

All three tests exercise the owner-accepted public seam directly. Their expected values and refusal are fixed by the accepted contract rather than recomputed from the implementation.

## Changed-test inventory

Derived from the scoped test diff in `work/change.patch`:

- `tests.test_slug.SlugTests.test_parse_trims_whitespace`
- `tests.test_slug.SlugTests.test_parse_lowercases_ascii`
- `tests.test_slug.SlugTests.test_parse_refuses_empty`

## Reconciliation

| Changed test identity | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- |
| `tests.test_slug.SlugTests.test_parse_trims_whitespace` | `Slug.parse("  hello  ")` | `work/authority.md:3,5` | Cycle 1 |
| `tests.test_slug.SlugTests.test_parse_lowercases_ascii` | `Slug.parse("HELLO")` | `work/authority.md:3,6` | Cycle 2 |
| `tests.test_slug.SlugTests.test_parse_refuses_empty` | `Slug.parse("   ")` | `work/authority.md:3,7` | Cycle 3 |

Let `I` be the changed-test inventory, `R` the reconciliation identities, and `E` the red-green evidence identities above.

- Inventory without reconciliation: `I ∖ R = ∅`
- Reconciliation without inventory: `R ∖ I = ∅`
- Evidence without reconciliation: `E ∖ R = ∅`
- Reconciliation without evidence: `R ∖ E = ∅`

## Aggregate verification and review readiness

- Command: `python3 -m unittest discover -s tests -v`
- Result: `Ran 3 tests; OK`
- Review readiness: all three changed test identities are reconciled one-to-one with an authorized public seam and a retained red-green cycle; both-direction identity differences are empty, and the recorded aggregate run is green.
