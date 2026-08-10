from triage_service import triage


def test_triage_reports_exactly_processed_and_untestable():
    assert triage([]) == {"processed": [], "untestable": []}


def test_decided_records_are_processed_in_the_order_given():
    # Given in an order that no sort by id, nor by severity rank, would produce.
    records = [
        {"id": "r3", "severity": "high"},
        {"id": "r1", "severity": "low"},
        {"id": "r2", "severity": "medium"},
    ]

    assert triage(records) == {"processed": ["r3", "r1", "r2"], "untestable": []}


def test_undecidable_record_is_reported_and_the_rest_still_processed():
    # The undecidable record sits in the middle, so stopping there would lose "c".
    records = [
        {"id": "a", "severity": "low"},
        {"id": "b", "severity": "critical"},
        {"id": "c", "severity": "high"},
    ]

    assert triage(records) == {"processed": ["a", "c"], "untestable": ["b"]}
