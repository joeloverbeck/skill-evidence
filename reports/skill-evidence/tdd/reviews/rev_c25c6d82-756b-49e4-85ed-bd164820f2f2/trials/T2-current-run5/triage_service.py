"""Triage of severity-tagged records."""

DECIDABLE_SEVERITIES = frozenset({"low", "medium", "high"})


def triage(records):
    """Split record ids into those that could be decided and those that could not.

    Returns a dict with exactly two keys, ``processed`` and ``untestable``, each a
    list of record ids in the order the records were given. A record whose severity
    is not decidable is set aside under ``untestable``; the remaining records are
    processed as normal.
    """
    processed = []
    untestable = []
    for record in records:
        bucket = (
            processed if record["severity"] in DECIDABLE_SEVERITIES else untestable
        )
        bucket.append(record["id"])
    return {"processed": processed, "untestable": untestable}
