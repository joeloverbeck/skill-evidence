"""Alert router.

An event is a mapping with the fields `severity`, `retrospective` and `source`.
"""


def should_alert(event):
    """Return True when the event warrants an alert.

    Retrospective events are never alerted on, whatever their severity.
    """
    if event["retrospective"]:
        return False
    return event["severity"] == "critical"


def escalation_count(events):
    """Return how many of `events` are critical.

    Escalation counting is deliberately independent of alerting: a retrospective
    critical is not alerted on but still counts here.
    """
    return sum(1 for event in events if event["severity"] == "critical")
