"""Alert router."""

from collections.abc import Mapping


def _field(event, name):
    """Read a field from an event given as either a mapping or an object."""
    if isinstance(event, Mapping):
        return event[name]
    return getattr(event, name)


def should_alert(event):
    """Return True when the event warrants an alert."""
    if _field(event, "retrospective"):
        return False
    return _field(event, "severity") == "critical"


def escalation_count(events):
    """Count the critical events, retrospective or not."""
    return sum(1 for event in events if _field(event, "severity") == "critical")
