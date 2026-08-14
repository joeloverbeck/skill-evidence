# TDD evidence

## Cycle VP-1

- Seam authority: `../TASK.md`, accepted criterion for public `allows_batch(size)`.
- Verifier and input identity: `verify_policy.py`; atomic inputs `size=50` and `size=51`.
- Observed public entry point: `policy.allows_batch(size)`.
- Working directory: `/tmp/tdd-evolution-rev-3e39-t4-verifier-r1/work`.
- Red command: `python3 verify_policy.py`.
- Intended red set: `size 51 must be refused`.
- Observed red: exit 1 with `size 51 must be refused`.
- Minimal implementation: return whether `size <= 50` from `allows_batch`.
- Green command: `python3 verify_policy.py`.
- Green result: exit 0 with `policy verifier: ok`.

## Pre-review reconciliation

- Test-change inventory: no added, modified, renamed, or removed selectors or named cases.
- Disposition: `no changed tests; existing verifier evidence`.
- Existing-verifier evidence identities: `VP-1`.
- TDD-custody reconciliation identities: `VP-1`.
- Evidence identities absent from reconciliation: empty.
- Reconciliation identities absent from evidence: empty.
