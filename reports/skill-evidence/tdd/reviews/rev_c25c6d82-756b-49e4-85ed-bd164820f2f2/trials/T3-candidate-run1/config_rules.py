REPLICAS_MIN = 2


def normalize(config):
    """Return the deployment config with the house rules applied."""
    normalized = dict(config)
    normalized["replicas"] = max(normalized["replicas"], REPLICAS_MIN)
    normalized["timeout"] = int(normalized["timeout"])
    normalized["region"] = normalized["region"].lower()
    return normalized
