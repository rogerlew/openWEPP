#!/usr/bin/env python3
"""Execute the prospectively frozen legacy-RST sensitivity extension."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True
REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
BASE_TOOL = PACKAGE / "tools/run_experiment.py"
FREEZE = ARTIFACTS / "legacy-rst-extension-freeze.json"
RECEIPT = ARTIFACTS / "legacy-rst-extension-receipt.json"
RESULTS = ARTIFACTS / "legacy-rst-extension-results.json"
OUTPUT = REPO / "target/snowbird_rst_prepeak_flux_legacy_rst_extension"
RUNS = OUTPUT / "runs"
FIXTURES = OUTPUT / "fixtures"
DETAIL = OUTPUT / "annual-flux-ledger.json"


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


base = load_module("rst_flux_base", BASE_TOOL)
base.OUTPUT = OUTPUT
base.RUNS = RUNS
base.FIXTURES = FIXTURES
base.DETAIL = DETAIL


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def execute_cell(lane: Any, rst: float, fixture: Path) -> dict[str, Any]:
    cell = base.variant_id(rst)
    run_dir = RUNS / lane.lane_id / cell
    run_dir.mkdir(parents=True)
    stem = f"{lane.lane_id}-{cell}"
    runfile = run_dir / f"{stem}.run"
    trace = run_dir / f"{stem}.snow.jsonl"
    fixture_stem = base.w1.eb04r.legacy.observed_harness.discover_run_stem(fixture)
    base.w1.eb04r.legacy.observed_harness.write_runfile(
        runfile, fixture, fixture_stem, run_dir, stem
    )
    command = base.w1.eb04r.legacy.observed_harness.cli_command(
        base.OPENWEPP, fixture, runfile, run_dir, "direct-production-executor"
    )
    environment, removed, effective = base.w1.eb04r.sanitized_environment(
        os.environ, "B", trace
    )
    environment["OPENWEPP_SNOWDENSITY1035_PHASE_MODEL"] = "legacy_rst"
    effective["OPENWEPP_SNOWDENSITY1035_PHASE_MODEL"] = "legacy_rst"
    completed = subprocess.run(
        command, cwd=REPO, env=environment, text=True, capture_output=True
    )
    (run_dir / "stdout.txt").write_text(completed.stdout)
    (run_dir / "stderr.txt").write_text(completed.stderr)
    if completed.returncode != 0:
        raise RuntimeError(
            f"openWEPP failed for {lane.lane_id}/{cell}: {completed.stderr[-2000:]}"
        )
    manifest = json.loads((run_dir / "openwepp_hillslope_run_manifest.json").read_text())
    manifest_rst = float(manifest["coupling_vectors"]["winter"]["rst"])
    if abs(manifest_rst - rst) > 1.0e-12:
        raise RuntimeError(f"manifest rst mismatch: {manifest_rst} != {rst}")
    return {
        "argv": [str(x) for x in command],
        "effective_openwepp_environment": effective,
        "manifest_rst_c": manifest_rst,
        "removed_openwepp_keys": removed,
        "returncode": 0,
        "snow_txt_sha256": sha256(fixture / "snow.txt"),
        "trace_sha256": sha256(trace),
        "wat_sha256": sha256(run_dir / f"{stem}.wat.parquet"),
    }


def execute() -> None:
    if RECEIPT.exists() or OUTPUT.exists():
        raise RuntimeError("refusing to overwrite result-bearing extension evidence")
    freeze = json.loads(FREEZE.read_text())
    bindings = {
        Path(__file__): freeze["extension_tool_sha256"],
        BASE_TOOL: freeze["base_tool_sha256"],
        base.OPENWEPP: freeze["openwepp_binary_sha256"],
        ARTIFACTS / "experiment-freeze.json": freeze["base_freeze_sha256"],
        ARTIFACTS / "execution-receipt.json": freeze["base_receipt_sha256"],
        ARTIFACTS / "response-results.json": freeze["base_results_sha256"],
    }
    for path, expected in bindings.items():
        if sha256(path) != expected:
            raise RuntimeError(f"binding mismatch: {path}")
    lanes = base.w1.selected_lanes()
    executed: dict[str, Any] = {}
    details: dict[str, Any] = {}
    summaries: dict[str, Any] = {}
    for lane in lanes:
        executed[lane.lane_id] = {}
        details[lane.lane_id] = {}
        summaries[lane.lane_id] = {}
        for rst in base.RST_VALUES:
            fixture = base.prepare_fixture(lane, rst)
            executed[lane.lane_id][base.variant_id(rst)] = execute_cell(lane, rst, fixture)
            annual, summary = base.analyze_cell(lane, rst)
            details[lane.lane_id][base.variant_id(rst)] = annual
            summaries[lane.lane_id][base.variant_id(rst)] = summary
            print("EXECUTED_LEGACY_RST", lane.lane_id, rst)
    write_json(DETAIL, {"schema_version": 1, "phase_model": "legacy_rst", "sites": details})
    write_json(RECEIPT, {
        "schema_version": 1,
        "freeze_sha256": sha256(FREEZE),
        "phase_model": "legacy_rst",
        "cell_count": len(lanes) * len(base.RST_VALUES),
        "annual_ledger_path": relative(DETAIL),
        "annual_ledger_sha256": sha256(DETAIL),
        "executed": executed,
    })
    for site in summaries.values():
        baseline = site[base.variant_id(0.0)]
        for rst in base.RST_VALUES:
            row = site[base.variant_id(rst)]
            row["delta_peak_swe_ratio_vs_rst_0"] = row["median_peak_swe_ratio"] - baseline["median_peak_swe_ratio"]
            row["delta_accumulation_m_vs_rst_0"] = row["median_accumulation_m"] - baseline["median_accumulation_m"]
            row["delta_snowpack_loss_m_vs_rst_0"] = row["median_snowpack_swe_loss_m"] - baseline["median_snowpack_swe_loss_m"]
            if row["maximum_daily_mass_closure_m"] > 1.0e-12 or row["maximum_window_mass_closure_m"] > 1.0e-11:
                raise RuntimeError(f"mass closure exceeded for {row}")
    write_json(RESULTS, {
        "schema_version": 1,
        "freeze_sha256": sha256(FREEZE),
        "receipt_sha256": sha256(RECEIPT),
        "phase_model": "legacy_rst",
        "stress_range_disposition": "rst above 1 C is ASSUMED_FOR_EXECUTION diagnostic stress only",
        "sites": summaries,
    })
    print("LEGACY_RST_EXTENSION_COMPLETE", len(lanes) * len(base.RST_VALUES))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    if not args.execute:
        raise RuntimeError("--execute is required")
    execute()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
