"""Manifest statistics."""

from collections.abc import Mapping, Sequence


def manifest_stats(groups: Mapping[str, Sequence[object]]) -> dict[str, object]:
    """Return aggregate and alphabetically ordered per-country counts."""

    countries = {
        country: len(groups[country])
        for country in sorted(groups)
    }
    return {
        "total": sum(countries.values()),
        "countries": countries,
    }
