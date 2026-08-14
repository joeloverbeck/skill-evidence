# TDD Evidence

## Scope and seam authority

- Scoped change: `work/change.patch`
- Execution record: `work/work.log`
- Seam authority: `work/authority.md`, the owner-accepted contract for `Slug.parse(str) -> Slug`
- Observed public entry point for every cycle: `Slug.parse(str) -> Slug`

## Observed red-green cycles

### TDD-1 — trim surrounding whitespace

- Seam authority: `work/authority.md`: surrounding whitespace is ignored by `Slug.parse(str) -> Slug`.
- Test identity: `tests/test_slug.py` — `SlugTests.test_parse_trims_whitespace`
- Observed public entry point: `Slug.parse(str) -> Slug`
- Red command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v`
- Intended failure: expected `"hello"`.
- Observed failure: observed `"  hello  "`.
- Green command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v`
- Green result: `OK`

### TDD-2 — lowercase ASCII letters

- Seam authority: `work/authority.md`: ASCII letters are lowercased by `Slug.parse(str) -> Slug`.
- Test identity: `tests/test_slug.py` — `SlugTests.test_parse_lowercases_ascii`
- Observed public entry point: `Slug.parse(str) -> Slug`
- Red command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v`
- Intended failure: expected `"hello"`.
- Observed failure: observed `"HELLO"`.
- Green command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v`
- Green result: `OK`

### TDD-3 — refuse an empty result

- Seam authority: `work/authority.md`: an empty result is refused with `ValueError("empty slug")` by `Slug.parse(str) -> Slug`.
- Test identity: `tests/test_slug.py` — `SlugTests.test_parse_refuses_empty`
- Observed public entry point: `Slug.parse(str) -> Slug`
- Red command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v`
- Intended failure: `ValueError("empty slug")`.
- Observed failure: no exception.
- Green command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v`
- Green result: `OK`

## Changed-test inventory and reconciliation

| Change kind | File | Exact identity | Public entry point | Seam authority | Disposition |
|---|---|---|---|---|---|
| Added | `tests/test_slug.py` | `SlugTests.test_parse_trims_whitespace` | `Slug.parse(str) -> Slug` | `work/authority.md`: surrounding whitespace is ignored | TDD custody: TDD-1 |
| Added | `tests/test_slug.py` | `SlugTests.test_parse_lowercases_ascii` | `Slug.parse(str) -> Slug` | `work/authority.md`: ASCII letters are lowercased | TDD custody: TDD-2 |
| Added | `tests/test_slug.py` | `SlugTests.test_parse_refuses_empty` | `Slug.parse(str) -> Slug` | `work/authority.md`: empty result is refused with `ValueError("empty slug")` | TDD custody: TDD-3 |

Bidirectional reconciliation:

- Changed-test inventory minus reconciliation: empty.
- Reconciliation minus changed-test inventory: empty.
- TDD evidence identities minus TDD-custody reconciliation identities: empty.
- TDD-custody reconciliation identities minus TDD evidence identities: empty.

## Aggregate verification

- Command: `python3 -m unittest discover -s tests -v`
- Recorded result: `Ran 3 tests; OK`

## Review readiness

All three changed test identities have focused observed red and green commands, ratified public-seam authority, and one-to-one TDD custody. The complete changed-test inventory and TDD evidence reconcile in both directions with empty differences. The recorded aggregate suite result is green, so this scoped change is ready for code review.
