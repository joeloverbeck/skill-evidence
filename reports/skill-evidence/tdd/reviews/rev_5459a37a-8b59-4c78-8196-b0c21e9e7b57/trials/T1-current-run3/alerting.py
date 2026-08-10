"""Alert router.

Public interface (see TASK.md):

- ``should_alert(event) -> bool``
- ``escalation_count(events) -> int``

An event carries ``severity`` (``"critical"``, ``"warning"`` or ``"info"``),
``retrospective`` (bool) and ``source`` (str), and may be supplied either as a
mapping or as an object with those attributes.
"""

from collections.abc import Mapping


def _field(event, name):
    """Read ``name`` off an event given either as a mapping or as a plain object."""
    if isinstance(event, Mapping):
        return event[name]
    return getattr(event, name)


def should_alert(event):
    """Return True when this event should raise an alert."""
    if _field(event, "retrospective"):
        return False
    return _field(event, "severity") == "critical"


def escalation_count(events):
    """Return how many of ``events`` are critical, retrospective or not."""
    return sum(1 for event in events if _field(event, "severity") == "critical")
