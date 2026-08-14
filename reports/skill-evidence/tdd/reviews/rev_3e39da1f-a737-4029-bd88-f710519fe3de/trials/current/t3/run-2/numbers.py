"""Small public numeric helpers."""


def clamp(value: int, minimum: int, maximum: int) -> int:
    if value < minimum:
        return minimum
    raise NotImplementedError
