# Seam agreement (written before the first test)

## Ratified seams

| # | Seam under test | Authority |
|---|---|---|
| S1 | `duration.format_duration(ms) -> str`, reached as `from duration import format_duration` — the sole exported entry point of the public module `duration.py` | `TASK.md` (this work's authoritative spec): it names the module, the signature, and six acceptance behaviors |

No other seam is ratified. Nothing in `duration.py` other than `format_duration` is public,
so there is no second ingress path and no private helper is tested directly.

## Public-invariant ingress enumeration (tests.md, "Public invariant coverage")

The invariant under test is "a duration renders in the canonical form, and a negative duration is
refused". Exported ingress paths that can admit an `ms` value: `format_duration` only. There is no
parser, constructor, factory, or deserializer that could bypass its validation, so the direct probe
at `format_duration` is also the complete bypass probe. Re-check this if the module ever exports a
second admitting function.

On the refusal path (behavior 6) the test asserts the refusal via `pytest.raises(ValueError)`,
which by construction observes no returned string. There is no protected payload in this domain, so
"non-observation of protected payloads" reduces to "no formatted value is produced"; the
`pytest.raises` form supplies that.

## Decisions I made without a user (no user was available to confirm)

1. **TASK.md is treated as the authoritative PRD.** SKILL.md allows a seam ratified in "an
   authoritative PRD, live issue, ADR, or explicit user decision". TASK.md is the only specification
   in scope and it fixes both the interface and the goldens, so I treat it as that authority rather
   than blocking on a question no one can answer.
2. **Scope is exactly the six acceptance behaviors.** I did not invent extra goldens (e.g. what
   `3600000` renders as, or whether `1500` is `"1.5s"` or `"1s"`), because no authority fixes those
   results and an unratified golden would freeze a guess as a spec.
3. **`/code-review` is not invoked.** The task asks for working code and tests, not a review pass.
   I still perform the pre-review reconciliation gate (see NOTES.md) so the evidence is
   review-ready, and no review-reentry rows exist because no review ran.

## Discriminating goldens — rejected alternatives each behavior must kill

| Behavior | Golden | Plausible implementation the golden rejects |
|---|---|---|
| 1 | `format_duration(0) == "0s"` | `""`, `"0.00s"`, `"0ms"` |
| 2 | `format_duration(450) == "0.45s"` | `"0s"` (integer-truncating), `"0.5s"` (one decimal), `"450ms"` |
| 3 | `format_duration(9000) == "9s"` | `"9.00s"` (always two decimals), `"0m 9s"` |
| 4 | `format_duration(90000) == "1m 30s"` | `"90s"` (no minute rollover), `"1m 30.00s"`, `"0h 1m 30s"` |
| 5 | `format_duration(3723000) == "1h 2m 3s"` | `"62m 3s"` (no hour rollover), `"1h 2m"` (dropped tail), `"1h 2m 3.00s"` |
| 6 | `format_duration(-1)` raises `ValueError` | returning `"-0.00s"`/`"0s"`, or raising a different type |

Each golden is an independent literal taken from TASK.md, not recomputed the way the code computes
it, and each differs textually from every rejected alternative in its row — so an exact-equality
assertion is what discriminates; no extra `!=` assertion is needed to establish the difference.
