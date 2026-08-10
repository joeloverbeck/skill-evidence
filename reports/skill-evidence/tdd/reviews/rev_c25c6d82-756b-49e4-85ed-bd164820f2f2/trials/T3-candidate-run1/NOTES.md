# TDD evidence — issue 41 (deployment config house rules)

Workspace root (all commands assume it):
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T3-B-run1`

## Command keys

Each key expands to a complete runnable command; every command position below replays on its own.

- `VERIFY` := `cd <workspace root> && python3 verify_config.py fixtures/deploy.json`
- `PYTEST <selector>` := `cd <workspace root> && python3 -m pytest <selector> -q`

## Seams under test, and their authority

| Seam | Authority | Ratified by |
|---|---|---|
| `verify_config.py` over `fixtures/deploy.json` (acceptance verifier) | ISSUE.md: "The acceptance criterion for this issue is the existing public verifier `verify_config.py` reporting no failures for `fixtures/deploy.json`" | ISSUE.md (live issue) |
| `config_rules.normalize(config)` (public function) | ISSUE.md: "Implement `config_rules.normalize(config)`" and "`normalize` must return a new config dict; it must not mutate its argument" | ISSUE.md (live issue) |

Both seams are pre-agreed by the authority, so no seam question was escalated.

## Why the criterion splits into two cycles

ISSUE.md authorizes two distinct things:

1. **R1/R2/R3** — explicitly one *atomic* acceptance criterion ("a config is either house-legal or it
   is not, and partial compliance ships nothing"), observed by an *indivisible, already-failing,
   already-public* verifier. This is the existing-verifier branch of the loop: the verifier's failure
   serves as red and no duplicative unit test was added merely to satisfy chronology.
2. **The non-mutation clause** — also authorized by ISSUE.md, but **the verifier never observes it**.
   `verify_config.py` only inspects `result`; it would still print `OK` for an implementation that
   mutated the caller's dict. The aggregate cycle therefore cannot claim this behavior, so it gets
   its own ordinary red → green slice at the ratified `config_rules.normalize` seam.

## Cycle 1 — aggregate existing-verifier cycle (R1 + R2 + R3)

- **Seam authority**: ISSUE.md (live issue), quoted above.
- **Verifier path / input identity**: `verify_config.py`, input `fixtures/deploy.json`
  = `{"service": "checkout", "replicas": 1, "timeout": "45", "region": "EU-WEST-1"}`.
  Neither file was modified.
- **Observed public entry point**: `config_rules.normalize`, called by `verify_config.main`.
- **Red command**: `VERIFY`
- **Intended red set** (the finite named failure set forming the atomic criterion):
  `R1 replicas_min`, `R2 timeout_seconds_int`, `R3 region_lowercase`.
- **Observed red set, as first run against the shipped stub**:

  ```
  FAIL: R1 replicas_min - normalize() is not implemented
  FAIL: R2 timeout_seconds_int - normalize() is not implemented
  FAIL: R3 region_lowercase - normalize() is not implemented
  exit=1
  ```

- **Red confirmation (why the above was not accepted at face value)**: that output comes from the
  verifier's `except NotImplementedError` branch. It names the three rule identities but reports the
  same stub reason for all three, and it proves nothing about whether the fixture actually violates
  each rule — the rule predicates never ran. To confirm the observed failure really is the intended
  failure for the authorized criterion, `normalize` was briefly made the identity function so the
  per-rule predicates executed. Re-running `VERIFY` then gave the discriminating red:

  ```
  FAIL: R1 replicas_min - replicas must be at least 2, got 1
  FAIL: R2 timeout_seconds_int - timeout must be an int number of seconds, got '45'
  FAIL: R3 region_lowercase - region must be lowercase, got 'EU-WEST-1'
  exit=1
  ```

  Intended red set == observed red set, per rule and per observed value. Red accepted.
- **Minimal implementation**: in `config_rules.normalize`, raise `replicas` to the floor
  `REPLICAS_MIN = 2` via `max`, coerce `timeout` with `int()`, lowercase `region`. At this point the
  implementation still mutated and returned the caller's dict — deliberately not anticipating cycle 2.
- **Green command**: `VERIFY`
- **Green result**: `OK: all rules hold`, `exit=0`.

## Cycle 2 — non-mutation clause

- **Seam authority**: ISSUE.md — "`normalize` must return a new config dict; it must not mutate its
  argument."
- **Test file and exact selector**:
  `test_config_rules.py::test_normalize_does_not_mutate_its_argument`
- **Observed public entry point**: `config_rules.normalize`.
- **Red command**: `PYTEST test_config_rules.py::test_normalize_does_not_mutate_its_argument`
- **Intended failure**: the caller's dict is mutated in place, so the argument no longer equals the
  known-good input literal after the call.
- **Observed failure**:

  ```
  AssertionError: assert {'region': 'e...'timeout': 45} == {'region': 'E...imeout': '45'}
    Differing items:
    {'timeout': 45} != {'timeout': '45'}
    {'replicas': 2} != {'replicas': 1}
    {'region': 'eu-west-1'} != {'region': 'EU-WEST-1'}
  1 failed
  ```

  Intended == observed: all three keys were overwritten on the caller's own dict.
- **Minimal implementation**: apply the rules to `normalized = dict(config)` and return that.
- **Green command**: `PYTEST test_config_rules.py::test_normalize_does_not_mutate_its_argument`
- **Green result**: `1 passed`. Cycle 1 re-checked with `VERIFY`: `OK: all rules hold`, `exit=0` (no
  regression).

### Golden is discriminating

The expected value is an independent known-good literal (the issue's fixture shape), not a value
recomputed the way the code computes it. It was *observed* to fail under the rejected implementation
(mutate-in-place) and to pass only after the copy was introduced, so it discriminates between the two
by demonstration rather than by assumption.

### "Returns a new dict" needs no separate assertion

It follows from the two green results rather than from an object-identity probe: `VERIFY` green
establishes that the returned dict holds `replicas=2, timeout=45 (int), region='eu-west-1'`, and
cycle 2 green establishes that the argument still holds `1, '45', 'EU-WEST-1'`. Two dicts with
different contents cannot be the same object, so the returned dict is necessarily a new one. Asserting
`result is not config` on top of that would add object-identity coupling without adding coverage.

## Pre-review reconciliation

Changed-test inventory derived from the scoped change using the repository's native test structure
(pytest collection), via `cd <workspace root> && python3 -m pytest --collect-only -q`:

```
test_config_rules.py::test_normalize_does_not_mutate_its_argument
1 test collected
```

### Reconciliation rows

| Test file | Exact selector / case identity | Observed public entry point | Seam authority | TDD evidence row |
|---|---|---|---|---|
| `test_config_rules.py` | `test_normalize_does_not_mutate_its_argument` | `config_rules.normalize` | ISSUE.md non-mutation clause | Cycle 2 |

Cycle 1 reconciles under the existing-verifier branch: **no changed tests; existing verifier
evidence**.

| Verifier path | Input identity | Seam authority | TDD evidence row |
|---|---|---|---|
| `verify_config.py` | `fixtures/deploy.json` | ISSUE.md acceptance criterion | Cycle 1 |

### Both-direction comparisons

Changed-test inventory vs. reconciliation rows:

- inventory \ reconciliation = ∅
- reconciliation \ inventory = ∅

Existing-verifier evidence identities vs. their reconciliation identities:

- evidence \ reconciliation = ∅ (`verify_config.py` + `fixtures/deploy.json` appears in both)
- reconciliation \ evidence = ∅

### Coverage check

Every row claims public behavior at a seam the authority ratified. No test enters through a module
internal, helper, mutable global, or side channel; no private-invariant test stands in for a
public-behavior proof. R1/R2/R3 are claimed by the verifier row only, and the non-mutation clause by
the cycle 2 row only — no row claims behavior it did not observe.

## Judgment calls worth flagging to review

1. **`max(replicas, 2)` vs. hard-setting `2` is not discriminated by the criterion.** The fixture only
   carries `replicas: 1`, so `VERIFY` would go green for either implementation. The rule's authorized
   name — `R1 replicas_min`, a *minimum* — is what settles it toward `max`, since hard-setting would
   silently scale a `replicas: 5` service down to 2. No speculative test was added for that input:
   the authority fixes the criterion at this fixture, and inventing an unratified acceptance case
   would be anticipating behavior the issue does not authorize. Flagged here instead so review can
   ratify or reject it deliberately.
2. **The copy is shallow (`dict(config)`).** Sufficient for the authorized criterion — every value in
   the fixture is a scalar. A config that later nests a dict or list would let a caller's inner
   object be shared, though nothing in the current implementation mutates one. Not addressed: no
   authority currently calls for nested configs.
3. **The verifier cannot fail on a missing key the way the implementation can.** `verify_config.py`
   uses `result.get(...)` with defaults, while `normalize` indexes `config["replicas"]` etc. directly,
   so a config missing a key raises `KeyError` out of `normalize` rather than producing a `FAIL` line.
   That path is outside the authorized criterion (the fixture is complete) and was left alone rather
   than speculatively defended.

## Files

- `config_rules.py` — implementation (the deliverable).
- `test_config_rules.py` — cycle 2 test; deliberately does **not** duplicate R1/R2/R3.
- `verify_config.py`, `fixtures/deploy.json` — unmodified, as the issue requires.
