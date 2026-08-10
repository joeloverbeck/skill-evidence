"""Replace secrets in free text with a fixed placeholder."""

import re

PLACEHOLDER = "[REDACTED]"

# Anything shorter than this is ordinary text, not a secret: redacting it would
# shred the surrounding prose.
MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Return `text` with every occurrence of every secret replaced.

    Matching ignores case; the surrounding text keeps its own casing. Where two
    secrets match at the same position, the longer one wins. Secrets shorter
    than `MIN_SECRET_LENGTH` are ignored.
    """
    usable = [secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH]
    if not usable:
        return text

    # One pass over the input: alternation is leftmost-first, so ordering the
    # branches longest-first makes the longest secret win at a given position,
    # and a placeholder written into the output is never rescanned.
    pattern = "|".join(
        re.escape(secret) for secret in sorted(usable, key=len, reverse=True)
    )
    return re.sub(pattern, PLACEHOLDER, text, flags=re.IGNORECASE)
