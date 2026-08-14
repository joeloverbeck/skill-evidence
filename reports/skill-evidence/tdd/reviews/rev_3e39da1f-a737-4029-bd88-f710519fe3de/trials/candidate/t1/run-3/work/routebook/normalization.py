"""Destination normalization."""


def normalize_destination(city: str, country: str) -> tuple[str, str]:
    """Return a destination without surrounding whitespace."""

    normalized_city = " ".join(city.split())
    if not normalized_city:
        raise ValueError("city must not be blank")
    normalized_country = country.strip()
    if len(normalized_country) == 2:
        normalized_country = normalized_country.upper()
    return normalized_city, normalized_country
