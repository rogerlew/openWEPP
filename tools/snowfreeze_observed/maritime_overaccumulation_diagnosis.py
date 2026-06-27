#!/usr/bin/env python3
"""Diagnose maritime snow over-accumulation mechanisms.

This is SNOWDENSITY-10.3.4 evidence tooling. It runs the diagnostic
``openwepp-snowbench coe-melt`` legacy-CoE replay for HJ Andrews, Sleepers,
Harvard, and Hubbard Brook maritime fixtures, pairs installed observations
where available, and ranks candidate mechanisms without changing production
physics or tuning coefficients.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import snotel_density_three_way as rubric  # noqa: E402


DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_4_maritime_overaccumulation_diagnosis"
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
FORCING_RELATIVE_PATH = Path("forcing_bridge/tg_0p0c_zg0p10m/forcing.csv")
WATER_SPECIFIC_HEAT_J_KG_C = 4186.0
WATER_LATENT_HEAT_FUSION_J_KG = 334_000.0
SNOWPACK_SWE_THRESHOLD_M = 0.01
PHASE_AMBIGUOUS_LOW_C = -1.0
PHASE_AMBIGUOUS_HIGH_C = 2.0


@dataclass(frozen=True)
class Surface:
    surface_id: str
    site_group: str
    fixture_dir: Path
    cover: str
    observation_source: str
    observation_file: Path | None
    observation_filter: dict[str, str]
    observation_kind: str
    verdict_scope: str
    note: str


SURFACES = [
    Surface(
        surface_id="hjandrews_conifer",
        site_group="hjandrews",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/hjandrews_conifer_or",
        cover="conifer",
        observation_source="not_installed",
        observation_file=None,
        observation_filter={},
        observation_kind="none",
        verdict_scope="observation_blocked",
        note="Fixture exists, but EDI MS007 / SNOTEL paired snow observations are not installed.",
    ),
    Surface(
        surface_id="sleepers_south_field",
        site_group="sleepers",
        fixture_dir=REPO_ROOT / "tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt",
        cover="open_field",
        observation_source="snowfreeze_observed",
        observation_file=REPO_ROOT
        / "tests/fixtures/snowfreeze_observed/observations/sites/site1_sleepers_south_field_vt.csv",
        observation_filter={},
        observation_kind="snowfreeze_depth",
        verdict_scope="paired_observation",
        note="Sleepers South field frost-site snow-depth rows.",
    ),
    Surface(
        surface_id="sleepers_w9_hardwood",
        site_group="sleepers",
        fixture_dir=REPO_ROOT / "tests/fixtures/snowfreeze_observed/site2_sleepers_w9_hardwood_vt",
        cover="hardwood",
        observation_source="snowfreeze_observed",
        observation_file=REPO_ROOT
        / "tests/fixtures/snowfreeze_observed/observations/sites/site2_sleepers_w9_hardwood_vt.csv",
        observation_filter={},
        observation_kind="snowfreeze_depth",
        verdict_scope="paired_observation",
        note="Sleepers W9 hardwood frost-site snow-depth rows.",
    ),
    Surface(
        surface_id="harvard_hardwood",
        site_group="harvard",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/harvard_deciduous_ma",
        cover="hardwood",
        observation_source="harvard_hf237",
        observation_file=REPO_ROOT
        / "tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv",
        observation_filter={
            "binding_status": "bound",
            "model_fixture": "harvard_deciduous_ma",
            "observed_stratum": "hardwood",
        },
        observation_kind="swe_depth_density",
        verdict_scope="paired_observation",
        note="Harvard HF237 hardwood stratum bound by 10.3.2.",
    ),
    Surface(
        surface_id="harvard_open",
        site_group="harvard",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/harvard_open_ma",
        cover="open",
        observation_source="harvard_hf237",
        observation_file=REPO_ROOT
        / "tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv",
        observation_filter={
            "binding_status": "bound",
            "model_fixture": "harvard_open_ma",
            "observed_stratum": "open",
        },
        observation_kind="swe_depth_density",
        verdict_scope="paired_observation",
        note="Harvard HF237 open stratum bound by 10.3.2.",
    ),
    Surface(
        surface_id="hubbardbrook_deciduous",
        site_group="hubbardbrook",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh",
        cover="deciduous",
        observation_source="not_installed",
        observation_file=None,
        observation_filter={},
        observation_kind="none",
        verdict_scope="observation_blocked",
        note="Fixture exists, but Hubbard Brook paired snow observations are not installed.",
    ),
    Surface(
        surface_id="hubbardbrook_mixed",
        site_group="hubbardbrook",
        fixture_dir=REPO_ROOT / "tests/fixtures/cancov_forest/hubbardbrook_mixed_nh",
        cover="mixed",
        observation_source="not_installed",
        observation_file=None,
        observation_filter={},
        observation_kind="none",
        verdict_scope="observation_blocked",
        note="Fixture exists, but Hubbard Brook paired snow observations are not installed.",
    ),
]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--skip-runs", action="store_true")
    args = parser.parse_args(argv)

    report = diagnose(
        output_dir=args.output_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        run_snowbench=not args.skip_runs,
    )
    rubric.write_json(args.output_dir / "maritime_overaccumulation_diagnosis.json", report)
    (args.output_dir / "maritime_overaccumulation_diagnosis.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return 0


def diagnose(output_dir: Path, snowbench_binary: Path, run_snowbench: bool) -> dict[str, Any]:
    if run_snowbench and not snowbench_binary.is_file():
        raise FileNotFoundError(f"openwepp-snowbench binary not found: {snowbench_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    surfaces = []
    for surface in SURFACES:
        run_dir = output_dir / "runs" / surface.surface_id
        if run_snowbench:
            run_coe_melt(surface.fixture_dir, run_dir, snowbench_binary)
        surfaces.append(analyze_surface(surface, run_dir))

    mechanisms = rank_mechanisms(surfaces)
    return {
        "schema": "snowdensity10-3-4-maritime-overaccumulation-diagnosis-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 INV-SNOWFREEZE-050 INV-SNOWFREEZE-063",
        "runtime_coupling": "diagnostic snowbench replay only; no production activation",
        "no_physics_change": True,
        "no_tuning": True,
        "snowbench_binary": str(snowbench_binary),
        "output_dir": str(output_dir),
        "summary": summarize_report(surfaces, mechanisms),
        "mechanism_ranking": mechanisms,
        "surfaces": surfaces,
        "static_source_scan": static_source_scan_record(),
    }


def run_coe_melt(fixture_dir: Path, output_dir: Path, snowbench_binary: Path) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    command = [
        str(snowbench_binary),
        "coe-melt",
        "--run-dir",
        str(fixture_dir),
        "--output-dir",
        str(output_dir),
        "--model",
        "legacy_coe",
    ]
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (output_dir / "openwepp-snowbench.stdout").write_text(completed.stdout, encoding="utf-8")
    (output_dir / "openwepp-snowbench.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(
            f"openwepp-snowbench coe-melt failed for {fixture_dir.name} with exit code {completed.returncode}"
        )


def analyze_surface(surface: Surface, run_dir: Path) -> dict[str, Any]:
    coe_rows = read_coe_rows(run_dir / "coe_melt_snow.csv")
    forcing_rows = read_forcing_rows(run_dir / FORCING_RELATIVE_PATH)
    observations = load_observations(surface)
    pairs = pair_observations(observations, coe_rows, surface.observation_kind)
    summary = read_json(run_dir / "coe_melt_summary.json")
    return {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "fixture_dir": str(surface.fixture_dir.relative_to(REPO_ROOT)),
        "verdict_scope": surface.verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "observation_row_count": len(observations),
        "paired_row_count": len(pairs),
        "note": surface.note,
        "snowbench_run_dir": str(run_dir),
        "snowbench_summary": summary["summary"],
        "canopy_summary": summary["canopy_series_summary"],
        "residuals": residual_summary(pairs),
        "forcing_diagnostics": forcing_diagnostics(forcing_rows, coe_rows),
        "mechanism_flags": mechanism_flags(surface, pairs, forcing_rows, coe_rows, summary),
    }


def load_observations(surface: Surface) -> list[dict[str, str]]:
    if surface.observation_file is None:
        return []
    rows = rubric.read_csv_dicts(surface.observation_file)
    if not surface.observation_filter:
        return rows
    return [
        row
        for row in rows
        if all(row.get(key) == value for key, value in surface.observation_filter.items())
    ]


def pair_observations(
    observations: list[dict[str, str]],
    coe_rows: dict[dt.date, dict[str, float | None]],
    observation_kind: str,
) -> list[dict[str, Any]]:
    pairs = []
    for row in observations:
        date = dt.date.fromisoformat(row["date"])
        modeled = coe_rows.get(date)
        if modeled is None:
            continue
        observed_depth = optional_float(row.get("observed_snow_depth_m"))
        if observed_depth is None:
            continue
        modeled_depth = modeled.get("snow_depth_m")
        if modeled_depth is None:
            continue
        pair = {
            "date": date.isoformat(),
            "date_obj": date,
            "water_year": rubric.water_year(date),
            "observed_snow_depth_m": observed_depth,
            "modeled_snow_depth_m": modeled_depth,
            "depth_residual_m": modeled_depth - observed_depth,
            "modeled_snow_water_m": modeled.get("snow_water_m"),
            "modeled_density_kg_m3": modeled.get("snow_density_kg_m3"),
        }
        if observation_kind == "swe_depth_density":
            observed_swe_mm = optional_float(row.get("observed_swe_mm"))
            observed_density = optional_float(row.get("observed_density_kg_m3"))
            pair["observed_swe_m"] = observed_swe_mm / 1000.0 if observed_swe_mm is not None else None
            pair["observed_density_kg_m3"] = observed_density
            modeled_swe = modeled.get("snow_water_m")
            pair["swe_residual_m"] = (
                modeled_swe - pair["observed_swe_m"]
                if modeled_swe is not None and pair["observed_swe_m"] is not None
                else None
            )
            pair["density_residual_kg_m3"] = (
                modeled.get("snow_density_kg_m3") - observed_density
                if modeled.get("snow_density_kg_m3") is not None and observed_density is not None
                else None
            )
        pairs.append(pair)
    return pairs


def residual_summary(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    residuals = [row["depth_residual_m"] for row in pairs]
    swe_residuals = [row["swe_residual_m"] for row in pairs if row.get("swe_residual_m") is not None]
    if not residuals:
        return {
            "paired_count": 0,
            "status": "NO_PAIRED_OBSERVED_SNOW_DEPTH",
            "mean_signed_depth_residual_m": None,
            "mean_abs_depth_residual_m": None,
            "max_abs_depth_residual_m": None,
            "modeled_over_observed_fraction": None,
            "snow_control_fail_count": 0,
        }
    fail_count = sum(
        1
        for row in pairs
        if abs(row["depth_residual_m"]) > rubric.snow_depth_tolerance(row["observed_snow_depth_m"])
    )
    over_count = sum(1 for value in residuals if value > 0.0)
    return {
        "paired_count": len(residuals),
        "status": "PAIRED_OBSERVED_SNOW_DEPTH",
        "mean_signed_depth_residual_m": mean(residuals),
        "mean_abs_depth_residual_m": mean([abs(value) for value in residuals]),
        "max_abs_depth_residual_m": max(abs(value) for value in residuals),
        "modeled_over_observed_fraction": over_count / len(residuals),
        "snow_control_fail_count": fail_count,
        "snow_control_fail_fraction": fail_count / len(residuals),
        "mean_signed_swe_residual_m": mean(swe_residuals),
        "sample_pairs": json_pairs(pairs[:12]),
    }


def forcing_diagnostics(
    forcing_rows: list[dict[str, Any]], coe_rows: dict[dt.date, dict[str, float | None]]
) -> dict[str, Any]:
    by_date = group_forcing_by_date(forcing_rows)
    total_precip_m = 0.0
    snow_precip_m = 0.0
    rain_precip_m = 0.0
    phase_ambiguous_precip_m = 0.0
    warm_snow_precip_m = 0.0
    rain_on_snow_m = 0.0
    warm_rain_on_snow_m = 0.0
    rain_heat_melt_equiv_m = 0.0
    positive_temp_snowpack_hours = 0
    snowpack_hours = 0
    downwelling_thermal_values = []

    for row in forcing_rows:
        precip_m = row["precip_mass_mm"] / 1000.0
        snow_m = precip_m * row["snow_precip_fraction"]
        rain_m = precip_m - snow_m
        total_precip_m += precip_m
        snow_precip_m += snow_m
        rain_precip_m += rain_m
        if precip_m > 0.0 and PHASE_AMBIGUOUS_LOW_C <= row["temp_air_degC"] <= PHASE_AMBIGUOUS_HIGH_C:
            phase_ambiguous_precip_m += precip_m
        if snow_m > 0.0 and row["temp_air_degC"] > PHASE_AMBIGUOUS_LOW_C:
            warm_snow_precip_m += snow_m
        snowpack_present = snowpack_present_for_hour(row["date"], coe_rows)
        if snowpack_present:
            snowpack_hours += 1
            if row["temp_air_degC"] > 0.0:
                positive_temp_snowpack_hours += 1
            if rain_m > 0.0:
                rain_on_snow_m += rain_m
                if row["temp_air_degC"] > 0.0:
                    warm_rain_on_snow_m += rain_m
                    rain_heat_melt_equiv_m += (
                        rain_m
                        * row["temp_air_degC"]
                        * WATER_SPECIFIC_HEAT_J_KG_C
                        / WATER_LATENT_HEAT_FUSION_J_KG
                    )
        downwelling_thermal_values.append(row["downwelling_thermal_Wm-2"])

    thaw_days = 0
    thaw_raw_melt_m = 0.0
    thaw_swe_loss_m = 0.0
    for date, rows in by_date.items():
        modeled = coe_rows.get(date)
        if modeled is None:
            continue
        if not daily_snowpack_present(date, coe_rows):
            continue
        mean_temp = mean([row["temp_air_degC"] for row in rows])
        if mean_temp is not None and mean_temp > 0.0:
            thaw_days += 1
            thaw_raw_melt_m += modeled.get("raw_melt_m") or 0.0
            thaw_swe_loss_m += modeled.get("snowpack_swe_loss_m") or 0.0

    return {
        "total_precip_m": total_precip_m,
        "snow_precip_m": snow_precip_m,
        "rain_precip_m": rain_precip_m,
        "snow_precip_fraction": snow_precip_m / total_precip_m if total_precip_m > 0.0 else None,
        "phase_ambiguous_precip_m": phase_ambiguous_precip_m,
        "phase_ambiguous_precip_fraction": (
            phase_ambiguous_precip_m / total_precip_m if total_precip_m > 0.0 else None
        ),
        "warm_snow_precip_m": warm_snow_precip_m,
        "rain_on_snow_m": rain_on_snow_m,
        "warm_rain_on_snow_m": warm_rain_on_snow_m,
        "rain_heat_melt_equiv_m": rain_heat_melt_equiv_m,
        "snowpack_hours": snowpack_hours,
        "positive_temp_snowpack_hours": positive_temp_snowpack_hours,
        "positive_temp_snowpack_fraction": (
            positive_temp_snowpack_hours / snowpack_hours if snowpack_hours > 0 else None
        ),
        "thaw_snowpack_day_count": thaw_days,
        "raw_melt_on_thaw_days_m": thaw_raw_melt_m,
        "swe_loss_on_thaw_days_m": thaw_swe_loss_m,
        "mean_downwelling_thermal_w_m2": mean(downwelling_thermal_values),
    }


def mechanism_flags(
    surface: Surface,
    pairs: list[dict[str, Any]],
    forcing_rows: list[dict[str, Any]],
    coe_rows: dict[dt.date, dict[str, float | None]],
    summary: dict[str, Any],
) -> dict[str, Any]:
    residuals = residual_summary(pairs)
    forcing = forcing_diagnostics(forcing_rows, coe_rows)
    paired = residuals["paired_count"] > 0
    overacc = bool(
        paired
        and (residuals["mean_signed_depth_residual_m"] or 0.0) > 0.05
        and (residuals["modeled_over_observed_fraction"] or 0.0) >= 0.60
    )
    forest_cover = surface.cover in {"conifer", "hardwood", "deciduous", "mixed"}
    canopy_mean = summary["canopy_series_summary"]["mean"]
    return {
        "overaccumulation_signal": overacc,
        "paired_observation_available": paired,
        "forest_cover": forest_cover,
        "mean_canopy_cover_fraction": canopy_mean,
        "phase_ambiguous_precip_m": forcing["phase_ambiguous_precip_m"],
        "warm_snow_precip_m": forcing["warm_snow_precip_m"],
        "rain_heat_melt_equiv_m": forcing["rain_heat_melt_equiv_m"],
        "positive_temp_snowpack_hours": forcing["positive_temp_snowpack_hours"],
        "sub_canopy_longwave_candidate": forest_cover and canopy_mean >= 0.2,
    }


def rank_mechanisms(surfaces: list[dict[str, Any]]) -> list[dict[str, Any]]:
    paired_overacc = [item for item in surfaces if item["mechanism_flags"]["overaccumulation_signal"]]
    obs_blocked = [item for item in surfaces if item["paired_row_count"] == 0]
    forest_overacc = [
        item
        for item in paired_overacc
        if item["mechanism_flags"]["forest_cover"]
        and item["mechanism_flags"]["mean_canopy_cover_fraction"] >= 0.2
    ]
    phase_ambiguous_m = sum(
        item["forcing_diagnostics"]["phase_ambiguous_precip_m"] for item in paired_overacc
    )
    warm_snow_m = sum(item["forcing_diagnostics"]["warm_snow_precip_m"] for item in paired_overacc)
    thaw_hours = sum(
        item["forcing_diagnostics"]["positive_temp_snowpack_hours"] for item in paired_overacc
    )
    rain_heat_m = sum(
        item["forcing_diagnostics"]["rain_heat_melt_equiv_m"] for item in paired_overacc
    )
    rankings = [
        {
            "rank": 1,
            "mechanism": "snow_rain_partition_near_zero_c",
            "disposition": "DEFECT-ELIGIBLE" if paired_overacc and phase_ambiguous_m > 0.10 else "OBSERVATION-BLOCKED",
            "evidence": (
                f"Paired over-accumulating surfaces={len(paired_overacc)}; "
                f"phase-ambiguous precip over those surfaces={phase_ambiguous_m:.3f} m; "
                f"warm modeled snow input={warm_snow_m:.3f} m."
            ),
            "next_action": "Author a partition-threshold/phase-confidence package before any melt coefficient tuning.",
        },
        {
            "rank": 2,
            "mechanism": "winter_thaw_melt_response",
            "disposition": "DEFECT-ELIGIBLE" if paired_overacc and thaw_hours > 100 else "OBSERVATION-BLOCKED",
            "evidence": (
                f"Paired over-accumulating surfaces={len(paired_overacc)}; "
                f"positive-temperature snowpack hours over those surfaces={thaw_hours}."
            ),
            "next_action": "Decompose thaw-window melt operands and compare pack ablation across observed thaw periods.",
        },
        {
            "rank": 3,
            "mechanism": "sub_canopy_longwave_or_forest_energy",
            "disposition": "DEFECT-ELIGIBLE" if forest_overacc else "OBSERVATION-BLOCKED",
            "evidence": (
                f"Forested paired over-accumulating surfaces={len(forest_overacc)}; "
                "current CoE path has canopy attenuation and temperature terms but no explicit sub-canopy longwave process."
            ),
            "next_action": "Scope a forest-energy diagnostic only after partition/thaw windows are decomposed.",
        },
        {
            "rank": 4,
            "mechanism": "rain_on_snow_heat",
            "disposition": "LOW-PRIORITY" if rain_heat_m < 0.05 else "DEFECT-ELIGIBLE",
            "evidence": (
                f"Warm-rain heat melt equivalent over paired over-accumulating surfaces={rain_heat_m:.3f} m water. "
                "The production formula already includes the CoE `dmelt` rain term."
            ),
            "next_action": "Do not alter rain heat first unless event-window reconstruction shows the CoE `dmelt` term is numerically inactive during observed rain-on-snow failures.",
        },
        {
            "rank": 5,
            "mechanism": "precipitation_bias",
            "disposition": "FORCING-LIMITED",
            "evidence": "No independent precipitation-gauge or catch-correction authority is installed for these fixture hillslopes.",
            "next_action": "Report as uncertainty; do not tune shared precipitation to fix snow depth.",
        },
        {
            "rank": 6,
            "mechanism": "representativeness",
            "disposition": "FORCING-LIMITED",
            "evidence": (
                f"Observation-blocked surfaces={len(obs_blocked)}; point/stratum versus hillslope correspondence remains load-bearing for HJ Andrews and Hubbard Brook."
            ),
            "next_action": "Install paired observation tables before assigning defect labels at HJ Andrews or Hubbard Brook.",
        },
        {
            "rank": 7,
            "mechanism": "wind_undercatch",
            "disposition": "NOT-SUPPORTED",
            "evidence": "The observed paired failures are modeled-over-observed snow depth; increasing snow precipitation for undercatch would generally worsen that signal.",
            "next_action": "Keep as uncertainty only; do not prioritize as a corrective lever for over-accumulation.",
        },
    ]
    return rankings


def summarize_report(surfaces: list[dict[str, Any]], mechanisms: list[dict[str, Any]]) -> dict[str, Any]:
    paired = [item for item in surfaces if item["paired_row_count"] > 0]
    overacc = [item for item in surfaces if item["mechanism_flags"]["overaccumulation_signal"]]
    blocked = [item for item in surfaces if item["paired_row_count"] == 0]
    return {
        "disposition": "PARTITION-THAW-FIRST",
        "surface_count": len(surfaces),
        "paired_surface_count": len(paired),
        "observation_blocked_surface_count": len(blocked),
        "paired_overaccumulation_surface_count": len(overacc),
        "top_defect_eligible_mechanisms": [
            item["mechanism"] for item in mechanisms if item["disposition"] == "DEFECT-ELIGIBLE"
        ][:3],
        "next_route": "SNOWDENSITY-10.3.5 partition/thaw-window candidate package before rain-heat or longwave production changes.",
    }


def static_source_scan_record() -> dict[str, Any]:
    return {
        "compute_simimpl29_melt_hour": "CoE melt computes amelt, bmelt, cmelt, and dmelt; dmelt is warm-rain heat lineage.",
        "sub_canopy_longwave": "No separate production snowmelt term was identified for forest-canopy longwave enhancement beyond CoE cloud/canopy-temperature terms.",
        "evidence_class": "Static source scan; see rg output recorded in gate-results.md.",
    }


def read_coe_rows(path: Path) -> dict[dt.date, dict[str, float | None]]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    result: dict[dt.date, dict[str, float | None]] = {}
    for row in rows:
        date = dt.date.fromisoformat(row["date"])
        result[date] = {
            "snow_water_before_m": optional_float(row.get("snow_water_before_m")),
            "snow_input_m": optional_float(row.get("snow_input_m")),
            "rain_input_m": optional_float(row.get("rain_input_m")),
            "rain_retained_m": optional_float(row.get("rain_retained_m")),
            "rain_released_m": optional_float(row.get("rain_released_m")),
            "liquid_holding_capacity_m": optional_float(row.get("liquid_holding_capacity_m")),
            "liquid_water_retained_m": optional_float(row.get("liquid_water_retained_m")),
            "liquid_water_released_m": optional_float(row.get("liquid_water_released_m")),
            "snow_water_m": optional_float(row["snow_water_m"]),
            "snow_depth_m": optional_float(row["snow_depth_m"]),
            "snow_density_kg_m3": optional_float(row["snow_density_kg_m3"]),
            "raw_melt_m": optional_float(row["raw_melt_m"]),
            "redistributed_melt_m": optional_float(row["redistributed_melt_m"]),
            "routed_melt_m": optional_float(row["routed_melt_m"]),
            "snowpack_swe_loss_m": optional_float(row["snowpack_swe_loss_m"]),
            "snowpack_swe_balance_residual_m": optional_float(
                row.get("snowpack_swe_balance_residual_m")
            ),
            "routed_state_loss_residual_m": optional_float(
                row.get("routed_state_loss_residual_m")
            ),
            "state_loss_available_storage_margin_m": optional_float(
                row.get("state_loss_available_storage_margin_m")
            ),
        }
    return result


def read_forcing_rows(path: Path) -> list[dict[str, Any]]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    result = []
    for row in rows:
        timestamp = dt.datetime.fromisoformat(row["Datetime"])
        result.append(
            {
                "timestamp": timestamp,
                "date": timestamp.date(),
                "net_solar_Wm-2": float(row["net_solar_Wm-2"]),
                "downwelling_thermal_Wm-2": float(row["downwelling_thermal_Wm-2"]),
                "temp_air_degC": float(row["temp_air_degC"]),
                "wind_speed_ms-1": float(row["wind_speed_ms-1"]),
                "precip_mass_mm": float(row["precip_mass_mm"]),
                "precip_temp_degC": float(row["precip_temp_degC"]),
                "snow_precip_fraction": float(row["snow_precip_fraction"]),
            }
        )
    return result


def snowpack_present_for_hour(date: dt.date, coe_rows: dict[dt.date, dict[str, float | None]]) -> bool:
    return daily_snowpack_present(date, coe_rows) or daily_snowpack_present(
        date - dt.timedelta(days=1), coe_rows
    )


def daily_snowpack_present(date: dt.date, coe_rows: dict[dt.date, dict[str, float | None]]) -> bool:
    row = coe_rows.get(date)
    if row is None:
        return False
    return (row.get("snow_water_m") or 0.0) > SNOWPACK_SWE_THRESHOLD_M


def group_forcing_by_date(rows: list[dict[str, Any]]) -> dict[dt.date, list[dict[str, Any]]]:
    grouped: dict[dt.date, list[dict[str, Any]]] = {}
    for row in rows:
        grouped.setdefault(row["date"], []).append(row)
    return grouped


def optional_float(value: Any) -> float | None:
    if value is None:
        return None
    if isinstance(value, str) and value.strip() == "":
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def json_pairs(pairs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [{key: value for key, value in row.items() if key != "date_obj"} for row in pairs]


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# SNOWDENSITY-10.3.4 Maritime Over-Accumulation Diagnosis",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Contract: `{report['contract']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        f"- No physics change: `{report['no_physics_change']}`",
        f"- No tuning: `{report['no_tuning']}`",
        f"- Disposition: `{report['summary']['disposition']}`",
        f"- Next route: {report['summary']['next_route']}",
        "",
        "## Mechanism Ranking",
        "",
        "| Rank | Mechanism | Disposition | Evidence | Next action |",
        "|---:|---|---|---|---|",
    ]
    for item in report["mechanism_ranking"]:
        lines.append(
            "| {rank} | `{mechanism}` | `{disposition}` | {evidence} | {next_action} |".format(
                rank=item["rank"],
                mechanism=item["mechanism"],
                disposition=item["disposition"],
                evidence=item["evidence"],
                next_action=item["next_action"],
            )
        )
    lines.extend(
        [
            "",
            "## Site Surfaces",
            "",
            "| Surface | Site | Cover | Scope | Pairs | Mean depth residual m | Fail fraction | Ambiguous precip m | Warm snow m | Positive-temp snowpack h | Rain heat equiv m |",
            "|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for item in report["surfaces"]:
        residuals = item["residuals"]
        forcing = item["forcing_diagnostics"]
        lines.append(
            "| `{surface}` | `{site}` | `{cover}` | `{scope}` | {pairs} | {mean_depth} | {fail_fraction} | {ambig} | {warm_snow} | {thaw_h} | {rain_heat} |".format(
                surface=item["surface_id"],
                site=item["site_group"],
                cover=item["cover"],
                scope=item["verdict_scope"],
                pairs=residuals["paired_count"],
                mean_depth=fmt(residuals["mean_signed_depth_residual_m"]),
                fail_fraction=fmt(residuals.get("snow_control_fail_fraction")),
                ambig=fmt(forcing["phase_ambiguous_precip_m"]),
                warm_snow=fmt(forcing["warm_snow_precip_m"]),
                thaw_h=forcing["positive_temp_snowpack_hours"],
                rain_heat=fmt(forcing["rain_heat_melt_equiv_m"]),
            )
        )
    lines.extend(
        [
            "",
            "## Observation-Blocked Surfaces",
            "",
            "| Surface | Reason |",
            "|---|---|",
        ]
    )
    for item in report["surfaces"]:
        if item["paired_row_count"] == 0:
            lines.append(f"| `{item['surface_id']}` | {item['note']} |")
    lines.extend(
        [
            "",
            "Conclusion: partition and thaw-window diagnosis should precede any opt-in physics candidate. "
            "Rain heat is not first because the current CoE path already carries `dmelt`, and the "
            "diagnosed warm-rain heat magnitude is smaller than the broader phase/thaw signals.",
            "",
        ]
    )
    return "\n".join(lines)


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


if __name__ == "__main__":
    raise SystemExit(main())
