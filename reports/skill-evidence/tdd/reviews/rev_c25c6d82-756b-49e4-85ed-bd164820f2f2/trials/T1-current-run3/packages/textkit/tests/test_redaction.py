from textkit.redaction import redact


def test_every_occurrence_of_a_secret_is_replaced():
    assert (
        redact("hunter2 then hunter2 again", ["hunter2"])
        == "[REDACTED] then [REDACTED] again"
    )


def test_matching_is_case_insensitive():
    assert (
        redact("Hunter2 HUNTER2 hunter2", ["hunter2"])
        == "[REDACTED] [REDACTED] [REDACTED]"
    )


def test_the_longer_secret_wins_when_two_match_at_the_same_position():
    # "pass" is listed first, so list order opposes length order here.
    redacted = redact("my password here", ["pass", "password"])

    assert redacted == "my [REDACTED] here"
    # The rejected alternative -- replacing each secret in turn, in list order --
    # consumes "pass" first and strands the tail of the longer secret.
    assert redacted != "my [REDACTED]word here"


def test_a_secret_shorter_than_four_characters_is_ignored():
    # "cat" is 3 characters and must survive; "lion" is exactly 4 and must not.
    # Applying no length rule at all would redact both; an off-by-one threshold
    # that also dropped 4-character secrets would redact neither.
    assert redact("cat and lion", ["cat", "lion"]) == "cat and [REDACTED]"


def test_text_is_untouched_when_every_secret_is_too_short():
    assert redact("the cat sat on the mat", ["cat", "at"]) == "the cat sat on the mat"
