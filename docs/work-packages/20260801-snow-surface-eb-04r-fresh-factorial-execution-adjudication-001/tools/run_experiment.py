#!/usr/bin/env python3
"""Execute and adjudicate the prospectively frozen EB-04R factorial."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04r_factorial"
RUNS = OUTPUT / "runs"
BINARY = REPO / "target/debug/openwepp-cli-hill"
ATTEMPT = ARTIFACTS / "execution-attempt.json"
PROTOCOL = ARTIFACTS / "prospective-decision-protocol.md"
FREEZE = ARTIFACTS / "pre-execution-freeze.json"
OPERAND_LINEAGE = ARTIFACTS / "operand-lineage.csv"
EB04_PACKAGE = REPO / "docs/work-packages/20260730-snow-surface-eb-04-factorial-execution-adjudication-001"
EB04_TOOL = EB04_PACKAGE / "tools/run_factorial.py"
EB04_PROTOCOL = EB04_PACKAGE / "artifacts/prospective-decision-protocol.md"
EB04_REPORT = EB04_PACKAGE / "artifacts/factorial-results.json"
EB04E_PACKAGE = REPO / "docs/work-packages/20260731-snow-surface-eb-04e-corrected-population-runtime-qualification-001"
EB04E_AUDIT_TOOL = EB04E_PACKAGE / "tools/run_qualification.py"
EB04E_VERIFY_TOOL = EB04E_PACKAGE / "tools/verify_retained_outputs.py"
EB04E_REPORT = EB04E_PACKAGE / "artifacts/qualification-results.json"
EB04E_SEAL = EB04E_PACKAGE / "artifacts/retained-output-seal.json"
SCORING_DEPENDENCIES = {
    "cross_snotel_mechanism_rubric": REPO / "tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py",
    "observed_harness": REPO / "tools/snowfreeze_observed/observed_harness.py",
    "snotel_density_three_way": REPO / "tools/snowfreeze_observed/snotel_density_three_way.py",
}
PREFIX = "OPENWEPP_"

CELLS = {
    "B": ("disabled", "disabled"),
    "L": ("dilley_unsworth_subcanopy_v1", "disabled"),
    "S": ("disabled", "neutral_bulk_stage3_v1"),
    "LS": ("dilley_unsworth_subcanopy_v1", "neutral_bulk_stage3_v1"),
}
NON_TARGET_ENV = {
    "OPENWEPP_SNOWDENSITY1038_MELT_MODEL": "coe_liquid_holding_capacity_v1",
    "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "physics_bulk_multilayer_density_v1",
    "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL": "harder_pomeroy_hourly",
    "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "layered_thermal_liquid_v1",
}


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


legacy = load_module("eb04r_frozen_eb04", EB04_TOOL)
audit = load_module("eb04r_eb04e_audit", EB04E_AUDIT_TOOL)
retained = load_module("eb04r_eb04e_retained", EB04E_VERIFY_TOOL)

legacy.PACKAGE = PACKAGE
legacy.ARTIFACTS = ARTIFACTS
legacy.FIGURES = FIGURES
legacy.OUTPUT = OUTPUT
legacy.BINARY = BINARY
legacy.ATTEMPT = ATTEMPT
legacy.PROTOCOL = PROTOCOL
legacy.CELLS = CELLS
legacy.NON_TARGET_ENV = NON_TARGET_ENV

_save_figure = legacy.save_figure
_write_sidecar = legacy.write_sidecar


def mapped_save_figure(fig: Any, stem: str) -> None:
    _save_figure(fig, stem.replace("eb04-", "eb04r-", 1))


def mapped_write_sidecar(stem: str, *args: str) -> None:
    _write_sidecar(stem.replace("eb04-", "eb04r-", 1), *args)


legacy.save_figure = mapped_save_figure
legacy.write_sidecar = mapped_write_sidecar


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--self-check", action="store_true")
    group.add_argument("--freeze", action="store_true")
    group.add_argument("--execute", action="store_true")
    group.add_argument("--analysis-only", action="store_true")
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    FIGURES.mkdir(parents=True, exist_ok=True)
    RUNS.mkdir(parents=True, exist_ok=True)
    if args.self_check:
        self_check()
        print("EB-04R sanitizer, provenance, and anti-alias self-check: PASS")
        return 0
    if args.freeze:
        prepare_freeze()
        print(f"EB-04R pre-execution freeze: PASS ({sha256(FREEZE)})")
        return 0
    if args.execute:
        execute_once(args.workers)
    return analyze()


def sanitized_environment(
    ambient: dict[str, str], cell: str, trace: Path
) -> tuple[dict[str, str], list[str], dict[str, str]]:
    if cell not in CELLS:
        raise ValueError(f"unknown cell {cell}")
    removed = sorted(key for key in ambient if key.startswith(PREFIX))
    environment = {
        key: value for key, value in ambient.items() if not key.startswith(PREFIX)
    }
    longwave, sublimation = CELLS[cell]
    effective = {
        **NON_TARGET_ENV,
        "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": longwave,
        "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": sublimation,
        "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace.resolve()),
    }
    environment.update(effective)
    observed = {
        key: value for key, value in environment.items() if key.startswith(PREFIX)
    }
    if observed != effective or len(observed) != 7:
        raise RuntimeError("sanitized OPENWEPP environment is not the exact frozen mapping")
    return environment, removed, effective


def expected_selected_models(cell: str) -> dict[str, str]:
    return {
        "density": NON_TARGET_ENV["OPENWEPP_SNOWDENSITY09_DENSITY_MODEL"],
        "melt": NON_TARGET_ENV["OPENWEPP_SNOWDENSITY1038_MELT_MODEL"],
        "phase": NON_TARGET_ENV["OPENWEPP_SNOWDENSITY1035_PHASE_MODEL"],
        "liquid": NON_TARGET_ENV["OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL"],
        "longwave": CELLS[cell][0],
        "sublimation": CELLS[cell][1],
    }


def serializable_cells() -> dict[str, list[str]]:
    return {cell: list(selectors) for cell, selectors in CELLS.items()}


def self_check() -> None:
    required = [
        PROTOCOL,
        EB04_TOOL,
        EB04_PROTOCOL,
        EB04_REPORT,
        EB04E_AUDIT_TOOL,
        EB04E_VERIFY_TOOL,
        EB04E_REPORT,
        EB04E_SEAL,
        *SCORING_DEPENDENCIES.values(),
    ]
    if not all(path.is_file() for path in required):
        raise FileNotFoundError("required predecessor authority is missing")
    lanes = legacy.fixed_lanes()
    if len(lanes) != 12 or len({lane.lane_id for lane in lanes}) != 12:
        raise RuntimeError("frozen population is not exactly 12 unique lanes")
    expected_roles = {"INDEPENDENT_VALIDATION": 10, "DIAGNOSTIC_ONLY": 2}
    counts = {
        role: sum(lane.role == role for lane in lanes) for role in expected_roles
    }
    if counts != expected_roles:
        raise RuntimeError(f"observation-role drift: {counts}")
    assert_population_matches_eb04(lanes)
    qualification = json.loads(EB04E_REPORT.read_text(encoding="utf-8"))
    if qualification.get("acceptance_passes") is not True:
        raise RuntimeError("EB-04E prerequisite is not a recorded PASS")
    fake = {
        "PATH": "/usr/bin",
        "OPENWEPP_UNRELATED_SECRET": "must-not-survive",
        "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": "wrong",
    }
    env, removed, effective = sanitized_environment(
        fake, "LS", Path("/tmp/eb04r-self-check.snow.jsonl")
    )
    if sorted(removed) != [
        "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL",
        "OPENWEPP_UNRELATED_SECRET",
    ]:
        raise RuntimeError("sanitizer did not enumerate removed keys")
    if "must-not-survive" in json.dumps({"removed": removed, "effective": effective}):
        raise RuntimeError("removed environment value leaked into provenance")
    if env["PATH"] != "/usr/bin" or len(effective) != 7:
        raise RuntimeError("sanitizer damaged non-target ambient state")
    if effective["OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL"] != CELLS["LS"][0]:
        raise RuntimeError("target selector did not overwrite ambient state")
    audit.self_check()
    signed_mechanism_self_check()
    anti_alias = retained.anti_alias_checks(
        json.loads(EB04E_SEAL.read_text(encoding="utf-8"))
    )
    if not all(anti_alias.values()):
        raise RuntimeError(f"consumer anti-alias control failed: {anti_alias}")


def assert_population_matches_eb04(lanes: list[Any]) -> None:
    frozen = {
        lane["lane_id"]: lane
        for lane in json.loads(EB04_REPORT.read_text(encoding="utf-8"))["lanes"]
    }
    if set(frozen) != {lane.lane_id for lane in lanes}:
        raise RuntimeError("lane IDs drifted from immutable EB-04 population")
    for lane in lanes:
        old = frozen[lane.lane_id]
        current = {
            "corpus": lane.corpus,
            "climate": lane.climate,
            "stratum": lane.stratum,
            "role": lane.role,
            "fixture": relative(lane.fixture_dir),
            "observation_file": relative(lane.observation_file),
            "observation_sha256": sha256(lane.observation_file),
            "fixture_sha256": legacy.tree_sha256(lane.fixture_dir),
        }
        expected = {
            key: old[key]
            for key in (
                "corpus", "climate", "stratum", "role", "fixture",
                "observation_file", "observation_sha256",
            )
        }
        expected["fixture_sha256"] = old["cells"]["B"]["fixture_sha256"]
        if current != expected:
            raise RuntimeError(f"immutable EB-04 lane drift: {lane.lane_id}")


def prepare_freeze() -> None:
    if FREEZE.exists() or ATTEMPT.exists():
        raise RuntimeError("freeze or result-bearing attempt already exists")
    if not BINARY.is_file():
        raise FileNotFoundError("exact runner is missing")
    if any(path.is_file() for path in RUNS.rglob("*")):
        raise RuntimeError("result-bearing output exists before freeze")
    if "Status: `FROZEN BEFORE EXECUTION`" not in PROTOCOL.read_text(encoding="utf-8"):
        raise RuntimeError("prospective protocol is not marked frozen")
    for review in (ARTIFACTS / "review_agent_a.md", ARTIFACTS / "review_agent_b.md"):
        if "Status: `PASS_TO_FREEZE`" not in review.read_text(encoding="utf-8"):
            raise RuntimeError(f"pre-execution review has not passed: {review}")
    self_check()
    assert_clean_source_inputs()
    write_json(FREEZE, expected_freeze_receipt())


def execute_once(workers: int) -> None:
    if workers < 1:
        raise ValueError("workers must be positive")
    if ATTEMPT.exists():
        raise RuntimeError("EB-04R result-bearing attempt already exists; retry forbidden")
    if not BINARY.is_file():
        raise FileNotFoundError(f"build exact runner first: {BINARY}")
    self_check()
    assert_clean_source_inputs()
    freeze = validate_freeze_receipt()
    lanes = legacy.fixed_lanes()
    assert_population_matches_eb04(lanes)
    expected = {(lane.lane_id, cell) for lane in lanes for cell in CELLS}
    existing = [path for path in RUNS.rglob("*") if path.is_file()]
    if existing:
        raise RuntimeError("pre-existing result output violates one-round protocol")
    started = time.time()
    attempt: dict[str, Any] = {
        "schema": "snow-surface-eb04r-execution-attempt-v1",
        "status": "STARTED",
        "started_unix_seconds": started,
        "source_commit": git_head(),
        "source_test_diff_sha256": source_test_diff_sha256(),
        "source_input_tree_sha256": source_input_tree_hashes(),
        "decision_dependency_sha256": decision_dependency_hashes(),
        "binary_sha256": sha256(BINARY),
        "executed_binary_sha256": sha256(BINARY),
        "tool_sha256": sha256(Path(__file__)),
        "protocol_sha256": sha256(PROTOCOL),
        "eb04_tool_sha256": sha256(EB04_TOOL),
        "eb04_protocol_sha256": sha256(EB04_PROTOCOL),
        "eb04_report_sha256": sha256(EB04_REPORT),
        "eb04e_report_sha256": sha256(EB04E_REPORT),
        "freeze_receipt_sha256": sha256(FREEZE),
        "freeze_id": freeze["freeze_id"],
        "population_keys": sorted(f"{lane}/{cell}" for lane, cell in expected),
        "population": population_manifest(lanes),
        "frozen_cells": serializable_cells(),
        "frozen_non_target_environment": NON_TARGET_ENV,
        "retry_policy": "NO_RETRY",
        "workers": workers,
        "results": {},
    }
    write_json(ATTEMPT, attempt)
    results: dict[str, Any] = {}
    executor = ThreadPoolExecutor(max_workers=workers)
    futures: dict[Any, tuple[str, str]] = {}
    try:
        for lane in lanes:
            for cell in CELLS:
                futures[executor.submit(execute_cell, lane, cell)] = (
                    lane.lane_id, cell
                )
        for future in as_completed(futures):
            lane_id, cell = futures[future]
            result = future.result()
            key = f"{lane_id}/{cell}"
            results[key] = result
            attempt["status"] = "RUNNING"
            attempt["results"] = dict(sorted(results.items()))
            attempt["result_count"] = len(results)
            write_json(ATTEMPT, attempt)
            print(f"{key}: {'PASS' if result['returncode'] == 0 else 'FAIL'}")
    except BaseException as error:
        for future in futures:
            future.cancel()
        executor.shutdown(wait=True, cancel_futures=True)
        results.update(discover_completed_results(expected))
        attempt.update(
            {
                "status": "INTERRUPTED_HOLD",
                "completed_unix_seconds": time.time(),
                "result_count": len(results),
                "results": dict(sorted(results.items())),
                "interruption_type": type(error).__name__,
                "interruption_message": str(error),
            }
        )
        write_json(ATTEMPT, attempt)
        raise
    else:
        executor.shutdown(wait=True)
    if set(results) != {f"{lane}/{cell}" for lane, cell in expected}:
        attempt.update(
            {
                "status": "INTERRUPTED_HOLD",
                "completed_unix_seconds": time.time(),
                "result_count": len(results),
                "results": dict(sorted(results.items())),
                "interruption_type": "InventoryMismatch",
                "interruption_message": "execution result inventory differs from frozen population",
            }
        )
        write_json(ATTEMPT, attempt)
        raise RuntimeError("execution result inventory differs from frozen population")
    attempt.update(
        {
            "status": "COMPLETE",
            "completed_unix_seconds": time.time(),
            "result_count": len(results),
            "results": dict(sorted(results.items())),
        }
    )
    write_json(ATTEMPT, attempt)


def discover_completed_results(expected: set[tuple[str, str]]) -> dict[str, Any]:
    recovered: dict[str, Any] = {}
    for lane_id, cell in expected:
        path = RUNS / lane_id / cell / "eb04r-experiment-provenance.json"
        if not path.is_file():
            continue
        provenance = json.loads(path.read_text(encoding="utf-8"))
        recovered[f"{lane_id}/{cell}"] = {
            "returncode": int(provenance["returncode"]),
            "provenance": relative(path),
            "provenance_sha256": sha256(path),
        }
    return recovered


def execute_cell(lane: Any, cell: str) -> dict[str, Any]:
    run_dir = RUNS / lane.lane_id / cell
    run_dir.mkdir(parents=True, exist_ok=True)
    stem = f"{lane.lane_id}-{cell}"
    trace = run_dir / f"{stem}.snow.jsonl"
    wat = run_dir / f"{stem}.wat.parquet"
    runfile = run_dir / f"{stem}.run"
    manifest = run_dir / "openwepp_hillslope_run_manifest.json"
    stdout = run_dir / "stdout.txt"
    stderr = run_dir / "stderr.txt"
    provenance_path = run_dir / "eb04r-experiment-provenance.json"
    for path in (trace, wat, runfile, manifest, stdout, stderr, provenance_path):
        if path.exists():
            raise RuntimeError(f"pre-existing cell artifact violates one-run protocol: {path}")
    fixture_stem = legacy.observed_harness.discover_run_stem(lane.fixture_dir)
    legacy.observed_harness.write_runfile(
        runfile, lane.fixture_dir, fixture_stem, run_dir, stem
    )
    command = legacy.observed_harness.cli_command(
        BINARY, lane.fixture_dir, runfile, run_dir, "direct-production-executor"
    )
    environment, removed, effective = sanitized_environment(os.environ, cell, trace)
    started = time.time()
    completed = subprocess.run(
        command,
        cwd=REPO,
        env=environment,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    stdout.write_text(completed.stdout, encoding="utf-8")
    stderr.write_text(completed.stderr, encoding="utf-8")
    finished = time.time()
    files = {
        name: file_identity(path)
        for name, path in {
            "runfile": runfile,
            "runtime_manifest": manifest,
            "wat": wat,
            "trace": trace,
            "stdout": stdout,
            "stderr": stderr,
        }.items()
        if path.is_file()
    }
    provenance = {
        "schema": "snow-surface-eb04r-cell-provenance-v1",
        "lane_id": lane.lane_id,
        "cell": cell,
        "started_unix_seconds": started,
        "completed_unix_seconds": finished,
        "returncode": completed.returncode,
        "argv": [str(value) for value in command],
        "source_commit": git_head(),
        "source_test_diff_sha256": source_test_diff_sha256(),
        "source_input_tree_sha256": source_input_tree_hashes(),
        "decision_dependency_sha256": decision_dependency_hashes(),
        "binary_sha256": sha256(BINARY),
        "tool_sha256": sha256(Path(__file__)),
        "protocol_sha256": sha256(PROTOCOL),
        "fixture": relative(lane.fixture_dir),
        "fixture_sha256": legacy.tree_sha256(lane.fixture_dir),
        "environment_policy": "REMOVE_ALL_INHERITED_OPENWEPP_THEN_INSTALL_EXACT_SEVEN",
        "removed_openwepp_key_names": removed,
        "effective_openwepp_environment": effective,
        "selected_models": expected_selected_models(cell),
        "selection_evidence": "sanitized selector input plus source/binary identity; runtime trace behavior independently corroborated during audit",
        "files": files,
    }
    write_json(provenance_path, provenance)
    return {
        "returncode": completed.returncode,
        "provenance": relative(provenance_path),
        "provenance_sha256": sha256(provenance_path),
    }


def analyze() -> int:
    if not ATTEMPT.is_file():
        raise FileNotFoundError("analysis requires the result-bearing attempt")
    attempt = json.loads(ATTEMPT.read_text(encoding="utf-8"))
    validate_attempt(attempt)
    lanes = legacy.fixed_lanes()
    results: dict[tuple[str, str], dict[str, Any]] = {}
    for lane in lanes:
        for cell in CELLS:
            results[(lane.lane_id, cell)] = audit_cell(lane, cell, attempt)
    population_gate = all(
        result["execution_status"] == "PASS" for result in results.values()
    )
    if population_gate:
        for lane in lanes:
            for cell in CELLS:
                score_cell(lane, cell, results[(lane.lane_id, cell)])
    report = legacy.adjudicate(lanes, results, attempt)
    strengthen_report(report, attempt)
    report["execution"].update(
        {
            "population_physical_and_provenance_gate_passes": population_gate,
            "physical_gate_passed_before_observation_load": population_gate,
            "observations_loaded_for_scoring": population_gate,
        }
    )
    if population_gate:
        independently_reconstruct_decision(report)
    else:
        report["independent_decision_reconstruction"] = {
            "status": "NOT_ASSESSED",
            "reason": "complete population physical/provenance gate did not pass",
        }
    write_json(ARTIFACTS / "factorial-results.json", report)
    legacy.write_effects_csv(report)
    write_scientific_summary(report)
    for path in FIGURES.glob("eb04r-*.*"):
        path.unlink()
    if report["execution"]["all_cells_completed"]:
        legacy.make_figures(lanes, report)
    else:
        make_hold_figure(report)
    normalize_generated_text()
    legacy.check_artifact_inventory()
    write_evidence(report)
    print(json.dumps(report["decision"], indent=2, sort_keys=True))
    return 0 if report["decision"]["outcome"] != "HOLD_PHYSICAL_OR_PROVENANCE_GATE" else 2


def validate_attempt(attempt: dict[str, Any]) -> None:
    if attempt.get("status") != "COMPLETE" or attempt.get("result_count") != 48:
        raise RuntimeError("result-bearing attempt is not exactly complete")
    checks = {
        "source_commit": git_head(),
        "source_test_diff_sha256": source_test_diff_sha256(),
        "tool_sha256": sha256(Path(__file__)),
        "protocol_sha256": sha256(PROTOCOL),
        "eb04_tool_sha256": sha256(EB04_TOOL),
        "eb04_protocol_sha256": sha256(EB04_PROTOCOL),
        "eb04_report_sha256": sha256(EB04_REPORT),
        "eb04e_report_sha256": sha256(EB04E_REPORT),
        "decision_dependency_sha256": decision_dependency_hashes(),
        "source_input_tree_sha256": source_input_tree_hashes(),
        "freeze_receipt_sha256": sha256(FREEZE),
    }
    for key, expected in checks.items():
        if attempt.get(key) != expected:
            raise RuntimeError(f"attempt identity drift for {key}")
    expected_keys = {
        f"{lane.lane_id}/{cell}" for lane in legacy.fixed_lanes() for cell in CELLS
    }
    if set(attempt["results"]) != expected_keys:
        raise RuntimeError("attempt keys differ from the exact frozen 12x4 matrix")
    lanes = legacy.fixed_lanes()
    assert_population_matches_eb04(lanes)
    if attempt.get("population") != population_manifest(lanes):
        raise RuntimeError("fixture, observation, role, or lane metadata drift")
    if attempt.get("frozen_cells") != serializable_cells():
        raise RuntimeError("cell selector drift")
    if attempt.get("frozen_non_target_environment") != NON_TARGET_ENV:
        raise RuntimeError("non-target selector drift")


def audit_cell(lane: Any, cell: str, attempt: dict[str, Any]) -> dict[str, Any]:
    run_dir = RUNS / lane.lane_id / cell
    stem = f"{lane.lane_id}-{cell}"
    trace = run_dir / f"{stem}.snow.jsonl"
    wat = run_dir / f"{stem}.wat.parquet"
    runfile = run_dir / f"{stem}.run"
    manifest_path = run_dir / "openwepp_hillslope_run_manifest.json"
    stdout = run_dir / "stdout.txt"
    stderr = run_dir / "stderr.txt"
    provenance_path = run_dir / "eb04r-experiment-provenance.json"
    provenance = json.loads(provenance_path.read_text(encoding="utf-8"))
    key = f"{lane.lane_id}/{cell}"
    provenance_passes = validate_cell_provenance(
        key, cell, trace, wat, runfile, manifest_path, stdout, stderr,
        provenance_path, provenance, attempt
    )
    returncode = int(provenance["returncode"])
    base = {
        "lane_id": lane.lane_id,
        "cell": cell,
        "returncode": returncode,
        "fixture_sha256": legacy.tree_sha256(lane.fixture_dir),
        "non_target_environment": NON_TARGET_ENV,
        "target_selectors": {
            "longwave": CELLS[cell][0],
            "sublimation": CELLS[cell][1],
        },
        "environment_provenance": relative(provenance_path),
        "environment_provenance_sha256": sha256(provenance_path),
        "environment_provenance_passes": provenance_passes,
        "run_dir": relative(run_dir),
        "command": provenance["argv"],
    }
    if returncode != 0 or not all(path.is_file() for path in (trace, wat, manifest_path)):
        stderr = run_dir / "stderr.txt"
        failure = stderr.read_text(encoding="utf-8").splitlines() if stderr.is_file() else []
        base.update(
            {
                "execution_status": "FAIL",
                "failure": failure[-1] if failure else "incomplete retained output",
                "physical": {"passes": False},
                "state": {},
                "rubric_profile": None,
                "observation_metrics": None,
                "frost_metrics": None,
            }
        )
        return base
    independent = audit.audit_trace(trace)
    cross = retained.verify_wat_trace(trace, wat, cell)
    rows = legacy.read_jsonl(trace)
    modeled, wat_rows = legacy.load_wat(wat)
    signed = signed_mechanism_audit(rows, cell)
    physical = {
        **independent,
        "passes": (
            independent["physical_passes"]
            and cross["selector_behavior_passes"]
            and cross["non_target_trace_identity_passes"]
            and cross["layer_state_coupling_passes"]
            and cross["finite_operands_pass"]
            and cross["hourly_vector_length_passes"]
            and signed["passes"]
            and cross["maximum_abs_swe_residual_m"] <= 1.0e-9
            and cross["maximum_abs_depth_residual_m"] <= 1.0e-9
        ),
        "wat_trace": cross,
        "signed_mechanism": signed,
        "total_longwave_energy_mj_m2": independent["total_longwave_energy_j_m2"] / 1.0e6,
        "total_latent_energy_mj_m2": independent["total_latent_energy_j_m2"] / 1.0e6,
    }
    state = legacy.state_metrics(modeled, wat_rows, rows)
    base.update(
        {
            "execution_status": "PASS" if physical["passes"] and provenance_passes else "FAIL",
            "wat": relative(wat),
            "trace": relative(trace),
            "wat_sha256": sha256(wat),
            "trace_sha256": sha256(trace),
            "trace_row_count": independent["trace_row_count"],
            "physical": physical,
            "state": state,
            "rubric_profile": None,
            "observation_metrics": None,
            "frost_metrics": None,
            "selected_models_corroborated": (
                cross["non_target_trace_identity_passes"]
                and cross["selector_behavior_passes"]
            ),
        }
    )
    return base


def score_cell(lane: Any, cell: str, result: dict[str, Any]) -> None:
    """Load observations only after the complete population gate has passed."""
    wat = REPO / result["wat"]
    modeled, _ = legacy.load_wat(wat)
    observations = legacy.load_observations(lane)
    if lane.role == "INDEPENDENT_VALIDATION":
        result["rubric_profile"] = legacy.rubric.rubric_profile(
            observations, modeled, cell
        )
        result["observation_metrics"] = legacy.rubric.model_metrics(
            observations, modeled, cell
        )
    elif lane.role == "DIAGNOSTIC_ONLY":
        result["frost_metrics"] = legacy.frost_metrics(observations, modeled)


def signed_mechanism_audit(rows: list[dict[str, Any]], cell: str) -> dict[str, Any]:
    violations = 0
    active_sublimation_rows = 0
    for row in rows:
        sublimation = float(row["sublimation_m"])
        vapor = float(row["stage3_vapor_mass_exchange_kg_m2"])
        latent = float(row["stage3_latent_energy_j_m2"])
        hourly_mass = [float(value) for value in row["stage3_hourly_vapor_mass_exchange_kg_m2"]]
        hourly_flux = [float(value) for value in row["stage3_hourly_latent_flux_w_m2"]]
        if sublimation < 0.0 or vapor > 1.0e-12 or latent > 1.0e-6:
            violations += 1
        if any(value > 1.0e-12 for value in hourly_mass) or any(
            value > 1.0e-12 for value in hourly_flux
        ):
            violations += 1
        if sublimation > 0.0:
            active_sublimation_rows += 1
            if cell not in {"S", "LS"} or vapor >= 0.0 or latent >= 0.0:
                violations += 1
        elif cell not in {"S", "LS"} and (vapor != 0.0 or latent != 0.0):
            violations += 1
    return {
        "row_level_sign_violation_count": violations,
        "active_sublimation_row_count": active_sublimation_rows,
        "passes": violations == 0,
    }


def signed_mechanism_self_check() -> None:
    zero = {
        "sublimation_m": 0.0,
        "stage3_vapor_mass_exchange_kg_m2": 0.0,
        "stage3_latent_energy_j_m2": 0.0,
        "stage3_hourly_vapor_mass_exchange_kg_m2": [0.0] * 24,
        "stage3_hourly_latent_flux_w_m2": [0.0] * 24,
    }
    if not signed_mechanism_audit([zero], "B")["passes"]:
        raise RuntimeError("valid disabled signed-mechanism control failed")
    wrong_sign = dict(zero)
    wrong_sign.update(
        sublimation_m=0.001,
        stage3_vapor_mass_exchange_kg_m2=1.0,
        stage3_latent_energy_j_m2=1.0,
    )
    if signed_mechanism_audit([wrong_sign], "S")["passes"]:
        raise RuntimeError("positive vapor/latent wrong-sign alias was accepted")
    disabled_alias = dict(zero)
    disabled_alias.update(
        sublimation_m=0.001,
        stage3_vapor_mass_exchange_kg_m2=-1.0,
        stage3_latent_energy_j_m2=-1.0,
        stage3_hourly_vapor_mass_exchange_kg_m2=[-1.0 / 24.0] * 24,
        stage3_hourly_latent_flux_w_m2=[-1.0 / 24.0] * 24,
    )
    if signed_mechanism_audit([disabled_alias], "B")["passes"]:
        raise RuntimeError("disabled-path nonzero alias was accepted")


def validate_cell_provenance(
    key: str,
    cell: str,
    trace: Path,
    wat: Path,
    runfile: Path,
    manifest_path: Path,
    stdout: Path,
    stderr: Path,
    provenance_path: Path,
    provenance: dict[str, Any],
    attempt: dict[str, Any],
) -> bool:
    expected_effective = sanitized_environment({}, cell, trace)[2]
    recorded = attempt["results"][key]
    if recorded["provenance_sha256"] != sha256(provenance_path):
        return False
    if provenance["effective_openwepp_environment"] != expected_effective:
        return False
    if provenance.get("selected_models") != expected_selected_models(cell):
        return False
    if set(provenance["effective_openwepp_environment"]) != set(expected_effective):
        return False
    if provenance["environment_policy"] != "REMOVE_ALL_INHERITED_OPENWEPP_THEN_INSTALL_EXACT_SEVEN":
        return False
    removed = provenance.get("removed_openwepp_key_names")
    if (
        not isinstance(removed, list)
        or len(removed) != len(set(removed))
        or removed != sorted(removed)
        or any(not isinstance(name, str) or not name.startswith(PREFIX) for name in removed)
    ):
        return False
    if provenance["source_commit"] != attempt["source_commit"]:
        return False
    if provenance.get("source_test_diff_sha256") != attempt["source_test_diff_sha256"]:
        return False
    if provenance.get("source_input_tree_sha256") != attempt["source_input_tree_sha256"]:
        return False
    if provenance.get("decision_dependency_sha256") != attempt["decision_dependency_sha256"]:
        return False
    if provenance["binary_sha256"] != attempt["binary_sha256"]:
        return False
    if provenance["tool_sha256"] != attempt["tool_sha256"] or provenance["protocol_sha256"] != attempt["protocol_sha256"]:
        return False
    started = float(provenance.get("started_unix_seconds", -1.0))
    completed = float(provenance.get("completed_unix_seconds", -1.0))
    if not (
        float(attempt["started_unix_seconds"]) <= started <= completed
        <= float(attempt["completed_unix_seconds"])
    ):
        return False
    for name, path in {
        "runfile": runfile, "runtime_manifest": manifest_path, "wat": wat,
        "trace": trace, "stdout": stdout, "stderr": stderr,
    }.items():
        if not path.is_file() or provenance["files"].get(name) != file_identity(path):
            return False
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    return bool(
        manifest["source_commit"] == attempt["source_commit"]
        and manifest["binary_sha256"] == attempt["binary_sha256"]
        and manifest["runtime_selection"]["selected"] == "direct-production-executor"
        and manifest["runtime_selection"]["fallback_reason"] is None
        and manifest["execution_provenance"]["scheduler_outcome_class"] == "completed"
        and manifest["output_checksums"].get(str(wat.resolve())) == sha256(wat)
        and manifest["input_checksums"].get(str(runfile.resolve())) == sha256(runfile)
        and manifest["argv"] == provenance["argv"]
    )


def strengthen_report(report: dict[str, Any], attempt: dict[str, Any]) -> None:
    cells = [cell for lane in report["lanes"] for cell in lane["cells"].values()]
    all_provenance = all(cell.get("environment_provenance_passes", False) for cell in cells)
    enabled_longwave = sum(
        abs(cell["physical"]["total_longwave_energy_j_m2"]) > 0.0
        for lane in report["lanes"]
        for name, cell in lane["cells"].items()
        if name in {"L", "LS"} and cell["execution_status"] == "PASS"
    )
    enabled_sublimation = sum(
        cell["physical"]["total_sublimation_m"] > 0.0
        for lane in report["lanes"]
        for name, cell in lane["cells"].items()
        if name in {"S", "LS"} and cell["execution_status"] == "PASS"
    )
    disabled_clean = all(
        (name in {"L", "LS"} or cell["physical"]["total_longwave_energy_j_m2"] == 0.0)
        and (name in {"S", "LS"} or cell["physical"]["total_sublimation_m"] == 0.0)
        for lane in report["lanes"]
        for name, cell in lane["cells"].items()
        if cell["execution_status"] == "PASS"
    )
    report["schema"] = "snow-surface-eb04r-factorial-v1"
    report["source"].update(
        {
            "binary_sha256": attempt["binary_sha256"],
            "tool_sha256": attempt["tool_sha256"],
            "protocol_sha256": attempt["protocol_sha256"],
            "eb04_protocol_sha256": attempt["eb04_protocol_sha256"],
            "eb04_report_sha256": attempt["eb04_report_sha256"],
            "eb04e_report_sha256": attempt["eb04e_report_sha256"],
            "decision_dependency_sha256": attempt["decision_dependency_sha256"],
            "source_input_tree_sha256": attempt["source_input_tree_sha256"],
            "environment_policy": "REMOVE_ALL_INHERITED_OPENWEPP_THEN_INSTALL_EXACT_SEVEN",
        }
    )
    report["execution"].update(
        {
            "all_environment_provenance_passes": all_provenance,
            "longwave_enabled_nonzero_cells": enabled_longwave,
            "sublimation_enabled_nonzero_cells": enabled_sublimation,
            "disabled_target_paths_clean": disabled_clean,
        }
    )
    criteria = report["decision"]["criteria"]
    criteria["all_ls_physical_and_trace_gates_pass"] &= all_provenance
    criteria["mechanism_operands_nonzero"] = (
        enabled_longwave == 24
        and enabled_sublimation == 24
        and disabled_clean
        and all(
            cell["physical"].get("signed_mechanism", {}).get("passes", False)
            for cell in cells
        )
    )
    criteria["no_forcing_or_input_mutation"] &= all_provenance
    report["decision"]["criterion_status"] = {
        key: "PASS" if value else "FAIL" for key, value in criteria.items()
    }
    physical_gate = (
        report["execution"]["all_cells_completed"]
        and all_provenance
        and all(cell["physical"].get("passes", False) for cell in cells)
    )
    if not physical_gate:
        outcome = "HOLD_PHYSICAL_OR_PROVENANCE_GATE"
    elif all(criteria.values()):
        outcome = "GO_TO_EB05_PROMOTION_ASSESSMENT"
    else:
        outcome = "CLOSE_NONPROMOTION_EMPIRICAL_RULE"
    report["decision"].update(
        {
            "outcome": outcome,
            "stop_loss_invoked": outcome != "GO_TO_EB05_PROMOTION_ASSESSMENT",
            "another_round_authorized": False,
        }
    )
    old = json.loads(EB04_REPORT.read_text(encoding="utf-8"))
    report["bounded_eb04_comparison"] = {
        "purpose": "availability and completion context only; corrected-runtime scores are not compared to defect-affected EB-04 scores",
        "eb04_completed_cells": sum(
            cell["execution_status"] == "PASS"
            for lane in old["lanes"] for cell in lane["cells"].values()
        ),
        "eb04r_completed_cells": sum(cell["execution_status"] == "PASS" for cell in cells),
        "eb04_ls_scored_independent_lanes": old["aggregate_rubric"]["LS"]["complete_lane_count"],
        "eb04r_ls_scored_independent_lanes": report["aggregate_rubric"]["LS"]["complete_lane_count"],
        "score_comparison_authorized": False,
        "reason": "EB-04 was invalidated by runtime thermal/layer defects corrected before EB-04R",
    }


def independently_reconstruct_decision(report: dict[str, Any]) -> None:
    """Rebuild empirical reductions without calling the inherited reducers."""
    lanes = [lane for lane in report["lanes"] if lane["role"] == "INDEPENDENT_VALIDATION"]
    scores = {"fail": 0, "marginal": 1, "pass": 2, "strong": 3}

    def labels(lane: dict[str, Any], cell: str) -> dict[str, str]:
        return {
            item["cell_id"]: item["ordinal_label"]
            for item in lane["cells"][cell]["rubric_profile"]["cells"]
            if item["forcing_robust"] and item["ordinal_label"] in scores
        }

    aggregates: dict[str, Any] = {}
    for cell in CELLS:
        values = [value for lane in lanes for value in labels(lane, cell).values()]
        aggregates[cell] = {
            "robust_ordinal_score": sum(scores[value] for value in values),
            "robust_fail_count": sum(value == "fail" for value in values),
            "robust_available_cell_count": len(values),
            "complete_lane_count": len(lanes),
        }
        for key, value in aggregates[cell].items():
            if report["aggregate_rubric"][cell][key] != value:
                raise RuntimeError(f"independent aggregate mismatch: {cell}/{key}")

    new_failures = []
    for lane in lanes:
        baseline = labels(lane, "B")
        for rubric_cell, value in labels(lane, "LS").items():
            if value == "fail" and baseline.get(rubric_cell) != "fail":
                new_failures.append((lane["lane_id"], rubric_cell, baseline.get(rubric_cell)))
    reported_new = [
        (item["lane_id"], item["rubric_cell"], None if item["baseline"] == "unavailable" else item["baseline"])
        for item in report["new_robust_failures"]
    ]
    if sorted(new_failures) != sorted(reported_new):
        raise RuntimeError("independent new-failure reconstruction mismatch")

    protected: dict[str, dict[str, int]] = {}
    for group, predicate in {
        "open_controls": lambda lane: lane["stratum"] == "open",
        "canopy_strata": lambda lane: lane["stratum"] != "open",
    }.items():
        protected[group] = {
            cell: sum(
                scores[value]
                for lane in lanes if predicate(lane)
                for value in labels(lane, cell).values()
            )
            for cell in CELLS
        }
    if protected != report["protected_group_scores"]:
        raise RuntimeError("independent protected-group reconstruction mismatch")

    findings = []
    for lane in lanes:
        for response in ("mean_swe_m", "median_disappearance_dowy"):
            effect = lane["effects"][response]
            longwave, sublimation, combined = (
                effect["longwave_main"], effect["sublimation_main"], effect["combined"]
            )
            if longwave * sublimation < 0.0 and abs(combined) < max(abs(longwave), abs(sublimation)):
                findings.append((lane["lane_id"], response, longwave, sublimation, combined))
    reported = [
        (item["lane_id"], item["response"], item["longwave_main"], item["sublimation_main"], item["combined"])
        for item in report["compensation_audit"]["findings"]
    ]
    if sorted(findings) != sorted(reported) or report["compensation_audit"]["evaluated"] is not True:
        raise RuntimeError("independent compensation reconstruction mismatch")
    all_cells = [cell for lane in report["lanes"] for cell in lane["cells"].values()]
    reconstructed_criteria = {
        "all_ls_physical_and_trace_gates_pass": all(
            lane["trace_identity_passes"]
            and lane["cells"]["LS"]["physical"]["passes"]
            and lane["cells"]["LS"]["environment_provenance_passes"]
            for lane in report["lanes"]
        ),
        "robust_ordinal_score_increases": (
            aggregates["LS"]["robust_ordinal_score"]
            > aggregates["B"]["robust_ordinal_score"]
        ),
        "robust_fail_count_decreases": (
            aggregates["LS"]["robust_fail_count"]
            < aggregates["B"]["robust_fail_count"]
        ),
        "no_new_lane_robust_fail": not new_failures,
        "protected_group_scores_not_worse": all(
            values["LS"] >= values["B"] for values in protected.values()
        ),
        "mechanism_operands_nonzero": (
            report["execution"]["longwave_enabled_nonzero_cells"] == 24
            and report["execution"]["sublimation_enabled_nonzero_cells"] == 24
            and report["execution"]["disabled_target_paths_clean"]
            and all(cell["physical"]["signed_mechanism"]["passes"] for cell in all_cells)
        ),
        "no_compensating_error_pattern": not findings,
        "no_forcing_or_input_mutation": all(
            lane["trace_identity_passes"] for lane in report["lanes"]
        ) and all(cell["environment_provenance_passes"] for cell in all_cells),
    }
    if reconstructed_criteria != report["decision"]["criteria"]:
        raise RuntimeError("independent eight-part criterion reconstruction mismatch")
    reconstructed_outcome = (
        "GO_TO_EB05_PROMOTION_ASSESSMENT"
        if all(reconstructed_criteria.values())
        else "CLOSE_NONPROMOTION_EMPIRICAL_RULE"
    )
    if reconstructed_outcome != report["decision"]["outcome"]:
        raise RuntimeError("independent final-outcome reconstruction mismatch")
    report["independent_decision_reconstruction"] = {
        "passes": True,
        "aggregates": aggregates,
        "new_failure_count": len(new_failures),
        "protected_group_scores": protected,
        "compensation_finding_count": len(findings),
        "criteria": reconstructed_criteria,
        "outcome": reconstructed_outcome,
    }


def write_scientific_summary(report: dict[str, Any]) -> None:
    decision = report["decision"]
    aggregate = report["aggregate_rubric"]
    lines = [
        "# EB-04R Scientific Adjudication",
        "",
        f"Status: `{decision['outcome']}`",
        "",
        "Evidence class: `Ran`",
        "",
        "## Primary Result",
        "",
        (
            f"B and LS each completed `{aggregate['B']['complete_lane_count']}` "
            "independent-validation lanes. "
            f"B's forcing-robust ordinal score is `{aggregate['B']['robust_ordinal_score']}` "
            f"with `{aggregate['B']['robust_fail_count']}` failures; LS scores "
            f"`{aggregate['LS']['robust_ordinal_score']}` with "
            f"`{aggregate['LS']['robust_fail_count']}` failures."
        ),
        "",
        "## Promotion Criteria",
        "",
        "| Criterion | Result |",
        "| --- | --- |",
    ]
    lines.extend(
        f"| {key.replace('_', ' ')} | {value} |"
        for key, value in decision["criterion_status"].items()
    )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"`{decision['outcome']}`.",
            "",
            "## Claim Limits",
            "",
            "- Warm-maritime conifer transfer remains withheld.",
            "- SNOTEL open controls cannot identify canopy longwave.",
            "- Sleepers lanes remain diagnostic-only.",
            "- Deterministic effects are not sampling confidence intervals.",
            "- No coefficient, forcing, fixture, observation, rubric, default, or process equation changed.",
            "",
            "## Stop-Loss",
            "",
            (
                "No further round is authorized from these results."
                if decision["stop_loss_invoked"]
                else "The empirical stop-loss is not invoked; EB-05 may assess promotion."
            ),
            "",
        ]
    )
    (ARTIFACTS / "scientific-adjudication.md").write_text(
        "\n".join(lines), encoding="utf-8"
    )


def make_hold_figure(report: dict[str, Any]) -> None:
    """Render a readable audit-status matrix without assuming retained traces."""
    import matplotlib

    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    lane_names = [lane["lane_id"] for lane in report["lanes"]]
    cell_names = list(CELLS)
    status = [
        [
            int(lane["cells"][cell]["execution_status"] == "PASS")
            for cell in cell_names
        ]
        for lane in report["lanes"]
    ]
    fig, ax = plt.subplots(figsize=(7.5, 7.5))
    image = ax.imshow(status, cmap="RdYlGn", vmin=0, vmax=1, aspect="auto")
    ax.set_xticks(range(len(cell_names)), cell_names)
    ax.set_yticks(range(len(lane_names)), lane_names)
    ax.set_title("EB-04R physical and provenance audit status")
    fig.colorbar(image, ax=ax, ticks=[0, 1], label="0 held, 1 passed")
    legacy.save_figure(fig, "eb04r-factorial-audit-status")
    legacy.write_sidecar(
        "eb04r-factorial-audit-status",
        "Which frozen cells passed every runtime, provenance, and physical gate?",
        "All 12 fixed lanes and four B/L/S/LS cells; no observation score is loaded.",
        "Binary audit status: 1 passed, 0 held.",
        "Status combines subprocess completion, sealed files, sanitized selectors, WAT/trace/layer coupling, conservation, finiteness, and signed mechanism checks.",
        "A zero may reflect missing retained output or any failed provenance or physical gate; it does not identify a single cause.",
        "No failed cell is omitted, imputed, scored, or retried.",
        "The matrix shows why empirical adjudication was withheld while preserving the complete planned population.",
        "This figure cannot support a scientific promotion or nonpromotion claim because the population gate did not pass.",
    )


def write_evidence(report: dict[str, Any]) -> None:
    lanes = report["lanes"]
    cells = [cell for lane in lanes for cell in lane["cells"].values()]
    rows = sum(int(cell.get("trace_row_count", 0)) for cell in cells)
    maxima: dict[str, float] = {}
    passing = [cell for cell in cells if cell["execution_status"] == "PASS"]
    residual_keys = (
        passing[0]["physical"]["maximum_residuals"] if passing else {}
    )
    for key in residual_keys:
        maxima[key] = max(
            float(cell["physical"]["maximum_residuals"][key])
            for cell in passing
        )
    physical_pass = len(passing) == 48
    environment = [
        "# Environment Provenance",
        "",
        f"Status: `{'PASS' if physical_pass else 'HOLD'}`",
        "",
        "Evidence class: `Ran`",
        "",
        "The 48 planned cell provenance records are evaluated under policy",
        "`REMOVE_ALL_INHERITED_OPENWEPP_THEN_INSTALL_EXACT_SEVEN`. Removed",
        "ambient key names are retained without values. Every effective mapping",
        "contains exactly four non-target selectors, two target selectors, and",
        "one cell-specific trace path. Command, source, binary, tool, protocol,",
        "fixture, runfile, runtime manifest, WAT, trace, stdout, stderr, timestamps,",
        "and return code must reconcile to the result-bearing attempt. The status",
        "above reports whether the complete population satisfied that requirement.",
        "",
    ]
    (ARTIFACTS / "environment-provenance.md").write_text(
        "\n".join(environment), encoding="utf-8"
    )
    conservation = [
        "# Independent Conservation And Consumer Evidence",
        "",
        f"Status: `{'PASS' if physical_pass else 'HOLD'}`",
        "",
        "Evidence class: `Ran`",
        "",
        f"The independent consumer streamed `{rows}` daily rows across 48 cells.",
        "Mass, surface/cold energy, hourly aggregation, vapor/latent, WAT,",
        "complete layer-vector, finite-number, and 24-hour cardinality gates were",
        "applied; the status above reports population-wide acceptance.",
        "",
        "| Residual | Population maximum |",
        "| --- | ---: |",
    ]
    conservation.extend(
        f"| {key.replace('_', ' ')} | `{value}` |" for key, value in maxima.items()
    )
    conservation.append("")
    (ARTIFACTS / "conservation-evidence.md").write_text(
        "\n".join(conservation), encoding="utf-8"
    )
    inventory = [
        "# Execution Inventory",
        "",
        f"Status: `{'PASS' if physical_pass else 'HOLD'}`",
        "",
        "Evidence class: `Ran`",
        "",
        f"Exactly `{len(lanes)}` lanes and `{len(cells)}` cells were attempted once;",
        f"`{sum(cell['execution_status'] == 'PASS' for cell in cells)}` completed.",
        f"The retained population contains `{rows}` daily WAT/trace rows.",
        "",
    ]
    (ARTIFACTS / "execution-inventory.md").write_text(
        "\n".join(inventory), encoding="utf-8"
    )
    stems = sorted(path.stem for path in FIGURES.glob("*.svg"))
    figure_lines = [
        "# Figure Inventory",
        "",
        "Status: `PASS`",
        "",
        f"Exactly `{len(stems)}` SVG figures have one Markdown sidecar each:",
        "",
    ] + [f"- `{stem}.svg` / `{stem}.md`" for stem in stems]
    figure_lines.append("")
    (ARTIFACTS / "figure-inventory.md").write_text(
        "\n".join(figure_lines), encoding="utf-8"
    )


def normalize_generated_text() -> None:
    paths = [ARTIFACTS / "factorial-effects.csv", *FIGURES.glob("*.svg")]
    for path in paths:
        lines = path.read_text(encoding="utf-8").splitlines()
        path.write_text("\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8")


def file_identity(path: Path) -> dict[str, Any]:
    stat = path.stat()
    return {
        "sha256": sha256(path),
        "size_bytes": stat.st_size,
        "mtime_ns": stat.st_mtime_ns,
        "ctime_ns": stat.st_ctime_ns,
    }


def population_manifest(lanes: list[Any]) -> list[dict[str, Any]]:
    return [
        {
            "lane_id": lane.lane_id,
            "corpus": lane.corpus,
            "role": lane.role,
            "stratum": lane.stratum,
            "climate": lane.climate,
            "fixture": relative(lane.fixture_dir),
            "fixture_sha256": legacy.tree_sha256(lane.fixture_dir),
            "observation_file": relative(lane.observation_file),
            "observation_sha256": sha256(lane.observation_file),
            "observation_filter": lane.observation_filter,
        }
        for lane in lanes
    ]


def source_test_diff_sha256() -> str:
    output = subprocess.run(
        ["git", "diff", "--binary", "--", "crates", "tests"],
        cwd=REPO,
        check=True,
        stdout=subprocess.PIPE,
    ).stdout
    return hashlib.sha256(output).hexdigest()


def source_input_tree_hashes() -> dict[str, str]:
    return {
        "crates": legacy.tree_sha256(REPO / "crates"),
        "tests": legacy.tree_sha256(REPO / "tests"),
    }


def assert_clean_source_inputs() -> None:
    status = subprocess.run(
        ["git", "status", "--porcelain=v1", "--untracked-files=all", "--", "crates", "tests"],
        cwd=REPO, check=True, text=True, stdout=subprocess.PIPE,
    ).stdout.strip()
    if status:
        raise RuntimeError(f"production/test source inputs are not clean:\n{status}")


def decision_dependency_hashes() -> dict[str, str]:
    dependencies = {
        "eb04_tool": EB04_TOOL,
        "eb04e_audit_tool": EB04E_AUDIT_TOOL,
        "eb04e_retained_verifier": EB04E_VERIFY_TOOL,
        "eb04e_retained_output_seal": EB04E_SEAL,
        "operand_lineage": OPERAND_LINEAGE,
        **SCORING_DEPENDENCIES,
    }
    return {name: sha256(path) for name, path in sorted(dependencies.items())}


def expected_freeze_receipt() -> dict[str, Any]:
    lanes = legacy.fixed_lanes()
    return {
        "schema": "snow-surface-eb04r-pre-execution-freeze-v1",
        "freeze_id": "SNOW-SURFACE-EB-04R-20260801-001",
        "status": "FROZEN",
        "source_commit": git_head(),
        "source_test_diff_sha256": source_test_diff_sha256(),
        "source_input_tree_sha256": source_input_tree_hashes(),
        "binary_sha256": sha256(BINARY),
        "tool_sha256": sha256(Path(__file__)),
        "protocol_sha256": sha256(PROTOCOL),
        "eb04_protocol_sha256": sha256(EB04_PROTOCOL),
        "eb04_report_sha256": sha256(EB04_REPORT),
        "eb04e_report_sha256": sha256(EB04E_REPORT),
        "decision_dependency_sha256": decision_dependency_hashes(),
        "population": population_manifest(lanes),
        "frozen_cells": serializable_cells(),
        "frozen_non_target_environment": NON_TARGET_ENV,
        "self_check": "PASS",
        "pre_execution_reviews": "PASS",
        "result_bearing_attempts_before_freeze": 0,
    }


def validate_freeze_receipt() -> dict[str, Any]:
    if not FREEZE.is_file():
        raise FileNotFoundError("frozen pre-execution receipt is missing")
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    expected = expected_freeze_receipt()
    if frozen != expected:
        raise RuntimeError("pre-execution freeze receipt does not match current identities")
    return frozen


def git_head() -> str:
    return subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO,
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    ).stdout.strip()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO))


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, value: Any) -> None:
    temporary = path.with_name(f".{path.name}.tmp")
    temporary.write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    temporary.replace(path)


if __name__ == "__main__":
    raise SystemExit(main())
