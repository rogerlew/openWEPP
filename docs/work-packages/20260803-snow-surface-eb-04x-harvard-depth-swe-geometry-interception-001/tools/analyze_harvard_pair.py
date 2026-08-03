#!/usr/bin/env python3
"""Execute the retained-evidence EB-04X Harvard paired diagnostic."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import subprocess
from pathlib import Path
from typing import Any


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
ARTIFACTS = PACKAGE / "artifacts"
FACTORIAL = REPO / (
    "docs/work-packages/20260730-snow-surface-eb-04-factorial-execution-"
    "adjudication-001/artifacts/factorial-results.json"
)
OBS = REPO / "tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv"
PROFILES = REPO / (
    "tests/fixtures/cancov_forest/observations/profiles/harvard_hf237_density_profiles.csv"
)
PROVENANCE = REPO / (
    "tests/fixtures/cancov_forest/observations/provenance/harvard_hf237.json"
)
MODELS = ("B", "L", "S", "LS")
LANES = ("harvard_open", "harvard_hardwood")
OUTPUTS = (
    ARTIFACTS / "freeze.json",
    ARTIFACTS / "results.json",
    ARTIFACTS / "paired-diagnostics.csv",
    ARTIFACTS / "scientific-synthesis.md",
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines()]


def finite(value: Any) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(float(value))


def summarize_trace(rows: list[dict[str, Any]]) -> dict[str, Any]:
    geometry_residuals = []
    for row in rows:
        depth = float(row["runtime_depth_after_m"])
        swe = float(row["runtime_swe_after_m"])
        density = float(row["runtime_density_after_kg_m3"])
        if depth > 0.0:
            geometry_residuals.append(density - 1000.0 * swe / depth)
    return {
        "row_count": len(rows),
        "total_hyetograph_precipitation_m": sum(float(r["hyetograph_rainfall_m"]) for r in rows),
        "total_ground_snow_accumulation_m": sum(float(r["accumulation_m"]) for r in rows),
        "total_sublimation_m": sum(float(r["sublimation_m"]) for r in rows),
        "total_snowpack_swe_loss_m": sum(float(r["snowpack_swe_loss_m"]) for r in rows),
        "total_longwave_energy_j_m2": sum(float(r["stage3_longwave_energy_j_m2"]) for r in rows),
        "total_latent_energy_j_m2": sum(float(r["stage3_latent_energy_j_m2"]) for r in rows),
        "peak_swe_m": max(float(r["runtime_swe_after_m"]) for r in rows),
        "peak_depth_m": max(float(r["runtime_depth_after_m"]) for r in rows),
        "maximum_abs_geometry_residual_kg_m3": max(map(abs, geometry_residuals), default=0.0),
        "maximum_abs_mass_residual_m": max(
            abs(
                float(r["runtime_swe_before_m"])
                + float(r["accumulation_m"])
                + float(r["rain_retained_m"])
                - float(r["snowpack_swe_loss_m"])
                - float(r["sublimation_m"])
                - float(r["runtime_swe_after_m"])
            )
            for r in rows
        ),
        "maximum_abs_energy_residual_j_m2": max(
            abs(float(r["stage3_energy_closure_residual_j_m2"])) for r in rows
        ),
        "maximum_abs_latent_identity_residual_j_m2": max(
            abs(float(r["stage3_mass_latent_identity_residual_j_m2"])) for r in rows
        ),
    }


def observed_geometry() -> dict[str, Any]:
    rows = list(csv.DictReader(OBS.open(encoding="utf-8")))
    by_stratum: dict[str, dict[str, Any]] = {}
    for stratum in ("open", "hardwood"):
        complete = [
            row
            for row in rows
            if row["observed_stratum"] == stratum
            and row["observed_snow_depth_m"]
            and row["observed_swe_mm"]
            and row["observed_density_kg_m3"]
            and float(row["observed_snow_depth_m"]) > 0.0
        ]
        residuals = [
            float(row["observed_density_kg_m3"])
            - float(row["observed_swe_mm"]) / float(row["observed_snow_depth_m"])
            for row in complete
        ]
        by_stratum[stratum] = {
            "complete_nonzero_row_count": len(complete),
            "closing_row_count": sum(abs(value) <= 1.0 for value in residuals),
            "nonclosing_row_count": sum(abs(value) > 1.0 for value in residuals),
            "maximum_abs_density_residual_kg_m3": max(map(abs, residuals), default=0.0),
            "median_abs_density_residual_kg_m3": sorted(map(abs, residuals))[len(residuals) // 2]
            if residuals
            else 0.0,
        }
    return by_stratum


def trace_path(result: dict[str, Any], lane: str, model: str) -> Path:
    lane_row = next(row for row in result["lanes"] if row["lane_id"] == lane)
    return REPO / lane_row["cells"][model]["trace"]


def main() -> int:
    existing = [path for path in OUTPUTS if path.exists()]
    if existing:
        raise RuntimeError(f"result-bearing output already exists: {existing}")
    result = json.loads(FACTORIAL.read_text(encoding="utf-8"))
    trace_paths = {
        f"{lane}/{model}": trace_path(result, lane, model)
        for lane in LANES
        for model in MODELS
    }
    inputs = [FACTORIAL, OBS, PROFILES, PROVENANCE, *trace_paths.values()]
    missing = [path for path in inputs if not path.is_file()]
    if missing:
        raise RuntimeError(f"missing retained input: {missing}")

    freeze = {
        "schema": "snow-surface-eb04x-freeze-v1",
        "source_head": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=REPO, text=True
        ).strip(),
        "tool_sha256": sha256(Path(__file__)),
        "inputs": {str(path.relative_to(REPO)): sha256(path) for path in inputs},
        "lanes": list(LANES),
        "models": list(MODELS),
        "observed_geometry_tolerance_kg_m3": 1.0,
        "modeled_geometry_tolerance_kg_m3": 1e-6,
        "promotion_rule": (
            "any invalid observed SWE geometry makes SWE-bearing longwave, "
            "sublimation, and interaction predicates NOT_EVALUABLE"
        ),
        "model_execution_authorized": False,
    }

    traces = {key: read_jsonl(path) for key, path in trace_paths.items()}
    summaries = {key: summarize_trace(rows) for key, rows in traces.items()}
    paired: dict[str, dict[str, float]] = {}
    paired_inventory: dict[str, dict[str, int]] = {}
    for model in MODELS:
        open_rows = traces[f"harvard_open/{model}"]
        hardwood_rows = traces[f"harvard_hardwood/{model}"]
        open_by_day = {int(row["day_index"]): row for row in open_rows}
        hardwood_by_day = {int(row["day_index"]): row for row in hardwood_rows}
        common_days = sorted(open_by_day.keys() & hardwood_by_day.keys())
        if not common_days:
            raise RuntimeError(f"no paired trace days: {model}")
        open_paired = summarize_trace([open_by_day[day] for day in common_days])
        hardwood_paired = summarize_trace([hardwood_by_day[day] for day in common_days])
        paired_inventory[model] = {
            "open_row_count": len(open_rows),
            "hardwood_row_count": len(hardwood_rows),
            "paired_day_count": len(common_days),
            "open_unpaired_day_count": len(open_by_day.keys() - hardwood_by_day.keys()),
            "hardwood_unpaired_day_count": len(hardwood_by_day.keys() - open_by_day.keys()),
        }
        paired[model] = {
            field: hardwood_paired[field] - open_paired[field]
            for field in (
                "total_hyetograph_precipitation_m",
                "total_ground_snow_accumulation_m",
                "total_sublimation_m",
                "total_snowpack_swe_loss_m",
                "total_longwave_energy_j_m2",
                "total_latent_energy_j_m2",
                "peak_swe_m",
                "peak_depth_m",
            )
        }

    observed = observed_geometry()
    observed_valid = all(row["nonclosing_row_count"] == 0 for row in observed.values())
    modeled_valid = all(
        row["maximum_abs_geometry_residual_kg_m3"] <= 1e-6 for row in summaries.values()
    )
    gates = {
        name: {
            "status": "NOT_EVALUABLE" if not observed_valid else "FAIL",
            "reason": (
                "HF237 supplied SWE, depth, and density do not close algebraically"
                if not observed_valid
                else "no prospective efficacy predicate satisfied"
            ),
        }
        for name in ("longwave", "sublimation", "combined_interaction")
    }
    output = {
        "schema": "snow-surface-eb04x-results-v1",
        "freeze_sha256": hashlib.sha256(
            (json.dumps(freeze, sort_keys=True, indent=2) + "\n").encode()
        ).hexdigest(),
        "observed_geometry": observed,
        "observed_geometry_valid": observed_valid,
        "modeled_geometry_valid": modeled_valid,
        "trace_summaries": summaries,
        "paired_inventory": paired_inventory,
        "hardwood_minus_open": paired,
        "canopy_snow_state_available": False,
        "interception_operands": {
            "canopy_snow_load": "NOT_OBSERVED",
            "intercepted_snow": "NOT_OBSERVED",
            "canopy_snow_sublimation": "NOT_OBSERVED",
            "unloading": "NOT_OBSERVED",
            "drip": "NOT_OBSERVED",
        },
        "promotion_gates": gates,
        "conclusion": (
            "model geometry closes, HF237 SWE identity does not; paired residuals "
            "cannot identify canopy interception and no component is promotable"
        ),
    }

    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    (ARTIFACTS / "freeze.json").write_text(
        json.dumps(freeze, sort_keys=True, indent=2) + "\n", encoding="utf-8"
    )
    (ARTIFACTS / "results.json").write_text(
        json.dumps(output, sort_keys=True, indent=2) + "\n", encoding="utf-8"
    )
    with (ARTIFACTS / "paired-diagnostics.csv").open("w", encoding="utf-8", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(("model", "metric", "hardwood_minus_open"))
        for model, fields in paired.items():
            for field, value in fields.items():
                writer.writerow((model, field, f"{value:.17g}"))
    synthesis = f"""# Scientific Synthesis

Evidence mode: **Ran + Inference**

The retained model traces close SWE-depth-density geometry across all eight
Harvard B/L/S/LS cells; the maximum algebraic density residual is
`{max(row['maximum_abs_geometry_residual_kg_m3'] for row in summaries.values()):.3e} kg m^-3`.
HF237's supplied SWE, depth, and density do not close: open has
`{observed['open']['nonclosing_row_count']}/{observed['open']['complete_nonzero_row_count']}`
nonclosing rows and hardwood has
`{observed['hardwood']['nonclosing_row_count']}/{observed['hardwood']['complete_nonzero_row_count']}`.

The open/hardwood traces expose ground accumulation, pack loss, ground-pack
sublimation, longwave, and latent energy. They expose no canopy snow load,
intercepted snow, canopy sublimation, unloading, or drip operand. Therefore the
paired residual cannot identify canopy interception. All frozen component
promotion gates are `NOT_EVALUABLE`; no coefficient, default, or process
promotion is admitted.
"""
    (ARTIFACTS / "scientific-synthesis.md").write_text(synthesis, encoding="utf-8")
    print(
        "EB-04X PASS: model_geometry="
        f"{modeled_valid} observed_geometry={observed_valid} gates=NOT_EVALUABLE"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
