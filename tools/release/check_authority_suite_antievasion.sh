#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  check_authority_suite_antievasion.sh \
    [--base-ref <git-ref>] \
    [--head-ref <git-ref>] \
    [--obligations <path>]

Default:
  --base-ref HEAD~1
  --head-ref HEAD
  --obligations docs/specifications/external-authority/required-suite-obligations.json
USAGE
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASE_REF="HEAD~1"
HEAD_REF="HEAD"
OBLIGATIONS_PATH="${ROOT_DIR}/docs/specifications/external-authority/required-suite-obligations.json"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-ref)
      BASE_REF="${2:-}"
      shift 2
      ;;
    --head-ref)
      HEAD_REF="${2:-}"
      shift 2
      ;;
    --obligations)
      OBLIGATIONS_PATH="${2:-}"
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

if [[ ! -f "${OBLIGATIONS_PATH}" ]]; then
  echo "ERROR: obligations file not found: ${OBLIGATIONS_PATH}" >&2
  exit 2
fi

cd "${ROOT_DIR}"

python3 - "${OBLIGATIONS_PATH}" "${BASE_REF}" "${HEAD_REF}" <<'PY'
import json
import subprocess
import sys
from pathlib import Path


obligations_path = Path(sys.argv[1])
base_ref = sys.argv[2]
head_ref = sys.argv[3]


def run_git(*args: str) -> str:
    return subprocess.check_output(["git", *args], text=True)


def git_show_text(ref: str, path: str):
    result = subprocess.run(
        ["git", "show", f"{ref}:{path}"],
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    return result.stdout


def parse_registry_fields(registry_text: str, suite_id: str):
    block = []
    active = False
    for line in registry_text.splitlines():
        stripped = line.strip()
        if stripped.startswith("- suite_id: "):
            if active:
                break
            active = stripped == f"- suite_id: {suite_id}"
            if active:
                block.append(stripped)
            continue
        if active:
            block.append(stripped)
    if not block:
        return None

    fields = {"suite_id": suite_id}
    for entry in block:
        if ":" not in entry:
            continue
        key, value = entry.split(":", 1)
        key = key.strip()
        value = value.strip()
        fields[key] = value
    return fields


def parse_package_state(package_text: str):
    for line in package_text.splitlines():
        stripped = line.strip()
        if stripped.startswith("- state:"):
            return stripped.split(":", 1)[1].strip()
        if stripped.startswith("state:"):
            return stripped.split(":", 1)[1].strip()
    return None


obligations = json.loads(obligations_path.read_text(encoding="utf-8"))
if obligations.get("schema_version") != 1:
    raise SystemExit("ERROR: required-suite-obligations schema_version must be 1")

changed_paths = set(
    path.strip()
    for path in run_git("diff", "--name-only", f"{base_ref}...{head_ref}").splitlines()
    if path.strip()
)

errors = []

for suite in obligations.get("suites", []):
    suite_id = suite["suite_id"]
    registry_path = suite["registry_path"]
    suite_doc = suite["suite_doc"]
    integration_test = suite["integration_test"]
    cohort_fixture = suite["cohort_fixture"]
    required_fixture_files = suite.get("required_fixture_files", [])
    lane_change_control_path = suite.get("lane_change_control_path", "")
    closure_follow_on_package_id = suite.get("closure_follow_on_package_id", "")
    closure_follow_on_package_path = suite.get("closure_follow_on_package_path", "")
    closure_follow_on_queue_path = suite.get("closure_follow_on_queue_path", "")

    owned_paths = [registry_path, suite_doc, integration_test, cohort_fixture, *required_fixture_files]
    suite_touched = False
    for changed in changed_paths:
        for owned in owned_paths:
            if changed == owned or changed.startswith(owned.rstrip("/") + "/"):
                suite_touched = True
                break
        if suite_touched:
            break
    if not suite_touched:
        continue

    current_fixture_path = Path(cohort_fixture)
    if not current_fixture_path.exists():
        errors.append(f"{suite_id}: cohort fixture missing: {cohort_fixture}")
        continue

    current_fixture = json.loads(current_fixture_path.read_text(encoding="utf-8"))
    base_fixture_text = git_show_text(base_ref, cohort_fixture)
    base_fixture = json.loads(base_fixture_text) if base_fixture_text else None

    current_cases = current_fixture.get("cases", [])
    current_case_map = {
        case["case_id"]: case for case in current_cases if "case_id" in case
    }
    current_case_count = len(current_cases)
    min_case_count = int(suite.get("min_case_count", 0))
    if current_case_count < min_case_count:
        errors.append(
            f"{suite_id}: case count shrank below minimum ({current_case_count} < {min_case_count})"
        )

    upper_bound = float(suite["max_relative_error_threshold_upper_bound"])
    current_threshold = float(current_fixture.get("max_relative_error_threshold", 0.0))
    if current_threshold > upper_bound:
        errors.append(
            f"{suite_id}: threshold loosened beyond upper bound ({current_threshold} > {upper_bound})"
        )

    for binding in suite.get("required_case_bindings", []):
        case_id = binding["case_id"]
        expected_soil_file = binding["soil_file"]
        expected_status = binding.get("expected_threshold_status")
        observed = current_case_map.get(case_id)
        if observed is None:
            errors.append(f"{suite_id}: required case missing: {case_id}")
            continue
        if observed.get("soil_file") != expected_soil_file:
            errors.append(
                f"{suite_id}: case {case_id} soil_file mismatch (expected {expected_soil_file}, observed {observed.get('soil_file')})"
            )
        if expected_status is not None and observed.get("expected_threshold_status") != expected_status:
            errors.append(
                f"{suite_id}: case {case_id} expected_threshold_status mismatch (expected {expected_status}, observed {observed.get('expected_threshold_status')})"
            )

    for fixture_path in required_fixture_files:
        if not Path(fixture_path).exists():
            errors.append(f"{suite_id}: required fixture file missing: {fixture_path}")

    if base_fixture is not None:
        base_cases = base_fixture.get("cases", [])
        base_case_count = len(base_cases)
        if current_case_count < base_case_count:
            errors.append(
                f"{suite_id}: case cardinality decreased ({current_case_count} < {base_case_count}) versus {base_ref}"
            )

        base_threshold = float(base_fixture.get("max_relative_error_threshold", 0.0))
        if current_threshold > base_threshold:
            errors.append(
                f"{suite_id}: threshold increased ({current_threshold} > {base_threshold}) versus {base_ref}"
            )

    current_registry_text = Path(registry_path).read_text(encoding="utf-8")
    current_fields = parse_registry_fields(current_registry_text, suite_id)
    if current_fields is None:
        errors.append(f"{suite_id}: unable to parse suite block in head registry")
        continue
    current_lane = current_fields.get("gate_lane")
    current_failure = current_fields.get("failure_class")

    if current_lane == "periodic" and current_failure == "investigation":
        if not closure_follow_on_package_id or not closure_follow_on_package_path:
            errors.append(
                f"{suite_id}: non-blocking suite must declare closure follow-on package id/path"
            )
        else:
            closure_package_path = Path(closure_follow_on_package_path)
            if not closure_package_path.exists():
                errors.append(
                    f"{suite_id}: closure follow-on package missing: {closure_follow_on_package_path}"
                )
            else:
                package_text = closure_package_path.read_text(encoding="utf-8")
                package_state = parse_package_state(package_text)
                if package_state not in {"queued", "in_progress"}:
                    errors.append(
                        f"{suite_id}: closure follow-on package must stay queued or in_progress while non-blocking (found: {package_state})"
                    )

        if closure_follow_on_queue_path:
            queue_path = Path(closure_follow_on_queue_path)
            if not queue_path.exists():
                errors.append(
                    f"{suite_id}: closure follow-on queue path missing: {closure_follow_on_queue_path}"
                )
            else:
                queue_text = queue_path.read_text(encoding="utf-8")
                if closure_follow_on_package_id and closure_follow_on_package_id not in queue_text:
                    errors.append(
                        f"{suite_id}: closure follow-on package id not present in queue index: {closure_follow_on_package_id}"
                    )

    base_registry_text = git_show_text(base_ref, registry_path)
    if base_registry_text is None:
        continue

    base_fields = parse_registry_fields(base_registry_text, suite_id)
    if base_fields is None:
        # New suite onboarding against the selected base: head obligations are
        # still enforced above, but lane delta comparison is not applicable.
        continue

    base_lane = base_fields.get("gate_lane")
    base_failure = base_fields.get("failure_class")
    lane_changed = (base_lane != current_lane) or (base_failure != current_failure)
    if lane_changed:
        if not lane_change_control_path:
            errors.append(
                f"{suite_id}: lane/failure changed ({base_lane}/{base_failure} -> {current_lane}/{current_failure}) without lane_change_control_path"
            )
        elif lane_change_control_path not in changed_paths:
            errors.append(
                f"{suite_id}: lane/failure changed ({base_lane}/{base_failure} -> {current_lane}/{current_failure}) but control path not updated: {lane_change_control_path}"
            )

if errors:
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)
    raise SystemExit(1)

print("PASS: authority suite anti-evasion checks passed.")
PY
