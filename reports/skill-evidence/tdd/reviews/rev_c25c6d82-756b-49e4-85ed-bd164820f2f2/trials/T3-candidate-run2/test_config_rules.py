"""Tests for the deployment config house rules.

Seam under test: the public function `config_rules.normalize(config)`, ratified by
ISSUE.md ("Implement `config_rules.normalize(config)`").

Rules R1/R2/R3 are covered by the public acceptance verifier `verify_config.py` against
`fixtures/deploy.json` and are deliberately not duplicated here. This file covers only the
requirement the verifier cannot observe: ISSUE.md's "`normalize` must return a new config
dict; it must not mutate its argument."
"""

import config_rules


def test_normalize_returns_new_config_without_mutating_argument():
    # A rule-violating input is required for this to discriminate: given an
    # already-compliant config, an in-place implementation would leave the caller's dict
    # untouched too, and the test would pass against the forbidden behavior.
    config = {"service": "checkout", "replicas": 1, "timeout": "45", "region": "EU-WEST-1"}

    result = config_rules.normalize(config)

    assert result is not config
    assert config == {"service": "checkout", "replicas": 1, "timeout": "45", "region": "EU-WEST-1"}
