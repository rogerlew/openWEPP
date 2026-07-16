#!/usr/bin/env python3
"""Reconstruct the ASSURE-06 strict result from identified retained evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
from typing import Any


def read_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"expected a JSON object: {path}")
    return value


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def value(identifier: str, number: int | float, unit: str, precision: str) -> dict[str, Any]:
    if isinstance(number, float) and not math.isfinite(number):
        raise ValueError(f"non-finite value for {identifier}")
    return {"id": identifier, "precision": precision, "unit_id": unit, "value": number}


def require_close(observed: float, expected: float, label: str) -> None:
    if not math.isclose(observed, expected, rel_tol=1e-15, abs_tol=1e-15):
        raise ValueError(f"{label}: declared {observed!r} != reconstructed {expected!r}")


def build(args: argparse.Namespace) -> dict[str, Any]:
    jennings = read_json(args.jennings)
    activation = read_json(args.activation)
    residuals = read_json(args.residuals)
    frost = read_json(args.frost)
    conservation = read_json(args.conservation)

    if digest(args.conservation_source) != conservation["source_sha256"]:
        raise ValueError("conservation source digest does not match declared identity")
    if digest(args.conservation_log) != conservation["log_sha256"]:
        raise ValueError("conservation log digest does not match declared identity")
    conservation_text = args.conservation_source.read_text(encoding="utf-8")
    conservation_log = args.conservation_log.read_text(encoding="utf-8")
    for operand in (
        "8.881784197001252e-16 mm",
        "exactly `0 mm`",
        "8.526512829121202e-14 mm",
        "-2.632616347142402e-14 mm",
    ):
        if operand not in conservation_text:
            raise ValueError(f"conservation source is missing retained operand {operand!r}")
    for marker in (
        "row 0 P_mm 4.4000000000000004",
        "row 1 P_mm 0",
        "freeze_growth row 2 year 1 julian 3",
        "material_thaw row 1384 year 4 julian 290",
    ):
        if marker not in conservation_log:
            raise ValueError(f"conservation log is missing selected row {marker!r}")
    verified_rows: dict[str, dict[str, Any]] = {}
    for row in conservation["selected_rows"]:
        if row["row_id"].startswith("frost_"):
            reconstructed = (
                row["prior_liquid_mm"]
                + row["prior_frozen_mm"]
                + row["input_mm"]
                - row["sink_mm"]
                - row["current_liquid_mm"]
                - row["current_frozen_mm"]
            )
        else:
            reconstructed = (
                row["prior_storage_mm"]
                + row["input_mm"]
                - row["sink_mm"]
                - row["current_storage_mm"]
            )
        require_close(row["residual_mm"], reconstructed, row["row_id"])
        verified_rows[row["row_id"]] = {**row, "reconstructed_residual_mm": reconstructed}

    hp = jennings["harder_pomeroy_hourly"]
    legacy = jennings["legacy_rst_0c"]
    hp_total = sum(
        hp[key]
        for key in (
            "observed_rain_predicted_rain",
            "observed_rain_predicted_snow",
            "observed_snow_predicted_rain",
            "observed_snow_predicted_snow",
        )
    )
    legacy_total = sum(
        legacy[key]
        for key in (
            "observed_rain_predicted_rain",
            "observed_rain_predicted_snow",
            "observed_snow_predicted_rain",
            "observed_snow_predicted_snow",
        )
    )
    if hp_total != jennings["rows_scored"] or legacy_total != hp_total:
        raise ValueError("phase confusion matrices do not close to rows_scored")
    hp_accuracy = (
        hp["observed_rain_predicted_rain"] + hp["observed_snow_predicted_snow"]
    ) / hp_total
    legacy_accuracy = (
        legacy["observed_rain_predicted_rain"]
        + legacy["observed_snow_predicted_snow"]
    ) / legacy_total
    require_close(hp["accuracy"], hp_accuracy, "Harder-Pomeroy accuracy")
    require_close(legacy["accuracy"], legacy_accuracy, "legacy accuracy")

    snow = activation["model_summaries"]["harder_pomeroy_default"]
    aggregate = snow["aggregate"]
    sites = snow["site_summaries"]
    robust_counts = aggregate["forcing_robust_counts_by_label"]
    available = sum(robust_counts[label] for label in ("fail", "marginal", "pass", "strong"))
    if available != aggregate["robust_available_cell_count"]:
        raise ValueError("snow robust-label counts do not close")
    paired_snotel = sum(site["paired_count"] for site in sites if site["corpus"] == "snotel_observed")
    paired_canopy = sum(site["paired_count"] for site in sites if site["corpus"] == "cancov_forest")
    site_reports = {site["site_id"]: site for site in activation["site_reports"]}
    snow_site_slugs = {
        "snotel_mica_creek_st_joe_id": "mica",
        "snotel_paradise_wa": "paradise",
        "snotel_css_lab_ca": "css",
        "snotel_snowbird_ut": "snowbird",
        "snotel_niwot_co": "niwot",
        "harvard_open": "harvard_open",
        "harvard_hardwood": "harvard_hardwood",
        "marcell_conifer": "marcell_conifer",
        "marcell_deciduous": "marcell_deciduous",
        "marcell_open": "marcell_open",
    }

    clusters = residuals["residual_clusters"]
    component = clusters["by_component_class"]
    if sum(component.values()) != clusters["fail_count"]:
        raise ValueError("snow residual-family counts do not close")

    frost_sites = frost["sites"]
    frost_tube = [site for site in frost_sites if site["method"] == "frost_tube"]
    isotherm = [
        site for site in frost_sites if site["method"] == "soil_temperature_zero_c_isotherm"
    ]
    frost_matches = sum(site["metrics"]["frost_depth_residual_count"] for site in frost_tube)
    frost_max = max(site["metrics"]["max_abs_residual_m"] for site in frost_tube)
    snow_controls = sum(site["metrics"]["snow_depth_control_count"] for site in frost_tube)
    snow_control_failures = sum(
        site["metrics"]["snow_depth_control_fail_count"] for site in frost_tube
    )
    isotherm_bounds = sum(site["metrics"]["isotherm_upper_bound_count"] for site in isotherm)
    isotherm_exceedances = sum(
        site["metrics"]["isotherm_upper_bound_exceedance_count"] for site in isotherm
    )
    frost_site_slugs = {
        "site1_sleepers_south_field_vt": "sleepers_south",
        "site2_sleepers_w9_hardwood_vt": "sleepers_w9",
        "site4_ggd498_morris_mn": "morris",
        "site3_scan_mandan_nd": "mandan",
        "site5_reynolds_creek_us_rls_id": "reynolds",
    }

    trace = activation["trace_proof"]["harder_pomeroy_default"]
    values = [
        value("phase_rows_read", jennings["rows_read"], "row_count", "exact retained integer"),
        value("phase_rows_scored", hp_total, "row_count", "reconstructed confusion-matrix total"),
        value("phase_rows_skipped", jennings["rows_skipped"], "row_count", "exact retained integer"),
        value("phase_rows_skipped_percent", 100.0 * jennings["rows_skipped"] / jennings["rows_read"], "percent", "reconstructed from read and skipped rows"),
        value("phase_stations", jennings["stations_scored"], "station_count", "exact retained integer"),
        value("hp_accuracy_percent", 100.0 * hp_accuracy, "percent", "scaled reconstructed accuracy"),
        value("legacy_accuracy_percent", 100.0 * legacy_accuracy, "percent", "scaled reconstructed accuracy"),
        value("accuracy_difference_points", 100.0 * (hp_accuracy - legacy_accuracy), "percentage_point", "difference of scaled reconstructed accuracies"),
        value("hp_rain_as_rain", hp["observed_rain_predicted_rain"], "row_count", "exact retained integer"),
        value("hp_rain_as_snow", hp["observed_rain_predicted_snow"], "row_count", "exact retained integer"),
        value("hp_snow_as_rain", hp["observed_snow_predicted_rain"], "row_count", "exact retained integer"),
        value("hp_snow_as_snow", hp["observed_snow_predicted_snow"], "row_count", "exact retained integer"),
        value("legacy_rain_as_rain", legacy["observed_rain_predicted_rain"], "row_count", "exact retained integer"),
        value("legacy_rain_as_snow", legacy["observed_rain_predicted_snow"], "row_count", "exact retained integer"),
        value("legacy_snow_as_rain", legacy["observed_snow_predicted_rain"], "row_count", "exact retained integer"),
        value("legacy_snow_as_snow", legacy["observed_snow_predicted_snow"], "row_count", "exact retained integer"),
        value("phase_threshold_bias", jennings["threshold_summary"]["mean_bias_c"], "degC", "retained station aggregate"),
        value("phase_threshold_mae", jennings["threshold_summary"]["mean_absolute_error_c"], "degC", "retained station aggregate"),
        value("observed_humidity_contrast", jennings["humidity_threshold_contrast"]["observed_high_minus_low_c"], "degC", "retained high-minus-low aggregate"),
        value("predicted_humidity_contrast", jennings["humidity_threshold_contrast"]["predicted_high_minus_low_c"], "degC", "retained high-minus-low aggregate"),
        value("humidity_group_station_count", jennings["humidity_threshold_contrast"]["low_rh_station_count"], "station_count", "exact retained group size"),
        value("snow_surface_count", len(sites), "site_count", "counted retained site summaries"),
        value("snotel_paired_rows", paired_snotel, "row_count", "sum of five retained SNOTEL site counts"),
        value("canopy_paired_rows", paired_canopy, "row_count", "sum of five retained canopy-site counts"),
        value("snow_robust_available", available, "cell_count", "sum of available forcing-robust labels"),
        value("snow_robust_fail", robust_counts["fail"], "cell_count", "exact retained label count"),
        value("snow_robust_marginal", robust_counts["marginal"], "cell_count", "exact retained label count"),
        value("snow_robust_pass", robust_counts["pass"], "cell_count", "exact retained label count"),
        value("snow_robust_strong", robust_counts["strong"], "cell_count", "exact retained label count"),
        value("snow_pass_strong_percent", 100.0 * (robust_counts["pass"] + robust_counts["strong"]) / available, "percent", "reconstructed from label counts"),
        value("snow_density_residual_cells", component["density"], "cell_count", "retained residual classification"),
        value("snow_geometry_residual_cells", component["depth_density_geometry"], "cell_count", "retained residual classification"),
        value("snow_timing_residual_cells", component["timing"], "cell_count", "retained residual classification"),
        value("frost_tube_site_count", len(frost_tube), "site_count", "counted method-specific sites"),
        value("frost_tube_matches", frost_matches, "row_count", "sum of site frost-depth residual counts"),
        value("frost_tube_max_abs_residual", frost_max, "m", "maximum retained site residual"),
        value("frost_snow_control_rows", snow_controls, "row_count", "sum of paired snow-control rows"),
        value("frost_snow_control_failures", snow_control_failures, "row_count", "sum of failed paired snow controls"),
        value("frost_snow_control_failure_percent", 100.0 * snow_control_failures / snow_controls, "percent", "reconstructed from paired snow-control counts"),
        value("isotherm_site_count", len(isotherm), "site_count", "counted method-specific sites"),
        value("isotherm_bound_rows", isotherm_bounds, "row_count", "sum of evaluated upper-bound rows"),
        value("isotherm_exceedances", isotherm_exceedances, "row_count", "sum of site upper-bound exceedances"),
        value("isotherm_exceedance_percent", 100.0 * isotherm_exceedances / isotherm_bounds, "percent", "reconstructed from bound counts"),
        value("partition_trace_rows", trace["row_count"], "row_count", "exact production trace count"),
        value("partition_precip_rows", trace["precip_row_count"], "row_count", "exact active-precipitation trace count"),
        value("partition_max_abs_residual", trace["max_abs_partition_residual_m"], "m", "maximum reconstructed production residual"),
        value("partition_tolerance", trace["partition_conservation_tolerance_m"], "m", "declared production-trace allowance"),
        value("snow_accumulation_residual", abs(verified_rows["snow_accumulation"]["reconstructed_residual_mm"]), "mm", "independent produced-WAT reconstruction"),
        value("snow_release_residual", abs(verified_rows["snow_release"]["reconstructed_residual_mm"]), "mm", "independent produced-WAT reconstruction"),
        value("frost_max_storage_residual", max(abs(verified_rows["frost_freeze_growth"]["reconstructed_residual_mm"]), abs(verified_rows["frost_material_thaw"]["reconstructed_residual_mm"])), "mm", "maximum absolute produced-WAT reconstruction residual"),
    ]
    for site in sites:
        slug = snow_site_slugs[site["site_id"]]
        counts = site["forcing_robust_counts_by_label"]
        cells = {
            cell["cell_id"]: cell
            for cell in site_reports[site["site_id"]]["models"]["harder_pomeroy_default"]["rubric_profile"]["cells"]
        }
        values.extend(
            [
                value(f"snow_{slug}_paired", site["paired_count"], "row_count", "exact retained site count"),
                *[
                    value(f"snow_{slug}_{label}", counts.get(label, 0), "cell_count", "exact retained site label count")
                    for label in ("fail", "marginal", "pass", "strong")
                ],
                value(f"snow_{slug}_density_kge", cells["seasonal_densification_trajectory"]["metrics"]["kge"], "dimensionless", "exact retained site diagnostic"),
                value(f"snow_{slug}_peak_swe_offset_days", cells["seasonal_peak_swe_date"]["metrics"]["median_offset_days"], "day", "exact retained median modeled-minus-observed offset"),
                value(f"snow_{slug}_meltout_offset_days", cells["seasonal_ablation_meltout_date"]["metrics"]["median_offset_days"], "day", "exact retained median modeled-minus-observed offset"),
                value(f"snow_{slug}_geometry_ratio", cells["seasonal_depth_swe_slope"]["metrics"]["slope_ratio"], "dimensionless", "exact retained modeled-to-observed slope ratio"),
            ]
        )
    for site in frost_sites:
        slug = frost_site_slugs[site["site_id"]]
        metrics = site["metrics"]
        if site["method"] == "frost_tube":
            values.extend(
                [
                    value(f"frost_{slug}_matches", metrics["frost_depth_residual_count"], "row_count", "exact retained site count"),
                    value(f"frost_{slug}_max_abs_residual", metrics["max_abs_residual_m"], "m", "exact retained site maximum"),
                    value(f"frost_{slug}_snow_rows", metrics["snow_depth_control_count"], "row_count", "exact retained site count"),
                    value(f"frost_{slug}_snow_failures", metrics["snow_depth_control_fail_count"], "row_count", "exact retained site count"),
                ]
            )
        else:
            bounds = metrics["isotherm_upper_bound_count"]
            exceedances = metrics["isotherm_upper_bound_exceedance_count"]
            values.extend(
                [
                    value(f"frost_{slug}_bounds", bounds, "row_count", "exact retained site count"),
                    value(f"frost_{slug}_exceedances", exceedances, "row_count", "exact retained site count"),
                    value(f"frost_{slug}_exceedance_percent", 100.0 * exceedances / bounds, "percent", "reconstructed site rate"),
                    value(f"frost_{slug}_max_margin", metrics["max_isotherm_upper_bound_margin_m"], "m", "exact retained site maximum"),
                ]
            )
    for row in verified_rows.values():
        slug = row["row_id"]
        if slug.startswith("frost_"):
            prior = row["prior_liquid_mm"] + row["prior_frozen_mm"]
            current = row["current_liquid_mm"] + row["current_frozen_mm"]
        else:
            prior = row["prior_storage_mm"]
            current = row["current_storage_mm"]
        values.extend(
            [
                value(f"conservation_{slug}_prior", prior, "mm", "retained selected-row operand"),
                value(f"conservation_{slug}_input", row["input_mm"], "mm", "retained selected-row operand"),
                value(f"conservation_{slug}_sink", row["sink_mm"], "mm", "retained selected-row operand"),
                value(f"conservation_{slug}_current", current, "mm", "retained selected-row operand"),
                value(f"conservation_{slug}_residual", row["reconstructed_residual_mm"], "mm", "reconstructed selected-row residual"),
            ]
        )
        if slug.startswith("frost_"):
            values.extend(
                [
                    value(f"conservation_{slug}_prior_liquid", row["prior_liquid_mm"], "mm", "retained selected-row operand"),
                    value(f"conservation_{slug}_prior_frozen", row["prior_frozen_mm"], "mm", "retained selected-row operand"),
                    value(f"conservation_{slug}_current_liquid", row["current_liquid_mm"], "mm", "retained selected-row operand"),
                    value(f"conservation_{slug}_current_frozen", row["current_frozen_mm"], "mm", "retained selected-row operand"),
                ]
            )
    return {"result_id": "SF-RESULT-SYNTHESIS", "schema_version": 1, "values": values}


def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser()
    p.add_argument("--jennings", required=True, type=Path)
    p.add_argument("--activation", required=True, type=Path)
    p.add_argument("--residuals", required=True, type=Path)
    p.add_argument("--frost", required=True, type=Path)
    p.add_argument("--conservation", required=True, type=Path)
    p.add_argument("--conservation-source", required=True, type=Path)
    p.add_argument("--conservation-log", required=True, type=Path)
    return p


if __name__ == "__main__":
    print(json.dumps(build(parser().parse_args()), indent=2, sort_keys=True))
