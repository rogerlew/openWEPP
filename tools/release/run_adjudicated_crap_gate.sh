#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_adjudicated_crap_gate.sh \
    [--base-ref <git-ref>] \
    [--head-ref <git-ref>] \
    [--scope <global|affected>] \
    [--package <cargo-package>]... \
    [--nextest-profile <profile>] \
    [--output-dir <path>] \
    [--crap-json <existing-report.json>] \
    [--retained-provenance <repository-evidence.md>] \
    [--adjudications <registry.json>]

Without --crap-json, the gate collects fresh workspace LCOV and cargo-crap JSON.
Supplying --crap-json skips measurement and performs a retained, assessment-only
evaluation; --retained-provenance is then required. Alternate adjudication
registries are allowed only for retained assessment and can never close current
source. Fresh closure always uses the canonical registry.
--base-ref enables touched-production-file reporting; the workspace actionable
set is always enforced whether or not a base ref is supplied.
Affected mode is fresh-only, requires one or more exact packages and a base
ref, and enforces the actionable set for those packages. Repeating `--package`
adds the terminal plan's complete affected/reverse-dependent package closure.
Global mode remains the default
and is unchanged for critical, campaign, and release closure.
USAGE
}

require_value() {
  local value="${2:-}"
  if [[ -z "${value}" || "${value}" == --* ]]; then
    return 1
  fi
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PYTHON_BIN="${ROOT_DIR}/.venv/bin/python"
CHECKER="${ROOT_DIR}/tools/release/check_adjudicated_crap.py"
ADJUDICATIONS="${ROOT_DIR}/tools/release/adjudicated_crap_exceptions.json"
OUTPUT_DIR="${ROOT_DIR}/target/adjudicated-crap"
CRAP_JSON=""
RETAINED_PROVENANCE=""
BASE_REF=""
HEAD_REF=""
SCOPE="global"
PACKAGES=()
NEXTEST_PROFILE=""
ADJUDICATIONS_OVERRIDDEN=0
HELP_REQUESTED=0
PARSE_ERRORS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-ref)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      BASE_REF="${2:-}"
      shift 2
      ;;
    --head-ref)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      HEAD_REF="${2:-}"
      shift 2
      ;;
    --scope)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      SCOPE="${2:-}"
      shift 2
      ;;
    --package)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      PACKAGES+=("${2:-}")
      shift 2
      ;;
    --nextest-profile)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      NEXTEST_PROFILE="${2:-}"
      shift 2
      ;;
    --output-dir)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      OUTPUT_DIR="${2:-}"
      shift 2
      ;;
    --crap-json)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      CRAP_JSON="${2:-}"
      shift 2
      ;;
    --retained-provenance)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      RETAINED_PROVENANCE="${2:-}"
      shift 2
      ;;
    --adjudications)
      if ! require_value "$1" "${2:-}"; then
        PARSE_ERRORS+=("$1 requires a non-empty value")
        shift
        continue
      fi
      ADJUDICATIONS="${2:-}"
      ADJUDICATIONS_OVERRIDDEN=1
      shift 2
      ;;
    -h|--help)
      HELP_REQUESTED=1
      shift
      ;;
    *)
      PARSE_ERRORS+=("unknown argument: $1")
      shift
      ;;
  esac
done

if [[ -n "${CRAP_JSON}" ]]; then
  ACQUISITION_MODE="retained"
else
  ACQUISITION_MODE="fresh"
fi

if [[ "${HELP_REQUESTED}" -eq 1 && "${#PARSE_ERRORS[@]}" -eq 0 ]]; then
  usage
  exit 0
fi

if [[ -n "${OPENWEPP_GATE_ARTIFACT_ROOT:-}" ]]; then
  if [[ "${OPENWEPP_GATE_ARTIFACT_ROOT}" != /* || "${OUTPUT_DIR}" == /* || "${OUTPUT_DIR}" == *".."* ]]; then
    echo "ERROR: executor artifact relocation requires an absolute root and safe relative output path" >&2
    exit 2
  fi
  OUTPUT_DIR="${OPENWEPP_GATE_ARTIFACT_ROOT}/${OUTPUT_DIR}"
fi
mkdir -p -- "${OUTPUT_DIR}"

GENERATED_FILES=(
  adjudication-registry.json
  adjudicated-crap-report.json
  adjudicated-crap-report.md
  cargo-version.txt
  cargo-crap-version.txt
  cargo-crap.log
  cargo-llvm-cov-version.txt
  llvm-cov-clean.log
  llvm-cov.log
  nextest.toml
  run-status.json
  rustc-version.txt
  sha256sums.txt
  source-manifest-after.json
  source-manifest-before.json
  source-manifest-final.json
  workspace-metadata.json
  workspace-crap.json
  workspace.lcov
)
RUN_STARTED_UTC="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
finalize() {
  local exit_status="$?"
  trap - EXIT
  local result="PASS"
  if [[ "${exit_status}" -ne 0 ]]; then
    result="FAIL"
  fi
  {
    printf '{\n'
    printf '  "acquisition_mode": "%s",\n' "${ACQUISITION_MODE}"
    printf '  "exit_status": %s,\n' "${exit_status}"
    printf '  "finished_utc": "%s",\n' "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    printf '  "result": "%s",\n' "${result}"
    printf '  "started_utc": "%s"\n' "${RUN_STARTED_UTC}"
    printf '}\n'
  } > "${OUTPUT_DIR}/run-status.json"
  : > "${OUTPUT_DIR}/sha256sums.txt"
  for generated_file in "${GENERATED_FILES[@]}"; do
    if [[ "${generated_file}" != "sha256sums.txt" && -f "${OUTPUT_DIR}/${generated_file}" ]]; then
      sha256sum "${OUTPUT_DIR}/${generated_file}" >> "${OUTPUT_DIR}/sha256sums.txt"
    fi
  done
  exit "${exit_status}"
}
trap finalize EXIT

for generated_file in "${GENERATED_FILES[@]}"; do
  rm -f -- "${OUTPUT_DIR}/${generated_file}"
done

# Establish a clean failure envelope before validating options, prerequisites,
# or mode combinations. Reusing an output directory can therefore never leave
# an earlier PASS report visible after a failed invocation.
if [[ "${#PARSE_ERRORS[@]}" -ne 0 ]]; then
  for parse_error in "${PARSE_ERRORS[@]}"; do
    echo "ERROR: ${parse_error}" >&2
  done
  usage >&2
  exit 2
fi
if [[ ! -x "${PYTHON_BIN}" ]]; then
  echo "ERROR: repository Python is unavailable: ${PYTHON_BIN}" >&2
  exit 2
fi
if [[ ! -f "${ADJUDICATIONS}" ]]; then
  echo "ERROR: adjudication registry is unavailable: ${ADJUDICATIONS}" >&2
  exit 2
fi
if [[ -n "${HEAD_REF}" && -z "${BASE_REF}" ]]; then
  echo "ERROR: --head-ref requires --base-ref" >&2
  exit 2
fi
if [[ "${SCOPE}" != "global" && "${SCOPE}" != "affected" ]]; then
  echo "ERROR: --scope must be exactly global or affected" >&2
  exit 2
fi
if [[ "${SCOPE}" == "affected" ]]; then
  if [[ "${ACQUISITION_MODE}" != "fresh" || -z "${BASE_REF}" || "${#PACKAGES[@]}" -eq 0 ]]; then
    echo "ERROR: affected scope requires fresh acquisition, --base-ref, and at least one --package" >&2
    exit 2
  fi
  for package in "${PACKAGES[@]}"; do
    if [[ ! "${package}" =~ ^[A-Za-z0-9][A-Za-z0-9_-]*$ ]]; then
      echo "ERROR: --package is not a safe Cargo package name" >&2
      exit 2
    fi
  done
  if [[ ! "${NEXTEST_PROFILE}" =~ ^[A-Za-z0-9][A-Za-z0-9_-]*$ ]]; then
    echo "ERROR: affected scope requires a safe --nextest-profile" >&2
    exit 2
  fi
elif [[ "${#PACKAGES[@]}" -ne 0 || -n "${NEXTEST_PROFILE}" ]]; then
  echo "ERROR: --package and --nextest-profile require --scope affected" >&2
  exit 2
fi
if [[ -z "${CARGO_TARGET_DIR:-}" ]]; then
  CARGO_TARGET_DIR="${OUTPUT_DIR}/cargo-target"
  export CARGO_TARGET_DIR
fi
if [[ "${ACQUISITION_MODE}" == "fresh" ]]; then
  CARGO_BUILD_JOBS=4
  CARGO_PROFILE_TEST_DEBUG=0
  export CARGO_BUILD_JOBS CARGO_PROFILE_TEST_DEBUG
  NEXTEST_CONFIG="${OUTPUT_DIR}/nextest.toml"
  NEXTEST_STORE="${OUTPUT_DIR}/nextest"
  COVERAGE_TMP="${OUTPUT_DIR}/tmp"
  mkdir -p "${COVERAGE_TMP}"
  TMPDIR="${COVERAGE_TMP}"
  export TMPDIR
  "${PYTHON_BIN}" - "${ROOT_DIR}/.config/nextest.toml" "${NEXTEST_CONFIG}" "${NEXTEST_STORE}" <<'PY'
import json
import pathlib
import sys

source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
expected = 'dir = "target/nextest"'
if source.count(expected) != 1:
    raise SystemExit("canonical nextest store declaration is missing or ambiguous")
replacement = f"dir = {json.dumps(sys.argv[3])}"
pathlib.Path(sys.argv[2]).write_text(source.replace(expected, replacement), encoding="utf-8")
PY
fi
if [[ "${ACQUISITION_MODE}" == "retained" ]]; then
  if [[ -z "${RETAINED_PROVENANCE}" ]]; then
    echo "ERROR: --crap-json requires --retained-provenance" >&2
    exit 2
  fi
  if [[ -n "${BASE_REF}" || -n "${HEAD_REF}" ]]; then
    echo "ERROR: retained assessment cannot use --base-ref or --head-ref" >&2
    exit 2
  fi
else
  if [[ -n "${RETAINED_PROVENANCE}" ]]; then
    echo "ERROR: --retained-provenance requires --crap-json" >&2
    exit 2
  fi
  if [[ "${ADJUDICATIONS_OVERRIDDEN}" -eq 1 ]]; then
    echo "ERROR: fresh closure cannot override the canonical adjudication registry" >&2
    exit 2
  fi
fi

cp -- "${ADJUDICATIONS}" "${OUTPUT_DIR}/adjudication-registry.json"

if [[ "${ACQUISITION_MODE}" == "fresh" ]]; then
  command -v cargo >/dev/null
  command -v rustc >/dev/null
  cargo --version --verbose > "${OUTPUT_DIR}/cargo-version.txt"
  rustc --version --verbose > "${OUTPUT_DIR}/rustc-version.txt"
  LLVM_COV_VERSION="$(cargo llvm-cov --version)"
  CRAP_VERSION="$(cargo crap --version)"
  printf '%s\n' "${LLVM_COV_VERSION}" > "${OUTPUT_DIR}/cargo-llvm-cov-version.txt"
  printf '%s\n' "${CRAP_VERSION}" > "${OUTPUT_DIR}/cargo-crap-version.txt"
  if [[ "${LLVM_COV_VERSION}" != "cargo-llvm-cov 0.8.7" ]]; then
    echo "ERROR: cargo-llvm-cov 0.8.7 is required; observed ${LLVM_COV_VERSION}" >&2
    exit 2
  fi
  if [[ "${CRAP_VERSION}" != "cargo-crap 0.2.2" ]]; then
    echo "ERROR: cargo-crap 0.2.2 is required; observed ${CRAP_VERSION}" >&2
    exit 2
  fi

  echo "INFO: collecting fresh workspace coverage for adjudicated CRAP"
  "${PYTHON_BIN}" "${CHECKER}" \
    --repo-root "${ROOT_DIR}" \
    --snapshot-production-sources "${OUTPUT_DIR}/source-manifest-before.json"
  cargo llvm-cov clean --workspace > "${OUTPUT_DIR}/llvm-cov-clean.log" 2>&1
  COVERAGE_SCOPE_ARGS=(--workspace)
  REPORT_SCOPE_ARGS=()
  CRAP_SCOPE_ARGS=(--workspace)
  if [[ "${SCOPE}" == "affected" ]]; then
    COVERAGE_SCOPE_ARGS=()
    REPORT_SCOPE_ARGS=()
    for package in "${PACKAGES[@]}"; do
      COVERAGE_SCOPE_ARGS+=(--package "${package}")
      REPORT_SCOPE_ARGS+=(--package "${package}")
    done
  else
    WORKSPACE_METADATA="${OUTPUT_DIR}/workspace-metadata.json"
    cargo metadata --locked --offline --no-deps --format-version 1 \
      > "${WORKSPACE_METADATA}"
    mapfile -t WORKSPACE_PACKAGES < <(
      "${PYTHON_BIN}" - "${WORKSPACE_METADATA}" <<'PY'
import json
import pathlib
import sys

payload = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
members = set(payload["workspace_members"])
names = sorted(package["name"] for package in payload["packages"] if package["id"] in members)
if not names:
    raise SystemExit("locked Cargo metadata contains no workspace packages")
print(*names, sep="\n")
PY
    )
    if [[ "${#WORKSPACE_PACKAGES[@]}" -eq 0 ]]; then
      echo "ERROR: global LCOV report requires explicit workspace packages" >&2
      exit 2
    fi
    for package in "${WORKSPACE_PACKAGES[@]}"; do
      REPORT_SCOPE_ARGS+=(--package "${package}")
    done
  fi
  COVERAGE_PROFILE="${NEXTEST_PROFILE:-full}"
  source <(cargo llvm-cov show-env --sh)
  cargo nextest run "${COVERAGE_SCOPE_ARGS[@]}" \
    --profile "${COVERAGE_PROFILE}" \
    --target-dir "${CARGO_LLVM_COV_TARGET_DIR}" \
    --config-file "${NEXTEST_CONFIG}" \
    > "${OUTPUT_DIR}/llvm-cov.log" 2>&1
  cargo llvm-cov report "${REPORT_SCOPE_ARGS[@]}" \
    --lcov \
    --output-path "${OUTPUT_DIR}/workspace.lcov" \
    >> "${OUTPUT_DIR}/llvm-cov.log" 2>&1
  CRAP_JSON="${OUTPUT_DIR}/workspace-crap.json"
  cargo crap "${CRAP_SCOPE_ARGS[@]}" \
    --lcov "${OUTPUT_DIR}/workspace.lcov" \
    --min 0 \
    --format json \
    --output "${CRAP_JSON}" \
    > "${OUTPUT_DIR}/cargo-crap.log" 2>&1
  "${PYTHON_BIN}" "${CHECKER}" \
    --repo-root "${ROOT_DIR}" \
    --snapshot-production-sources "${OUTPUT_DIR}/source-manifest-after.json"
  if ! cmp -s \
    "${OUTPUT_DIR}/source-manifest-before.json" \
    "${OUTPUT_DIR}/source-manifest-after.json"; then
    echo "ERROR: production source or Git index changed during CRAP measurement" >&2
    exit 2
  fi
fi

if [[ ! -f "${CRAP_JSON}" ]]; then
  echo "ERROR: CRAP JSON is unavailable: ${CRAP_JSON}" >&2
  exit 2
fi
if ! cmp -s "${ADJUDICATIONS}" "${OUTPUT_DIR}/adjudication-registry.json"; then
  echo "ERROR: adjudication registry changed during CRAP measurement" >&2
  exit 2
fi

CHECK_ARGS=(
  --repo-root "${ROOT_DIR}"
  --crap-json "${CRAP_JSON}"
  --adjudications "${ADJUDICATIONS}"
  --acquisition-mode "${ACQUISITION_MODE}"
  --report-json "${OUTPUT_DIR}/adjudicated-crap-report.json"
  --report-markdown "${OUTPUT_DIR}/adjudicated-crap-report.md"
)
if [[ "${ACQUISITION_MODE}" == "fresh" ]]; then
  CHECK_ARGS+=(
    --source-manifest "${OUTPUT_DIR}/source-manifest-before.json"
    --lcov "${OUTPUT_DIR}/workspace.lcov"
    --cargo-version-file "${OUTPUT_DIR}/cargo-version.txt"
    --rustc-version-file "${OUTPUT_DIR}/rustc-version.txt"
    --llvm-cov-version-file "${OUTPUT_DIR}/cargo-llvm-cov-version.txt"
    --cargo-crap-version-file "${OUTPUT_DIR}/cargo-crap-version.txt"
  )
else
  CHECK_ARGS+=(--retained-provenance "${RETAINED_PROVENANCE}")
fi
if [[ -n "${BASE_REF}" ]]; then
  CHECK_ARGS+=(--base-ref "${BASE_REF}")
fi
if [[ -n "${HEAD_REF}" ]]; then
  CHECK_ARGS+=(--head-ref "${HEAD_REF}")
fi
if [[ "${SCOPE}" == "affected" ]]; then
  for package in "${PACKAGES[@]}"; do
    CHECK_ARGS+=(--expected-package "${package}")
  done
fi

set +e
"${PYTHON_BIN}" "${CHECKER}" "${CHECK_ARGS[@]}"
CHECK_STATUS=$?
set -e

if [[ "${ACQUISITION_MODE}" == "fresh" ]]; then
  "${PYTHON_BIN}" "${CHECKER}" \
    --repo-root "${ROOT_DIR}" \
    --snapshot-production-sources "${OUTPUT_DIR}/source-manifest-final.json"
  if ! cmp -s \
    "${OUTPUT_DIR}/source-manifest-before.json" \
    "${OUTPUT_DIR}/source-manifest-final.json"; then
    echo "ERROR: production source or Git index changed before report publication" >&2
    exit 2
  fi
fi
if ! cmp -s "${ADJUDICATIONS}" "${OUTPUT_DIR}/adjudication-registry.json"; then
  echo "ERROR: adjudication registry changed before report publication" >&2
  exit 2
fi
if [[ "${CHECK_STATUS}" -ne 0 ]]; then
  exit "${CHECK_STATUS}"
fi

echo "INFO: adjudicated CRAP artifacts: ${OUTPUT_DIR}"
