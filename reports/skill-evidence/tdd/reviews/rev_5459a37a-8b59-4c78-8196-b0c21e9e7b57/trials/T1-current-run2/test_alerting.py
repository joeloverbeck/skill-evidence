from alerting import escalation_count, should_alert


def test_critical_event_alerts():
    event = {"severity": "critical", "retrospective": False, "source": "payments-api"}

    assert should_alert(event) is True


def test_info_event_does_not_alert():
    event = {"severity": "info", "retrospective": False, "source": "payments-api"}

    assert should_alert(event) is False


def test_retrospective_critical_event_does_not_alert():
    event = {"severity": "critical", "retrospective": True, "source": "payments-api"}

    assert should_alert(event) is False


def test_escalation_count_counts_criticals_including_retrospective():
    # Hand-counted from TASK.md behavior 4: three of these five events are
    # critical. Alternative rules give different answers on this input --
    # "criticals that alert" gives 1, "any retrospective event" gives 4,
    # "anything above info" gives 4, "every event" gives 5.
    events = [
        {"severity": "critical", "retrospective": False, "source": "payments-api"},
        {"severity": "critical", "retrospective": True, "source": "payments-api"},
        {"severity": "critical", "retrospective": True, "source": "billing-worker"},
        {"severity": "warning", "retrospective": True, "source": "billing-worker"},
        {"severity": "info", "retrospective": True, "source": "search-indexer"},
    ]

    assert escalation_count(events) == 3
