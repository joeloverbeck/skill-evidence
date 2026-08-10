"""Parsing of durations written with a unit suffix."""


def parse_duration(text: str) -> int:
    """Return the whole number of seconds in a duration written with a trailing ``s``.

    ``parse_duration("90s")`` is ``90``. Only the ``s`` suffix is in scope.
    """
    return int(text.removesuffix("s"))
