"""Loading of profile payloads."""


class ProfileError(Exception):
    """Raised when a profile payload cannot be loaded."""


def load_profile(payload: dict) -> dict:
    """Load a profile from ``payload``.

    Raises ``ProfileError`` naming the offending field if the payload is not
    loadable.
    """
    if "email" not in payload:
        raise ProfileError("profile payload is missing required field: email")
    return dict(payload)
