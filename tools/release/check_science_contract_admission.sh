#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASE_REF=""
HEAD_REF="HEAD"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-ref) BASE_REF="${2:-}"; shift 2 ;;
    --head-ref) HEAD_REF="${2:-}"; shift 2 ;;
    *) echo "ERROR: unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [[ -z "${BASE_REF}" ]]; then
  echo "ERROR: --base-ref is required" >&2
  exit 2
fi

cd "${ROOT_DIR}"
git rev-parse --verify "${BASE_REF}^{commit}" >/dev/null
git rev-parse --verify "${HEAD_REF}^{commit}" >/dev/null
git merge-base --is-ancestor "${BASE_REF}" "${HEAD_REF}"

python3 - "${BASE_REF}" "${HEAD_REF}" <<'PY'
import json
import re
import subprocess
import sys
import tomllib
from pathlib import Path

base_ref, head_ref = sys.argv[1:]
index_path = Path("docs/specifications/science-contracts/index.md")
if not index_path.is_file():
    raise SystemExit("ERROR: science-contract registry is missing")

row_re = re.compile(r"^\| `(?P<id>SC-[A-Z0-9]+-\d{3})` \|")
field_re = re.compile(r"^(contract_id|status|maturity):\s*(.+?)\s*$")
rows = []
in_registry = False
for line in index_path.read_text(encoding="utf-8").splitlines():
    if line == "## Current Registry":
        in_registry = True
        continue
    if in_registry and line.startswith("## "):
        in_registry = False
    if not in_registry:
        continue
    match = row_re.match(line)
    if not match:
        if line.startswith("|") and not (
            line.startswith("| contract_id") or line.startswith("|---")
        ):
            raise SystemExit(f"ERROR: malformed registry data row: {line}")
        continue
    cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
    if len(cells) != 10:
        raise SystemExit(f"ERROR: malformed registry row: {line}")
    rows.append((match.group("id"), cells[2], cells[3], cells[5]))

if not rows:
    raise SystemExit("ERROR: science-contract registry has no contract rows")
ids = [row[0] for row in rows]
paths = [row[3] for row in rows]
if len(ids) != len(set(ids)) or len(paths) != len(set(paths)):
    raise SystemExit("ERROR: science-contract registry contains duplicate IDs or paths")
if ids != sorted(ids):
    raise SystemExit("ERROR: science-contract registry is not sorted by contract_id")

checked = 0
active_paths = set()
active_ids = set()
active_metadata = {}
for contract_id, status, maturity, path_text in rows:
    if status == "withdrawn":
        continue
    path = Path(path_text)
    if not path.is_file():
        raise SystemExit(f"ERROR: registered contract is missing: {path}")
    fields = {}
    for line in path.read_text(encoding="utf-8").splitlines()[1:80]:
        if line == "---":
            break
        match = field_re.match(line)
        if match:
            fields[match.group(1)] = match.group(2)
    expected = {"contract_id": contract_id, "status": status, "maturity": maturity}
    if fields != expected:
        raise SystemExit(
            f"ERROR: registry/front-matter mismatch for {contract_id}: "
            f"expected={expected} observed={fields}"
        )
    checked += 1
    active_paths.add(path_text)
    active_ids.add(contract_id)
    active_metadata[contract_id] = {"status": status, "maturity": maturity}

if checked == 0:
    raise SystemExit("ERROR: no non-withdrawn science contracts were admitted")

contract_files = {
    str(path)
    for path in Path("docs/specifications/science-contracts/contracts").glob("SC-*.md")
    if not path.name.startswith("SC-INFILE-")
}
if contract_files != active_paths:
    missing = sorted(active_paths - contract_files)
    unregistered = sorted(contract_files - active_paths)
    raise SystemExit(
        f"ERROR: contract registry/file set differs: missing={missing} unregistered={unregistered}"
    )

input_registry = Path("docs/specifications/wepp-input-files/input-surface-registry.md")
if not input_registry.is_file():
    raise SystemExit("ERROR: parser input surface registry is missing")
input_ids = []
for line in input_registry.read_text(encoding="utf-8").splitlines():
    if not line.startswith("| `infile-"):
        continue
    cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
    if len(cells) != 5:
        raise SystemExit(f"ERROR: malformed parser input registry row: {line}")
    if cells[3] == "active":
        if not re.fullmatch(r"SC-INFILE-[A-Z0-9-]+-001", cells[4]):
            raise SystemExit(f"ERROR: active parser surface has invalid contract: {line}")
        input_ids.append(cells[4])
if len(input_ids) != len(set(input_ids)) or not input_ids:
    raise SystemExit("ERROR: active parser contract registry is empty or duplicated")
input_contract_files = {
    path.stem: path
    for path in Path("docs/specifications/science-contracts/contracts").glob("SC-INFILE-*.md")
}
if set(input_ids) != set(input_contract_files):
    raise SystemExit(
        "ERROR: parser registry/file set differs: "
        f"missing={sorted(set(input_ids) - set(input_contract_files))} "
        f"unregistered={sorted(set(input_contract_files) - set(input_ids))}"
    )
for contract_id in input_ids:
    fields = {}
    for line in input_contract_files[contract_id].read_text(encoding="utf-8").splitlines()[1:80]:
        if line == "---":
            break
        match = field_re.match(line)
        if match:
            fields[match.group(1)] = match.group(2)
    if fields.get("contract_id") != contract_id or not fields.get("status") or not fields.get("maturity"):
        raise SystemExit(f"ERROR: parser contract front matter is incomplete: {contract_id}")
    active_ids.add(contract_id)
    active_metadata[contract_id] = {
        "status": fields["status"],
        "maturity": fields["maturity"],
    }
    checked += 1

science_tokens = (
    "climate", "hillslope", "input-contract", "kernel", "landuse", "legacy",
    "management", "meteorology", "phenology", "runner", "sim-contract",
    "summary", "topology", "unit-boundary", "watershed",
)
impact = json.loads(Path("tools/release/authority-policy/impact-map.json").read_text())
entries = impact["entries"]
gate_definitions = {
    definition["gate_definition_id"]: definition
    for definition in json.loads(
        Path("tools/release/authority-policy/gate-definitions.json").read_text()
    )["definitions"]
}

suite_fields = {}
suite_refs = {}
current = None
for line in Path("docs/specifications/external-authority/registry.yaml").read_text().splitlines():
    stripped = line.strip()
    if stripped.startswith("- suite_id: "):
        current = stripped.split(":", 1)[1].strip()
        suite_fields[current] = {}
        suite_refs[current] = set()
    elif current and stripped.startswith("- SC-"):
        suite_refs[current].add(stripped.removeprefix("- "))
    elif current and ":" in stripped and not stripped.startswith("-"):
        key, value = stripped.split(":", 1)
        if key in {"status", "authority_level", "gate_lane", "failure_class"}:
            suite_fields[current][key] = value.strip()

changed = subprocess.check_output(
    ["git", "diff", "--name-only", "--diff-filter=ACDMRT", base_ref, head_ref],
    text=True,
).splitlines()
for changed_path in changed:
    if changed_path.startswith("docs/specifications/science-contracts/contracts/SC-"):
        contract_id = Path(changed_path).stem
        metadata = active_metadata.get(contract_id)
        if metadata != {"status": "approved", "maturity": "active"}:
            raise SystemExit(
                f"ERROR: changed science contract is not approved/active: "
                f"{contract_id} metadata={metadata}"
            )

science_paths = []
for changed_path in changed:
    parts = Path(changed_path).parts
    if len(parts) < 3 or parts[0] != "crates":
        continue
    manifest = Path(parts[0], parts[1], "Cargo.toml")
    if manifest.is_file():
        manifest_text = manifest.read_text(encoding="utf-8")
    else:
        manifest_text = None
        for ref in (head_ref, base_ref):
            observed = subprocess.run(
                ["git", "show", f"{ref}:{manifest}"],
                text=True,
                capture_output=True,
                check=False,
            )
            if observed.returncode == 0:
                manifest_text = observed.stdout
                break
        if manifest_text is None:
            raise SystemExit(f"ERROR: changed crate manifest cannot be resolved: {manifest}")
    package = tomllib.loads(manifest_text)["package"]["name"]
    if any(token in package for token in science_tokens):
        science_paths.append(changed_path)

for changed_path in science_paths:
    matches = []
    for entry in entries:
        matcher = entry["matcher"]
        if matcher["kind"] == "exact_path" and changed_path == matcher["value"]:
            matches.append(entry)
        elif matcher["kind"] == "path_prefix" and (
            changed_path == matcher["value"].rstrip("/")
            or changed_path.startswith(matcher["value"].rstrip("/") + "/")
        ):
            matches.append(entry)
    admitted = [entry for entry in matches if len(entry.get("contracts", [])) == 1]
    if len(admitted) != 1:
        raise SystemExit(
            f"ERROR: {changed_path} requires one current SC contract binding; observed={len(admitted)}"
        )
    entry = admitted[0]
    contract_id = entry["contracts"][0]
    if contract_id not in active_ids:
        raise SystemExit(f"ERROR: {changed_path} references unknown contract {contract_id}")
    if active_metadata[contract_id] != {"status": "approved", "maturity": "active"}:
        raise SystemExit(
            f"ERROR: {changed_path} references provisional contract {contract_id}: "
            f"{active_metadata[contract_id]}"
        )
    covering_targets = entry.get("covering_test_targets", [])
    if not covering_targets:
        raise SystemExit(f"ERROR: {changed_path} has no explicit A1 hard-invariant binding")
    a1_definitions = [
        gate_definitions.get(definition_id)
        for definition_id in entry.get("gate_definition_ids", [])
    ]
    for target in covering_targets:
        valid_a1 = any(
            definition
            and definition["authority_class"] == "A1"
            and definition["executor"]["kind"] == "NEXTEST_V1"
            and definition["outcome_policy"] == "BLOCKING"
            and definition["failure_classification"] == "HARD_FAIL"
            and definition["inventory_mode"] == "EXACT"
            and definition["inventory_source"].startswith("NEXTEST_")
            and target in definition["arguments_template"]
            for definition in a1_definitions
        )
        if not valid_a1:
            raise SystemExit(
                f"ERROR: {changed_path} target {target} is not bound to an executable A1 gate"
            )
    suites = entry.get("authority_suites", [])
    applicable_a3 = sorted(
        suite for suite, fields in suite_fields.items()
        if fields == {
            "status": "active",
            "authority_level": "4",
            "gate_lane": "required",
            "failure_class": "hard-fail",
        }
        and any(ref.startswith(f"{contract_id}#") for ref in suite_refs.get(suite, set()))
    )
    valid_a3 = [
        suite for suite in suites
        if suite_fields.get(suite) == {
            "status": "active",
            "authority_level": "4",
            "gate_lane": "required",
            "failure_class": "hard-fail",
        }
        and any(ref.startswith(f"{contract_id}#") for ref in suite_refs.get(suite, set()))
    ]
    if sorted(suites) != applicable_a3 or len(valid_a3) != len(suites):
        raise SystemExit(
            f"ERROR: {changed_path} A3 binding differs from applicable registry suites: "
            f"declared={sorted(suites)} applicable={applicable_a3}"
        )

base = subprocess.check_output(["git", "rev-parse", f"{base_ref}^{{commit}}"], text=True).strip()
head = subprocess.check_output(["git", "rev-parse", f"{head_ref}^{{commit}}"], text=True).strip()
print(f"A0_ADMITTED contracts={checked} science_surfaces={len(science_paths)} base={base} head={head}")
PY
