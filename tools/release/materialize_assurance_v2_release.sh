#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 6 ]]; then
  echo "Usage: materialize_assurance_v2_release.sh ROOT RELEASE_DIR SNAPSHOT RECEIPT COMMIT CONFIGURATION" >&2
  exit 2
fi

ROOT_DIR="$1"
RELEASE_DIR="$2"
V2_ASSURANCE_SNAPSHOT="$3"
V2_ASSURANCE_RECEIPT="$4"
V2_ASSURANCE_RELEASE_COMMIT="$5"
V2_ASSURANCE_RELEASE_CONFIGURATION="$6"

V2_SNAPSHOT_ID="$(basename "${V2_ASSURANCE_SNAPSHOT}")"
V2_RECEIPT_FILE="$(basename "${V2_ASSURANCE_RECEIPT}")"
V2_RECEIPT_ID="${V2_RECEIPT_FILE%.json}"
if [[ ! "${V2_SNAPSHOT_ID}" =~ ^[0-9a-f]{64}$ || ! "${V2_RECEIPT_FILE}" =~ ^[0-9a-f]{64}\.json$ ]]; then
  echo "ERROR: verified v2 snapshot and receipt must use content-addressed names." >&2
  exit 1
fi

(
  cd "${ROOT_DIR}"
  cargo run --quiet -p openwepp-assurance -- \
    verify-release \
    --all \
    --snapshot-dir "${V2_ASSURANCE_SNAPSHOT}" \
    --receipt "${V2_ASSURANCE_RECEIPT}" \
    --release-commit "${V2_ASSURANCE_RELEASE_COMMIT}" \
    --release-configuration "${V2_ASSURANCE_RELEASE_CONFIGURATION}"
)

V2_RELEASE_ROOT="${RELEASE_DIR}/assurance-v2"
V2_RELEASE_SNAPSHOT="${V2_RELEASE_ROOT}/snapshots/${V2_SNAPSHOT_ID}"
V2_RELEASE_RECEIPT="${V2_RELEASE_ROOT}/receipts/${V2_RECEIPT_FILE}"
if [[ -e "${V2_RELEASE_SNAPSHOT}" || -L "${V2_RELEASE_SNAPSHOT}" || -e "${V2_RELEASE_RECEIPT}" || -L "${V2_RELEASE_RECEIPT}" ]]; then
  echo "ERROR: v2 release materialization destination already exists." >&2
  exit 1
fi
mkdir -p "${V2_RELEASE_ROOT}/snapshots" "${V2_RELEASE_ROOT}/receipts"
cp -a -- "${V2_ASSURANCE_SNAPSHOT}" "${V2_RELEASE_SNAPSHOT}"
cp -- "${V2_ASSURANCE_RECEIPT}" "${V2_RELEASE_RECEIPT}"

(
  cd "${ROOT_DIR}"
  cargo run --quiet -p openwepp-assurance -- \
    verify-release \
    --all \
    --snapshot-dir "${V2_RELEASE_SNAPSHOT}" \
    --receipt "${V2_RELEASE_RECEIPT}" \
    --release-commit "${V2_ASSURANCE_RELEASE_COMMIT}" \
    --release-configuration "${V2_ASSURANCE_RELEASE_CONFIGURATION}"
) > "${V2_RELEASE_ROOT}/verification.txt"

sidecar="${RELEASE_DIR}/assurance-v2-publication.json"
sidecar_prepare="${sidecar}.prepare"
printf '%s\n' "{\"format\":\"openwepp-assurance-release-reference:1\",\"release_commit\":\"${V2_ASSURANCE_RELEASE_COMMIT}\",\"release_configuration\":\"${V2_ASSURANCE_RELEASE_CONFIGURATION}\",\"snapshot_id\":\"${V2_SNAPSHOT_ID}\",\"receipt_id\":\"${V2_RECEIPT_ID}\"}" > "${sidecar_prepare}"
mv -- "${sidecar_prepare}" "${sidecar}"
(
  cd "${V2_RELEASE_ROOT}"
  sha256sum \
    "snapshots/${V2_SNAPSHOT_ID}/manifest.json" \
    "receipts/${V2_RECEIPT_FILE}" \
    > SHA256SUMS
  sha256sum --check SHA256SUMS
)
