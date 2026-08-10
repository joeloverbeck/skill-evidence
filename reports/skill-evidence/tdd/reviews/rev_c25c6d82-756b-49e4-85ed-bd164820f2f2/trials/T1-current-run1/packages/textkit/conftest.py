"""Test harness setup: make the `textkit` package importable from the tests.

There is no installed distribution and no packaging metadata in this workspace,
so the package root has to go on `sys.path` for `import textkit.redaction` to
resolve. This file carries no product behavior.
"""

import sys
from pathlib import Path

PACKAGE_ROOT = str(Path(__file__).parent)

if PACKAGE_ROOT not in sys.path:
    sys.path.insert(0, PACKAGE_ROOT)
