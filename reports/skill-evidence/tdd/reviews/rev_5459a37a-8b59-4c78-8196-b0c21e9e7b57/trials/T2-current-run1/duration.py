"""Render a duration in milliseconds as a human-readable string."""


def format_duration(ms):
    """Return ``ms`` milliseconds as a duration string.

    Zero renders as ``"0s"``. A sub-second duration renders with two decimal
    places (``450`` -> ``"0.45s"``). From one second up, the duration renders as
    whole seconds, rolling over into minutes and then hours as each unit fills
    (``90000`` -> ``"1m 30s"``, ``3723000`` -> ``"1h 2m 3s"``).

    Raises ``ValueError`` if ``ms`` is negative.
    """
    if ms < 0:
        raise ValueError(f"duration must not be negative, got {ms}")
    if ms == 0:
        return "0s"
    if ms < 1000:
        return f"{ms / 1000:.2f}s"
    seconds = ms // 1000
    if seconds < 60:
        return f"{seconds}s"
    minutes, seconds = divmod(seconds, 60)
    if minutes < 60:
        return f"{minutes}m {seconds}s"
    hours, minutes = divmod(minutes, 60)
    return f"{hours}h {minutes}m {seconds}s"
