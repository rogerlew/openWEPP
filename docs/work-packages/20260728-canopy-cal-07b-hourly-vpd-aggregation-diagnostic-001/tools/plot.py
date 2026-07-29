#!/usr/bin/env python3
"""Render accessible deterministic CAL-07B SVG diagnostics."""

from __future__ import annotations

import csv
import hashlib
import html
from pathlib import Path

PKG = Path(__file__).resolve().parents[1]
ART = PKG / "artifacts"
FIG = ART / "figures"
WIDTH, HEIGHT = 1240, 820
BLUE, GREEN, ORANGE, RED, GRAY = "#2864A5", "#16823A", "#C25B12", "#B42318", "#5B6770"


def rows(name: str) -> list[dict[str, str]]:
    with (ART / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


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
        f"{name}:{hashlib.sha256((ART / name).read_bytes()).hexdigest()}"
        for name in bindings
    )
    parts.append(f'<metadata id="source-bindings">{html.escape(metadata)}</metadata>')
    parts.append("</svg>")
    (FIG / filename).write_text("\n".join(parts) + "\n", encoding="utf-8")


def path(points: list[tuple[float, float]]) -> str:
    return " ".join(
        ("M" if index == 0 else "L") + f"{x:.2f},{y:.2f}"
        for index, (x, y) in enumerate(points)
    )


def hourly_operands() -> None:
    data = rows("hourly-reconstruction.csv")
    dates = sorted({row["date"] for row in data})
    parts = start(
        "Published hourly-average POWER operands remain VPD-positive",
        "Three date columns show hourly-average temperature and dew point above paired reconstructed VPD. All 72 hourly-product VPD values remain above zero.",
    )
    for index, day in enumerate(dates):
        selected = [row for row in data if row["date"] == day]
        x, w = 75 + index * 390, 350
        parts.append(f'<text x="{x+w/2}" y="78" text-anchor="middle" class="label">{day} (LST)</text>')
        y, h = 95, 270
        parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" class="panel"/>')
        temps = [float(row[field]) for row in selected for field in ("t2m_c", "t2mdew_c")]
        low, high = min(temps) - 0.5, max(temps) + 0.5
        px = lambda hour: x + int(hour) / 23 * w
        py_t = lambda value: y + h - (value - low) / (high - low) * h
        for fraction in (0.0, 0.5, 1.0):
            value = low + fraction * (high - low)
            yy = py_t(value)
            parts.append(f'<line x1="{x}" y1="{yy}" x2="{x+w}" y2="{yy}" class="grid"/>')
            parts.append(f'<text x="{x-7}" y="{yy+4}" text-anchor="end" class="small">{value:.1f}</text>')
        for field, color, dash in (("t2m_c", BLUE, ""), ("t2mdew_c", GREEN, "5 3")):
            points = [(px(row["hour_lst"]), py_t(float(row[field]))) for row in selected]
            parts.append(
                f'<path d="{path(points)}" fill="none" stroke="{color}" stroke-width="2"'
                + (f' stroke-dasharray="{dash}"' if dash else "")
                + "/>"
            )
        y2, h2 = 430, 265
        parts.append(f'<rect x="{x}" y="{y2}" width="{w}" height="{h2}" class="panel"/>')
        values = [float(row["hourly_product_vpd_pa"]) for row in selected]
        vmax = max(values) * 1.08
        py_v = lambda value: y2 + h2 - value / vmax * h2
        parts.append(f'<line x1="{x}" y1="{py_v(0)}" x2="{x+w}" y2="{py_v(0)}" stroke="{RED}" stroke-width="1.5"/>')
        for fraction in (0.0, 0.5, 1.0):
            value = fraction * vmax
            yy = py_v(value)
            parts.append(f'<line x1="{x}" y1="{yy}" x2="{x+w}" y2="{yy}" class="grid"/>')
            parts.append(f'<text x="{x-7}" y="{yy+4}" text-anchor="end" class="small">{value:.0f}</text>')
        points = [(px(row["hour_lst"]), py_v(float(row["hourly_product_vpd_pa"]))) for row in selected]
        parts.append(f'<path d="{path(points)}" fill="none" stroke="{ORANGE}" stroke-width="2.2"/>')
        for hour in (0, 6, 12, 18, 23):
            parts.append(f'<text x="{px(str(hour))}" y="{y2+h2+20}" text-anchor="middle" class="small">{hour:02d}</text>')
        parts.append(f'<text x="{x+w-5}" y="{py_v(0)-6}" text-anchor="end" class="small" fill="{RED}">0 Pa</text>')
    parts.extend(
        [
            f'<line x1="420" y1="770" x2="460" y2="770" stroke="{BLUE}" stroke-width="2"/>',
            '<text x="468" y="775" class="small">T2M hourly average</text>',
            f'<line x1="650" y1="770" x2="690" y2="770" stroke="{GREEN}" stroke-width="2" stroke-dasharray="5 3"/>',
            '<text x="698" y="775" class="small">T2MDEW hourly average</text>',
            f'<line x1="900" y1="770" x2="940" y2="770" stroke="{ORANGE}" stroke-width="2"/>',
            '<text x="948" y="775" class="small">reconstructed hourly-product VPD</text>',
        ]
    )
    finish(parts, "cal07b-hourly-operands-and-vpd.svg", ("hourly-reconstruction.csv",))


def decomposition() -> None:
    data = rows("daily-decomposition.csv")
    parts = start(
        "Temperature-extrema summarization drives the negative daily VPD",
        "Grouped bars decompose contract daily VPD minus mean hourly-product VPD into a large negative temperature-extrema term and a smaller positive dew-point nonlinearity term.",
    )
    x0, y0, w, h = 105, 100, 1070, 575
    parts.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" class="panel"/>')
    ymin, ymax = -130.0, 80.0
    py = lambda value: y0 + h - (value - ymin) / (ymax - ymin) * h
    for value in (-120, -80, -40, 0, 40, 80):
        yy = py(value)
        parts.append(f'<line x1="{x0}" y1="{yy}" x2="{x0+w}" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="{x0-10}" y="{yy+4}" text-anchor="end" class="small">{value:+d}</text>')
    parts.append(f'<line x1="{x0}" y1="{py(0)}" x2="{x0+w}" y2="{py(0)}" stroke="{GRAY}" stroke-width="1.5"/>')
    series = (
        ("mean_hourly_product_vpd_pa", "hourly mean", BLUE),
        ("temperature_extrema_summary_term_pa", "temperature term", ORANGE),
        ("dewpoint_nonlinearity_term_pa", "dew-point term", GREEN),
        ("reconstructed_contract_vpd_pa", "contract daily", RED),
    )
    for case_index, row in enumerate(data):
        center = x0 + (case_index + 0.5) * w / len(data)
        for series_index, (field, _, color) in enumerate(series):
            value = float(row[field])
            bx = center + (series_index - 1.5) * 42 - 15
            top, bottom = py(max(value, 0.0)), py(min(value, 0.0))
            parts.append(f'<rect x="{bx}" y="{top}" width="30" height="{bottom-top}" fill="{color}"/>')
            label_y = top - 7 if value >= 0 else bottom + 15
            parts.append(f'<text x="{bx+15}" y="{label_y}" text-anchor="middle" class="small">{value:+.1f}</text>')
        parts.append(f'<text x="{center}" y="{y0+h+27}" text-anchor="middle" class="label">{row["date"]}</text>')
    for index, (_, label, color) in enumerate(series):
        xx = 275 + index * 205
        parts.append(f'<rect x="{xx}" y="750" width="14" height="14" fill="{color}"/>')
        parts.append(f'<text x="{xx+21}" y="762" class="small">{label}</text>')
    parts.append('<text x="28" y="405" transform="rotate(-90 28 405)" class="label">VPD or additive term (Pa)</text>')
    finish(parts, "cal07b-additive-driver-decomposition.svg", ("daily-decomposition.csv",))


def source_reconstruction() -> None:
    data = rows("daily-decomposition.csv")
    parts = start(
        "Hourly products reconstruct the frozen CAL-07 daily operands",
        "Temperature panels show reconstructed minus reported daily minimum, maximum, and mean dew point; the lower panel shows the resulting contract-VPD residual.",
    )
    x0, w = 120, 1040
    y, h = 105, 390
    parts.append(f'<rect x="{x0}" y="{y}" width="{w}" height="{h}" class="panel"/>')
    low, high = -0.006, 0.006
    py = lambda value: y + h - (value - low) / (high - low) * h
    for value in (-0.005, 0.0, 0.005):
        yy = py(value)
        parts.append(f'<line x1="{x0}" y1="{yy}" x2="{x0+w}" y2="{yy}" class="grid"/>')
        parts.append(f'<text x="{x0-10}" y="{yy+4}" text-anchor="end" class="small">{value:+.3f}</text>')
    fields = (("tmin_residual_c", BLUE), ("tmax_residual_c", ORANGE), ("tdew_residual_c", GREEN))
    for index, row in enumerate(data):
        center = x0 + (index + 0.5) * w / len(data)
        for offset, (field, color) in zip((-32, 0, 32), fields):
            value = float(row[field])
            yzero, yvalue = py(0.0), py(value)
            parts.append(f'<rect x="{center+offset-10}" y="{min(yzero,yvalue)}" width="20" height="{abs(yvalue-yzero)+0.8}" fill="{color}"/>')
        parts.append(f'<text x="{center}" y="{y+h+24}" text-anchor="middle" class="small">{row["date"]}</text>')
    y2, h2 = 570, 125
    parts.append(f'<rect x="{x0}" y="{y2}" width="{w}" height="{h2}" class="panel"/>')
    vmax = 0.4
    py2 = lambda value: y2 + h2 - value / vmax * h2
    parts.append(f'<line x1="{x0}" y1="{py2(0)}" x2="{x0+w}" y2="{py2(0)}" class="grid"/>')
    for index, row in enumerate(data):
        center = x0 + (index + 0.5) * w / len(data)
        value = float(row["contract_vpd_residual_pa"])
        parts.append(f'<rect x="{center-18}" y="{py2(value)}" width="36" height="{py2(0)-py2(value)}" fill="{RED}"/>')
        parts.append(f'<text x="{center}" y="{py2(value)-7}" text-anchor="middle" class="small">{value:+.3f} Pa</text>')
    parts.append('<text x="32" y="305" transform="rotate(-90 32 305)" class="label">reconstructed − reported (°C)</text>')
    parts.append('<text x="56" y="690" transform="rotate(-90 56 690)" class="small">VPD residual</text>')
    for index, (label, color) in enumerate((("Tmin", BLUE), ("Tmax", ORANGE), ("mean Tdew", GREEN))):
        xx = 430 + index * 145
        parts.append(f'<rect x="{xx}" y="760" width="14" height="14" fill="{color}"/>')
        parts.append(f'<text x="{xx+21}" y="772" class="small">{label}</text>')
    finish(parts, "cal07b-source-reconstruction.svg", ("daily-decomposition.csv",))


def manifest() -> None:
    paths = [
        ART / "hourly-reconstruction.csv",
        ART / "daily-decomposition.csv",
        ART / "attribution.csv",
        *sorted(FIG.glob("*.svg")),
        *sorted(FIG.glob("*.md")),
    ]
    with (ART / "result-manifest.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=("path", "sha256", "bytes"))
        writer.writeheader()
        for source in paths:
            writer.writerow(
                {
                    "path": source.relative_to(PKG),
                    "sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
                    "bytes": source.stat().st_size,
                }
            )


def main() -> None:
    FIG.mkdir(exist_ok=True)
    hourly_operands()
    decomposition()
    source_reconstruction()
    if len(list(FIG.glob("*.md"))) == 3:
        manifest()


if __name__ == "__main__":
    main()
