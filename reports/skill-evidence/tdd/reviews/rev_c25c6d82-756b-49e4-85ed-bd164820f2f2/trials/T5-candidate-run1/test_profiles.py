import pytest

from profiles import ProfileError, load_profile


def test_load_profile_rejects_payload_missing_email_and_names_the_field():
    # Other fields are present, so an error message that names a field it can see
    # ("name", "id") or that stays generic ("invalid payload") cannot pass this test.
    payload = {"id": "u-17", "display_name": "Ada Lovelace"}

    with pytest.raises(ProfileError) as excinfo:
        load_profile(payload)

    assert "email" in str(excinfo.value)


def test_load_profile_accepts_payload_that_has_an_email():
    # Pins the scope of the rejection above: it is conditional on the missing field,
    # not a blanket refusal. Asserts only the `-> dict` return type that TASK.md's
    # signature fixes, since the authority does not fix the returned contents.
    payload = {"id": "u-17", "display_name": "Ada Lovelace", "email": "ada@example.com"}

    assert isinstance(load_profile(payload), dict)
