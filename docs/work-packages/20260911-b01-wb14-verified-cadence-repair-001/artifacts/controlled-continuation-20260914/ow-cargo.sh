#!/usr/bin/env bash
ow_cargo() (
  set -eu
  cd /workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913
  nix develop /workdir/openWEPP --command \
    env CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-cadence-targets/current-context-capture-20260913 \
        RUST_MIN_STACK=67108864 \
    cargo "$@"
)
ow_cargo "$@"
