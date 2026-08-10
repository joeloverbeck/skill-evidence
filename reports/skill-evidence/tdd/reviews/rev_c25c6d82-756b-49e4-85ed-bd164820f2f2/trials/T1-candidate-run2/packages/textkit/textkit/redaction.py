import re

REDACTION_MARKER = "[REDACTED]"
MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Replace every occurrence of each secret in ``text`` with ``[REDACTED]``.

    Matching ignores case. Where two secrets match at the same position, the longer
    one wins. Secrets shorter than four characters are ignored.
    """
    usable = [secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH]
    if not usable:
        return text

    # Longest first: Python's alternation takes the first branch that matches at a
    # position, so this is what makes the longer secret win. One `sub` pass over the
    # original text also keeps the scanner off the markers it just wrote.
    longest_first = sorted(usable, key=len, reverse=True)
    pattern = re.compile(
        "|".join(re.escape(secret) for secret in longest_first), re.IGNORECASE
    )
    return pattern.sub(REDACTION_MARKER, text)
