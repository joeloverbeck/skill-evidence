def parse_duration(text: str) -> int:
    """Convert a duration written with a trailing ``s`` into whole seconds.

    Only the ``s`` suffix is in scope; no other unit is recognised.
    """
    return int(text.removesuffix("s"))
