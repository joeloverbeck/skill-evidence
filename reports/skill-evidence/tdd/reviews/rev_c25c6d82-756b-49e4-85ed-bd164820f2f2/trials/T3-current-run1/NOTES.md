# TDD evidence — Issue 41 (deployment config house rules)

All commands are run from the workspace root (the directory holding `verify_config.py`).

## Command keys

| Key | Command |
| --- | --- |
| `CMD-V` | `python3 verify_config.py fixtures/deploy.json` |
| `CMD-T1` | `python3 -m pytest test_config_rules.py::test_normalize_leaves_caller_config_unmutated -q` |
| `CMD-ALL` | `python3 -m pytest -q` |
| `CMD-COLLECT` | `python3 -m pytest --collect-only -q` |

## Seams under test

| Seam | Authority |
| --- | --- |
| `config_rules.normalize(config)` — return value, observed through `verify_config.py` | ISSUE.md (Issue 41): "**Authority: this issue.** The acceptance criterion ... is the existing public verifier `verify_config.py` reporting no failures for `fixtures/deploy.json`." |
| `config_rules.normalize(config)` — effect on the caller's argument | ISSUE.md (Issue 41): "`normalize` must return a new config dict; it must not mutate its argument." |

Both seams are pre-agreed by the issue, so no seam question was raised with the user.

## Cycle 1 — R1/R2/R3 (aggregate existing-verifier cycle)

This cycle takes the authoritative-verifier branch of the loop rules: the issue supplies an
already-failing public verifier, so its failure serves as red and no duplicative test was added
merely to satisfy chronology. The aggregate (rather than per-rule) form is used because the issue
declares the verifier indivisible and the three named rules "one atomic acceptance criterion".

| Field | Value |
| --- | --- |
| Verifier path | `verify_config.py` |
| Complete command | `CMD-V` = `python3 verify_config.py fixtures/deploy.json` |
| Input identity | `fixtures/deploy.json` — `{"service": "checkout", "replicas": 1, "timeout": "45", "region": "EU-WEST-1"}` |
| Seam authority | ISSUE.md, Issue 41 (acceptance criterion + "do not modify" clause) |
| Intended red set | `R1 replicas_min`, `R2 timeout_seconds_int`, `R3 region_lowercase` — the finite named failure set of the atomic criterion |
| Observed red set | Exactly those three, via the `NotImplementedError` arm: `FAIL: R1 replicas_min - normalize() is not implemented`, `FAIL: R2 timeout_seconds_int - normalize() is not implemented`, `FAIL: R3 region_lowercase - normalize() is not implemented`; exit 1 |
| Red confirmed as intended | Yes — observed set equals intended set, no extra or missing rule |
| Minimal implementation | `config_rules.normalize`: floor `replicas` at 2, coerce `timeout` with `int()`, lowercase `region`. Applied in place on the argument at this point — the copy was **not** written yet, so this cycle did not anticipate cycle 2. |
| Green command | `CMD-V` |
| Green result | `OK: all rules hold`; exit 0 |

## Cycle 2 — non-mutation of the caller's config

Not duplicative of cycle 1: `verify_config.py` only inspects `normalize`'s **return value** and never
looks at the caller's dict, so the cycle-1 row cannot carry this public-behavior proof. The issue
states the requirement as a separate normative sentence, so it gets its own slice.

| Field | Value |
| --- | --- |
| Seam authority | ISSUE.md, Issue 41: "`normalize` must return a new config dict; it must not mutate its argument." |
| Test file | `test_config_rules.py` |
| Exact selector | `test_config_rules.py::test_normalize_leaves_caller_config_unmutated` |
| Observed public entry point | `config_rules.normalize(config)` — called directly, argument inspected afterwards |
| Red command | `CMD-T1` |
| Intended failure | Argument dict mutated in place by the cycle-1 implementation |
| Observed failure | `AssertionError` on the argument assertion: `{'timeout': 30} != {'timeout': '30'}`, `{'region': 'us-east-2'} != {'region': 'US-EAST-2'}`, `{'replicas': 2} != {'replicas': 1}`; 1 failed |
| Minimal implementation | `normalized = dict(config)`; apply the three rules to the copy; return the copy |
| Green command | `CMD-T1` |
| Green result | 1 passed |
| Regression check | `CMD-V` still reports `OK: all rules hold`, exit 0; `CMD-ALL` reports 1 passed |

### Golden sensitivity (cycle 2)

The input requires all three rules to fire, so the test is discriminating in both directions: a no-op
`normalize` that returns its argument unchanged fails the return-value assertion, and a mutating
`normalize` fails the argument assertion (observed as the red above). Expected values are known-good
literals fixed by the issue's rules, not recomputed the way the implementation computes them.

## Pre-review reconciliation

Changed-test inventory derived from the scoped change using the repository's native test structure
(`CMD-COLLECT`), not from memory. Collected output: `test_config_rules.py::test_normalize_leaves_caller_config_unmutated` (1 test collected).

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `test_config_rules.py` | `test_normalize_leaves_caller_config_unmutated` | `config_rules.normalize(config)` | ISSUE.md, Issue 41 non-mutation sentence | Cycle 2 |

Inventory vs reconciliation, both directions:

- In inventory, missing from reconciliation: **none**.
- In reconciliation, missing from inventory: **none**.

Existing-verifier evidence identities vs their reconciliation identities, both directions:

- Evidence identities: verifier `verify_config.py`, input `fixtures/deploy.json`.
- Reconciliation identities: verifier `verify_config.py`, input `fixtures/deploy.json`.
- In evidence, missing from reconciliation: **none**. In reconciliation, missing from evidence: **none**.

No test enters through a module, helper, mutable global, or side channel that the authority did not
ratify: the only test calls the public `config_rules.normalize` named by the issue, and cycle 1 is
observed through the issue's own public verifier.

## Review re-entry

None. `/code-review` has not been run on this change, so there are no review-reentry rows and no
`Findings fixed during review` ledger to reconcile against.

## Open question for the author — R1 has no ratified golden above the floor

Flagging rather than silently resolving, because inventing an unratified golden would be my decision,
not the authority's.

`R1 replicas_min` is satisfied by at least two different rules, and `fixtures/deploy.json`
(`replicas: 1`) cannot tell them apart:

- **floor at 2** — `max(replicas, 2)`, so `replicas: 5` stays `5` (what I shipped);
- **clamp to 2** — assign `2` unconditionally, so `replicas: 5` is silently downgraded to `2`.

Measured, not assumed: with the clamp-to-2 alternative substituted, the fixture still satisfies all
three of the verifier's rule predicates, and `replicas: 5` comes back as `2`; under the shipped
implementation it comes back as `5`. So the acceptance fixture is **not discriminating** for this
distinction, and the difference is invisible to the criterion.

I chose the floor because the rule is named `replicas_min` and the verifier's own message says
"replicas must be at least 2", but the issue fixes no required result for a config that already
exceeds the minimum. I did not add a test for `replicas: 5`: that would pin behavior to my reading of
the rule name rather than to a ratified authority. If the floor is the intended rule, the issue (or a
fixture) should state the required result for an above-minimum config so a discriminating golden can
be written.

Two smaller observations about the criterion itself, noted only because they bound what the cycle-1
row proves — both are in `verify_config.py`, which the issue forbids modifying:

- R2 uses `isinstance(result.get("timeout"), int)`, and `bool` is a subclass of `int` in Python, so a
  boolean `timeout` would satisfy the check.
- R1/R3 read missing keys through `.get(...)` defaults, so the criterion does not distinguish an
  absent key from a normalized one. `normalize` itself raises `KeyError` on a config with no
  `timeout` or `region`; the authority does not specify behavior for incomplete configs, so I left
  that unhandled rather than inventing an unratified contract.

## Files

- `config_rules.py` — implementation (the deliverable).
- `test_config_rules.py` — cycle 2 test.
- `verify_config.py`, `fixtures/deploy.json` — unmodified, per the issue.
