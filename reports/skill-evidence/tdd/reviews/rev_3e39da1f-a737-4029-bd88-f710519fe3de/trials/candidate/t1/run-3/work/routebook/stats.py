"""Manifest statistics."""

from collections.abc import Mapping, Sequence


def manifest_stats(groups: Mapping[str, Sequence[object]]) -> dict[str, object]:
    """Return route totals overall and by country."""

    country_counts = {country: len(groups[country]) for country in sorted(groups)}
    return {
        "total": sum(country_counts.values()),
        "countries": country_counts,
    }
