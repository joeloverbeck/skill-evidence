def normalize(config):
    """Return the deployment config with the house rules applied."""
    normalized = dict(config)
    normalized["replicas"] = max(config.get("replicas", 0), 2)
    normalized["timeout"] = int(config["timeout"])
    normalized["region"] = str(config["region"]).lower()
    return normalized
