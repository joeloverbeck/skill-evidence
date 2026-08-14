# TDD evidence

## Existing-verifier cycle: accepted batch-size boundary

- Seam authority: `TASK.md` specifies the public `allows_batch(size)` criterion and names `work/verify_policy.py` as its authoritative verifier.
- Verifier and input identity: `work/verify_policy.py`; boundary inputs `size=50` and `size=51`.
- Observed public entry point: `policy.allows_batch(size)`.
- Intended red set: `size 51 must be refused`; the `size=50` probe must not fail.
- Red command: `python3 work/verify_policy.py; verifier_status=$?; echo "exit_status=$verifier_status"; exit "$verifier_status"`
- Observed red: output `size 51 must be refused` and `exit_status=1`; no `size 50 must be allowed` failure was reported.
- Minimal implementation: changed `allows_batch` to return `size <= 50`.
- Green command: `python3 work/verify_policy.py`
- Green result: output `policy verifier: ok`; exit status 0.

## Reconciliation

- Scoped test inventory: empty; no test was added, modified, renamed, or removed.
- Reconciliation identity: no changed tests; existing verifier evidence for `work/verify_policy.py` with inputs `size=50` and `size=51`.
- Existing-verifier evidence identities missing from reconciliation: empty.
- Reconciliation identities missing from existing-verifier evidence: empty.
