#!/usr/bin/env python3
"""Independently reconstruct 21L table reductions and disposition."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import hashlib
import json
import math
import statistics
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
DEFAULT_ROOT = REPO / "target/snow_warm_mixed_prepeak_loss_energy_attribution_v2"
CANONICAL = (
    "snotel_mica_creek_st_joe_id",
    "snotel_niwot_co",
    "snotel_paradise_wa",
    "snotel_snowbird_ut",
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def floats_equal(left: float, right: float, tolerance: float = 1e-12) -> bool:
    return math.isclose(left, right, rel_tol=0.0, abs_tol=tolerance)


def correlation(left: list[float], right: list[float]) -> float:
    left_mean = statistics.fmean(left)
    right_mean = statistics.fmean(right)
    numerator = sum((x - left_mean) * (y - right_mean) for x, y in zip(left, right))
    denominator = math.sqrt(
        sum((x - left_mean) ** 2 for x in left)
        * sum((y - right_mean) ** 2 for y in right)
    )
    return numerator / denominator


def rank(values: list[float]) -> list[float]:
    output = [0.0] * len(values)
    for index, value in enumerate(values):
        lower = sum(candidate < value for candidate in values)
        equal = sum(candidate == value for candidate in values)
        output[index] = lower + (equal + 1) / 2.0
    return output


def execute(root: Path) -> dict[str, Any]:
    result_path = root / "results.json"
    receipt_path = root / "execution-receipt.json"
    freeze_path = PACKAGE / "artifacts/analysis-freeze.json"
    result = json.loads(result_path.read_text(encoding="utf-8"))
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    freeze = json.loads(freeze_path.read_text(encoding="utf-8"))
    constants = freeze["constants"]
    annual = rows(root / "tables/annual-attribution.csv")
    dry = rows(root / "tables/dry-intervals-corrected.csv")
    comparison = rows(root / "tables/dry-interval-predecessor-comparison.csv")
    pairs = rows(root / "tables/snowbird-pairs.csv")
    daily = rows(root / "tables/daily-attribution.csv")
    monthly = rows(root / "tables/monthly-attribution.csv")
    hourly = rows(root / "tables/hourly-classification.csv")
    empirical = rows(root / "tables/empirical-term-attribution.csv")
    stage3 = rows(root / "tables/stage3-response.csv")
    forcing_state = rows(root / "tables/forcing-state-contrasts.csv")
    if len(annual) != result["counts"]["annual_rows"]:
        raise RuntimeError("annual count mismatch")
    for row in annual:
        for field in (
            "coe_cap_adjustment_m",
            "cold_day_coe_cap_adjustment_m",
            "mixed_day_coe_cap_adjustment_m",
            "warm_day_coe_cap_adjustment_m",
        ):
            value = float(row[field])
            if not math.isfinite(value):
                raise RuntimeError(f"non-finite retained cap field: {field}")
    if len(dry) != result["counts"]["dry_interval_rows"]:
        raise RuntimeError("dry count mismatch")
    if len(pairs) != result["counts"]["snowbird_pair_rows"]:
        raise RuntimeError("pair count mismatch")
    if len(comparison) != result["counts"]["canonical_dry_interval_rows"]:
        raise RuntimeError("comparison count mismatch")
    for key, table in (
        ("daily_rows", daily), ("monthly_rows", monthly),
        ("hourly_class_rows", hourly), ("empirical_term_rows", empirical),
        ("stage3_response_rows", stage3),
        ("forcing_state_contrast_rows", forcing_state),
    ):
        if len(table) != result["counts"][key]:
            raise RuntimeError(f"{key} count mismatch")
    mass_tolerance = float(constants["mass_tolerance_m"])
    energy_tolerance = float(constants["energy_tolerance_j_m2"])
    for row in hourly:
        component_sum = sum(float(row[f"coe_{name}_m"]) for name in ("amelt", "bmelt", "cmelt", "dmelt"))
        if not floats_equal(component_sum, float(row["coe_uncapped_m"]), mass_tolerance):
            raise RuntimeError("independent hourly CoE uncapped reconstruction failed")
        if not floats_equal(component_sum + float(row["coe_cap_adjustment_m"]), float(row["coe_applied_m"]), mass_tolerance):
            raise RuntimeError("independent hourly CoE applied reconstruction failed")
    for row in stage3:
        mass = (
            float(row["stage3_incoming_liquid_m"])
            - float(row["stage3_routed_liquid_m"])
            - float(row["stage3_retained_liquid_delta_m"])
            - float(row["stage3_refrozen_liquid_m"])
        )
        energy = (
            float(row["stage3_surface_energy_j_m2"])
            + float(row["stage3_conduction_energy_j_m2"])
            + float(row["stage3_latent_refreeze_energy_j_m2"])
            + float(row["stage3_cold_content_export_j_m2"])
            - (float(row["stage3_cold_content_before_j_m2"]) - float(row["stage3_cold_content_after_j_m2"]))
        )
        if abs(mass) > mass_tolerance or abs(energy) > energy_tolerance:
            raise RuntimeError("independent daily Stage-3 reconstruction failed")
    contrast_fields = (
        "hourly_tmax_c", "wind_m_s", "dewpoint_c", "canopy_cover_fraction",
        "radiation_mj_m2", "cloud_fraction_mean", "fixture_precip_m", "snowfall_m",
        "runtime_depth_before_m", "runtime_density_before_kg_m3",
        "retained_liquid_before_m",
    )
    contrast_by_lane = {row["lane"]: row for row in forcing_state}
    for lane, reported_contrast in contrast_by_lane.items():
        lane_rows = [row for row in daily if row["lane"] == lane]
        years = sorted({int(row["water_year"]) for row in lane_rows})
        deltas: dict[str, list[float]] = {field: [] for field in contrast_fields}
        valid_years = 0
        for water_year in years:
            eligible = [
                row for row in lane_rows
                if int(row["water_year"]) == water_year
                and row["thermal_class"] != "unclassified"
            ]
            material = [row for row in eligible if row["material_pack_loss"] == "True"]
            nonmaterial = [row for row in eligible if row["material_pack_loss"] == "False"]
            if not material or not nonmaterial:
                continue
            valid_years += 1
            for field in contrast_fields:
                deltas[field].append(
                    statistics.median(float(row[field]) for row in material)
                    - statistics.median(float(row[field]) for row in nonmaterial)
                )
        if valid_years != int(reported_contrast["paired_year_count"]):
            raise RuntimeError(f"forcing/state paired-year count mismatch: {lane}")
        for field, values in deltas.items():
            reported_median = float(reported_contrast[f"median_annual_material_minus_nonmaterial_{field}"])
            reported_fraction = float(reported_contrast[f"fraction_years_positive_{field}"])
            if not floats_equal(statistics.median(values), reported_median):
                raise RuntimeError(f"forcing/state median mismatch: {lane} {field}")
            if not floats_equal(sum(value > 0.0 for value in values) / len(values), reported_fraction):
                raise RuntimeError(f"forcing/state direction mismatch: {lane} {field}")
    monthly_by_key = {
        (row["lane"], int(row["water_year"]), int(row["month"]), row["thermal_class"]): row
        for row in monthly
    }
    daily_by_month: dict[tuple[str, int, int, str], list[dict[str, str]]] = {}
    for row in daily:
        key = (row["lane"], int(row["water_year"]), int(row["month"]), row["thermal_class"])
        daily_by_month.setdefault(key, []).append(row)
    if daily_by_month.keys() != monthly_by_key.keys():
        raise RuntimeError("monthly key inventory mismatch")
    monthly_additive = (
        "fixture_precip_m", "snowfall_m", "rain_retained_m", "rain_released_m",
        "pack_loss_m", "sublimation_m", "raw_melt_m", "routed_melt_m",
        "coe_applied_positive_m", "coe_cap_adjustment_m", "radiation_mj_m2",
        "stage3_surface_energy_j_m2", "stage3_shortwave_energy_j_m2",
        "stage3_longwave_energy_j_m2", "stage3_latent_energy_j_m2",
        "stage3_conduction_energy_j_m2", "stage3_cold_content_change_j_m2",
        "stage3_refrozen_liquid_m", "stage3_retained_liquid_delta_m",
        "stage3_routed_liquid_m", "coe_amelt_positive_m", "coe_bmelt_positive_m",
        "coe_cmelt_positive_m", "coe_dmelt_positive_m", "coe_amelt_m", "coe_bmelt_m",
        "coe_cmelt_m", "coe_dmelt_m",
    )
    for key, source_rows in daily_by_month.items():
        reported = monthly_by_key[key]
        if len(source_rows) != int(reported["day_count"]):
            raise RuntimeError(f"monthly day count mismatch: {key}")
        if sum(row["material_pack_loss"] == "True" for row in source_rows) != int(reported["material_pack_loss_day_count"]):
            raise RuntimeError(f"monthly material-loss day count mismatch: {key}")
        if not floats_equal(
            statistics.fmean(float(row["cloud_fraction_mean"]) for row in source_rows),
            float(reported["mean_cloud_fraction"]),
        ):
            raise RuntimeError(f"monthly cloud mean mismatch: {key}")
        for field in monthly_additive:
            if not floats_equal(sum(float(row[field]) for row in source_rows), float(reported[field])):
                raise RuntimeError(f"monthly reduction mismatch: {key} {field}")
    empirical_by_key = {
        (row["lane"], int(row["water_year"]), row["thermal_class"], row["term"]): row
        for row in empirical
    }
    for row in annual:
        for thermal in ("cold_day", "mixed_day", "warm_day"):
            for term in ("amelt", "bmelt", "cmelt", "dmelt"):
                key = (row["lane"], int(row["water_year"]), thermal, term)
                reported = empirical_by_key.get(key)
                if reported is None:
                    raise RuntimeError(f"missing empirical row: {key}")
                for source_field, target_field in (
                    (f"{thermal}_{term}_m", "signed_m"),
                    (f"{thermal}_{term}_positive_m", "positive_m"),
                    (f"{thermal}_coe_cap_adjustment_m", "cap_adjustment_m"),
                    (f"{thermal}_coe_applied_positive_m", "applied_positive_m"),
                ):
                    if not floats_equal(float(row[source_field]), float(reported[target_field])):
                        raise RuntimeError(f"empirical reduction mismatch: {key} {target_field}")
    dry_by_key = {
        (row["lane"], int(row["water_year"]), row["interval_start"], row["interval_end"]): row
        for row in dry if row["role"] == "CANONICAL"
    }
    comparison_by_key = {
        (row["site"], int(row["water_year"]), row["interval_start"], row["interval_end"]): row
        for row in comparison
    }
    if dry_by_key.keys() != comparison_by_key.keys():
        raise RuntimeError("dry comparison key inventory mismatch")
    predecessor_path = REPO / "target/snow_accumulation_target_feasibility_input_loss_discrimination_v2/tables/dry-intervals.csv"
    if sha256(predecessor_path) != freeze["source_identities"]["predecessor_21j_dry_intervals_sha256"]:
        raise RuntimeError("predecessor dry table identity mismatch")
    predecessor = {
        (row["site"], int(row["water_year"]), row["interval_start"], row["interval_end"]): row
        for row in rows(predecessor_path)
    }
    if predecessor.keys() != comparison_by_key.keys():
        raise RuntimeError("predecessor dry key inventory mismatch")
    daily_by_lane_date = {(row["lane"], row["date"]): row for row in daily}
    for key, dry_row in dry_by_key.items():
        compared = comparison_by_key[key]
        if not floats_equal(float(dry_row["modeled_pack_loss_m"]), float(compared["corrected_modeled_pack_loss_m"])):
            raise RuntimeError(f"dry comparison current-loss mismatch: {key}")
        if not floats_equal(float(predecessor[key]["modeled_pack_loss_m"]), float(compared["pre_21k_modeled_pack_loss_m"])):
            raise RuntimeError(f"dry comparison predecessor-loss mismatch: {key}")
        start = dt.date.fromisoformat(key[2])
        end = dt.date.fromisoformat(key[3])
        reconstructed = 0.0
        stamp = start
        while stamp <= end:
            reconstructed += float(daily_by_lane_date[(key[0], stamp.isoformat())]["pack_loss_m"])
            stamp += dt.timedelta(days=1)
        if not floats_equal(reconstructed, float(dry_row["modeled_pack_loss_m"])):
            raise RuntimeError(f"dry interval daily-loss reconstruction mismatch: {key}")

    independent_sites = []
    for site in CANONICAL:
        site_rows = [row for row in annual if row["lane"] == site and row["warm_mixed_pack_loss_fraction"]]
        shares = [float(row["warm_mixed_pack_loss_fraction"]) for row in site_rows]
        signal = (
            len(site_rows) >= constants["minimum_eligible_years_per_site"]
            and statistics.median(shares) >= constants["warm_mixed_loss_fraction_threshold"]
            and sum(value >= 0.5 for value in shares) / len(shares) >= constants["site_direction_fraction"]
        )
        component_medians = {
            name: statistics.median(
                float(row[f"mixed_day_{name}_positive_m"])
                + float(row[f"warm_day_{name}_positive_m"])
                for row in site_rows
            )
            for name in ("amelt", "bmelt", "cmelt", "dmelt")
        }
        independent_sites.append({
            "site": site,
            "eligible_year_count": len(site_rows),
            "median_warm_mixed_pack_loss_fraction": statistics.median(shares),
            "fraction_years_warm_mixed_share_at_least_half": sum(value >= 0.5 for value in shares) / len(shares),
            "warm_mixed_site_signal": signal,
            "dominant_positive_empirical_term_warm_mixed": max(
                component_medians, key=component_medians.get
            ),
            "component_annual_medians": component_medians,
            "pearson": correlation(
                [float(row["coe_applied_positive_m"]) for row in site_rows],
                [float(row["modeled_pack_loss_m"]) for row in site_rows],
            ),
            "spearman": correlation(
                rank([float(row["coe_applied_positive_m"]) for row in site_rows]),
                rank([float(row["modeled_pack_loss_m"]) for row in site_rows]),
            ),
        })
    reported = {row["site"]: row for row in result["site_summary"]}
    for independent in independent_sites:
        current = reported[independent["site"]]
        if independent["eligible_year_count"] != current["eligible_year_count"]:
            raise RuntimeError("site year count mismatch")
        for field in ("median_warm_mixed_pack_loss_fraction", "fraction_years_warm_mixed_share_at_least_half"):
            if not floats_equal(independent[field], current[field]):
                raise RuntimeError(f"site summary mismatch: {independent['site']} {field}")
        if independent["warm_mixed_site_signal"] != current["warm_mixed_site_signal"]:
            raise RuntimeError("site signal mismatch")
        if independent["dominant_positive_empirical_term_warm_mixed"] != current["dominant_positive_empirical_term_warm_mixed"]:
            raise RuntimeError("site dominant empirical term mismatch")
        for name, value in independent["component_annual_medians"].items():
            if not floats_equal(value, current[f"warm_mixed_positive_{name}_annual_median_m"]):
                raise RuntimeError(f"site component annual median mismatch: {independent['site']} {name}")
        if not floats_equal(independent["pearson"], current["annual_coe_applied_positive_vs_pack_loss_pearson"]):
            raise RuntimeError("site Pearson association mismatch")
        if not floats_equal(independent["spearman"], current["annual_coe_applied_positive_vs_pack_loss_spearman"]):
            raise RuntimeError("site Spearman association mismatch")

    deltas = [float(row["scaled_minus_canonical_pack_loss_m"]) for row in pairs]
    nonzero = [value for value in deltas if abs(value) > constants["zero_tolerance_m"]]
    direction = max(
        sum(value > 0.0 for value in nonzero) / len(nonzero),
        sum(value < 0.0 for value in nonzero) / len(nonzero),
    )
    independent_state = (
        len(pairs) >= constants["minimum_eligible_years_per_site"]
        and statistics.median(abs(value) for value in deltas) >= constants["scaled_state_response_materiality_m"]
        and direction >= constants["scaled_direction_fraction"]
    )
    if independent_state != result["snowbird_pair_summary"]["state_signal"]:
        raise RuntimeError("Snowbird state signal mismatch")

    site_count = sum(row["warm_mixed_site_signal"] for row in independent_sites)
    coverage = all(row["eligible_year_count"] >= constants["minimum_eligible_years_per_site"] for row in independent_sites)
    systemic = site_count >= constants["systemic_site_count"]
    if not coverage:
        verdict = "UNRESOLVED_OR_COVERAGE_LIMITED"
    elif systemic and independent_state:
        verdict = "MULTIFACTOR_WARM_MIXED_AND_STATE_SIGNAL"
    elif systemic:
        verdict = "WARM_MIXED_COE_LOSS_CONCENTRATION_SIGNAL"
    elif independent_state:
        verdict = "STATE_MEDIATED_INPUT_SENSITIVITY_SIGNAL"
    else:
        verdict = "NO_SYSTEMIC_WARM_MIXED_SIGNAL"
    if verdict != result["verdict"] or verdict != receipt["verdict"]:
        raise RuntimeError("verdict mismatch")

    max_dry_delta = max(abs(float(row["corrected_minus_pre_21k_m"])) for row in comparison)
    if not floats_equal(max_dry_delta, result["maximum_abs_corrected_minus_pre_21k_dry_loss_m"]):
        raise RuntimeError("dry rebaseline delta mismatch")
    for path_text, record in receipt["outputs_before_receipt"].items():
        path = REPO / path_text
        if sha256(path) != record["sha256"] or path.stat().st_size != record["size_bytes"]:
            raise RuntimeError(f"receipt output identity mismatch: {path_text}")
    verification = {
        "schema_version": 1,
        "status": "PASS",
        "evidence_mode": "Ran: independent reduction of accepted source tables",
        "result_sha256": sha256(result_path),
        "execution_receipt_sha256": sha256(receipt_path),
        "freeze_sha256": sha256(freeze_path),
        "tool_sha256": sha256(Path(__file__)),
        "counts": {
            "annual_rows": len(annual),
            "dry_rows": len(dry),
            "comparison_rows": len(comparison),
            "pair_rows": len(pairs),
            "daily_rows": len(daily),
            "monthly_rows": len(monthly),
            "hourly_rows": len(hourly),
            "empirical_rows": len(empirical),
            "stage3_rows": len(stage3),
            "forcing_state_contrast_rows": len(forcing_state),
        },
        "site_reconstruction": independent_sites,
        "snowbird_state_signal": independent_state,
        "maximum_abs_corrected_minus_pre_21k_dry_loss_m": max_dry_delta,
        "verdict": verdict,
    }
    output = root / "independent-verification.json"
    output.write_text(json.dumps(verification, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return verification


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=DEFAULT_ROOT)
    args = parser.parse_args()
    print(json.dumps(execute(args.root.resolve()), indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
