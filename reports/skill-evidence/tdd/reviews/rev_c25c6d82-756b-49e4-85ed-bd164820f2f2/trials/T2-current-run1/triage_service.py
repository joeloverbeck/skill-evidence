"""Triage of severity-tagged records into decided and undecidable ids."""

DECIDABLE_SEVERITIES = frozenset({"low", "medium", "high"})


def triage(records):
    """Split record ids into those that could be decided and those that could not."""
    processed = []
    untestable = []
    for record in records:
        decided = record["severity"] in DECIDABLE_SEVERITIES
        (processed if decided else untestable).append(record["id"])
    return {"processed": processed, "untestable": untestable}
