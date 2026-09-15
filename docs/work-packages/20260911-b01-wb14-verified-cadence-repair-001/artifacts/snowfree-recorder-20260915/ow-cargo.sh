#!/usr/bin/env bash
set -eu
source_root=$1
build_target=$2
shift 2
cd "$source_root"
nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR="$build_target" RUST_MIN_STACK=67108864 cargo "$@"
