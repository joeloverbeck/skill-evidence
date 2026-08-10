"""Behaviour of `textkit.redaction.redact`.

Seam: `textkit.redaction.redact` (authority: TASK.md). Every test calls that public
function and asserts on its return value; nothing here touches module internals.
"""

from textkit.redaction import redact


def test_replaces_every_occurrence_of_each_secret():
    # TASK.md req 1. "beta" is exactly 4 characters, so this golden also pins the
    # inclusive edge of req 4's floor: length 4 is kept, not dropped.
    result = redact("alpha beta alpha", ["alpha", "beta"])

    assert result == "[REDACTED] [REDACTED] [REDACTED]"
    # Discriminating: a first-occurrence-only substitution (`str.replace(s, r, 1)` or
    # `re.sub(..., count=1)`) would leave the trailing "alpha" in place.
    assert result != "[REDACTED] [REDACTED] alpha"


def test_matches_secrets_case_insensitively():
    # TASK.md req 2: the secret "hunter2" also redacts "Hunter2" and "HUNTER2".
    result = redact("Hunter2 and HUNTER2 and hunter2", ["hunter2"])

    assert result == "[REDACTED] and [REDACTED] and [REDACTED]"
    # Discriminating: case-sensitive matching redacts only the exact-case occurrence.
    assert result != "Hunter2 and HUNTER2 and [REDACTED]"


def test_longer_secret_wins_when_two_match_at_the_same_position():
    # TASK.md req 3. Both secrets match at the same position; the shorter one is
    # listed first so that any list-order-driven implementation picks the wrong one.
    result = redact("the password is set", ["pass", "password"])

    assert result == "the [REDACTED] is set"
    # Discriminating: taking the first alternative that matches (regex alternation in
    # the given order, or a sequential replace loop) redacts "pass" and strands "word".
    assert result != "the [REDACTED]word is set"


def test_ignores_secrets_shorter_than_four_characters():
    # TASK.md req 4: "cat" is 3 characters, so it is too short to be a secret and
    # ordinary text must survive untouched.
    result = redact("the cat sat on the mat", ["cat"])

    assert result == "the cat sat on the mat"
    # Discriminating: without the length floor the short secret shreds ordinary text.
    assert result != "the [REDACTED] sat on the mat"


def test_matches_punctuation_in_a_secret_literally():
    # TASK.md req 1 replaces occurrences *of the secret*. "abcd" is not an occurrence
    # of the secret "a.cd", so it must survive; real secrets do contain punctuation.
    result = redact("a.cd and abcd", ["a.cd"])

    assert result == "[REDACTED] and abcd"
    # Discriminating: an unescaped pattern makes "." a wildcard and over-redacts "abcd".
    assert result != "[REDACTED] and [REDACTED]"
