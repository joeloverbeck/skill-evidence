from textkit.redaction import redact


def test_every_occurrence_of_a_secret_is_replaced():
    # Two occurrences, so a replace-the-first-one-only implementation cannot pass.
    assert (
        redact("token swordfish, backup swordfish", ["swordfish"])
        == "token [REDACTED], backup [REDACTED]"
    )


def test_matching_is_case_insensitive():
    # Only the exact-case occurrence would go if matching were case-sensitive.
    assert (
        redact("Hunter2 and HUNTER2 and hunter2", ["hunter2"])
        == "[REDACTED] and [REDACTED] and [REDACTED]"
    )


def test_longer_secret_wins_at_the_same_position():
    # "pass" and "password" both match at the same index, and the shorter one is
    # listed first, so an in-list-order implementation would leave "word" behind.
    # Both are >= 4 characters, so requirement 4 does not decide this case.
    assert (
        redact("my password is set", ["pass", "password"]) == "my [REDACTED] is set"
    )


def test_secret_shorter_than_four_characters_is_ignored():
    # Honouring "cat" would shred the ordinary words around it, which is the harm
    # requirement 4 exists to prevent.
    assert (
        redact("cat catalog and cathedral", ["cat"]) == "cat catalog and cathedral"
    )


def test_marker_text_is_not_itself_redacted():
    # "acted" never occurs in the input; it only appears inside the marker that
    # redacting "password" introduces. Requirement 1 is about occurrences in `text`,
    # and the replacement has to stay the literal "[REDACTED]".
    assert (
        redact("my password is set", ["password", "acted"]) == "my [REDACTED] is set"
    )


def test_secret_of_exactly_four_characters_is_still_redacted():
    # Requirement 4 ignores secrets *shorter than* 4, so 4 itself is honoured. This
    # guard exists because every other golden here survives the "4 or shorter is
    # ignored" off-by-one; see NOTES.md, "Regression guard outside the loop".
    assert redact("key 4f9c rotated", ["4f9c"]) == "key [REDACTED] rotated"
