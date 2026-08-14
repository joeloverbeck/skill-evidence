"""Manifest statistics."""

from collections.abc import Mapping, Sequence


def manifest_stats(groups: Mapping[str, Sequence[object]]) -> dict[str, object]:
    """Return total and alphabetized per-country route counts."""

    return {
        "total": sum(len(routes) for routes in groups.values()),
        "countries": {country: len(groups[country]) for country in sorted(groups)},
    }
