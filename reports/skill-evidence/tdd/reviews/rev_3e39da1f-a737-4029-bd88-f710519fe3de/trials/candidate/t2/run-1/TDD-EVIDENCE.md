# TDD evidence

## Scope and authority

- Scoped change: `work/change.patch`
- Execution record: `work/work.log`
- Ratified public seam: `Slug.parse(str) -> Slug`
- Seam authority: `work/authority.md`, "Accepted contract"
- Required behavior: ignore surrounding whitespace, lowercase ASCII letters, and refuse an empty normalized result with `ValueError("empty slug")`.

## Observed red-green cycles

### TDD-1: surrounding whitespace is ignored

- Seam authority: `work/authority.md`, "Accepted contract"
- Test file: `tests/test_slug.py`
- Exact selector: `tests.test_slug.SlugTests.test_parse_trims_whitespace`
- Observed public entry point: `Slug.parse(str) -> Slug`
- Independent expected value: the authority requires `"  hello  "` to normalize to the literal `"hello"`, which differs from the observed untrimmed alternative `"  hello  "`.
- Red command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v`
- Intended failure: expected `"hello"`.
- Observed failure: received `"  hello  "`.
- Green command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_trims_whitespace -v`
- Green result: `OK`.

### TDD-2: ASCII letters are lowercased

- Seam authority: `work/authority.md`, "Accepted contract"
- Test file: `tests/test_slug.py`
- Exact selector: `tests.test_slug.SlugTests.test_parse_lowercases_ascii`
- Observed public entry point: `Slug.parse(str) -> Slug`
- Independent expected value: the authority requires `"HELLO"` to normalize to the literal `"hello"`, which differs from the observed uppercase alternative `"HELLO"`.
- Red command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v`
- Intended failure: expected `"hello"`.
- Observed failure: received `"HELLO"`.
- Green command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_lowercases_ascii -v`
- Green result: `OK`.

### TDD-3: an empty normalized result is refused

- Seam authority: `work/authority.md`, "Accepted contract"
- Test file: `tests/test_slug.py`
- Exact selector: `tests.test_slug.SlugTests.test_parse_refuses_empty`
- Observed public entry point: `Slug.parse(str) -> Slug`
- Independent expected value: the authority requires whitespace-only input to raise `ValueError("empty slug")`; accepting it and returning normally is the rejected alternative.
- Red command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v`
- Intended failure: `ValueError("empty slug")` must be raised.
- Observed failure: no exception was raised.
- Green command: `python3 -m unittest tests.test_slug.SlugTests.test_parse_refuses_empty -v`
- Green result: `OK`.

## Aggregate result

- Command: `python3 -m unittest discover -s tests -v`
- Result: `Ran 3 tests; OK`.

## Changed-test inventory and reconciliation

The inventory below covers every added, modified, renamed, or removed exact test identity in `work/change.patch`.

| Change | File | Exact identity | Public entry point | Seam authority | Disposition |
| --- | --- | --- | --- | --- | --- |
| Added | `tests/test_slug.py` | `tests.test_slug.SlugTests.test_parse_trims_whitespace` | `Slug.parse(str) -> Slug` | `work/authority.md`, "Accepted contract" | TDD custody: TDD-1 |
| Added | `tests/test_slug.py` | `tests.test_slug.SlugTests.test_parse_lowercases_ascii` | `Slug.parse(str) -> Slug` | `work/authority.md`, "Accepted contract" | TDD custody: TDD-2 |
| Added | `tests/test_slug.py` | `tests.test_slug.SlugTests.test_parse_refuses_empty` | `Slug.parse(str) -> Slug` | `work/authority.md`, "Accepted contract" | TDD custody: TDD-3 |

Two-way reconciliation:

- Changed-test inventory minus reconciliation identities: empty.
- Reconciliation identities minus changed-test inventory: empty.
- TDD evidence identities minus TDD-custody reconciliation identities: empty.
- TDD-custody reconciliation identities minus TDD evidence identities: empty.

## Review readiness

All three changed test identities have ratified public-seam authority, complete and separately stated red and green commands, observed red failures, green results, and exact TDD-custody links. The aggregate suite is recorded green, and both required bidirectional identity comparisons are empty. This scoped change is ready for code review.
