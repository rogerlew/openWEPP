#!/usr/bin/env python3
"""Execute and summarize the EB-04V nine-lane density diagnostic population."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import importlib.util
import json
import math
import statistics
import sys
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
OUTPUT = REPO / "target/snow_surface_eb04v_density_diagnostics"
RUNS = OUTPUT / "runs"
BINARY = REPO / "target/release/openwepp-cli-hill"
EB04R_TOOL = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-"
    "execution-adjudication-001/tools/run_experiment.py"
)
EB04R_RESULTS = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-"
    "execution-adjudication-001/artifacts/factorial-results.json"
)
FREEZE = ARTIFACTS / "population-freeze.json"
RECEIPT = ARTIFACTS / "execution-receipt.json"
RESULTS = ARTIFACTS / "density-process-results.json"
SUMMARY_CSV = ARTIFACTS / "density-process-summary.csv"
SYNTHESIS = ARTIFACTS / "scientific-synthesis.md"

PROCESS_FIELDS = {
    "fresh mixing": "density_process_fresh_snow_mixing_delta_kg_m3",
    "wet compaction": "density_process_wet_compaction_delta_kg_m3",
    "destructive metamorphism": (
        "density_process_destructive_metamorphism_delta_kg_m3"
    ),
    "overburden compaction": "density_process_overburden_compaction_delta_kg_m3",
    "structural projection": "density_process_structural_projection_delta_kg_m3",
    "climate fallback": "density_process_climate_fallback_delta_kg_m3",
    "internal cap": "density_process_internal_cap_delta_kg_m3",
    "runtime cap": "density_process_runtime_cap_delta_kg_m3",
    "Stage 3 adjustment": "density_process_downstream_stage3_delta_kg_m3",
}
COMPACTION = ("wet compaction", "destructive metamorphism", "overburden compaction")
COLORS = {
    "fresh mixing": "#3B82F6",
    "wet compaction": "#14B8A6",
    "destructive metamorphism": "#F59E0B",
    "overburden compaction": "#DC2626",
    "structural projection": "#7C3AED",
    "climate fallback": "#8B5CF6",
    "internal cap": "#64748B",
    "runtime cap": "#111827",
    "Stage 3 adjustment": "#EC4899",
}
LANE_LABELS = {
    "harvard_hardwood": "Harvard hardwood",
    "marcell_conifer": "Marcell conifer",
    "marcell_deciduous": "Marcell deciduous",
    "marcell_open": "Marcell open",
    "snotel_css_lab_ca": "SNOTEL CSS Lab, CA",
    "snotel_mica_creek_st_joe_id": "SNOTEL Mica Creek/St. Joe, ID",
    "snotel_niwot_co": "SNOTEL Niwot, CO",
    "snotel_paradise_wa": "SNOTEL Paradise, WA",
    "snotel_snowbird_ut": "SNOTEL Snowbird, UT",
}


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


eb04r = load_module("eb04v_eb04r_harness", EB04R_TOOL)
eb04r.RUNS = RUNS
eb04r.BINARY = BINARY


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(REPO.resolve()))


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def selected_lanes() -> list[Any]:
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    expected = {row["lane_id"] for row in frozen["lanes"]}
    lanes = [lane for lane in eb04r.legacy.fixed_lanes() if lane.lane_id in expected]
    if len(lanes) != 9 or {lane.lane_id for lane in lanes} != expected:
        raise RuntimeError("runtime lanes differ from the frozen nine-lane population")
    return lanes


def validate_retained_identity(receipt: dict[str, Any]) -> None:
    if receipt["population_freeze_sha256"] != sha256(FREEZE):
        raise RuntimeError("population freeze no longer matches execution receipt")
    expected_keys = {
        f"{lane.lane_id}/{cell}" for lane in selected_lanes() for cell in eb04r.CELLS
    }
    if set(receipt["results"]) != expected_keys:
        raise RuntimeError("execution receipt does not bind the frozen 36 cells")
    file_names = {
        "runfile": lambda stem: f"{stem}.run",
        "runtime_manifest": lambda _stem: "openwepp_hillslope_run_manifest.json",
        "stderr": lambda _stem: "stderr.txt",
        "stdout": lambda _stem: "stdout.txt",
        "trace": lambda stem: f"{stem}.snow.jsonl",
        "wat": lambda stem: f"{stem}.wat.parquet",
    }
    for key, receipt_row in sorted(receipt["results"].items()):
        lane_id, cell = key.split("/", maxsplit=1)
        run_dir = RUNS / lane_id / cell
        provenance_path = REPO / receipt_row["provenance"]
        if sha256(provenance_path) != receipt_row["provenance_sha256"]:
            raise RuntimeError(f"provenance identity drift for {key}")
        provenance = json.loads(provenance_path.read_text(encoding="utf-8"))
        if provenance["binary_sha256"] != receipt["binary_sha256"]:
            raise RuntimeError(f"binary provenance drift for {key}")
        if provenance["lane_id"] != lane_id or provenance["cell"] != cell:
            raise RuntimeError(f"lane/cell provenance drift for {key}")
        stem = f"{lane_id}-{cell}"
        for file_id, name_for_stem in file_names.items():
            path = run_dir / name_for_stem(stem)
            sealed = provenance["files"][file_id]
            if sha256(path) != sealed["sha256"] or path.stat().st_size != sealed["size_bytes"]:
                raise RuntimeError(f"retained {file_id} identity drift for {key}")


def optional_float(value: Any) -> float | None:
    try:
        parsed = float(value)
    except (TypeError, ValueError):
        return None
    return parsed if math.isfinite(parsed) else None


def pearson(left: list[float], right: list[float]) -> float | None:
    if len(left) < 3 or len(left) != len(right):
        return None
    left_mean = statistics.fmean(left)
    right_mean = statistics.fmean(right)
    left_delta = [value - left_mean for value in left]
    right_delta = [value - right_mean for value in right]
    denominator = math.sqrt(
        sum(value * value for value in left_delta)
        * sum(value * value for value in right_delta)
    )
    if denominator <= 1.0e-18:
        return None
    return sum(a * b for a, b in zip(left_delta, right_delta)) / denominator


def kge_components(observed: list[float], modeled: list[float]) -> dict[str, Any]:
    if len(observed) < 3 or len(observed) != len(modeled):
        return {"kge": None, "r": None, "beta": None, "gamma": None}
    observed_mean = statistics.fmean(observed)
    modeled_mean = statistics.fmean(modeled)
    observed_std = statistics.stdev(observed)
    modeled_std = statistics.stdev(modeled)
    correlation = pearson(observed, modeled)
    if (
        correlation is None
        or abs(observed_mean) <= 1.0e-12
        or abs(modeled_mean) <= 1.0e-12
        or observed_std <= 1.0e-12
    ):
        return {"kge": None, "r": correlation, "beta": None, "gamma": None}
    beta = modeled_mean / observed_mean
    gamma = (modeled_std / abs(modeled_mean)) / (observed_std / abs(observed_mean))
    kge = 1.0 - math.sqrt(
        (correlation - 1.0) ** 2 + (beta - 1.0) ** 2 + (gamma - 1.0) ** 2
    )
    return {"kge": kge, "r": correlation, "beta": beta, "gamma": gamma}


def observed_density_by_date(lane: Any) -> dict[dt.date, float]:
    density = {}
    for row in observation_rows(lane):
        try:
            date = dt.date.fromisoformat(row["date"])
        except (KeyError, TypeError, ValueError):
            continue
        swe_mm = optional_float(row.get("observed_swe_mm"))
        depth_m = optional_float(row.get("observed_snow_depth_m"))
        value = optional_float(row.get("observed_density_kg_m3"))
        if swe_mm is not None and depth_m is not None and value is not None:
            density[date] = value
    return density


def retained_density_anchors(frozen: dict[str, Any]) -> dict[str, dict[str, Any]]:
    authority = frozen["observation_operator_authority"]
    if authority["path"] != relative(EB04R_RESULTS):
        raise RuntimeError("frozen density-operator authority path drift")
    if authority["sha256"] != sha256(EB04R_RESULTS):
        raise RuntimeError("frozen density-operator authority hash drift")
    predecessor = json.loads(EB04R_RESULTS.read_text(encoding="utf-8"))
    anchors = {}
    for lane in predecessor["lanes"]:
        lane_id = lane["lane_id"]
        if lane_id not in {row["lane_id"] for row in frozen["lanes"]}:
            continue
        density_cell = next(
            row
            for row in lane["cells"]["B"]["rubric_profile"]["cells"]
            if row["cell_id"] == "seasonal_densification_trajectory"
        )
        anchors[lane_id] = density_cell["metrics"]
    if set(anchors) != {row["lane_id"] for row in frozen["lanes"]}:
        raise RuntimeError("retained density anchors do not cover the frozen lanes")
    return anchors


def execute(workers: int) -> None:
    if RECEIPT.exists() or any(RUNS.rglob("*.snow.jsonl")):
        raise RuntimeError("EB-04V execution already exists; use --analysis-only")
    if not BINARY.is_file():
        raise FileNotFoundError(f"build the exact release binary first: {BINARY}")
    lanes = selected_lanes()
    RUNS.mkdir(parents=True, exist_ok=True)
    futures = {}
    results = {}
    with ThreadPoolExecutor(max_workers=workers) as executor:
        for lane in lanes:
            for cell in eb04r.CELLS:
                future = executor.submit(eb04r.execute_cell, lane, cell)
                futures[future] = (lane.lane_id, cell)
        for future in as_completed(futures):
            lane_id, cell = futures[future]
            result = future.result()
            key = f"{lane_id}/{cell}"
            results[key] = result
            print(f"{key}: {'PASS' if result['returncode'] == 0 else 'FAIL'}")
    if len(results) != 36 or any(row["returncode"] != 0 for row in results.values()):
        raise RuntimeError("the frozen 36-cell population did not complete")
    write_json(
        RECEIPT,
        {
            "schema": "snow-surface-eb04v-execution-receipt-v1",
            "evidence_role": "DIAGNOSTIC_ONLY",
            "binary": relative(BINARY),
            "binary_sha256": sha256(BINARY),
            "population_freeze_sha256": sha256(FREEZE),
            "cell_count": 36,
            "environment_policy": "REMOVE_ALL_INHERITED_OPENWEPP_THEN_INSTALL_EXACT_SEVEN",
            "results": dict(sorted(results.items())),
        },
    )


def observation_rows(lane: Any) -> list[dict[str, str]]:
    with lane.observation_file.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    if lane.observation_filter:
        rows = [
            row
            for row in rows
            if all(row.get(key) == value for key, value in lane.observation_filter.items())
        ]
    return rows


def observed_phase_bounds(lane: Any) -> dict[int, tuple[dt.date, dt.date, dt.date]]:
    grouped: dict[int, list[tuple[dt.date, float]]] = defaultdict(list)
    for row in observation_rows(lane):
        try:
            date = dt.date.fromisoformat(row["date"])
            swe = float(row["observed_swe_mm"])
        except (KeyError, TypeError, ValueError):
            continue
        if swe > 0.0:
            grouped[int(row["water_year"])].append((date, swe))
    bounds = {}
    for water_year, values in grouped.items():
        ordered = sorted(values)
        peak_swe = max(value for _, value in ordered)
        peak = min(date for date, value in ordered if value == peak_swe)
        bounds[water_year] = (ordered[0][0], peak, ordered[-1][0])
    return bounds


def phase_for_date(
    date: dt.date, bounds: dict[int, tuple[dt.date, dt.date, dt.date]]
) -> str:
    water_year = date.year + 1 if date.month >= 10 else date.year
    if water_year not in bounds:
        return "outside observed frame"
    snow_on, peak, snow_off = bounds[water_year]
    if date < snow_on or date > snow_off:
        return "outside observed frame"
    if date < peak:
        return "pre-peak accumulation"
    if date == peak:
        return "observed peak anchor"
    return "post-peak ablation"


def trace_and_modeled(
    lane_id: str,
    cell: str,
) -> tuple[list[dict[str, Any]], dict[dt.date, dict[str, float | None]]]:
    run_dir = RUNS / lane_id / cell
    stem = f"{lane_id}-{cell}"
    trace_path = run_dir / f"{stem}.snow.jsonl"
    wat_path = run_dir / f"{stem}.wat.parquet"
    rows = [json.loads(line) for line in trace_path.read_text(encoding="utf-8").splitlines()]
    modeled = eb04r.legacy.observed_harness.load_modeled_wat(wat_path)
    if len(rows) != len(modeled):
        raise RuntimeError(f"trace/WAT chronology mismatch for {lane_id}/{cell}")
    return rows, modeled


def analyze() -> dict[str, Any]:
    receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
    if receipt["binary_sha256"] != sha256(BINARY):
        raise RuntimeError("executed binary hash no longer matches release binary")
    validate_retained_identity(receipt)
    frozen = json.loads(FREEZE.read_text(encoding="utf-8"))
    retained_anchors = retained_density_anchors(frozen)
    bias = {row["lane_id"]: row["bias_partition"] for row in frozen["lanes"]}
    lanes = {lane.lane_id: lane for lane in selected_lanes()}
    summaries = []
    phase_totals: dict[tuple[str, str, str], list[float]] = defaultdict(lambda: [0.0, 0.0])
    driver_pairs: dict[tuple[str, str], tuple[list[float], list[float]]] = defaultdict(
        lambda: ([], [])
    )
    error_pairs: dict[tuple[str, str], tuple[list[float], list[float]]] = defaultdict(
        lambda: ([], [])
    )
    kge_rows = []
    maximum_closure = 0.0
    maximum_emitted_closure_difference = 0.0
    maximum_omitted_process_residual = {process: 0.0 for process in PROCESS_FIELDS}
    fresh_density_anti_alias_count = 0
    schema_set = set()
    for lane_id in sorted(lanes):
        bounds = observed_phase_bounds(lanes[lane_id])
        observed_density = observed_density_by_date(lanes[lane_id])
        for cell in eb04r.CELLS:
            rows, modeled = trace_and_modeled(lane_id, cell)
            dates = list(modeled)
            totals = {name: 0.0 for name in PROCESS_FIELDS}
            applicable = 0
            fresh_days = 0
            cap_days = 0
            internal_cap_days = 0
            runtime_cap_days = 0
            fallback_days = 0
            paired_by_phase: dict[str, tuple[list[float], list[float]]] = defaultdict(
                lambda: ([], [])
            )
            for row, date in zip(rows, dates):
                schema_set.add(row.get("schema"))
                initial = float(row["density_process_initial_density_kg_m3"])
                final = float(row["density_process_final_density_kg_m3"])
                process_values = {
                    process: float(row[field]) for process, field in PROCESS_FIELDS.items()
                }
                increments = list(process_values.values())
                reconstructed = final - initial - sum(increments)
                emitted = float(row["density_process_closure_residual_kg_m3"])
                maximum_closure = max(maximum_closure, abs(reconstructed))
                maximum_emitted_closure_difference = max(
                    maximum_emitted_closure_difference, abs(emitted - reconstructed)
                )
                for process, value in process_values.items():
                    maximum_omitted_process_residual[process] = max(
                        maximum_omitted_process_residual[process],
                        abs(reconstructed + value),
                    )
                if row["density_process_fresh_snow_density_available"]:
                    fresh_density_anti_alias_count += int(
                        abs(
                            float(row["density_process_fresh_snow_density_kg_m3"])
                            - final
                        )
                        > 1.0e-9
                    )
                observed = observed_density.get(date)
                modeled_row = modeled[date]
                depth_m = optional_float(modeled_row.get("snow_depth_m"))
                swe_m = optional_float(modeled_row.get("snow_water_m"))
                signed_error = None
                if observed is not None and depth_m is not None and swe_m is not None:
                    modeled_density = swe_m * 1_000.0 / depth_m if depth_m > 1.0e-9 else 0.0
                    phase = phase_for_date(date, bounds)
                    for phase_key in ("all observed", phase):
                        observed_values, modeled_values = paired_by_phase[phase_key]
                        observed_values.append(observed)
                        modeled_values.append(modeled_density)
                    signed_error = modeled_density - observed

                if not row["density_process_applicable"]:
                    continue
                applicable += 1
                fresh_days += int(row["density_process_fresh_snow_density_available"])
                internal_cap_active = abs(process_values["internal cap"]) > 1.0e-9
                runtime_cap_active = abs(process_values["runtime cap"]) > 1.0e-9
                internal_cap_days += int(internal_cap_active)
                runtime_cap_days += int(runtime_cap_active)
                cap_days += int(internal_cap_active or runtime_cap_active)
                fallback_days += int(row["density_process_climate_fallback_used"])
                phase = phase_for_date(date, bounds)
                for process, value in process_values.items():
                    totals[process] += value
                    key = (bias[lane_id], phase, process)
                    phase_totals[key][0] += value
                    phase_totals[key][1] += 1.0

                drivers = {
                    "liquid input": float(
                        row["density_process_liquid_for_compaction_mass_kg_m2"]
                    ),
                    "compaction temperature": float(
                        row["density_process_compaction_temperature_c"]
                    ),
                    "initial snow load": float(
                        row["density_process_initial_snow_mass_kg_m2"]
                    ),
                    "settle age": float(row["runtime_settle_day_count_before"]),
                    "layer-count change": float(
                        row["snow_layer_count_after"] - row["snow_layer_count_before"]
                    ),
                }
                for relation, driver, process in [
                    ("wet compaction vs liquid input", "liquid input", "wet compaction"),
                    (
                        "destructive metamorphism vs temperature",
                        "compaction temperature",
                        "destructive metamorphism",
                    ),
                    (
                        "overburden compaction vs initial load",
                        "initial snow load",
                        "overburden compaction",
                    ),
                ]:
                    left, right = driver_pairs[(bias[lane_id], relation)]
                    left.append(drivers[driver])
                    right.append(process_values[process])

                if signed_error is None:
                    continue
                association_values = {
                    **drivers,
                    "fresh mixing": process_values["fresh mixing"],
                    "wet compaction": process_values["wet compaction"],
                    "destructive metamorphism": process_values["destructive metamorphism"],
                    "overburden compaction": process_values["overburden compaction"],
                    "structural projection": process_values["structural projection"],
                    "internal cap": process_values["internal cap"],
                    "runtime cap": process_values["runtime cap"],
                }
                for relation, value in association_values.items():
                    left, right = error_pairs[(bias[lane_id], relation)]
                    left.append(value)
                    right.append(signed_error)
            for phase, (observed_values, modeled_values) in sorted(paired_by_phase.items()):
                kge_rows.append(
                    {
                        "lane_id": lane_id,
                        "cell": cell,
                        "bias_partition": bias[lane_id],
                        "phase": phase,
                        "paired_count": len(observed_values),
                        "mean_signed_error_kg_m3": (
                            statistics.fmean(
                                modeled_value - observed_value
                                for observed_value, modeled_value in zip(
                                    observed_values, modeled_values
                                )
                            )
                            if observed_values
                            else None
                        ),
                        **kge_components(observed_values, modeled_values),
                    }
                )
            summary = {
                "lane_id": lane_id,
                "cell": cell,
                "bias_partition": bias[lane_id],
                "trace_row_count": len(rows),
                "applicable_day_count": applicable,
                "fresh_snow_day_count": fresh_days,
                "cap_day_count": cap_days,
                "internal_cap_day_count": internal_cap_days,
                "runtime_cap_day_count": runtime_cap_days,
                "climate_fallback_used_day_count": fallback_days,
                "process_mean_daily_kg_m3": {
                    process: value / applicable if applicable else 0.0
                    for process, value in totals.items()
                },
            }
            summaries.append(summary)
    if schema_set != {"openwepp-r7h-direct-production-snow-trace-v2"}:
        raise RuntimeError(f"unexpected trace schemas: {schema_set}")
    if maximum_closure > 1.0e-9:
        raise RuntimeError(f"density ledger closure failed: {maximum_closure}")
    if maximum_emitted_closure_difference > 1.0e-12:
        raise RuntimeError("emitted and independently reconstructed closure disagree")
    if maximum_omitted_process_residual["overburden compaction"] <= 1.0e-6:
        raise RuntimeError("omitted-overburden anti-tautology vector did not fail")
    if fresh_density_anti_alias_count == 0:
        raise RuntimeError("fresh density never differed from final density")
    anchor_validation = []
    for lane_id, expected in sorted(retained_anchors.items()):
        actual = next(
            row
            for row in kge_rows
            if row["lane_id"] == lane_id
            and row["cell"] == "B"
            and row["phase"] == "all observed"
        )
        differences = {
            key: abs(actual[key] - expected[key])
            for key in ("kge", "r", "beta", "gamma")
        }
        if actual["paired_count"] != expected["paired_count"] or max(differences.values()) > 1.0e-12:
            raise RuntimeError(f"retained B density operator drift for {lane_id}")
        anchor_validation.append(
            {
                "lane_id": lane_id,
                "paired_count": actual["paired_count"],
                "maximum_abs_kge_component_difference": max(differences.values()),
            }
        )
    phase_means = [
        {
            "bias_partition": bias_partition,
            "phase": phase,
            "process": process,
            "mean_daily_kg_m3": values[0] / values[1] if values[1] else 0.0,
            "applicable_process_rows": int(values[1]),
        }
        for (bias_partition, phase, process), values in sorted(phase_totals.items())
    ]
    driver_associations = [
        {
            "bias_partition": bias_partition,
            "relation": relation,
            "paired_count": len(values[0]),
            "pearson_r": pearson(*values),
        }
        for (bias_partition, relation), values in sorted(driver_pairs.items())
    ]
    error_associations = [
        {
            "bias_partition": bias_partition,
            "conditioning_variable": relation,
            "paired_count": len(values[0]),
            "pearson_r_with_signed_density_error": pearson(*values),
        }
        for (bias_partition, relation), values in sorted(error_pairs.items())
    ]
    result = {
        "schema": "snow-surface-eb04v-density-process-results-v1",
        "evidence_role": "DIAGNOSTIC_ONLY",
        "binary_sha256": receipt["binary_sha256"],
        "analysis_tool_sha256": sha256(Path(__file__)),
        "execution_receipt_sha256": sha256(RECEIPT),
        "population_freeze_sha256": sha256(FREEZE),
        "cell_count": len(summaries),
        "lane_count": len(lanes),
        "maximum_abs_closure_residual_kg_m3": maximum_closure,
        "maximum_emitted_closure_difference_kg_m3": maximum_emitted_closure_difference,
        "maximum_omitted_process_residual_kg_m3": maximum_omitted_process_residual,
        "fresh_density_anti_alias_row_count": fresh_density_anti_alias_count,
        "retained_density_operator_authority_sha256": sha256(EB04R_RESULTS),
        "retained_density_operator_anchor_validation": anchor_validation,
        "summaries": summaries,
        "phase_means": phase_means,
        "driver_associations": driver_associations,
        "density_error_associations": error_associations,
        "kge_components": kge_rows,
    }
    write_json(RESULTS, result)
    write_summary_csv(summaries)
    make_figures(result)
    write_synthesis(result)
    return result


def write_summary_csv(summaries: list[dict[str, Any]]) -> None:
    with SUMMARY_CSV.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "lane_id",
                "cell",
                "bias_partition",
                "applicable_days",
                "internal_cap_days",
                "runtime_cap_days",
                "climate_fallback_used_days",
                *PROCESS_FIELDS,
            ]
        )
        for row in summaries:
            writer.writerow(
                [
                    row["lane_id"], row["cell"], row["bias_partition"],
                    row["applicable_day_count"],
                    row["internal_cap_day_count"],
                    row["runtime_cap_day_count"],
                    row["climate_fallback_used_day_count"],
                    *(row["process_mean_daily_kg_m3"][name] for name in PROCESS_FIELDS),
                ]
            )


def save_figure(fig: Any, stem: str) -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    fig.savefig(FIGURES / f"{stem}.svg", format="svg", bbox_inches="tight")


def make_figures(result: dict[str, Any]) -> None:
    import matplotlib.pyplot as plt
    import numpy as np

    summaries = result["summaries"]
    lanes = sorted({row["lane_id"] for row in summaries})
    fig, ax = plt.subplots(figsize=(11, 6.5))
    left = np.zeros(len(lanes))
    for process in COMPACTION:
        values = [
            sum(
                row["process_mean_daily_kg_m3"][process]
                for row in summaries if row["lane_id"] == lane
            ) / 4.0
            for lane in lanes
        ]
        ax.barh(
            [LANE_LABELS[lane] for lane in lanes],
            values,
            left=left,
            label=process,
            color=COLORS[process],
        )
        left += np.asarray(values)
    ax.set_xlabel("Mean density increase on applicable model days (kg m⁻³ day⁻¹)")
    ax.set_title("What compacts the modeled snowpack? (four-cell mean)")
    ax.grid(axis="x", alpha=0.25, zorder=0)
    ax.legend(
        loc="upper center",
        bbox_to_anchor=(0.5, -0.13),
        ncol=3,
        frameon=True,
    )
    save_figure(fig, "eb04v-compaction-by-lane")
    plt.close(fig)

    phases = ["pre-peak accumulation", "observed peak anchor", "post-peak ablation"]
    groups = ["density_shape_over_beta", "density_shape_under_beta"]
    lookup = {
        (row["bias_partition"], row["phase"], row["process"]): row["mean_daily_kg_m3"]
        for row in result["phase_means"]
    }
    fig, axes = plt.subplots(1, 2, figsize=(12, 5.2), sharey=True)
    for ax, group in zip(axes, groups):
        bottom = np.zeros(len(phases))
        for process in COMPACTION:
            values = [lookup.get((group, phase, process), 0.0) for phase in phases]
            ax.bar(phases, values, bottom=bottom, label=process, color=COLORS[process])
            bottom += np.asarray(values)
        ax.set_title("Over-density group" if "over" in group else "Under-density group")
        ax.tick_params(axis="x", rotation=20)
        ax.grid(axis="y", alpha=0.25, zorder=0)
    axes[0].set_ylabel("Mean density increase (kg m⁻³ day⁻¹)")
    handles, labels = axes[1].get_legend_handles_labels()
    fig.legend(
        handles,
        labels,
        loc="upper center",
        bbox_to_anchor=(0.5, -0.01),
        ncol=3,
        frameon=True,
    )
    fig.suptitle("Compaction changes across observed seasonal phases")
    save_figure(fig, "eb04v-compaction-by-phase")
    plt.close(fig)

    signed = (
        "fresh mixing",
        "structural projection",
        "internal cap",
        "runtime cap",
        "Stage 3 adjustment",
    )
    fig, ax = plt.subplots(figsize=(10.5, 5.5))
    x = np.arange(len(signed))
    width = 0.36
    for offset, group in [(-width / 2, groups[0]), (width / 2, groups[1])]:
        members = [row for row in summaries if row["bias_partition"] == group]
        values = [
            sum(row["process_mean_daily_kg_m3"][process] for row in members) / len(members)
            for process in signed
        ]
        ax.bar(
            x + offset, values, width,
            label="Over-density failures" if "over" in group else "Under-density failures",
        )
    ax.axhline(0.0, color="#111827", linewidth=1.0)
    ax.set_xticks(x, signed)
    ax.set_ylabel("Signed mean density change (kg m⁻³ day⁻¹)")
    ax.set_title("Non-compaction terms can oppose or reinforce densification")
    ax.grid(axis="y", alpha=0.25, zorder=0)
    ax.legend(loc="best", frameon=True)
    save_figure(fig, "eb04v-signed-structural-effects")
    plt.close(fig)

    association_variables = [
        "liquid input",
        "compaction temperature",
        "initial snow load",
        "fresh mixing",
        "wet compaction",
        "internal cap",
        "structural projection",
    ]
    association_lookup = {
        (row["bias_partition"], row["conditioning_variable"]): row[
            "pearson_r_with_signed_density_error"
        ]
        for row in result["density_error_associations"]
    }
    fig, ax = plt.subplots(figsize=(11, 6.2))
    y = np.arange(len(association_variables))
    for offset, group in [(-width / 2, groups[0]), (width / 2, groups[1])]:
        values = [association_lookup.get((group, variable), 0.0) for variable in association_variables]
        ax.barh(
            y + offset,
            values,
            width,
            label="Over-density failures" if "over" in group else "Under-density failures",
        )
    ax.axvline(0.0, color="#111827", linewidth=1.0)
    ax.set_yticks(y, association_variables)
    ax.set_xlim(-1.0, 1.0)
    ax.set_xlabel("Pearson r with signed density error (modeled − observed)")
    ax.set_title("Which modeled states coincide with density error?")
    ax.grid(axis="x", alpha=0.25, zorder=0)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.12), ncol=2, frameon=True)
    save_figure(fig, "eb04v-density-error-associations")
    plt.close(fig)

    all_kge = [row for row in result["kge_components"] if row["phase"] == "all observed"]
    fig, ax = plt.subplots(figsize=(10.5, 6.2))
    y = np.arange(len(lanes))
    for marker, component, label in [
        ("o", "r", "correlation r"),
        ("s", "beta", "bias ratio β"),
        ("^", "gamma", "variability ratio γ"),
    ]:
        values = []
        for lane in lanes:
            finite = [
                row[component]
                for row in all_kge
                if row["lane_id"] == lane and row[component] is not None
            ]
            values.append(statistics.fmean(finite) if finite else math.nan)
        ax.scatter(values, y, marker=marker, s=55, label=label)
    ax.axvline(1.0, color="#111827", linewidth=1.0, linestyle="--")
    ax.set_yticks(y, [LANE_LABELS[lane] for lane in lanes])
    ax.set_xlabel("Four-cell arithmetic mean of KGE component")
    ax.set_title("Observed-density trajectory components by lane")
    ax.grid(axis="x", alpha=0.25, zorder=0)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.12), ncol=3, frameon=True)
    save_figure(fig, "eb04v-kge-components-by-lane")
    plt.close(fig)
    write_sidecars()


def write_sidecars() -> None:
    sidecars = {
        "eb04v-compaction-by-lane": (
            "Compaction contributions by lane",
            "Arithmetic mean of four cell-level daily density-ledger means. The figure shows whether wet compaction, destructive metamorphism, or overburden dominates each retained density failure.",
            "Bars are diagnostic contributions from the executed B/L/S/LS traces, not fitted effects or unique causal estimates."
        ),
        "eb04v-compaction-by-phase": (
            "Compaction by observed seasonal phase",
            "Process increments grouped by observation-defined snow-on, peak-SWE anchor, and post-peak frame, separately for retained over- and under-density trajectory failures. Each bar is weighted by applicable modeled rows in its group and phase.",
            "Sparse peak-anchor rows and uneven observation periods limit direct comparison of bar precision. Observations were already consumed and remain diagnostic-only."
        ),
        "eb04v-signed-structural-effects": (
            "Signed non-compaction density effects",
            "Fresh-snow mixing, the internal process cap, structural mass/layer projection, the runtime cap, and downstream Stage-3 mass change can counteract or reinforce compaction. Negative bars reduce bulk density.",
            "These terms explain ledger movement, not observation residual ownership. Means pool sites and cells with different record lengths."
        ),
        "eb04v-density-error-associations": (
            "Density-error associations",
            "Pearson associations between modeled state/process values and signed matched-date density error on applicable modeled-snow rows. Positive values coincide with modeled density that is high relative to observations.",
            "Association is not causation. Drivers and process terms covary seasonally, and B/L/S/LS repeat the same observations rather than supplying independent replicates. The observations were already consumed as diagnostic evidence."
        ),
        "eb04v-kge-components-by-lane": (
            "Observed-density KGE components by lane",
            "Four-cell arithmetic means of matched-date density correlation, bias ratio, and variability ratio. The frozen operator retains observed-snow dates when the model has no snow and assigns modeled density `0 kg m^-3`. The dashed line at one is the ideal component value.",
            "Averaging components summarizes B/L/S/LS context; it is not a new promotion score, and observation counts differ by lane and phase."
        ),
    }
    for stem, (title, caption, limitation) in sidecars.items():
        (FIGURES / f"{stem}.md").write_text(
            f"# {title}\n\n## Caption\n\n{caption}\n\n## Population and method\n\n"
            "Nine immutable EB-04U density-failure lanes, each executed under B, L, S, and LS. "
            "Process means are arithmetic means of the four cell-level daily means unless the "
            "caption identifies matched observations or phase-row weighting. Daily process and "
            "driver state derives from the real `openwepp-r7h-direct-production-snow-trace-v2` "
            "consumer; modeled density derives from WAT SWE/depth, and matched observations retain "
            "fixture quality flags. Per-cell applicable and matched-row counts are retained in "
            "`../density-process-results.json`.\n\n"
            f"## Interpretation limits\n\n{limitation} EB-04V changes no physics and makes no promotion claim.\n",
            encoding="utf-8",
        )


def write_synthesis(result: dict[str, Any]) -> None:
    rows = result["summaries"]
    by_bias = defaultdict(list)
    for row in rows:
        by_bias[row["bias_partition"]].append(row)
    lines = [
        "# Scientific Synthesis", "", "Evidence class: `[DIRECT][Ran] + [INFERENCE][Ran]`.", "",
        f"All {result['cell_count']} frozen cells completed. Maximum independently readable ledger closure was "
        f"`{result['maximum_abs_closure_residual_kg_m3']:.3e} kg m^-3`.", "",
        "## Empirical result", "",
    ]
    for group in ("density_shape_over_beta", "density_shape_under_beta"):
        members = by_bias[group]
        means = {
            process: sum(row["process_mean_daily_kg_m3"][process] for row in members) / len(members)
            for process in COMPACTION
        }
        dominant = max(means, key=means.get)
        label = "over-density" if "over" in group else "under-density"
        lines.append(
            f"- In the retained {label} group, `{dominant}` is the largest four-cell-arithmetic-mean positive compaction contribution "
            f"(`{means[dominant]:.4f} kg m^-3 day^-1`)."
        )
        applicable = sum(row["applicable_day_count"] for row in members)
        internal_cap_days = sum(row["internal_cap_day_count"] for row in members)
        runtime_cap_days = sum(row["runtime_cap_day_count"] for row in members)
        fallback_days = sum(row["climate_fallback_used_day_count"] for row in members)
        lines.append(
            f"- The {label} cells contain {applicable:,} applicable modeled snow days; "
            f"material internal caps occur on {internal_cap_days:,}, runtime caps on "
            f"{runtime_cap_days:,}, and the climate fallback is used on {fallback_days:,}."
        )
    association = {
        (row["bias_partition"], row["relation"]): row["pearson_r"]
        for row in result["driver_associations"]
    }
    association_lines = []
    for group in ("density_shape_over_beta", "density_shape_under_beta"):
        label = "over-density" if "over" in group else "under-density"
        values = []
        for relation in (
            "wet compaction vs liquid input",
            "destructive metamorphism vs temperature",
            "overburden compaction vs initial load",
        ):
            value = association.get((group, relation))
            values.append(f"{relation}: r={value:.3f}" if value is not None else f"{relation}: undefined")
        association_lines.append(f"- {label}: " + "; ".join(values) + ".")
    lines += [
        "", "Across both retained bias directions, wet compaction is much larger than overburden compaction and destructive metamorphism in the arithmetic mean of cell-level daily means. The under-density group therefore does not have inactive modeled compaction. That fact does not prove compaction is sufficient relative to forcing and the opposing fresh-mixing, structural, and cap terms, so it does not by itself reject a carefully defined compaction candidate.",
        "", "## Driver and observation association", "",
        *association_lines,
        "", "The phase-row-weighted results show a strong increase in wet compaction during ablation in both groups, but the under-density peak anchor is slightly lower than its pre-peak value. The KGE table and figures retain correlation, bias ratio, variability ratio, matched count, and signed error by lane/cell/phase. These are diagnostic associations against already-consumed observations, not a validation or fitted effect.",
        "", "The ledger resolves which implemented terms move density and when. It does not establish that one coefficient or omitted process uniquely causes the observed trajectory contradiction: fresh-snow mixing, selective layer removal/merge, caps, and Stage-3 mass exchange can offset compaction, and the three compaction drivers covary with temperature, load, and liquid water.",
        "", "## Decision", "",
        "EB-04V is an observability and calibration-readiness result, not an efficacy round. The evidence supports mechanics-specific successor hypotheses but does not authorize coefficient fitting, physics amendment, or promotion. A prospective candidate must first seal independent evidence, materiality, replication, and site-spread rules.", "",
    ]
    SYNTHESIS.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--analysis-only", action="store_true")
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    if args.execute == args.analysis_only:
        parser.error("select exactly one of --execute or --analysis-only")
    if args.execute:
        execute(args.workers)
    result = analyze()
    print(json.dumps({
        "cells": result["cell_count"],
        "max_closure_kg_m3": result["maximum_abs_closure_residual_kg_m3"],
        "status": "PASS",
    }, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
