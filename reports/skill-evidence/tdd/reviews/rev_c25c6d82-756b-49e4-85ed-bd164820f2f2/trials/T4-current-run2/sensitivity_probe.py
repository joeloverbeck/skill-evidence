"""Discriminating-golden check for the parse_duration goldens.

tests.md requires that a golden differ from each plausible rejected
implementation: "If a golden would also pass after the forbidden behavior is
substituted, it is not discriminating and must be replaced."

This substitutes each rejected implementation for the real one and reports
whether the goldens in test_duration.py still hold. Every rejected
implementation must be killed by at least one golden.

Run: python3 sensitivity_probe.py
"""

REJECTED = {
    "constant from the worked example": lambda text: 90,
    "first character only": lambda text: int(text[0]),
    "length of the text": lambda text: len(text),
    "treats the value as minutes": lambda text: int(text.removesuffix("s")) * 60,
    "drops every digit after the first": lambda text: int(text[0]) if text else 0,
}

# The goldens, stated independently of the implementation: (input, required).
GOLDENS = [("90s", 90), ("7s", 7)]


def killed_by(impl):
    """Return the goldens that this implementation fails."""
    failures = []
    for text, required in GOLDENS:
        try:
            actual = impl(text)
        except Exception as exc:  # a raising implementation is also a failure
            actual = f"raised {type(exc).__name__}"
        if actual != required:
            failures.append(f'parse_duration("{text}") -> {actual}, required {required}')
    return failures


def main():
    survivors = []
    for name, impl in REJECTED.items():
        failures = killed_by(impl)
        status = "KILLED" if failures else "SURVIVED"
        print(f"{status}: {name}")
        for failure in failures:
            print(f"    {failure}")
        if not failures:
            survivors.append(name)

    print()
    if survivors:
        print(f"NOT DISCRIMINATING - {len(survivors)} rejected implementation(s) survived:")
        for name in survivors:
            print(f"    {name}")
        return 1
    print("All rejected implementations are killed by at least one golden.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
