"""Triage of severity-tagged records into decided and undecidable ids."""


DECIDED_SEVERITIES = ("low", "medium", "high")


def triage(records):
    """Split ``records`` into processed and untestable ids."""
    processed = []
    untestable = []
    for record in records:
        if record["severity"] in DECIDED_SEVERITIES:
            processed.append(record["id"])
        else:
            untestable.append(record["id"])
    return {"processed": processed, "untestable": untestable}
