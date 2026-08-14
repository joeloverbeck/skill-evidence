# TDD evidence

## Scope and authority

- Authorized seam: the public `Receipt.to_dict()` return value, authorized by `authority.md` as the exact wire change from `{"id": "r1"}` to `{"receipt_id": "r1"}` before implementation.
- Supplied chronology: `work.log` records that production changed first, the existing compatibility assertion changed second, and the assertion's first recorded run passed.
- This document reconciles the supplied completed state and record. It does not reconstruct or rerun history.

## Changed-test inventory

| Test file | Exact selector | Observed public entry point | Seam authority |
|---|---|---|---|
| `test_receipt.py` | `test_receipt.ReceiptTests.test_public_dictionary_shape` | `Receipt("r1").to_dict()` | `authority.md`, accepted compatibility contract |

Inventory identity set:

```text
test_receipt.ReceiptTests.test_public_dictionary_shape
```

## Reconciliation

| Inventory identity | Reconciliation identity | Evidence classification |
|---|---|---|
| `test_receipt.ReceiptTests.test_public_dictionary_shape` | `test_receipt.ReceiptTests.test_public_dictionary_shape` | Post-production compatibility verification; not a TDD cycle |

```text
changed-test inventory minus reconciliation: empty
reconciliation minus changed-test inventory: empty
```

The test observes the authorized public seam directly. Its literal expected dictionary discriminates the accepted `receipt_id` field from the replaced `id` field.

## Recorded execution evidence

| Test identity | Complete command | Recorded observation | Custody |
|---|---|---|---|
| `test_receipt.ReceiptTests.test_public_dictionary_shape` | `python3 -m unittest test_receipt.ReceiptTests.test_public_dictionary_shape -v` | First recorded run: `Ran 1 test`; `OK` | Verification only |
| Aggregate suite | `python3 -m unittest discover -v` | `Ran 1 test`; `OK` | Verification only |

## TDD custody conclusion

No red observation exists in the supplied record: production was changed before the compatibility assertion, and that assertion passed on its first recorded run. Therefore no red→green evidence row can truthfully be retained for this change, and neither the production edit nor the compatibility test edit is under TDD custody. The accepted pre-implementation seam authority, exact changed-test reconciliation, focused passing run, and aggregate passing run are retained as review evidence only.
