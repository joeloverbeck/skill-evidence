"""Redaction of known secrets from free text."""

import re

REDACTION_MARKER = "[REDACTED]"

#: Shorter strings than this are too common in ordinary prose to redact.
MINIMUM_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Replace every occurrence of every secret in ``text`` with the marker.

    Where two secrets match at the same position, the longer one wins.
    Secrets shorter than :data:`MINIMUM_SECRET_LENGTH` are ignored entirely.
    """
    usable = [secret for secret in secrets if len(secret) >= MINIMUM_SECRET_LENGTH]
    if not usable:
        return text
    # One left-to-right pass over the text: alternatives are tried longest
    # first, so the longest secret matching at a position is the one replaced.
    longest_first = sorted(usable, key=lambda secret: (-len(secret), secret))
    pattern = "|".join(re.escape(secret) for secret in longest_first)
    return re.sub(pattern, REDACTION_MARKER, text, flags=re.IGNORECASE)
