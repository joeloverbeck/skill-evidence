"""Manifest statistics."""

from collections.abc import Mapping, Sequence


def manifest_stats(groups: Mapping[str, Sequence[object]]) -> dict[str, object]:
    """Return the legacy total-only statistics mapping."""

    return {"total": sum(len(routes) for routes in groups.values())}
