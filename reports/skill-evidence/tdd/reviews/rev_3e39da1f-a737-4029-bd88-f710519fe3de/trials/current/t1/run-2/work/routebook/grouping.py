"""Route grouping."""

from collections.abc import Iterable

from .normalization import normalize_destination


def group_routes(routes: Iterable[dict[str, str]]) -> dict[str, list[dict[str, str]]]:
    """Group routes by normalized country code."""

    grouped: dict[str, list[dict[str, str]]] = {}
    route_ids: set[str] = set()
    for route in routes:
        if route["id"] in route_ids:
            raise ValueError(f"duplicate route id: {route['id']}")
        route_ids.add(route["id"])
        city, country = normalize_destination(route["city"], route["country"])
        normalized_route = dict(route)
        normalized_route.update(city=city, country=country)
        grouped.setdefault(country, []).append(normalized_route)
    for country_routes in grouped.values():
        country_routes.sort(key=lambda route: (route["city"], route["id"]))
    return grouped
