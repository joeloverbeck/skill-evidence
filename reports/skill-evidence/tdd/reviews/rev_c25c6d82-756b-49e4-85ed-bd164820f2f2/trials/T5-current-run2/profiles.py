"""Loading of profile payloads."""


class ProfileError(Exception):
    """Raised when a profile payload cannot be loaded."""


def load_profile(payload: dict) -> dict:
    if "email" not in payload:
        raise ProfileError("missing required field: email")
    return payload
