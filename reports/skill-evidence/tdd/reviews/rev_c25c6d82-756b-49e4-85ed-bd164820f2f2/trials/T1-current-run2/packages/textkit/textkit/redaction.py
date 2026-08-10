"""Replace secret substrings in text with a fixed placeholder."""

import re

PLACEHOLDER = "[REDACTED]"

#: Secrets shorter than this are too short to be secrets; redacting them would
#: shred ordinary text, so they are ignored entirely.
MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Return `text` with every occurrence of every secret replaced.

    Matching is case-insensitive; text outside the matches is left untouched.
    When two secrets match at the same position, the longer one wins.
    Secrets shorter than `MIN_SECRET_LENGTH` are ignored.
    """
    usable = [secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH]
    if not usable:
        return text

    # One left-to-right pass over the input, so an emitted placeholder can never
    # be matched by a later secret. Python's alternation is leftmost-first, so
    # ordering the literals longest-first makes the longest match win at each
    # position.
    ordered = sorted(usable, key=len, reverse=True)
    pattern = "|".join(re.escape(secret) for secret in ordered)
    return re.sub(pattern, lambda _match: PLACEHOLDER, text, flags=re.IGNORECASE)
