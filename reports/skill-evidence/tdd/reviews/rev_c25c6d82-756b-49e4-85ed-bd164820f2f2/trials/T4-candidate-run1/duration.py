def parse_duration(text: str) -> int:
    return int(text.removesuffix("s"))
