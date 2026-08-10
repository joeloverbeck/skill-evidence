"""Tests for the public `config_rules.normalize` seam.

Seam authority: ISSUE.md (Issue 41) names `config_rules.normalize(config)` as the
public entry point and requires that it "return a new config dict; it must not
mutate its argument". The acceptance verifier `verify_config.py` only inspects
`normalize`'s return value, so it cannot observe the non-mutation requirement.
"""

import config_rules


def test_normalize_leaves_caller_config_unmutated():
    # Input requires all three house rules to fire, so a no-op implementation
    # cannot pass this test by leaving the argument untouched.
    config = {"service": "billing", "replicas": 1, "timeout": "30", "region": "US-EAST-2"}

    result = config_rules.normalize(config)

    # Expected values are known-good literals fixed by Issue 41's rules
    # (R1 at least 2, R2 int seconds, R3 lowercase region), not recomputed.
    assert result == {"service": "billing", "replicas": 2, "timeout": 30, "region": "us-east-2"}
    assert config == {"service": "billing", "replicas": 1, "timeout": "30", "region": "US-EAST-2"}
