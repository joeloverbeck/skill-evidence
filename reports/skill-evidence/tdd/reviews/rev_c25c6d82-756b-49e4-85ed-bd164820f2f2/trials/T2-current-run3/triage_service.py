DECIDABLE_SEVERITIES = ("low", "medium", "high")


def triage(records):
    processed = []
    untestable = []
    for record in records:
        if record["severity"] in DECIDABLE_SEVERITIES:
            processed.append(record["id"])
        else:
            untestable.append(record["id"])
    return {"processed": processed, "untestable": untestable}
