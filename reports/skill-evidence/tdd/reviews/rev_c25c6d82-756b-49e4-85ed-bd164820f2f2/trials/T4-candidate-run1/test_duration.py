from duration import parse_duration


def test_seconds_suffix_gives_that_many_seconds():
    assert parse_duration("90s") == 90


def test_a_different_seconds_value_gives_its_own_number():
    assert parse_duration("5s") == 5
