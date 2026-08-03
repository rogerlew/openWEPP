#!/usr/bin/env python3
"""Generate SNOTEL-conditioned CLIGEN climates and execute Snowbird snowbench."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import importlib.util
import json
import math
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True
REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FREEZE = ARTIFACTS / "experiment-freeze.json"
RECEIPT = ARTIFACTS / "execution-receipt.json"
RESULTS = ARTIFACTS / "response-results.json"
OUTPUT = REPO / "target/snowbird_snotel_cligen_swe_response"
GENERATED = OUTPUT / "generated"
FIXTURES = OUTPUT / "fixtures"
RUNS = OUTPUT / "runs"
SOURCE_FIXTURE = REPO / "tests/fixtures/snotel_observed/snotel_snowbird_ut"
SOURCE_CLI = SOURCE_FIXTURE / "p8.cli"
OBSERVATION = REPO / "tests/fixtures/snotel_observed/observations/sites/snotel_snowbird_ut.csv"
CLIGEN = Path("/home/workdir/cligen-rs/target/release/cligen")
STATION = Path("/home/workdir/cligen-rs/target/public-sync-check/stations/us-2015/2026.07/ut420072.par")
OPENWEPP = REPO / "target/release/openwepp-cli-hill"
W1_TOOL = REPO / "docs/work-packages/20260802-snow-surface-eb-04w1-precipitation-scaling-calibration-001/tools/run_precipitation_scaling.py"
EB04W_TOOL = REPO / "docs/work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001/tools/run_accumulation_diagnostics.py"
EB04R_TOOL = REPO / "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-execution-adjudication-001/tools/run_experiment.py"
START = dt.date(1990, 1, 1)
END = dt.date(2024, 12, 31)
VARIANTS = ("original_fixture", "cligen_control", "snotel_p", "snotel_t", "snotel_pt")


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


w1 = load_module("snowbird_snotel_w1", W1_TOOL)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def load_cli() -> tuple[list[str], dict[dt.date, list[str]]]:
    lines = SOURCE_CLI.read_text().splitlines()
    rows: dict[dt.date, list[str]] = {}
    for line in lines:
        fields = line.split()
        if len(fields) != 13:
            continue
        try:
            stamp = dt.date(int(fields[2]), int(fields[1]), int(fields[0]))
            list(map(float, fields[3:]))
        except ValueError:
            continue
        if stamp in rows:
            raise RuntimeError(f"duplicate CLI date {stamp}")
        rows[stamp] = fields
    expected = (END - START).days + 1
    selected = {d: row for d, row in rows.items() if START <= d <= END}
    if len(selected) != expected or list(selected) != sorted(selected):
        raise RuntimeError("source CLI does not contain the complete frozen period")
    header_end = next(i for i, line in enumerate(lines) if line.split() == rows[min(rows)])
    return lines[:header_end], selected


def load_snotel() -> dict[dt.date, dict[str, float | int | None]]:
    rows: dict[dt.date, dict[str, float | int | None]] = {}
    with OBSERVATION.open(newline="") as handle:
        for raw in csv.DictReader(handle):
            stamp = dt.date.fromisoformat(raw["date"])
            def value(name: str) -> float | None:
                return float(raw[name]) if raw[name] else None
            rows[stamp] = {
                "water_year": int(raw["water_year"]),
                "cumulative_mm": value("observed_precip_mm"),
                "tmax_c": value("observed_tmax_c"),
                "tmin_c": value("observed_tmin_c"),
                "increment_mm": None,
            }
    previous_date: dt.date | None = None
    previous: dict[str, float | int | None] | None = None
    for stamp, row in rows.items():
        if previous_date is not None and previous is not None:
            current = row["cumulative_mm"]
            prior = previous["cumulative_mm"]
            if (
                stamp - previous_date == dt.timedelta(days=1)
                and row["water_year"] == previous["water_year"]
                and isinstance(current, float)
                and isinstance(prior, float)
            ):
                delta = current - prior
                if delta < -1.0e-9:
                    raise RuntimeError(f"negative SNOTEL cumulative delta on {stamp}")
                row["increment_mm"] = max(0.0, delta)
        previous_date, previous = stamp, row
    return rows


def prn_integer(value: float) -> int:
    return math.floor(value + 0.5) if value >= 0.0 else math.ceil(value - 0.5)


def write_prn(variant: str, source: dict[dt.date, list[str]], observed: dict[dt.date, dict[str, float | int | None]]) -> dict[str, Any]:
    path = GENERATED / f"{variant}.prn"
    lines = []
    p_sub = t_sub = 0
    for stamp, fields in source.items():
        precip_mm = float(fields[3])
        tmax_c = float(fields[7])
        tmin_c = float(fields[8])
        obs = observed.get(stamp)
        if variant in {"snotel_p", "snotel_pt"} and obs is not None and isinstance(obs["increment_mm"], float):
            precip_mm = float(obs["increment_mm"])
            p_sub += 1
        if (
            variant in {"snotel_t", "snotel_pt"}
            and obs is not None
            and isinstance(obs["tmax_c"], float)
            and isinstance(obs["tmin_c"], float)
            and float(obs["tmax_c"]) >= float(obs["tmin_c"])
        ):
            tmax_c = float(obs["tmax_c"])
            tmin_c = float(obs["tmin_c"])
            t_sub += 1
        precip = prn_integer(precip_mm / 0.254)
        tmax = prn_integer(tmax_c * 9.0 / 5.0 + 32.0)
        tmin = prn_integer(tmin_c * 9.0 / 5.0 + 32.0)
        lines.append(f"{stamp.month:<5}{stamp.day:<5}{stamp.year:<5}{precip:<5}{tmax:<5}{tmin:<5}")
    path.write_text("\n".join(lines) + "\n")
    return {"path": relative(path), "sha256": sha256(path), "rows": len(lines), "precipitation_substitutions": p_sub, "temperature_substitutions": t_sub}


def write_runspec(variant: str, prn: Path) -> Path:
    path = GENERATED / f"{variant}.yaml"
    cli = GENERATED / f"{variant}.cli"
    path.write_text(
        "cligen_runspec: 1\n"
        f"station: {{ par: {STATION} }}\n"
        "mode: observed\n"
        "simulation: { begin_year: 1990, years: 35, interpolation: monthly_mean_preserving }\n"
        "rng: { burn: 0 }\n"
        f"observed: {{ prn: {prn} }}\n"
        f"output: {{ cli: {cli}, quality: true }}\n"
    )
    return path


def make_cli(variant: str, header: list[str], source: dict[dt.date, list[str]], observed: dict[dt.date, dict[str, float | int | None]]) -> dict[str, Any]:
    if variant == "original_fixture":
        cli = GENERATED / f"{variant}.cli"
        body = [" ".join(source[d]) for d in source]
        corrected_header = list(header)
        corrected_header[4] = "    40.60  -111.63        2651          40        1990              35          source-row subset; original daily fields; -I2"
        cli.write_text("\n".join(corrected_header + body) + "\n")
        return {"cli_path": relative(cli), "cli_sha256": sha256(cli), "prn": None}
    prn_info = write_prn(variant, source, observed)
    runspec = write_runspec(variant, REPO / prn_info["path"])
    completed = subprocess.run([str(CLIGEN), "run", str(runspec)], cwd=REPO, text=True, capture_output=True)
    if completed.returncode != 0:
        raise RuntimeError(f"CLIGEN failed for {variant}: {completed.stderr}")
    cli = GENERATED / f"{variant}.cli"
    return {"cli_path": relative(cli), "cli_sha256": sha256(cli), "quality_sha256": sha256(Path(f"{cli}.quality.json")), "runspec_sha256": sha256(runspec), "prn": prn_info}


def fixture_for(variant: str, cli: Path) -> Path:
    destination = FIXTURES / variant / "snotel_snowbird_ut"
    shutil.copytree(SOURCE_FIXTURE, destination)
    shutil.copy2(cli, destination / "p8.cli")
    return destination


def execute_variant(variant: str, fixture: Path) -> dict[str, Any]:
    lane = next(x for x in w1.selected_lanes() if x.lane_id == "snotel_snowbird_ut")
    run_dir = RUNS / variant / lane.lane_id / "B"
    run_dir.mkdir(parents=True)
    stem = f"{lane.lane_id}-B"
    runfile = run_dir / f"{stem}.run"
    trace = run_dir / f"{stem}.snow.jsonl"
    w1.eb04r.legacy.observed_harness.write_runfile(runfile, fixture, "p8", run_dir, stem)
    command = w1.eb04r.legacy.observed_harness.cli_command(OPENWEPP, fixture, runfile, run_dir, "direct-production-executor")
    environment, removed, effective = w1.eb04r.sanitized_environment(os.environ, "B", trace)
    completed = subprocess.run(command, cwd=REPO, env=environment, text=True, capture_output=True)
    (run_dir / "stdout.txt").write_text(completed.stdout)
    (run_dir / "stderr.txt").write_text(completed.stderr)
    if completed.returncode != 0:
        raise RuntimeError(f"openWEPP failed for {variant}: {completed.stderr[-1000:]}")
    return {"returncode": 0, "argv": [str(x) for x in command], "removed_openwepp_keys": removed, "effective_openwepp_environment": effective, "trace_sha256": sha256(trace), "wat_sha256": sha256(run_dir / f"{stem}.wat.parquet")}


def analyze_variant(variant: str) -> dict[str, Any]:
    lane = next(x for x in w1.selected_lanes() if x.lane_id == "snotel_snowbird_ut")
    run_root = RUNS / variant
    w1.eb04w.RUNS = run_root
    cell = w1.eb04w.analyze_cell(lane, "B")
    peak = w1.operator_metric(lane, "seasonal_peak_swe_date", -44.5, cell, run_root)
    meltout = w1.operator_metric(lane, "seasonal_ablation_meltout_date", 0.0, cell, run_root)
    closure = max(
        cell["maximum_melt_component_closure_m"],
        cell["maximum_uncapped_melt_component_closure_m"],
        cell["maximum_daily_applied_raw_melt_closure_m"],
        cell["maximum_accumulation_closure_m"],
        cell["maximum_phase_amount_closure_m"],
        cell["maximum_snow_depth_swe_closure_m"],
        cell["maximum_trace_wat_swe_closure_m"],
        cell["maximum_trace_wat_depth_closure_m"],
        peak["maximum_mass_closure_m"],
        meltout["maximum_mass_closure_m"],
    )
    return {
        "median_peak_swe_ratio": peak["median_peak_ratio"],
        "median_peak_date_offset_days": peak["executed_offset_days"],
        "median_meltout_offset_days": meltout["executed_offset_days"],
        "median_effective_input_ratio": peak["median_effective_input_ratio"],
        "median_storage_ratio": peak["median_storage_ratio"],
        "median_prepeak_loss_m": peak["median_prepeak_loss_m"],
        "median_prepeak_coe_melt_m": peak["median_prepeak_coe_melt_m"],
        "maximum_closure_m": closure,
    }


def changed_field_counts(control: Path, candidate: Path, field_count: int) -> list[int]:
    control_rows = [line.split() for line in control.read_text().splitlines() if len(line.split()) == field_count]
    candidate_rows = [line.split() for line in candidate.read_text().splitlines() if len(line.split()) == field_count]
    if len(control_rows) != len(candidate_rows):
        raise RuntimeError("comparison row inventory differs")
    return [sum(a[index] != b[index] for a, b in zip(control_rows, candidate_rows)) for index in range(field_count)]


def execute() -> None:
    if RECEIPT.exists() or OUTPUT.exists():
        raise RuntimeError("refusing to overwrite result-bearing evidence")
    freeze = json.loads(FREEZE.read_text())
    bindings = {
        SOURCE_CLI: freeze["source_climate_sha256"],
        OBSERVATION: freeze["snotel_observation_sha256"],
        CLIGEN: freeze["cligen_binary_sha256"],
        STATION: freeze["alta_station_sha256"],
        OPENWEPP: freeze["openwepp_binary_sha256"],
        Path(__file__): freeze["experiment_tool_sha256"],
        W1_TOOL: freeze["w1_tool_sha256"],
        EB04W_TOOL: freeze["eb04w_tool_sha256"],
        EB04R_TOOL: freeze["eb04r_tool_sha256"],
    }
    for path, expected in bindings.items():
        if sha256(path) != expected:
            raise RuntimeError(f"binding mismatch: {path}")
    GENERATED.mkdir(parents=True)
    header, source = load_cli()
    observed = load_snotel()
    generated = {}
    executed = {}
    for variant in VARIANTS:
        generated[variant] = make_cli(variant, header, source, observed)
        fixture = fixture_for(variant, REPO / generated[variant]["cli_path"])
        executed[variant] = execute_variant(variant, fixture)
        print("EXECUTED", variant)
    control_prn = GENERATED / "cligen_control.prn"
    control_cli = GENERATED / "cligen_control.cli"
    for variant in ("snotel_p", "snotel_t", "snotel_pt"):
        generated[variant]["effective_prn_changed_columns"] = changed_field_counts(
            control_prn, GENERATED / f"{variant}.prn", 6
        )
        generated[variant]["realized_cli_changed_columns"] = changed_field_counts(
            control_cli, GENERATED / f"{variant}.cli", 13
        )
    receipt = {"schema_version": 2, "freeze_sha256": sha256(FREEZE), "tool_chain_hashes": {relative(Path(__file__)): sha256(Path(__file__)), relative(W1_TOOL): sha256(W1_TOOL), relative(EB04W_TOOL): sha256(EB04W_TOOL), relative(EB04R_TOOL): sha256(EB04R_TOOL)}, "generated": generated, "executed": executed}
    write_json(RECEIPT, receipt)
    results = {variant: analyze_variant(variant) for variant in VARIANTS}
    for row in results.values():
        if row["maximum_closure_m"] > 1.0e-12:
            raise RuntimeError(f"closure exceeded: {row['maximum_closure_m']}")
    write_json(RESULTS, {"schema_version": 1, "freeze_sha256": sha256(FREEZE), "receipt_sha256": sha256(RECEIPT), "variants": results})
    print("SNOWBIRD_SNOTEL_CLIGEN_COMPLETE", len(results))


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
