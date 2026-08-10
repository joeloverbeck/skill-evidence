"""Replace secrets in free text with a fixed placeholder."""

PLACEHOLDER = "[REDACTED]"

MIN_SECRET_LENGTH = 4


def redact(text: str, secrets: list[str]) -> str:
    """Return `text` with every occurrence of every secret replaced by `[REDACTED]`.

    Matching is case-insensitive; text that is not part of a secret keeps its
    original casing. Where two secrets match at the same position, the longer
    one is redacted. Secrets shorter than `MIN_SECRET_LENGTH` are ignored: they
    are too short to be secrets, and redacting them would shred ordinary text.
    """
    candidates = sorted(
        (secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH),
        key=len,
        reverse=True,
    )

    redacted: list[str] = []
    position = 0
    while position < len(text):
        match = _longest_match_at(text, position, candidates)
        if match is None:
            redacted.append(text[position])
            position += 1
        else:
            redacted.append(PLACEHOLDER)
            position += len(match)
    return "".join(redacted)


def _longest_match_at(text: str, position: int, candidates: list[str]) -> str | None:
    """Return the first of `candidates` (longest first) matching `text` at `position`."""
    for secret in candidates:
        if text[position : position + len(secret)].lower() == secret.lower():
            return secret
    return None
