"""Behavior of the public `redact` seam.

Seam: `textkit.redaction.redact(text, secrets)`.
Authority: TASK.md, "Requirements for `redact(text: str, secrets: list[str]) -> str`".
"""

from textkit.redaction import redact


def test_every_occurrence_of_a_secret_is_replaced():
    """Requirement 1: every occurrence, not just the first."""
    assert (
        redact("token abc123 and abc123 again", ["abc123"])
        == "token [REDACTED] and [REDACTED] again"
    )


def test_matching_is_case_insensitive():
    """Requirement 2: the secret matches in any case, and the rest of the text
    keeps its own casing (a lowercase-everything implementation would not)."""
    assert (
        redact("Login with Hunter2 or HUNTER2 now", ["hunter2"])
        == "Login with [REDACTED] or [REDACTED] now"
    )


def test_secret_is_matched_literally_not_as_a_pattern():
    """Requirement 1: an occurrence is a literal occurrence of the secret.

    `p@ssXword` is not the secret, so it must survive; an implementation that
    treats the secret as a regex would redact it too.
    """
    assert (
        redact("use p@ss.word here, not p@ssXword", ["p@ss.word"])
        == "use [REDACTED] here, not p@ssXword"
    )


def test_longer_secret_wins_when_both_match_at_the_same_position():
    """Requirement 3: `pass` and `password` both start at the same index, and
    the longer one takes the whole match.

    The shorter secret is listed first on purpose: an implementation that
    applies secrets in the order given would leave a `word` tail behind.
    """
    assert (
        redact("my password is safe", ["pass", "password"])
        == "my [REDACTED] is safe"
    )


def test_only_occurrences_in_the_input_text_are_redacted():
    """Requirement 1: an occurrence is an occurrence *in `text`*.

    `redact` never appears in the input, so it must match nothing. An
    implementation that substitutes secrets one after another would find it
    inside the `[REDACTED]` it had just written.
    """
    assert redact("pw hunter2", ["hunter2", "redact"]) == "pw [REDACTED]"


def test_secret_shorter_than_four_characters_is_ignored():
    """Requirement 4: `cat` (3) is ignored, `acme` (4) is not.

    Both halves are in one golden so it discriminates in both directions: no
    length rule at all would shred `cat`, and a `> 4` threshold would spare
    `acme`.
    """
    assert (
        redact("the cat sat at acme corp", ["cat", "acme"])
        == "the cat sat at [REDACTED] corp"
    )


def test_text_is_unchanged_when_every_secret_is_too_short():
    """Requirement 4: ignored *entirely*, so nothing is left to redact."""
    assert (
        redact("the cat sat on the mat", ["cat", "the"])
        == "the cat sat on the mat"
    )




