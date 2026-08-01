#!/usr/bin/env python3
"""Adjudicate immutable EB-04R outputs under the frozen EB-04S authority."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
CONTRACT = REPO / "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md"
AUTHORITY_FREEZE = ARTIFACTS / "authority-freeze.json"
AUTHORITY_SEAL = ARTIFACTS / "authority-seal.json"
EB04R_PACKAGE = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-"
    "execution-adjudication-001"
)
EB04R_TOOL = EB04R_PACKAGE / "tools/run_experiment.py"
ATTEMPT = EB04R_PACKAGE / "artifacts/execution-attempt.json"
RETAINED_ROOT = REPO / "target/snow_surface_eb04r_factorial"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tree_sha256(root: Path) -> str:
    digest = hashlib.sha256()
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        digest.update(path.relative_to(root).as_posix().encode())
        digest.update(b"\0")
        digest.update(bytes.fromhex(sha256(path)))
    return digest.hexdigest()


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def validate_authority() -> dict[str, Any]:
    seal = json.loads(AUTHORITY_SEAL.read_text(encoding="utf-8"))
    freeze = json.loads(AUTHORITY_FREEZE.read_text(encoding="utf-8"))
    if seal.get("status") != "FROZEN_DUAL_VERIFIED" or not seal.get("phase_b_authorized"):
        raise RuntimeError("Phase B authority is not dual-verified and authorized")
    if seal["authority_freeze_sha256"] != sha256(AUTHORITY_FREEZE):
        raise RuntimeError("authority freeze hash drift")
    if seal["canonical_contract_sha256"] != sha256(CONTRACT):
        raise RuntimeError("canonical contract hash drift")
    if seal["canonical_contract_version"] != 6 or freeze["result_bearing_inputs_read"]:
        raise RuntimeError("authority receipt is not an admissible version-6 result-blind freeze")
    predicates = seal["distinct_predicates"]
    if predicates != {
        "hourly_daily_vapor_aggregation_kg_m2": 1.0e-9,
        "represented_layer_lifecycle_kg_m2": 1.0e-9,
        "same_residual_and_vapor_to_sublimation_kg_m2": 1.0e-6,
    }:
        raise RuntimeError("authority predicates differ from the dual-reviewed seal")
    return seal


def analyze() -> dict[str, Any]:
    seal = validate_authority()
    prior_package_hash = tree_sha256(EB04R_PACKAGE)
    retained_hash = tree_sha256(RETAINED_ROOT)
    module = load_module("eb04s_readonly_eb04r", EB04R_TOOL)

    def forbid_subprocess(*_args: Any, **_kwargs: Any) -> Any:
        raise RuntimeError("EB-04S forbids every subprocess/model invocation")

    module.subprocess.run = forbid_subprocess
    attempt = json.loads(ATTEMPT.read_text(encoding="utf-8"))
    if attempt.get("status") != "COMPLETE" or attempt.get("result_count") != 48:
        raise RuntimeError("EB-04R attempt is not the exact complete retained population")
    lanes = module.legacy.fixed_lanes()
    expected_keys = {
        f"{lane.lane_id}/{cell}" for lane in lanes for cell in module.CELLS
    }
    if set(attempt["results"]) != expected_keys:
        raise RuntimeError("EB-04R attempt keys differ from the frozen 12x4 population")
    frozen_identity_checks = {
        "tool_sha256": sha256(EB04R_TOOL),
        "protocol_sha256": sha256(module.PROTOCOL),
        "eb04_tool_sha256": sha256(module.EB04_TOOL),
        "eb04_protocol_sha256": sha256(module.EB04_PROTOCOL),
        "eb04_report_sha256": sha256(module.EB04_REPORT),
        "eb04e_report_sha256": sha256(module.EB04E_REPORT),
        "decision_dependency_sha256": module.decision_dependency_hashes(),
        "source_input_tree_sha256": module.source_input_tree_hashes(),
        "freeze_receipt_sha256": sha256(module.FREEZE),
    }
    for key, observed in frozen_identity_checks.items():
        if attempt.get(key) != observed:
            raise RuntimeError(f"EB-04R frozen identity drift for {key}")
    module.assert_population_matches_eb04(lanes)
    if attempt.get("population") != module.population_manifest(lanes):
        raise RuntimeError("fixture, observation, role, filter, or lane metadata drift")
    if attempt.get("frozen_cells") != module.serializable_cells():
        raise RuntimeError("target-selector drift")
    if attempt.get("frozen_non_target_environment") != module.NON_TARGET_ENV:
        raise RuntimeError("non-target selector drift")
    # The inherited pure report reducer asks Git for HEAD metadata. Bind that
    # field to the executed attempt instead so this analysis launches no process.
    module.legacy.git_head = lambda: attempt["source_commit"]

    results: dict[tuple[str, str], dict[str, Any]] = {}
    for lane in lanes:
        for cell in module.CELLS:
            results[(lane.lane_id, cell)] = module.audit_cell(lane, cell, attempt)
    maximum_vapor_aggregation_residual = max(
        result["physical"]["maximum_residuals"]["vapor_kg_m2"]
        for result in results.values()
    )
    frozen_vapor_aggregation_gate = maximum_vapor_aggregation_residual <= 1.0e-12
    population_gate = frozen_vapor_aggregation_gate and all(
        result["execution_status"] == "PASS" for result in results.values()
    )
    observation_accessed = False
    if population_gate:
        for lane in lanes:
            for cell in module.CELLS:
                module.score_cell(lane, cell, results[(lane.lane_id, cell)])
        observation_accessed = True

    report = module.legacy.adjudicate(lanes, results, attempt)
    module.strengthen_report(report, attempt)
    report["execution"].update(
        {
            "population_physical_and_provenance_gate_passes": population_gate,
            "physical_gate_passed_before_observation_load": population_gate,
            "observations_loaded_for_scoring": observation_accessed,
            "simulation_subprocesses_launched_by_eb04s": 0,
        }
    )
    if population_gate:
        module.independently_reconstruct_decision(report)
    else:
        report["independent_decision_reconstruction"] = {
            "status": "NOT_ASSESSED",
            "reason": "complete retained physical/provenance gate did not pass",
        }

    cells = [cell for lane in report["lanes"] for cell in lane["cells"].values()]
    provenance_files = 0
    for cell in cells:
        provenance = json.loads((REPO / cell["environment_provenance"]).read_text(encoding="utf-8"))
        provenance_files += len(provenance["files"])
    report["eb04s_retained_adjudication"] = {
        "schema": "snow-surface-eb04s-retained-adjudication-v1",
        "authority_seal_sha256": sha256(AUTHORITY_SEAL),
        "authority_freeze_sha256": seal["authority_freeze_sha256"],
        "canonical_contract_sha256": seal["canonical_contract_sha256"],
        "canonical_contract_version": 6,
        "vapor_to_sublimation_tolerance_kg_m2": 1.0e-6,
        "frozen_vapor_aggregation_tolerance_kg_m2": 1.0e-12,
        "maximum_vapor_aggregation_residual_kg_m2": maximum_vapor_aggregation_residual,
        "frozen_vapor_aggregation_gate": "PASS" if frozen_vapor_aggregation_gate else "FAIL",
        "frozen_identity_check_count": len(frozen_identity_checks),
        "frozen_population_and_selector_bindings": "PASS",
        "eb04r_attempt_sha256": sha256(ATTEMPT),
        "eb04r_package_tree_sha256_before": prior_package_hash,
        "retained_output_tree_sha256_before": retained_hash,
        "cell_count": len(cells),
        "provenance_file_identity_count": provenance_files,
        "model_rerun": False,
        "eb04r_history_rewritten": False,
    }
    if tree_sha256(EB04R_PACKAGE) != prior_package_hash:
        raise RuntimeError("EB-04R package changed during retained adjudication")
    if tree_sha256(RETAINED_ROOT) != retained_hash:
        raise RuntimeError("retained output tree changed during adjudication")
    report["eb04s_retained_adjudication"].update(
        {
            "eb04r_package_tree_sha256_after": prior_package_hash,
            "retained_output_tree_sha256_after": retained_hash,
            "immutable_input_check": "PASS",
        }
    )
    return report


def write_summary(report: dict[str, Any]) -> None:
    decision = report["decision"]
    execution = report["execution"]
    rubric = report["aggregate_rubric"]
    meta = report["eb04s_retained_adjudication"]
    maximum = max(
        cell["physical"]["maximum_residuals"]["vapor_sublimation_kg_m2"]
        for lane in report["lanes"]
        for cell in lane["cells"].values()
    )
    text = f"""# Retained-Output Scientific Adjudication

Evidence mode: `Ran` (analysis only; no model subprocess).

## Physical And Provenance Gate

- Immutable cells: `{meta['cell_count']}` (`12 lanes x 4 cells`).
- Retained provenance file identities rechecked: `{meta['provenance_file_identity_count']}`.
- EB-04R package and retained-output hashes before/after: `PASS`.
- Simulation subprocesses launched: `0`.
- Authority-bound vapor-to-sublimation tolerance: `1e-6 kg m^-2`.
- Maximum retained vapor-to-sublimation residual: `{maximum:.17g} kg m^-2`.
- EB-04R-frozen vapor-aggregation tolerance: `1e-12 kg m^-2`.
- Maximum retained vapor-aggregation residual: `{meta['maximum_vapor_aggregation_residual_kg_m2']:.17g} kg m^-2` (`{meta['frozen_vapor_aggregation_gate']}`).
- Frozen tool/protocol/source/fixture/observation/role/filter/selector/decision dependencies: `PASS`.
- Population physical/provenance gate: `{'PASS' if execution['population_physical_and_provenance_gate_passes'] else 'FAIL'}`.
- Observations loaded only after that complete gate: `{execution['observations_loaded_for_scoring']}`.

## Unchanged Empirical Rule

Baseline B score/failures: `{rubric['B']['robust_ordinal_score']}` / `{rubric['B']['robust_fail_count']}`.

Combined LS score/failures: `{rubric['LS']['robust_ordinal_score']}` / `{rubric['LS']['robust_fail_count']}`.

The combined mechanisms increase the robust ordinal score but do not reduce
the robust failure count. The prospectively frozen rule requires both.

Decision: `{decision['outcome']}`.

Stop-loss invoked: `{decision['stop_loss_invoked']}`. Another calibration or
factorial round authorized: `{decision['another_round_authorized']}`.

This successor decision does not rewrite EB-04R. It is a separate retrospective
adjudication under authority frozen without result access.
"""
    (ARTIFACTS / "retained-output-adjudication.md").write_text(text, encoding="utf-8")


def self_check() -> None:
    validate_authority()
    source = Path(__file__).read_text(encoding="utf-8")
    forbidden_imports = ("import " + "subprocess", "from " + "subprocess")
    if any(token in source for token in forbidden_imports):
        raise RuntimeError("adjudicator contains a subprocess launch path")
    if not ATTEMPT.is_file() or not RETAINED_ROOT.is_dir():
        raise RuntimeError("retained EB-04R evidence is unavailable")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--self-check", action="store_true")
    group.add_argument("--analyze", action="store_true")
    args = parser.parse_args()
    if args.self_check:
        self_check()
        print("EB-04S retained-only adjudicator self-check: PASS")
        return 0
    report = analyze()
    write_json(ARTIFACTS / "retained-adjudication.json", report)
    write_summary(report)
    print(json.dumps(report["decision"], indent=2, sort_keys=True))
    return 0 if report["decision"]["outcome"] != "HOLD_PHYSICAL_OR_PROVENANCE_GATE" else 2


if __name__ == "__main__":
    raise SystemExit(main())
