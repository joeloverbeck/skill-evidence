"""Tests for behavior of config_rules.normalize that verify_config.py cannot observe.

Seam: the public function ``config_rules.normalize``.
Authority: ISSUE.md - "``normalize`` must return a new config dict; it must not
mutate its argument." The acceptance verifier only inspects normalize's return
value, so it can never see the caller's dict being mutated.
"""
import config_rules


def test_normalize_leaves_its_argument_unchanged():
    # Every field violates a house rule, so a normalize that works in place is
    # forced to change all three and cannot escape detection.
    config = {"service": "billing", "replicas": 1, "timeout": "30", "region": "US-EAST-2"}

    config_rules.normalize(config)

    # Independent literal restating the caller's dict as it was passed in.
    assert config == {"service": "billing", "replicas": 1, "timeout": "30", "region": "US-EAST-2"}


def test_normalize_returns_a_new_dict_for_an_already_compliant_config():
    # An already-compliant config is the one input where "does not mutate" and
    # "returns a new dict" come apart: a `return config` fast path mutates
    # nothing yet hands the caller back its own dict. See NOTES.md - this guard
    # is sensitivity-checked, not the product of a red -> green cycle.
    config = {"service": "billing", "replicas": 3, "timeout": 30, "region": "us-east-2"}

    assert config_rules.normalize(config) is not config
