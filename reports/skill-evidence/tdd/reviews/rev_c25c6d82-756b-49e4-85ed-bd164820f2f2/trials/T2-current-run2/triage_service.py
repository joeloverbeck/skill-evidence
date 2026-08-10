"""Triage of severity-tagged records into decided and undecidable ids."""

DECIDABLE_SEVERITIES = ("low", "medium", "high")


def triage(records):
    """Split record ids into those that could be decided and those that could not.

    Returns a dict with exactly two keys, ``processed`` and ``untestable``, each
    holding record ids in the order the records were given. A record whose
    severity is not one of ``DECIDABLE_SEVERITIES`` cannot be decided: its id is
    recorded as untestable and the remaining records are processed as normal.
    """
    processed = []
    untestable = []
    for record in records:
        if record["severity"] in DECIDABLE_SEVERITIES:
            processed.append(record["id"])
        else:
            untestable.append(record["id"])
    return {"processed": processed, "untestable": untestable}
