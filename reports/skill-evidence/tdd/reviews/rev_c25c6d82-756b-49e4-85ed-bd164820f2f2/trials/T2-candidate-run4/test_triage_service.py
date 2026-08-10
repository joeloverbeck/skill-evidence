from triage_service import triage


def test_no_records_gives_empty_processed_and_untestable_lists():
    assert triage([]) == {"processed": [], "untestable": []}


def test_low_medium_and_high_records_are_processed_in_the_order_given():
    records = [
        {"id": "r3", "severity": "high"},
        {"id": "r1", "severity": "low"},
        {"id": "r2", "severity": "medium"},
    ]

    assert triage(records) == {"processed": ["r3", "r1", "r2"], "untestable": []}


def test_undecidable_severities_are_untestable_and_later_records_still_process():
    records = [
        {"id": "r5", "severity": "high"},
        {"id": "r9", "severity": "critical"},
        {"id": "r1", "severity": "low"},
        {"id": "r2", "severity": "urgent"},
    ]

    assert triage(records) == {
        "processed": ["r5", "r1"],
        "untestable": ["r9", "r2"],
    }
