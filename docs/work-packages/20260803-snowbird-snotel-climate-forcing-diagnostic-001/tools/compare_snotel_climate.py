#!/usr/bin/env python3
"""Compare the retained Snowbird WEPP climate with normalized NRCS SNOTEL."""

from __future__ import annotations

import csv
import hashlib
import json
import math
from datetime import date, timedelta
from pathlib import Path
from statistics import fmean

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
MANIFEST = PACKAGE / "artifacts/comparison-manifest.json"
OUTPUT = PACKAGE / "artifacts/comparison-results.json"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def pearson(xs: list[float], ys: list[float]) -> float | None:
    if len(xs) < 2:
        return None
    mx, my = fmean(xs), fmean(ys)
    sx = sum((x - mx) ** 2 for x in xs)
    sy = sum((y - my) ** 2 for y in ys)
    if sx == 0.0 or sy == 0.0:
        return None
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / math.sqrt(sx * sy)


def quantile(values: list[float], probability: float) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    position = probability * (len(ordered) - 1)
    lo = int(position)
    hi = min(lo + 1, len(ordered) - 1)
    weight = position - lo
    return ordered[lo] * (1.0 - weight) + ordered[hi] * weight


def metric(pairs: list[tuple[float, float]]) -> dict[str, float | int | None]:
    fixture = [p[0] for p in pairs]
    observed = [p[1] for p in pairs]
    residuals = [a - b for a, b in pairs]
    return {
        "n": len(pairs),
        "fixture_mean": fmean(fixture),
        "snotel_mean": fmean(observed),
        "fixture_minus_snotel_bias": fmean(residuals),
        "mae": fmean(abs(x) for x in residuals),
        "correlation": pearson(fixture, observed),
    }


def load_cli(path: Path) -> dict[date, dict[str, float]]:
    rows: dict[date, dict[str, float]] = {}
    for line in path.read_text().splitlines():
        fields = line.split()
        if len(fields) != 13:
            continue
        try:
            day, month, year = map(int, fields[:3])
            values = list(map(float, fields[3:]))
            stamp = date(year, month, day)
        except ValueError:
            continue
        if stamp in rows:
            raise RuntimeError(f"duplicate fixture date {stamp}")
        rows[stamp] = {
            "precip_mm": values[0],
            "tmax_c": values[4],
            "tmin_c": values[5],
        }
    if not rows or list(rows) != sorted(rows):
        raise RuntimeError("fixture dates are empty, duplicate, or non-monotonic")
    return rows


def optional_float(value: str) -> float | None:
    if value == "":
        return None
    parsed = float(value)
    if not math.isfinite(parsed):
        raise RuntimeError("nonfinite SNOTEL value")
    return parsed


def load_snotel(path: Path) -> dict[date, dict[str, float | int | None]]:
    rows: dict[date, dict[str, float | int | None]] = {}
    with path.open(newline="") as handle:
        for raw in csv.DictReader(handle):
            stamp = date.fromisoformat(raw["date"])
            if stamp in rows:
                raise RuntimeError(f"duplicate SNOTEL date {stamp}")
            rows[stamp] = {
                "water_year": int(raw["water_year"]),
                "precip_cumulative_mm": optional_float(raw["observed_precip_mm"]),
                "tmax_c": optional_float(raw["observed_tmax_c"]),
                "tmin_c": optional_float(raw["observed_tmin_c"]),
            }
    if not rows or list(rows) != sorted(rows):
        raise RuntimeError("SNOTEL dates are empty, duplicate, or non-monotonic")
    previous_stamp: date | None = None
    previous_row: dict[str, float | int | None] | None = None
    for stamp, row in rows.items():
        row["precip_increment_mm"] = None
        if previous_stamp is not None and previous_row is not None:
            current = row["precip_cumulative_mm"]
            previous = previous_row["precip_cumulative_mm"]
            if (
                stamp - previous_stamp == timedelta(days=1)
                and row["water_year"] == previous_row["water_year"]
                and isinstance(current, float)
                and isinstance(previous, float)
            ):
                delta = current - previous
                if delta < -1.0e-9:
                    raise RuntimeError(f"negative cumulative precipitation difference {stamp}: {delta}")
                row["precip_increment_mm"] = max(0.0, delta)
        previous_stamp, previous_row = stamp, row
    return rows


def main() -> int:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    manifest = json.loads(MANIFEST.read_text())
    paths = {
        "climate": ROOT / manifest["climate_path"],
        "observation": ROOT / manifest["observation_path"],
        "provenance": ROOT / manifest["provenance_path"],
    }
    for name, path in paths.items():
        expected = manifest[f"{name}_sha256"]
        if digest(path) != expected:
            raise RuntimeError(f"{name} hash mismatch")
    fixture = load_cli(paths["climate"])
    snotel = load_snotel(paths["observation"])
    common = sorted(set(fixture) & set(snotel))
    if not common:
        raise RuntimeError("no common dates")

    def selected(months: set[int] | None = None) -> list[date]:
        return [d for d in common if months is None or d.month in months]

    populations = {
        "all": selected(),
        "wet_winter": selected(set(manifest["seasons"]["wet_winter"])),
        "snow_season": selected(set(manifest["seasons"]["snow_season"])),
    }
    results: dict[str, object] = {}
    for name, dates in populations.items():
        precip = [
            (fixture[d]["precip_mm"], float(snotel[d]["precip_increment_mm"]))
            for d in dates
            if isinstance(snotel[d]["precip_increment_mm"], float)
        ]
        tmax = [
            (fixture[d]["tmax_c"], float(snotel[d]["tmax_c"]))
            for d in dates
            if isinstance(snotel[d]["tmax_c"], float)
        ]
        tmin = [
            (fixture[d]["tmin_c"], float(snotel[d]["tmin_c"]))
            for d in dates
            if isinstance(snotel[d]["tmin_c"], float)
        ]
        positive_fixture = [a for a, _ in precip if a > 0.0]
        positive_snotel = [b for _, b in precip if b > 0.0]
        results[name] = {
            "precipitation": {
                **metric(precip),
                "fixture_total_mm": sum(a for a, _ in precip),
                "snotel_total_mm": sum(b for _, b in precip),
                "fixture_to_snotel_total_ratio": sum(a for a, _ in precip) / sum(b for _, b in precip),
                "wet_day_threshold_mm": 0.254,
                "fixture_wet_days": sum(a >= 0.254 for a, _ in precip),
                "snotel_wet_days": sum(b >= 0.254 for _, b in precip),
                "wet_day_agreement_fraction": fmean((a >= 0.254) == (b >= 0.254) for a, b in precip),
                "positive_event_quantiles_mm": {
                    "fixture": {str(q): quantile(positive_fixture, q) for q in (0.5, 0.9, 0.99)},
                    "snotel": {str(q): quantile(positive_snotel, q) for q in (0.5, 0.9, 0.99)},
                },
            },
            "tmax_c": metric(tmax),
            "tmin_c": metric(tmin),
        }

    boundary_intervals = []
    for year in range(1990, 2025):
        start, end = date(year - 1, 10, 1), date(year, 9, 30)
        if start not in snotel or end not in snotel:
            continue
        sv0 = snotel[start]["precip_cumulative_mm"]
        sv1 = snotel[end]["precip_cumulative_mm"]
        fixture_dates = [d for d in fixture if start < d <= end]
        if (
            not isinstance(sv0, float)
            or not isinstance(sv1, float)
            or len(fixture_dates) != (end - start).days
        ):
            continue
        observed_total = sv1 - sv0
        if observed_total <= 0.0:
            continue
        fixture_total = sum(fixture[d]["precip_mm"] for d in fixture_dates)
        boundary_intervals.append({
            "water_year": year,
            "fixture_total_mm": fixture_total,
            "snotel_total_mm": observed_total,
            "fixture_to_snotel_ratio": fixture_total / observed_total,
        })

    receipt = {
        "schema_version": 1,
        "status": "SNOTEL_CLIMATE_COMPARISON_COMPLETE",
        "manifest_sha256": digest(MANIFEST),
        "source_hashes": {name: digest(path) for name, path in paths.items()},
        "common_period": [common[0].isoformat(), common[-1].isoformat()],
        "common_date_count": len(common),
        "results": results,
        "october1_to_september30_boundary_intervals": boundary_intervals,
        "boundary_interval_summary": {
            "n": len(boundary_intervals),
            "mean_fixture_to_snotel_ratio": fmean(x["fixture_to_snotel_ratio"] for x in boundary_intervals),
            "median_fixture_to_snotel_ratio": quantile([x["fixture_to_snotel_ratio"] for x in boundary_intervals], 0.5),
            "intervals_below_0_75": sum(x["fixture_to_snotel_ratio"] < 0.75 for x in boundary_intervals),
        },
        "claim_limits": manifest["claim_limits"],
    }
    OUTPUT.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    print("SNOTEL_CLIMATE_COMPARISON_COMPLETE", len(common), len(boundary_intervals))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
