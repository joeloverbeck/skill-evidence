from triage_service import triage


def test_decided_records_are_processed_in_the_order_given():
    records = [
        {"id": "r-3", "severity": "high"},
        {"id": "r-1", "severity": "low"},
        {"id": "r-2", "severity": "medium"},
    ]

    assert triage(records)["processed"] == ["r-3", "r-1", "r-2"]


def test_result_has_exactly_the_processed_and_untestable_keys():
    records = [{"id": "r-1", "severity": "low"}]

    assert sorted(triage(records).keys()) == ["processed", "untestable"]


def test_record_with_an_undecidable_severity_is_listed_untestable():
    records = [{"id": "r-9", "severity": "critical"}]

    assert triage(records)["untestable"] == ["r-9"]


def test_processing_carries_on_past_an_undecidable_record():
    records = [
        {"id": "r-1", "severity": "critical"},
        {"id": "r-2", "severity": "low"},
    ]

    assert triage(records)["processed"] == ["r-2"]
