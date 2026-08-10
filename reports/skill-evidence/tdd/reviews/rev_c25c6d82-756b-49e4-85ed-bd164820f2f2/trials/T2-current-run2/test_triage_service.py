from triage_service import triage


def test_triage_returns_exactly_the_processed_and_untestable_keys():
    result = triage([{"id": "r1", "severity": "low"}])

    assert sorted(result) == ["processed", "untestable"]


def test_low_medium_and_high_records_are_processed_in_the_given_order():
    records = [
        {"id": "r2", "severity": "medium"},
        {"id": "r3", "severity": "high"},
        {"id": "r1", "severity": "low"},
    ]

    assert triage(records)["processed"] == ["r2", "r3", "r1"]


def test_undecidable_records_are_untestable_and_later_records_still_process():
    records = [
        {"id": "r1", "severity": "high"},
        {"id": "r2", "severity": "unknown"},
        {"id": "r3", "severity": "low"},
        {"id": "r4", "severity": ""},
        {"id": "r5", "severity": "medium"},
    ]

    result = triage(records)

    assert result["untestable"] == ["r2", "r4"]
    assert result["processed"] == ["r1", "r3", "r5"]
