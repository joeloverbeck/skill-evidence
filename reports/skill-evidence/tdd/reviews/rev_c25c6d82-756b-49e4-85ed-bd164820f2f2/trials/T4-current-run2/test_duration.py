from duration import parse_duration


def test_seconds_suffix_gives_whole_seconds():
    # Worked example fixed by the requirement: parse_duration("90s") is 90.
    assert parse_duration("90s") == 90


def test_seconds_come_from_the_text_not_a_fixed_value():
    # The requirement converts whatever number carries the "s" suffix, so a
    # second value must read differently from the first one.
    assert parse_duration("7s") == 7
