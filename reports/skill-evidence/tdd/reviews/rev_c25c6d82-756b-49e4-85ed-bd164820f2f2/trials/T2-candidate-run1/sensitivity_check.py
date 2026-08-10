"""Discriminating-golden check: every golden must fail under the behavior it rejects.

Replaces `triage_service.py` with each rejected implementation in a throwaway
directory, runs the real test file against it, and asserts the named golden fails.
Run with: python3 sensitivity_check.py
"""

import pathlib
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent
TESTS = ROOT / "test_triage_service.py"
SCRATCH = ROOT / ".sensitivity_run"

MUTANTS = [
    (
        "M1 sorted by id",
        "test_decided_records_are_reported_in_the_order_given",
        """
def triage(records):
    processed = sorted(r["id"] for r in records if r["severity"] in ("low", "medium", "high"))
    untestable = [r["id"] for r in records if r["severity"] not in ("low", "medium", "high")]
    return {"processed": processed, "untestable": untestable}
""",
    ),
    (
        "M2 grouped by severity",
        "test_decided_records_are_reported_in_the_order_given",
        """
def triage(records):
    processed = [
        r["id"] for sev in ("low", "medium", "high") for r in records if r["severity"] == sev
    ]
    untestable = [r["id"] for r in records if r["severity"] not in ("low", "medium", "high")]
    return {"processed": processed, "untestable": untestable}
""",
    ),
    (
        "M3 untestable key omitted when empty",
        "test_result_has_exactly_the_processed_and_untestable_keys",
        """
def triage(records):
    processed = [r["id"] for r in records if r["severity"] in ("low", "medium", "high")]
    untestable = [r["id"] for r in records if r["severity"] not in ("low", "medium", "high")]
    result = {"processed": processed}
    if untestable:
        result["untestable"] = untestable
    return result
""",
    ),
    (
        "M4 extra diagnostic key",
        "test_result_has_exactly_the_processed_and_untestable_keys",
        """
def triage(records):
    processed = [r["id"] for r in records if r["severity"] in ("low", "medium", "high")]
    untestable = [r["id"] for r in records if r["severity"] not in ("low", "medium", "high")]
    return {"processed": processed, "untestable": untestable, "count": len(records)}
""",
    ),
    (
        "M5 stops at first undecidable record",
        "test_undecidable_severity_is_recorded_and_processing_continues",
        """
def triage(records):
    processed = []
    untestable = []
    for r in records:
        if r["severity"] not in ("low", "medium", "high"):
            untestable.append(r["id"])
            break
        processed.append(r["id"])
    return {"processed": processed, "untestable": untestable}
""",
    ),
    (
        "M6 undecidable records dropped silently",
        "test_undecidable_severity_is_recorded_and_processing_continues",
        """
def triage(records):
    processed = [r["id"] for r in records if r["severity"] in ("low", "medium", "high")]
    return {"processed": processed, "untestable": []}
""",
    ),
    (
        "M7 case-insensitive severity match",
        "test_undecidable_severity_is_recorded_and_processing_continues",
        """
def triage(records):
    processed = []
    untestable = []
    for r in records:
        if r["severity"].lower() in ("low", "medium", "high"):
            processed.append(r["id"])
        else:
            untestable.append(r["id"])
    return {"processed": processed, "untestable": untestable}
""",
    ),
]


def main():
    if SCRATCH.exists():
        shutil.rmtree(SCRATCH)
    failures = []
    try:
        for index, (name, golden, source) in enumerate(MUTANTS):
            case = SCRATCH / f"m{index}"
            case.mkdir(parents=True)
            (case / "triage_service.py").write_text(source)
            (case / "test_triage_service.py").write_text(TESTS.read_text())
            run = subprocess.run(
                [sys.executable, "-m", "pytest", f"test_triage_service.py::{golden}", "-q"],
                cwd=case,
                capture_output=True,
                text=True,
            )
            caught = run.returncode != 0
            print(f"{'CAUGHT ' if caught else 'MISSED '} {name} -> {golden}")
            if not caught:
                failures.append(name)
    finally:
        shutil.rmtree(SCRATCH, ignore_errors=True)
    if failures:
        print(f"\nNON-DISCRIMINATING GOLDENS for: {', '.join(failures)}")
        return 1
    print(f"\nAll {len(MUTANTS)} rejected implementations were caught.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
