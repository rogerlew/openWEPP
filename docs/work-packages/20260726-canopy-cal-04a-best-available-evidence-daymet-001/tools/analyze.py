#!/usr/bin/env python3
"""Build deterministic Daymet forcing and calibration-design evidence."""

from __future__ import annotations

import csv
import math
import re
from collections import defaultdict
from datetime import date, timedelta
from pathlib import Path
from statistics import mean, median


ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
RAW = ROOT / "references/canopy_phenology/daymet_calibration/raw"
GEOMETRY = ARTIFACTS / "hubbard-plot-geometry.csv"
TIMING = (
    ROOT
    / "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001"
    / "artifacts/cal04-timing-windows.csv"
)
FIXTURE = ROOT / "tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.cli"


def es_kpa(temperature_c: float) -> float:
    return 0.6108 * math.exp(17.27 * temperature_c / (temperature_c + 237.3))


def photoperiod_hours(latitude_deg: float, ordinal_day: int) -> float:
    latitude = math.radians(latitude_deg)
    day = float(ordinal_day)
    declination = 0.409 * math.sin((2.0 * math.pi * day / 365.0) - 1.39)
    sunset_cosine = -math.tan(latitude) * math.tan(declination)
    sunset_angle = math.acos(max(-1.0, min(1.0, sunset_cosine)))
    return 24.0 * sunset_angle / math.pi


def quantile(values: list[float], probability: float) -> float:
    ordered = sorted(values)
    position = (len(ordered) - 1) * probability
    lower = math.floor(position)
    upper = math.ceil(position)
    if lower == upper:
        return ordered[lower]
    fraction = position - lower
    return ordered[lower] * (1.0 - fraction) + ordered[upper] * fraction


def correlation(left: list[float], right: list[float]) -> float:
    left_mean = mean(left)
    right_mean = mean(right)
    numerator = sum(
        (x - left_mean) * (y - right_mean) for x, y in zip(left, right, strict=True)
    )
    denominator = math.sqrt(
        sum((x - left_mean) ** 2 for x in left)
        * sum((y - right_mean) ** 2 for y in right)
    )
    return numerator / denominator


def regression_slope(x: list[float], y: list[float]) -> float:
    x_mean = mean(x)
    y_mean = mean(y)
    denominator = sum((value - x_mean) ** 2 for value in x)
    return sum(
        (a - x_mean) * (b - y_mean) for a, b in zip(x, y, strict=True)
    ) / denominator


def read_geometry() -> dict[str, dict[str, float | str]]:
    with GEOMETRY.open(newline="", encoding="utf-8") as stream:
        return {
            row["plot_id"]: {
                **row,
                "latitude_deg": float(row["latitude_deg"]),
                "longitude_deg": float(row["longitude_deg"]),
                "source_elevation_m": float(row["source_elevation_m"]),
            }
            for row in csv.DictReader(stream)
        }


def daymet_date(year: int, yday: int) -> date:
    # Daymet retains February 29 and discards December 31 in leap years.
    return date(year, 1, 1) + timedelta(days=yday - 1)


def read_daymet(
    geometry: dict[str, dict[str, float | str]],
) -> tuple[list[dict[str, object]], dict[tuple[str, int, int], dict[str, object]]]:
    rows: list[dict[str, object]] = []
    index: dict[tuple[str, int, int], dict[str, object]] = {}
    for plot, geo in geometry.items():
        path = RAW / f"hubbard_{plot}_daymet_v4r1_1989_2024.csv"
        with path.open(newline="", encoding="utf-8") as stream:
            metadata = [next(stream).rstrip("\n") for _ in range(6)]
            match = re.fullmatch(r"Elevation: ([0-9]+) meters", metadata[3])
            if match is None:
                raise ValueError(f"unrecognized Daymet elevation: {metadata[3]}")
            grid_elevation_m = int(match.group(1))
            reader = csv.DictReader(stream)
            count = 0
            for source in reader:
                year = int(source["year"])
                yday = int(source["yday"])
                tmax = float(source["tmax (deg c)"])
                tmin = float(source["tmin (deg c)"])
                vp_pa = float(source["vp (Pa)"])
                dayl_hours = float(source["dayl (s)"]) / 3600.0
                vpd_pa = (
                    0.5 * (es_kpa(tmax) + es_kpa(tmin)) - vp_pa / 1000.0
                ) * 1000.0
                if not math.isfinite(vpd_pa) or vpd_pa < 0.0:
                    raise ValueError(
                        f"invalid Daymet-derived VPD {vpd_pa} at {plot} {year}-{yday}"
                    )
                native_dayl = photoperiod_hours(float(geo["latitude_deg"]), yday)
                row: dict[str, object] = {
                    "plot_id": plot,
                    "year": year,
                    "yday": yday,
                    "date": daymet_date(year, yday).isoformat(),
                    "source_elevation_m": f"{float(geo['source_elevation_m']):.4f}",
                    "daymet_grid_elevation_m": grid_elevation_m,
                    "elevation_error_m": f"{grid_elevation_m - float(geo['source_elevation_m']):.4f}",
                    "tmax_c": f"{tmax:.2f}",
                    "tmin_c": f"{tmin:.2f}",
                    "vp_pa": f"{vp_pa:.2f}",
                    "derived_vpd_pa": f"{vpd_pa:.6f}",
                    "daymet_daylength_hours": f"{dayl_hours:.6f}",
                    "native_photoperiod_hours": f"{native_dayl:.12f}",
                    "daylength_difference_minutes": f"{(dayl_hours - native_dayl) * 60.0:.6f}",
                }
                rows.append(row)
                index[(plot, year, yday)] = row
                count += 1
            if count != 36 * 365:
                raise ValueError(f"{plot} returned {count} rows, expected 13140")
    return rows, index


def read_fixture() -> tuple[list[dict[str, object]], dict[tuple[int, int], dict[str, object]]]:
    rows: list[dict[str, object]] = []
    index: dict[tuple[int, int], dict[str, object]] = {}
    with FIXTURE.open(encoding="utf-8") as stream:
        for line in stream:
            fields = line.split()
            if len(fields) != 13:
                continue
            try:
                day, month, year = map(int, fields[:3])
                tmax = float(fields[7])
                tmin = float(fields[8])
                dewpoint = float(fields[12])
            except ValueError:
                continue
            if year < 1989:
                continue
            calendar = date(year, month, day)
            yday = calendar.timetuple().tm_yday
            vpd_pa = (
                0.5 * (es_kpa(tmax) + es_kpa(tmin)) - es_kpa(dewpoint)
            ) * 1000.0
            if not math.isfinite(vpd_pa) or vpd_pa < 0.0:
                raise ValueError(f"invalid fixture VPD {vpd_pa} at {calendar}")
            row: dict[str, object] = {
                "year": year,
                "yday": yday,
                "date": calendar.isoformat(),
                "tmax_c": f"{tmax:.2f}",
                "tmin_c": f"{tmin:.2f}",
                "dewpoint_c": f"{dewpoint:.2f}",
                "derived_vpd_pa": f"{vpd_pa:.6f}",
                "native_photoperiod_hours": f"{photoperiod_hours(43.94, yday):.12f}",
            }
            rows.append(row)
            index[(year, yday)] = row
    if len(rows) != 36 * 365 + 9:
        # Gregorian fixture retains leap-day and December 31, unlike Daymet.
        raise ValueError(f"fixture returned unexpected 1989-2024 row count {len(rows)}")
    return rows, index


def rolling(
    index: dict[tuple[str, int, int], dict[str, object]],
    plot: str,
    year: int,
    end_yday: int,
    field: str,
    days: int = 21,
) -> float:
    values = [
        float(index[(plot, year, yday)][field])
        for yday in range(end_yday - days + 1, end_yday + 1)
    ]
    return mean(values)


def build_join(
    geometry: dict[str, dict[str, float | str]],
    index: dict[tuple[str, int, int], dict[str, object]],
) -> list[dict[str, object]]:
    joined: list[dict[str, object]] = []
    with TIMING.open(newline="", encoding="utf-8") as stream:
        for observation in csv.DictReader(stream):
            if observation["role"] != "CALIBRATION":
                continue
            plot = observation["site"]
            year = int(observation["year"])
            start = int(observation["interval_start_doy"])
            end = int(observation["interval_end_doy"])
            before = index[(plot, year, start)]
            after = index[(plot, year, end)]
            joined.append(
                {
                    "record_id": observation["record_id"],
                    "year": year,
                    "species": observation["species"],
                    "plot_id": plot,
                    "source_elevation_m": f"{float(geometry[plot]['source_elevation_m']):.4f}",
                    "interval_start_doy": start,
                    "interval_end_doy": end,
                    "interval_width_days": end - start,
                    "descriptive_midpoint_doy_not_truth": f"{0.5 * (start + end):.3f}",
                    "start_tmin_c": before["tmin_c"],
                    "end_tmin_c": after["tmin_c"],
                    "end_21d_mean_tmin_c": f"{rolling(index, plot, year, end, 'tmin_c'):.6f}",
                    "start_vpd_pa": before["derived_vpd_pa"],
                    "end_vpd_pa": after["derived_vpd_pa"],
                    "end_21d_mean_vpd_pa": f"{rolling(index, plot, year, end, 'derived_vpd_pa'):.6f}",
                    "start_photoperiod_hours": before["native_photoperiod_hours"],
                    "end_photoperiod_hours": after["native_photoperiod_hours"],
                    "end_21d_mean_photoperiod_hours": f"{rolling(index, plot, year, end, 'native_photoperiod_hours'):.9f}",
                }
            )
    if len(joined) != 932:
        raise ValueError(f"joined {len(joined)} calibration intervals, expected 932")
    return joined


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise ValueError(f"refusing empty output {path}")
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def forcing_support(daymet_rows: list[dict[str, object]]) -> list[dict[str, object]]:
    spring = [row for row in daymet_rows if 60 <= int(row["yday"]) <= 180]
    fields = {
        "tmin_c": "tmin_c",
        "derived_vpd_pa": "vpd_pa",
        "native_photoperiod_hours": "photoperiod_hours",
    }
    result: list[dict[str, object]] = []
    for field, operand in fields.items():
        values = [float(row[field]) for row in spring]
        for label, probability in [
            ("q00", 0.0),
            ("q05", 0.05),
            ("q25", 0.25),
            ("q50", 0.50),
            ("q75", 0.75),
            ("q95", 0.95),
            ("q100", 1.0),
        ]:
            result.append(
                {
                    "support_population": "all_plots_all_years_yday_060_180",
                    "operand": operand,
                    "quantile": label,
                    "value": f"{quantile(values, probability):.9f}",
                    "n": len(values),
                    "role": "forcing_support_not_probability_prior",
                }
            )
    return result


def proposed_grid(support: list[dict[str, object]]) -> list[dict[str, object]]:
    by_operand: dict[str, list[dict[str, object]]] = defaultdict(list)
    for row in support:
        by_operand[str(row["operand"])].append(row)
    pairs = [
        ("temperature", "tmin_c", "minimum_temperature_inactive_c", "minimum_temperature_unconstrained_c"),
        ("vpd", "vpd_pa", "vapor_pressure_deficit_unconstrained_pa", "vapor_pressure_deficit_inactive_pa"),
        ("photoperiod", "photoperiod_hours", "photoperiod_inactive_hours", "photoperiod_unconstrained_hours"),
    ]
    result: list[dict[str, object]] = []
    for family, support_operand, lower_name, upper_name in pairs:
        levels = by_operand[support_operand]
        for lower_index, lower in enumerate(levels):
            for upper_index, upper in enumerate(levels):
                if lower_index >= upper_index:
                    continue
                result.append(
                    {
                        "family": family,
                        "pair_id": f"{family}-{lower['quantile']}-{upper['quantile']}",
                        "lower_operand": lower_name,
                        "lower_level": lower["quantile"],
                        "lower_value": lower["value"],
                        "upper_operand": upper_name,
                        "upper_level": upper["quantile"],
                        "upper_value": upper["value"],
                        "role": "forcing_support_grid_not_probability_prior",
                    }
                )
    if len(result) != 63:
        raise ValueError(f"expected 63 pair rows, found {len(result)}")
    return result


def fixture_comparison(
    daymet_rows: list[dict[str, object]],
    fixture_index: dict[tuple[int, int], dict[str, object]],
) -> list[dict[str, object]]:
    grouped: dict[tuple[int, int], list[dict[str, object]]] = defaultdict(list)
    for row in daymet_rows:
        grouped[(int(row["year"]), int(row["yday"]))].append(row)
    output: list[dict[str, object]] = []
    for (year, yday), rows in sorted(grouped.items()):
        fixture = fixture_index[(year, yday)]
        output.append(
            {
                "year": year,
                "yday": yday,
                "date": fixture["date"],
                "plot_mean_tmin_c": f"{mean(float(row['tmin_c']) for row in rows):.6f}",
                "fixture_tmin_c": fixture["tmin_c"],
                "tmin_difference_c": f"{float(fixture['tmin_c']) - mean(float(row['tmin_c']) for row in rows):.6f}",
                "plot_mean_vpd_pa": f"{mean(float(row['derived_vpd_pa']) for row in rows):.6f}",
                "fixture_vpd_pa": fixture["derived_vpd_pa"],
                "vpd_difference_pa": f"{float(fixture['derived_vpd_pa']) - mean(float(row['derived_vpd_pa']) for row in rows):.6f}",
            }
        )
    return output


def write_analysis_docs(
    geometry: dict[str, dict[str, float | str]],
    daymet_rows: list[dict[str, object]],
    joined: list[dict[str, object]],
    comparison: list[dict[str, object]],
) -> None:
    spring = [row for row in daymet_rows if 60 <= int(row["yday"]) <= 180]
    tmin = [float(row["tmin_c"]) for row in spring]
    vpd = [float(row["derived_vpd_pa"]) for row in spring]
    photo = [float(row["native_photoperiod_hours"]) for row in spring]
    elevations = [float(row["source_elevation_m"]) for row in spring]
    matrix = {
        "tmin_vpd": correlation(tmin, vpd),
        "tmin_photo": correlation(tmin, photo),
        "vpd_photo": correlation(vpd, photo),
        "elevation_tmin": correlation(elevations, tmin),
        "elevation_vpd": correlation(elevations, vpd),
    }

    centered_x: list[float] = []
    centered_mid: list[float] = []
    centered_lower: list[float] = []
    centered_upper: list[float] = []
    groups: dict[tuple[int, str], list[dict[str, object]]] = defaultdict(list)
    for row in joined:
        groups[(int(row["year"]), str(row["species"]))].append(row)
    for rows in groups.values():
        if len(rows) < 2:
            continue
        elevation_mean = mean(float(row["source_elevation_m"]) for row in rows)
        midpoint_mean = mean(float(row["descriptive_midpoint_doy_not_truth"]) for row in rows)
        lower_mean = mean(float(row["interval_start_doy"]) for row in rows)
        upper_mean = mean(float(row["interval_end_doy"]) for row in rows)
        for row in rows:
            centered_x.append(float(row["source_elevation_m"]) - elevation_mean)
            centered_mid.append(float(row["descriptive_midpoint_doy_not_truth"]) - midpoint_mean)
            centered_lower.append(float(row["interval_start_doy"]) - lower_mean)
            centered_upper.append(float(row["interval_end_doy"]) - upper_mean)
    slope_mid = regression_slope(centered_x, centered_mid) * 100.0
    slope_lower = regression_slope(centered_x, centered_lower) * 100.0
    slope_upper = regression_slope(centered_x, centered_upper) * 100.0

    source_errors = [
        int(row["daymet_grid_elevation_m"]) - float(row["source_elevation_m"])
        for row in daymet_rows[:: 36 * 365]
    ]
    tmin_diffs = [float(row["tmin_difference_c"]) for row in comparison]
    vpd_diffs = [float(row["vpd_difference_pa"]) for row in comparison]

    fixed_window: dict[tuple[str, int], list[dict[str, object]]] = defaultdict(list)
    for row in daymet_rows:
        if 60 <= int(row["yday"]) <= 120:
            fixed_window[(str(row["plot_id"]), int(row["year"]))].append(row)
    observation_midpoints: dict[tuple[str, int], list[float]] = defaultdict(list)
    for row in joined:
        observation_midpoints[(str(row["plot_id"]), int(row["year"]))].append(
            float(row["descriptive_midpoint_doy_not_truth"])
        )
    association_rows: list[tuple[str, float, float, float]] = []
    for key, midpoints in observation_midpoints.items():
        forcing = fixed_window[key]
        association_rows.append(
            (
                key[0],
                mean(midpoints),
                mean(float(row["tmin_c"]) for row in forcing),
                mean(float(row["derived_vpd_pa"]) for row in forcing),
            )
        )
    anomalies: list[tuple[float, float, float]] = []
    by_plot: dict[str, list[tuple[str, float, float, float]]] = defaultdict(list)
    for row in association_rows:
        by_plot[row[0]].append(row)
    for rows in by_plot.values():
        midpoint_mean = mean(row[1] for row in rows)
        tmin_mean = mean(row[2] for row in rows)
        vpd_mean = mean(row[3] for row in rows)
        anomalies.extend(
            (
                row[1] - midpoint_mean,
                row[2] - tmin_mean,
                row[3] - vpd_mean,
            )
            for row in rows
        )
    midpoint_anomaly = [row[0] for row in anomalies]
    tmin_anomaly = [row[1] for row in anomalies]
    vpd_anomaly = [row[2] for row in anomalies]
    r_mid_tmin = correlation(midpoint_anomaly, tmin_anomaly)
    r_mid_vpd = correlation(midpoint_anomaly, vpd_anomaly)
    r_tmin_vpd = correlation(tmin_anomaly, vpd_anomaly)
    denominator = 1.0 - r_tmin_vpd**2
    beta_tmin = (r_mid_tmin - r_tmin_vpd * r_mid_vpd) / denominator
    beta_vpd = (r_mid_vpd - r_tmin_vpd * r_mid_tmin) / denominator

    (ARTIFACTS / "correlation-and-confounding.md").write_text(
        f"""# Correlation and Confounding

Evidence class: `Ran deterministic forcing analysis`

Population: all nine plots, 1989–2024, Daymet days 60–180
(`n={len(spring):,}` plot-days).

| Pair | Pearson correlation |
| --- | ---: |
| Tmin / VPD | {matrix['tmin_vpd']:.4f} |
| Tmin / photoperiod | {matrix['tmin_photo']:.4f} |
| VPD / photoperiod | {matrix['vpd_photo']:.4f} |
| source elevation / Tmin | {matrix['elevation_tmin']:.4f} |
| source elevation / VPD | {matrix['elevation_vpd']:.4f} |

The seasonal correlations demonstrate that six GSI thresholds cannot be
assumed independently identifiable. The later calibration must retain complete
profiles and correlated/equifinal families. These correlations describe the
forcing population and are not probability priors.

Across the full daily comparison, protected-fixture minus nine-plot Daymet
mean Tmin has mean {mean(tmin_diffs):.3f} °C and median
{median(tmin_diffs):.3f} °C. The corresponding VPD difference has mean
{mean(vpd_diffs):.1f} Pa and median {median(vpd_diffs):.1f} Pa. The fixture
therefore cannot be treated as a byte-equivalent representation of all plot
microclimates.
""",
        encoding="utf-8",
    )
    (ARTIFACTS / "elevation-analysis.md").write_text(
        f"""# Elevation Analysis

Evidence class: `Ran deterministic descriptive analysis`

The EML plot elevations span
{min(float(row['source_elevation_m']) for row in geometry.values()):.1f} to
{max(float(row['source_elevation_m']) for row in geometry.values()):.1f} m.
Daymet grid elevations span
{min(int(row['daymet_grid_elevation_m']) for row in daymet_rows):d} to
{max(int(row['daymet_grid_elevation_m']) for row in daymet_rows):d} m.
Grid-minus-source elevation error spans {min(source_errors):.1f} to
{max(source_errors):.1f} m.

After centering within year and species, descriptive P3 interval slopes are:

- lower interval bound: {slope_lower:.3f} days per 100 m;
- upper interval bound: {slope_upper:.3f} days per 100 m;
- interval midpoint diagnostic: {slope_mid:.3f} days per 100 m.

The midpoint is explicitly not treated as an observed transition date. The
lower/upper slopes bracket sensitivity to weekly interval censoring. These
descriptive values may check consistency with the retained 2.7 ± 0.4 days per
100 m summary, but cannot replace an interval-censored calibration objective.

Daymet is a 1-km gridded estimate and smooths plot topography. It is suitable
for forcing-support and anomaly analysis, not proof of plot microclimate.
""",
        encoding="utf-8",
    )
    (ARTIFACTS / "vpd-and-photoperiod-method.md").write_text(
        """# VPD and Photoperiod Method

Evidence class: `Static equation binding + Ran deterministic derivation`

Daymet supplies daily `Tmax`, `Tmin` in degrees Celsius, actual vapor pressure
`VP` in pascals, and day length in seconds. Derived VPD uses the native
runner's saturation and daily-mean saturation algebra:

`es(T) = 0.6108 * exp(17.27*T/(T+237.3))` kPa

`VPD = 1000 * (0.5*(es(Tmax)+es(Tmin)) - VP/1000)` Pa.

The native climate path obtains actual vapor pressure as `es(dewpoint)`,
whereas this analysis uses Daymet's supplied daily-average actual VP directly.
Thus the saturation/deficit algebra is native-equivalent, but the actual-VP
source is not identical.

The derivation rejects negative or non-finite VPD; it does not clamp. All
118,260 plot-days passed.

Native photoperiod mirrors `openwepp-plant-phenology` FAO-56 geometry:
declination `0.409*sin(2*pi*d/365 - 1.39)`, bounded sunset cosine, and
`24*omega/pi` hours. Daymet day length is retained separately for comparison.

Daymet uses 365 records in every year: leap day is present and December 31 is
discarded in leap years. Spring P3 joins use Daymet `yday`, which matches
Gregorian ordinal days through the spring observation period.
""",
        encoding="utf-8",
    )
    (ARTIFACTS / "phenology-anomaly-association.md").write_text(
        f"""# Phenology and Fixed-Window Forcing Anomalies

Evidence class: `Ran deterministic descriptive association`

To avoid defining meteorological predictors relative to the observed event
date, this analysis uses the fixed Daymet window yday 60–120. One descriptive
P3 interval midpoint is averaged per plot-year only for association
diagnostics; it is not treated as the calibration observation or exact event
date. Plot means are removed, leaving within-plot interannual anomalies
(`n={len(anomalies)}` plot-years).

| Association | Pearson correlation |
| --- | ---: |
| P3 timing anomaly / mean Tmin anomaly | {r_mid_tmin:.4f} |
| P3 timing anomaly / mean VPD anomaly | {r_mid_vpd:.4f} |
| mean Tmin anomaly / mean VPD anomaly | {r_tmin_vpd:.4f} |

Warmer and higher-VPD early springs are associated with earlier P3 brackets
(negative timing correlations). In a standardized two-predictor descriptive
regression, coefficients are {beta_tmin:.4f} for Tmin and {beta_vpd:.4f} for
VPD. The weaker conditional VPD coefficient and predictor correlation show
that the observations contain temperature leverage but limited independent VPD
leverage.

This is not a fitted GSI model, causal attribution, or threshold estimate.
Photoperiod is omitted from the fixed-window anomaly regression because it is
deterministic by latitude/calendar and has negligible interannual variation at
a fixed plot. Threshold calibration must still profile the complete GSI vector
and retain equifinality.
""",
        encoding="utf-8",
    )


def main() -> int:
    geometry = read_geometry()
    daymet_rows, daymet_index = read_daymet(geometry)
    fixture_rows, fixture_index = read_fixture()
    joined = build_join(geometry, daymet_index)
    support = forcing_support(daymet_rows)
    grid = proposed_grid(support)
    comparison = fixture_comparison(daymet_rows, fixture_index)

    write_csv(ARTIFACTS / "daymet-daily-derived.csv", daymet_rows)
    write_csv(ARTIFACTS / "fixture-daily-derived.csv", fixture_rows)
    write_csv(ARTIFACTS / "phenology-forcing-join.csv", joined)
    write_csv(ARTIFACTS / "forcing-support-summary.csv", support)
    write_csv(ARTIFACTS / "proposed-domain-grid.csv", grid)
    write_csv(ARTIFACTS / "fixture-daymet-comparison.csv", comparison)
    write_analysis_docs(geometry, daymet_rows, joined, comparison)
    print(
        f"PASS analysis: {len(daymet_rows)} Daymet rows, {len(fixture_rows)} "
        f"fixture rows, {len(joined)} intervals, {len(grid)} pair-grid rows"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
