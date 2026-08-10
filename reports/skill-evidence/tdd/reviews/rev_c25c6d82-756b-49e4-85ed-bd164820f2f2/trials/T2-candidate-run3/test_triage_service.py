from triage_service import triage


def test_returns_exactly_processed_and_untestable_keys():
    result = triage([])

    assert sorted(result.keys()) == ["processed", "untestable"]


def test_low_medium_and_high_are_processed_in_given_order():
    records = [
        {"id": "r3", "severity": "low"},
        {"id": "r1", "severity": "high"},
        {"id": "r2", "severity": "medium"},
    ]

    result = triage(records)

    assert result["processed"] == ["r3", "r1", "r2"]
    assert result["untestable"] == []


def test_undecidable_severity_is_untestable_and_later_records_still_processed():
    records = [
        {"id": "r1", "severity": "high"},
        {"id": "r2", "severity": "critical"},
        {"id": "r3", "severity": "low"},
        {"id": "r4", "severity": "HIGH"},
    ]

    result = triage(records)

    assert result["untestable"] == ["r2", "r4"]
    assert result["processed"] == ["r1", "r3"]
