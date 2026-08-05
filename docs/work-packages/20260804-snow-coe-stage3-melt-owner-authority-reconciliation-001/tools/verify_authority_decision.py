#!/usr/bin/env python3
"""Verify the frozen 21N authority inputs and adjudicated contract state."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
from pathlib import Path
from typing import Any


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git_blob(repo: Path, revision: str, path: str) -> bytes:
    result = subprocess.run(
        ["git", "show", f"{revision}:{path}"],
        cwd=repo,
        check=True,
        capture_output=True,
    )
    return result.stdout


def require(condition: bool, message: str, checks: list[dict[str, Any]]) -> None:
    checks.append({"check": message, "passed": condition})
    if not condition:
        raise ValueError(message)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--freeze", type=Path, required=True)
    parser.add_argument("--decision", type=Path, required=True)
    parser.add_argument("--libsnobal-root", type=Path, default=Path("/home/workdir/pysnobal"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    root = args.root.resolve()
    freeze = json.loads(args.freeze.read_text(encoding="utf-8"))
    decision = json.loads(args.decision.read_text(encoding="utf-8"))
    checks: list[dict[str, Any]] = []

    base = freeze["openwepp_source_head"]
    for name, item in freeze["inputs"].items():
        actual = sha256(git_blob(root, base, item["path"]))
        require(actual == item["sha256"], f"frozen openWEPP input: {name}", checks)

    lib_commit = freeze["pinned_libsnobal_commit"]
    for name, item in freeze["git_blob_inputs"].items():
        actual_data = git_blob(args.libsnobal_root, lib_commit, item["path"])
        require(sha256(actual_data) == item["sha256"], f"frozen libsnobal input: {name}", checks)
        require(len(actual_data) == item["size_bytes"], f"frozen libsnobal size: {name}", checks)

    require(decision["selected_outcome"] == "STAGE3_MELT_OWNER_ADMITTED", "selected outcome", checks)
    predicates = decision["predicate_results"]
    require(not predicates["coe_specific_independent_validation"], "CoE validation predicate failed", checks)
    require(not predicates["coe_enforceable_transferability_envelope"], "CoE envelope predicate failed", checks)
    require(predicates["independent_positive_energy_to_bounded_melt_authority"], "energy-to-melt authority", checks)
    require(predicates["reproducible_stage3_solid_liquid_routing_chronology"], "Stage3 chronology", checks)
    require(not predicates["dual_owner_allowed"], "dual owner prohibited", checks)
    require(decision["runtime_disposition"] == "IMPLEMENTATION_HOLD", "runtime implementation hold", checks)
    require(not decision["current_runtime"]["runtime_changed_by_package"], "runtime unchanged declaration", checks)

    energy = (root / "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md").read_text(encoding="utf-8")
    snow = (root / "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md").read_text(encoding="utf-8")
    for token in (
        "contract_version: 7",
        "INV-SNOWENERGY-029",
        "INV-SNOWENERGY-030",
        "OBL-SNOWENERGY-C-013",
        "GAP-SNOWENERGY-011",
    ):
        require(token in energy, f"energy contract token: {token}", checks)
    for token in (
        "contract_version: 126",
        "INV-SNOWFREEZE-093",
        "OBL-SNOWFREEZE-P-066",
        "GAP-SNOWFREEZE-006",
    ):
        require(token in snow, f"snow contract token: {token}", checks)

    for name in ("stage3_source", "coe_source"):
        item = freeze["inputs"][name]
        actual = sha256((root / item["path"]).read_bytes())
        require(actual == item["sha256"], f"production source unchanged: {name}", checks)

    receipt = {
        "schema_version": 1,
        "status": "PASS",
        "evidence_class": "Ran",
        "selected_outcome": decision["selected_outcome"],
        "checks": checks,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    print(f"PASS: {len(checks)} authority checks")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
