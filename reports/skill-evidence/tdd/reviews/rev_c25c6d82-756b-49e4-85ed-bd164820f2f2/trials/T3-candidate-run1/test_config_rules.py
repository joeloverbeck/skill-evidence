"""Tests for clauses of issue 41 that the acceptance verifier does not observe.

The R1/R2/R3 rules are covered by the authoritative verifier
(`python3 verify_config.py fixtures/deploy.json`) and are deliberately not
duplicated here. This file covers only the issue's non-mutation clause.
"""

import config_rules


def test_normalize_does_not_mutate_its_argument():
    """Issue 41: normalize must not mutate the config it is given."""
    config = {
        "service": "checkout",
        "replicas": 1,
        "timeout": "45",
        "region": "EU-WEST-1",
    }

    config_rules.normalize(config)

    assert config == {
        "service": "checkout",
        "replicas": 1,
        "timeout": "45",
        "region": "EU-WEST-1",
    }
