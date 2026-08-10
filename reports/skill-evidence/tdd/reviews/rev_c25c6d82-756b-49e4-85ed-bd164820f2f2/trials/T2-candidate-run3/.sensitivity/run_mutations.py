"""Sensitivity check: each mutation is a plausible rejected implementation.

For every mutation, the suite MUST fail. A mutation that still passes means the
goldens do not discriminate against that wrong behavior.
"""

import pathlib
import shutil
import subprocess
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent.parent

MUTATIONS = {
    "sorted output (ignores given order)": '''
DECIDED = ("low", "medium", "high")


def triage(records):
    processed, untestable = [], []
    for r in records:
        (processed if r["severity"] in DECIDED else untestable).append(r["id"])
    return {"processed": sorted(processed), "untestable": sorted(untestable)}
''',
    "stops at first undecidable record": '''
DECIDED = ("low", "medium", "high")


def triage(records):
    processed, untestable = [], []
    for r in records:
        if r["severity"] not in DECIDED:
            untestable.append(r["id"])
            break
        processed.append(r["id"])
    return {"processed": processed, "untestable": untestable}
''',
    "drops undecidable records entirely": '''
DECIDED = ("low", "medium", "high")


def triage(records):
    return {
        "processed": [r["id"] for r in records if r["severity"] in DECIDED],
        "untestable": [],
    }
''',
    "case-insensitive severity match": '''
DECIDED = ("low", "medium", "high")


def triage(records):
    processed, untestable = [], []
    for r in records:
        target = processed if r["severity"].lower() in DECIDED else untestable
        target.append(r["id"])
    return {"processed": processed, "untestable": untestable}
''',
    "extra third key in result": '''
DECIDED = ("low", "medium", "high")


def triage(records):
    processed, untestable = [], []
    for r in records:
        (processed if r["severity"] in DECIDED else untestable).append(r["id"])
    return {"processed": processed, "untestable": untestable, "total": len(records)}
''',
    "returns whole records instead of ids": '''
DECIDED = ("low", "medium", "high")


def triage(records):
    processed, untestable = [], []
    for r in records:
        (processed if r["severity"] in DECIDED else untestable).append(r)
    return {"processed": processed, "untestable": untestable}
''',
}


def main():
    for name, source in MUTATIONS.items():
        with tempfile.TemporaryDirectory() as tmp:
            tmp = pathlib.Path(tmp)
            shutil.copy(ROOT / "test_triage_service.py", tmp / "test_triage_service.py")
            (tmp / "triage_service.py").write_text(source)
            proc = subprocess.run(
                ["python3", "-m", "pytest", str(tmp / "test_triage_service.py"), "-q"],
                capture_output=True,
                text=True,
            )
            verdict = "CAUGHT (suite failed)" if proc.returncode != 0 else "MISSED (suite passed)"
            print(f"{verdict:<24} <- {name}")


if __name__ == "__main__":
    main()
