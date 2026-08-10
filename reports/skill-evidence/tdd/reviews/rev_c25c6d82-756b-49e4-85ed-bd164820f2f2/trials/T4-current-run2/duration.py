def parse_duration(text: str) -> int:
    """Convert a duration written with a trailing "s" to whole seconds."""
    return int(text.removesuffix("s"))
