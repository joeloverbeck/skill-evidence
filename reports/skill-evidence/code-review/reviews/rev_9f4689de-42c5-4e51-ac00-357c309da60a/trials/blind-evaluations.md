# Blind Evaluations

## T1 reproduction

- COBALT: C2 `true`; C3 `true`; mechanism result `reproduced`.
- AMBER: C2 `false`; C3 `false`; mechanism result `not reproduced with witnesses expressed`.

AMBER is materially better for the target mechanism. It produces separate Standards and Spec axes,
binds each run's axes and summary to one patch SHA-256, identifies `src/five.txt` as `3` rather than
required `2`, preserves finding/routing fields, and reports aggregate gate success. COBALT cannot
express the authorized uncommitted patch and launches neither axis.

The evaluator identified one material anomaly: AMBER run 2 reports a different patch SHA-256 from
runs 1 and 3 despite byte-identical logical input and unchanged pre/post snapshots. Each run is
internally consistent, so M2 is suppressed, but cross-run identity is not stable.

## T2 committed review

Both arms pass. Neither is materially worse; tie. No material or severe regression.

## T3 no-spec review

Both arms pass. The explanatory compliance sentence in one arm is harmless and invents no
requirement. Neither is materially worse; tie. No material or severe regression.

## T4 axis separation

Neither arm satisfies the frozen no-Spec-finding criterion. This is a fixture/rubric inconsistency,
not an arm-specific regression: the immutable `printf 'hello\\n'` fixture naturally supports both
arms' literal-backslash-n reading. Both correctly report the Standards naming violation, keep the
axes separate, and otherwise satisfy identity, source, routing, ledger, and aggregation checks.
Preference: tie.

## T5 scoped dirty-tree review

The blocking arm fails. The patch-identity arm passes: both axes bind to the same patch identity,
the Spec finding is correct, the unrelated sentinel is never cited, and repository preservation
holds. Preference: patch-identity arm, decisively.

## Comparative decision

The candidate materially improves M2 and is noninferior on the observed protected behaviors, but
the acceptance gate rejects it. Identical logical T1 inputs did not receive a stable patch identity
across fresh runs, which is a material identity regression, and T4 did not satisfy its frozen
expected reading. The workflow allows no second behavioral candidate in this review.
