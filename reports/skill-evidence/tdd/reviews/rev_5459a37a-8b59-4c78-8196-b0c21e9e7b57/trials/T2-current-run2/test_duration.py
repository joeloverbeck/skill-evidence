import pytest

from duration import format_duration


def test_zero_formats_as_bare_zero_seconds():
    assert format_duration(0) == "0s"


def test_sub_second_formats_with_two_decimal_places():
    assert format_duration(450) == "0.45s"


def test_whole_seconds_under_a_minute_format_without_decimals():
    assert format_duration(9000) == "9s"


def test_at_least_a_minute_splits_into_minutes_and_seconds():
    assert format_duration(90000) == "1m 30s"


def test_at_least_an_hour_splits_into_hours_minutes_and_seconds():
    assert format_duration(3723000) == "1h 2m 3s"


def test_negative_input_is_rejected():
    with pytest.raises(ValueError):
        format_duration(-1)
