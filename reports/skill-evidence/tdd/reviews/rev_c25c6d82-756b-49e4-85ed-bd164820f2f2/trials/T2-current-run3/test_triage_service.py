from triage_service import triage


def test_triage_reports_exactly_processed_and_untestable():
    assert set(triage([]).keys()) == {"processed", "untestable"}


def test_decided_records_are_processed_in_the_order_given():
    records = [
        {"id": "T-3", "severity": "high"},
        {"id": "T-1", "severity": "low"},
        {"id": "T-2", "severity": "medium"},
    ]

    assert triage(records) == {
        "processed": ["T-3", "T-1", "T-2"],
        "untestable": [],
    }


def test_record_with_unrecognized_severity_is_untestable():
    records = [{"id": "T-9", "severity": "critical"}]

    assert triage(records) == {
        "processed": [],
        "untestable": ["T-9"],
    }


def test_later_records_are_still_triaged_after_an_undecidable_one():
    records = [
        {"id": "T-1", "severity": "low"},
        {"id": "T-2", "severity": "blocker"},
        {"id": "T-3", "severity": "high"},
        {"id": "T-4", "severity": ""},
        {"id": "T-5", "severity": "medium"},
    ]

    assert triage(records) == {
        "processed": ["T-1", "T-3", "T-5"],
        "untestable": ["T-2", "T-4"],
    }


def test_severity_matching_is_case_sensitive():
    records = [{"id": "T-7", "severity": "HIGH"}]

    assert triage(records) == {
        "processed": [],
        "untestable": ["T-7"],
    }
