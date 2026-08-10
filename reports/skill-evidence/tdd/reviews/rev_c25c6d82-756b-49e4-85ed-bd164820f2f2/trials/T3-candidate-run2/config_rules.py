REPLICAS_MIN = 2


def normalize(config):
    """Return the deployment config with the house rules applied.

    Returns a new dict; the argument is never mutated (ISSUE.md, Issue 41).

    R1 replicas_min      - replicas is floored at REPLICAS_MIN.
    R2 timeout_seconds_int - timeout is coerced to an int number of seconds.
    R3 region_lowercase  - region is lowercased.
    """
    result = dict(config)
    result["replicas"] = max(result["replicas"], REPLICAS_MIN)
    result["timeout"] = int(result["timeout"])
    result["region"] = result["region"].lower()
    return result
