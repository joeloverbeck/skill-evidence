from policy import allows_batch


def main() -> None:
    failures = []
    if not allows_batch(50):
        failures.append("size 50 must be allowed")
    if allows_batch(51):
        failures.append("size 51 must be refused")
    if failures:
        raise SystemExit("; ".join(failures))
    print("policy verifier: ok")


if __name__ == "__main__":
    main()
