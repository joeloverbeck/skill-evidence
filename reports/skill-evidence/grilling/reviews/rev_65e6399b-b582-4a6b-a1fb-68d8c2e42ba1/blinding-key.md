# Evaluator blinding key

Fixed before any candidate output existed, so the assignment cannot be chosen after a result is
in hand. Evaluators see only `Version A` / `Version B` and never receive this file.

| Trial | Version A | Version B |
|---|---|---|
| R1 | candidate | current |
| R2 | current | candidate |
| R3 | current | candidate |
| A1 | candidate | current |
| C1 | candidate | current |
| C2 | current | candidate |
| S1 | candidate | current |

For the three-run reproduction trials the six outputs are pooled and interleaved instead of
labelled A/B: odd positions (`output-1/3/5`) hold the Version A arm's runs 1–3 in order, even
positions (`output-2/4/6`) hold the Version B arm's. The interleave is fixed by this key and the
evaluator never sees it.
