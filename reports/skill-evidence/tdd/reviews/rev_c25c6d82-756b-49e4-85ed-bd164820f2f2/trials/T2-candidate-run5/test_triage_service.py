from triage_service import triage


def test_decided_records_are_processed_in_input_order():
    result = triage(
        [
            {"id": "r-9", "severity": "high"},
            {"id": "r-2", "severity": "low"},
            {"id": "r-5", "severity": "medium"},
        ]
    )

    assert result["processed"] == ["r-9", "r-2", "r-5"]


def test_result_carries_exactly_the_processed_and_untestable_keys():
    result = triage([{"id": "r-9", "severity": "high"}])

    assert set(result) == {"processed", "untestable"}


def test_undecidable_records_are_recorded_and_processing_continues():
    result = triage(
        [
            {"id": "r-3", "severity": "high"},
            {"id": "r-8", "severity": "critical"},
            {"id": "r-1", "severity": "medium"},
            {"id": "r-6", "severity": "HIGH"},
            {"id": "r-4", "severity": "low"},
        ]
    )

    assert result == {
        "processed": ["r-3", "r-1", "r-4"],
        "untestable": ["r-8", "r-6"],
    }
