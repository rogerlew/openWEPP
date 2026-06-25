#!/usr/bin/env python3
"""Baseline non-SNOTEL frost sites against the v74 snow/frost rubric."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import observed_harness  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowfreeze-non-snotel-rubric-baseline-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 INV-SNOWFREEZE-050 TOL-SNOWFREEZE-011"
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowfrost_fidelity_i0_non_snotel_rubric_baseline"
DEFAULT_BINARY = REPO_ROOT / "target/release/openwepp-cli-hill"
FROST_DEPTH_ABS_TOL_M = 0.10
FROST_DEPTH_REL_TOL = 0.25


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument(
        "--runtime",
        choices=["direct-production-executor", "compatibility"],
        default="direct-production-executor",
    )
    parser.add_argument(
        "--skip-model-runs",
        action="store_true",
        help="Read existing site_reports instead of running openwepp-cli-hill.",
    )
    args = parser.parse_args(argv)

    run_baseline(
        observations_dir=args.observations_dir.resolve(),
        output_dir=args.output_dir.resolve(),
        binary=args.binary.resolve(),
        runtime=args.runtime,
        skip_model_runs=args.skip_model_runs,
    )
    return 0


def run_baseline(
    observations_dir: Path,
    output_dir: Path,
    binary: Path,
    runtime: str,
    skip_model_runs: bool,
) -> None:
    observed_harness.validate_observations(observations_dir)
    manifest = json.loads((observations_dir / "manifest.json").read_text(encoding="utf-8"))
    output_dir.mkdir(parents=True, exist_ok=True)
    site_reports_dir = output_dir / "site_reports"
    site_reports_dir.mkdir(parents=True, exist_ok=True)

    site_profiles = []
    for site in manifest["sites"]:
        site_id = site["site_id"]
        site_output = site_reports_dir / site_id
        site_output.mkdir(parents=True, exist_ok=True)
        observed_harness.compare_site(
            site_id=site_id,
            observations_dir=observations_dir,
            output_dir=site_output,
            binary=binary,
            no_run=skip_model_runs,
            runtime=runtime,
        )
        report = json.loads((site_output / "comparison_report.json").read_text(encoding="utf-8"))
        observations = load_csv(observations_dir / site["observation_file"])
        modeled = observed_harness.load_modeled_wat(Path(report["wat_output"]))
        site_profiles.append(build_site_profile(site, report, observations, modeled))

    payload = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "runtime": runtime,
        "observations_dir": str(observations_dir.relative_to(REPO_ROOT)),
        "output_dir": str(output_dir.relative_to(REPO_ROOT)),
        "site_count": len(site_profiles),
        "summary": summarize(site_profiles),
        "sites": site_profiles,
    }
    write_json(output_dir / "non_snotel_rubric_baseline.json", payload)
    (output_dir / "non_snotel_rubric_baseline.md").write_text(
        render_markdown(payload), encoding="utf-8"
    )


def build_site_profile(
    site: dict[str, Any],
    report: dict[str, Any],
    observations: list[dict[str, str]],
    modeled: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    rows = paired_rows(observations, modeled)
    snow_pairs = [
        row
        for row in rows
        if row["observed_snow_depth_m"] is not None and row["modeled_snow_depth_m"] is not None
    ]
    frost_tube_pairs = [
        row
        for row in rows
        if row["method"] == "frost_tube"
        and row["censoring"] == "none"
        and row["observed_frost_depth_m"] is not None
    ]
    frost_target_pairs = [
        row
        for row in rows
        if row["censoring"] == "none" and row["observed_frost_or_isotherm_m"] is not None
    ]
    isotherm_pairs = [
        row
        for row in rows
        if row["method"] == "soil_temperature_zero_c_isotherm"
        and row["censoring"] == "none"
        and row["observed_isotherm_depth_m"] is not None
    ]

    cells = snow_cells(snow_pairs, report) + frost_cells(
        rows=rows,
        frost_tube_pairs=frost_tube_pairs,
        frost_target_pairs=frost_target_pairs,
        isotherm_pairs=isotherm_pairs,
    )
    profile = {
        "schema": "snowfrost-fidelity-non-snotel-profile-v1",
        "model_id": "openwepp_current",
        "contract": CONTRACT,
        "paired_count": len(rows),
        "cells": cells,
        "summary": rubric.summarize_rubric_cells(cells),
    }
    return {
        "site_id": site["site_id"],
        "fixture": site["fixture"],
        "source_id": site["source_id"],
        "method": site["method"],
        "runtime": report.get("runtime"),
        "snow_control_status": report.get("snow_control_status"),
        "harness_verdict": report.get("verdict"),
        "comparison_report": str(Path(report["wat_output"]).parent.relative_to(REPO_ROOT) / "comparison_report.json"),
        "metrics": site_metrics(report),
        "rubric_profile": profile,
    }


def snow_cells(pairs: list[dict[str, Any]], report: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        rubric.unavailable_cell(
            "long_term_peak_swe_bias",
            "Long-term",
            "mean peak SWE bias",
            "L",
            "non-SNOTEL frost sites do not provide observed SWE.",
        ),
        rubric.magnitude_bias_cell(
            pairs,
            "long_term_peak_depth_bias",
            "Long-term",
            "mean peak snow-depth bias",
            "L",
            "observed_snow_depth_m",
            "modeled_snow_depth_m",
            "m",
        ),
        rubric.unavailable_cell(
            "long_term_cold_season_bulk_density",
            "Long-term",
            "mean cold-season bulk density",
            "R",
            "non-SNOTEL frost sites do not provide paired SWE/density.",
        ),
        rubric.snow_cover_duration_cell(pairs),
        rubric.unavailable_cell(
            "seasonal_swe_timeseries",
            "Seasonal",
            "SWE trajectory",
            "L",
            "non-SNOTEL frost sites do not provide observed SWE.",
        ),
        rubric.time_series_kge_cell(
            pairs,
            "seasonal_depth_timeseries",
            "Seasonal",
            "snow-depth trajectory",
            "L",
            "observed_snow_depth_m",
            "modeled_snow_depth_m",
        ),
        rubric.unavailable_cell(
            "seasonal_densification_trajectory",
            "Seasonal",
            "densification trajectory rho(t)",
            "R",
            "non-SNOTEL frost sites do not provide paired SWE/density.",
        ),
        rubric.timing_cell(
            pairs,
            "seasonal_accumulation_onset_date",
            "Seasonal",
            "snow accumulation onset date",
            "R",
            rubric.first_snow_date_by_water_year,
        ),
        rubric.timing_cell(
            pairs,
            "seasonal_peak_depth_date",
            "Seasonal",
            "peak snow-depth date",
            "R",
            lambda rows: rubric.peak_date_by_water_year(
                rows, "observed_snow_depth_m", "modeled_snow_depth_m"
            ),
        ),
        rubric.unavailable_cell(
            "seasonal_depth_swe_slope",
            "Seasonal",
            "depth-SWE seasonal slope",
            "R",
            "non-SNOTEL frost sites do not provide observed SWE.",
        ),
        rubric.timing_cell(
            pairs,
            "seasonal_ablation_meltout_date",
            "Seasonal",
            "snow ablation melt-out date",
            "R",
            rubric.last_snow_date_by_water_year,
        ),
        snow_bias_sign_cell(pairs),
        snow_control_gate_cell(report),
        rubric.unavailable_cell(
            "event_new_snow_density",
            "Event",
            "new-snow density per storm",
            "R",
            "non-SNOTEL frost sites lack storm-resolved new-snow density.",
        ),
        rubric.unavailable_cell(
            "event_rain_on_snow_response",
            "Event",
            "rain-on-snow response",
            "R",
            "event pairing requires storm forcing and phase-confidence windows.",
        ),
        rubric.unavailable_cell(
            "cross_cutting_conservation",
            "Cross-cutting",
            "conservation",
            "R",
            "external observation comparison does not reconstruct model mass/energy closure.",
        ),
    ]


def frost_cells(
    rows: list[dict[str, Any]],
    frost_tube_pairs: list[dict[str, Any]],
    frost_target_pairs: list[dict[str, Any]],
    isotherm_pairs: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    return [
        measurement_correspondence_cell(rows),
        frost_isotherm_upper_bound_cell(isotherm_pairs),
        frost_timing_cell(
            frost_target_pairs,
            "frost_onset_timing",
            "Seasonal",
            "frost onset timing",
            first_frost_date_by_water_year,
        ),
        frost_timing_cell(
            frost_target_pairs,
            "frost_thaw_timing",
            "Seasonal",
            "frost thaw timing",
            last_frost_date_by_water_year,
        ),
        frost_duration_cell(frost_target_pairs),
        rubric.magnitude_bias_cell(
            frost_tube_pairs,
            "frost_max_depth_bias",
            "Long-term",
            "max frost depth",
            "L",
            "observed_frost_depth_m",
            "modeled_frdp_m",
            "m",
        ),
        rubric.time_series_kge_cell(
            frost_tube_pairs,
            "frost_depth_timeseries",
            "Seasonal",
            "frost-depth trajectory",
            "L",
            "observed_frost_depth_m",
            "modeled_frdp_m",
        ),
    ]


def paired_rows(
    observations: list[dict[str, str]], modeled: dict[dt.date, dict[str, float | None]]
) -> list[dict[str, Any]]:
    pairs = []
    for observation in observations:
        observed_date = dt.date.fromisoformat(observation["date"])
        modeled_row = modeled.get(observed_date)
        if modeled_row is None:
            continue
        frost_depth = parse_optional_float(observation["observed_frost_depth_m"])
        isotherm_depth = parse_optional_float(observation["observed_isotherm_depth_m"])
        snow_depth = parse_optional_float(observation["observed_snow_depth_m"])
        pairs.append(
            {
                "date": observed_date.isoformat(),
                "date_obj": observed_date,
                "water_year": observed_harness.water_year(observed_date),
                "method": observation["method"],
                "censoring": observation["censoring"],
                "observed_snow_depth_m": snow_depth,
                "modeled_snow_depth_m": modeled_row.get("snow_depth_m"),
                "modeled_snow_water_m": modeled_row.get("snow_water_m"),
                "observed_frost_depth_m": frost_depth,
                "observed_isotherm_depth_m": isotherm_depth,
                "observed_frost_or_isotherm_m": (
                    frost_depth if frost_depth is not None else isotherm_depth
                ),
                "modeled_frdp_m": modeled_row["frdp_m"],
            }
        )
    return pairs


def snow_bias_sign_cell(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    if not pairs:
        return rubric.unavailable_cell(
            "cross_cutting_snow_depth_bias_sign",
            "Cross-cutting",
            "snow-depth bias-sign consistency",
            "R",
            "no paired snow-depth rows.",
        )
    residuals = [
        row["modeled_snow_depth_m"] - row["observed_snow_depth_m"] for row in pairs
    ]
    over = sum(1 for value in residuals if value > 0.0)
    under = sum(1 for value in residuals if value < 0.0)
    dominant = max(over, under) / len(residuals)
    if dominant >= 0.90:
        score = {"ordinal": 0, "label": "fail"}
    elif dominant >= 0.75:
        score = {"ordinal": 1, "label": "marginal"}
    elif dominant >= 0.60:
        score = {"ordinal": 2, "label": "pass"}
    else:
        score = {"ordinal": 3, "label": "strong"}
    return rubric.scored_cell(
        cell_id="cross_cutting_snow_depth_bias_sign",
        timescale="Cross-cutting",
        signature="snow-depth bias-sign consistency",
        tier="R",
        metric="dominant residual sign fraction",
        metric_units="unitless",
        score=score,
        metrics={
            "paired_count": len(residuals),
            "modeled_over_observed_count": over,
            "modeled_under_observed_count": under,
            "dominant_fraction": dominant,
            "mean_signed_bias_m": sum(residuals) / len(residuals),
        },
    )


def snow_control_gate_cell(report: dict[str, Any]) -> dict[str, Any]:
    status = report.get("snow_control_status")
    metrics = report.get("metrics", {})
    if status == observed_harness.SNOW_CONTROL_PASSED:
        score = {"ordinal": 3, "label": "strong"}
    elif status == observed_harness.SNOW_CONTROL_FAILED:
        score = {"ordinal": 0, "label": "fail"}
    else:
        return rubric.unavailable_cell(
            "cross_cutting_snow_control_gate",
            "Cross-cutting",
            "snow-control gate for frost attribution",
            "R",
            f"snow-control status {status} cannot score frost attribution.",
        )
    return rubric.scored_cell(
        cell_id="cross_cutting_snow_control_gate",
        timescale="Cross-cutting",
        signature="snow-control gate for frost attribution",
        tier="R",
        metric="TOL-SNOWFREEZE-009 paired snow-depth gate",
        metric_units="status",
        score=score,
        metrics={
            "snow_control_status": status,
            "paired_count": metrics.get("snow_depth_control_count"),
            "fail_count": metrics.get("snow_depth_control_fail_count"),
            "mean_signed_snow_depth_residual_m": metrics.get(
                "mean_signed_snow_depth_residual_m"
            ),
            "max_abs_snow_depth_residual_m": metrics.get(
                "max_abs_snow_depth_residual_m"
            ),
        },
    )


def measurement_correspondence_cell(rows: list[dict[str, Any]]) -> dict[str, Any]:
    if not rows:
        return rubric.unavailable_cell(
            "frost_measurement_correspondence",
            "Cross-cutting",
            "measurement correspondence",
            "R",
            "no matched observation rows.",
        )
    methods = sorted({row["method"] for row in rows})
    recognized = all(
        method in {"frost_tube", "soil_temperature_zero_c_isotherm"} for method in methods
    )
    score = {"ordinal": 3, "label": "strong"} if recognized else {"ordinal": 0, "label": "fail"}
    return rubric.scored_cell(
        cell_id="frost_measurement_correspondence",
        timescale="Cross-cutting",
        signature="measurement correspondence",
        tier="R",
        metric="recognized INV-SNOWFREEZE-047 method mapping",
        metric_units="status",
        score=score,
        metrics={
            "matched_count": len(rows),
            "methods": methods,
            "recognized": recognized,
        },
    )


def frost_isotherm_upper_bound_cell(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    if not pairs:
        return rubric.unavailable_cell(
            "frost_isotherm_upper_bound",
            "Cross-cutting",
            "frdp <= 0 degC isotherm upper bound",
            "R",
            "site does not provide non-censored isotherm observations.",
        )
    margins = [row["modeled_frdp_m"] - row["observed_isotherm_depth_m"] for row in pairs]
    exceedances = [
        margin
        for margin, row in zip(margins, pairs)
        if margin > frost_depth_tolerance_m(row["observed_isotherm_depth_m"])
    ]
    fraction = len(exceedances) / len(pairs)
    if len(exceedances) == 0:
        score = {"ordinal": 3, "label": "strong"}
    elif fraction <= 0.05:
        score = {"ordinal": 2, "label": "pass"}
    elif fraction <= 0.15:
        score = {"ordinal": 1, "label": "marginal"}
    else:
        score = {"ordinal": 0, "label": "fail"}
    return rubric.scored_cell(
        cell_id="frost_isotherm_upper_bound",
        timescale="Cross-cutting",
        signature="frdp <= 0 degC isotherm upper bound",
        tier="R",
        metric="fraction exceeding isotherm + TOL-SNOWFREEZE-007",
        metric_units="unitless",
        score=score,
        metrics={
            "paired_count": len(pairs),
            "exceedance_count": len(exceedances),
            "exceedance_fraction": fraction,
            "max_margin_m": max(margins),
            "median_margin_m": rubric.median(margins),
        },
    )


def frost_timing_cell(
    pairs: list[dict[str, Any]],
    cell_id: str,
    timescale: str,
    signature: str,
    extractor: Any,
) -> dict[str, Any]:
    offsets = extractor(pairs)
    if not offsets:
        return rubric.unavailable_cell(cell_id, timescale, signature, "R", "no paired timing offsets")
    values = [row["offset_days"] for row in offsets]
    return rubric.scored_cell(
        cell_id=cell_id,
        timescale=timescale,
        signature=signature,
        tier="R",
        metric="median modeled-minus-observed date offset",
        metric_units="days",
        score=rubric.score_timing(abs(rubric.median(values) or 0.0)),
        metrics={
            "annual_count": len(offsets),
            "median_offset_days": rubric.median(values),
            "iqr_offset_days": rubric.iqr(values),
            "annual_offsets": offsets,
        },
    )


def frost_duration_cell(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    durations = []
    for water_year_value, rows in rubric.pairs_by_water_year(pairs).items():
        observed = sum(1 for row in rows if row["observed_frost_or_isotherm_m"] > 0.0)
        modeled = sum(1 for row in rows if row["modeled_frdp_m"] > 0.0)
        if observed <= 0:
            continue
        durations.append(
            {
                "water_year": water_year_value,
                "observed_frozen_observation_days": observed,
                "modeled_frozen_observation_days": modeled,
                "residual_observation_days": modeled - observed,
            }
        )
    if not durations:
        return rubric.unavailable_cell(
            "frost_frozen_duration",
            "Seasonal",
            "frozen duration",
            "R",
            "no positive observed frozen-duration years.",
        )
    residuals = [row["residual_observation_days"] for row in durations]
    return rubric.scored_cell(
        cell_id="frost_frozen_duration",
        timescale="Seasonal",
        signature="frozen duration",
        tier="R",
        metric="median modeled-minus-observed frozen observation days",
        metric_units="days",
        score=rubric.score_timing(abs(rubric.median(residuals) or 0.0)),
        metrics={
            "annual_count": len(durations),
            "median_residual_observation_days": rubric.median(residuals),
            "iqr_residual_observation_days": rubric.iqr(residuals),
            "annual_durations": durations,
        },
    )


def first_frost_date_by_water_year(pairs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return frost_date_offsets(pairs, first=True)


def last_frost_date_by_water_year(pairs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return frost_date_offsets(pairs, first=False)


def frost_date_offsets(pairs: list[dict[str, Any]], first: bool) -> list[dict[str, Any]]:
    offsets = []
    for water_year_value, rows in rubric.pairs_by_water_year(pairs).items():
        ordered = sorted(rows, key=lambda row: row["date_obj"], reverse=not first)
        observed = next(
            (row["date_obj"] for row in ordered if row["observed_frost_or_isotherm_m"] > 0.0),
            None,
        )
        modeled = next(
            (row["date_obj"] for row in ordered if row["modeled_frdp_m"] > 0.0),
            None,
        )
        if observed is None or modeled is None:
            continue
        offsets.append(
            {
                "water_year": water_year_value,
                "observed_date": observed.isoformat(),
                "modeled_date": modeled.isoformat(),
                "offset_days": (modeled - observed).days,
            }
        )
    return offsets


def site_metrics(report: dict[str, Any]) -> dict[str, Any]:
    metrics = report.get("metrics", {})
    return {
        key: metrics.get(key)
        for key in [
            "observation_count",
            "matched_count",
            "snow_depth_control_count",
            "snow_depth_control_fail_count",
            "mean_signed_snow_depth_residual_m",
            "max_abs_snow_depth_residual_m",
            "frost_depth_residual_count",
            "max_abs_residual_m",
            "isotherm_upper_bound_count",
            "isotherm_upper_bound_exceedance_count",
            "max_isotherm_upper_bound_margin_m",
        ]
    }


def summarize(site_profiles: list[dict[str, Any]]) -> dict[str, Any]:
    rubric_counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    snow_status_counts: dict[str, int] = {}
    for site in site_profiles:
        status = site["snow_control_status"]
        snow_status_counts[status] = snow_status_counts.get(status, 0) + 1
        for cell in site["rubric_profile"]["cells"]:
            label = cell["ordinal_label"]
            rubric_counts[label] = rubric_counts.get(label, 0) + 1
            if cell["tier"] == "R":
                robust_counts[label] = robust_counts.get(label, 0) + 1
    return {
        "rubric_counts_by_label": dict(sorted(rubric_counts.items())),
        "forcing_robust_rubric_counts_by_label": dict(sorted(robust_counts.items())),
        "snow_control_status_counts": dict(sorted(snow_status_counts.items())),
        "openwepp_defective_cells": 0,
        "rubric_profile_not_scalar": True,
        "next_route": "snow-depth structural remediation before frost physics attribution",
        "production_physics_changed": False,
    }


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Non-SNOTEL Snow/Frost Rubric Baseline",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{payload['schema']}`",
        f"- Contract: `{payload['contract']}`",
        f"- Runtime: `{payload['runtime']}`",
        f"- Site count: `{payload['site_count']}`",
        f"- Rubric counts: `{payload['summary']['rubric_counts_by_label']}`",
        f"- Forcing-robust rubric counts: `{payload['summary']['forcing_robust_rubric_counts_by_label']}`",
        f"- Snow-control status counts: `{payload['summary']['snow_control_status_counts']}`",
        f"- OpenWEPP defective cells: `{payload['summary']['openwepp_defective_cells']}`",
        f"- Next route: `{payload['summary']['next_route']}`",
        "",
        "## Site Profile Summary",
        "",
        "| Site | Method | Snow control | Robust counts | Key blockers | Frost residual rows | Isotherm rows |",
        "| --- | --- | --- | --- | --- | ---: | ---: |",
    ]
    for site in payload["sites"]:
        summary = site["rubric_profile"]["summary"]
        blockers = key_blockers(site)
        lines.append(
            "| {site_id} | {method} | {snow} | {robust} | {blockers} | {frost_rows} | {iso_rows} |".format(
                site_id=site["site_id"],
                method=site["method"],
                snow=site["snow_control_status"],
                robust=rubric.rubric_counts_text(summary["forcing_robust_counts_by_label"]),
                blockers=", ".join(blockers) if blockers else "none",
                frost_rows=site["metrics"]["frost_depth_residual_count"] or 0,
                iso_rows=site["metrics"]["isotherm_upper_bound_count"] or 0,
            )
        )
    lines.extend(
        [
            "",
            "## Disposition",
            "",
            "- This is a baseline profile, not a remediation.",
            "- Snow-control failures or unavailable paired snow observations remain blockers before frost attribution.",
            "- SWE, density, event, and conservation cells unavailable for this non-SNOTEL corpus are explicit unavailable cells.",
            "- `OPENWEPP-DEFECTIVE` remains `0` because ADR-0017 requires independent correctness authority, not observation disagreement alone.",
            "",
        ]
    )
    return "\n".join(lines)


def key_blockers(site: dict[str, Any]) -> list[str]:
    blockers = []
    for cell in site["rubric_profile"]["cells"]:
        if cell["ordinal_label"] == "fail" and cell["tier"] == "R":
            blockers.append(cell["cell_id"])
    return blockers[:5]


def frost_depth_tolerance_m(observed_depth_m: float) -> float:
    return max(FROST_DEPTH_ABS_TOL_M, abs(observed_depth_m) * FROST_DEPTH_REL_TOL)


def parse_optional_float(value: str | None) -> float | None:
    if value is None or value == "":
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def load_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    raise SystemExit(main())
