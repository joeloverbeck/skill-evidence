# TDD Evidence

## Seam authority

The owner-accepted contract in `authority.md` ratifies the public seam
`Slug.parse(str) -> Slug` for all three behaviors: surrounding whitespace is
ignored, ASCII letters are lowercased, and an empty result is refused with
`ValueError("empty slug")`.

## Red-green cycles

| Row | Behavior | Test file | Exact test selector | Observed public entry point | Seam authority | Red command | Intended and observed red | Green command | Green result |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TDD-1 | Surrounding whitespace is ignored | `tests/test_slug.py` | `tests.test_slug.SlugTests.test_parse_trims_whitespace` | `Slug.parse(str) -> Slug` | Owner-accepted contract in `authority.md`: surrounding whitespace is ignored | `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v` | Intended: expected `"hello"`; observed: `"  hello  "` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v` | `OK` |
| TDD-2 | ASCII letters are lowercased | `tests/test_slug.py` | `tests.test_slug.SlugTests.test_parse_lowercases_ascii` | `Slug.parse(str) -> Slug` | Owner-accepted contract in `authority.md`: ASCII letters are lowercased | `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v` | Intended: expected `"hello"`; observed: `"HELLO"` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v` | `OK` |
| TDD-3 | An empty result is refused | `tests/test_slug.py` | `tests.test_slug.SlugTests.test_parse_refuses_empty` | `Slug.parse(str) -> Slug` | Owner-accepted contract in `authority.md`: an empty result is refused with `ValueError("empty slug")` | `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v` | Intended: `ValueError("empty slug")`; observed: no exception | `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v` | `OK` |

## Aggregate verification

- Command: `python3 -m unittest discover -s tests -v`
- Result: `Ran 3 tests; OK`

## Changed-test inventory

Derived from the scoped patch using the repository's `unittest` test-method
structure:

1. `tests.test_slug.SlugTests.test_parse_trims_whitespace`
2. `tests.test_slug.SlugTests.test_parse_lowercases_ascii`
3. `tests.test_slug.SlugTests.test_parse_refuses_empty`

## Test-to-seam reconciliation

| Changed test identity | Test file | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `tests.test_slug.SlugTests.test_parse_trims_whitespace` | `tests/test_slug.py` | `Slug.parse(str) -> Slug` | Owner-accepted whitespace contract in `authority.md` | TDD-1 |
| `tests.test_slug.SlugTests.test_parse_lowercases_ascii` | `tests/test_slug.py` | `Slug.parse(str) -> Slug` | Owner-accepted ASCII-lowercasing contract in `authority.md` | TDD-2 |
| `tests.test_slug.SlugTests.test_parse_refuses_empty` | `tests/test_slug.py` | `Slug.parse(str) -> Slug` | Owner-accepted empty-result refusal contract in `authority.md` | TDD-3 |

Two-way identity comparison:

- Changed-test inventory minus reconciliation identities: empty.
- Reconciliation identities minus changed-test inventory: empty.
- TDD evidence identities minus reconciliation identities: empty.
- Reconciliation identities minus TDD evidence identities: empty.

## Review readiness

All three changed test identities have owner-ratified public-seam authority,
retained focused red and green commands, observed intended failures, successful
focused green results, and a successful aggregate result. Every changed test is
reconciled one-to-one with a TDD evidence row, with both identity differences
empty. The scoped handoff is ready for code review.
