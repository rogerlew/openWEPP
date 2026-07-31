#!/usr/bin/env python3
"""Exercise EB-03 B/L/S/LS through the real direct-production hillslope runner."""

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
OUTPUT = REPO / "target/snow_surface_eb03_consumer_cells"
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


def main() -> int:
    if not BINARY.is_file():
        raise FileNotFoundError(f"build the direct runner first: {BINARY}")
    run_stem = observed_harness.discover_run_stem(FIXTURE)
    results = {}
    for cell, (longwave, sublimation) in CELLS.items():
        run_dir = OUTPUT / cell
        run_dir.mkdir(parents=True, exist_ok=True)
        runfile = run_dir / f"eb03-{cell}.run"
        trace = run_dir / f"eb03-{cell}.snow.jsonl"
        if trace.exists():
            trace.unlink()
        observed_harness.write_runfile(runfile, FIXTURE, run_stem, run_dir, f"eb03-{cell}")
        command = observed_harness.cli_command(
            BINARY, FIXTURE, runfile, run_dir, "direct-production-executor"
        )
        environment = os.environ.copy()
        environment.update(
            {
                "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL": "physics_bulk_multilayer_density_v1",
                "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL": "layered_thermal_liquid_v1",
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
        wat = run_dir / f"eb03-{cell}.wat.parquet"
        rows = [
            json.loads(line)
            for line in trace.read_text(encoding="utf-8").splitlines()
            if line.strip()
        ]
        results[cell] = {
            "longwave_selector": longwave,
            "sublimation_selector": sublimation,
            "command": [str(value) for value in command],
            "exit_code": completed.returncode,
            "failure": (
                completed.stderr.strip() if completed.returncode != 0 else None
            ),
            "wat_sha256": sha256(wat) if wat.is_file() else None,
            "trace_sha256": sha256(trace),
            "trace_row_count": len(rows),
            "active_snow_coupling_row_count": sum(
                bool(row["active_snow_coupling"]) for row in rows
            ),
            "sublimation_m_sum": sum(float(row["sublimation_m"]) for row in rows),
            "final_runtime_swe_m": float(rows[-1]["runtime_swe_after_m"]),
        }
    payload = {
        "schema": "snow-surface-eb03-real-consumer-cells-v1",
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
        else "HOLD"
    )
    ARTIFACT.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(payload, indent=2, sort_keys=True))
    return 0 if payload["disposition"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
