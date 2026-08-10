"""Behavior tests for the alert router.

Every test calls only the public functions named in TASK.md:
`alerting.should_alert` and `alerting.escalation_count`.
"""

from alerting import escalation_count, should_alert


def event(severity, retrospective=False, source="checkout-api"):
    """Build an event exactly as TASK.md describes one: severity, retrospective, source."""
    return {
        "severity": severity,
        "retrospective": retrospective,
        "source": source,
    }


def test_critical_event_alerts():
    assert should_alert(event("critical")) is True


def test_info_event_does_not_alert():
    assert should_alert(event("info")) is False


def test_retrospective_critical_event_does_not_alert():
    # `critical` is the discriminating severity here: a severity-only router would
    # still alert on this event. A retrospective `info` event would pass under that
    # rejected implementation too, so it would prove nothing.
    assert should_alert(event("critical", retrospective=True)) is False


def test_escalation_count_includes_retrospective_criticals():
    # Four criticals, two of them retrospective, among two non-critical events.
    # The expected 4 is counted by hand from this list against the rule in TASK.md
    # ("how many of them are severity=critical, whether or not retrospective"),
    # and it separates that rule from every plausible rival counting rule:
    #   count every event .................... 6
    #   count only events that alert ......... 2  (drops the retrospective criticals)
    #   count retrospective events ........... 3
    #   count non-retrospective events ....... 3
    #   count non-critical events ............ 2
    events = [
        event("critical", retrospective=False, source="checkout-api"),
        event("critical", retrospective=True, source="billing-worker"),
        event("critical", retrospective=True, source="billing-worker"),
        event("critical", retrospective=False, source="search-indexer"),
        event("warning", retrospective=False, source="checkout-api"),
        event("info", retrospective=True, source="cron-sweeper"),
    ]

    assert escalation_count(events) == 4
