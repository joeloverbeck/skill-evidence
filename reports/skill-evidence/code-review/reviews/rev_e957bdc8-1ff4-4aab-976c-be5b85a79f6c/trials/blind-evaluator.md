# Blind evaluator readings

## T1 current run 1

| Label | Witness expressed | Factual reason |
|---|---|---|
| ember | No | It reports no findings but never analyzes the `max_attempts` change or cites the supplied retry-policy spec. |

## T2 current runs

| Label | Witness expressed | Failure recurred | Coverage lines | Factual reason |
|---|---|---|---|---|
| cobalt | Yes | No | Pass | Confirms `PreparedReviewEvent` validation before separate append and says `AGENTS.md` overrides Middle Man. |
| linden | Yes | No | Pass | Reports no findings after checking `AGENTS.md` and recognizes the mandatory validated-before-append boundary. |
| sable | Yes | No | Pass | Confirms `PreparedReviewEvent` is validated before append and suppresses the possible Middle Man smell under `AGENTS.md`. |

The evaluator did not receive package paths, arm identity, the evidence store, or the diagnosis.
