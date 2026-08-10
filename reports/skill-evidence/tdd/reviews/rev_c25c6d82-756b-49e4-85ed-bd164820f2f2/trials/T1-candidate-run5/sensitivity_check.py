"""Sensitivity check: substitute each rejected implementation, list which tests die.

Throwaway harness. Copies textkit into .mutants/<name>/, rewrites redaction.py with
one rejected behaviour, and runs the real test file against it via a pythonpath
override so the retained suite is the thing doing the judging.
"""

import pathlib
import re
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent
SOURCE = ROOT / "packages" / "textkit" / "textkit" / "redaction.py"
MUTANTS = ROOT / ".mutants"

REJECTED = {
    "case-sensitive matching": (", flags=re.IGNORECASE", ""),
    "lowercase the whole text": (
        "REDACTION_MARKER, text, flags",
        "REDACTION_MARKER, text.lower(), flags",
    ),
    "declared list order wins, not longest": (
        "key=lambda secret: (-len(secret), secret)",
        "key=lambda secret: 0",
    ),
    "no minimum-length filter": (
        "[secret for secret in secrets if len(secret) >= MINIMUM_SECRET_LENGTH]",
        "list(secrets)",
    ),
    "minimum length is > 4 rather than >= 4": (
        ">= MINIMUM_SECRET_LENGTH",
        "> MINIMUM_SECRET_LENGTH",
    ),
    "secrets compiled as patterns, not escaped": (
        "re.escape(secret) for secret in longest_first",
        "secret for secret in longest_first",
    ),
    "only the first occurrence replaced": (
        "REDACTION_MARKER, text, flags=re.IGNORECASE)",
        "REDACTION_MARKER, text, count=1, flags=re.IGNORECASE)",
    ),
    "only the first secret applied": (
        "pattern = ",
        "longest_first = longest_first[:1]\n    pattern = ",
    ),
}

original = SOURCE.read_text()
shutil.rmtree(MUTANTS, ignore_errors=True)
failures = {}

for index, (label, (old, new)) in enumerate(REJECTED.items()):
    assert original.count(old) == 1, f"anchor not unique for {label!r}"
    package = MUTANTS / f"m{index}" / "textkit"
    package.mkdir(parents=True)
    (package / "__init__.py").write_text("")
    (package / "redaction.py").write_text(original.replace(old, new))
    completed = subprocess.run(
        [
            sys.executable,
            "-m",
            "pytest",
            "packages/textkit/tests/test_redaction.py",
            "-q",
            "-p",
            "no:cacheprovider",
            "-o",
            f"pythonpath=.mutants/m{index}",
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    failures[label] = sorted(
        set(re.findall(r"^FAILED \S+::(\S+)", completed.stdout, re.MULTILINE))
    ) or (["<no test died>"] if completed.returncode else ["<no test died>"])

for label, dead in failures.items():
    print(f"{label}:")
    for name in dead:
        print(f"    {name}")

shutil.rmtree(MUTANTS, ignore_errors=True)
