"""Triage of severity-tagged records into decided and undecidable ids."""

DECIDED_SEVERITIES = ("low", "medium", "high")


def triage(records):
    """Split record ids by whether their severity can be decided.

    Returns a dict with exactly two keys, ``processed`` and ``untestable``,
    each a list of record ids in the order the records were given.
    """
    processed = []
    untestable = []
    for record in records:
        bucket = processed if record["severity"] in DECIDED_SEVERITIES else untestable
        bucket.append(record["id"])
    return {"processed": processed, "untestable": untestable}
