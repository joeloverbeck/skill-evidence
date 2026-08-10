import pytest

from profiles import ProfileError, load_profile


def test_payload_without_email_is_rejected_with_error_naming_the_missing_field():
    payload = {"name": "Ada Lovelace", "age": 36}

    with pytest.raises(ProfileError) as excinfo:
        load_profile(payload)

    assert "email" in str(excinfo.value)
