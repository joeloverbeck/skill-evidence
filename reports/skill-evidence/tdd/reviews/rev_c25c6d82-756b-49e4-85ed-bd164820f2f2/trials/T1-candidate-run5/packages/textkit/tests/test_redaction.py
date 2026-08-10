from textkit.redaction import redact


def test_replaces_every_occurrence_of_every_secret_with_the_redaction_marker():
    assert (
        redact(
            "swordfish here, swordfish there, hunter2 too",
            ["swordfish", "hunter2"],
        )
        == "[REDACTED] here, [REDACTED] there, [REDACTED] too"
    )


def test_matches_secrets_case_insensitively_and_leaves_other_text_cased():
    assert (
        redact("Login used Hunter2 and backup HUNTER2", ["hunter2"])
        == "Login used [REDACTED] and backup [REDACTED]"
    )


def test_longest_secret_wins_when_two_secrets_match_at_the_same_position():
    assert (
        redact("leaked hunter2000 today", ["hunter", "hunter2000"])
        == "leaked [REDACTED] today"
    )


def test_ignores_secrets_shorter_than_four_characters_but_redacts_four():
    assert (
        redact("the cat sat on the sofa", ["cat", "sofa"])
        == "the cat sat on the [REDACTED]"
    )


def test_returns_the_text_unchanged_when_every_secret_is_too_short():
    assert redact("the cat sat on the mat", ["cat", "mat"]) == "the cat sat on the mat"


def test_matches_secrets_literally_rather_than_as_patterns():
    assert (
        redact("n.ll is redacted but null is not", ["n.ll"])
        == "[REDACTED] is redacted but null is not"
    )
