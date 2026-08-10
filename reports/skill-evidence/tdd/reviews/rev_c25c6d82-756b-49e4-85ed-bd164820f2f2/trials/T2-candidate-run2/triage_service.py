"""Triage of severity-tagged records into decided and undecidable outcomes."""

DECIDABLE_SEVERITIES = ("low", "medium", "high")


def triage(records):
    """Split ``records`` into the ids that could be decided and those that could not."""
    processed = []
    untestable = []
    for record in records:
        if record["severity"] in DECIDABLE_SEVERITIES:
            processed.append(record["id"])
        else:
            untestable.append(record["id"])
    return {"processed": processed, "untestable": untestable}
