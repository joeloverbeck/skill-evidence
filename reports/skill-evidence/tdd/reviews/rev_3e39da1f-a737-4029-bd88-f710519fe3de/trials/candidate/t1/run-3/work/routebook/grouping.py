"""Route grouping."""

from collections.abc import Iterable

from .normalization import normalize_destination


def group_routes(routes: Iterable[dict[str, str]]) -> dict[str, list[dict[str, str]]]:
    """Group route dictionaries by normalized country code."""

    grouped: dict[str, list[dict[str, str]]] = {}
    seen_ids: set[str] = set()
    for route in routes:
        route_id = route["id"]
        if route_id in seen_ids:
            raise ValueError(f"duplicate route id: {route_id}")
        seen_ids.add(route_id)
        city, country = normalize_destination(route["city"], route["country"])
        normalized_route = dict(route)
        normalized_route.update(city=city, country=country)
        grouped.setdefault(country, []).append(normalized_route)
    for grouped_routes in grouped.values():
        grouped_routes.sort(key=lambda route: (route["city"], route["id"]))
    return grouped
