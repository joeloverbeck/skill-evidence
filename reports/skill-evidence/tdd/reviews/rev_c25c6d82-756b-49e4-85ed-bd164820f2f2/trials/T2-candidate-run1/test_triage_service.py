from triage_service import triage


def test_decided_records_are_reported_in_the_order_given():
    records = [
        {"id": "b-2", "severity": "high"},
        {"id": "a-1", "severity": "low"},
        {"id": "c-3", "severity": "medium"},
    ]

    assert triage(records)["processed"] == ["b-2", "a-1", "c-3"]


def test_result_has_exactly_the_processed_and_untestable_keys():
    records = [
        {"id": "b-2", "severity": "high"},
        {"id": "a-1", "severity": "low"},
    ]

    assert sorted(triage(records).keys()) == ["processed", "untestable"]


def test_undecidable_severity_is_recorded_and_processing_continues():
    records = [
        {"id": "r-1", "severity": "low"},
        {"id": "r-2", "severity": "critical"},
        {"id": "r-3", "severity": "high"},
        {"id": "r-4", "severity": "HIGH"},
        {"id": "r-5", "severity": "medium"},
    ]

    assert triage(records) == {
        "processed": ["r-1", "r-3", "r-5"],
        "untestable": ["r-2", "r-4"],
    }
