#!/usr/bin/env python3
"""Reconstruct the frozen 21L warm/mixed CoE population without rerunning WEPP."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import subprocess
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

METERS_PER_INCH = 0.0254
LEGACY_INCHES_PER_METER = 39.37
LEGACY_METERS_PER_MILE = 1609.0
SECONDS_PER_HOUR = 3600.0
RHO_WATER_KG_M3 = 1000.0
LATENT_HEAT_FUSION_J_KG = 333550.0
TERM_KEYS = ("amelt", "bmelt", "cmelt", "dmelt")
SUBCOMPONENT_KEYS = ("b_temp", "b_clear", "c_open", "c_canopy")


class AuditError(RuntimeError):
    """Raised when a frozen audit invariant does not reproduce."""


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def finite(row: dict[str, str], key: str) -> float:
    try:
        value = float(row[key])
    except (KeyError, ValueError) as error:
        raise AuditError(f"invalid numeric field {key!r}: {row.get(key)!r}") from error
    if not math.isfinite(value):
        raise AuditError(f"non-finite numeric field {key!r}: {value!r}")
    return value


def reconstruct_terms(
    *,
    radiation_mj_m2: float,
    cloud_fraction: float,
    air_temperature_c: float,
    dewpoint_c: float,
    wind_m_s: float,
    rain_m: float,
    canopy_cover_fraction: float,
) -> dict[str, float]:
    """Mirror current Rust operation order and return terms in meters."""
    hrtef = air_temperature_c * (9.0 / 5.0)
    hrdtf = dewpoint_c * (9.0 / 5.0)
    amelt_in = (
        0.0607
        * radiation_mj_m2
        * 1.0
        * (1.0 - canopy_cover_fraction * 1.0)
    )
    b_temp_in = 0.025 / 24.0 * hrtef
    b_clear_in = (
        -(0.84 * (1.0 - cloud_fraction))
        * (1.0 - canopy_cover_fraction * 1.0)
        / 24.0
    )
    bmelt_in = b_temp_in + b_clear_in

    adjustment = 1.57 * 10.0 ** (-1.0 / 6.0)
    wind_mph = wind_m_s * SECONDS_PER_HOUR / LEGACY_METERS_PER_MILE
    if wind_mph > 0.0:
        c_open_in = (
            (0.0084 / 24.0)
            * wind_mph
            * (1.0 - 0.8 * canopy_cover_fraction * 1.0)
            * ((0.22 * hrtef) + (0.78 * hrdtf))
            * adjustment
        )
        c_canopy_in = (
            0.8
            * canopy_cover_fraction
            * 1.0
            * 0.045
            / 24.0
            * hrtef
        )
    else:
        c_open_in = 0.0
        # Current Rust replaces the wind branch with this whole calm-air term.
        c_canopy_in = 0.045 / 24.0 * hrtef
    cmelt_in = c_open_in + c_canopy_in

    rain_in = rain_m * LEGACY_INCHES_PER_METER
    if hrdtf > 0.0:
        dmelt_in = 0.007 * rain_in * hrdtf
    else:
        dmelt_in = 0.007 * rain_in * hrtef

    return {
        "amelt": amelt_in * METERS_PER_INCH,
        "bmelt": bmelt_in * METERS_PER_INCH,
        "cmelt": cmelt_in * METERS_PER_INCH,
        "dmelt": dmelt_in * METERS_PER_INCH,
        "b_temp": b_temp_in * METERS_PER_INCH,
        "b_clear": b_clear_in * METERS_PER_INCH,
        "c_open": c_open_in * METERS_PER_INCH,
        "c_canopy": c_canopy_in * METERS_PER_INCH,
    }


def verify_frozen_inputs(root: Path, freeze: dict[str, Any]) -> list[dict[str, Any]]:
    verified: list[dict[str, Any]] = []
    for name, item in freeze["inputs"].items():
        path = root / item["path"]
        if not path.is_file():
            raise AuditError(f"frozen input is missing: {path}")
        actual_size = path.stat().st_size
        actual_hash = sha256_path(path)
        if actual_size != item["size_bytes"] or actual_hash != item["sha256"]:
            raise AuditError(
                f"frozen input changed: {name}: expected "
                f"{item['size_bytes']}:{item['sha256']}, got {actual_size}:{actual_hash}"
            )
        verified.append(
            {
                "name": name,
                "path": item["path"],
                "size_bytes": actual_size,
                "sha256": actual_hash,
            }
        )

    legacy = freeze["pinned_legacy"]
    command = ["git", "show", f"{legacy['commit']}:{legacy['path']}"]
    result = subprocess.run(
        command,
        cwd=legacy["repository"],
        check=True,
        capture_output=True,
    )
    actual_hash = hashlib.sha256(result.stdout).hexdigest()
    if len(result.stdout) != legacy["size_bytes"] or actual_hash != legacy["sha256"]:
        raise AuditError("pinned legacy blob identity did not reproduce")
    verified.append(
        {
            "name": "pinned_legacy_melt",
            "path": f"{legacy['repository']}@{legacy['commit']}:{legacy['path']}",
            "size_bytes": len(result.stdout),
            "sha256": actual_hash,
        }
    )
    return verified


def parse_climate_daily_midpoints(path: Path) -> dict[str, float]:
    midpoints: dict[str, float] = {}
    with path.open(encoding="utf-8") as stream:
        for line in stream:
            fields = line.split()
            if len(fields) != 13:
                continue
            try:
                day, month, year = map(int, fields[:3])
                tmax_c = float(fields[7])
                tmin_c = float(fields[8])
            except (ValueError, OverflowError):
                continue
            stamp = f"{year:04d}-{month:02d}-{day:02d}"
            if stamp in midpoints:
                raise AuditError(f"duplicate climate date: {stamp}: {path}")
            midpoints[stamp] = (tmax_c + tmin_c) / 2.0
    if not midpoints:
        raise AuditError(f"no climate daily rows: {path}")
    return midpoints


def read_daily_inputs(
    path: Path, climates: dict[str, dict[str, float]]
) -> tuple[dict[tuple[str, str], dict[str, float]], dict[tuple[str, str], dict[str, float]]]:
    joins: dict[tuple[str, str], dict[str, float]] = {}
    expected: dict[tuple[str, str], dict[str, float]] = {}
    with path.open(newline="") as stream:
        for row in csv.DictReader(stream):
            if row["role"] != "CANONICAL" or row["thermal_class"] not in {
                "mixed_day",
                "warm_day",
            }:
                continue
            key = (row["lane"], row["date"])
            if key in joins:
                raise AuditError(f"duplicate daily join key: {key}")
            daily_mean_temp_c = climates.get(row["lane"], {}).get(row["date"])
            if daily_mean_temp_c is None:
                raise AuditError(f"missing climate caller operand for daily row: {key}")
            joins[key] = {
                "wind_m_s": finite(row, "wind_m_s"),
                "dewpoint_c": finite(row, "dewpoint_c"),
                "canopy_cover_fraction": finite(row, "canopy_cover_fraction"),
                "daily_mean_temp_c": daily_mean_temp_c,
            }
            expected[key] = {
                term: finite(row, f"coe_{term}_m") for term in TERM_KEYS
            }
            expected[key].update(
                {f"{term}_positive": finite(row, f"coe_{term}_positive_m") for term in TERM_KEYS}
            )
    if not joins:
        raise AuditError("no canonical warm/mixed daily join rows")
    return joins, expected


def blank_site() -> dict[str, Any]:
    return {
        "eligible_hour_count": 0,
        "day_count": 0,
        "term_sum_m": {key: 0.0 for key in TERM_KEYS},
        "positive_term_sum_m": {key: 0.0 for key in TERM_KEYS},
        "subcomponent_sum_m": {key: 0.0 for key in SUBCOMPONENT_KEYS},
        "applied_sum_m": 0.0,
        "uncapped_sum_m": 0.0,
        "cap_adjustment_sum_m": 0.0,
        "max_abs_term_reconstruction_residual_m": {key: 0.0 for key in TERM_KEYS},
        "max_abs_subcomponent_closure_residual_m": {"bmelt": 0.0, "cmelt": 0.0},
        "max_abs_uncapped_closure_residual_m": 0.0,
        "max_abs_applied_closure_residual_m": 0.0,
        "max_abs_latent_equivalent_flux_w_m2": {
            **{key: 0.0 for key in TERM_KEYS},
            "applied": 0.0,
        },
        "exposure_counts": defaultdict(int),
        "exposure_applied_depth_sum_m": defaultdict(float),
    }


def update_exposures(
    site: dict[str, Any], row: dict[str, str], joined: dict[str, float], terms: dict[str, float]
) -> None:
    air = finite(row, "air_temperature_c")
    dew = joined["dewpoint_c"]
    applied = finite(row, "coe_applied_m")
    density = finite(row, "pack_density_before_kg_m3")
    conditions = {
        "applied_positive": applied > 0.0,
        "applied_positive_air_at_or_below_freezing": applied > 0.0 and air <= 0.0,
        "applied_positive_dewpoint_at_or_below_freezing": applied > 0.0 and dew <= 0.0,
        "applied_positive_air_and_dewpoint_at_or_below_freezing": (
            applied > 0.0 and air <= 0.0 and dew <= 0.0
        ),
        "applied_positive_same_hour_snowfall": applied > 0.0 and finite(row, "snowfall_swe_m") > 0.0,
        "applied_positive_density_below_350_kg_m3": applied > 0.0 and density < 350.0,
        "applied_positive_pack_capped": applied > 0.0 and finite(row, "coe_cap_adjustment_m") < 0.0,
        "c_open_positive_dewpoint_at_or_below_freezing": terms["c_open"] > 0.0 and dew <= 0.0,
        "c_open_negative_air_above_freezing": terms["c_open"] < 0.0 and air > 0.0,
        "b_clear_negative": terms["b_clear"] < 0.0,
    }
    for key, applies in conditions.items():
        if applies:
            site["exposure_counts"][key] += 1
            site["exposure_applied_depth_sum_m"][key] += applied


def analyze(
    daily_path: Path,
    hourly_path: Path,
    tolerance_m: float,
    climates: dict[str, dict[str, float]],
) -> dict[str, Any]:
    joins, daily_expected = read_daily_inputs(daily_path, climates)
    site_data: dict[str, dict[str, Any]] = defaultdict(blank_site)
    seen_days: set[tuple[str, str]] = set()
    daily_actual: dict[tuple[str, str], dict[str, float]] = defaultdict(
        lambda: defaultdict(float)
    )
    selected_rows = 0
    with hourly_path.open(newline="") as stream:
        for row in csv.DictReader(stream):
            if (
                row["role"] != "CANONICAL"
                or row["eligible_hour"] != "True"
                or row["daily_thermal_class"] not in {"mixed_day", "warm_day"}
            ):
                continue
            key = (row["lane"], row["date"])
            joined = joins.get(key)
            if joined is None:
                raise AuditError(f"missing exact daily join for hourly row: {key}")
            caller_bypassed = (
                joined["daily_mean_temp_c"] < 0.0
                or finite(row, "pack_depth_before_m") <= 1.0e-12
            )
            terms = (
                {key: 0.0 for key in (*TERM_KEYS, *SUBCOMPONENT_KEYS)}
                if caller_bypassed
                else reconstruct_terms(
                    radiation_mj_m2=finite(row, "radiation_mj_m2"),
                    cloud_fraction=finite(row, "cloud_fraction"),
                    air_temperature_c=finite(row, "air_temperature_c"),
                    dewpoint_c=joined["dewpoint_c"],
                    wind_m_s=joined["wind_m_s"],
                    rain_m=finite(row, "rain_m"),
                    canopy_cover_fraction=joined["canopy_cover_fraction"],
                )
            )
            site = site_data[row["lane"]]
            site["eligible_hour_count"] += 1
            selected_rows += 1
            seen_days.add(key)

            published = {term: finite(row, f"coe_{term}_m") for term in TERM_KEYS}
            for term in TERM_KEYS:
                residual = terms[term] - published[term]
                site["max_abs_term_reconstruction_residual_m"][term] = max(
                    site["max_abs_term_reconstruction_residual_m"][term], abs(residual)
                )
                site["term_sum_m"][term] += published[term]
                site["positive_term_sum_m"][term] += max(0.0, published[term])
                daily_actual[key][term] += published[term]
                daily_actual[key][f"{term}_positive"] += max(0.0, published[term])
                flux = abs(published[term]) * RHO_WATER_KG_M3 * LATENT_HEAT_FUSION_J_KG / SECONDS_PER_HOUR
                site["max_abs_latent_equivalent_flux_w_m2"][term] = max(
                    site["max_abs_latent_equivalent_flux_w_m2"][term], flux
                )
            for component in SUBCOMPONENT_KEYS:
                site["subcomponent_sum_m"][component] += terms[component]

            b_residual = terms["b_temp"] + terms["b_clear"] - terms["bmelt"]
            c_residual = terms["c_open"] + terms["c_canopy"] - terms["cmelt"]
            site["max_abs_subcomponent_closure_residual_m"]["bmelt"] = max(
                site["max_abs_subcomponent_closure_residual_m"]["bmelt"], abs(b_residual)
            )
            site["max_abs_subcomponent_closure_residual_m"]["cmelt"] = max(
                site["max_abs_subcomponent_closure_residual_m"]["cmelt"], abs(c_residual)
            )

            uncapped = sum(terms[term] for term in TERM_KEYS)
            published_uncapped = finite(row, "coe_uncapped_m")
            site["max_abs_uncapped_closure_residual_m"] = max(
                site["max_abs_uncapped_closure_residual_m"], abs(uncapped - published_uncapped)
            )
            published_applied = finite(row, "coe_applied_m")
            site["max_abs_applied_closure_residual_m"] = max(
                site["max_abs_applied_closure_residual_m"],
                abs(uncapped + finite(row, "coe_cap_adjustment_m") - published_applied),
            )
            site["applied_sum_m"] += published_applied
            site["uncapped_sum_m"] += published_uncapped
            site["cap_adjustment_sum_m"] += finite(row, "coe_cap_adjustment_m")
            applied_flux = (
                abs(published_applied)
                * RHO_WATER_KG_M3
                * LATENT_HEAT_FUSION_J_KG
                / SECONDS_PER_HOUR
            )
            site["max_abs_latent_equivalent_flux_w_m2"]["applied"] = max(
                site["max_abs_latent_equivalent_flux_w_m2"]["applied"], applied_flux
            )
            update_exposures(site, row, joined, terms)

    if selected_rows == 0:
        raise AuditError("no eligible canonical warm/mixed hourly rows")
    if seen_days != set(joins):
        missing = sorted(set(joins) - seen_days)
        raise AuditError(f"daily rows without selected hourly rows: {missing[:5]}")

    max_daily_residual = {term: 0.0 for term in (*TERM_KEYS, *(f"{t}_positive" for t in TERM_KEYS))}
    for key, expected in daily_expected.items():
        for term, expected_value in expected.items():
            max_daily_residual[term] = max(
                max_daily_residual[term], abs(daily_actual[key][term] - expected_value)
            )
    for lane, _date in seen_days:
        site_data[lane]["day_count"] += 1

    maxima: list[float] = []
    result_sites: dict[str, Any] = {}
    for lane in sorted(site_data):
        site = site_data[lane]
        maxima.extend(site["max_abs_term_reconstruction_residual_m"].values())
        maxima.extend(site["max_abs_subcomponent_closure_residual_m"].values())
        maxima.append(site["max_abs_uncapped_closure_residual_m"])
        maxima.append(site["max_abs_applied_closure_residual_m"])
        site["exposure_counts"] = dict(sorted(site["exposure_counts"].items()))
        site["exposure_applied_depth_sum_m"] = dict(
            sorted(site["exposure_applied_depth_sum_m"].items())
        )
        result_sites[lane] = site
    maxima.extend(max_daily_residual.values())
    overall_max = max(maxima)
    if overall_max > tolerance_m:
        raise AuditError(
            f"reconstruction residual {overall_max:.17g} exceeds tolerance {tolerance_m:.17g}"
        )
    return {
        "schema_version": 1,
        "status": "PASS",
        "evidence_class": "Ran",
        "population": "canonical 21L eligible warm/mixed hours",
        "selected_hour_count": selected_rows,
        "selected_day_count": len(seen_days),
        "site_count": len(result_sites),
        "absolute_tolerance_m": tolerance_m,
        "overall_max_abs_reconstruction_residual_m": overall_max,
        "max_abs_daily_aggregation_residual_m": max_daily_residual,
        "magnitude_conversion": {
            "label": "ASSUMED_FOR_DIMENSIONAL_AUDIT",
            "rho_water_kg_m3": RHO_WATER_KG_M3,
            "latent_heat_fusion_j_kg": LATENT_HEAT_FUSION_J_KG,
            "interval_seconds": SECONDS_PER_HOUR,
            "interpretation": "latent-heat-equivalent magnitude only; not energy closure",
        },
        "sites": result_sites,
    }


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path)
    parser.add_argument("--freeze", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    args = parser.parse_args()

    root = args.root.resolve() if args.root else Path(__file__).resolve().parents[4]
    freeze_path = args.freeze.resolve()
    freeze = json.loads(freeze_path.read_text(encoding="utf-8"))
    verified = verify_frozen_inputs(root, freeze)
    rules = freeze["quantitative_rules"]
    climates = {
        item["lane"]: parse_climate_daily_midpoints(root / item["path"])
        for item in freeze["inputs"].values()
        if item.get("role") == "canonical_climate"
    }
    result = analyze(
        root / freeze["inputs"]["daily_table"]["path"],
        root / freeze["inputs"]["hourly_table"]["path"],
        float(rules["absolute_reconstruction_tolerance_m"]),
        climates,
    )
    result["freeze_sha256"] = sha256_path(freeze_path)
    write_json(args.output.resolve(), result)

    tool_path = Path(__file__).resolve()
    receipt = {
        "schema_version": 1,
        "status": "PASS",
        "evidence_class": "Ran",
        "executed_at_utc": datetime.now(timezone.utc).isoformat(),
        "command": "package-local audit_coe_authority.py with explicit freeze/output/receipt",
        "analyzer_sha256": sha256_path(tool_path),
        "freeze_sha256": sha256_path(freeze_path),
        "result_sha256": sha256_path(args.output.resolve()),
        "verified_inputs": verified,
        "selected_hour_count": result["selected_hour_count"],
        "selected_day_count": result["selected_day_count"],
        "overall_max_abs_reconstruction_residual_m": result[
            "overall_max_abs_reconstruction_residual_m"
        ],
    }
    write_json(args.receipt.resolve(), receipt)
    print(json.dumps({"status": "PASS", "result": str(args.output), "receipt": str(args.receipt)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
