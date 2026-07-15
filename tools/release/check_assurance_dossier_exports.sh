#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT_DIR}"

cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- check --all

bash tools/release/check_assurance_release_transition.sh --mode release

mapfile -t tracked_public < <(
  git ls-files 'usersum/assurance/**' |
    while IFS= read -r path; do
      [[ -f "${path}" ]] && printf '%s\n' "${path}"
    done |
    sort
)
if [[ "${#tracked_public[@]}" -ne 1 || "${tracked_public[0]}" != "usersum/assurance/README.md" ]]; then
  echo "ERROR: tracked public assurance surface must contain only usersum/assurance/README.md." >&2
  printf 'observed: %s\n' "${tracked_public[@]}" >&2
  exit 1
fi

if ! grep -Fxq 'documents: []' assurance/generated/wepppy-usersum.yaml; then
  echo "ERROR: dormant assurance export must enumerate zero documents." >&2
  exit 1
fi
if ! grep -Fxq 'vendoring_authorized: false' assurance/generated/wepppy-usersum.yaml; then
  echo "ERROR: dormant assurance export must prohibit vendoring." >&2
  exit 1
fi

echo "assurance export check: PASS reports=0 documents=0 vendoring_authorized=false"
