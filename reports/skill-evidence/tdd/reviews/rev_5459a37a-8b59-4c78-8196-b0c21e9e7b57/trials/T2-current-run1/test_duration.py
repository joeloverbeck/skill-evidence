import pytest

from duration import format_duration


def test_zero_renders_as_bare_zero_seconds():
    assert format_duration(0) == "0s"


def test_sub_second_renders_with_two_decimal_places():
    assert format_duration(450) == "0.45s"


def test_whole_seconds_render_without_decimals():
    assert format_duration(9000) == "9s"


def test_a_minute_and_a_half_rolls_over_into_minutes():
    assert format_duration(90000) == "1m 30s"


def test_more_than_an_hour_rolls_over_into_hours():
    assert format_duration(3723000) == "1h 2m 3s"


def test_a_negative_duration_is_refused():
    with pytest.raises(ValueError):
        format_duration(-1)
