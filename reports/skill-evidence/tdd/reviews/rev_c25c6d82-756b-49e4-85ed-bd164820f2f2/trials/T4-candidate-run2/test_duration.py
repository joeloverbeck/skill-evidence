from duration import parse_duration


def test_duration_with_seconds_suffix_parses_to_that_many_seconds():
    assert parse_duration("90s") == 90
