"""Behavior of the public ``textkit.redaction.redact`` seam.

Seam authority: TASK.md, "Requirements for ``redact(text: str, secrets: list[str]) -> str``".
"""

from textkit.redaction import redact


def test_redacts_every_occurrence_of_each_secret():
    assert redact(
        "login abcd1234 then abcd1234 again, key wxyz9876 done",
        ["abcd1234", "wxyz9876"],
    ) == "login [REDACTED] then [REDACTED] again, key [REDACTED] done"


def test_matches_secrets_case_insensitively():
    assert (
        redact("Hunter2 then HUNTER2 then hunter2", ["hunter2"])
        == "[REDACTED] then [REDACTED] then [REDACTED]"
    )


def test_longer_secret_wins_when_both_match_at_the_same_position():
    # The shorter secret is listed first, so list order cannot be what decides.
    assert (
        redact("the key is hunter2000 today", ["hunter2", "hunter2000"])
        == "the key is [REDACTED] today"
    )


def test_ignores_secrets_shorter_than_four_characters():
    # "cat" is 3 characters and must be ignored; "cart" is exactly 4 and must not be.
    assert redact("cat and cart", ["cat", "cart"]) == "cat and [REDACTED]"


def test_leaves_text_untouched_when_every_secret_is_too_short():
    # "ignored entirely" has to hold when nothing survives the length rule, too.
    assert redact("cat and dog", ["cat", "dog"]) == "cat and dog"
