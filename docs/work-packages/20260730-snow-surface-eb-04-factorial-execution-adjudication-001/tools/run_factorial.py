#!/usr/bin/env python3
"""Execute and adjudicate the frozen SNOW-SURFACE-EB-04 factorial."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import json
import math
import os
import subprocess
import sys
import time
from collections import Counter
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from pathlib import Path
from typing import Any

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04_factorial"
BINARY = REPO / "target/debug/openwepp-cli-hill"
ATTEMPT = ARTIFACTS / "execution-attempt.json"
PROTOCOL = ARTIFACTS / "prospective-decision-protocol.md"
TOOL_DIR = REPO / "tools/snowfreeze_observed"
sys.path.insert(0, str(TOOL_DIR))

import cross_snotel_mechanism_rubric as cross  # noqa: E402
import observed_harness  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402

CELLS = {
    "B": ("disabled", "disabled"),
    "L": ("dilley_unsworth_subcanopy_v1", "disabled"),
    "S": ("disabled", "neutral_bulk_stage3_v1"),
    "LS": ("dilley_unsworth_subcanopy_v1", "neutral_bulk_stage3_v1"),
}
LABEL_SCORE = {"fail": 0, "marginal": 1, "pass": 2, "strong": 3}
NON_TARGET_ENV = {
    "OPENWEPP_SNOWDENSITY1038_MELT_MODEL": "coe_liquid_holding_capacity_v1",
    "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "physics_bulk_multilayer_density_v1",
    "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL": "harder_pomeroy_hourly",
    "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "layered_thermal_liquid_v1",
}
SNOW_THRESHOLD_M = 0.001
PERSISTENCE_DAYS = 7
MASS_TOLERANCE_M = 1.0e-9
ENERGY_TOLERANCE_J_M2 = 1.0e-6
LATENT_MASS_TOLERANCE_J_M2 = 1.0e-6


@dataclass(frozen=True)
class Lane:
    lane_id: str
    corpus: str
    fixture_dir: Path
    observation_file: Path
    observation_filter: dict[str, str]
    stratum: str
    climate: str
    role: str


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workers", type=int, default=4)
    parser.add_argument(
        "--analysis-only",
        action="store_true",
        help="analyze the retained original round without running model cells",
    )
    args = parser.parse_args()
    if not BINARY.is_file():
        raise FileNotFoundError(f"build exact runner first: {BINARY}")
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    FIGURES.mkdir(parents=True, exist_ok=True)
    OUTPUT.mkdir(parents=True, exist_ok=True)

    lanes = fixed_lanes()
    if not args.analysis_only and ATTEMPT.exists():
        raise RuntimeError(
            f"{ATTEMPT} already records the bounded EB-04 round; "
            "only --analysis-only is permitted"
        )
    if args.analysis_only and not ATTEMPT.exists():
        raise FileNotFoundError(f"analysis-only requires {ATTEMPT}")
    attempt = (
        json.loads(ATTEMPT.read_text(encoding="utf-8"))
        if args.analysis_only
        else None
    )
    if not args.analysis_only:
        ATTEMPT.write_text(
            json.dumps(
                {
                    "schema": "snow-surface-eb04-execution-attempt-v1",
                    "status": "STARTED",
                    "started_unix_seconds": time.time(),
                    "command": [
                        ".venv/bin/python",
                        rel(Path(__file__)),
                        "--workers",
                        str(args.workers),
                    ],
                    "protocol_sha256": sha256(PROTOCOL),
                    "bounded_round": 1,
                    "retry_policy": "NO_RETRY",
                },
                indent=2,
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
        )
    tasks = [(lane, cell) for lane in lanes for cell in CELLS]
    results: dict[tuple[str, str], dict[str, Any]] = {}
    with ThreadPoolExecutor(max_workers=args.workers) as executor:
        futures = {
            executor.submit(run_cell, lane, cell, args.analysis_only): (lane, cell)
            for lane, cell in tasks
        }
        for future in as_completed(futures):
            lane, cell = futures[future]
            results[(lane.lane_id, cell)] = future.result()
            print(f"{lane.lane_id}/{cell}: {results[(lane.lane_id, cell)]['execution_status']}")

    report = adjudicate(lanes, results, attempt)
    (ARTIFACTS / "factorial-results.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    write_effects_csv(report)
    write_summary(report)
    if report["execution"]["all_cells_completed"]:
        make_figures(lanes, report)
    else:
        make_failure_figures(report)
    check_artifact_inventory()
    print(json.dumps(report["decision"], indent=2, sort_keys=True))
    return 0


def fixed_lanes() -> list[Lane]:
    lanes = []
    for site in cross.diagnostic_sites():
        stratum = (
            site.observation_filter.get("observed_stratum", "open")
            if site.observation_filter
            else "open"
        )
        lanes.append(
            Lane(
                lane_id=site.site_id,
                corpus=site.corpus,
                fixture_dir=site.fixture_dir,
                observation_file=site.observation_file,
                observation_filter=site.observation_filter,
                stratum=stratum,
                climate=site.snow_climate,
                role="INDEPENDENT_VALIDATION",
            )
        )
    frost_root = REPO / "tests/fixtures/snowfreeze_observed"
    for lane_id, fixture, stratum in [
        ("sleepers_south_open", "site1_sleepers_south_field_vt", "open"),
        ("sleepers_w9_hardwood", "site2_sleepers_w9_hardwood_vt", "hardwood"),
    ]:
        lanes.append(
            Lane(
                lane_id=lane_id,
                corpus="snowfreeze_observed",
                fixture_dir=frost_root / fixture,
                observation_file=frost_root / "observations/sites" / f"{fixture}.csv",
                observation_filter={},
                stratum=stratum,
                climate="humid_cold",
                role="DIAGNOSTIC_ONLY",
            )
        )
    return lanes


def run_cell(lane: Lane, cell: str, analysis_only: bool) -> dict[str, Any]:
    run_dir = OUTPUT / "runs" / lane.lane_id / cell
    run_dir.mkdir(parents=True, exist_ok=True)
    trace = run_dir / f"{lane.lane_id}-{cell}.snow.jsonl"
    runfile = run_dir / f"{lane.lane_id}-{cell}.run"
    run_id = f"{lane.lane_id}-{cell}"
    stem = observed_harness.discover_run_stem(lane.fixture_dir)
    observed_harness.write_runfile(runfile, lane.fixture_dir, stem, run_dir, run_id)
    command = observed_harness.cli_command(
        BINARY, lane.fixture_dir, runfile, run_dir, "direct-production-executor"
    )
    longwave, sublimation = CELLS[cell]
    env = os.environ.copy()
    env.update(NON_TARGET_ENV)
    env.update(
        {
            "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL": longwave,
            "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL": sublimation,
            "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace),
        }
    )
    wat = run_dir / f"{run_id}.wat.parquet"
    stderr_path = run_dir / "stderr.txt"
    if analysis_only and trace.is_file() and stderr_path.is_file():
        stderr_text = stderr_path.read_text(encoding="utf-8")
        returncode = 1 if "runtime surface failure" in stderr_text else 0
    elif analysis_only:
        returncode = 1
    else:
        if trace.exists():
            trace.unlink()
        completed = subprocess.run(
            command,
            cwd=REPO,
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        (run_dir / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
        stderr_path.write_text(completed.stderr, encoding="utf-8")
        returncode = completed.returncode
    if returncode != 0 or not wat.is_file() or not trace.is_file():
        trace_rows = read_jsonl(trace) if trace.is_file() else []
        stderr_text = (
            stderr_path.read_text(encoding="utf-8").strip()
            if stderr_path.is_file()
            else ""
        )
        return {
            "lane_id": lane.lane_id,
            "cell": cell,
            "execution_status": "FAIL",
            "returncode": returncode,
            "run_dir": rel(run_dir),
            "command": [str(value) for value in command],
            "failure": (
                stderr_text.splitlines()[-1]
                if stderr_text
                else "missing expected WAT or trace output"
            ),
            "failure_day_index": (
                int(trace_rows[-1]["day_index"]) + 1 if trace_rows else None
            ),
            "trace": rel(trace) if trace.is_file() else None,
            "trace_sha256": sha256(trace) if trace.is_file() else None,
            "trace_row_count": len(trace_rows),
            "fixture_sha256": tree_sha256(lane.fixture_dir),
            "non_target_environment": NON_TARGET_ENV,
            "target_selectors": {"longwave": longwave, "sublimation": sublimation},
            "physical": partial_physical_audit(trace_rows),
            "state": partial_state_metrics(trace_rows),
            "rubric_profile": None,
            "observation_metrics": None,
            "frost_metrics": None,
        }

    trace_rows = read_jsonl(trace)
    modeled, wat_rows = load_wat(wat)
    observations = load_observations(lane)
    profile = (
        rubric.rubric_profile(observations, modeled, cell)
        if lane.role == "INDEPENDENT_VALIDATION"
        else None
    )
    metrics = (
        rubric.model_metrics(observations, modeled, cell)
        if lane.role == "INDEPENDENT_VALIDATION"
        else None
    )
    physical = physical_audit(trace_rows)
    state = state_metrics(modeled, wat_rows, trace_rows)
    frost = frost_metrics(observations, modeled) if lane.role == "DIAGNOSTIC_ONLY" else None
    return {
        "lane_id": lane.lane_id,
        "cell": cell,
        "execution_status": "PASS",
        "returncode": 0,
        "fixture_sha256": tree_sha256(lane.fixture_dir),
        "non_target_environment": NON_TARGET_ENV,
        "target_selectors": {"longwave": CELLS[cell][0], "sublimation": CELLS[cell][1]},
        "run_dir": rel(run_dir),
        "command": [str(value) for value in command],
        "wat": rel(wat),
        "trace": rel(trace),
        "wat_sha256": sha256(wat),
        "trace_sha256": sha256(trace),
        "trace_row_count": len(trace_rows),
        "physical": physical,
        "state": state,
        "rubric_profile": profile,
        "observation_metrics": metrics,
        "frost_metrics": frost,
    }


def physical_audit(rows: list[dict[str, Any]]) -> dict[str, Any]:
    mass_residuals = [
        row["runtime_swe_before_m"]
        + row["accumulation_m"]
        + row["rain_retained_m"]
        - row["sublimation_m"]
        - row["snowpack_swe_loss_m"]
        - row["runtime_swe_after_m"]
        for row in rows
    ]
    temperatures = [
        float(row[key])
        for row in rows
        for key in (
            "snow_layer_minimum_temperature_after_c",
            "snow_layer_maximum_temperature_after_c",
        )
        if row[key] is not None
    ]
    enabled = [row for row in rows if row["stage3_energy_enabled"]]
    maximum_mass = max((abs(value) for value in mass_residuals), default=0.0)
    maximum_energy = max(
        (abs(row["stage3_energy_closure_residual_j_m2"]) for row in enabled),
        default=0.0,
    )
    maximum_latent = max(
        (
            abs(row["stage3_mass_latent_identity_residual_j_m2"])
            for row in enabled
        ),
        default=0.0,
    )
    cold_content_reconstruction = max(
        (
            abs(
                row["stage3_surface_energy_j_m2"]
                + row["stage3_conduction_energy_j_m2"]
                + row["stage3_latent_refreeze_energy_j_m2"]
                + row["stage3_cold_content_export_j_m2"]
                - (
                    row["stage3_cold_content_before_j_m2"]
                    - row["stage3_cold_content_after_j_m2"]
                )
                - row["stage3_energy_closure_residual_j_m2"]
            )
            for row in enabled
        ),
        default=0.0,
    )
    result = {
        "maximum_abs_mass_residual_m": maximum_mass,
        "maximum_abs_energy_residual_j_m2": maximum_energy,
        "maximum_abs_latent_mass_residual_j_m2": maximum_latent,
        "maximum_abs_published_cold_content_reconstruction_residual_j_m2": (
            cold_content_reconstruction
        ),
        "minimum_temperature_c": min(temperatures, default=None),
        "maximum_temperature_c": max(temperatures, default=None),
        "active_stage3_day_count": len(enabled),
        "total_sublimation_m": sum(row["sublimation_m"] for row in rows),
        "total_snowpack_swe_loss_m": sum(row["snowpack_swe_loss_m"] for row in rows),
        "total_refreeze_m": sum(row["stage3_refrozen_liquid_m"] for row in rows),
        "total_longwave_energy_mj_m2": sum(
            row["stage3_longwave_energy_j_m2"] for row in rows
        )
        / 1.0e6,
        "total_latent_energy_mj_m2": sum(
            row["stage3_latent_energy_j_m2"] for row in rows
        )
        / 1.0e6,
    }
    result["passes"] = (
        maximum_mass <= MASS_TOLERANCE_M
        and maximum_energy <= ENERGY_TOLERANCE_J_M2
        and maximum_latent <= LATENT_MASS_TOLERANCE_J_M2
        and cold_content_reconstruction <= ENERGY_TOLERANCE_J_M2
        and len(enabled) > 0
        and (
            not temperatures
            or (min(temperatures) > -273.15 and max(temperatures) <= 0.0)
        )
    )
    return result


def partial_physical_audit(rows: list[dict[str, Any]]) -> dict[str, Any]:
    if not rows:
        return {"passes": False, "active_stage3_day_count": 0}
    result = physical_audit(rows)
    result["passes"] = False
    result["partial_before_typed_failure"] = True
    return result


def partial_state_metrics(rows: list[dict[str, Any]]) -> dict[str, Any]:
    swe = [float(row["runtime_swe_after_m"]) for row in rows]
    return {
        "mean_swe_m": sum(swe) / len(swe) if swe else None,
        "peak_swe_m": max(swe, default=None),
        "mean_depth_m": (
            sum(float(row["runtime_depth_after_m"]) for row in rows) / len(rows)
            if rows
            else None
        ),
        "final_swe_m": swe[-1] if swe else None,
        "total_sublimation_m": sum(float(row["sublimation_m"]) for row in rows),
        "total_snowpack_swe_loss_m": sum(
            float(row["snowpack_swe_loss_m"]) for row in rows
        ),
        "total_refreeze_m": sum(
            float(row.get("stage3_refrozen_liquid_m", 0.0)) for row in rows
        ),
        "median_disappearance_dowy": None,
        "median_runoff_centroid_dowy": None,
        "partial_before_typed_failure": True,
    }


def state_metrics(
    modeled: dict[dt.date, dict[str, float | None]],
    wat_rows: list[dict[str, Any]],
    trace_rows: list[dict[str, Any]],
) -> dict[str, Any]:
    swe = [
        value["snow_water_m"]
        for value in modeled.values()
        if value["snow_water_m"] is not None
    ]
    depth = [
        value["snow_depth_m"]
        for value in modeled.values()
        if value["snow_depth_m"] is not None
    ]
    disappearance = disappearance_by_water_year(modeled)
    runoff = runoff_timing_by_water_year(wat_rows, disappearance)
    return {
        "mean_swe_m": sum(swe) / len(swe) if swe else None,
        "peak_swe_m": max(swe, default=None),
        "mean_depth_m": sum(depth) / len(depth) if depth else None,
        "final_swe_m": swe[-1] if swe else None,
        "total_sublimation_m": sum(row["sublimation_m"] for row in trace_rows),
        "total_snowpack_swe_loss_m": sum(
            row["snowpack_swe_loss_m"] for row in trace_rows
        ),
        "total_refreeze_m": sum(row["stage3_refrozen_liquid_m"] for row in trace_rows),
        "snow_disappearance": disappearance,
        "median_disappearance_dowy": median(
            [item["day_of_water_year"] for item in disappearance if item["date"]]
        ),
        "runoff_timing": runoff,
        "median_runoff_centroid_dowy": median(
            [item["centroid_day_of_water_year"] for item in runoff if item["centroid_date"]]
        ),
    }


def adjudicate(
    lanes: list[Lane],
    results: dict[tuple[str, str], dict[str, Any]],
    attempt: dict[str, Any] | None,
) -> dict[str, Any]:
    lane_reports = []
    for lane in lanes:
        cells = {cell: results[(lane.lane_id, cell)] for cell in CELLS}
        trace_identity = (
            len({cell["fixture_sha256"] for cell in cells.values() if "fixture_sha256" in cell})
            == 1
            and all(
                cell.get("non_target_environment") == NON_TARGET_ENV
                for cell in cells.values()
            )
        )
        effects = factorial_effects(cells)
        lane_reports.append(
            {
                "lane_id": lane.lane_id,
                "corpus": lane.corpus,
                "climate": lane.climate,
                "stratum": lane.stratum,
                "role": lane.role,
                "fixture": rel(lane.fixture_dir),
                "observation_file": rel(lane.observation_file),
                "observation_sha256": sha256(lane.observation_file),
                "trace_identity_passes": trace_identity,
                "cells": cells,
                "effects": effects,
            }
        )
    independent = [lane for lane in lane_reports if lane["role"] == "INDEPENDENT_VALIDATION"]
    aggregate = {cell: aggregate_rubric(independent, cell) for cell in CELLS}
    new_failures = new_robust_failures(independent)
    ls_scoring_complete = aggregate["LS"]["complete_lane_count"] == len(independent)
    group_scores = protected_group_scores(independent)
    all_completed = all(
        cell["execution_status"] == "PASS"
        for lane in lane_reports
        for cell in lane["cells"].values()
    )
    all_attempted = len(results) == len(lanes) * len(CELLS) and all(
        cell["execution_status"] in {"PASS", "FAIL"}
        for lane in lane_reports
        for cell in lane["cells"].values()
    )
    all_physical = all(
        lane["cells"]["LS"].get("physical", {}).get("passes", False)
        and lane["trace_identity_passes"]
        for lane in lane_reports
    )
    score_improved = (
        aggregate["LS"]["complete_lane_count"] == len(independent)
        and aggregate["B"]["complete_lane_count"] == len(independent)
        and
        aggregate["LS"]["robust_ordinal_score"]
        > aggregate["B"]["robust_ordinal_score"]
    )
    failures_reduced = (
        aggregate["LS"]["complete_lane_count"] == len(independent)
        and
        aggregate["LS"]["robust_fail_count"] < aggregate["B"]["robust_fail_count"]
    )
    protected_not_worse = all(
        values["LS"] is not None
        and values["B"] is not None
        and values["LS"] >= values["B"]
        for values in group_scores.values()
    )
    compensation = compensation_audit(independent)
    mechanism_nonzero = any(
        abs(lane["cells"]["L"]["physical"]["total_longwave_energy_mj_m2"]) > 0.0
        for lane in lane_reports
    ) and any(
        lane["cells"]["S"]["physical"]["total_sublimation_m"] > 0.0
        for lane in lane_reports
    )
    criteria = {
        "all_ls_physical_and_trace_gates_pass": all_physical,
        "robust_ordinal_score_increases": score_improved,
        "robust_fail_count_decreases": failures_reduced,
        "no_new_lane_robust_fail": ls_scoring_complete and not new_failures,
        "protected_group_scores_not_worse": protected_not_worse,
        "mechanism_operands_nonzero": mechanism_nonzero,
        "no_compensating_error_pattern": (
            compensation["evaluated"] and not compensation["detected"]
        ),
        "no_forcing_or_input_mutation": all(
            lane["trace_identity_passes"] for lane in lane_reports
        ),
    }
    outcome = (
        "GO_TO_EB05"
        if all(criteria.values())
        else "CLOSE_NONPROMOTION_PHYSICAL_GATE"
    )
    criterion_status = {
        key: ("PASS" if value else "FAIL") for key, value in criteria.items()
    }
    if not ls_scoring_complete:
        criterion_status["no_new_lane_robust_fail"] = "NOT_ASSESSED"
        criterion_status["no_compensating_error_pattern"] = "NOT_ASSESSED"
        criterion_status["robust_ordinal_score_increases"] = "NOT_ASSESSED"
        criterion_status["robust_fail_count_decreases"] = "NOT_ASSESSED"
        criterion_status["protected_group_scores_not_worse"] = "NOT_ASSESSED"
    return {
        "schema": "snow-surface-eb04-factorial-v1",
        "evidence_class": "Ran",
        "source": {
            "git_head": git_head(),
            "binary": rel(BINARY),
            "binary_sha256": (
                attempt["executed_binary_sha256"]
                if attempt is not None
                else sha256(BINARY)
            ),
            "analysis_binary_sha256": sha256(BINARY),
            "execution_attempt_sha256": sha256(ATTEMPT),
            "source_binding_limitation": (
                attempt.get("source_binding_limitation")
                if attempt is not None
                else None
            ),
            "protocol_sha256": sha256(PROTOCOL),
            "fixed_cells": CELLS,
            "fixed_non_target_environment": NON_TARGET_ENV,
            "snow_threshold_m": SNOW_THRESHOLD_M,
            "persistence_days": PERSISTENCE_DAYS,
        },
        "execution": {
            "lane_count": len(lanes),
            "cell_count": len(results),
            "all_cells_attempted": all_attempted,
            "all_cells_completed": all_completed,
            "all_trace_identity_passes": all(
                lane["trace_identity_passes"] for lane in lane_reports
            ),
        },
        "aggregate_rubric": aggregate,
        "protected_group_scores": group_scores,
        "new_robust_failures": new_failures,
        "compensation_audit": compensation,
        "decision": {
            "criteria": criteria,
            "criterion_status": criterion_status,
            "outcome": outcome,
            "stop_loss_invoked": outcome != "GO_TO_EB05",
            "another_round_authorized": False,
            "warm_maritime_conifer_transfer_claim": "WITHHELD_DATA_LIMITED",
        },
        "lanes": lane_reports,
    }


def factorial_effects(cells: dict[str, dict[str, Any]]) -> dict[str, Any]:
    responses = [
        "mean_swe_m",
        "peak_swe_m",
        "mean_depth_m",
        "total_sublimation_m",
        "total_snowpack_swe_loss_m",
        "total_refreeze_m",
        "median_disappearance_dowy",
        "median_runoff_centroid_dowy",
    ]
    effects = {}
    for response in responses:
        values = {cell: cells[cell].get("state", {}).get(response) for cell in CELLS}
        if any(value is None for value in values.values()) or any(
            cells[cell]["execution_status"] != "PASS" for cell in CELLS
        ):
            effects[response] = {
                "values": values,
                "status": "UNAVAILABLE_INCOMPLETE_FACTORIAL",
            }
            continue
        effects[response] = {
            "values": values,
            "longwave_main": values["L"] - values["B"],
            "sublimation_main": values["S"] - values["B"],
            "combined": values["LS"] - values["B"],
            "interaction": values["LS"] - values["L"] - values["S"] + values["B"],
            "status": "AVAILABLE",
        }
    return effects


def aggregate_rubric(lanes: list[dict[str, Any]], cell: str) -> dict[str, Any]:
    counts: Counter[str] = Counter()
    score = fail_count = available = 0
    for lane in lanes:
        profile = lane["cells"][cell]["rubric_profile"]
        if profile is None:
            continue
        for item in profile["cells"]:
            if not item["forcing_robust"]:
                continue
            label = item["ordinal_label"]
            counts[label] += 1
            if label in LABEL_SCORE:
                score += LABEL_SCORE[label]
                available += 1
                fail_count += int(label == "fail")
    return {
        "forcing_robust_counts_by_label": dict(sorted(counts.items())),
        "robust_available_cell_count": available,
        "robust_fail_count": fail_count,
        "robust_ordinal_score": score,
        "complete_lane_count": sum(
            lane["cells"][cell]["rubric_profile"] is not None for lane in lanes
        ),
    }


def new_robust_failures(lanes: list[dict[str, Any]]) -> list[dict[str, str]]:
    findings = []
    for lane in lanes:
        baseline = robust_labels(lane["cells"]["B"]["rubric_profile"])
        combined_profile = lane["cells"]["LS"]["rubric_profile"]
        if combined_profile is None:
            continue
        combined = robust_labels(combined_profile)
        for cell_id, label in combined.items():
            if label == "fail" and baseline.get(cell_id) != "fail":
                findings.append(
                    {
                        "lane_id": lane["lane_id"],
                        "rubric_cell": cell_id,
                        "baseline": baseline.get(cell_id, "unavailable"),
                        "combined": label,
                    }
                )
    return findings


def protected_group_scores(
    lanes: list[dict[str, Any]],
) -> dict[str, dict[str, int | None]]:
    groups = {
        "open_controls": [lane for lane in lanes if lane["stratum"] == "open"],
        "canopy_strata": [lane for lane in lanes if lane["stratum"] != "open"],
    }
    return {
        group: {
            cell: (
                aggregate_rubric(group_lanes, cell)["robust_ordinal_score"]
                if aggregate_rubric(group_lanes, cell)["complete_lane_count"]
                == len(group_lanes)
                else None
            )
            for cell in CELLS
        }
        for group, group_lanes in groups.items()
    }


def compensation_audit(lanes: list[dict[str, Any]]) -> dict[str, Any]:
    findings = []
    evaluated = all(
        all(
            lane["effects"].get(response, {}).get("status") == "AVAILABLE"
            for response in ("mean_swe_m", "median_disappearance_dowy")
        )
        for lane in lanes
    )
    for lane in lanes:
        for response in ("mean_swe_m", "median_disappearance_dowy"):
            effect = lane["effects"].get(response, {})
            if effect.get("status") != "AVAILABLE":
                continue
            longwave = effect["longwave_main"]
            sublimation = effect["sublimation_main"]
            combined = effect["combined"]
            if longwave * sublimation < 0.0 and abs(combined) < max(
                abs(longwave), abs(sublimation)
            ):
                findings.append(
                    {
                        "lane_id": lane["lane_id"],
                        "response": response,
                        "longwave_main": longwave,
                        "sublimation_main": sublimation,
                        "combined": combined,
                    }
                )
    return {
        "evaluated": evaluated,
        "detected": bool(findings) if evaluated else None,
        "findings": findings,
    }


def robust_labels(profile: dict[str, Any]) -> dict[str, str]:
    return {
        item["cell_id"]: item["ordinal_label"]
        for item in profile["cells"]
        if item["forcing_robust"]
    }


def load_observations(lane: Lane) -> list[dict[str, str]]:
    with lane.observation_file.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    if lane.observation_filter:
        rows = [
            row
            for row in rows
            if all(row.get(key) == value for key, value in lane.observation_filter.items())
        ]
    return rows


def load_wat(
    path: Path,
) -> tuple[dict[dt.date, dict[str, float | None]], list[dict[str, Any]]]:
    import pyarrow.parquet as pq

    modeled = observed_harness.load_modeled_wat(path)
    columns = pq.read_table(path).to_pydict()
    rows = []
    for index in range(len(columns["water_year"])):
        month = int(columns["month"][index])
        water_year = int(columns["water_year"][index])
        year = water_year - 1 if month >= 10 else water_year
        date = dt.date(year, month, int(columns["day_of_month"][index]))
        rows.append(
            {
                "date": date,
                "water_year": water_year,
                "rm_mm": float(columns["RM"][index] or 0.0),
                "snow_water_m": modeled[date]["snow_water_m"],
                "snow_depth_m": modeled[date]["snow_depth_m"],
            }
        )
    return modeled, rows


def disappearance_by_water_year(
    modeled: dict[dt.date, dict[str, float | None]]
) -> list[dict[str, Any]]:
    by_year: dict[int, list[tuple[dt.date, float]]] = {}
    for date, row in modeled.items():
        swe = row["snow_water_m"]
        if swe is None:
            continue
        by_year.setdefault(water_year(date), []).append((date, swe))
    output = []
    for year, rows in sorted(by_year.items()):
        rows.sort()
        last_peak_index = max(range(len(rows)), key=lambda index: rows[index][1])
        found = None
        for index in range(last_peak_index, len(rows) - PERSISTENCE_DAYS + 1):
            window = rows[index : index + PERSISTENCE_DAYS]
            if all(value <= SNOW_THRESHOLD_M for _, value in window):
                found = window[0][0]
                break
        output.append(
            {
                "water_year": year,
                "date": found.isoformat() if found else None,
                "day_of_water_year": day_of_water_year(found) if found else None,
            }
        )
    return output


def runoff_timing_by_water_year(
    rows: list[dict[str, Any]], disappearance: list[dict[str, Any]]
) -> list[dict[str, Any]]:
    end_by_year = {
        item["water_year"]: dt.date.fromisoformat(item["date"])
        for item in disappearance
        if item["date"]
    }
    output = []
    years = sorted({row["water_year"] for row in rows})
    for year in years:
        start = dt.date(year - 1, 10, 1)
        end = end_by_year.get(year, dt.date(year, 9, 30))
        selected = [
            row for row in rows if start <= row["date"] <= end and row["rm_mm"] > 0.0
        ]
        total = sum(row["rm_mm"] for row in selected)
        if total <= 0.0:
            output.append(
                {
                    "water_year": year,
                    "centroid_date": None,
                    "centroid_day_of_water_year": None,
                    "peak_date": None,
                    "peak_day_of_water_year": None,
                }
            )
            continue
        centroid_index = sum(
            (row["date"] - start).days * row["rm_mm"] for row in selected
        ) / total
        centroid = start + dt.timedelta(days=round(centroid_index))
        peak = min(
            (row for row in selected if row["rm_mm"] == max(r["rm_mm"] for r in selected)),
            key=lambda row: row["date"],
        )["date"]
        output.append(
            {
                "water_year": year,
                "centroid_date": centroid.isoformat(),
                "centroid_day_of_water_year": day_of_water_year(centroid),
                "peak_date": peak.isoformat(),
                "peak_day_of_water_year": day_of_water_year(peak),
            }
        )
    return output


def frost_metrics(
    observations: list[dict[str, str]],
    modeled: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    residuals = []
    for row in observations:
        raw = row.get("observed_frost_depth_m", "")
        if not raw:
            continue
        date = dt.date.fromisoformat(row["date"])
        if date not in modeled:
            continue
        residuals.append(modeled[date]["frdp_m"] - float(raw))
    return {
        "paired_count": len(residuals),
        "median_modeled_minus_observed_m": median(residuals),
        "mean_abs_residual_m": (
            sum(abs(value) for value in residuals) / len(residuals)
            if residuals
            else None
        ),
    }


def write_effects_csv(report: dict[str, Any]) -> None:
    path = ARTIFACTS / "factorial-effects.csv"
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "lane_id",
                "role",
                "response",
                "units",
                "status",
                "B",
                "L",
                "S",
                "LS",
                "longwave_main",
                "sublimation_main",
                "combined",
                "interaction",
            ]
        )
        units = {
            "mean_swe_m": "m",
            "peak_swe_m": "m",
            "mean_depth_m": "m",
            "total_sublimation_m": "m",
            "total_snowpack_swe_loss_m": "m",
            "total_refreeze_m": "m",
            "median_disappearance_dowy": "day_of_water_year",
            "median_runoff_centroid_dowy": "day_of_water_year",
        }
        for lane in report["lanes"]:
            for response, effect in lane["effects"].items():
                writer.writerow(
                    [
                        lane["lane_id"],
                        lane["role"],
                        response,
                        units[response],
                        effect["status"],
                        effect["values"]["B"],
                        effect["values"]["L"],
                        effect["values"]["S"],
                        effect["values"]["LS"],
                        effect.get("longwave_main"),
                        effect.get("sublimation_main"),
                        effect.get("combined"),
                        effect.get("interaction"),
                    ]
                )


def write_summary(report: dict[str, Any]) -> None:
    decision = report["decision"]
    aggregate = report["aggregate_rubric"]
    lines = [
        "# EB-04 Scientific Adjudication",
        "",
        "Status: `executed / hold / nonpromotion`",
        "",
        "Evidence mode: `Ran`",
        "",
        f"Decision: `{decision['outcome']}`.",
        "",
        (
            "Package disposition: `HOLD`. The retained trace does not publish "
            "shortwave or the signed per-step latent/mass operands required "
            "for the preregistered independent reconstructions. The frozen "
            "one-round protocol forbids a result-aware rerun to repair those "
            "evidence gaps."
        ),
        "",
        "## Primary Result",
        "",
        (
            f"B completed `{aggregate['B']['complete_lane_count']}` independent "
            f"lanes with ordinal score `{aggregate['B']['robust_ordinal_score']}`. "
            f"LS completed `{aggregate['LS']['complete_lane_count']}` independent "
            "lanes, so no LS observation score or fail-count comparison is "
            "admissible."
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
            "## Claim Limits",
            "",
            "- The warm-maritime conifer transfer claim remains withheld because no paired bound observation lane is installed.",
            "- SNOTEL constrains open-control behavior but cannot identify canopy longwave.",
            "- Sleepers frost comparisons are diagnostic-only.",
            "- No coefficient, forcing, fixture, observation, default, or process equation changed.",
            "",
            "## Stop-Loss",
            "",
            (
                "The single-round stop-loss is invoked; another tuning round is "
                "not authorized from the same evidence."
                if decision["stop_loss_invoked"]
                else "The stop-loss is not invoked; EB-05 may evaluate promotion."
            ),
            "",
        ]
    )
    (ARTIFACTS / "scientific-adjudication.md").write_text(
        "\n".join(lines), encoding="utf-8"
    )


def make_failure_figures(report: dict[str, Any]) -> None:
    import matplotlib

    matplotlib.use("Agg")
    matplotlib.rcParams["svg.hashsalt"] = "snow-surface-eb04"
    import matplotlib.dates as mdates
    import matplotlib.pyplot as plt
    import numpy as np

    failed = [
        (lane, cell_name, cell)
        for lane in report["lanes"]
        for cell_name, cell in lane["cells"].items()
        if cell["execution_status"] == "FAIL" and cell.get("trace")
    ]
    representative_lane, representative_cell, representative = max(
        (item for item in failed if item[1] == "LS"),
        key=lambda item: item[2]["trace_row_count"],
    )
    partial = load_partial_trace_series(REPO / representative["trace"])
    days = np.arange(len(partial["surface_mj_m2"]))

    fig, axes = plt.subplots(2, 1, figsize=(10, 7), sharex=True)
    for key, label, color in [
        ("longwave_mj_m2", "Net longwave", "#7a5195"),
        ("latent_mj_m2", "Latent", "#ef5675"),
        ("conduction_mj_m2", "Active/lower conduction", "#ffa600"),
        ("surface_mj_m2", "Applied surface total", "#003f5c"),
    ]:
        axes[0].plot(days, partial[key], label=label, color=color, alpha=0.85)
    axes[0].axhline(0.0, color="#444444", linewidth=0.8)
    axes[0].set_ylabel("Daily energy (MJ m$^{-2}$)")
    axes[0].set_title(
        f"{representative_lane['lane_id']} {representative_cell}: trace before typed failure"
    )
    axes[0].legend(ncol=2, frameon=False)
    axes[1].plot(days, partial["temperature_c"], color="#003f5c")
    axes[1].set_ylabel("Minimum snow temperature (°C)")
    axes[1].set_xlabel("Simulation day index")
    save_figure(fig, "eb04-energy-components-before-failure")
    write_sidecar(
        "eb04-energy-components-before-failure",
        "What energy-component and thermal chronology preceded the longest-running failed candidate cell?",
        f"{representative_lane['lane_id']} {representative_cell}, all retained trace days before the typed conductivity-path failure.",
        "Daily energy in MJ m^-2 and minimum snow temperature in deg C.",
        "Already-computed Stage 3 daily operands are plotted against simulation day index; no failed day is fabricated.",
        "Deterministic partial execution; it has no sampling interval and no post-failure states.",
        "The failed day and all later dates are excluded because the runtime failed closed.",
        "The trace demonstrates that the candidate reached the real thermal consumer and identifies the chronology available before rejection.",
        "A partial trajectory cannot support an observation score or a full-season mechanism conclusion.",
    )

    fig, axes = plt.subplots(2, 1, figsize=(10, 7), sharex=True)
    axes[0].plot(
        days,
        partial["cumulative_melt_mm"],
        label="Snowpack SWE loss",
        color="#003f5c",
    )
    axes[0].set_ylabel("Cumulative loss (mm)")
    axes[0].legend(frameon=False, loc="upper left")
    axes[0].set_title(
        f"{representative_lane['lane_id']} {representative_cell}: cumulative fluxes before failure"
    )
    axes[1].plot(
        days,
        partial["cumulative_sublimation_mm"],
        label="Sublimation",
        color="#ef5675",
    )
    axes[1].plot(
        days,
        partial["cumulative_refreeze_mm"],
        label="Refreeze",
        color="#ffa600",
    )
    axes[1].set_xlabel("Simulation day index")
    axes[1].set_ylabel("Cumulative water (mm)")
    axes[1].legend(frameon=False, loc="upper left")
    save_figure(fig, "eb04-cumulative-fluxes-before-failure")
    write_sidecar(
        "eb04-cumulative-fluxes-before-failure",
        "How much sublimation, snowpack loss, and refreeze accumulated before the candidate failed closed?",
        f"{representative_lane['lane_id']} {representative_cell}, retained pre-failure trace.",
        "Cumulative mm water equivalent.",
        "Daily trace amounts are cumulatively summed; refreeze remains an internal phase transfer.",
        "The series is truncated by a typed runtime failure and is not comparable to a completed water balance.",
        "No post-failure value, observation, or extrapolation is included.",
        "The plot preserves mechanism magnitude without pretending the failed cell completed.",
        "Totals are lower bounds on a hypothetical completed run and cannot support promotion.",
    )

    lane_names = [lane["lane_id"] for lane in report["lanes"]]
    cell_names = list(CELLS)
    completion = np.array(
        [
            [
                int(lane["cells"][cell]["execution_status"] == "PASS")
                for cell in cell_names
            ]
            for lane in report["lanes"]
        ]
    )
    fig, ax = plt.subplots(figsize=(7, 7))
    image = ax.imshow(completion, cmap="RdYlGn", vmin=0, vmax=1, aspect="auto")
    ax.set_xticks(range(len(cell_names)), cell_names)
    ax.set_yticks(range(len(lane_names)), lane_names)
    ax.set_title("Factorial execution completion")
    fig.colorbar(image, ax=ax, ticks=[0, 1], label="0 failed, 1 completed")
    save_figure(fig, "eb04-factorial-completion")
    write_sidecar(
        "eb04-factorial-completion",
        "Which preregistered B/L/S/LS cells completed the real production consumer?",
        "All 12 fixed lanes and 48 fixed cells.",
        "Binary execution status: 1 completed, 0 typed failure.",
        "Status comes from the original single round; the operator attests that no failed cell was retried.",
        "Execution status is separate from scientific conformance.",
        "No cell is omitted; warning-only stderr does not count as failure.",
        "The matrix makes the asymmetric S/LS failure pattern and two L failures directly visible.",
        "A completed cell may still fail a physical or scientific criterion; completion alone is not promotion evidence.",
    )

    fig, ax = plt.subplots(figsize=(11, 5.5))
    failure_labels = [
        f"{lane['lane_id']} {cell_name}" for lane, cell_name, _ in failed
    ]
    failure_days = [cell["failure_day_index"] for _, _, cell in failed]
    ax.bar(np.arange(len(failed)), failure_days, color="#d62728")
    ax.set_xticks(np.arange(len(failed)), failure_labels, rotation=55, ha="right")
    ax.set_ylabel("First rejected simulation day")
    ax.set_title("Typed failure chronology")
    save_figure(fig, "eb04-failure-chronology")
    write_sidecar(
        "eb04-failure-chronology",
        "When did each failed cell first reach its typed domain rejection?",
        "Every failed cell in the single fixed round.",
        "One-based simulation day index.",
        "The index is one day after the last successfully published trace row.",
        "Fixture start dates differ, so day index supports within-run chronology but not cross-site calendar attribution.",
        "Completed cells are absent because they have no failure day.",
        "Late failures show why short smoke fixtures did not establish population-wide runtime admissibility.",
        "Twenty-two failures use a wrapper that reports layer density as the rejected conductivity-path value; two are prior-layer thickness reconciliation failures. EB-04 does not infer the hidden meteorology sub-error.",
    )

    completed_counts = [
        sum(
            lane["cells"][cell]["execution_status"] == "PASS"
            for lane in report["lanes"]
        )
        for cell in cell_names
    ]
    fig, ax = plt.subplots(figsize=(7, 5))
    ax.bar(cell_names, completed_counts, color=["#555555", "#7a5195", "#ef5675", "#003f5c"])
    ax.axhline(len(report["lanes"]), color="#333333", linestyle="--", linewidth=1.0)
    ax.set_ylabel("Completed lanes (of 12)")
    ax.set_title("Candidate reach across protected lanes")
    save_figure(fig, "eb04-protected-lane-reach")
    write_sidecar(
        "eb04-protected-lane-reach",
        "Did each factorial cell reach all protected open and canopy lanes?",
        "All 12 fixed validation and diagnostic lanes.",
        "Count of completed real-consumer executions.",
        "Counts retain every original execution outcome; the dashed line marks the required 12-lane inventory.",
        "This is execution reach, not an empirical accuracy interval.",
        "No failed cell is counted as completed even when it produced a long partial trace.",
        "B has full reach; candidate mechanisms do not, so protected-lane promotion criteria cannot be evaluated.",
        "Reach does not diagnose the underlying physical cause of the typed failure.",
    )

    marcell = next(lane for lane in report["lanes"] if lane["lane_id"] == "marcell_conifer")
    completed_series = {
        cell: load_plot_series(REPO / marcell["cells"][cell]["wat"], REPO / marcell["cells"][cell]["trace"])
        for cell in ("B", "L")
    }
    selected_year = max(
        {water_year(date) for date in completed_series["B"]["dates"]},
        key=lambda year: max(
            (
                swe
                for date, swe in zip(
                    completed_series["B"]["dates"], completed_series["B"]["swe_m"]
                )
                if water_year(date) == year
            ),
            default=0.0,
        ),
    )
    fig, ax = plt.subplots(figsize=(10, 5.5))
    for cell, color in [("B", "#555555"), ("L", "#7a5195")]:
        series = completed_series[cell]
        indices = [
            index
            for index, date in enumerate(series["dates"])
            if water_year(date) == selected_year
        ]
        ax.plot(
            [series["dates"][index] for index in indices],
            [series["swe_m"][index] * 1000.0 for index in indices],
            label=cell,
            color=color,
        )
    lane_definition = next(lane for lane in fixed_lanes() if lane.lane_id == "marcell_conifer")
    observed_dates, observed_swe = [], []
    for row in load_observations(lane_definition):
        if (
            row.get("observed_swe_mm")
            and water_year(dt.date.fromisoformat(row["date"])) == selected_year
        ):
            observed_dates.append(dt.date.fromisoformat(row["date"]))
            observed_swe.append(float(row["observed_swe_mm"]))
    ax.scatter(observed_dates, observed_swe, label="Observed", color="#d62728", s=24, zorder=4)
    ax.set_ylabel("SWE (mm)")
    ax.set_title(f"Completed Marcell conifer cells, WY{selected_year}")
    ax.xaxis.set_major_formatter(mdates.DateFormatter("%b"))
    ax.legend(frameon=False)
    save_figure(fig, "eb04-completed-observed-simulated-swe")
    write_sidecar(
        "eb04-completed-observed-simulated-swe",
        "What observable seasonal trend can be interpreted from cells that actually completed?",
        f"Marcell conifer B and L plus bound observations, water year {selected_year}.",
        "SWE in mm.",
        "Completed daily WAT states are plotted with exact-date snow-course observations.",
        "Observation spatial variability and fixture-forcing mismatch remain; no uncertainty interval is inferred.",
        "S and LS are excluded because they failed and produced no complete WAT series.",
        "The figure preserves the interpretable completed subset while making no combined-candidate claim.",
        "It cannot substitute for the missing four-cell factorial or support LS promotion.",
    )

    harvard_hardwood = next(
        lane for lane in report["lanes"] if lane["lane_id"] == "harvard_hardwood"
    )
    harvard_open = next(
        lane for lane in report["lanes"] if lane["lane_id"] == "harvard_open"
    )
    fig, axes = plt.subplots(1, 2, figsize=(11, 5), sharey=True)
    for axis, cell, color in [
        (axes[0], "B", "#555555"),
        (axes[1], "L", "#7a5195"),
    ]:
        for lane, style in [(harvard_hardwood, "-"), (harvard_open, "--")]:
            series = load_plot_series(
                REPO / lane["cells"][cell]["wat"], REPO / lane["cells"][cell]["trace"]
            )
            axis.plot(
                series["dates"],
                [value * 1000.0 for value in series["swe_m"]],
                linestyle=style,
                color=color,
                label=lane["stratum"],
            )
        axis.set_title(f"Harvard {cell}")
        axis.legend(frameon=False)
    axes[0].set_ylabel("SWE (mm)")
    save_figure(fig, "eb04-completed-paired-forest-open")
    write_sidecar(
        "eb04-completed-paired-forest-open",
        "How do completed B and L cells express Harvard hardwood-versus-open trajectories?",
        "Full Harvard hardwood and open fixture records.",
        "Daily SWE in mm.",
        "Solid lines are hardwood and dashed lines are open; panels separate B and L.",
        "The fixtures share the Harvard climate construction but retain canopy/landuse and forcing-representativeness limitations.",
        "S and LS are excluded because both Harvard candidate cells failed.",
        "The plot satisfies paired forest/open human interpretation for the completed subset.",
        "It cannot characterize sublimation or the combined interaction after failure.",
    )


def make_figures(lanes: list[Lane], report: dict[str, Any]) -> None:
    import matplotlib

    matplotlib.use("Agg")
    matplotlib.rcParams["svg.hashsalt"] = "snow-surface-eb04"
    import matplotlib.dates as mdates
    import matplotlib.pyplot as plt
    import numpy as np

    representative = next(lane for lane in report["lanes"] if lane["lane_id"] == "marcell_conifer")
    series = {
        cell: load_plot_series(REPO / representative["cells"][cell]["wat"], REPO / representative["cells"][cell]["trace"])
        for cell in CELLS
    }
    water_year_value = max(
        {water_year(date) for date in series["B"]["dates"]},
        key=lambda year: max(
            (
                swe
                for date, swe in zip(series["B"]["dates"], series["B"]["swe_m"])
                if water_year(date) == year
            ),
            default=0.0,
        ),
    )
    mask = [
        index
        for index, date in enumerate(series["B"]["dates"])
        if water_year(date) == water_year_value
    ]

    fig, axes = plt.subplots(2, 1, figsize=(10, 7), sharex=True)
    ls = series["LS"]
    dates = [ls["dates"][index] for index in mask]
    for key, label, color in [
        ("longwave_mj_m2", "Net longwave", "#7a5195"),
        ("latent_mj_m2", "Latent", "#ef5675"),
        ("conduction_mj_m2", "Active/lower conduction", "#ffa600"),
        ("surface_mj_m2", "Applied surface total", "#003f5c"),
    ]:
        axes[0].plot(dates, [ls[key][index] for index in mask], label=label, color=color)
    axes[0].axhline(0.0, color="#444444", linewidth=0.8)
    axes[0].set_ylabel("Daily energy (MJ m$^{-2}$)")
    axes[0].set_title(f"Marcell conifer LS energy components, WY{water_year_value}")
    axes[0].legend(ncol=2, frameon=False)
    for cell, color in zip(CELLS, ["#333333", "#7a5195", "#ef5675", "#003f5c"]):
        axes[1].plot(
            dates,
            [series[cell]["temperature_c"][index] for index in mask],
            label=cell,
            color=color,
        )
    axes[1].set_ylabel("Minimum snow temperature (°C)")
    axes[1].xaxis.set_major_formatter(mdates.DateFormatter("%b"))
    axes[1].legend(ncol=4, frameon=False)
    save_figure(fig, "eb04-energy-components")
    write_sidecar(
        "eb04-energy-components",
        "How do longwave, latent exchange, internal conduction, and the applied surface total evolve through a representative conifer snow season?",
        f"Marcell conifer fixture, LS components and B/L/S/LS temperatures, water year {water_year_value}.",
        "Energy is the signed daily Stage 3 diagnostic total in MJ m^-2; temperature is deg C.",
        "Daily runtime trace totals; positive energy warms the active snow control volume. The lower panel uses the minimum populated snow-layer temperature.",
        "The curves are deterministic model output; forcing and observation representativeness uncertainty is not a plotted confidence interval.",
        "Snow-free days carry zero energy and no temperature. No observed energy-flux series is available.",
        "Longwave and latent exchange are visibly distinct operands feeding the common thermal state.",
        "This is mechanism interpretation, not an observed energy-flux validation.",
    )

    fig, ax = plt.subplots(figsize=(10, 5.5))
    for cell, color in zip(CELLS, ["#333333", "#7a5195", "#ef5675", "#003f5c"]):
        ax.plot(
            dates,
            [series[cell]["swe_m"][index] * 1000.0 for index in mask],
            label=cell,
            color=color,
        )
    observations = load_observations(
        next(lane for lane in lanes if lane.lane_id == "marcell_conifer")
    )
    observed_dates, observed_swe = [], []
    for row in observations:
        if row.get("observed_swe_mm") and water_year(dt.date.fromisoformat(row["date"])) == water_year_value:
            observed_dates.append(dt.date.fromisoformat(row["date"]))
            observed_swe.append(float(row["observed_swe_mm"]))
    ax.scatter(observed_dates, observed_swe, label="Observed", color="#d62728", s=24, zorder=4)
    ax.set_ylabel("SWE (mm)")
    ax.set_title(f"Observed and simulated Marcell conifer SWE, WY{water_year_value}")
    ax.xaxis.set_major_formatter(mdates.DateFormatter("%b"))
    ax.legend(ncol=5, frameon=False)
    save_figure(fig, "eb04-observed-simulated-swe")
    write_sidecar(
        "eb04-observed-simulated-swe",
        "Do the four mechanism cells move seasonal SWE toward or away from observed conifer snow-course values?",
        f"Marcell conifer fixture and bound snow-course observations, water year {water_year_value}.",
        "Snow water equivalent in mm.",
        "Daily WAT states are plotted continuously; normalized observation means appear on exact measurement dates without interpolation.",
        "Observations are spatial snow-course means; climate forcing is fixture-based and not observation-coincident at every point.",
        "Dates without an observed SWE value are omitted.",
        "The separation among B/L/S/LS shows the modeled marginal and combined seasonal response.",
        "One water year is illustrative; the decision uses the full installed record and frozen rubric.",
    )

    fig, axes = plt.subplots(2, 1, figsize=(10, 7), sharex=True)
    for cell, color in zip(CELLS, ["#333333", "#7a5195", "#ef5675", "#003f5c"]):
        axes[0].plot(
            dates,
            [series[cell]["swe_m"][index] * 1000.0 for index in mask],
            label=cell,
            color=color,
        )
    for key, label, color in [
        ("cumulative_sublimation_mm", "Sublimation", "#ef5675"),
        ("cumulative_melt_mm", "Snowpack SWE loss", "#003f5c"),
        ("cumulative_refreeze_mm", "Refreeze", "#ffa600"),
    ]:
        axes[1].plot(
            dates,
            [series["LS"][key][index] for index in mask],
            label=label,
            color=color,
        )
    axes[0].set_ylabel("SWE (mm)")
    axes[0].set_title(f"Snow storage and cumulative fluxes, WY{water_year_value}")
    axes[0].legend(ncol=4, frameon=False)
    axes[1].set_ylabel("Cumulative water (mm)")
    axes[1].xaxis.set_major_formatter(mdates.DateFormatter("%b"))
    axes[1].legend(ncol=3, frameon=False)
    save_figure(fig, "eb04-storage-flux-trajectories")
    write_sidecar(
        "eb04-storage-flux-trajectories",
        "How do the candidate mechanisms alter snow storage while sublimation, snowpack loss, and refreeze accumulate?",
        f"Marcell conifer fixture, B/L/S/LS SWE and LS cumulative fluxes, water year {water_year_value}.",
        "SWE and cumulative fluxes in mm water equivalent.",
        "Daily trace amounts are cumulatively summed within the water year; refreeze is an internal phase transfer and is not treated as an external mass input.",
        "Deterministic model output; no flux observations are installed.",
        "The snowpack-loss series includes the authoritative CoE snow-state loss, not total hillslope runoff.",
        "The paired panels distinguish external vapor loss from internal melt/refreeze evolution.",
        "Cumulative curves establish mechanism magnitude but not empirical correctness by themselves.",
    )

    fig, axes = plt.subplots(1, 2, figsize=(11, 5), sharey=True)
    for axis, forest_id, open_id, title in [
        (axes[0], "marcell_conifer", "marcell_open", "Marcell: conifer vs open"),
        (axes[1], "harvard_hardwood", "harvard_open", "Harvard: hardwood vs open"),
    ]:
        for lane_id, style in [(forest_id, "-"), (open_id, "--")]:
            lane = next(item for item in report["lanes"] if item["lane_id"] == lane_id)
            for cell, color in [("B", "#555555"), ("LS", "#0072B2")]:
                plot = load_plot_series(REPO / lane["cells"][cell]["wat"], REPO / lane["cells"][cell]["trace"])
                axis.plot(
                    plot["dates"],
                    [value * 1000.0 for value in plot["swe_m"]],
                    linestyle=style,
                    color=color,
                    alpha=0.8,
                    label=f"{lane['stratum']} {cell}",
                )
        axis.set_title(title)
        axis.set_xlabel("Date")
        axis.legend(frameon=False, fontsize=8)
    axes[0].set_ylabel("SWE (mm)")
    save_figure(fig, "eb04-paired-forest-open")
    write_sidecar(
        "eb04-paired-forest-open",
        "Do B and LS preserve interpretable canopy-versus-open snow trajectories under each site's shared fixture climate?",
        "Full Marcell conifer/open and Harvard hardwood/open records.",
        "Daily SWE in mm.",
        "Solid lines are canopy strata and dashed lines are open fixtures; gray is B and blue is LS.",
        "Within each site family the climate construction is shared, but local canopy/landuse inputs differ. Observed forcing mismatch remains.",
        "The unbound Harvard hemlock series and unavailable warm-maritime paired observations are excluded.",
        "The figure exposes whether LS shifts canopy and open controls coherently rather than improving only an aggregate score.",
        "Long records compress individual winters; use the machine-readable lane results for exact metrics.",
    )

    independent = [lane for lane in report["lanes"] if lane["role"] == "INDEPENDENT_VALIDATION"]
    lane_names = [lane["lane_id"] for lane in independent]
    x = np.arange(len(lane_names))
    fig, ax = plt.subplots(figsize=(11, 5.5))
    width = 0.24
    for offset, key, label, color in [
        (-width, "longwave_main", "L − B", "#7a5195"),
        (0.0, "sublimation_main", "S − B", "#ef5675"),
        (width, "combined", "LS − B", "#003f5c"),
    ]:
        ax.bar(
            x + offset,
            [lane["effects"]["mean_swe_m"][key] * 1000.0 for lane in independent],
            width,
            label=label,
            color=color,
        )
    ax.axhline(0.0, color="#333333", linewidth=0.8)
    ax.set_xticks(x, lane_names, rotation=35, ha="right")
    ax.set_ylabel("Change in mean SWE (mm)")
    ax.set_title("Marginal and combined mean-SWE effects")
    ax.legend(frameon=False)
    save_figure(fig, "eb04-factorial-main-effects")
    write_sidecar(
        "eb04-factorial-main-effects",
        "How large are the longwave, sublimation, and combined effects on mean SWE in each independent-validation lane?",
        "Five SNOTEL open controls and five bound Harvard/Marcell canopy/open lanes.",
        "Cell-minus-baseline change in record-mean SWE, mm.",
        "Bars use the preregistered factorial contrasts L-B, S-B, and LS-B with identical non-target settings.",
        "No confidence interval is inferred because forcing uncertainty and temporal autocorrelation are not represented as replicate draws.",
        "Sleepers diagnostic-only lanes are excluded.",
        "The sign and heterogeneity show where each mechanism adds or removes modeled snow storage.",
        "A favorable sign is response-dependent; promotion uses the observed rubric and protected gates, not this plot alone.",
    )

    fig, ax = plt.subplots(figsize=(11, 5))
    interaction = [
        lane["effects"]["mean_swe_m"]["interaction"] * 1000.0 for lane in independent
    ]
    colors = ["#d62728" if value < 0 else "#2ca02c" for value in interaction]
    ax.bar(x, interaction, color=colors)
    ax.axhline(0.0, color="#333333", linewidth=0.8)
    ax.set_xticks(x, lane_names, rotation=35, ha="right")
    ax.set_ylabel("Interaction residual (mm mean SWE)")
    ax.set_title("LS − L − S + B interaction")
    save_figure(fig, "eb04-interaction-residual")
    write_sidecar(
        "eb04-interaction-residual",
        "Are longwave and sublimation additive, or does their shared thermal state produce material interaction?",
        "All independent-validation lanes.",
        "Factorial interaction residual in mm record-mean SWE.",
        "Computed exactly as LS-L-S+B from runs sharing forcing and all non-target settings.",
        "The residual is deterministic and has no sampling interval.",
        "Diagnostic-only Sleepers lanes are excluded.",
        "Zero is additive; departures quantify the coupled response through snow temperature and cold content.",
        "Interaction magnitude does not establish which mechanism is empirically correct.",
    )

    cell_order = list(CELLS)
    matrix = np.array(
        [
            [
                lane["cells"][cell]["rubric_profile"]["summary"][
                    "forcing_robust_counts_by_label"
                ].get("fail", 0)
                for cell in cell_order
            ]
            for lane in independent
        ]
    )
    fig, ax = plt.subplots(figsize=(7, 7))
    image = ax.imshow(matrix, cmap="YlOrRd", aspect="auto")
    ax.set_xticks(range(len(cell_order)), cell_order)
    ax.set_yticks(range(len(lane_names)), lane_names)
    ax.set_xlabel("Factorial cell")
    ax.set_title("Forcing-robust failure counts")
    fig.colorbar(image, ax=ax, label="Failure count")
    save_figure(fig, "eb04-protected-contrary-evidence")
    write_sidecar(
        "eb04-protected-contrary-evidence",
        "Does any candidate hide contrary observed-signature evidence behind an aggregate score?",
        "Forcing-robust rubric failures by independent-validation lane and factorial cell.",
        "Count of rubric cells labeled fail.",
        "Counts come from the frozen INV-SNOWFREEZE-050 rubric; unavailable and forcing-limited cells do not count as robust failures.",
        "Observation and forcing representativeness limits remain lane-specific.",
        "Sleepers diagnostic-only lanes and warm-maritime transfer claims are excluded.",
        "Darker cells identify protected-lane contrary evidence that the promotion reduction must retain.",
        "A failure label remains an unresolved observation discrepancy under ADR-0017, not automatic proof of an openWEPP defect.",
    )


def load_plot_series(wat: Path, trace: Path) -> dict[str, Any]:
    modeled, wat_rows = load_wat(wat)
    rows = read_jsonl(trace)
    dates = [row["date"] for row in wat_rows]
    if len(rows) != len(dates):
        raise ValueError(f"trace/WAT row mismatch: {trace} {len(rows)} != {len(dates)}")
    cumulative_sublimation = cumulative([row["sublimation_m"] * 1000.0 for row in rows])
    cumulative_melt = cumulative([row["snowpack_swe_loss_m"] * 1000.0 for row in rows])
    cumulative_refreeze = cumulative([row["stage3_refrozen_liquid_m"] * 1000.0 for row in rows])
    return {
        "dates": dates,
        "swe_m": [modeled[date]["snow_water_m"] or 0.0 for date in dates],
        "surface_mj_m2": [row["stage3_surface_energy_j_m2"] / 1.0e6 for row in rows],
        "longwave_mj_m2": [row["stage3_longwave_energy_j_m2"] / 1.0e6 for row in rows],
        "latent_mj_m2": [row["stage3_latent_energy_j_m2"] / 1.0e6 for row in rows],
        "conduction_mj_m2": [row["stage3_conduction_energy_j_m2"] / 1.0e6 for row in rows],
        "temperature_c": [
            row["snow_layer_minimum_temperature_after_c"]
            if row["snow_layer_minimum_temperature_after_c"] is not None
            else math.nan
            for row in rows
        ],
        "cumulative_sublimation_mm": cumulative_sublimation,
        "cumulative_melt_mm": cumulative_melt,
        "cumulative_refreeze_mm": cumulative_refreeze,
    }


def load_partial_trace_series(trace: Path) -> dict[str, Any]:
    rows = read_jsonl(trace)
    return {
        "surface_mj_m2": [
            row["stage3_surface_energy_j_m2"] / 1.0e6 for row in rows
        ],
        "longwave_mj_m2": [
            row["stage3_longwave_energy_j_m2"] / 1.0e6 for row in rows
        ],
        "latent_mj_m2": [
            row["stage3_latent_energy_j_m2"] / 1.0e6 for row in rows
        ],
        "conduction_mj_m2": [
            row["stage3_conduction_energy_j_m2"] / 1.0e6 for row in rows
        ],
        "temperature_c": [
            row["snow_layer_minimum_temperature_after_c"]
            if row["snow_layer_minimum_temperature_after_c"] is not None
            else math.nan
            for row in rows
        ],
        "cumulative_sublimation_mm": cumulative(
            [row["sublimation_m"] * 1000.0 for row in rows]
        ),
        "cumulative_melt_mm": cumulative(
            [row["snowpack_swe_loss_m"] * 1000.0 for row in rows]
        ),
        "cumulative_refreeze_mm": cumulative(
            [row["stage3_refrozen_liquid_m"] * 1000.0 for row in rows]
        ),
    }


def save_figure(fig: Any, stem: str) -> None:
    path = FIGURES / f"{stem}.svg"
    fig.tight_layout()
    fig.savefig(path, format="svg", metadata={"Date": None})
    import matplotlib.pyplot as plt

    plt.close(fig)


def write_sidecar(
    stem: str,
    question: str,
    population: str,
    units: str,
    processing: str,
    uncertainty: str,
    exclusions: str,
    interpretation: str,
    limitation: str,
) -> None:
    payload = f"""# {stem}

Status: `complete`

Evidence mode: `Ran`

Figure: [`{stem}.svg`]({stem}.svg)

## Caption

{question} {population} {units}

## Question

{question}

## Population

{population}

## Units

{units}

## Processing

{processing}

## Uncertainty

{uncertainty}

## Exclusions

{exclusions}

## Interpretation

{interpretation}

## Limitation

{limitation}
"""
    (FIGURES / f"{stem}.md").write_text(payload, encoding="utf-8")


def check_artifact_inventory() -> None:
    svgs = {path.stem for path in FIGURES.glob("*.svg")}
    sidecars = {path.stem for path in FIGURES.glob("*.md")}
    if svgs != sidecars or not svgs:
        raise ValueError(f"figure/sidecar mismatch: SVG={svgs}, Markdown={sidecars}")


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [
        json.loads(line)
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]


def tree_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    for item in sorted(candidate for candidate in path.rglob("*") if candidate.is_file()):
        digest.update(str(item.relative_to(path)).encode())
        digest.update(item.read_bytes())
    return digest.hexdigest()


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git_head() -> str:
    return subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO,
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    ).stdout.strip()


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(REPO))
    except ValueError:
        return str(path.resolve())


def water_year(date: dt.date) -> int:
    return date.year + 1 if date.month >= 10 else date.year


def day_of_water_year(date: dt.date | None) -> int | None:
    if date is None:
        return None
    start = dt.date(water_year(date) - 1, 10, 1)
    return (date - start).days + 1


def median(values: list[float | int]) -> float | None:
    if not values:
        return None
    ordered = sorted(float(value) for value in values)
    middle = len(ordered) // 2
    if len(ordered) % 2:
        return ordered[middle]
    return (ordered[middle - 1] + ordered[middle]) / 2.0


def cumulative(values: list[float]) -> list[float]:
    total = 0.0
    output = []
    for value in values:
        total += value
        output.append(total)
    return output


if __name__ == "__main__":
    raise SystemExit(main())
