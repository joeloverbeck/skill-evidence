"""Put the `textkit` package on sys.path for the suite.

There is no installed distribution or packaging config in this workspace, so the
suite resolves the package from the source tree.
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
