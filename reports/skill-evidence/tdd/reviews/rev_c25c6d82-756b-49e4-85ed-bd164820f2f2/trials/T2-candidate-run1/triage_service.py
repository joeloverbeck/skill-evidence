"""Triage of severity-tagged records."""

DECIDABLE_SEVERITIES = ("low", "medium", "high")


def triage(records):
    """Split record ids into the ones that could be decided and the ones that could not."""
    processed = []
    untestable = []
    for record in records:
        if record["severity"] in DECIDABLE_SEVERITIES:
            processed.append(record["id"])
        else:
            untestable.append(record["id"])
    return {"processed": processed, "untestable": untestable}
