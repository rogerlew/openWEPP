#!/usr/bin/env python3
"""Render deterministic accessible SVG figures and sidecars for CAL-07C."""

from __future__ import annotations

import csv
import hashlib
import html
import math
import statistics
from collections import defaultdict
from datetime import date
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
FIG = ART / "figures"
WIDTH, HEIGHT = 1240, 780
GREEN, BLUE, ORANGE, RED, PURPLE, GRAY = (
    "#16823A",
    "#2864A5",
    "#C25B12",
    "#B42318",
    "#7651A3",
    "#5B6770",
)


def rows(name: str) -> list[dict[str, str]]:
    with (ART / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def start(title: str, desc: str) -> list[str]:
    return [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" '
        f'viewBox="0 0 {WIDTH} {HEIGHT}" role="img" aria-labelledby="title desc">',
        f'<title id="title">{html.escape(title)}</title>',
        f'<desc id="desc">{html.escape(desc)}</desc>',
        "<style>text{font-family:Arial,sans-serif;fill:#182026}"
        ".title{font-size:24px;font-weight:700}.label{font-size:13px}"
        ".small{font-size:11px}.grid{stroke:#d9e2ec;stroke-width:1}"
        ".panel{fill:#fff;stroke:#bcccdc}</style>",
        '<rect width="100%" height="100%" fill="#fff"/>',
        f'<text x="48" y="42" class="title">{html.escape(title)}</text>',
    ]


def finish(parts: list[str], filename: str, bindings: tuple[str, ...]) -> None:
    metadata = ";".join(
        f"{name}:{sha(ART / name)}" for name in bindings if (ART / name).exists()
    )
    parts.append(f'<metadata id="source-bindings">{html.escape(metadata)}</metadata>')
    parts.append("</svg>")
    (FIG / filename).write_text("\n".join(parts) + "\n", encoding="utf-8")


def polyline(points: list[tuple[float, float]]) -> str:
    return " ".join(
        ("M" if index == 0 else "L") + f"{x:.2f},{y:.2f}"
        for index, (x, y) in enumerate(points)
    )


def write_sidecar(path: Path, title: str, body: str) -> None:
    path.write_text(f"# {title}\n\n{body.strip()}\n", encoding="utf-8")


def source_admission_binding() -> str:
    return (
        "`artifacts/source-manifest.csv` SHA-256 "
        f"`{sha(ART / 'source-manifest.csv')}`, "
        "`artifacts/admission-table.csv` SHA-256 "
        f"`{sha(ART / 'admission-table.csv')}`, and "
        "`artifacts/forcing-source-summary.csv` SHA-256 "
        f"`{sha(ART / 'forcing-source-summary.csv')}`"
    )


def vpd_audit() -> None:
    daily = rows("daily-vpd-reconstruction.csv")
    begin = date.fromisoformat(daily[0]["date"]).toordinal()
    end = date.fromisoformat(daily[-1]["date"]).toordinal()
    x0, y0, w, h = 95, 90, 1060, 420
    values = [float(row["admitted_hourly_mean_vpd_pa"]) for row in daily] + [
        float(row["daily_contract_vpd_pa"]) for row in daily
    ]
    ymin, ymax = min(values) - 10.0, max(values) * 1.03
    px = lambda day: x0 + (date.fromisoformat(day).toordinal() - begin) / (end - begin) * w
    py = lambda value: y0 + h - (value - ymin) / (ymax - ymin) * h
    parts = start(
        "CAL-07C admits Alerce daily VPD from hourly paired products",
        "The daily contract VPD series is compared with the admitted hourly paired-product daily mean; negative contract days and negative hourly-component days are visible.",
    )
    parts.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" class="panel"/>')
    for tick in (0, 500, 1000, 1500, 2000):
        yy = py(tick)
        if yy < y0 or yy > y0 + h:
            continue
        parts.append(f'<line x1="{x0}" y1="{yy}" x2="{x0+w}" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="{x0-8}" y="{yy+4}" text-anchor="end" class="small">{tick}</text>')
    zero = py(0)
    parts.append(f'<line x1="{x0}" y1="{zero}" x2="{x0+w}" y2="{zero}" stroke="{RED}" stroke-width="1.5"/>')
    admitted = [(px(row["date"]), py(float(row["admitted_hourly_mean_vpd_pa"]))) for row in daily]
    contract = [(px(row["date"]), py(float(row["daily_contract_vpd_pa"]))) for row in daily]
    parts.append(f'<path d="{polyline(contract)}" fill="none" stroke="{GRAY}" stroke-width="1.3" stroke-dasharray="5 3"/>')
    parts.append(f'<path d="{polyline(admitted)}" fill="none" stroke="{BLUE}" stroke-width="2"/>')
    for row in daily:
        if float(row["daily_contract_vpd_pa"]) < 0.0:
            parts.append(f'<circle cx="{px(row["date"]):.2f}" cy="{py(float(row["daily_contract_vpd_pa"])):.2f}" r="4.5" fill="{RED}"/>')
    y2, h2 = 575, 110
    parts.append(f'<rect x="{x0}" y="{y2}" width="{w}" height="{h2}" class="panel"/>')
    max_count = max(int(row["negative_hourly_count"]) for row in daily)
    for row in daily:
        count = int(row["negative_hourly_count"])
        if count:
            bx = px(row["date"])
            bh = count / max_count * (h2 - 20)
            parts.append(f'<rect x="{bx:.2f}" y="{y2+h2-bh:.2f}" width="1.4" height="{bh:.2f}" fill="{ORANGE}"/>')
    for year in range(2022, 2027):
        xpos = px(f"{year}-07-01" if year < 2026 else "2026-04-01")
        parts.append(f'<text x="{xpos:.2f}" y="{y0+h+26}" text-anchor="middle" class="small">{year}</text>')
    parts.append(f'<text x="35" y="325" transform="rotate(-90 35 325)" class="label">daily VPD (Pa)</text>')
    parts.append(f'<text x="46" y="675" transform="rotate(-90 46 675)" class="small">negative hourly rows/day</text>')
    parts.extend(
        [
            f'<line x1="330" y1="735" x2="370" y2="735" stroke="{BLUE}" stroke-width="2"/>',
            '<text x="378" y="739" class="small">admitted daily mean of hourly products</text>',
            f'<line x1="690" y1="735" x2="730" y2="735" stroke="{GRAY}" stroke-width="2" stroke-dasharray="5 3"/>',
            '<text x="738" y="739" class="small">original daily contract VPD</text>',
            f'<rect x="960" y="726" width="16" height="16" fill="{ORANGE}"/>',
            '<text x="984" y="739" class="small">negative hourly components retained</text>',
        ]
    )
    finish(parts, "cal07c-vpd-reconstruction-audit.svg", ("daily-vpd-reconstruction.csv",))
    negative_days = sum(1 for row in daily if int(row["negative_hourly_count"]))
    write_sidecar(
        FIG / "cal07c-vpd-reconstruction-audit.md",
        "Alerce VPD Reconstruction Audit",
        f"""
## Caption

Original CAL-07 daily contract VPD and CAL-07C admitted hourly-product daily
mean VPD for Alerce Costero. Red points are the three original negative daily
contract values. The lower panel shows days with negative hourly paired-product
components; {negative_days} days contain at least one such hour, but every
admitted daily mean is nonnegative.

## How to read it

The blue line is the VPD operand passed to the CAL-07C package-local GSI
executor. The dashed gray line is the original OBL-PLANT-P-013 daily-summary
reconstruction from CAL-07. Orange bars are counts of retained negative hourly
components inside the daily signed mean.

## Plain-language takeaway

The three CAL-07 daily failures disappear at the daily operand level when VPD
is reconstructed as the mean of paired hourly POWER products. The hourly source
is still not physically clean at every hour, so the result is bounded research
forcing evidence, not production authority.

## Methods and source binding

The figure binds `artifacts/daily-vpd-reconstruction.csv`, SHA-256
`{sha(ART / 'daily-vpd-reconstruction.csv')}`, plus
{source_admission_binding()}. VPD uses `1000*(es(T2M)-es(T2MDEW))` at each
hour and an arithmetic daily mean over exact 24-hour LST days. Units are Pa.

## Limitations

No hourly negative value is clipped, deleted, or hidden; 349 negative hourly
components remain a claim ceiling. The retained POWER grid elevation remains
99.4 m while the camera site is approximately 840 m. This package does not
replace OBL-PLANT-P-013 in production.

## Accessibility

The lines, points, and bars use color plus distinct geometry. Zero is a
visible horizontal reference, axes carry units, and the SVG includes title,
description, and source metadata.
""",
    )


def observed_and_modeled() -> None:
    daily = rows("ensemble-daily.csv")
    parts = start(
        "CAL-07C observed greenness and frozen-ensemble GSI",
        "Two time-series panels compare annually normalized camera GCC90 with the 37-member GSI median and 5th-to-95th percentile band during 2024 and 2025.",
    )
    begin = date(2024, 1, 1).toordinal()
    end = date(2025, 12, 31).toordinal()
    for index, (site, label) in enumerate(
        (("SH-DB-BEZA", "Beza Mahafaly - deciduous dry forest"), ("SH-EN-ALERCE", "Alerce Costero - evergreen forest"))
    ):
        source_label = (
            "VPD: CAL-07 daily summary unchanged"
            if site == "SH-DB-BEZA"
            else "VPD: POWER hourly-product daily mean"
        )
        x, y, w, h = 85, 90 + index * 325, 1090, 260
        parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" class="panel"/>')
        for fraction in (0.0, 0.25, 0.5, 0.75, 1.0):
            yy = y + h - fraction * h
            parts.append(f'<line x1="{x}" y1="{yy}" x2="{x+w}" y2="{yy}" class="grid"/>')
            parts.append(f'<text x="{x-9}" y="{yy+4}" text-anchor="end" class="small">{fraction:.2g}</text>')
        selected = [row for row in daily if row["site_id"] == site and row["year"] in {"2024", "2025"}]
        px = lambda day: x + (date.fromisoformat(day).toordinal() - begin) / (end - begin) * w
        py = lambda value: y + h - value * h
        upper = [(px(row["date"]), py(float(row["gsi_p95"]))) for row in selected]
        lower = [(px(row["date"]), py(float(row["gsi_p05"]))) for row in reversed(selected)]
        polygon = " ".join(f"{a:.2f},{b:.2f}" for a, b in upper + lower)
        parts.append(f'<polygon points="{polygon}" fill="#8fbada" opacity=".35"/>')
        median = [(px(row["date"]), py(float(row["gsi_median"]))) for row in selected]
        parts.append(f'<path d="{polyline(median)}" fill="none" stroke="{BLUE}" stroke-width="2.4"/>')
        by_year: dict[str, list[dict[str, str]]] = defaultdict(list)
        for row in selected:
            if row["observed_gcc90"]:
                by_year[row["year"]].append(row)
        observed_points = []
        for annual in by_year.values():
            values = [float(row["observed_gcc90"]) for row in annual]
            low, high = min(values), max(values)
            if high > low:
                observed_points.extend(
                    (px(row["date"]), py((float(row["observed_gcc90"]) - low) / (high - low)))
                    for row in annual
                )
        parts.append(f'<path d="{polyline(sorted(observed_points))}" fill="none" stroke="{GREEN}" stroke-width="1.5" stroke-dasharray="5 3"/>')
        parts.append(f'<rect x="{x+4}" y="{y+7}" width="310" height="38" fill="#fff" opacity=".86"/>')
        parts.append(f'<text x="{x+8}" y="{y+20}" class="label">{label}</text>')
        parts.append(f'<text x="{x+8}" y="{y+38}" class="small">{source_label}</text>')
        for year in (2024, 2025):
            parts.append(f'<text x="{px(f"{year}-07-01")}" y="{y+h+24}" text-anchor="middle" class="small">{year}</text>')
    parts.extend(
        [
            f'<line x1="355" y1="744" x2="395" y2="744" stroke="{BLUE}" stroke-width="3"/>',
            '<text x="403" y="749" class="small">GSI ensemble median and 5th-95th band</text>',
            f'<line x1="720" y1="744" x2="760" y2="744" stroke="{GREEN}" stroke-width="2" stroke-dasharray="5 3"/>',
            '<text x="768" y="749" class="small">annually normalized raw GCC90</text>',
        ]
    )
    finish(parts, "cal07c-observed-and-modeled-seasons.svg", ("ensemble-daily.csv",))
    write_sidecar(
        FIG / "cal07c-observed-and-modeled-seasons.md",
        "Observed Greenness And CAL-07C GSI",
        f"""
## Caption

Two Southern Hemisphere PhenoCam lanes compared with the frozen 37-member GSI
ensemble after the Alerce VPD forcing blocker is lifted for bounded execution.

## How to read it

The blue line is the ensemble median and the blue band spans the 5th to 95th
percentiles. The green dashed line is annually normalized raw GCC90 on admitted
camera days.

## Plain-language takeaway

CAL-07C restores the ability to look at Southern Hemisphere timing and shape,
but the camera greenness remains a relative proxy and not absolute LAI,
biomass, or canopy cover.

## Methods and source binding

The figure binds `artifacts/ensemble-daily.csv`, SHA-256
`{sha(ART / 'ensemble-daily.csv')}`, plus {source_admission_binding()}.
CAL-04B accepted members are retained without refit or ranking. Alerce uses
the daily mean of POWER hourly paired-product VPD over exact 24-hour LST
days; Beza keeps the CAL-07 daily-summary VPD operator.

## Limitations

Both camera products are provisional and share the PhenoCam processing method.
POWER forcing is gridded/reanalysis evidence, not on-site meteorology. No VPD
value is clipped, and CAL-07C does not replace OBL-PLANT-P-013 in production.

## Accessibility

Median, uncertainty band, and observed proxy use separate line styles and
geometry. Panels are separated by site and include units-free normalized
fraction axes.
""",
    )


def score_summary() -> None:
    site_summary = rows("site-summary.csv")
    shape = rows("shape-scores.csv")
    transitions = rows("transition-residuals.csv")
    parts = start(
        "CAL-07C timing and shape score summary",
        "Bars summarize median normalized shape correlation by site and year, with deciduous transition residuals retained below.",
    )
    x0, y0, w, h = 100, 100, 1040, 310
    parts.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" class="panel"/>')
    groups = []
    for site in ("SH-DB-BEZA", "SH-EN-ALERCE"):
        for year in ("2024", "2025"):
            values = [float(row["pearson_r"]) for row in shape if row["site_id"] == site and row["year"] == year]
            groups.append((site, year, statistics.median(values)))
    ymin, ymax = -1.0, 1.0
    py = lambda value: y0 + h - (value - ymin) / (ymax - ymin) * h
    for tick in (-1.0, -0.5, 0.0, 0.5, 1.0):
        yy = py(tick)
        parts.append(f'<line x1="{x0}" y1="{yy}" x2="{x0+w}" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="{x0-8}" y="{yy+4}" text-anchor="end" class="small">{tick:+.1f}</text>')
    for index, (site, year, value) in enumerate(groups):
        cx = x0 + (index + 0.5) * w / len(groups)
        color = GREEN if value > 0 else RED
        top, bottom = py(max(value, 0.0)), py(min(value, 0.0))
        parts.append(f'<rect x="{cx-38}" y="{top}" width="76" height="{bottom-top}" fill="{color}"/>')
        parts.append(f'<text x="{cx}" y="{top-8 if value>=0 else bottom+17}" text-anchor="middle" class="small">{value:+.3f}</text>')
        parts.append(f'<text x="{cx}" y="{y0+h+24}" text-anchor="middle" class="small">{site}</text>')
        parts.append(f'<text x="{cx}" y="{y0+h+40}" text-anchor="middle" class="small">{year}</text>')
    y2, h2 = 505, 150
    parts.append(f'<rect x="{x0}" y="{y2}" width="{w}" height="{h2}" class="panel"/>')
    residuals = [float(row["residual_days"]) for row in transitions if row["residual_days"]]
    low, high = min(min(residuals), 0.0), max(max(residuals), 0.0)
    px = lambda value: x0 + (value - low) / (high - low) * w
    for tick in range(math.floor(low / 30) * 30, math.ceil(high / 30) * 30 + 1, 30):
        xx = px(tick)
        parts.append(f'<line x1="{xx}" y1="{y2}" x2="{xx}" y2="{y2+h2}" class="grid"/>')
        parts.append(f'<text x="{xx}" y="{y2+h2+20}" text-anchor="middle" class="small">{tick:+d}</text>')
    parts.append(f'<line x1="{px(0)}" y1="{y2}" x2="{px(0)}" y2="{y2+h2}" stroke="{GRAY}" stroke-width="1.5"/>')
    for index, value in enumerate(residuals):
        yj = y2 + 15 + (index % 37) / 36 * (h2 - 30)
        parts.append(f'<circle cx="{px(value):.2f}" cy="{yj:.2f}" r="2.5" fill="{PURPLE}" opacity=".55"/>')
    parts.append('<text x="95" y="690" class="small">Beza modeled GSI 0.5 crossing minus provisional GCC transition date (days)</text>')
    finish(parts, "cal07c-score-summary.svg", ("shape-scores.csv", "transition-residuals.csv", "site-summary.csv"))
    summary_lines = "; ".join(
        f"{row['site_id']} median r {row['shape_r_median']}, RMSE {row['shape_rmse_median']}"
        for row in site_summary
    )
    residual_count = len(residuals)
    residual_total = len(transitions)
    write_sidecar(
        FIG / "cal07c-score-summary.md",
        "Timing And Shape Score Summary",
        f"""
## Caption

Median normalized shape correlations by site/year and Beza deciduous midpoint
timing residuals for every frozen ensemble member.

## How to read it

Positive bars indicate same-direction annual normalized shape agreement.
Purple points are modeled minus observed transition dates; zero means exact
agreement with the provisional PhenoCam midpoint. Blank member/event rows do
not appear as points because no same-direction modeled crossing was found.

## Plain-language takeaway

The result is descriptive bounded evidence. It evaluates whether the frozen
ensemble has Southern Hemisphere timing/shape support after the forcing
blocker is removed; it does not refit or choose members. Only
{residual_count} of {residual_total} Beza member/event rows found a
same-direction crossing, so transition chronology remains contradicted.

## Methods and source binding

The figure binds `shape-scores.csv`, `transition-residuals.csv`, and
`site-summary.csv`, plus {source_admission_binding()}. Alerce uses the POWER
hourly-product daily-mean VPD operand; Beza keeps the CAL-07 daily-summary VPD
operand. Summary: {summary_lines}.

## Limitations

No pass threshold is invented for timing residual magnitude. Shape scores use
camera-supported days only and annual min-max normalization. No VPD value is
clipped, and CAL-07C does not replace OBL-PLANT-P-013 in production.

## Accessibility

Bars carry signed numeric labels. Timing residuals are plotted against a
visible zero line with days as the unit.
""",
    )


def evidence_boundaries() -> None:
    data = rows("verdict-matrix.csv")
    parts = start(
        "CAL-07C evidence boundaries",
        "A status plot shows which cells are lifted, bounded, supported, contradicted, or not evaluated.",
    )
    positions = {
        "CONTRADICTED": 430,
        "NOT_EVALUATED": 585,
        "LIFTED_FOR_BOUNDED_EXECUTION": 760,
        "BOUNDED": 960,
        "SUPPORTED": 1110,
    }
    colors = {
        "CONTRADICTED": RED,
        "NOT_EVALUATED": GRAY,
        "LIFTED_FOR_BOUNDED_EXECUTION": BLUE,
        "BOUNDED": ORANGE,
        "SUPPORTED": GREEN,
    }
    for status, xpos in positions.items():
        label = status.replace("_", " ")
        parts.append(f'<text x="{xpos}" y="82" text-anchor="middle" class="small">{html.escape(label)}</text>')
    for index, row in enumerate(data):
        yy = 120 + index * 50
        parts.append(f'<line x1="400" y1="{yy}" x2="1140" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="48" y="{yy+5}" class="label">{html.escape(row["cell"])}</text>')
        parts.append(f'<circle cx="{positions[row["status"]]}" cy="{yy}" r="8.5" fill="{colors[row["status"]]}"/>')
    finish(parts, "cal07c-evidence-boundaries.svg", ("verdict-matrix.csv",))
    write_sidecar(
        FIG / "cal07c-evidence-boundaries.md",
        "CAL-07C Evidence Boundaries",
        f"""
## Caption

Verdict matrix for CAL-07C after package-local Alerce hourly VPD admission and
bounded result execution.

## How to read it

Each row is an evidence cell. Dots show whether the cell is lifted for bounded
execution, bounded, supported, contradicted, or not evaluated.

## Plain-language takeaway

CAL-07C can lift the immediate Alerce VPD input blocker for this research
execution. It still cannot claim production forcing authority, absolute canopy
amplitude, phase-transformed real-consumer chronology, or litter/decomposition
consequences.

## Methods and source binding

The figure binds `artifacts/verdict-matrix.csv`, SHA-256
`{sha(ART / 'verdict-matrix.csv')}`, plus {source_admission_binding()}. The
Alerce source operator is the POWER hourly-product daily mean over exact
24-hour LST days; Beza remains CAL-07 daily-summary VPD.

## Limitations

`LIFTED_FOR_BOUNDED_EXECUTION` is package-local. It is not an amendment to
OBL-PLANT-P-013 and does not change production behavior. Negative hourly
components were retained signed; none were clipped.

## Accessibility

The matrix uses row labels plus explicit status columns. Color is supplemental
to position and text.
""",
    )


def manifest() -> None:
    paths = [
        *sorted(ART.glob("*.csv")),
        *sorted(FIG.glob("*.svg")),
        *sorted(FIG.glob("*.md")),
    ]
    paths = [path for path in paths if path.name != "result-manifest.csv"]
    with (ART / "result-manifest.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=("path", "sha256", "bytes"),
            lineterminator="\n",
        )
        writer.writeheader()
        for path in paths:
            writer.writerow(
                {
                    "path": path.relative_to(PKG),
                    "sha256": sha(path),
                    "bytes": path.stat().st_size,
                }
            )


def main() -> None:
    FIG.mkdir(exist_ok=True)
    vpd_audit()
    observed_and_modeled()
    score_summary()
    evidence_boundaries()
    manifest()


if __name__ == "__main__":
    main()
