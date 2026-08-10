"""Redaction of known secrets from free text."""

import re

PLACEHOLDER = "[REDACTED]"

#: Anything shorter than this is too short to be a secret; redacting it would shred
#: ordinary text.
MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Return ``text`` with every occurrence of every secret replaced by ``[REDACTED]``.

    Matching is case-insensitive. Where two secrets match at the same position, the
    longer one wins. Secrets shorter than ``MIN_SECRET_LENGTH`` are ignored entirely.
    """
    usable = [secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH]
    # Longest first: regex alternation is leftmost-first, so ordering the branches by
    # descending length is what makes the longer secret win at a shared position.
    longest_first = sorted(usable, key=len, reverse=True)
    if not longest_first:
        # An empty alternation would match the empty string at every position.
        return text
    pattern = re.compile(
        "|".join(re.escape(secret) for secret in longest_first), re.IGNORECASE
    )
    return pattern.sub(PLACEHOLDER, text)
