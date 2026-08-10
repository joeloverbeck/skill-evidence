"""Sensitivity check: substitute each rejected implementation and confirm the
golden that is supposed to reject it actually fails.

A golden that still passes under its rejected variant is not discriminating.
Scratch harness -- not part of the deliverable.
"""

import importlib
import re
import sys
from pathlib import Path

RUN_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(RUN_ROOT / "packages" / "textkit"))
sys.path.insert(0, str(RUN_ROOT / "packages" / "textkit" / "tests"))

import test_redaction  # noqa: E402

PLACEHOLDER = "[REDACTED]"


def v_first_only(text, secrets):
    for s in [x for x in secrets if len(x) >= 4]:
        text = re.sub(re.escape(s), PLACEHOLDER, text, count=1, flags=re.IGNORECASE)
    return text


def v_lowercase_all(text, secrets):
    usable = [s for s in secrets if len(s) >= 4]
    text = text.lower()
    for s in sorted(usable, key=len, reverse=True):
        text = text.replace(s.lower(), PLACEHOLDER)
    return text


def v_no_escape(text, secrets):
    usable = [s for s in secrets if len(s) >= 4]
    if not usable:
        return text
    pattern = "|".join(sorted(usable, key=len, reverse=True))
    return re.sub(pattern, PLACEHOLDER, text, flags=re.IGNORECASE)


def v_given_order(text, secrets):
    for s in [x for x in secrets if len(x) >= 4]:
        text = re.sub(re.escape(s), PLACEHOLDER, text, flags=re.IGNORECASE)
    return text


def v_sequential_longest_first(text, secrets):
    usable = [s for s in secrets if len(s) >= 4]
    for s in sorted(usable, key=len, reverse=True):
        text = re.sub(re.escape(s), PLACEHOLDER, text, flags=re.IGNORECASE)
    return text


def v_no_length_rule(text, secrets):
    if not secrets:
        return text
    pattern = "|".join(re.escape(s) for s in sorted(secrets, key=len, reverse=True))
    return re.sub(pattern, PLACEHOLDER, text, flags=re.IGNORECASE)


def v_threshold_off_by_one(text, secrets):
    usable = [s for s in secrets if len(s) > 4]
    if not usable:
        return text
    pattern = "|".join(re.escape(s) for s in sorted(usable, key=len, reverse=True))
    return re.sub(pattern, PLACEHOLDER, text, flags=re.IGNORECASE)


def v_no_empty_guard(text, secrets):
    usable = [s for s in secrets if len(s) >= 4]
    pattern = "|".join(re.escape(s) for s in sorted(usable, key=len, reverse=True))
    return re.sub(pattern, PLACEHOLDER, text, flags=re.IGNORECASE)


# (variant id, replacement impl, test that must reject it)
CASES = [
    ("M1 replace-first-occurrence-only", v_first_only,
     "test_every_occurrence_of_a_secret_is_replaced"),
    ("M2 lowercase-the-whole-text", v_lowercase_all,
     "test_matching_is_case_insensitive"),
    ("M3 secret-used-as-regex", v_no_escape,
     "test_secret_is_matched_literally_not_as_a_pattern"),
    ("M4 apply-secrets-in-given-order", v_given_order,
     "test_longer_secret_wins_when_both_match_at_the_same_position"),
    ("M5 sequential-substitution-longest-first", v_sequential_longest_first,
     "test_only_occurrences_in_the_input_text_are_redacted"),
    ("M6 no-minimum-length-rule", v_no_length_rule,
     "test_secret_shorter_than_four_characters_is_ignored"),
    ("M7 threshold-strictly-greater-than-4", v_threshold_off_by_one,
     "test_secret_shorter_than_four_characters_is_ignored"),
    ("M8 no-empty-secret-set-guard", v_no_empty_guard,
     "test_text_is_unchanged_when_every_secret_is_too_short"),
]

ALL_TESTS = [n for n in dir(test_redaction) if n.startswith("test_")]
real = importlib.import_module("textkit.redaction").redact

failures = []
for variant_id, impl, guard_name in CASES:
    test_redaction.redact = impl
    try:
        getattr(test_redaction, guard_name)()
        verdict = "PASSED -> golden is NOT discriminating"
        failures.append((variant_id, guard_name))
    except AssertionError:
        verdict = "rejected (test failed as required)"
    print(f"{variant_id:42s} vs {guard_name:58s} {verdict}")

# Every test must pass against the real implementation.
test_redaction.redact = real
for name in sorted(ALL_TESTS):
    getattr(test_redaction, name)()

print()
if failures:
    print(f"NON-DISCRIMINATING GOLDENS: {failures}")
    sys.exit(1)
print(f"All {len(CASES)} rejected variants are caught; "
      f"all {len(ALL_TESTS)} tests pass against the real implementation.")
