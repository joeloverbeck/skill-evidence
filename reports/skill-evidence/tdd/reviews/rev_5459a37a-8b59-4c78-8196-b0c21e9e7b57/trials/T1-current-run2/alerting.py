"""Alert routing for incoming events.

An event is a mapping with the fields ``severity`` (one of ``"critical"``,
``"warning"``, ``"info"``), ``retrospective`` (bool), and ``source`` (str).
"""


def should_alert(event):
    """Return True when the event should raise an alert."""
    if event["retrospective"]:
        return False
    return event["severity"] == "critical"


def escalation_count(events):
    """Return how many events are critical, retrospective ones included."""
    return sum(1 for event in events if event["severity"] == "critical")
