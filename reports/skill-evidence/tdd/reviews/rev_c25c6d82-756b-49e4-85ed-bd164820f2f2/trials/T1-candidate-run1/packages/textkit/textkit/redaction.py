"""Replace secret substrings in text with a fixed redaction marker."""

import re

REDACTION = "[REDACTED]"

#: Secrets shorter than this are ignored entirely: they are too short to be a secret,
#: and redacting them would shred ordinary text.
MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Return `text` with every occurrence of every secret replaced by `[REDACTED]`.

    Matching is case-insensitive. Where two secrets match at the same position, the
    longer one wins. Secrets shorter than `MIN_SECRET_LENGTH` are ignored entirely.
    """
    usable = [secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH]
    if not usable:
        # An empty alternation matches the empty string at every position, which would
        # redact the gaps between characters instead of nothing.
        return text
    # Python's alternation is leftmost-first, not longest-match, so ordering the
    # alternatives longest-first is what makes the longer secret win at a tie position.
    longest_first = sorted(usable, key=len, reverse=True)
    # Secrets are literal strings, not patterns: punctuation in a secret must match
    # itself rather than act as a wildcard (or fail to compile at all).
    pattern = re.compile(
        "|".join(re.escape(secret) for secret in longest_first), re.IGNORECASE
    )
    return pattern.sub(REDACTION, text)
