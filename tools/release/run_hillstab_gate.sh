#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_hillstab_gate.sh \
    --openwepp-binary <path> \
    --cohort-seeds-csv <path> \
    --watchlist-csv <path> \
    [--wepp-forest-root <path>] \
    [--wc1-root <path>] \
    [--scratch-root <path>] \
    [--output-json <path>] \
    [--jobs <n>] \
    [--timeout-seconds <seconds>] \
    [--limit-1166 <n>] \
    [--limit-watchlist <n>] \
    [--keep-passing-workdirs] \
    [--expect-suite <suite=count> ...]

This script executes the HILLSTAB01 cohort harness and enforces pass/fail
assertions from the emitted JSON payload.
USAGE
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HARNESS="${ROOT_DIR}/docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py"
ASSERT="${ROOT_DIR}/tools/release/assert_hillstab_success.py"

OPENWEPP_BINARY=""
COHORT_SEEDS_CSV=""
WATCHLIST_CSV=""
WEPP_FOREST_ROOT="/workdir/wepp-forest"
WC1_ROOT="/wc1/runs"
SCRATCH_ROOT="/tmp/openwepp_release_gate_hillstab"
OUTPUT_JSON="/tmp/openwepp_release_gate_hillstab_results.json"
JOBS=4
TIMEOUT_SECONDS=180
LIMIT_1166=""
LIMIT_WATCHLIST=""
KEEP_PASSING_WORKDIRS=0
EXPECT_SUITE_ARGS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --openwepp-binary)
      OPENWEPP_BINARY="${2:-}"
      shift 2
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
    --scratch-root)
      SCRATCH_ROOT="${2:-}"
      shift 2
      ;;
    --output-json)
      OUTPUT_JSON="${2:-}"
      shift 2
      ;;
    --jobs)
      JOBS="${2:-}"
      shift 2
      ;;
    --timeout-seconds)
      TIMEOUT_SECONDS="${2:-}"
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

if [[ -z "${OPENWEPP_BINARY}" || -z "${COHORT_SEEDS_CSV}" || -z "${WATCHLIST_CSV}" ]]; then
  echo "ERROR: --openwepp-binary, --cohort-seeds-csv, and --watchlist-csv are required." >&2
  usage >&2
  exit 2
fi

HARNESS_CMD=(
  python3
  "${HARNESS}"
  --openwepp-binary "${OPENWEPP_BINARY}"
  --cohort-seeds-csv "${COHORT_SEEDS_CSV}"
  --watchlist-csv "${WATCHLIST_CSV}"
  --wepp-forest-root "${WEPP_FOREST_ROOT}"
  --wc1-root "${WC1_ROOT}"
  --scratch-root "${SCRATCH_ROOT}"
  --output-json "${OUTPUT_JSON}"
  --jobs "${JOBS}"
  --timeout-seconds "${TIMEOUT_SECONDS}"
)

if [[ -n "${LIMIT_1166}" ]]; then
  HARNESS_CMD+=(--limit-1166 "${LIMIT_1166}")
fi
if [[ -n "${LIMIT_WATCHLIST}" ]]; then
  HARNESS_CMD+=(--limit-watchlist "${LIMIT_WATCHLIST}")
fi
if [[ "${KEEP_PASSING_WORKDIRS}" -eq 1 ]]; then
  HARNESS_CMD+=(--keep-passing-workdirs)
fi

echo "INFO: running hillstab harness"
"${HARNESS_CMD[@]}"

ASSERT_CMD=(
  python3
  "${ASSERT}"
  --results-json "${OUTPUT_JSON}"
  --require-suite wb05b_1166
  --require-suite release_gate_watchlist
)
ASSERT_CMD+=("${EXPECT_SUITE_ARGS[@]}")

echo "INFO: asserting hillstab harness results"
"${ASSERT_CMD[@]}"

echo "INFO: hillstab gate passed (${OUTPUT_JSON})"
