from duration import parse_duration


def test_parses_the_worked_example_90s_as_90_seconds():
    assert parse_duration("90s") == 90


def test_parses_a_different_seconds_duration_than_the_worked_example():
    assert parse_duration("5s") == 5
