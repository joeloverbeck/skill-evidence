"""Public batch policy."""


def allows_batch(size: int) -> bool:
    return size <= 50
