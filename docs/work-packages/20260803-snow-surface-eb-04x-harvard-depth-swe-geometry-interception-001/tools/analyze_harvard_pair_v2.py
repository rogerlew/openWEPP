#!/usr/bin/env python3
"""Terminal-v2 EB-04X trajectory, phase, and paired-state analysis."""

from __future__ import annotations

import csv
import hashlib
import importlib.util
import json
import math
import subprocess
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
OUT = PACKAGE / "artifacts/terminal-v2"
PROTOCOL = PACKAGE / "artifacts/promotion-protocol.json"
BASE_TOOL = PACKAGE / "tools/analyze_harvard_pair.py"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_base():
    spec = importlib.util.spec_from_file_location("eb04x_base", BASE_TOOL)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load base analyzer")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def guarded_rows(base: Any, path: Path) -> list[dict[str, Any]]:
    rows = base.read_jsonl(path)
    indices = [int(row["day_index"]) for row in rows]
    if indices != sorted(indices) or len(indices) != len(set(indices)):
        raise RuntimeError(f"nonmonotonic or duplicate day index: {path}")
    if indices != list(range(len(indices))):
        raise RuntimeError(f"trace is not a contiguous zero-based prefix: {path}")
    return rows


def write_csv(path: Path, header: tuple[str, ...], rows: list[tuple[Any, ...]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(header)
        writer.writerows(rows)


def main() -> int:
    if OUT.exists():
        raise RuntimeError("terminal-v2 output already exists")
    if not PROTOCOL.is_file():
        raise RuntimeError("prospective promotion protocol missing")
    base = load_base()
    factorial = json.loads(base.FACTORIAL.read_text(encoding="utf-8"))
    lanes = {
        row["lane_id"]: row for row in factorial["lanes"] if row["lane_id"] in base.LANES
    }
    traces: dict[str, list[dict[str, Any]]] = {}
    trace_paths: dict[str, Path] = {}
    for lane in base.LANES:
        for model in base.MODELS:
            cell = lanes[lane]["cells"][model]
            path = REPO / cell["trace"]
            if sha256(path) != cell["trace_sha256"]:
                raise RuntimeError(f"committed trace identity mismatch: {lane}/{model}")
            key = f"{lane}/{model}"
            trace_paths[key] = path
            traces[key] = guarded_rows(base, path)

    summaries = {key: base.summarize_trace(rows) for key, rows in traces.items()}
    if any(row["maximum_abs_geometry_residual_kg_m3"] > 1e-6 for row in summaries.values()):
        raise RuntimeError("modeled SWE-depth-density geometry failed")
    if any(row["maximum_abs_mass_residual_m"] > 1e-12 for row in summaries.values()):
        raise RuntimeError("modeled daily mass closure failed")

    daily_obs_rows = list(csv.DictReader(base.OBS.open(encoding="utf-8")))
    profile_rows = list(csv.DictReader(base.PROFILES.open(encoding="utf-8")))
    observed_geometry = base.observed_geometry()
    observed_valid = all(row["nonclosing_row_count"] == 0 for row in observed_geometry.values())

    profile_groups: dict[tuple[str, str], list[float]] = defaultdict(list)
    profile_output: list[tuple[Any, ...]] = []
    for row in profile_rows:
        stratum = row["observed_stratum"]
        if stratum not in ("open", "hardwood"):
            continue
        density = float(row["observed_density_kg_m3"])
        profile_groups[(stratum, row["date"])].append(density)
        profile_output.append(
            (stratum, row["date"], row["profile_depth_m"], density, row["source_record_id"])
        )

    trajectory_rows: list[tuple[Any, ...]] = []
    trajectory_metrics: dict[str, dict[str, float]] = {}
    for lane in base.LANES:
        for model in base.MODELS:
            cell = lanes[lane]["cells"][model]
            key = f"{lane}/{model}"
            observation_metrics = cell.get("observation_metrics")
            if observation_metrics is None:
                trajectory_metrics[key] = {
                    "status": "UNAVAILABLE_INCOMPLETE_CELL",
                    "paired_observation_count": 0,
                }
                continue
            pairs = observation_metrics["sample_pairs"]
            abs_depth = []
            abs_density = []
            for pair in pairs:
                abs_depth.append(abs(float(pair["depth_residual_m"])))
                abs_density.append(abs(float(pair["density_residual_kg_m3"])))
                stratum = "open" if lane == "harvard_open" else "hardwood"
                profile = profile_groups.get((stratum, pair["date"]), [])
                trajectory_rows.append(
                    (
                        lane,
                        model,
                        pair["date"],
                        pair["water_year"],
                        pair["observed_snow_depth_m"],
                        pair["modeled_snow_depth_m"],
                        pair["observed_density_kg_m3"],
                        pair["modeled_density_kg_m3"],
                        sum(profile) / len(profile) if profile else "",
                        len(profile),
                    )
                )
            trajectory_metrics[key] = {
                "status": "AVAILABLE",
                "paired_observation_count": len(pairs),
                "mean_abs_depth_error_m": sum(abs_depth) / len(abs_depth),
                "mean_abs_density_error_kg_m3": sum(abs_density) / len(abs_density),
            }

    phase_rows: list[tuple[Any, ...]] = []
    phase_summary: dict[str, dict[str, Any]] = {}
    extrema_rows: list[tuple[Any, ...]] = []
    pairing: dict[str, dict[str, int | bool]] = {}
    for model in base.MODELS:
        open_rows = traces[f"harvard_open/{model}"]
        hardwood_rows = traces[f"harvard_hardwood/{model}"]
        paired_count = min(len(open_rows), len(hardwood_rows))
        common = list(range(paired_count))
        pairing[model] = {
            "paired_day_count": paired_count,
            "open_row_count": len(open_rows),
            "hardwood_row_count": len(hardwood_rows),
            "common_days_are_contiguous_prefix": True,
            "full_duration_paired": len(open_rows) == len(hardwood_rows),
        }
        deltas: dict[str, list[tuple[int, float]]] = {
            "swe_m": [],
            "depth_m": [],
            "density_kg_m3": [],
        }
        equal_precip = equal_accum = equal_phase = 0
        for day in common:
            op = open_rows[day]
            hw = hardwood_rows[day]
            precip_equal = math.isclose(
                float(op["hyetograph_rainfall_m"]),
                float(hw["hyetograph_rainfall_m"]),
                abs_tol=1e-12,
            )
            accumulation_equal = math.isclose(
                float(op["accumulation_m"]), float(hw["accumulation_m"]), abs_tol=1e-12
            )
            phase_equal = op["snow_phase_model"] == hw["snow_phase_model"]
            equal_precip += precip_equal
            equal_accum += accumulation_equal
            equal_phase += phase_equal
            if model in ("B", "LS"):
                phase_rows.append(
                    (
                        model,
                        day,
                        op["snow_phase_model"],
                        hw["snow_phase_model"],
                        op["hyetograph_rainfall_m"],
                        hw["hyetograph_rainfall_m"],
                        op["accumulation_m"],
                        hw["accumulation_m"],
                        precip_equal,
                        accumulation_equal,
                        phase_equal,
                    )
                )
            deltas["swe_m"].append(
                (day, float(hw["runtime_swe_after_m"]) - float(op["runtime_swe_after_m"]))
            )
            deltas["depth_m"].append(
                (day, float(hw["runtime_depth_after_m"]) - float(op["runtime_depth_after_m"]))
            )
            deltas["density_kg_m3"].append(
                (
                    day,
                    float(hw["runtime_density_after_kg_m3"])
                    - float(op["runtime_density_after_kg_m3"]),
                )
            )
        phase_summary[model] = {
            "paired_day_count": paired_count,
            "equal_precipitation_day_count": equal_precip,
            "equal_accumulation_day_count": equal_accum,
            "equal_phase_model_day_count": equal_phase,
            "equal_precipitation_different_accumulation_day_count": sum(
                1
                for row in phase_rows
                if row[0] == model and row[8] and not row[9]
            )
            if model in ("B", "LS")
            else None,
        }
        for metric, values in deltas.items():
            minimum = min(values, key=lambda item: item[1])
            maximum = max(values, key=lambda item: item[1])
            extrema_rows.append((model, metric, "minimum", minimum[0], minimum[1]))
            extrema_rows.append((model, metric, "maximum", maximum[0], maximum[1]))

    full = {model: bool(pairing[model]["full_duration_paired"]) for model in base.MODELS}
    screens: dict[str, dict[str, str]] = {}
    for name, duration_ok in (
        ("longwave", full["B"] and full["L"]),
        ("sublimation", full["B"] and full["S"]),
        ("combined_interaction", all(full.values())),
    ):
        if not observed_valid or not duration_ok:
            reason = (
                "HF237 supplied SWE geometry is not admitted"
                if not observed_valid
                else "required full-duration pairing is unavailable"
            )
            screens[name] = {"status": "NOT_EVALUABLE", "reason": reason}
        else:
            screens[name] = {"status": "FAIL", "reason": "quantitative predicate not met"}

    inputs = [
        base.FACTORIAL,
        base.OBS,
        base.PROFILES,
        base.PROVENANCE,
        PROTOCOL,
        BASE_TOOL,
        Path(__file__),
        *trace_paths.values(),
    ]
    freeze = {
        "schema": "snow-surface-eb04x-terminal-v2-freeze-v1",
        "source_head": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=REPO, text=True
        ).strip(),
        "inputs": {str(path.relative_to(REPO)): sha256(path) for path in inputs},
        "protocol_sha256": sha256(PROTOCOL),
        "pairing_rule": "unique monotonic zero-based day_index; common contiguous prefix",
        "model_execution_authorized": False,
    }
    results = {
        "schema": "snow-surface-eb04x-terminal-v2-results-v1",
        "observed_geometry": observed_geometry,
        "observed_geometry_valid": observed_valid,
        "modeled_geometry_valid": True,
        "profile_row_count": len(profile_output),
        "profile_date_group_count": len(profile_groups),
        "trajectory_metrics": trajectory_metrics,
        "pairing": pairing,
        "phase_identity": phase_summary,
        "paired_state_extrema": extrema_rows,
        "promotion_screens": screens,
        "canopy_snow_operands_available": False,
        "conclusion": "no component promotion; canopy snow interception remains unidentifiable",
    }

    OUT.mkdir(parents=True, exist_ok=False)
    (OUT / "freeze.json").write_text(json.dumps(freeze, indent=2, sort_keys=True) + "\n")
    (OUT / "results.json").write_text(json.dumps(results, indent=2, sort_keys=True) + "\n")
    write_csv(
        OUT / "density-trajectory.csv",
        (
            "lane",
            "model",
            "date",
            "water_year",
            "observed_depth_m",
            "modeled_depth_m",
            "observed_density_kg_m3",
            "modeled_density_kg_m3",
            "profile_mean_density_kg_m3",
            "profile_layer_count",
        ),
        trajectory_rows,
    )
    write_csv(
        OUT / "profile-density.csv",
        ("stratum", "date", "profile_depth_m", "density_kg_m3", "source_record_id"),
        profile_output,
    )
    write_csv(
        OUT / "daily-phase-identity.csv",
        (
            "model",
            "day_index",
            "open_phase_model",
            "hardwood_phase_model",
            "open_precipitation_m",
            "hardwood_precipitation_m",
            "open_ground_accumulation_m",
            "hardwood_ground_accumulation_m",
            "precipitation_equal",
            "accumulation_equal",
            "phase_model_equal",
        ),
        phase_rows,
    )
    write_csv(
        OUT / "paired-state-extrema.csv",
        ("model", "metric", "operator", "day_index", "hardwood_minus_open"),
        extrema_rows,
    )
    print(
        "EB-04X terminal-v2 PASS: trajectories="
        f"{len(trajectory_rows)} phase_rows={len(phase_rows)} profiles={len(profile_output)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
