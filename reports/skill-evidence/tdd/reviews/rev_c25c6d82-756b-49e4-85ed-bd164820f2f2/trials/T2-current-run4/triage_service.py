DECIDABLE_SEVERITIES = ("low", "medium", "high")


def triage(records):
    return {
        "processed": [
            record["id"]
            for record in records
            if record["severity"] in DECIDABLE_SEVERITIES
        ],
        "untestable": [
            record["id"]
            for record in records
            if record["severity"] not in DECIDABLE_SEVERITIES
        ],
    }
