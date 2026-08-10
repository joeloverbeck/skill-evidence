"""Alert router.

Public seams: `should_alert` and `escalation_count`.
"""

from collections.abc import Mapping


def _field(event, name):
    """Read `name` off an event in either form TASK.md allows: dict or object."""
    if isinstance(event, Mapping):
        return event[name]
    return getattr(event, name)


def should_alert(event):
    if _field(event, "retrospective"):
        return False
    return _field(event, "severity") == "critical"


def escalation_count(events):
    return sum(1 for event in events if _field(event, "severity") == "critical")
