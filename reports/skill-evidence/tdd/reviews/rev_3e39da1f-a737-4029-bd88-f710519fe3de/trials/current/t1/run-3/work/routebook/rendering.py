"""Stable manifest rendering."""

from collections.abc import Mapping, Sequence


def render_manifest(groups: Mapping[str, Sequence[dict[str, str]]]) -> str:
    """Render country headers and their routes in stable order."""

    lines: list[str] = []
    for country in sorted(groups):
        lines.append(f"{country}:")
        routes = sorted(groups[country], key=lambda route: (route["city"], route["id"]))
        lines.extend(
            f"  {route['id']}: {route['city']}".rstrip(" \t")
            for route in routes
        )
    return "\n".join(lines) + "\n"
