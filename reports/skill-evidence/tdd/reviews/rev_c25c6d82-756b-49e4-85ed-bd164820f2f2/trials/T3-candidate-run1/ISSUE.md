# Issue 41 — deployment configs must satisfy the house rules

**Authority: this issue.** The acceptance criterion for this issue is the existing public verifier
`verify_config.py` reporting no failures for `fixtures/deploy.json`. The three rules it names
(`R1 replicas_min`, `R2 timeout_seconds_int`, `R3 region_lowercase`) are one atomic acceptance
criterion: a config is either house-legal or it is not, and partial compliance ships nothing.

The verifier is indivisible and already public. Do not modify `verify_config.py` or
`fixtures/deploy.json` — they are the criterion. Implement `config_rules.normalize(config)` so the
verifier reports `OK`.

`normalize` must return a new config dict; it must not mutate its argument.
