#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: check_assurance_release_transition.sh --mode <validate|release> [--root <path>]

Validation mode confirms only that the caller selected the non-assembly route.
Release mode fails closed if an ASSURE-03 transition marker, any catalog other
than the exact typed zero-report transition catalog, or any active/malformed
retired v1 source/public route is present.
USAGE
}

MODE=""
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode)
      MODE="${2:-}"
      shift 2
      ;;
    --root)
      ROOT_DIR="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "ERROR: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ "${MODE}" != "validate" && "${MODE}" != "release" ]]; then
  echo "ERROR: --mode must be exactly 'validate' or 'release'." >&2
  exit 2
fi

if [[ ! -d "${ROOT_DIR}" ]]; then
  echo "ERROR: assurance preflight root is not a directory: ${ROOT_DIR}" >&2
  exit 2
fi
ROOT_DIR="$(cd "${ROOT_DIR}" && pwd -P)"

if [[ "${MODE}" == "validate" ]]; then
  echo "assurance transition preflight: PASS mode=validate assembly_authorized=false"
  exit 0
fi

CATALOG="${ROOT_DIR}/assurance/catalog.yaml"
EXPECTED_CATALOG_SHA256="cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f"
if [[ -e "${ROOT_DIR}/assurance/V1_PUBLICATION_TRANSITION" || -L "${ROOT_DIR}/assurance/V1_PUBLICATION_TRANSITION" ]]; then
  echo "ERROR: release assembly is blocked by assurance/V1_PUBLICATION_TRANSITION." >&2
  exit 1
fi
if [[ ! -f "${CATALOG}" || -L "${CATALOG}" ]]; then
  echo "ERROR: release assembly requires a regular, non-symlink assurance/catalog.yaml." >&2
  exit 1
fi
CATALOG_REAL="$(realpath -e "${CATALOG}")"
if [[ "${CATALOG_REAL}" != "${ROOT_DIR}/assurance/catalog.yaml" ]]; then
  echo "ERROR: release assembly rejects an assurance catalog outside the selected root." >&2
  exit 1
fi
CATALOG_SHA256="$(sha256sum "${CATALOG}" | awk '{print $1}')"
if [[ "${CATALOG_SHA256}" != "${EXPECTED_CATALOG_SHA256}" ]]; then
  echo "ERROR: release assembly requires the exact typed v1-retired zero-report catalog bytes." >&2
  exit 1
fi

retired_paths=(
  assurance/dossiers
  assurance/methods
  assurance/schemas
  assurance/templates/application-context-worksheet.md
  assurance/templates/dossier.md
  assurance/templates/method.md
  usersum/assurance/application-context-worksheet.md
  usersum/assurance/dossiers
  usersum/assurance/methods
)
for relative in "${retired_paths[@]}"; do
  path="${ROOT_DIR}/${relative}"
  if [[ -e "${path}" || -L "${path}" ]]; then
    if [[ -L "${path}" || ! -d "${path}" ]]; then
      echo "ERROR: release assembly rejects retired v1 route: ${relative}" >&2
      exit 1
    fi
    if find "${path}" -mindepth 1 -print -quit | grep -q .; then
      echo "ERROR: release assembly rejects nonempty retired v1 route: ${relative}" >&2
      exit 1
    fi
  fi
done

echo "assurance transition preflight: PASS mode=release publication_state=v1_retired_zero_reports reports=0"
