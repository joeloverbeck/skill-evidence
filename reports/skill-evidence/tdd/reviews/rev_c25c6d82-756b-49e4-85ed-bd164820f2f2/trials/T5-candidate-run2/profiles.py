"""Loading and validation of profile payloads."""


class ProfileError(Exception):
    """Raised when a profile payload is rejected."""


def load_profile(payload: dict) -> dict:
    if "email" not in payload:
        raise ProfileError("missing required field: 'email'")

    # The accepted-payload path is deliberately unimplemented: no test authorizes
    # what a loaded profile is yet (copy vs. alias, normalisation, defaults), and
    # the requirement in TASK.md covers only rejection of a payload with no
    # 'email' key. The next slice replaces this line.
    raise NotImplementedError("load_profile: accepted-payload path is not specified yet")
