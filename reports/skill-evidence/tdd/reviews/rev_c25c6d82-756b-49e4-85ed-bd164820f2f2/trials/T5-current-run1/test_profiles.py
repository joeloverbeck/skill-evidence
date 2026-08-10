"""Behaviour tests for the public profile-loading interface.

Seam under test: ``profiles.load_profile`` as the public entry point, with
``profiles.ProfileError`` as the public error type. Both names, and the
``load_profile(payload: dict) -> dict`` signature, are fixed by TASK.md.
Nothing here reaches inside the module.
"""

import pytest

from profiles import ProfileError, load_profile


def test_payload_without_email_is_rejected_naming_the_missing_field():
    # The payload carries a field, just not the required one, so a message
    # that merely echoed the payload could not name "email" by accident.
    payload = {"name": "Ada Lovelace"}

    with pytest.raises(ProfileError) as rejection:
        load_profile(payload)

    # "email" is the field name fixed by the requirement, not a value read
    # back out of the payload under test.
    assert "email" in str(rejection.value)


def test_payload_carrying_an_email_is_not_rejected():
    # Without this case the refusal above is also satisfied by a
    # load_profile that rejects every payload, which would name the field
    # while rejecting payloads the requirement does not reject.
    result = load_profile({"name": "Ada Lovelace", "email": "ada@example.com"})

    # The declared return type is all TASK.md fixes for the accepted path,
    # so that is all this asserts.
    assert isinstance(result, dict)
