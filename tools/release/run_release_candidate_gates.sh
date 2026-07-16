#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_release_candidate_gates.sh \
    --mode <validate|release> \
    [--release-tag <yymmddsuffix>] \
    [--release-dir <path>] \
    [--v2-assurance-snapshot <path>] \
    [--v2-assurance-receipt <path>] \
    [--v2-assurance-release-commit <sha>] \
    [--v2-assurance-release-configuration <id>] \
    [--skip-stability] \
    [--skip-authority-required] \
    [--run-authority-periodic] \
    [--run-authority-manual] \
    [--authority-registry <path>] \
    [--authority-report <path>] \
    [--crap-base-ref <git-ref>] \
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

Mode is required. Validation mode executes non-assembly workspace gates and
cannot create assurance snapshots, release binaries, sidecars, or release-
candidate artifacts. Release mode explicitly enables assembly after assurance
transition preflight. Use --skip-stability to omit the release stability gate.
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
MODE=""
SKIP_STABILITY=0
SKIP_AUTHORITY_REQUIRED=0
RUN_AUTHORITY_PERIODIC=0
RUN_AUTHORITY_MANUAL=0
V2_ASSURANCE_SNAPSHOT=""
V2_ASSURANCE_RECEIPT=""
V2_ASSURANCE_RELEASE_COMMIT=""
V2_ASSURANCE_RELEASE_CONFIGURATION=""

AUTHORITY_REGISTRY="${ROOT_DIR}/docs/specifications/external-authority/registry.yaml"
AUTHORITY_REPORT=""
AUTHORITY_INVESTIGATION_FAILURES=0
CRAP_BASE_REF=""

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
    --mode)
      MODE="${2:-}"
      shift 2
      ;;
    --release-tag)
      RELEASE_TAG="${2:-}"
      shift 2
      ;;
    --release-dir)
      RELEASE_DIR="${2:-}"
      shift 2
      ;;
    --v2-assurance-snapshot)
      V2_ASSURANCE_SNAPSHOT="${2:-}"
      shift 2
      ;;
    --v2-assurance-receipt)
      V2_ASSURANCE_RECEIPT="${2:-}"
      shift 2
      ;;
    --v2-assurance-release-commit)
      V2_ASSURANCE_RELEASE_COMMIT="${2:-}"
      shift 2
      ;;
    --v2-assurance-release-configuration)
      V2_ASSURANCE_RELEASE_CONFIGURATION="${2:-}"
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
    --crap-base-ref)
      CRAP_BASE_REF="${2:-}"
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

if [[ "${MODE}" != "validate" && "${MODE}" != "release" ]]; then
  echo "ERROR: --mode must be exactly 'validate' or 'release'." >&2
  exit 2
fi

# This preflight runs before creating an evidence/release directory. In release
# mode it is the fail-closed ASSURE03-REL-001/ASSURE-04D assembly boundary.
assurance_preflight_args=(
  --mode "${MODE}"
  --root "${ROOT_DIR}"
)
if [[ -n "${V2_ASSURANCE_SNAPSHOT}" || -n "${V2_ASSURANCE_RECEIPT}" || -n "${V2_ASSURANCE_RELEASE_COMMIT}" || -n "${V2_ASSURANCE_RELEASE_CONFIGURATION}" ]]; then
  assurance_preflight_args+=(
    --v2-snapshot "${V2_ASSURANCE_SNAPSHOT}"
    --v2-receipt "${V2_ASSURANCE_RECEIPT}"
    --release-commit "${V2_ASSURANCE_RELEASE_COMMIT}"
    --release-configuration "${V2_ASSURANCE_RELEASE_CONFIGURATION}"
  )
fi
bash "${ROOT_DIR}/tools/release/check_assurance_release_transition.sh" \
  "${assurance_preflight_args[@]}"

if [[ ! -f "${AUTHORITY_REGISTRY}" ]]; then
  echo "ERROR: authority registry not found: ${AUTHORITY_REGISTRY}" >&2
  exit 2
fi

if [[ "${MODE}" == "release" && ! "${RELEASE_TAG}" =~ ^[0-9]{6}[a-z0-9_-]*$ ]]; then
  echo "ERROR: release tag must match ^[0-9]{6}[a-z0-9_-]*$ (observed '${RELEASE_TAG}')" >&2
  exit 2
fi

if [[ -z "${RELEASE_DIR}" ]]; then
  if [[ "${MODE}" == "release" ]]; then
    RELEASE_DIR="$(mktemp -d "/tmp/openwepp_release_${RELEASE_TAG}_XXXXXX")"
  else
    RELEASE_DIR="$(mktemp -d "/tmp/openwepp_validation_XXXXXX")"
  fi
fi
mkdir -p "${RELEASE_DIR}"

if [[ "${MODE}" == "release" && -n "${V2_ASSURANCE_SNAPSHOT}" ]]; then
  bash "${ROOT_DIR}/tools/release/materialize_assurance_v2_release.sh" \
    "${ROOT_DIR}" \
    "${RELEASE_DIR}" \
    "${V2_ASSURANCE_SNAPSHOT}" \
    "${V2_ASSURANCE_RECEIPT}" \
    "${V2_ASSURANCE_RELEASE_COMMIT}" \
    "${V2_ASSURANCE_RELEASE_CONFIGURATION}"
fi

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

active_suite_fixture_roots() {
  awk '
    function flush_suite() {
      if (suite_id != "" && suite_status == "active" && fixture_root != "") {
        print suite_id "|" fixture_root
      }
      suite_id = ""
      suite_status = ""
      fixture_root = ""
    }
    $1 == "-" && $2 == "suite_id:" {
      flush_suite()
      suite_id = $3
      next
    }
    $1 == "status:" {
      suite_status = $2
      next
    }
    $1 == "fixture_root:" {
      fixture_root = $2
      next
    }
    END {
      flush_suite()
    }
  ' "${AUTHORITY_REGISTRY}"
}

verify_fixture_provenance_entry() {
  local provenance_file="$1"
  local suite_id="$2"
  local fixture_path="$3"
  local expected_sha="$4"

  awk \
    -v provenance_file_path="${provenance_file}" \
    -v expected_suite_id="${suite_id}" \
    -v target_path="${fixture_path}" \
    -v expected_sha="${expected_sha}" \
    '
      function trim(value) {
        sub(/^[[:space:]]+/, "", value)
        sub(/[[:space:]]+$/, "", value)
        return value
      }
      function parse_value(line) {
        return trim(substr(line, index(line, ":") + 1))
      }
      function flush_item() {
        if (!in_item) {
          return
        }
        if (path == target_path) {
          found = 1
          if (sha256 != expected_sha) {
            printf("ERROR: suite %s fixture %s sha mismatch in %s (expected %s observed %s)\n", expected_suite_id, target_path, provenance_file_path, expected_sha, sha256) > "/dev/stderr"
            bad = 1
          }
          if (source_repo == "" || source_commit == "" || source_path == "" || source_sha256 == "" || transform_note == "") {
            printf("ERROR: suite %s fixture %s missing required provenance keys in %s\n", expected_suite_id, target_path, provenance_file_path) > "/dev/stderr"
            bad = 1
          }
          if (source_sha256 !~ /^[0-9a-f]{64}$/) {
            printf("ERROR: suite %s fixture %s source_sha256 is not 64-char lowercase hex in %s\n", expected_suite_id, target_path, provenance_file_path) > "/dev/stderr"
            bad = 1
          }
        }
        in_item = 0
        path = ""
        sha256 = ""
        source_repo = ""
        source_commit = ""
        source_path = ""
        source_sha256 = ""
        transform_note = ""
      }
      BEGIN {
        in_item = 0
        found = 0
        bad = 0
        schema_seen = 0
        suite_seen = 0
        listed_suite = ""
      }
      /^schema_version:[[:space:]]*/ {
        schema_seen = 1
        next
      }
      /^suite_id:[[:space:]]*/ {
        suite_seen = 1
        listed_suite = $2
        next
      }
      /^[[:space:]]*-[[:space:]]path:[[:space:]]*/ {
        flush_item()
        in_item = 1
        path = parse_value($0)
        next
      }
      in_item && /^[[:space:]]*sha256:[[:space:]]*/ {
        sha256 = parse_value($0)
        next
      }
      in_item && /^[[:space:]]*source_repo:[[:space:]]*/ {
        source_repo = parse_value($0)
        next
      }
      in_item && /^[[:space:]]*source_commit:[[:space:]]*/ {
        source_commit = parse_value($0)
        next
      }
      in_item && /^[[:space:]]*source_path:[[:space:]]*/ {
        source_path = parse_value($0)
        next
      }
      in_item && /^[[:space:]]*source_sha256:[[:space:]]*/ {
        source_sha256 = parse_value($0)
        next
      }
      in_item && /^[[:space:]]*transform_note:[[:space:]]*/ {
        transform_note = parse_value($0)
        next
      }
      END {
        flush_item()
        if (!schema_seen) {
          printf("ERROR: fixture provenance file missing schema_version: %s\n", provenance_file_path) > "/dev/stderr"
          exit 1
        }
        if (!suite_seen || listed_suite != expected_suite_id) {
          printf("ERROR: fixture provenance suite_id mismatch in %s (expected %s observed %s)\n", provenance_file_path, expected_suite_id, listed_suite) > "/dev/stderr"
          exit 1
        }
        if (!found) {
          printf("ERROR: fixture provenance missing entry for suite %s fixture %s in %s\n", expected_suite_id, target_path, provenance_file_path) > "/dev/stderr"
          exit 1
        }
        if (bad) {
          exit 1
        }
      }
    ' "${provenance_file}"
}

run_authority_fixture_integrity_gate() {
  local rows=()
  local row suite_id fixture_root_abs fixture_root_rel lock_file provenance_file digest fixture_path

  mapfile -t rows < <(active_suite_fixture_roots)
  if [[ "${#rows[@]}" -eq 0 ]]; then
    echo "ERROR: no active suites discovered in authority registry: ${AUTHORITY_REGISTRY}" >&2
    exit 1
  fi

  for row in "${rows[@]}"; do
    suite_id="${row%%|*}"
    fixture_root_rel="${row#*|}"
    fixture_root_abs="${ROOT_DIR}/${fixture_root_rel}"
    lock_file="${fixture_root_abs}/fixtures.sha256"
    provenance_file="${fixture_root_abs}/fixtures.provenance.yaml"

    if [[ ! -d "${fixture_root_abs}" ]]; then
      echo "ERROR: fixture root for suite ${suite_id} does not exist: ${fixture_root_abs}" >&2
      exit 1
    fi
    if [[ ! -f "${lock_file}" ]]; then
      echo "ERROR: fixture hash lock missing for suite ${suite_id}: ${lock_file}" >&2
      exit 1
    fi
    if [[ ! -f "${provenance_file}" ]]; then
      echo "ERROR: fixture provenance file missing for suite ${suite_id}: ${provenance_file}" >&2
      exit 1
    fi

    (
      cd "${fixture_root_abs}"
      sha256sum --check --strict fixtures.sha256 > /dev/null
    )

    while read -r digest fixture_path; do
      [[ -z "${digest}" ]] && continue
      if [[ ! "${digest}" =~ ^[0-9a-f]{64}$ ]]; then
        echo "ERROR: malformed fixture sha256 digest in ${lock_file}: ${digest}" >&2
        exit 1
      fi
      if [[ ! -f "${fixture_root_abs}/${fixture_path}" ]]; then
        echo "ERROR: fixture listed in lock file is missing: ${fixture_root_abs}/${fixture_path}" >&2
        exit 1
      fi
      verify_fixture_provenance_entry "${provenance_file}" "${suite_id}" "${fixture_path}" "${digest}"
    done < "${lock_file}"

    {
      echo "- fixture_integrity suite=${suite_id} fixture_root=${fixture_root_rel} status=pass"
    } >> "${AUTHORITY_REPORT}"
  done
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

echo "INFO: checking scientific assurance zero-report exports"
bash tools/release/check_assurance_dossier_exports.sh

echo "INFO: running workspace release gates"
cargo nextest --version > "${RELEASE_DIR}/cargo-nextest-version.txt"
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
.venv/bin/python -m unittest -v tests.python.test_adjudicated_crap_gate

CRAP_GATE_ARGS=(--output-dir "${RELEASE_DIR}/adjudicated-crap")
if [[ -n "${CRAP_BASE_REF}" ]]; then
  CRAP_GATE_ARGS+=(--base-ref "${CRAP_BASE_REF}")
fi
bash tools/release/run_adjudicated_crap_gate.sh "${CRAP_GATE_ARGS[@]}"

ASSURANCE_SNAPSHOT_MANIFEST=""
if [[ "${MODE}" == "release" ]]; then
  ASSURANCE_SNAPSHOT_ROOT="${RELEASE_DIR}/assurance-snapshots"
  ASSURANCE_SNAPSHOT_MANIFEST="${ASSURANCE_SNAPSHOT_ROOT}/${RELEASE_TAG}/manifest.json"
  echo "INFO: creating zero-report scientific assurance snapshot '${RELEASE_TAG}'"
  cargo run --quiet -p openwepp-assurance -- build --all \
    --snapshot "${RELEASE_TAG}" \
    --snapshot-root "${ASSURANCE_SNAPSHOT_ROOT}" \
    > "${RELEASE_DIR}/assurance-snapshot-build.txt"
  sha256sum "${ASSURANCE_SNAPSHOT_MANIFEST}" \
    > "${RELEASE_DIR}/assurance-snapshot.sha256"
fi

{
  echo "# Authority Suite Gate Results"
  echo
  echo "- generated_utc: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  echo "- registry: ${AUTHORITY_REGISTRY}"
  echo "- fixture_integrity_enforced: true"
  echo "- required_lane_enabled: $((1 - SKIP_AUTHORITY_REQUIRED))"
  echo "- periodic_lane_enabled: ${RUN_AUTHORITY_PERIODIC}"
  echo "- manual_lane_enabled: ${RUN_AUTHORITY_MANUAL}"
} > "${AUTHORITY_REPORT}"

echo "INFO: evaluating authority-suite lanes"
echo "INFO: verifying authority fixture hash locks and provenance"
run_authority_fixture_integrity_gate
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

if [[ "${MODE}" == "validate" ]]; then
  echo "INFO: validation gate automation passed"
  echo "INFO: validation_evidence_dir=${RELEASE_DIR}"
  exit 0
fi

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
echo "INFO: assurance_snapshot=${ASSURANCE_SNAPSHOT_MANIFEST}"
if [[ "${SKIP_STABILITY}" -eq 0 ]]; then
  echo "INFO: stability_results_json=${HILLSTAB_OUTPUT_JSON}"
fi
