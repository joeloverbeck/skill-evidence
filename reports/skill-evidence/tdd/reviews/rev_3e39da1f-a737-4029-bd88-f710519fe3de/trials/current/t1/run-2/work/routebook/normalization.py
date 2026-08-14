"""Destination normalization."""


def normalize_destination(city: str, country: str) -> tuple[str, str]:
    """Return a normalized destination."""

    normalized_city = " ".join(city.split())
    if not normalized_city:
        raise ValueError("city must not be blank")
    return normalized_city, country.strip().upper()
