from textkit.redaction import redact


def test_replaces_every_occurrence_of_every_secret():
    text = "user hunter2 logged in with hunter2 and key swordfish"

    assert redact(text, ["hunter2", "swordfish"]) == (
        "user [REDACTED] logged in with [REDACTED] and key [REDACTED]"
    )


def test_matches_secrets_case_insensitively():
    text = "Hunter2 HUNTER2 hunter2 swordfish"

    assert redact(text, ["hunter2", "SWORDFISH"]) == (
        "[REDACTED] [REDACTED] [REDACTED] [REDACTED]"
    )


def test_longest_secret_wins_when_two_match_at_the_same_position():
    text = "my password123 here"

    assert redact(text, ["password", "password123"]) == "my [REDACTED] here"


def test_ignores_secrets_shorter_than_four_characters():
    text = "the cat sat on mat1 with hunter2"

    assert redact(text, ["cat", "mat1", "hunter2"]) == (
        "the cat sat on [REDACTED] with [REDACTED]"
    )


def test_returns_text_unchanged_when_every_secret_is_too_short():
    text = "the cat sat"

    assert redact(text, ["ab", "cat"]) == "the cat sat"
