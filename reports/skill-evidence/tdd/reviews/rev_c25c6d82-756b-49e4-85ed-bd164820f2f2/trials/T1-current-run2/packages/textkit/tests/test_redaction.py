import pytest

from textkit.redaction import redact


def test_replaces_every_occurrence_of_every_secret():
    text = "user=alice pass=hunter2; retry user=alice pass=hunter2"

    assert redact(text, ["hunter2", "alice"]) == (
        "user=[REDACTED] pass=[REDACTED]; retry user=[REDACTED] pass=[REDACTED]"
    )


def test_matches_secrets_regardless_of_case_and_keeps_surrounding_case():
    text = "Login: Hunter2 then HUNTER2 and hunter2 (Case Kept)"

    assert redact(text, ["hunter2"]) == (
        "Login: [REDACTED] then [REDACTED] and [REDACTED] (Case Kept)"
    )


def test_longer_secret_wins_when_both_match_at_the_same_position():
    text = "pw=password123!"

    # Listed shortest-first: the winner must be chosen by length, not by list order.
    assert redact(text, ["password", "password123"]) == "pw=[REDACTED]!"


def test_secret_occurring_only_inside_the_placeholder_is_not_redacted_again():
    # "acted" is absent from the text but present in "[REDACTED]" (as "ACTED").
    text = "pw=password123 stays clean"

    assert redact(text, ["password123", "acted"]) == "pw=[REDACTED] stays clean"


@pytest.mark.parametrize(
    ("secret", "expected"),
    [
        pytest.param("cat", "the cats sat, cat naps", id="three-chars-ignored"),
        pytest.param(
            "cats", "the [REDACTED] sat, cat naps", id="four-chars-redacted"
        ),
    ],
)
def test_secrets_shorter_than_four_characters_are_ignored(secret, expected):
    # The pair pins the threshold at exactly 4: "cat" must survive and "cats"
    # must not, so neither `len < 3` nor `len <= 4` can pass both cases.
    text = "the cats sat, cat naps"

    assert redact(text, [secret]) == expected
