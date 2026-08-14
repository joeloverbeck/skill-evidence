"""Destination normalization."""


def normalize_destination(city: str, country: str) -> tuple[str, str]:
    """Return a normalized destination."""

    normalized_city = " ".join(city.split())
    if not normalized_city:
        raise ValueError("city must not be blank")

    normalized_country = country.upper() if len(country) == 2 else country
    return normalized_city, normalized_country
