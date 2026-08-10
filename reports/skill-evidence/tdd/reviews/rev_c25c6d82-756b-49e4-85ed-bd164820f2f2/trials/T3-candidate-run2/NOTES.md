# Issue 41 — TDD evidence

## Stable keys

`WS` = `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T3-B-run2`

Every command below is written out in full and replays standalone from any working directory
(`verify_config.py` puts its own directory on `sys.path`, so `import config_rules` resolves; pytest
prepends the test file's directory for the same reason). Environment: Python 3.14.4, pytest 9.0.2.

## Seams under test, written down before any test

| Seam | Authority | Status |
| --- | --- | --- |
| `config_rules.normalize(config)` — public function | ISSUE.md, Issue 41: "Implement `config_rules.normalize(config)`" | Ratified by a live issue; already agreed, no user question needed |
| `verify_config.py <fixture>` — public acceptance verifier (CLI: stdout rule lines + exit code) | ISSUE.md: "The acceptance criterion for this issue is the existing public verifier `verify_config.py` reporting no failures for `fixtures/deploy.json`" | Ratified by the same issue |

ISSUE.md is self-declared authority ("**Authority: this issue.**") and is the only authority
consulted. No ADR, PRD, or user decision was available or needed.

## Why two cycles, and why in this order

The issue states two separable requirements:

1. R1/R2/R3 — covered by the existing verifier, which the issue declares indivisible and one atomic
   acceptance criterion.
2. "`normalize` must return a new config dict; it must not mutate its argument." — **the verifier
   never observes this.** It inspects only `result`, so an in-place implementation returning its own
   argument reports `OK`. The aggregate verifier evidence therefore cannot carry this behavior, and
   it gets its own focused slice.

Slice A (non-mutation) was run **before** slice B (rules) for two reasons, both of which improve the
evidence rather than merely reordering it:

- Slice A's minimal green (`return dict(config)`) contains no rule logic, so slice B's red was still
  genuinely red.
- It converted slice B's red from the stub-shaped failure into a **data-driven** one (see below).
  Doing slice B first would have forced slice A's red to be manufactured by deliberately writing an
  in-place implementation that the authority already forbids.

### The pre-existing red was not used as the red of record

Before any code, the verifier printed:

```
FAIL: R1 replicas_min - normalize() is not implemented
FAIL: R2 timeout_seconds_int - normalize() is not implemented
FAIL: R3 region_lowercase - normalize() is not implemented
```

The method requires confirming that the verifier's observed failure *is the intended failure for the
authorized criterion*. This one is not: it comes from the `except NotImplementedError` branch, which
prints those three lines for **any** input, including a fully house-legal one. It names the rules
without evaluating them. It is recorded here as the baseline, and slice B's red of record is the
data-driven failure set observed once `normalize` returned a value, which cites the fixture's actual
offending values and so genuinely pins the criterion to this fixture.

## Cycle A — normalize returns a new config dict and does not mutate its argument

| Field | Value |
| --- | --- |
| Seam authority | ISSUE.md, Issue 41: "`normalize` must return a new config dict; it must not mutate its argument." |
| Test file | `WS/test_config_rules.py` |
| Exact selector | `test_config_rules.py::test_normalize_returns_new_config_without_mutating_argument` |
| Observed public entry point | `config_rules.normalize(config)` called directly; observed via the returned object's identity and the caller's own dict |
| Red command | `python3 -m pytest -q "$WS/test_config_rules.py::test_normalize_returns_new_config_without_mutating_argument"` |
| Intended red | The unimplemented seam refuses the call — no new dict is returned |
| Observed red | `NotImplementedError` raised at `config_rules.py:3`; `1 failed`, exit 1 |
| Minimal implementation | `config_rules.normalize` body changed from `raise NotImplementedError` to `return dict(config)` — no rule logic |
| Green command | `python3 -m pytest -q "$WS/test_config_rules.py::test_normalize_returns_new_config_without_mutating_argument"` |
| Green result | `1 passed`, exit 0 |

## Cycle B — aggregate cycle against the existing authoritative verifier

The method's aggregate branch is used here, and each of its three preconditions holds:

- **Indivisible verifier** — `verify_config.py` exposes no per-rule selector. It takes one fixture
  path, evaluates all three rules, and prints every failure in one run.
- **Finite named failure set** — exactly `R1 replicas_min`, `R2 timeout_seconds_int`,
  `R3 region_lowercase`.
- **One atomic acceptance criterion** — ratified verbatim by the authority: "one atomic acceptance
  criterion: a config is either house-legal or it is not, and partial compliance ships nothing."

Because the verifier supports no focus, no per-rule cycle was available. Per the method, this row
names the verifier and input identity in place of a test file and selector, and **no duplicative
unit tests for R1/R2/R3 were written merely to satisfy chronology**.

| Field | Value |
| --- | --- |
| Seam authority | ISSUE.md, Issue 41 (acceptance criterion = this verifier reporting no failures for this fixture) |
| Verifier path | `WS/verify_config.py` (unmodified) |
| Complete command | `python3 $WS/verify_config.py $WS/fixtures/deploy.json` |
| Input identity | `WS/fixtures/deploy.json` (unmodified), sha256 `72942e52fc595d409da27949c071ae1ca21052f8f89b2a7841847079345dacce`, content `{"service": "checkout", "replicas": 1, "timeout": "45", "region": "EU-WEST-1"}` |
| Observed public entry point | The verifier CLI process — stdout rule lines plus exit code |
| Red command | `python3 $WS/verify_config.py $WS/fixtures/deploy.json` |

Intended red set (all three rules failing on this fixture's data: `replicas` 1 is below the floor,
`timeout` `"45"` is a str not an int, `region` `"EU-WEST-1"` is not lowercase) — and the observed red
set was identical, character for character, exit 1:

```
FAIL: R1 replicas_min - replicas must be at least 2, got 1
FAIL: R2 timeout_seconds_int - timeout must be an int number of seconds, got '45'
FAIL: R3 region_lowercase - region must be lowercase, got 'EU-WEST-1'
```

| Field | Value |
| --- | --- |
| Minimal implementation | On the copy already returned by cycle A: `result["replicas"] = max(result["replicas"], REPLICAS_MIN)`; `result["timeout"] = int(result["timeout"])`; `result["region"] = result["region"].lower()` |
| Green command | `python3 $WS/verify_config.py $WS/fixtures/deploy.json` |
| Green result | `OK: all rules hold`, exit 0 |

Regression check after cycle B (cycle A's behavior is not observable to the verifier, so it was
re-run explicitly): `python3 -m pytest -q $WS` → `1 passed`, exit 0.

## Pre-review reconciliation

Git is forbidden in this environment, so the scoped change set was enumerated from the workspace
inventory taken before any edit. Files changed: `config_rules.py` (production) and
`test_config_rules.py` (new, the only test file). `verify_config.py` and `fixtures/deploy.json` were
not modified — confirmed by the fixture sha256 above and by the verifier's unchanged message text.

Changed-test inventory, derived with the repository's native test structure
(`python3 -m pytest --collect-only -q $WS`):

1. `test_config_rules.py::test_normalize_returns_new_config_without_mutating_argument`

Reconciliation rows, one per changed test selector:

| Test file | Exact selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `WS/test_config_rules.py` | `test_normalize_returns_new_config_without_mutating_argument` | `config_rules.normalize(config)` | ISSUE.md, Issue 41 ("must return a new config dict; it must not mutate its argument") | Cycle A |

Both-direction comparison — inventory vs reconciliation:

- In inventory, absent from reconciliation: **none**.
- In reconciliation, absent from inventory: **none**.

Coverage check: the row's test enters through the ratified public function `config_rules.normalize`
— no module internal, helper, mutable global, or side channel — and the public behavior it claims
(a new dict is returned, the caller's dict is untouched) is exactly the behavior it observes. No
private-invariant test is standing in for a public-behavior proof.

Aggregate-branch reconciliation for cycle B: **no changed tests; existing verifier evidence.**
Both-direction comparison of the existing-verifier evidence identities against their reconciliation
identities:

| Identity | Evidence row | Reconciliation |
| --- | --- | --- |
| Verifier path | `WS/verify_config.py` | `WS/verify_config.py` |
| Complete command | `python3 $WS/verify_config.py $WS/fixtures/deploy.json` | `python3 $WS/verify_config.py $WS/fixtures/deploy.json` |
| Input identity | `WS/fixtures/deploy.json` sha256 `72942e52…dacce` | `WS/fixtures/deploy.json` sha256 `72942e52…dacce` |
| Seam authority | ISSUE.md, Issue 41 | ISSUE.md, Issue 41 |

- In evidence, absent from reconciliation: **none**.
- In reconciliation, absent from evidence: **none**.

## Anti-pattern and sensitivity checks

- **Not tautological.** The unit test's expected value is a literal dict written out independently,
  not recomputed from `config` after the call.
- **Discriminating input, verified not assumed.** With an already-compliant config, an in-place
  implementation would leave the caller's dict untouched and the test would pass against the
  forbidden behavior — so a rule-violating input is required. Substituting an in-place
  implementation in memory and re-running the test: it **fails**, as it must.
- **No mocks.** No internal collaborator is mocked; there is no system boundary here to mock.
- **Shallow copy is sufficient.** `dict(config)` plus top-level assignment never touches a nested
  value, so the argument is unmutated for any input shape, not just this fixture's.

## Flagged: the ratified criterion is under-discriminating for R1

I cannot modify the verifier or the fixture, so this is raised rather than fixed. The fixture's
`replicas: 1` does not distinguish a floor from a hardcode: substituting `result["replicas"] = 2`
in memory still yields `OK: all rules hold`, exit 0. That forbidden implementation would silently
downscale a 5-replica service, and the acceptance criterion would not catch it.

I implemented `max(result["replicas"], REPLICAS_MIN)`, matching the rule's name (`replicas_min`) and
message ("at least 2"), and confirmed it preserves `replicas: 5`. But **no ratified check pins that
behavior**, and I did not add a test for it: it passes against the current implementation on
arrival, so there is no red, and "preserve replicas above the minimum" is nowhere ratified by the
authority. Writing it would be a green-on-arrival characterization test, not a TDD slice, and would
leave a reconciliation row with no honest evidence row behind it. If the authority ratifies that
behavior, it needs a fixture or verifier change — which the issue forbids — and that should be a
new issue.

The same under-discrimination applies more weakly to R2 (`int("45")`, `round(float("45"))`, and
truncation all yield `45`, so the fixture fixes no rounding rule for fractional inputs) and R3
(`.lower()` and `.casefold()` agree on `"EU-WEST-1"`). Neither has a ratified requirement that would
tell those alternatives apart, so no behavior was invented for them.
