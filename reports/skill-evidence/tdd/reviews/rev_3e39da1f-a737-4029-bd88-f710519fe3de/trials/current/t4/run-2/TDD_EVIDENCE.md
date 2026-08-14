# TDD evidence

## BATCH-BOUNDARY

- Seam authority: `TASK.md` names executable `verify_policy.py` as authoritative for the accepted criterion that public `allows_batch(size)` allows size 50 and refuses sizes above 50.
- Verifier and input identity: `work/verify_policy.py`; boundary inputs `50` and `51`.
- Observed public entry point: `policy.allows_batch(size)`.
- Red command: `python3 work/verify_policy.py` (working directory: `/tmp/tdd-evolution-rev-3e39-t4-verifier-k2`).
- Intended red: exit nonzero with `size 51 must be refused`, and no failure for size 50.
- Observed red: exit 1 with `size 51 must be refused`, and no failure for size 50.
- Minimal implementation: make `allows_batch(size)` return whether `size <= 50`.
- Green command: `python3 work/verify_policy.py` (working directory: `/tmp/tdd-evolution-rev-3e39-t4-verifier-k2`).
- Green result: exit 0 with `policy verifier: ok`.

## Reconciliation

- Changed-test inventory: no changed tests; existing verifier evidence.
- Existing-verifier evidence identities: `BATCH-BOUNDARY`.
- Reconciliation identities: `BATCH-BOUNDARY`.
- Evidence identities absent from reconciliation: none.
- Reconciliation identities absent from evidence: none.
