"""Redaction of known secrets out of free text."""

import re

PLACEHOLDER = "[REDACTED]"

#: Secrets shorter than this are too short to be secrets, and redacting them
#: would shred ordinary text.
MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Replace every occurrence of each secret in `text` with `[REDACTED]`.

    Matching is case-insensitive. Where two secrets match at the same position,
    the longer one wins. Secrets shorter than `MIN_SECRET_LENGTH` are ignored.
    """
    # One left-to-right pass over the original text. Python's alternation is
    # leftmost-first, so ordering the branches longest-first makes the longest
    # secret win wherever several match at the same position.
    usable = [secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH]
    if not usable:
        # An empty alternation matches at every position and would shred the text.
        return text
    by_length = sorted(usable, key=len, reverse=True)
    pattern = "|".join(re.escape(secret) for secret in by_length)
    return re.sub(pattern, PLACEHOLDER, text, flags=re.IGNORECASE)
