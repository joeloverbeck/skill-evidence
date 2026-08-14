# TDD evidence

## Scope and seam authority

The accepted authority is `work/authority.md`. It ratifies the public
`Receipt.to_dict()` representation and requires the identifier key to be
`receipt_id` rather than `id`.

The observed public seam is `Receipt.to_dict()`, exercised through
`Receipt("r1").to_dict()`.

## Changed-test inventory

| Test file | Exact selector | Observed public entry point | Basis for inventory |
| --- | --- | --- | --- |
| `work/test_receipt.py` | `test_receipt.ReceiptTests.test_public_dictionary_shape` | `Receipt("r1").to_dict()` | `work/work.log` records that the existing compatibility assertion was edited after production |

## TDD cycle evidence

There is no qualifying red-to-green evidence row for this change.

The supplied work log fixes the chronology: production changed first, the
existing compatibility assertion changed second, and the first recorded
focused run passed. No failing observation was recorded before the production
change or before the compatibility assertion was edited. The recorded focused
command and outcome are:

```text
python3 -m unittest test_receipt.ReceiptTests.test_public_dictionary_shape -v
Ran 1 test; OK
```

The recorded aggregate verification is:

```text
python3 -m unittest discover -v
Ran 1 test; OK
```

These are passing verification observations, not red and green positions in a
TDD cycle. They therefore do not place the production change or test edit under
TDD custody.

## Changed-test reconciliation

| Test file | Exact selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `work/test_receipt.py` | `test_receipt.ReceiptTests.test_public_dictionary_shape` | `Receipt("r1").to_dict()` | `work/authority.md`: accepted public `Receipt.to_dict()` wire representation | None; the recorded chronology contains no red-to-green cycle |

Changed-test inventory identities minus reconciliation identities: empty.

Reconciliation identities minus changed-test inventory identities: empty.

The changed test is reconciled to the agreed public seam and has present
coverage, but it is explicitly outside TDD custody.

## Present-state verification

The following commands were run independently during reconciliation from
`work/`; both exited with status 0 and reported one passing test:

```text
python3 -m unittest test_receipt.ReceiptTests.test_public_dictionary_shape -v
python3 -m unittest discover -v
```

This present-state verification does not reconstruct or replace missing
historical red evidence.
