"""Stable manifest rendering."""

from collections.abc import Mapping, Sequence


def render_manifest(groups: Mapping[str, Sequence[dict[str, str]]]) -> str:
    """Render the legacy debug representation."""

    return repr(dict(groups))
