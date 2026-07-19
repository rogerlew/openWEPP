#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: $0 <base-commit> <head-commit>" >&2
  exit 2
fi

readonly BASE_COMMIT="$(git rev-parse --verify --end-of-options "$1^{commit}")"
readonly HEAD_COMMIT="$(git rev-parse --verify --end-of-options "$2^{commit}")"
readonly BOOTSTRAP_ROOT="$(mktemp -d "${RUNNER_TEMP:-/tmp}/openwepp-bootstrap.XXXXXX")"
trap 'rm -rf -- "${BOOTSTRAP_ROOT}"' EXIT

fetch_tree() {
  local revision="$1" destination="$2"
  mkdir -p "${destination}"
  git archive "${revision}" | tar -x -C "${destination}"
  cargo fetch --locked --manifest-path "${destination}/Cargo.toml"
}

fetch_tree "${BASE_COMMIT}" "${BOOTSTRAP_ROOT}/base"
fetch_tree "${HEAD_COMMIT}" "${BOOTSTRAP_ROOT}/head"
