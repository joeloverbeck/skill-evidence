"""Human-readable formatting of durations expressed in milliseconds."""


def format_duration(ms):
    """Render a duration given in milliseconds as a human-readable string."""
    if ms < 0:
        raise ValueError(f"duration must not be negative (got {ms} milliseconds)")
    if ms == 0:
        return "0s"
    if ms < 1000:
        return f"{ms / 1000:.2f}s"

    minutes, seconds = divmod(ms // 1000, 60)
    hours, minutes = divmod(minutes, 60)
    if hours:
        return f"{hours}h {minutes}m {seconds}s"
    if minutes:
        return f"{minutes}m {seconds}s"
    return f"{seconds}s"
