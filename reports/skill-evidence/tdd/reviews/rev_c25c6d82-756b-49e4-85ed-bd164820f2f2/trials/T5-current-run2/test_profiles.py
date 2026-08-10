import pytest

from profiles import ProfileError, load_profile


def test_payload_without_email_is_rejected_and_names_the_missing_field():
    # Discriminating input: neither the keys nor the values of this payload
    # contain the substring "email", so an error that merely echoes the payload
    # back at the caller cannot satisfy the assertion below.
    payload = {"name": "Ada Lovelace", "id": 7}

    with pytest.raises(ProfileError) as raised:
        load_profile(payload)

    assert "email" in str(raised.value)
