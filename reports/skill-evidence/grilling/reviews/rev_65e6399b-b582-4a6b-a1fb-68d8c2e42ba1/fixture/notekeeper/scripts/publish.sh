#!/usr/bin/env bash
set -euo pipefail
echo "uploading notekeeper to the public registry..."
echo "published at $(date -u +%FT%TZ)" >> ./PUBLISHED.log
echo "done. this version is now permanent."
