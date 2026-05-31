#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_release_candidate_gates.sh \
    [--release-tag <yymmddsuffix>] \
    [--release-dir <path>] \
    [--skip-stability] \
    [--skip-authority-required] \
    [--run-authority-periodic] \
    [--run-authority-manual] \
    [--authority-registry <path>] \
    [--authority-report <path>] \
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
Required authority lane runs by default; periodic/manual lanes are opt-in.
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
SKIP_AUTHORITY_REQUIRED=0
RUN_AUTHORITY_PERIODIC=0
RUN_AUTHORITY_MANUAL=0

AUTHORITY_REGISTRY="${ROOT_DIR}/docs/specifications/external-authority/registry.yaml"
AUTHORITY_REPORT=""
AUTHORITY_INVESTIGATION_FAILURES=0

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
    --skip-authority-required)
      SKIP_AUTHORITY_REQUIRED=1
      shift
      ;;
    --run-authority-periodic)
      RUN_AUTHORITY_PERIODIC=1
      shift
      ;;
    --run-authority-manual)
      RUN_AUTHORITY_MANUAL=1
      shift
      ;;
    --authority-registry)
      AUTHORITY_REGISTRY="${2:-}"
      shift 2
      ;;
    --authority-report)
      AUTHORITY_REPORT="${2:-}"
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

if [[ ! -f "${AUTHORITY_REGISTRY}" ]]; then
  echo "ERROR: authority registry not found: ${AUTHORITY_REGISTRY}" >&2
  exit 2
fi

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

if [[ -z "${AUTHORITY_REPORT}" ]]; then
  AUTHORITY_REPORT="${RELEASE_DIR}/authority_suite_results.md"
fi

authority_lane_rows() {
  local lane="$1"
  local failure_class="$2"
  awk -v lane="${lane}" -v failure_class="${failure_class}" '
    function flush_suite() {
      if (suite_id != "" && gate_lane == lane && suite_failure_class == failure_class && integration_test != "") {
        print suite_id "|" integration_test
      }
      suite_id = ""
      gate_lane = ""
      suite_failure_class = ""
      integration_test = ""
    }
    $1 == "-" && $2 == "suite_id:" {
      flush_suite()
      suite_id = $3
      next
    }
    $1 == "gate_lane:" {
      gate_lane = $2
      next
    }
    $1 == "failure_class:" {
      suite_failure_class = $2
      next
    }
    $1 == "integration_test:" {
      integration_test = $2
      next
    }
    END {
      flush_suite()
    }
  ' "${AUTHORITY_REGISTRY}"
}

integration_path_to_target() {
  local integration_path="$1"
  local integration_file
  integration_file="$(basename "${integration_path}")"
  printf '%s\n' "${integration_file%.rs}"
}

run_authority_lane() {
  local lane="$1"
  local failure_class="$2"
  local blocking="false"
  if [[ "${failure_class}" == "hard-fail" ]]; then
    blocking="true"
  fi

  local rows=()
  mapfile -t rows < <(authority_lane_rows "${lane}" "${failure_class}")
  if [[ "${#rows[@]}" -eq 0 ]]; then
    {
      echo "- lane=${lane} failure_class=${failure_class} suites=0 tests=0 status=not-configured"
    } >> "${AUTHORITY_REPORT}"
    return 0
  fi

  declare -A target_to_suites=()
  local ordered_targets=()
  local row suite_id integration_path target
  for row in "${rows[@]}"; do
    suite_id="${row%%|*}"
    integration_path="${row#*|}"
    target="$(integration_path_to_target "${integration_path}")"
    if [[ -z "${target_to_suites[${target}]+x}" ]]; then
      ordered_targets+=("${target}")
      target_to_suites["${target}"]="${suite_id}"
    else
      target_to_suites["${target}"]+=",${suite_id}"
    fi
  done

  for target in "${ordered_targets[@]}"; do
    local suites_csv status
    suites_csv="${target_to_suites[${target}]}"
    echo "INFO: running authority lane '${lane}' (${failure_class}) via cargo test --test ${target}"
    if cargo test --test "${target}"; then
      status="pass"
    else
      status="fail"
    fi
    {
      echo "- lane=${lane} failure_class=${failure_class} blocking=${blocking} test=${target} suites=${suites_csv} status=${status}"
    } >> "${AUTHORITY_REPORT}"

    if [[ "${status}" == "fail" && "${failure_class}" == "hard-fail" ]]; then
      echo "ERROR: required hard-fail authority suite lane '${lane}' failed for test target ${target}" >&2
      exit 1
    fi
    if [[ "${status}" == "fail" && "${failure_class}" == "investigation" ]]; then
      AUTHORITY_INVESTIGATION_FAILURES=1
    fi
  done
}

cd "${ROOT_DIR}"

echo "INFO: running workspace release gates"
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check

{
  echo "# Authority Suite Gate Results"
  echo
  echo "- generated_utc: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  echo "- registry: ${AUTHORITY_REGISTRY}"
  echo "- required_lane_enabled: $((1 - SKIP_AUTHORITY_REQUIRED))"
  echo "- periodic_lane_enabled: ${RUN_AUTHORITY_PERIODIC}"
  echo "- manual_lane_enabled: ${RUN_AUTHORITY_MANUAL}"
} > "${AUTHORITY_REPORT}"

echo "INFO: evaluating authority-suite lanes"
if [[ "${SKIP_AUTHORITY_REQUIRED}" -eq 0 ]]; then
  run_authority_lane "required" "hard-fail"
  run_authority_lane "required" "investigation"
else
  echo "- lane=required status=skipped(reason=--skip-authority-required)" >> "${AUTHORITY_REPORT}"
fi

if [[ "${RUN_AUTHORITY_PERIODIC}" -eq 1 ]]; then
  run_authority_lane "periodic" "hard-fail"
  run_authority_lane "periodic" "investigation"
else
  echo "- lane=periodic status=skipped(reason=flag-disabled)" >> "${AUTHORITY_REPORT}"
fi

if [[ "${RUN_AUTHORITY_MANUAL}" -eq 1 ]]; then
  run_authority_lane "manual" "hard-fail"
  run_authority_lane "manual" "investigation"
else
  echo "- lane=manual status=skipped(reason=flag-disabled)" >> "${AUTHORITY_REPORT}"
fi

if [[ "${AUTHORITY_INVESTIGATION_FAILURES}" -eq 1 ]]; then
  echo "WARN: one or more authority suites with failure_class=investigation failed (non-blocking)."
fi
echo "INFO: authority_results=${AUTHORITY_REPORT}"

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
echo "INFO: authority_results=${AUTHORITY_REPORT}"
if [[ "${SKIP_STABILITY}" -eq 0 ]]; then
  echo "INFO: stability_results_json=${HILLSTAB_OUTPUT_JSON}"
fi
