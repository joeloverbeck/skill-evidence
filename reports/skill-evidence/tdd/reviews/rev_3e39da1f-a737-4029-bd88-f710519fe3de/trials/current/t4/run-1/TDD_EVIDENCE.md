# TDD evidence

## Cycle `accepted-batch-boundary`

- Seam authority: `../TASK.md:3-6` designates `verify_policy.py` as the authoritative verifier for the public `allows_batch(size)` boundary and requires size 50 to be allowed while sizes above 50 are refused.
- Existing verifier: `verify_policy.py`.
- Input identity: the verifier's `allows_batch(50)` and `allows_batch(51)` probes, together forming the accepted boundary criterion.
- Observed public entry point: `policy.allows_batch(size)`.
- Red command: `/usr/bin/python3 verify_policy.py`.
- Intended red: exit 1 with `size 51 must be refused`; the size-50 probe must not appear among the failures.
- Observed red: exit 1 with `size 51 must be refused`; the size-50 probe did not appear among the failures.
- Minimal implementation: return whether `size <= 50` from `allows_batch`.
- Green command: `/usr/bin/python3 verify_policy.py`.
- Green result: exit 0 with `policy verifier: ok`.

## Pre-review reconciliation

- Changed-test inventory: no changed tests; existing verifier evidence.
- Existing-verifier evidence identities: `accepted-batch-boundary`.
- Reconciliation identities: `accepted-batch-boundary`.
- Evidence identities missing from reconciliation: none.
- Reconciliation identities missing from evidence: none.
