#!/usr/bin/env python3
"""Independently verify the retained snow adjudication traces and summaries."""

from __future__ import annotations

import csv
import datetime as dt
import hashlib
import json
import statistics
import sys
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
OUTPUT = REPO / "target/snow_prepeak_mass_transition_physics_adjudication_v2"
RESULTS = OUTPUT / "results/cross-fixture-results.json"
RECEIPT = OUTPUT / "execution-receipt.json"
VERIFY_OUTPUT = OUTPUT / "independent-verification.json"
ZERO = 1.0e-12
RHO_WATER_KG_M3 = 1000.0
LATENT_HEAT_FUSION_J_KG = 333_550.0
ABS_COMPARE_TOLERANCE = 1.0e-10


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.write_text(
        json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def climate_dates(path: Path) -> list[dt.date]:
    dates = []
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            fields = line.split()
            if len(fields) != 13:
                continue
            try:
                stamp = dt.date(int(fields[2]), int(fields[1]), int(fields[0]))
                list(map(float, fields[3:]))
            except ValueError:
                continue
            dates.append(stamp)
    if not dates or dates != sorted(dates):
        raise RuntimeError(f"invalid climate chronology: {path}")
    return dates


def observed_peaks(path: Path) -> dict[int, tuple[dt.date, float]]:
    peaks: dict[int, tuple[dt.date, float]] = {}
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            raw = row.get("observed_swe_mm")
            if raw in (None, ""):
                continue
            candidate = (dt.date.fromisoformat(row["date"]), float(raw) / 1000.0)
            water_year = int(row["water_year"])
            if candidate[1] <= 0.0:
                continue
            current = peaks.get(water_year)
            if current is None or candidate[1] > current[1]:
                peaks[water_year] = candidate
    return peaks


def new_accumulator(start: dt.date, end: dt.date, observed: float) -> dict[str, Any]:
    return {
        "start": start,
        "end": end,
        "observed": observed,
        "row_count": 0,
        "modeled_peak_m": -1.0,
        "modeled_peak_date": None,
        "snowfall_m": 0.0,
        "solid_pack_loss_m": 0.0,
        "gross_positive_applied_coe_m": 0.0,
        "negative_applied_coe_m": 0.0,
        "daily_local_signed_opportunity_m": 0.0,
        "post_coe_stage3_cold_opportunity_m": 0.0,
        "stage3_incoming_m": 0.0,
        "stage3_retained_positive_m": 0.0,
        "stage3_refrozen_m": 0.0,
    }


def close_enough(actual: float, expected: float) -> bool:
    return abs(actual - expected) <= ABS_COMPARE_TOLERANCE


def verify_site(
    site: str,
    site_receipt: dict[str, Any],
    expected_annual: dict[int, dict[str, Any]],
) -> dict[str, Any]:
    fixture = OUTPUT / "fixtures" / site
    climate = next(iter(sorted(fixture.glob("*.cli"))), None)
    if climate is None:
        raise RuntimeError(f"climate file missing for {site}")
    dates = climate_dates(climate)
    observation = REPO / site_receipt["observation"]["path"]
    peaks = observed_peaks(observation)
    accumulators = {
        year: new_accumulator(dt.date(year - 1, 10, 1), peak[0], peak[1])
        for year, peak in peaks.items()
        if year != 2025
        and dt.date(year - 1, 10, 1) in dates
        and peak[0] in dates
    }
    by_date = {
        stamp: year
        for year, accumulator in accumulators.items()
        for stamp in dates
        if accumulator["start"] <= stamp <= accumulator["end"]
    }
    trace_name = f"{site}-adjudication.snow.jsonl"
    trace_record = site_receipt["outputs"][trace_name]
    trace = REPO / trace_record["path"]
    if sha256(trace) != trace_record["sha256"]:
        raise RuntimeError(f"trace hash differs for {site}")
    max_storage = 0.0
    max_accumulation_hourly_residual = 0.0
    max_handoff = 0.0
    max_stage3 = 0.0
    max_residual_difference = 0.0
    row_count = 0
    with trace.open(encoding="utf-8") as handle:
        for stamp, line in zip(dates, handle, strict=True):
            row = json.loads(line)
            row_count += 1
            if row["schema"] != "openwepp-r7h-direct-production-snow-trace-v4":
                raise RuntimeError(f"unexpected trace schema for {site}")
            gross_positive = 0.0
            negative = 0.0
            hourly_snowfall = 0.0
            for hour in row["accumulation_melt_hourly"]:
                applied = float(hour["coe_melt_applied_m"])
                gross_positive += max(applied, 0.0)
                negative += min(applied, 0.0)
                hourly_snowfall += float(hour["snowfall_swe_m"])
            storage = (
                float(row["runtime_swe_after_m"])
                - float(row["runtime_swe_before_m"])
                - hourly_snowfall
                - float(row["rain_retained_m"])
                + float(row["snowpack_swe_loss_m"])
                + float(row["sublimation_m"])
            )
            handoff = float(row["stage3_incoming_liquid_m"]) - (
                float(row["snowpack_swe_loss_m"])
                + float(row["rain_released_m"])
            )
            stage3 = (
                float(row["stage3_incoming_liquid_m"])
                - float(row["stage3_routed_liquid_m"])
                - float(row["stage3_retained_liquid_delta_m"])
                - float(row["stage3_refrozen_liquid_m"])
            )
            residual_difference = stage3 - float(
                row["stage3_liquid_closure_residual_m"]
            )
            max_storage = max(max_storage, abs(storage))
            max_accumulation_hourly_residual = max(
                max_accumulation_hourly_residual,
                abs(float(row["accumulation_m"]) - hourly_snowfall),
            )
            max_handoff = max(max_handoff, abs(handoff))
            max_stage3 = max(max_stage3, abs(stage3))
            max_residual_difference = max(
                max_residual_difference, abs(residual_difference)
            )
            year = by_date.get(stamp)
            if year is None:
                continue
            accumulator = accumulators[year]
            accumulator["row_count"] += 1
            modeled_swe = float(row["runtime_swe_after_m"])
            if modeled_swe > accumulator["modeled_peak_m"]:
                accumulator["modeled_peak_m"] = modeled_swe
                accumulator["modeled_peak_date"] = stamp
            accumulator["snowfall_m"] += hourly_snowfall
            accumulator["solid_pack_loss_m"] += float(row["snowpack_swe_loss_m"])
            accumulator["gross_positive_applied_coe_m"] += gross_positive
            accumulator["negative_applied_coe_m"] += negative
            accumulator["daily_local_signed_opportunity_m"] += gross_positive - max(
                gross_positive + negative, 0.0
            )
            accumulator["post_coe_stage3_cold_opportunity_m"] += min(
                float(row["stage3_incoming_liquid_m"]),
                max(float(row["stage3_cold_content_before_j_m2"]), 0.0)
                / (RHO_WATER_KG_M3 * LATENT_HEAT_FUSION_J_KG),
            )
            accumulator["stage3_incoming_m"] += float(
                row["stage3_incoming_liquid_m"]
            )
            accumulator["stage3_retained_positive_m"] += max(
                float(row["stage3_retained_liquid_delta_m"]), 0.0
            )
            accumulator["stage3_refrozen_m"] += float(
                row["stage3_refrozen_liquid_m"]
            )
        if handle.readline():
            raise RuntimeError(f"trace has more rows than climate for {site}")
    if row_count != len(dates):
        raise RuntimeError(f"trace has fewer rows than climate for {site}")

    comparisons = []
    for year, accumulator in sorted(accumulators.items()):
        expected = expected_annual[year]
        derived = {
            "modeled_peak_swe_m": accumulator["modeled_peak_m"],
            "snowfall_m": accumulator["snowfall_m"],
            "solid_pack_loss_m": accumulator["solid_pack_loss_m"],
            "gross_positive_applied_coe_m": accumulator[
                "gross_positive_applied_coe_m"
            ],
            "negative_applied_coe_m": accumulator["negative_applied_coe_m"],
            "daily_local_signed_opportunity_m": accumulator[
                "daily_local_signed_opportunity_m"
            ],
            "post_coe_stage3_cold_opportunity_m": accumulator[
                "post_coe_stage3_cold_opportunity_m"
            ],
            "stage3_incoming_m": accumulator["stage3_incoming_m"],
            "stage3_retained_positive_m": accumulator[
                "stage3_retained_positive_m"
            ],
            "stage3_refrozen_m": accumulator["stage3_refrozen_m"],
        }
        field_differences = {
            field: value - float(expected[field])
            for field, value in derived.items()
            if not close_enough(value, float(expected[field]))
        }
        date_matches = accumulator["modeled_peak_date"].isoformat() == expected[
            "modeled_peak_date"
        ]
        if field_differences or not date_matches:
            raise RuntimeError(
                f"independent annual reconstruction differs for {site}/{year}: "
                f"{field_differences}, date_match={date_matches}"
            )
        comparisons.append(
            {
                "water_year": year,
                "row_count": accumulator["row_count"],
                "field_count": len(derived),
                "exact_peak_date": date_matches,
                "maximum_abs_field_difference_m": max(
                    abs(derived[field] - float(expected[field])) for field in derived
                ),
            }
        )
    return {
        "trace_row_count": row_count,
        "primary_window_count": len(comparisons),
        "annual_comparisons": comparisons,
        "maximum_abs_storage_closure_m": max_storage,
        "maximum_abs_accumulation_hourly_residual_m": (
            max_accumulation_hourly_residual
        ),
        "maximum_abs_handoff_closure_m": max_handoff,
        "maximum_abs_stage3_reconstructed_residual_m": max_stage3,
        "maximum_abs_stage3_producer_residual_difference_m": max_residual_difference,
    }


def main() -> int:
    results = json.loads(RESULTS.read_text(encoding="utf-8"))
    receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
    annual_by_site: dict[str, dict[int, dict[str, Any]]] = {}
    for row in results["annual"]:
        if row["right_censored"]:
            continue
        annual_by_site.setdefault(row["site"], {})[int(row["water_year"])] = row
    sites = {
        site: verify_site(site, site_receipt, annual_by_site[site])
        for site, site_receipt in sorted(receipt["sites"].items())
    }
    all_annual = [
        comparison
        for site in sites.values()
        for comparison in site["annual_comparisons"]
    ]
    verification = {
        "schema_version": 1,
        "status": "PASS",
        "evidence_mode": "Ran: independent direct trace and annual reconstruction",
        "result_sha256": sha256(RESULTS),
        "receipt_sha256": sha256(RECEIPT),
        "site_count": len(sites),
        "trace_row_count": sum(site["trace_row_count"] for site in sites.values()),
        "primary_window_count": len(all_annual),
        "maximum_abs_annual_field_difference_m": max(
            row["maximum_abs_field_difference_m"] for row in all_annual
        ),
        "maximum_abs_storage_closure_m": max(
            site["maximum_abs_storage_closure_m"] for site in sites.values()
        ),
        "maximum_abs_accumulation_hourly_residual_m": max(
            site["maximum_abs_accumulation_hourly_residual_m"]
            for site in sites.values()
        ),
        "maximum_abs_handoff_closure_m": max(
            site["maximum_abs_handoff_closure_m"] for site in sites.values()
        ),
        "maximum_abs_stage3_reconstructed_residual_m": max(
            site["maximum_abs_stage3_reconstructed_residual_m"]
            for site in sites.values()
        ),
        "maximum_abs_stage3_producer_residual_difference_m": max(
            site["maximum_abs_stage3_producer_residual_difference_m"]
            for site in sites.values()
        ),
        "sites": sites,
    }
    write_json(VERIFY_OUTPUT, verification)
    print(json.dumps({key: value for key, value in verification.items() if key != "sites"}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
