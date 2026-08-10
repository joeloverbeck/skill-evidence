"""Replace secrets in free text with a fixed placeholder."""

PLACEHOLDER = "[REDACTED]"

MIN_SECRET_LENGTH = 4
"""Shorter strings are too common in ordinary prose to redact."""


def redact(text: str, secrets: list[str]) -> str:
    """Return `text` with every occurrence of every secret replaced."""
    secrets = [secret for secret in secrets if len(secret) >= MIN_SECRET_LENGTH]
    out: list[str] = []
    position = 0
    while position < len(text):
        width = _longest_match_at(text, position, secrets)
        if width:
            out.append(PLACEHOLDER)
            position += width
        else:
            out.append(text[position])
            position += 1
    return "".join(out)


def _longest_match_at(text: str, position: int, secrets: list[str]) -> int:
    """Width of the longest secret matching at `position`, or 0 if none does."""
    return max(
        (
            len(secret)
            for secret in secrets
            if text[position : position + len(secret)].lower() == secret.lower()
        ),
        default=0,
    )
