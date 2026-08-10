from triage_service import triage


def test_decided_record_id_goes_in_processed():
    result = triage([{"id": "r-1", "severity": "high"}])

    assert result == {"processed": ["r-1"], "untestable": []}


def test_processed_ids_keep_the_order_the_records_were_given():
    result = triage(
        [
            {"id": "z-1", "severity": "high"},
            {"id": "a-9", "severity": "high"},
            {"id": "m-3", "severity": "high"},
        ]
    )

    assert result == {"processed": ["z-1", "a-9", "m-3"], "untestable": []}


def test_undecidable_severity_is_untestable_and_later_records_still_processed():
    result = triage(
        [
            {"id": "z-1", "severity": "catastrophic"},
            {"id": "r-2", "severity": "high"},
            {"id": "a-1", "severity": ""},
        ]
    )

    assert result == {"processed": ["r-2"], "untestable": ["z-1", "a-1"]}


def test_low_and_medium_are_decided_severities_too():
    result = triage(
        [
            {"id": "z-1", "severity": "low"},
            {"id": "a-9", "severity": "medium"},
        ]
    )

    assert result == {"processed": ["z-1", "a-9"], "untestable": []}
