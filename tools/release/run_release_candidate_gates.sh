#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_release_candidate_gates.sh \
    [--release-tag <yymmddsuffix>] \
    [--release-dir <path>] \
    [--skip-stability] \
    [--cohort-seeds-csv <path>] \
    [--watchlist-csv <path>] \
    [--wepp-forest-root <path>] \
    [--wc1-root <path>] \
    [--hillstab-scratch-root <path>] \
    [--hillstab-output-json <path>] \
    [--hillstab-jobs <n>] \
    [--hillstab-timeout-seconds <seconds>] \
    [--limit-1166 <n>] \
    [--limit-watchlist <n>] \
    [--keep-passing-workdirs] \
    [--expect-suite <suite=count> ...]

Default behavior executes workspace/release gates and stability gate.
Use --skip-stability to run only workspace/release gates.
USAGE
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER_BIN="${ROOT_DIR}/target/release/open_wepp_runner"
HILLSLOPE_BIN="${ROOT_DIR}/target/release/openwepp-cli-hill"
WATERSHED_BIN="${ROOT_DIR}/target/release/openwepp-cli-watershed"
STABILITY_SCRIPT="${ROOT_DIR}/tools/release/run_hillstab_gate.sh"

RELEASE_TAG="$(date -u +%y%m%d)ci"
RELEASE_DIR=""
SKIP_STABILITY=0

COHORT_SEEDS_CSV=""
WATCHLIST_CSV=""
WEPP_FOREST_ROOT="/workdir/wepp-forest"
WC1_ROOT="/wc1/runs"
HILLSTAB_SCRATCH_ROOT="/tmp/openwepp_release_gate_hillstab"
HILLSTAB_OUTPUT_JSON=""
HILLSTAB_JOBS=4
HILLSTAB_TIMEOUT_SECONDS=180
LIMIT_1166=""
LIMIT_WATCHLIST=""
KEEP_PASSING_WORKDIRS=0
EXPECT_SUITE_ARGS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release-tag)
      RELEASE_TAG="${2:-}"
      shift 2
      ;;
    --release-dir)
      RELEASE_DIR="${2:-}"
      shift 2
      ;;
    --skip-stability)
      SKIP_STABILITY=1
      shift
      ;;
    --cohort-seeds-csv)
      COHORT_SEEDS_CSV="${2:-}"
      shift 2
      ;;
    --watchlist-csv)
      WATCHLIST_CSV="${2:-}"
      shift 2
      ;;
    --wepp-forest-root)
      WEPP_FOREST_ROOT="${2:-}"
      shift 2
      ;;
    --wc1-root)
      WC1_ROOT="${2:-}"
      shift 2
      ;;
    --hillstab-scratch-root)
      HILLSTAB_SCRATCH_ROOT="${2:-}"
      shift 2
      ;;
    --hillstab-output-json)
      HILLSTAB_OUTPUT_JSON="${2:-}"
      shift 2
      ;;
    --hillstab-jobs)
      HILLSTAB_JOBS="${2:-}"
      shift 2
      ;;
    --hillstab-timeout-seconds)
      HILLSTAB_TIMEOUT_SECONDS="${2:-}"
      shift 2
      ;;
    --limit-1166)
      LIMIT_1166="${2:-}"
      shift 2
      ;;
    --limit-watchlist)
      LIMIT_WATCHLIST="${2:-}"
      shift 2
      ;;
    --keep-passing-workdirs)
      KEEP_PASSING_WORKDIRS=1
      shift
      ;;
    --expect-suite)
      EXPECT_SUITE_ARGS+=("--expect-suite" "${2:-}")
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

if [[ ! "${RELEASE_TAG}" =~ ^[0-9]{6}[a-z0-9_-]*$ ]]; then
  echo "ERROR: release tag must match ^[0-9]{6}[a-z0-9_-]*$ (observed '${RELEASE_TAG}')" >&2
  exit 2
fi

if [[ -z "${RELEASE_DIR}" ]]; then
  RELEASE_DIR="$(mktemp -d "/tmp/openwepp_release_${RELEASE_TAG}_XXXXXX")"
fi
mkdir -p "${RELEASE_DIR}"

if [[ -z "${HILLSTAB_OUTPUT_JSON}" ]]; then
  HILLSTAB_OUTPUT_JSON="${RELEASE_DIR}/hillstab_results.json"
fi

cd "${ROOT_DIR}"

echo "INFO: running workspace release gates"
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check

echo "INFO: building release binaries"
cargo build --release -p openwepp-runner --bin open_wepp_runner --bin openwepp-cli-hill --bin openwepp-cli-watershed

cp "${WATERSHED_BIN}" "${RELEASE_DIR}/openwepp_${RELEASE_TAG}"
cp "${HILLSLOPE_BIN}" "${RELEASE_DIR}/openwepp_${RELEASE_TAG}_hill"

if [[ -f "${ROOT_DIR}/target/release/openwepp-cli-replay" ]]; then
  cp "${ROOT_DIR}/target/release/openwepp-cli-replay" "${RELEASE_DIR}/openwepp_${RELEASE_TAG}_replay"
fi

echo "INFO: emitting sidecars"
"${RUNNER_BIN}" release sidecar --binary "${RELEASE_DIR}/openwepp_${RELEASE_TAG}" --role watershed
"${RUNNER_BIN}" release sidecar --binary "${RELEASE_DIR}/openwepp_${RELEASE_TAG}_hill" --role hillslope
if [[ -f "${RELEASE_DIR}/openwepp_${RELEASE_TAG}_replay" ]]; then
  "${RUNNER_BIN}" release sidecar --binary "${RELEASE_DIR}/openwepp_${RELEASE_TAG}_replay" --role replay
fi

echo "INFO: linting release directory"
"${RUNNER_BIN}" release lint --release-dir "${RELEASE_DIR}"

if [[ "${SKIP_STABILITY}" -eq 1 ]]; then
  echo "INFO: skipping stability gate (--skip-stability)"
else
  if [[ -z "${COHORT_SEEDS_CSV}" || -z "${WATCHLIST_CSV}" ]]; then
    echo "ERROR: stability gate requires --cohort-seeds-csv and --watchlist-csv." >&2
    exit 2
  fi
  STABILITY_CMD=(
    "${STABILITY_SCRIPT}"
    --openwepp-binary "${HILLSLOPE_BIN}"
    --cohort-seeds-csv "${COHORT_SEEDS_CSV}"
    --watchlist-csv "${WATCHLIST_CSV}"
    --wepp-forest-root "${WEPP_FOREST_ROOT}"
    --wc1-root "${WC1_ROOT}"
    --scratch-root "${HILLSTAB_SCRATCH_ROOT}"
    --output-json "${HILLSTAB_OUTPUT_JSON}"
    --jobs "${HILLSTAB_JOBS}"
    --timeout-seconds "${HILLSTAB_TIMEOUT_SECONDS}"
  )
  if [[ -n "${LIMIT_1166}" ]]; then
    STABILITY_CMD+=(--limit-1166 "${LIMIT_1166}")
  fi
  if [[ -n "${LIMIT_WATCHLIST}" ]]; then
    STABILITY_CMD+=(--limit-watchlist "${LIMIT_WATCHLIST}")
  fi
  if [[ "${KEEP_PASSING_WORKDIRS}" -eq 1 ]]; then
    STABILITY_CMD+=(--keep-passing-workdirs)
  fi
  STABILITY_CMD+=("${EXPECT_SUITE_ARGS[@]}")
  echo "INFO: running stability gate"
  "${STABILITY_CMD[@]}"
fi

echo "INFO: release gate automation passed"
echo "INFO: release_dir=${RELEASE_DIR}"
if [[ "${SKIP_STABILITY}" -eq 0 ]]; then
  echo "INFO: stability_results_json=${HILLSTAB_OUTPUT_JSON}"
fi
