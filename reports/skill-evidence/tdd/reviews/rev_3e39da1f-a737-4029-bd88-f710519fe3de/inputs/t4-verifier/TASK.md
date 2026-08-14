# Satisfy the accepted batch-size verifier

The executable `verify_policy.py` is the authoritative verifier for this task and currently fails
on the accepted criterion: public `allows_batch(size)` must return false above size 50 and true at
size 50. Using the TDD method in `method/`, confirm that the verifier's observed failure is the
intended failure, implement only enough in `policy.py` to make it green, and retain the required
verifier evidence. Do not add a duplicative test and do not commit.
