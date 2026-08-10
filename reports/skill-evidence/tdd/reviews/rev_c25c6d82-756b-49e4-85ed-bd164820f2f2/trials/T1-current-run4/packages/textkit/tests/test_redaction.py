from textkit.redaction import redact


def test_redacts_every_occurrence_of_a_secret():
    assert (
        redact("hunter2 logs in, then hunter2 logs out", ["hunter2"])
        == "[REDACTED] logs in, then [REDACTED] logs out"
    )


def test_redacts_each_secret_in_the_list():
    assert (
        redact("user alice42 key swordfish", ["alice42", "swordfish"])
        == "user [REDACTED] key [REDACTED]"
    )


def test_matches_secrets_case_insensitively_and_preserves_surrounding_case():
    assert (
        redact("Login: Hunter2, backup HUNTER2, old hunter2", ["hunter2"])
        == "Login: [REDACTED], backup [REDACTED], old [REDACTED]"
    )


def test_longer_secret_wins_when_two_secrets_match_at_the_same_position():
    assert redact("my password here", ["pass", "password"]) == "my [REDACTED] here"


def test_ignores_secrets_shorter_than_four_characters_but_redacts_four():
    assert (
        redact("the cat sat by the gate", ["cat", "gate"])
        == "the cat sat by the [REDACTED]"
    )
