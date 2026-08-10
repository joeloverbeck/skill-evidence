from textkit.redaction import redact


def test_replaces_every_occurrence_of_a_secret():
    assert (
        redact("hunter2 then hunter2 again", ["hunter2"])
        == "[REDACTED] then [REDACTED] again"
    )


def test_matches_secrets_case_insensitively():
    assert redact("Hunter2 and HUNTER2", ["hunter2"]) == "[REDACTED] and [REDACTED]"


def test_longer_secret_wins_when_two_match_at_the_same_position():
    assert redact("pw: hunter2000", ["hunter2", "hunter2000"]) == "pw: [REDACTED]"


def test_ignores_secrets_shorter_than_four_characters():
    assert redact("cat code1234", ["cat", "code"]) == "cat [REDACTED]1234"
