#!/usr/bin/env python3
"""Regenerate EB-02 analytical vectors and accessible SVG figures."""

from __future__ import annotations

import csv
import math
from pathlib import Path
from typing import Callable


ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = ROOT / "artifacts"
FIGURES = ARTIFACTS / "figures"
SIGMA = 5.670374419e-8
# Demonstration-only value used to execute a threshold-insensitive R_a=0
# polar-night guard vector. It is not canonical authority; EB-03 must bind
# R_a,min before runtime implementation.
ASSUMED_FOR_EXECUTION_EXTRATERRESTRIAL_MIN_MJ_M2_DAY = 1.0e-9


class CloudForcingUnavailable(ValueError):
    """Daily clearness cannot be evaluated for this daylight state."""


class AtmosphericStateOutOfAuthority(ValueError):
    """An empirical atmospheric result violates its physical output domain."""


def sky_view(cover: float) -> float:
    if not math.isfinite(cover) or not 0.0 <= cover < 1.0:
        raise ValueError("cover must be finite and in [0, 1)")
    return (1.0 - cover) ** 1.6


def cloud_fraction(clearness: float) -> float:
    if not math.isfinite(clearness):
        raise ValueError("clearness must be finite")
    return min(1.0, max(0.0, (0.80 - clearness) / 0.65))


def infer_cloud_fraction(
    solar_mj_m2_day: float,
    extraterrestrial_mj_m2_day: float,
    *,
    daylight: bool,
    extraterrestrial_min_mj_m2_day: float,
) -> float:
    for name, value in (
        ("solar radiation", solar_mj_m2_day),
        ("extraterrestrial radiation", extraterrestrial_mj_m2_day),
    ):
        if not math.isfinite(value) or value < 0.0:
            raise ValueError(f"{name} must be finite and non-negative")
    if (
        not daylight
        or extraterrestrial_mj_m2_day <= extraterrestrial_min_mj_m2_day
    ):
        raise CloudForcingUnavailable("daily clearness unavailable")
    return cloud_fraction(solar_mj_m2_day / extraterrestrial_mj_m2_day)


def atmospheric_longwave(
    air_temperature_k: float, vapor_pressure_kpa: float, clearness: float
) -> tuple[float, float, float, float]:
    if not math.isfinite(air_temperature_k) or air_temperature_k <= 0.0:
        raise ValueError("air temperature must be finite and positive kelvin")
    if not math.isfinite(vapor_pressure_kpa) or vapor_pressure_kpa < 0.0:
        raise ValueError("vapor pressure must be finite and non-negative")
    water = 4650.0 * vapor_pressure_kpa / air_temperature_k
    clear = (
        59.38
        + 113.7 * (air_temperature_k / 273.16) ** 6
        + 96.96 * math.sqrt(water / 25.0)
    )
    epsilon_clear = clear / (SIGMA * air_temperature_k**4)
    cloud = cloud_fraction(clearness)
    epsilon_all = (1.0 - 0.84 * cloud) * epsilon_clear + 0.84 * cloud
    atmospheric = epsilon_all * SIGMA * air_temperature_k**4
    derived = (water, clear, epsilon_clear, cloud, epsilon_all, atmospheric)
    if not all(math.isfinite(value) for value in derived):
        raise AtmosphericStateOutOfAuthority("non-finite atmospheric result")
    if not 0.0 <= epsilon_clear <= 1.0 or not 0.0 <= epsilon_all <= 1.0:
        raise AtmosphericStateOutOfAuthority(
            "effective atmospheric emissivity outside [0, 1]"
        )
    return water, clear, cloud, atmospheric


def longwave_components(
    cover: float,
    atmospheric_w_m2: float,
    canopy_temperature_k: float,
    snow_temperature_k: float,
) -> tuple[float, float, float, float]:
    for name, value in (
        ("atmospheric longwave", atmospheric_w_m2),
        ("canopy temperature", canopy_temperature_k),
        ("snow temperature", snow_temperature_k),
    ):
        if not math.isfinite(value):
            raise ValueError(f"{name} must be finite")
    if atmospheric_w_m2 < 0.0:
        raise ValueError("atmospheric longwave must be non-negative")
    if canopy_temperature_k <= 0.0 or snow_temperature_k <= 0.0:
        raise ValueError("thermal temperatures must be positive kelvin")
    view = sky_view(cover)
    canopy = SIGMA * canopy_temperature_k**4
    subcanopy = view * atmospheric_w_m2 + (1.0 - view) * canopy
    outgoing = SIGMA * snow_temperature_k**4
    return view, canopy, subcanopy, subcanopy - outgoing


def write_vectors() -> int:
    rows: list[dict[str, str]] = []

    def add(
        case_id: str,
        category: str,
        inputs: str,
        expected: str,
        observed: str,
        units: str,
        status: str = "PASS",
    ) -> None:
        rows.append(
            {
                "case_id": case_id,
                "category": category,
                "inputs": inputs,
                "expected": expected,
                "observed": observed,
                "units": units,
                "status": status,
            }
        )

    def add_numeric(
        case_id: str,
        category: str,
        inputs: str,
        expected: float,
        observed: float,
        units: str,
        tolerance: float,
    ) -> None:
        status = (
            "PASS"
            if math.isfinite(observed) and abs(observed - expected) <= tolerance
            else "FAIL"
        )
        add(
            case_id,
            category,
            inputs,
            f"{expected:.12f}",
            f"{observed:.12f}",
            units,
            status,
        )

    expected_sky_view = {
        0.0: 1.000000000000,
        0.2: 0.699751727324,
        0.5: 0.329876977693,
        0.9: 0.025118864315,
        0.999: 0.000015848932,
    }
    for cover, expected in expected_sky_view.items():
        view = sky_view(cover)
        add_numeric(
            f"sky_view_c{cover:g}",
            "sky_view",
            f"C={cover:.6f}",
            expected,
            view,
            "fraction",
            1.0e-9,
        )

    for clearness, expected in {
        1.1: 0.0,
        0.8: 0.0,
        0.475: 0.5,
        0.15: 1.0,
        0.0: 1.0,
    }.items():
        observed = cloud_fraction(clearness)
        add_numeric(
            f"cloud_kt{clearness:g}",
            "cloud_mapping",
            f"k_t={clearness:.6f}",
            expected,
            observed,
            "fraction",
            1.0e-9,
        )

    water, clear, cloud, atmospheric = atmospheric_longwave(273.15, 0.611, 0.475)
    add_numeric("water_reference", "atmospheric", "hour_T_a=273.15;daily_e_a=0.611", 10.401427786930, water, "kg m^-2", 1.0e-9)
    add_numeric("clear_reference", "atmospheric", "hour_T_a=273.15;daily_e_a=0.611", 235.596641453254, clear, "W m^-2", 1.0e-6)
    add_numeric("cloud_reference", "atmospheric", "daily_k_t=0.475", 0.500000000000, cloud, "fraction", 1.0e-9)
    add_numeric("all_sky_reference", "atmospheric", "hour_T_a=273.15;daily_e_a=0.611;daily_k_t=0.475", 269.222337409225, atmospheric, "W m^-2", 1.0e-6)
    _, _, _, cold_hour = atmospheric_longwave(263.15, 0.611, 0.475)
    _, _, _, warm_hour = atmospheric_longwave(283.15, 0.611, 0.475)
    add_numeric(
        "all_sky_cold_hour",
        "atmospheric_cadence",
        "hour_T_a=263.15;daily_e_a=0.611;daily_k_t=0.475",
        238.311138592317,
        cold_hour,
        "W m^-2",
        1.0e-6,
    )
    add_numeric(
        "all_sky_warm_hour",
        "atmospheric_cadence",
        "hour_T_a=283.15;daily_e_a=0.611;daily_k_t=0.475",
        304.957360458777,
        warm_hour,
        "W m^-2",
        1.0e-6,
    )
    add_numeric(
        "daily_mean_temperature_substitution_bias",
        "atmospheric_cadence",
        "mean(hour_T_a)=273.15;compare mean(hour_L) against L(mean_T)",
        2.411912116322,
        0.5 * (cold_hour + warm_hour) - atmospheric,
        "W m^-2",
        1.0e-6,
    )

    expected_longwave = {
        0.0: (269.222337409225, -23.949967759869),
        0.5: (294.212270964908, 1.039965795814),
        0.9: (305.577176736476, 12.404871567381),
    }
    for cover, (expected_subcanopy, expected_net) in expected_longwave.items():
        _, _, subcanopy, net = longwave_components(
            cover, atmospheric, 271.15, 268.15
        )
        add_numeric(
            f"subcanopy_c{cover:g}",
            "longwave_mixture",
            f"C={cover:.6f};L_atm={atmospheric:.9f};T_c=271.15",
            expected_subcanopy,
            subcanopy,
            "W m^-2",
            1.0e-6,
        )
        add_numeric(
            f"net_c{cover:g}",
            "net_longwave",
            f"C={cover:.6f};T_s=268.15",
            expected_net,
            net,
            "W m^-2",
            1.0e-6,
        )

    def expect_error(
        case_id: str,
        inputs: str,
        expected_type: type[Exception],
        call: Callable[[], object],
    ) -> None:
        try:
            call()
        except expected_type as error:
            add(
                case_id,
                "guard",
                inputs,
                expected_type.__name__,
                type(error).__name__,
                "not applicable",
            )
        except Exception as error:  # pragma: no cover - evidence failure path
            add(
                case_id,
                "guard",
                inputs,
                expected_type.__name__,
                type(error).__name__,
                "not applicable",
                "FAIL",
            )
        else:
            add(
                case_id,
                "guard",
                inputs,
                expected_type.__name__,
                "NO_ERROR",
                "not applicable",
                "FAIL",
            )

    expect_error("invalid_cover_negative", "C=-0.01", ValueError, lambda: sky_view(-0.01))
    expect_error("invalid_cover_closed", "C=1.0", ValueError, lambda: sky_view(1.0))
    expect_error("invalid_cover_nan", "C=NaN", ValueError, lambda: sky_view(math.nan))
    expect_error(
        "invalid_air_temperature_zero",
        "hour_T_a=0",
        ValueError,
        lambda: atmospheric_longwave(0.0, 0.611, 0.475),
    )
    expect_error(
        "invalid_air_temperature_nan",
        "hour_T_a=NaN",
        ValueError,
        lambda: atmospheric_longwave(math.nan, 0.611, 0.475),
    )
    expect_error(
        "invalid_vapor_pressure",
        "daily_e_a=-0.1",
        ValueError,
        lambda: atmospheric_longwave(273.15, -0.1, 0.475),
    )
    expect_error(
        "atmospheric_out_of_authority",
        "hour_T_a=100;daily_e_a=0",
        AtmosphericStateOutOfAuthority,
        lambda: atmospheric_longwave(100.0, 0.0, 0.475),
    )
    expect_error(
        "invalid_solar_radiation",
        "R_s=-0.1;R_a=10",
        ValueError,
        lambda: infer_cloud_fraction(
            -0.1,
            10.0,
            daylight=True,
            extraterrestrial_min_mj_m2_day=(
                ASSUMED_FOR_EXECUTION_EXTRATERRESTRIAL_MIN_MJ_M2_DAY
            ),
        ),
    )
    expect_error(
        "invalid_extraterrestrial_radiation",
        "R_s=0;R_a=-0.1",
        ValueError,
        lambda: infer_cloud_fraction(
            0.0,
            -0.1,
            daylight=True,
            extraterrestrial_min_mj_m2_day=(
                ASSUMED_FOR_EXECUTION_EXTRATERRESTRIAL_MIN_MJ_M2_DAY
            ),
        ),
    )
    expect_error(
        "invalid_clearness_nan",
        "k_t=NaN",
        ValueError,
        lambda: cloud_fraction(math.nan),
    )
    expect_error(
        "polar_night_cloud",
        "daylight=false;R_a=0;R_a_min=1e-9_ASSUMED_FOR_EXECUTION",
        CloudForcingUnavailable,
        lambda: infer_cloud_fraction(
            0.0,
            0.0,
            daylight=False,
            extraterrestrial_min_mj_m2_day=(
                ASSUMED_FOR_EXECUTION_EXTRATERRESTRIAL_MIN_MJ_M2_DAY
            ),
        ),
    )
    expect_error(
        "invalid_canopy_temperature",
        "T_c=0",
        ValueError,
        lambda: longwave_components(0.5, 270.0, 0.0, 268.15),
    )
    expect_error(
        "invalid_snow_temperature",
        "T_s=0",
        ValueError,
        lambda: longwave_components(0.5, 270.0, 271.15, 0.0),
    )
    expect_error(
        "invalid_atmospheric_flux",
        "L_atm=NaN",
        ValueError,
        lambda: longwave_components(0.5, math.nan, 271.15, 268.15),
    )
    add(
        "missing_thermal_provider",
        "governance",
        "T_s unavailable",
        "no production L_net",
        "RUNTIME_HOLD",
        "not applicable",
        "HOLD",
    )

    path = ARTIFACTS / "analytical-test-vectors.csv"
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=list(rows[0]),
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)
    return len(rows)


def svg_header(title: str, description: str) -> list[str]:
    return [
        '<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="620" viewBox="0 0 1000 620" role="img" aria-labelledby="title desc">',
        f"<title id=\"title\">{title}</title>",
        f"<desc id=\"desc\">{description}</desc>",
        '<rect width="1000" height="620" fill="#ffffff"/>',
        '<style>text{font-family:Arial,sans-serif;fill:#17202a}.axis{stroke:#17202a;stroke-width:2}.grid{stroke:#d5d8dc;stroke-width:1}.derived{fill:none;stroke:#146b8c;stroke-width:5}.reference{fill:none;stroke:#b5502c;stroke-width:4;stroke-dasharray:10 8}.cold{fill:none;stroke:#146b8c;stroke-width:5}.equal{fill:none;stroke:#6950a1;stroke-width:5}.warm{fill:none;stroke:#b5502c;stroke-width:5}</style>',
    ]


def polyline(points: list[tuple[float, float]], cls: str) -> str:
    values = " ".join(f"{x:.2f},{y:.2f}" for x, y in points)
    return f'<polyline class="{cls}" points="{values}"/>'


def write_sky_view_figure() -> None:
    x0, y0, width, height = 100.0, 500.0, 800.0, 380.0
    lines = svg_header(
        "Diffuse sky view derived from effective canopy cover",
        "The canonical Beer-law-derived sky-view curve decreases faster than the rejected direct plan-view gap alias.",
    )
    lines += [
        '<text x="500" y="45" text-anchor="middle" font-size="26" font-weight="bold">Canopy cover translated to diffuse sky view</text>',
        '<text x="500" y="78" text-anchor="middle" font-size="17">Analytical model-state translation; no fitted site coefficient</text>',
    ]
    for tick in range(6):
        frac = tick / 5
        x = x0 + frac * width
        y = y0 - frac * height
        lines += [
            f'<line class="grid" x1="{x}" y1="{y0}" x2="{x}" y2="{y0-height}"/>',
            f'<line class="grid" x1="{x0}" y1="{y}" x2="{x0+width}" y2="{y}"/>',
            f'<text x="{x}" y="{y0+28}" text-anchor="middle" font-size="15">{frac:.1f}</text>',
            f'<text x="{x0-18}" y="{y+5}" text-anchor="end" font-size="15">{frac:.1f}</text>',
        ]
    derived = []
    reference = []
    for index in range(100):
        cover = index / 100
        x = x0 + cover * width
        derived.append((x, y0 - sky_view(cover) * height))
        reference.append((x, y0 - (1.0 - cover) * height))
    lines += [
        f'<line class="axis" x1="{x0}" y1="{y0}" x2="{x0+width}" y2="{y0}"/>',
        f'<line class="axis" x1="{x0}" y1="{y0}" x2="{x0}" y2="{y0-height}"/>',
        polyline(derived, "derived"),
        polyline(reference, "reference"),
        '<text x="500" y="570" text-anchor="middle" font-size="18">Effective overhead canopy cover, C (fraction)</text>',
        '<text x="28" y="310" text-anchor="middle" font-size="18" transform="rotate(-90 28 310)">Diffuse sky-view factor (fraction)</text>',
        '<line class="derived" x1="610" y1="145" x2="660" y2="145"/><text x="672" y="151" font-size="16">Canonical: (1-C)^1.6</text>',
        '<line class="reference" x1="610" y1="177" x2="660" y2="177"/><text x="672" y="183" font-size="16">Rejected direct alias: 1-C</text>',
        "</svg>",
    ]
    (FIGURES / "eb02-sky-view-response.svg").write_text("\n".join(lines), encoding="utf-8")


def write_longwave_figure() -> None:
    x0, y0, width, height = 100.0, 500.0, 800.0, 330.0
    atmospheric = 270.0
    snow_temperature = 268.15
    canopy_temperatures = (263.15, 268.15, 273.15)
    styles = ("cold", "equal", "warm")
    labels = ("Canopy 5 C colder", "Canopy equals snow", "Canopy 5 C warmer")
    lines = svg_header(
        "Net longwave response to canopy cover and canopy temperature",
        "Analytical curves show increasing canopy control over snow net longwave as cover rises; atmospheric longwave is fixed.",
    )
    lines += [
        '<text x="500" y="45" text-anchor="middle" font-size="26" font-weight="bold">Canopy cover shifts the snow longwave balance</text>',
        '<text x="500" y="76" text-anchor="middle" font-size="16">Analytical illustration: atmospheric longwave = 270 W m⁻²; snow surface = -5 °C</text>',
    ]
    ymin, ymax = -45.0, 35.0
    for tick in range(6):
        frac = tick / 5
        x = x0 + frac * width
        lines += [
            f'<line class="grid" x1="{x}" y1="{y0}" x2="{x}" y2="{y0-height}"/>',
            f'<text x="{x}" y="{y0+28}" text-anchor="middle" font-size="15">{frac:.1f}</text>',
        ]
    for value in (-40, -20, 0, 20):
        y = y0 - (value - ymin) / (ymax - ymin) * height
        lines += [
            f'<line class="grid" x1="{x0}" y1="{y}" x2="{x0+width}" y2="{y}"/>',
            f'<text x="{x0-18}" y="{y+5}" text-anchor="end" font-size="15">{value}</text>',
        ]
    for canopy_temperature, style in zip(canopy_temperatures, styles):
        points = []
        for index in range(100):
            cover = index / 100
            _, _, _, net = longwave_components(
                cover, atmospheric, canopy_temperature, snow_temperature
            )
            points.append(
                (
                    x0 + cover * width,
                    y0 - (net - ymin) / (ymax - ymin) * height,
                )
            )
        lines.append(polyline(points, style))
    zero_y = y0 - (0.0 - ymin) / (ymax - ymin) * height
    lines += [
        f'<line x1="{x0}" y1="{zero_y}" x2="{x0+width}" y2="{zero_y}" stroke="#17202a" stroke-width="2" stroke-dasharray="3 5"/>',
        f'<line class="axis" x1="{x0}" y1="{y0}" x2="{x0+width}" y2="{y0}"/>',
        f'<line class="axis" x1="{x0}" y1="{y0}" x2="{x0}" y2="{y0-height}"/>',
        '<text x="500" y="570" text-anchor="middle" font-size="18">Effective overhead canopy cover, C (fraction)</text>',
        '<text x="28" y="335" text-anchor="middle" font-size="18" transform="rotate(-90 28 335)">Net longwave toward snow (W m^-2)</text>',
    ]
    for index, (style, label) in enumerate(zip(styles, labels)):
        y = 105 + 25 * index
        lines += [
            f'<line class="{style}" x1="625" y1="{y}" x2="670" y2="{y}"/>',
            f'<text x="682" y="{y+6}" font-size="15">{label}</text>',
        ]
    lines.append("</svg>")
    (FIGURES / "eb02-longwave-components.svg").write_text("\n".join(lines), encoding="utf-8")


def validate() -> None:
    with (ARTIFACTS / "analytical-test-vectors.csv").open(
        newline="", encoding="utf-8"
    ) as handle:
        rows = list(csv.DictReader(handle))
    required_cases = {
        "sky_view_c0",
        "sky_view_c0.5",
        "cloud_kt0",
        "cloud_kt0.8",
        "water_reference",
        "clear_reference",
        "all_sky_reference",
        "all_sky_cold_hour",
        "all_sky_warm_hour",
        "daily_mean_temperature_substitution_bias",
        "subcanopy_c0",
        "net_c0.9",
        "invalid_cover_negative",
        "invalid_cover_nan",
        "invalid_air_temperature_zero",
        "invalid_air_temperature_nan",
        "invalid_vapor_pressure",
        "atmospheric_out_of_authority",
        "invalid_solar_radiation",
        "invalid_extraterrestrial_radiation",
        "invalid_clearness_nan",
        "polar_night_cloud",
        "invalid_canopy_temperature",
        "invalid_snow_temperature",
        "invalid_atmospheric_flux",
        "missing_thermal_provider",
    }
    case_ids = [row["case_id"] for row in rows]
    if len(case_ids) != len(set(case_ids)):
        raise RuntimeError("duplicate analytical case ID")
    if not required_cases.issubset(case_ids):
        missing = sorted(required_cases.difference(case_ids))
        raise RuntimeError(f"missing required analytical cases: {missing}")
    allowed_status = {"PASS", "HOLD"}
    if not rows or any(row["status"] not in allowed_status for row in rows):
        raise RuntimeError("analytical vector failure")
    hold_cases = {row["case_id"] for row in rows if row["status"] == "HOLD"}
    if hold_cases != {"missing_thermal_provider"}:
        raise RuntimeError(f"unexpected analytical HOLD cases: {sorted(hold_cases)}")
    sky_rows = [row for row in rows if row["category"] == "sky_view"]
    sky_values = [float(row["observed"]) for row in sky_rows]
    if any(left <= right for left, right in zip(sky_values, sky_values[1:])):
        raise RuntimeError("sky-view response is not strictly decreasing")
    for path in FIGURES.glob("*.svg"):
        text = path.read_text(encoding="utf-8")
        for marker in ('role="img"', "<title", "<desc"):
            if marker not in text:
                raise RuntimeError(f"{path}: missing {marker}")


def main() -> None:
    FIGURES.mkdir(parents=True, exist_ok=True)
    vector_count = write_vectors()
    write_sky_view_figure()
    write_longwave_figure()
    validate()
    print(
        f"PASS: regenerated {vector_count} analytical vectors "
        "and 2 accessible SVG figures"
    )


if __name__ == "__main__":
    main()
