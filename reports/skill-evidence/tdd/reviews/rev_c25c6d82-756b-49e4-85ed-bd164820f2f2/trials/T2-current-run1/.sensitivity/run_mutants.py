"""Sensitivity check: each rejected implementation must be killed by the suite."""

import pathlib
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
WORK = pathlib.Path(__file__).resolve().parent / "work"

SOURCE = (ROOT / "triage_service.py").read_text()

LOOP = (
    '        decided = record["severity"] in DECIDABLE_SEVERITIES\n'
    '        (processed if decided else untestable).append(record["id"])\n'
)
RETURN = '    return {"processed": processed, "untestable": untestable}\n'

MUTANTS = {
    "sorted_outputs": (
        RETURN,
        '    return {"processed": sorted(processed), "untestable": sorted(untestable)}\n',
    ),
    "drop_undecided_records": (
        LOOP,
        '        if record["severity"] in DECIDABLE_SEVERITIES:\n'
        '            processed.append(record["id"])\n',
    ),
    "stop_at_first_undecided": (
        LOOP,
        '        if record["severity"] not in DECIDABLE_SEVERITIES:\n'
        '            untestable.append(record["id"])\n'
        "            break\n"
        '        processed.append(record["id"])\n',
    ),
    "accept_every_severity": (
        '        decided = record["severity"] in DECIDABLE_SEVERITIES\n',
        "        decided = True\n",
    ),
    "extra_result_key": (
        RETURN,
        '    return {"processed": processed, "untestable": untestable, "skipped": []}\n',
    ),
    "only_high_is_decidable": (
        'DECIDABLE_SEVERITIES = frozenset({"low", "medium", "high"})\n',
        'DECIDABLE_SEVERITIES = frozenset({"high"})\n',
    ),
}


def failing_tests(report: str) -> list[str]:
    return sorted(
        line.split("::", 1)[1].split(" ")[0]
        for line in report.splitlines()
        if line.startswith("FAILED test_triage_service.py::")
    )


def main() -> int:
    shutil.rmtree(WORK, ignore_errors=True)
    WORK.mkdir(parents=True)
    shutil.copy(ROOT / "test_triage_service.py", WORK / "test_triage_service.py")

    survivors = []
    for name, (old, new) in MUTANTS.items():
        if old not in SOURCE:
            print(f"{name}: ERROR anchor not found in source")
            survivors.append(name)
            continue
        (WORK / "triage_service.py").write_text(SOURCE.replace(old, new, 1))
        result = subprocess.run(
            [sys.executable, "-m", "pytest", "test_triage_service.py", "-q",
             "-p", "no:cacheprovider"],
            cwd=WORK, capture_output=True, text=True,
        )
        killers = failing_tests(result.stdout)
        if killers:
            print(f"{name}: KILLED by {', '.join(killers)}")
        else:
            print(f"{name}: SURVIVED")
            survivors.append(name)

    shutil.rmtree(WORK, ignore_errors=True)
    print("\nsurvivors:", survivors or "none")
    return 1 if survivors else 0


if __name__ == "__main__":
    raise SystemExit(main())
