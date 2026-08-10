"""Make the `textkit` package importable when pytest is run from the repo root.

There is no installed distribution here, so put the package root on sys.path.
"""

import sys
from pathlib import Path

PACKAGE_ROOT = str(Path(__file__).parent)
if PACKAGE_ROOT not in sys.path:
    sys.path.insert(0, PACKAGE_ROOT)
