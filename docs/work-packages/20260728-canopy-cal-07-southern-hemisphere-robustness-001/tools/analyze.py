#!/usr/bin/env python3
"""Analyze frozen CAL-07 kernel output without fitting or member selection."""

from __future__ import annotations

import csv
import hashlib
import math
import statistics
from collections import defaultdict
from datetime import date
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
INPUT = PKG / "inputs"


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def write(name: str, fields: tuple[str, ...], data: list[dict[str, object]]) -> None:
    with (ART / name).open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows(data)


def quantile(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    position = (len(ordered) - 1) * fraction
    lower = math.floor(position)
    upper = math.ceil(position)
    return ordered[lower] + (position - lower) * (ordered[upper] - ordered[lower])


def correlation(left: list[float], right: list[float]) -> float:
    left_mean = statistics.fmean(left)
    right_mean = statistics.fmean(right)
    numerator = sum((a - left_mean) * (b - right_mean) for a, b in zip(left, right))
    denominator = math.sqrt(
        sum((a - left_mean) ** 2 for a in left)
        * sum((b - right_mean) ** 2 for b in right)
    )
    return numerator / denominator if denominator else math.nan


def normalize_by_year(data: list[tuple[str, float]]) -> dict[str, float]:
    grouped: dict[str, list[tuple[str, float]]] = defaultdict(list)
    for day, value in data:
        grouped[day[:4]].append((day, value))
    result: dict[str, float] = {}
    for annual in grouped.values():
        low = min(value for _, value in annual)
        high = max(value for _, value in annual)
        if high > low:
            result.update((day, (value - low) / (high - low)) for day, value in annual)
    return result


def ordinal(day: str) -> int:
    return date.fromisoformat(day).toordinal()


def crossing_dates(model: list[dict[str, str]]) -> list[tuple[str, float]]:
    found: list[tuple[str, float]] = []
    previous = None
    for row in model:
        value = float(row["gsi"])
        if previous is not None:
            old = float(previous["gsi"])
            direction = ""
            if old < 0.5 <= value:
                direction = "rising"
            elif old >= 0.5 > value:
                direction = "falling"
            if direction:
                fraction = (0.5 - old) / (value - old)
                found.append((direction, ordinal(previous["date"]) + fraction))
        previous = row
    return found


def main() -> None:
    daily = rows(ART / "daily-kernel-output.csv")
    gates = {row["gate"]: row["status"] for row in rows(ART / "gate-results.csv")}
    observations = rows(INPUT / "observations.csv")
    transitions = rows(INPUT / "transitions.csv")
    members = sorted({row["candidate_id"] for row in daily})
    sites = sorted({row["site_id"] for row in daily})
    if len(members) != 37 or sites != ["SH-DB-BEZA", "SH-EN-ALERCE"]:
        raise ValueError("incomplete CAL-07 inventory")
    inventory: dict[tuple[str, str], int] = defaultdict(int)
    for row in daily:
        inventory[(row["site_id"], row["candidate_id"])] += 1
    if len(inventory) != 74 or set(inventory.values()) != {1666}:
        raise ValueError(f"incomplete site/member daily inventory: {inventory}")

    obs_map = {(row["site_id"], row["date"]): float(row["gcc_90"]) for row in observations}
    ensemble_daily: list[dict[str, object]] = []
    grouped_daily: dict[tuple[str, str], list[float]] = defaultdict(list)
    for row in daily:
        grouped_daily[(row["site_id"], row["date"])].append(float(row["gsi"]))
    for (site, day), values in sorted(grouped_daily.items()):
        obs = obs_map.get((site, day))
        ensemble_daily.append(
            {
                "site_id": site,
                "date": day,
                "year": day[:4],
                "doy": date.fromisoformat(day).timetuple().tm_yday,
                "gsi_p05": f"{quantile(values, 0.05):.9f}",
                "gsi_median": f"{statistics.median(values):.9f}",
                "gsi_p95": f"{quantile(values, 0.95):.9f}",
                "observed_gcc90": "" if obs is None else f"{obs:.9f}",
            }
        )
    write(
        "ensemble-daily.csv",
        ("site_id", "date", "year", "doy", "gsi_p05", "gsi_median", "gsi_p95", "observed_gcc90"),
        ensemble_daily,
    )

    daily_by_member: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for row in daily:
        daily_by_member[(row["site_id"], row["candidate_id"])].append(row)
    timing: list[dict[str, object]] = []
    db_transitions = sorted(
        (row for row in transitions if row["site_id"] == "SH-DB-BEZA"),
        key=lambda row: row["date_50"],
    )
    for member in members:
        crossings = crossing_dates(daily_by_member[("SH-DB-BEZA", member)])
        for index, event in enumerate(db_transitions):
            if index == 0 or index == len(db_transitions) - 1:
                continue
            observed_ordinal = ordinal(event["date_50"])
            lower = 0.5 * (
                ordinal(db_transitions[index - 1]["date_50"]) + observed_ordinal
            )
            upper = 0.5 * (
                observed_ordinal + ordinal(db_transitions[index + 1]["date_50"])
            )
            candidates = [
                value
                for direction, value in crossings
                if direction == event["direction"] and lower < value <= upper
            ]
            modeled = candidates[0] if candidates else None
            timing.append(
                {
                    "candidate_id": member,
                    "year": event["year"],
                    "direction": event["direction"],
                    "observed_date_50": event["date_50"],
                    "observed_doy_50": event["doy_50"],
                    "modeled_crossing_ordinal": "" if modeled is None else f"{modeled:.6f}",
                    "residual_days": (
                        "" if modeled is None else f"{modeled - observed_ordinal:.6f}"
                    ),
                    "same_direction_crossing_count": len(candidates),
                }
            )
    write(
        "transition-residuals.csv",
        (
            "candidate_id",
            "year",
            "direction",
            "observed_date_50",
            "observed_doy_50",
            "modeled_crossing_ordinal",
            "residual_days",
            "same_direction_crossing_count",
        ),
        timing,
    )

    shape_scores: list[dict[str, object]] = []
    for site in sites:
        for year in ("2024", "2025"):
            observed_raw = {
                row["date"]: float(row["gcc_90"])
                for row in observations
                if row["site_id"] == site and row["year"] == year
            }
            for member in members:
                model_raw = {
                    row["date"]: float(row["gsi"])
                    for row in daily_by_member[(site, member)]
                    if row["year"] == year
                }
                common = sorted(set(observed_raw) & set(model_raw))
                if len(common) < 180:
                    raise ValueError(f"insufficient shape support: {site} {year} {member}")
                left_raw = [observed_raw[day] for day in common]
                right_raw = [model_raw[day] for day in common]
                left_range = max(left_raw) - min(left_raw)
                right_range = max(right_raw) - min(right_raw)
                if left_range == 0.0 or right_range == 0.0:
                    pearson = ""
                    rmse_value = ""
                else:
                    left = [(value - min(left_raw)) / left_range for value in left_raw]
                    right = [(value - min(right_raw)) / right_range for value in right_raw]
                    pearson = f"{correlation(left, right):.9f}"
                    rmse_value = f"{math.sqrt(statistics.fmean((a - b) ** 2 for a, b in zip(left, right))):.9f}"
                shape_scores.append(
                    {
                        "site_id": site,
                        "year": year,
                        "candidate_id": member,
                        "paired_days": len(common),
                        "pearson_r": pearson,
                        "normalized_rmse": rmse_value,
                    }
                )
    write(
        "shape-scores.csv",
        ("site_id", "year", "candidate_id", "paired_days", "pearson_r", "normalized_rmse"),
        shape_scores,
    )

    summary: list[dict[str, object]] = []
    residuals = [float(row["residual_days"]) for row in timing if row["residual_days"]]
    timing_complete = len(residuals) == len(timing)
    shape_complete = len(shape_scores) == 148 and all(
        row["pearson_r"] and row["normalized_rmse"] for row in shape_scores
    )
    directional_shape_agreement = shape_complete and all(
        statistics.median(
            float(row["pearson_r"])
            for row in shape_scores
            if row["site_id"] == site and row["year"] == year
        )
        > 0.0
        for site in sites
        for year in ("2024", "2025")
    )
    max_closure = max(abs(float(row["mass_closure_residual_kg_m2"])) for row in daily)
    if max_closure > 1.0e-12:
        raise ValueError(f"mass closure failure: {max_closure}")
    phase_status = gates.get("producer_phase_transform")
    consumer_status = gates.get("real_consumer_ordering")
    if phase_status != "PASS" or consumer_status != "PASS":
        raise ValueError(f"required focused gates did not pass: {gates}")
    for site in sites:
        site_scores = [
            row for row in shape_scores if row["site_id"] == site and row["pearson_r"]
        ]
        site_obs = [float(row["gcc_90"]) for row in observations if row["site_id"] == site]
        site_daily = [row for row in daily if row["site_id"] == site]
        summary.append(
            {
                "site_id": site,
                "members": len(members),
                "kernel_days_per_member": len(site_daily) // len(members),
                "admitted_camera_days": len(site_obs),
                "observed_gcc90_min": f"{min(site_obs):.6f}",
                "observed_gcc90_max": f"{max(site_obs):.6f}",
                "shape_r_median": f"{statistics.median(float(row['pearson_r']) for row in site_scores):.6f}",
                "shape_rmse_median": f"{statistics.median(float(row['normalized_rmse']) for row in site_scores):.6f}",
                "transition_residual_median_days": (
                    f"{statistics.median(residuals):.3f}" if site == "SH-DB-BEZA" else ""
                ),
                "transition_abs_residual_p95_days": (
                    f"{quantile([abs(value) for value in residuals], 0.95):.3f}"
                    if site == "SH-DB-BEZA"
                    else ""
                ),
                "maximum_mass_closure_residual_kg_m2": f"{max_closure:.16e}",
            }
        )
    write(
        "site-summary.csv",
        (
            "site_id",
            "members",
            "kernel_days_per_member",
            "admitted_camera_days",
            "observed_gcc90_min",
            "observed_gcc90_max",
            "shape_r_median",
            "shape_rmse_median",
            "transition_residual_median_days",
            "transition_abs_residual_p95_days",
            "maximum_mass_closure_residual_kg_m2",
        ),
        summary,
    )

    verdicts = [
        {
            "cell": "signed-latitude calendar and seasonal direction",
            "status": (
                "BOUNDED"
                if timing_complete
                and directional_shape_agreement
                and phase_status == "PASS"
                else "CONTRADICTED"
            ),
            "basis": "two SH sites with provisional GCC proxies and gridded forcing",
        },
        {
            "cell": "producer-state cyclic phase invariance",
            "status": "SUPPORTED" if phase_status == "PASS" else "CONTRADICTED",
            "basis": "focused full wrapped-cycle producer-state test",
        },
        {
            "cell": "deciduous transition chronology",
            "status": "BOUNDED" if timing_complete else "CONTRADICTED",
            "basis": "provisional GCC transition proxy; residuals reported without invented threshold",
        },
        {
            "cell": "relative seasonal shape",
            "status": "BOUNDED" if directional_shape_agreement else "CONTRADICTED",
            "basis": "normalized GCC proxy comparison is descriptive and not fitted",
        },
        {
            "cell": "persistent evergreen realization",
            "status": "BOUNDED",
            "basis": "independent evergreen-class lane; GCC color is not foliage mass",
        },
        {
            "cell": "quantitative evergreen-floor agreement",
            "status": "NOT_EVALUATED",
            "basis": "no site-authoritative foliage-mass, LAI, or canopy-cover floor",
        },
        {
            "cell": "daily foliar mass closure",
            "status": "SUPPORTED" if max_closure <= 1.0e-12 else "CONTRADICTED",
            "basis": f"maximum independently reconstructed residual {max_closure:.3e} kg m-2",
        },
        {
            "cell": "real downstream consumer ordering and common-state lineage",
            "status": "SUPPORTED" if consumer_status == "PASS" else "CONTRADICTED",
            "basis": "focused direct-production consumer test passed",
        },
        {
            "cell": "phase-transformed real-consumer chronology",
            "status": "NOT_EVALUATED",
            "basis": "producer phase test and consumer ordering test are separate evidence",
        },
        {
            "cell": "absolute canopy amplitude",
            "status": "NOT_EVALUATED",
            "basis": "no site-authoritative LAI, biomass, or canopy-cover mapping",
        },
        {
            "cell": "needle/fine-woody/decomposition consequences",
            "status": "NOT_EVALUATED",
            "basis": "CAL-05 source authority remains missing",
        },
    ]
    write("verdict-matrix.csv", ("cell", "status", "basis"), verdicts)

    manifest_rows = []
    for path in sorted(ART.glob("*.csv")):
        if path.name == "result-manifest.csv":
            continue
        manifest_rows.append(
            {
                "path": path.relative_to(PKG),
                "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                "bytes": path.stat().st_size,
            }
        )
    write("result-manifest.csv", ("path", "sha256", "bytes"), manifest_rows)


if __name__ == "__main__":
    main()
