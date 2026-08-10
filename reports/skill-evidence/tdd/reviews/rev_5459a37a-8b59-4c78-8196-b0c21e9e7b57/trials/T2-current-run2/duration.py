"""Human-readable formatting of durations given in milliseconds."""


def format_duration(ms):
    """Return a human-readable rendering of a duration of ``ms`` milliseconds.

    Raises:
        ValueError: if ``ms`` is negative.
    """
    if ms < 0:
        raise ValueError(f"duration must not be negative, got {ms}")
    if 0 < ms < 1000:
        return f"{ms / 1000:.2f}s"
    minutes, seconds = divmod(ms // 1000, 60)
    hours, minutes = divmod(minutes, 60)
    if hours:
        return f"{hours}h {minutes}m {seconds}s"
    if minutes:
        return f"{minutes}m {seconds}s"
    return f"{seconds}s"
