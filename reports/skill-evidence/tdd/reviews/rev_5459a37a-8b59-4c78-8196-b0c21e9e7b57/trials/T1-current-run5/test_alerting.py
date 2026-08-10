"""Behavior tests for the alert router.

Every test here goes through the public seams named in SEAMS.md:
`alerting.should_alert` and `alerting.escalation_count`. Nothing private is
imported and nothing is observed through a side channel.
"""

from dataclasses import dataclass

from alerting import escalation_count, should_alert


@dataclass(frozen=True)
class Event:
    """The attribute-style event form TASK.md names alongside dicts.

    Test-local on purpose: the seams accept any event carrying these fields,
    so the tests must not depend on a production event type to say so.
    """

    severity: str
    retrospective: bool
    source: str


def test_critical_event_alerts():
    event = {"severity": "critical", "retrospective": False, "source": "checkout-api"}

    assert should_alert(event) is True


def test_info_event_does_not_alert():
    event = {"severity": "info", "retrospective": False, "source": "checkout-api"}

    assert should_alert(event) is False


def test_retrospective_critical_event_does_not_alert():
    # `critical` is the one severity the task pins as alerting, so it is the
    # only severity where retrospective suppression is observable at all.
    event = {"severity": "critical", "retrospective": True, "source": "checkout-api"}

    assert should_alert(event) is False


def test_escalation_count_includes_retrospective_criticals():
    # The golden is chosen so that only the required rule yields 2:
    #   required (all criticals, retrospective or not) -> 2
    #   criticals that alert (the rejected "count what paged us" rule) -> 1
    #   every event -> 4        every retrospective event -> 1
    #   every non-info event -> 3
    events = [
        {"severity": "critical", "retrospective": True, "source": "checkout-api"},
        {"severity": "critical", "retrospective": False, "source": "billing"},
        {"severity": "warning", "retrospective": False, "source": "billing"},
        {"severity": "info", "retrospective": False, "source": "checkout-api"},
    ]

    assert escalation_count(events) == 2


def test_should_alert_accepts_attribute_style_events():
    event = Event(severity="critical", retrospective=False, source="checkout-api")

    assert should_alert(event) is True


def test_escalation_count_accepts_attribute_style_events():
    # Same discriminating golden as the dict case, so both event forms are held
    # to one rule: only "all criticals, retrospective or not" gives 2.
    events = [
        Event(severity="critical", retrospective=True, source="checkout-api"),
        Event(severity="critical", retrospective=False, source="billing"),
        Event(severity="warning", retrospective=False, source="billing"),
        Event(severity="info", retrospective=False, source="checkout-api"),
    ]

    assert escalation_count(events) == 2
