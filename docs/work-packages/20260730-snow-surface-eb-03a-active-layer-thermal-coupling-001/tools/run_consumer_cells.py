#!/usr/bin/env python3
"""Exercise EB-03A B/L/S/LS through the real direct-production hillslope runner."""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FIXTURE = REPO / "tests/fixtures/cancov_forest/hjandrews_conifer_or"
BINARY = REPO / "target/debug/openwepp-cli-hill"
OUTPUT = REPO / "target/snow_surface_eb03a_consumer_cells"
ARTIFACT = PACKAGE / "artifacts/consumer-cells.json"
sys.path.insert(0, str(REPO / "tools/snowfreeze_observed"))

import observed_harness  # noqa: E402

CELLS = {
    "B_absent": (None, None),
    "B_empty": ("", ""),
    "B": ("disabled", "disabled"),
    "L": ("dilley_unsworth_subcanopy_v1", "disabled"),
    "S": ("disabled", "neutral_bulk_stage3_v1"),
    "LS": ("dilley_unsworth_subcanopy_v1", "neutral_bulk_stage3_v1"),
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def peak_g0_reconstruction_residual(row: dict[str, object]) -> float | None:
    requested = float(row["stage3_peak_g0_requested_w_m2"])
    if requested == 0.0:
        return None
    active_depth = float(row["stage3_peak_g0_active_depth_m"])
    lower_depth = float(row["stage3_peak_g0_lower_depth_m"])
    active_conductivity = float(
        row["stage3_peak_g0_active_conductivity_w_m_k"]
    )
    lower_conductivity = float(
        row["stage3_peak_g0_lower_conductivity_w_m_k"]
    )
    active_temperature = float(row["stage3_peak_g0_active_temperature_c"])
    lower_temperature = float(row["stage3_peak_g0_lower_temperature_c"])
    reconstructed = (
        2.0
        * active_conductivity
        * lower_conductivity
        * (lower_temperature - active_temperature)
        / (
            lower_conductivity * active_depth
            + active_conductivity * lower_depth
        )
    )
    return abs(requested - reconstructed)


def main() -> int:
    if not BINARY.is_file():
        raise FileNotFoundError(f"build the direct runner first: {BINARY}")
    run_stem = observed_harness.discover_run_stem(FIXTURE)
    results = {}
    for cell, (longwave, sublimation) in CELLS.items():
        run_dir = OUTPUT / cell
        run_dir.mkdir(parents=True, exist_ok=True)
        runfile = run_dir / f"eb03a-{cell}.run"
        trace = run_dir / f"eb03a-{cell}.snow.jsonl"
        if trace.exists():
            trace.unlink()
        observed_harness.write_runfile(
            runfile, FIXTURE, run_stem, run_dir, f"eb03a-{cell}"
        )
        command = observed_harness.cli_command(
            BINARY, FIXTURE, runfile, run_dir, "direct-production-executor"
        )
        environment = os.environ.copy()
        environment.update(
            {
                "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": (
                    "physics_bulk_multilayer_density_v1"
                ),
                "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": (
                    "layered_thermal_liquid_v1"
                ),
                "OPENWEPP_R7H_SNOW_TRACE_PATH": str(trace),
            }
        )
        for name, value in [
            ("OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL", longwave),
            ("OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL", sublimation),
        ]:
            if value is None:
                environment.pop(name, None)
            else:
                environment[name] = value
        completed = subprocess.run(
            command,
            cwd=REPO,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        (run_dir / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
        (run_dir / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
        wat = run_dir / f"eb03a-{cell}.wat.parquet"
        rows = [
            json.loads(line)
            for line in trace.read_text(encoding="utf-8").splitlines()
            if line.strip()
        ]
        snow_temperatures = [
            float(row[key])
            for row in rows
            for key in (
                "snow_layer_minimum_temperature_after_c",
                "snow_layer_maximum_temperature_after_c",
            )
            if row[key] is not None
        ]
        energy_residuals = [
            abs(float(row["stage3_energy_closure_residual_j_m2"]))
            for row in rows
            if row["stage3_energy_enabled"]
        ]
        stage3_rows = [row for row in rows if row["stage3_energy_enabled"]]
        peak_rows = [
            row
            for row in stage3_rows
            if float(row["stage3_peak_g0_requested_w_m2"]) != 0.0
        ]
        peak_reconstruction_residuals = [
            residual
            for row in peak_rows
            if (residual := peak_g0_reconstruction_residual(row)) is not None
        ]
        peak_carrier_residuals = [
            abs(
                float(row["stage3_peak_g0_requested_w_m2"])
                - float(row["stage3_peak_g0_w_m2"])
                - float(row["stage3_peak_g0_rejected_w_m2"])
            )
            for row in peak_rows
        ]
        peak_resistance_residuals = [
            max(
                abs(
                    float(row["stage3_peak_g0_active_resistance_m2_k_w"])
                    - float(row["stage3_peak_g0_active_depth_m"])
                    / float(
                        row["stage3_peak_g0_active_conductivity_w_m_k"]
                    )
                ),
                abs(
                    float(row["stage3_peak_g0_lower_resistance_m2_k_w"])
                    - float(row["stage3_peak_g0_lower_depth_m"])
                    / float(
                        row["stage3_peak_g0_lower_conductivity_w_m_k"]
                    )
                ),
            )
            for row in peak_rows
        ]
        results[cell] = {
            "longwave_selector": longwave,
            "sublimation_selector": sublimation,
            "command": [str(value) for value in command],
            "exit_code": completed.returncode,
            "failure": completed.stderr.strip() if completed.returncode != 0 else None,
            "wat_sha256": sha256(wat) if wat.is_file() else None,
            "trace_sha256": sha256(trace),
            "trace_row_count": len(rows),
            "active_snow_coupling_row_count": sum(
                bool(row["active_snow_coupling"]) for row in rows
            ),
            "sublimation_m_sum": sum(float(row["sublimation_m"]) for row in rows),
            "final_runtime_swe_m": float(rows[-1]["runtime_swe_after_m"]),
            "minimum_snow_temperature_c": min(snow_temperatures, default=None),
            "maximum_snow_temperature_c": max(snow_temperatures, default=None),
            "maximum_stage3_energy_residual_j_m2": max(
                energy_residuals, default=0.0
            ),
            "maximum_active_layer_depth_m": max(
                (
                    float(row["stage3_maximum_active_depth_m"])
                    for row in stage3_rows
                ),
                default=0.0,
            ),
            "maximum_lower_layer_depth_m": max(
                (float(row["stage3_maximum_lower_depth_m"]) for row in stage3_rows),
                default=0.0,
            ),
            "maximum_abs_g0_w_m2": max(
                (float(row["stage3_maximum_abs_g0_w_m2"]) for row in stage3_rows),
                default=0.0,
            ),
            "peak_g0_row_count": len(peak_rows),
            "maximum_peak_g0_reconstruction_residual_w_m2": max(
                peak_reconstruction_residuals, default=0.0
            ),
            "maximum_peak_g0_carrier_residual_w_m2": max(
                peak_carrier_residuals, default=0.0
            ),
            "maximum_peak_g0_resistance_residual_m2_k_w": max(
                peak_resistance_residuals, default=0.0
            ),
            "minimum_peak_g0_pressure_pa": min(
                (
                    float(row["stage3_peak_g0_pressure_pa"])
                    for row in peak_rows
                ),
                default=None,
            ),
            "minimum_substep_seconds": min(
                (
                    float(row["stage3_minimum_substep_seconds"])
                    for row in stage3_rows
                    if float(row["stage3_minimum_substep_seconds"]) > 0.0
                ),
                default=None,
            ),
            "maximum_active_energy_residual_j_m2": max(
                (
                    float(row["stage3_maximum_active_energy_residual_j_m2"])
                    for row in stage3_rows
                ),
                default=0.0,
            ),
            "maximum_lower_energy_residual_j_m2": max(
                (
                    float(row["stage3_maximum_lower_energy_residual_j_m2"])
                    for row in stage3_rows
                ),
                default=0.0,
            ),
            "maximum_conduction_cancellation_residual_j_m2": max(
                (
                    float(
                        row[
                            "stage3_maximum_conduction_cancellation_residual_j_m2"
                        ]
                    )
                    for row in stage3_rows
                ),
                default=0.0,
            ),
        }
    payload = {
        "schema": "snow-surface-eb03a-real-consumer-cells-v1",
        "evidence_class": "Ran",
        "fixture": str(FIXTURE.relative_to(REPO)),
        "binary": str(BINARY.relative_to(REPO)),
        "protected_controls": {
            "density_model": "physics_bulk_multilayer_density_v1",
            "stage3_liquid_model": "layered_thermal_liquid_v1",
            "default_activation_changed": False,
            "fixture_inputs_changed": False,
        },
        "cells": results,
        "rollback_identity": {
            "absent_matches_explicit_disabled_wat": (
                results["B_absent"]["wat_sha256"] == results["B"]["wat_sha256"]
            ),
            "empty_matches_explicit_disabled_wat": (
                results["B_empty"]["wat_sha256"] == results["B"]["wat_sha256"]
            ),
            "absent_matches_explicit_disabled_trace": (
                results["B_absent"]["trace_sha256"] == results["B"]["trace_sha256"]
            ),
            "empty_matches_explicit_disabled_trace": (
                results["B_empty"]["trace_sha256"] == results["B"]["trace_sha256"]
            ),
        },
    }
    payload["disposition"] = (
        "PASS"
        if all(payload["rollback_identity"].values())
        and all(cell["exit_code"] == 0 for cell in results.values())
        and all(
            cell["minimum_snow_temperature_c"] is None
            or cell["minimum_snow_temperature_c"] > -273.15
            for cell in results.values()
        )
        and all(
            cell["maximum_snow_temperature_c"] is None
            or cell["maximum_snow_temperature_c"] <= 0.0
            for cell in results.values()
        )
        and all(
            cell["maximum_stage3_energy_residual_j_m2"] <= 1.0e-6
            for cell in results.values()
        )
        and all(
            cell["active_snow_coupling_row_count"] > 0
            and cell["maximum_active_layer_depth_m"] > 0.0
            and cell["maximum_lower_layer_depth_m"] > 0.0
            and cell["maximum_abs_g0_w_m2"] > 0.0
            and cell["peak_g0_row_count"] > 0
            and cell["minimum_peak_g0_pressure_pa"] is not None
            and cell["minimum_peak_g0_pressure_pa"] > 0.0
            and cell["maximum_peak_g0_reconstruction_residual_w_m2"] <= 1.0e-10
            and cell["maximum_peak_g0_carrier_residual_w_m2"] <= 1.0e-12
            and (
                cell["maximum_peak_g0_resistance_residual_m2_k_w"]
                <= 1.0e-12
            )
            and
            cell["maximum_active_layer_depth_m"] <= 0.25 + 1.0e-12
            and cell["maximum_active_energy_residual_j_m2"] <= 1.0e-6
            and cell["maximum_lower_energy_residual_j_m2"] <= 1.0e-6
            and cell["maximum_conduction_cancellation_residual_j_m2"] <= 1.0e-12
            and (
                cell["minimum_substep_seconds"] is None
                or cell["minimum_substep_seconds"] in (60.0, 900.0, 3_600.0)
            )
            for cell in results.values()
        )
        else "HOLD"
    )
    ARTIFACT.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(json.dumps(payload, indent=2, sort_keys=True))
    return 0 if payload["disposition"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
