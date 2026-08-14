"""Route grouping."""

from collections.abc import Iterable

from .normalization import normalize_destination


def group_routes(routes: Iterable[dict[str, str]]) -> dict[str, list[dict[str, str]]]:
    """Group route dictionaries by normalized country code."""

    grouped: dict[str, list[dict[str, str]]] = {}
    route_ids: set[str] = set()
    for route in routes:
        if route["id"] in route_ids:
            raise ValueError(f'duplicate route ID: {route["id"]}')
        route_ids.add(route["id"])

        _, country = normalize_destination(route["city"], route["country"])
        grouped.setdefault(country, []).append(dict(route))

    for grouped_routes in grouped.values():
        grouped_routes.sort(
            key=lambda route: (
                normalize_destination(route["city"], route["country"])[0],
                route["id"],
            )
        )
    return grouped
