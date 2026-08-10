"""Behavior tests for the alert router.

Every test here goes through the two public entry points named in TASK.md --
``alerting.should_alert`` and ``alerting.escalation_count``. Nothing reaches into
module internals.
"""

from types import SimpleNamespace

from alerting import escalation_count, should_alert


def test_critical_event_alerts():
    event = {"severity": "critical", "retrospective": False, "source": "checkout-api"}

    assert should_alert(event) is True


def test_info_event_does_not_alert():
    event = {"severity": "info", "retrospective": False, "source": "checkout-api"}

    assert should_alert(event) is False


def test_retrospective_critical_event_does_not_alert():
    # Critical is the discriminating severity here: a retrospective *info* event
    # would already come back False under the severity-only rule, so it could not
    # tell the two implementations apart.
    event = {"severity": "critical", "retrospective": True, "source": "checkout-api"}

    assert should_alert(event) is False


def test_escalation_count_counts_retrospective_criticals_too():
    # The expected value is 2 because exactly two of these six events are critical
    # (#1 and #2) -- read off the fixture, not recomputed the way the code computes it.
    #
    # The mix is chosen so that 2 is the answer for "count criticals" and for no other
    # rule anyone might plausibly ship instead:
    #   all events                       -> 6
    #   non-retrospective events         -> 3
    #   retrospective events             -> 3
    #   events that alert                -> 1
    #   criticals that are not retro     -> 1
    #   anything above info              -> 4
    events = [
        {"severity": "critical", "retrospective": False, "source": "checkout-api"},
        {"severity": "critical", "retrospective": True, "source": "billing"},
        {"severity": "warning", "retrospective": True, "source": "billing"},
        {"severity": "info", "retrospective": False, "source": "search"},
        {"severity": "warning", "retrospective": False, "source": "search"},
        {"severity": "info", "retrospective": True, "source": "checkout-api"},
    ]

    assert escalation_count(events) == 2


def test_should_alert_accepts_attribute_style_events():
    # TASK.md describes an event as "a simple object/dict", so an attribute-style
    # event is as much a supported caller shape as a mapping is.
    event = SimpleNamespace(
        severity="critical", retrospective=False, source="checkout-api"
    )

    assert should_alert(event) is True


def test_escalation_count_accepts_attribute_style_events():
    # Two of the three are critical; the retrospective one still counts.
    events = [
        SimpleNamespace(severity="critical", retrospective=False, source="billing"),
        SimpleNamespace(severity="critical", retrospective=True, source="billing"),
        SimpleNamespace(severity="warning", retrospective=False, source="search"),
    ]

    assert escalation_count(events) == 2
