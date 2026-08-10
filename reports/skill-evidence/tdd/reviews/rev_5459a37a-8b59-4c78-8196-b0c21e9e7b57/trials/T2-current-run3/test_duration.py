"""Behavior tests for the public duration formatter.

Every test goes through the public seam `duration.format_duration`.
Expected values are literals taken from the task specification, never recomputed
the way the implementation computes them.
"""

import pytest

from duration import format_duration


def test_zero_renders_as_bare_zero_seconds():
    assert format_duration(0) == "0s"


def test_sub_second_renders_with_two_decimal_places():
    assert format_duration(450) == "0.45s"


def test_whole_seconds_under_a_minute_render_without_decimals():
    assert format_duration(9000) == "9s"


def test_ninety_seconds_rolls_over_into_minutes_and_seconds():
    assert format_duration(90000) == "1m 30s"


def test_multi_hour_duration_renders_hours_minutes_and_seconds():
    assert format_duration(3723000) == "1h 2m 3s"


def test_negative_duration_is_rejected_without_producing_a_formatted_duration():
    # One logical claim, asserted on both halves as the refusal-path rule requires:
    # the call refuses, and no rendered duration escapes through the raised error.
    with pytest.raises(ValueError) as raised:
        format_duration(-1)

    leaked = [
        rendering
        for rendering in ("-0.00s", "0.00s", "0s", "-1s", "-0s")
        if rendering in str(raised.value)
    ]
    assert leaked == []
