"""Stable manifest rendering."""

from collections.abc import Mapping, Sequence


def render_manifest(groups: Mapping[str, Sequence[dict[str, str]]]) -> str:
    """Render countries and their routes in a stable text format."""

    lines: list[str] = []
    for country in sorted(groups):
        lines.append(f"{country}:")
        for route in groups[country]:
            lines.append(f"  {route['id']}: {route['city']}")
    physical_lines = (
        physical_line
        for logical_line in lines
        for physical_line in logical_line.splitlines()
    )
    return "\n".join(line.rstrip(" \t") for line in physical_lines) + "\n"
