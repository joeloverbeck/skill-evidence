#!/usr/bin/env python3
"""Acceptance verifier for the deployment config house rules.

Usage: python3 verify_config.py fixtures/deploy.json
Exits 0 when every rule holds, 1 otherwise, printing one FAIL line per broken rule.
"""
import json
import sys

import config_rules


def main(path):
    with open(path) as handle:
        config = json.load(handle)
    try:
        result = config_rules.normalize(config)
    except NotImplementedError:
        print("FAIL: R1 replicas_min - normalize() is not implemented")
        print("FAIL: R2 timeout_seconds_int - normalize() is not implemented")
        print("FAIL: R3 region_lowercase - normalize() is not implemented")
        return 1
    failures = []
    if result.get("replicas", 0) < 2:
        failures.append("FAIL: R1 replicas_min - replicas must be at least 2, got %r" % result.get("replicas"))
    if not isinstance(result.get("timeout"), int):
        failures.append("FAIL: R2 timeout_seconds_int - timeout must be an int number of seconds, got %r" % result.get("timeout"))
    if result.get("region") != str(result.get("region", "")).lower():
        failures.append("FAIL: R3 region_lowercase - region must be lowercase, got %r" % result.get("region"))
    for line in failures:
        print(line)
    if failures:
        return 1
    print("OK: all rules hold")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
