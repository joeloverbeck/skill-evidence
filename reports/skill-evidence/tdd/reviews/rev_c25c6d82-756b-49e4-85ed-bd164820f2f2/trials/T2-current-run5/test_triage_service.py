from triage_service import triage


def test_triage_of_no_records_reports_two_empty_buckets():
    assert triage([]) == {"processed": [], "untestable": []}


def test_decided_records_are_processed_in_the_order_given():
    # Given in an order that no severity ranking would reproduce: a severity-sorted
    # result would be ["r-2", "r-3", "r-1"] (low, medium, high) or ["r-1", "r-3", "r-2"]
    # (high first). The required result is the order the caller supplied.
    records = [
        {"id": "r-1", "severity": "high"},
        {"id": "r-2", "severity": "low"},
        {"id": "r-3", "severity": "medium"},
    ]

    assert triage(records) == {
        "processed": ["r-1", "r-2", "r-3"],
        "untestable": [],
    }


def test_undecidable_severities_are_set_aside_and_the_rest_still_process():
    # "HIGH" is not "high": a severity that only matches after case folding is still
    # undecidable. The undecidable records sit mid-list, so an implementation that
    # stopped at the first one would yield processed ["r-1"], and one that dropped
    # them would yield untestable [].
    records = [
        {"id": "r-1", "severity": "low"},
        {"id": "r-2", "severity": "critical"},
        {"id": "r-3", "severity": "HIGH"},
        {"id": "r-4", "severity": "high"},
    ]

    assert triage(records) == {
        "processed": ["r-1", "r-4"],
        "untestable": ["r-2", "r-3"],
    }
