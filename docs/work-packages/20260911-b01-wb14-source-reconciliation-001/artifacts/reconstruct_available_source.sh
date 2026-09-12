#!/usr/bin/env bash
# Read immutable retained inputs and create a distinct recipe replay snapshot.
set -euo pipefail
repo=/workdir/openWEPP
out=/workdir/openwepp-experiments/b01-wb14-source-reconciliation
dst="$out/reconstructed-available145"
prior="$repo/docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts"
b01="$repo/docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts"
test ! -e "$dst"
mkdir "$dst"
sha256sum "$prior/R0-composition.patch" "$prior/R0-untracked-source.tar.gz" "$prior/R0-source.json" "$prior/PBC-final-build016.patch" "$prior/PBC-final-build016-source.json" "$b01/correction145.patch" "$b01/correction145-source.json"
git -C "$repo" archive e89befa4678eadec039b3e7f7fe0a176af8e9dc5 | tar -x -C "$dst"
patch -d "$dst" -p1 < "$prior/R0-composition.patch"
tar -xzf "$prior/R0-untracked-source.tar.gz" -C "$dst"
patch -d "$dst" -p1 < "$prior/PBC-final-build016.patch"
patch -d "$dst" -p1 < "$b01/correction145.patch"
"$repo/.venv/bin/python" "$out/compare_reconstructed_source.py" "$dst"
