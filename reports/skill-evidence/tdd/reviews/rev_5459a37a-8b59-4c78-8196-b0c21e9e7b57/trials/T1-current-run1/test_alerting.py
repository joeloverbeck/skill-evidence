"""Behavior tests for the alert router.

Every test here calls only the two public functions of `alerting`, the seams
ratified by TASK.md. Nothing reaches into module internals.
"""

from types import SimpleNamespace

from alerting import escalation_count, should_alert


def event(severity, retrospective=False, source="checkout-api"):
    """Build an event as the mapping shape TASK.md describes."""
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
    # The severity that alerts loudest, so the only thing that can suppress it
    # here is the retrospective flag.
    assert should_alert(event("critical", retrospective=True)) is False


def test_escalation_count_counts_retrospective_criticals_too():
    # Chosen so the required answer differs from every rejected rule:
    # criticals = 3, but "only what alerts" = 1, "everything" = 6,
    # "retrospective only" = 4, "non-retrospective only" = 2, and
    # "critical or warning" = 4. Only the rule TASK.md fixes yields 3.
    events = [
        event("critical", retrospective=False),
        event("critical", retrospective=True),
        event("critical", retrospective=True),
        event("warning", retrospective=True),
        event("info", retrospective=True),
        event("info", retrospective=False),
    ]

    assert escalation_count(events) == 3
    # The rejected alternative, measured through the other public seam: a
    # router that counted alerts would report 1 for this same list.
    assert sum(1 for e in events if should_alert(e)) == 1


def test_should_alert_reads_attribute_style_events():
    # TASK.md describes an event as "a simple object/dict", so an object with
    # the same fields is an equally valid caller shape.
    assert should_alert(SimpleNamespace(**event("critical"))) is True
    assert should_alert(SimpleNamespace(**event("critical", retrospective=True))) is False


def test_escalation_count_reads_attribute_style_events():
    events = [
        SimpleNamespace(**event("critical", retrospective=True)),
        SimpleNamespace(**event("warning", retrospective=False)),
        SimpleNamespace(**event("critical", retrospective=False)),
    ]

    assert escalation_count(events) == 2
