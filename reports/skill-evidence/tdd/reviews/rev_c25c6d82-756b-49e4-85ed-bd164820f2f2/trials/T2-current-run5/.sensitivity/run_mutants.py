"""Sensitivity check: each rejected implementation must be caught by the suite.

Runs the real test file against deliberately wrong implementations of triage(),
in a throwaway directory. Nothing here is a deliverable.
"""

import pathlib
import shutil
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent
SANDBOX = HERE / "sandbox"

MUTANTS = {
    "severity-sorted output": '''
RANK = {"low": 0, "medium": 1, "high": 2}
def triage(records):
    decided = [r for r in records if r["severity"] in RANK]
    decided.sort(key=lambda r: RANK[r["severity"]])
    return {
        "processed": [r["id"] for r in decided],
        "untestable": [r["id"] for r in records if r["severity"] not in RANK],
    }
''',
    "stops at first undecidable": '''
DECIDABLE = {"low", "medium", "high"}
def triage(records):
    processed, untestable = [], []
    for r in records:
        if r["severity"] not in DECIDABLE:
            untestable.append(r["id"])
            break
        processed.append(r["id"])
    return {"processed": processed, "untestable": untestable}
''',
    "drops undecidable silently": '''
DECIDABLE = {"low", "medium", "high"}
def triage(records):
    return {
        "processed": [r["id"] for r in records if r["severity"] in DECIDABLE],
        "untestable": [],
    }
''',
    "case-folds severity": '''
DECIDABLE = {"low", "medium", "high"}
def triage(records):
    processed, untestable = [], []
    for r in records:
        bucket = processed if r["severity"].lower() in DECIDABLE else untestable
        bucket.append(r["id"])
    return {"processed": processed, "untestable": untestable}
''',
    "adds a third key": '''
DECIDABLE = {"low", "medium", "high"}
def triage(records):
    processed, untestable = [], []
    for r in records:
        bucket = processed if r["severity"] in DECIDABLE else untestable
        bucket.append(r["id"])
    return {"processed": processed, "untestable": untestable, "count": len(records)}
''',
    "returns whole records, not ids": '''
DECIDABLE = {"low", "medium", "high"}
def triage(records):
    processed, untestable = [], []
    for r in records:
        bucket = processed if r["severity"] in DECIDABLE else untestable
        bucket.append(r)
    return {"processed": processed, "untestable": untestable}
''',
}

failures = []
for name, source in MUTANTS.items():
    if SANDBOX.exists():
        shutil.rmtree(SANDBOX)
    SANDBOX.mkdir(parents=True)
    shutil.copy(ROOT / "test_triage_service.py", SANDBOX / "test_triage_service.py")
    (SANDBOX / "triage_service.py").write_text(source)
    result = subprocess.run(
        [sys.executable, "-m", "pytest", "test_triage_service.py", "-q"],
        cwd=SANDBOX,
        capture_output=True,
        text=True,
    )
    caught = result.returncode != 0
    print(f"{'CAUGHT  ' if caught else 'ESCAPED '} {name}")
    if not caught:
        failures.append(name)

if SANDBOX.exists():
    shutil.rmtree(SANDBOX)

print()
print("all mutants caught" if not failures else f"NOT DISCRIMINATING: {failures}")
sys.exit(1 if failures else 0)
