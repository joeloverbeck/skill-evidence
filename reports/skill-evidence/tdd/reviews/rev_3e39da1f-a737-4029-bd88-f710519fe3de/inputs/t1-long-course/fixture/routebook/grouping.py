"""Route grouping."""

from collections.abc import Iterable


def group_routes(routes: Iterable[dict[str, str]]) -> dict[str, list[dict[str, str]]]:
    """Group legacy route dictionaries by their country spelling."""

    grouped: dict[str, list[dict[str, str]]] = {}
    for route in routes:
        grouped.setdefault(route["country"], []).append(dict(route))
    return grouped
