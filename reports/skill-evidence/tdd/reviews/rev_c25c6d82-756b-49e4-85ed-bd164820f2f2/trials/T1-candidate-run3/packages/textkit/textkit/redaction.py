"""Replace known secrets in free text with a fixed placeholder."""

import re

_PLACEHOLDER = "[REDACTED]"

# Shorter than this and a "secret" is too common to redact without shredding
# ordinary text, so it is ignored entirely.
_MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Return `text` with every occurrence of every secret replaced.

    Matching ignores case, in both the text and the secret. Where two secrets
    match at the same position, the longer one is redacted. Secrets shorter
    than four characters are ignored.
    """
    # One left-to-right pass: `re` takes the first alternative that matches at a
    # position, so ordering the alternatives longest-first makes the longest
    # secret win a tie.
    usable = [secret for secret in secrets if len(secret) >= _MIN_SECRET_LENGTH]
    if not usable:
        # An empty alternation would match at every position.
        return text
    alternatives = sorted(usable, key=len, reverse=True)
    pattern = "|".join(re.escape(secret) for secret in alternatives)
    return re.sub(pattern, _PLACEHOLDER, text, flags=re.IGNORECASE)
