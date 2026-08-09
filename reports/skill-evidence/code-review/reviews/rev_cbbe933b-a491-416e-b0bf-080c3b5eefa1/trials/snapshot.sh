#!/usr/bin/env bash
# usage: snapshot.sh <repo-dir> <label>   -> writes <repo-dir>/../<label>.snap
set -euo pipefail
repo="$1"; label="$2"
out="$repo/../$label.snap"
{
  echo "== rev-parse HEAD"; git -C "$repo" rev-parse HEAD
  echo "== status --porcelain"; git -C "$repo" status --porcelain | LC_ALL=C sort
  echo "== stash list"; git -C "$repo" stash list
  echo "== file hashes"
  ( cd "$repo" && find . -path ./.git -prune -o -type f -print | LC_ALL=C sort | xargs -r sha256sum )
} > "$out"
echo "wrote $out"
