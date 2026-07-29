#!/usr/bin/env python3
"""Build deterministic CAL-09 candidate figures from retained canopy evidence."""

from __future__ import annotations

import csv
import hashlib
import math
from collections import defaultdict
from datetime import datetime
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.dates as mdates
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
OUTPUT = PACKAGE / "artifacts/figure-candidates"

CAL04A = (
    ROOT
    / "docs/work-packages/"
    "20260726-canopy-cal-04a-best-available-evidence-daymet-001/artifacts"
)
CAL04B = (
    ROOT
    / "docs/work-packages/"
    "20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts"
)
CAL05 = (
    ROOT
    / "docs/work-packages/"
    "20260728-canopy-cal-05-litter-source-decomposition-readiness-001/artifacts"
)
CAL06 = (
    ROOT
    / "docs/work-packages/"
    "20260728-canopy-cal-06-canopy-gradient-congruence-001/artifacts"
)
CAL07C = (
    ROOT
    / "docs/work-packages/"
    "20260728-canopy-cal-07c-hourly-vpd-forcing-reconstruction-001/artifacts"
)
CAL07F = (
    ROOT
    / "docs/work-packages/"
    "20260729-canopy-cal-07f-observation-product-operator-audit-001/artifacts"
)

F4_SUMMARY = OUTPUT / "f4-temperate-timing-summary.csv"

HARVARD_SNOW = (
    ROOT
    / "tests/fixtures/cancov_forest/observations/sites/"
    "harvard_hf237_strata.csv"
)
MARCELL_SNOW = (
    ROOT
    / "tests/fixtures/cancov_forest/observations/sites/"
    "marcell_rds_2021_0016_stratum_means.csv"
)

COLORS = {
    "open": "#4D4D4D",
    "deciduous": "#2B7A0B",
    "mixed": "#8B5A2B",
    "conifer": "#2456A6",
    "observed": "#111111",
    "model": "#0072B2",
}
LINESTYLES = {
    "open": (0, (1, 2)),
    "deciduous": "solid",
    "mixed": (0, (6, 3)),
    "conifer": (0, (3, 2, 1, 2)),
}
SPECIES_COLORS = {
    "ACSA3": "#0072B2",
    "ACSA": "#0072B2",
    "BEAL": "#D55E00",
    "FAGR": "#009E73",
}

plt.rcParams.update(
    {
        "font.family": "DejaVu Sans",
        "font.size": 9,
        "axes.titlesize": 11,
        "axes.labelsize": 9,
        "legend.fontsize": 8,
        "figure.titlesize": 15,
        "svg.fonttype": "none",
        "svg.hashsalt": "openwepp-cal09-candidate-v1",
    }
)


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def save(fig: plt.Figure, stem: str) -> None:
    path = OUTPUT / f"{stem}.svg"
    fig.savefig(
        path,
        format="svg",
        bbox_inches="tight",
        metadata={"Date": None, "Creator": "CAL-09 deterministic candidate builder"},
    )
    plt.close(fig)
    normalized = "\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n"
    path.write_text(normalized)


def month_axis(axis: plt.Axes) -> None:
    axis.set_xlim(1, 365)
    axis.set_xticks([15, 105, 196, 288, 350], ["Jan", "Apr", "Jul", "Oct", "Dec"])
    axis.grid(axis="y", color="#D9E2EC", linewidth=0.7)


def band(
    axis: plt.Axes,
    frame: pd.DataFrame,
    value: str,
    color: str,
    label: str,
    linestyle: object = "solid",
    alpha: float = 0.14,
) -> None:
    summary = (
        frame.groupby("day_of_year")[value]
        .quantile([0.05, 0.5, 0.95])
        .unstack()
        .sort_index()
    )
    axis.fill_between(
        summary.index,
        summary[0.05],
        summary[0.95],
        color=color,
        alpha=alpha,
        linewidth=0,
    )
    axis.plot(
        summary.index,
        summary[0.5],
        color=color,
        linewidth=1.8,
        linestyle=linestyle,
        label=label,
    )


def candidate_rows(path: Path, accepted: set[str]) -> pd.DataFrame:
    parts: list[pd.DataFrame] = []
    for chunk in pd.read_csv(path, chunksize=250_000):
        selected = chunk[chunk["candidate_id"].isin(accepted)]
        if not selected.empty:
            parts.append(selected)
    if not parts:
        raise ValueError(f"no accepted candidate rows found in {path}")
    return pd.concat(parts, ignore_index=True)


def timing_summary(frame: pd.DataFrame, lane: str) -> pd.DataFrame:
    finite = frame[pd.to_numeric(frame["crossing_doy"], errors="coerce").notna()].copy()
    finite["crossing_doy"] = finite["crossing_doy"].astype(float)
    finite["observed_midpoint"] = (
        finite["lower_doy"].astype(float) + finite["upper_doy"].astype(float)
    ) / 2
    grouped = finite.groupby(["year", "species"], as_index=False).agg(
        observed_midpoint=("observed_midpoint", "median"),
        observed_lower=("lower_doy", "min"),
        observed_upper=("upper_doy", "max"),
        modeled_p05=("crossing_doy", lambda values: values.quantile(0.05)),
        modeled_median=("crossing_doy", "median"),
        modeled_p95=("crossing_doy", lambda values: values.quantile(0.95)),
        record_member_count=("crossing_doy", "size"),
    )
    grouped.insert(0, "lane", lane)
    return grouped


def f1_coefficient_response(daily: pd.DataFrame, configs: pd.DataFrame) -> None:
    frame = daily[(daily.site == "harvard") & (daily.stratum == "deciduous")].copy()
    crossings: list[tuple[str, int]] = []
    for member, group in frame.groupby("member_id"):
        ordered = group.sort_values("day_of_year")
        found = ordered[ordered.gsi21 >= 0.5]
        crossings.append((member, int(found.day_of_year.iloc[0]) if not found.empty else 999))
    crossings.sort(key=lambda item: (item[1], item[0]))
    selected = [crossings[0][0], crossings[len(crossings) // 2][0], crossings[-1][0]]
    labels = ("earliest", "middle", "latest")
    colors = ("#0072B2", "#009E73", "#D55E00")
    dashes = ("solid", (0, (6, 3)), (0, (2, 2)))

    fig, axes = plt.subplots(3, 1, figsize=(11.4, 8.2), sharex=True)
    fields = (
        ("gsi21", "GSI21 (fraction)"),
        ("canopy_cover_fraction", "Canopy cover (fraction)"),
        ("lai_m2_m2", "Leaf area index (m² m⁻²)"),
    )
    table_rows: list[list[str]] = []
    for member, timing_label, color, dash in zip(
        selected, labels, colors, dashes, strict=True
    ):
        values = frame[frame.member_id == member].sort_values("day_of_year")
        config = configs[configs.candidate_id == member].iloc[0]
        table_rows.append(
            [
                f"{timing_label}: {member}",
                f"{config.minimum_temperature_inactive_c:.1f}",
                f"{config.minimum_temperature_unconstrained_c:.1f}",
                f"{config.vapor_pressure_deficit_unconstrained_pa:.0f}",
                f"{config.vapor_pressure_deficit_inactive_pa:.0f}",
                f"{config.photoperiod_inactive_hours:.1f}",
                f"{config.photoperiod_unconstrained_hours:.1f}",
            ]
        )
        for axis, (field, ylabel) in zip(axes, fields, strict=True):
            axis.plot(
                values.day_of_year,
                values[field],
                color=color,
                linestyle=dash,
                linewidth=2,
            )
            axis.set_ylabel(ylabel)
            axis.grid(axis="y", color="#D9E2EC", linewidth=0.7)
    axes[0].set_ylim(-0.03, 1.03)
    axes[1].set_ylim(-0.03, 1.03)
    month_axis(axes[-1])
    axes[0].legend(
        [
            f"{role}: {member}"
            for role, member in zip(labels, selected, strict=True)
        ],
        frameon=False,
        ncol=3,
        loc="upper center",
    )
    coefficient_table = axes[-1].table(
        cellText=table_rows,
        colLabels=(
            "Accepted exemplar",
            "T inactive\n(°C)",
            "T free\n(°C)",
            "VPD free\n(Pa)",
            "VPD inactive\n(Pa)",
            "P inactive\n(h)",
            "P free\n(h)",
        ),
        cellLoc="center",
        bbox=(0.02, -0.72, 0.96, 0.48),
    )
    coefficient_table.auto_set_font_size(False)
    coefficient_table.set_fontsize(7.5)
    for column in range(len(table_rows[0])):
        header = coefficient_table[(0, column)]
        header.set_height(header.get_height() * 1.55)
        header.set_text_props(va="center")
    fig.suptitle("Candidate F1 — Accepted coefficient combinations shift seasonal state")
    fig.text(
        0.5,
        0.925,
        "Harvard deciduous 45-year daily climatology; examples span accepted-ensemble leaf-on timing",
        ha="center",
    )
    fig.text(
        0.5,
        0.115,
        "Combined parameter exemplars show covariance; they do not isolate one coefficient's causal effect.",
        ha="center",
        fontsize=8,
    )
    fig.subplots_adjust(top=0.88, bottom=0.30, hspace=0.12)
    save(fig, "f1-coefficient-response")

    configs[configs.candidate_id.isin(selected)].to_csv(
        OUTPUT / "f1-exemplar-coefficients.csv", index=False
    )


def f2_forest_class(daily: pd.DataFrame) -> None:
    fig, axes = plt.subplots(2, 3, figsize=(13.2, 7.2), sharex=True)
    sites = ("marcell", "harvard", "hubbard_brook")
    labels = ("Marcell", "Harvard", "Hubbard Brook")
    for column, (site, site_label) in enumerate(zip(sites, labels, strict=True)):
        site_frame = daily[daily.site == site]
        for stratum in ("open", "deciduous", "mixed", "conifer"):
            subset = site_frame[site_frame.stratum == stratum]
            if subset.empty:
                continue
            band(
                axes[0, column],
                subset,
                "canopy_cover_fraction",
                COLORS[stratum],
                stratum,
                LINESTYLES[stratum],
            )
            band(
                axes[1, column],
                subset,
                "lai_m2_m2",
                COLORS[stratum],
                stratum,
                LINESTYLES[stratum],
            )
        axes[0, column].set_title(site_label)
        axes[0, column].set_ylim(-0.03, 1.03)
        axes[1, column].set_ylim(bottom=-0.03)
        month_axis(axes[1, column])
        axes[1, column].set_xlabel("Calendar season")
    axes[0, 0].set_ylabel("Canopy cover (fraction)")
    axes[1, 0].set_ylabel("Leaf area index (m² m⁻²)")
    handles = [
        plt.Line2D(
            [0],
            [0],
            color=COLORS[name],
            linestyle=LINESTYLES[name],
            linewidth=2,
            label=name,
        )
        for name in ("open", "deciduous", "mixed", "conifer")
    ]
    fig.legend(handles=handles, loc="lower center", ncol=4, frameon=False)
    fig.suptitle("Candidate F2 — Forest classes retain distinct seasonal canopy states")
    fig.text(
        0.5,
        0.925,
        "Lines are 37-member medians; translucent ranges span the 5th–95th percentiles.",
        ha="center",
    )
    fig.subplots_adjust(top=0.87, bottom=0.12, wspace=0.18, hspace=0.15)
    save(fig, "f2-forest-class-seasonality")


def f3_litter_residue_frost(daily: pd.DataFrame) -> None:
    frame = daily[(daily.site == "harvard") & (daily.stratum.isin(["deciduous", "mixed"]))]
    fig, axes = plt.subplots(3, 1, figsize=(11.4, 8.2), sharex=True)
    fields = (
        ("leaf_litter_kg_m2", "Daily leaf litter (kg m⁻²)"),
        ("surface_residue_kg_m2", "Surface residue (kg m⁻²)"),
        ("frost_depth_mm", "Frost depth (mm)"),
    )
    for stratum in ("deciduous", "mixed"):
        subset = frame[frame.stratum == stratum]
        for axis, (field, ylabel) in zip(axes, fields, strict=True):
            band(
                axis,
                subset,
                field,
                COLORS[stratum],
                stratum,
                LINESTYLES[stratum],
            )
            axis.set_ylabel(ylabel)
    month_axis(axes[-1])
    axes[-1].set_xlabel("Calendar season")
    axes[0].legend(frameon=False, ncol=2)
    fig.suptitle("Candidate F3 — Leaf-off transfer propagates into residue and frost state")
    fig.text(
        0.5,
        0.925,
        "Harvard 45-year daily climatology across the accepted timing ensemble",
        ha="center",
    )
    fig.subplots_adjust(top=0.88, hspace=0.13)
    save(fig, "f3-litter-residue-frost")


def plot_timing_lane(axis: plt.Axes, frame: pd.DataFrame, title: str) -> None:
    for species, group in frame.groupby("species"):
        group = group.sort_values("year")
        color = SPECIES_COLORS.get(species, "#7A5195")
        axis.fill_between(
            group.year,
            group.modeled_p05,
            group.modeled_p95,
            color=color,
            alpha=0.12,
        )
        axis.plot(
            group.year,
            group.modeled_median,
            color=color,
            linestyle=(0, (5, 2)),
            linewidth=1.7,
            label=f"{species} modeled",
        )
        axis.errorbar(
            group.year,
            group.observed_midpoint,
            yerr=[
                group.observed_midpoint - group.observed_lower,
                group.observed_upper - group.observed_midpoint,
            ],
            fmt="o",
            markersize=3.5,
            color=color,
            capsize=2,
            label=f"{species} observed",
        )
    axis.set_title(title)
    axis.set_xlabel("Observation year")
    axis.set_ylabel("Day of year")
    axis.grid(color="#D9E2EC", linewidth=0.7)


def f4_temperate_timing() -> None:
    summary = pd.read_csv(F4_SUMMARY)
    cal_summary = summary[summary.lane == "Hubbard calibration"].copy()
    hv_summary = summary[summary.lane == "Harvard holdout"].copy()
    if cal_summary.empty or hv_summary.empty:
        raise ValueError("F4 summary must retain both calibration and holdout lanes")

    fig, axes = plt.subplots(2, 1, figsize=(12.0, 8.5), sharey=False)
    plot_timing_lane(axes[0], cal_summary, "Hubbard Brook calibration — spring half expansion")
    plot_timing_lane(axes[1], hv_summary, "Harvard independent holdout — autumn leaf fall")
    handles, labels = axes[0].get_legend_handles_labels()
    unique: dict[str, object] = {}
    for handle, label in zip(handles, labels, strict=True):
        unique.setdefault(label, handle)
    fig.legend(
        unique.values(),
        unique.keys(),
        loc="lower center",
        ncol=3,
        frameon=False,
    )
    fig.suptitle("Candidate F4 — Calibration fit does not transfer to Harvard chronology")
    fig.text(
        0.5,
        0.925,
        "Observed symbols show interval midpoints and bounds; dashed lines and bands show accepted-ensemble crossings.",
        ha="center",
    )
    fig.subplots_adjust(top=0.88, bottom=0.15, hspace=0.28)
    save(fig, "f4-temperate-observed-modeled-timing")


def f5_source_decay() -> None:
    design = pd.read_csv(CAL05 / "terminal-stock-ridge-design.csv")
    fig, axes = plt.subplots(1, 2, figsize=(12.0, 5.2))
    styles = ("solid", (0, (6, 3)), (0, (3, 2)), (0, (2, 2)), (0, (8, 2, 2, 2)))
    colors = ("#0072B2", "#009E73", "#E69F00", "#D55E00", "#7A5195")
    for (_, row), color, style in zip(design.iterrows(), colors, styles, strict=True):
        rate = float(row["surface_rate_d-1"])
        annual = float(row["synthetic_annual_surface_litter_input_kg_m2_yr"])
        stock = 0.2
        years = [0]
        stocks = [stock]
        daily_source = annual / 365.0
        daily_decay = math.exp(-rate)
        for year in range(1, 21):
            for _ in range(365):
                stock = stock * daily_decay + daily_source
            years.append(year)
            stocks.append(stock)
        label = row.ridge_id.replace("RIDGE-", "")
        axes[0].plot(
            years,
            stocks,
            color=color,
            linestyle=style,
            linewidth=2,
            label=label,
            zorder=3,
        )
        axes[1].scatter(
            float(row["yearly_rate_yr-1"]),
            annual,
            color=color,
            s=42,
            label=label,
            zorder=4,
        )
        label_offset = (-6, -5) if label == "K200" else (5, 3)
        label_alignment = "right" if label == "K200" else "left"
        label_vertical_alignment = "top" if label == "K200" else "baseline"
        axes[1].annotate(
            label,
            (float(row["yearly_rate_yr-1"]), annual),
            xytext=label_offset,
            textcoords="offset points",
            ha=label_alignment,
            va=label_vertical_alignment,
            zorder=5,
        )
    axes[0].axhline(
        float(design.target_terminal_stock_kg_m2.iloc[0]),
        color="#555555",
        linewidth=1,
        linestyle=(0, (2, 2)),
    )
    axes[0].set(xlabel="Simulation year", ylabel="Surface residue stock (kg m⁻²)")
    axes[1].set(
        xlabel="Decomposition rate (yr⁻¹)",
        ylabel="Annual synthetic litter source (kg m⁻² yr⁻¹)",
        xlim=(-0.1, 2.1),
    )
    for axis in axes:
        axis.set_axisbelow(True)
        axis.grid(color="#D9E2EC", linewidth=0.7, zorder=0)
    axes[0].legend(title="Ridge member", frameon=False, ncol=2)
    fig.suptitle("Candidate F5 — Different source and decay pairs converge on one terminal stock")
    fig.text(
        0.5,
        0.90,
        "Synthetic, assumed-for-execution ridge; it demonstrates operator equifinality, not an empirical forest fit.",
        ha="center",
    )
    fig.subplots_adjust(top=0.82, wspace=0.28)
    save(fig, "f5-source-decay-trajectories")


def observed_snow_climatology(path: Path, site: str) -> pd.DataFrame:
    frame = pd.read_csv(path)
    frame = frame[(frame.binding_status == "bound") & frame.observed_snow_depth_m.notna()].copy()
    frame["day_of_year"] = pd.to_datetime(frame.date).dt.dayofyear
    mapping = {"hardwood": "deciduous"}
    frame["stratum"] = frame.observed_stratum.replace(mapping)
    frame["site"] = site
    return (
        frame.groupby(["site", "stratum", "day_of_year"], as_index=False)
        .observed_snow_depth_m.agg(["median", "count"])
        .reset_index()
    )


def f6_canopy_gradient(daily: pd.DataFrame) -> None:
    observations = pd.concat(
        [
            observed_snow_climatology(HARVARD_SNOW, "harvard"),
            observed_snow_climatology(MARCELL_SNOW, "marcell"),
        ],
        ignore_index=True,
    )
    observations.to_csv(OUTPUT / "f6-observed-snow-climatology.csv", index=False)
    fig, axes = plt.subplots(2, 2, figsize=(13.0, 7.5), sharex="col")
    for column, (site, site_label) in enumerate(
        (("harvard", "Harvard"), ("marcell", "Marcell"))
    ):
        model = daily[daily.site == site]
        for stratum in ("open", "deciduous", "mixed", "conifer"):
            subset = model[model.stratum == stratum]
            if subset.empty:
                continue
            snow = subset.copy()
            snow["snow_depth_m"] = snow.snow_depth_mm / 1000.0
            band(
                axes[0, column],
                snow,
                "snow_depth_m",
                COLORS[stratum],
                f"{stratum} model",
                LINESTYLES[stratum],
                alpha=0.10,
            )
            band(
                axes[1, column],
                subset,
                "canopy_cover_fraction",
                COLORS[stratum],
                stratum,
                LINESTYLES[stratum],
                alpha=0.10,
            )
            observed = observations[
                (observations.site == site) & (observations.stratum == stratum)
            ]
            if not observed.empty:
                axes[0, column].scatter(
                    observed.day_of_year,
                    observed["median"],
                    s=9,
                    facecolors="white",
                    edgecolors=COLORS[stratum],
                    linewidths=0.7,
                    alpha=0.8,
                    label=f"{stratum} observed",
                )
        axes[0, column].set_title(f"{site_label} snow depth")
        axes[1, column].set_title(f"{site_label} modeled canopy cover")
        month_axis(axes[1, column])
        axes[1, column].set_xlabel("Calendar season")
    axes[0, 0].set_ylabel("Snow depth (m)")
    axes[1, 0].set_ylabel("Canopy cover (fraction)")
    handles, labels = axes[0, 1].get_legend_handles_labels()
    axes[0, 1].legend(handles, labels, loc="upper right", frameon=False, ncol=2)
    fig.suptitle("Candidate F6 — Canopy-gradient seasonality and snow response")
    fig.text(
        0.5,
        0.925,
        "Model lines/bands are 45-year climatologies across 37 members; open symbols are observed day-of-year medians.",
        ha="center",
    )
    fig.text(
        0.5,
        0.015,
        "The overlay shows seasonal congruence, not paired-date validation; Harvard SWE is excluded.",
        ha="center",
        fontsize=8,
    )
    fig.subplots_adjust(top=0.87, bottom=0.10, wspace=0.18, hspace=0.22)
    save(fig, "f6-canopy-gradient-snow-response")


def normalize_group(values: pd.Series) -> pd.Series:
    low = values.min()
    high = values.max()
    if not np.isfinite(low) or not np.isfinite(high) or high <= low:
        return pd.Series(np.nan, index=values.index)
    return (values - low) / (high - low)


def f7_hemisphere(ensemble: pd.DataFrame) -> None:
    frame = ensemble[ensemble.year.isin([2024, 2025])].copy()
    frame["date"] = pd.to_datetime(frame.date)
    frame["observed_relative"] = frame.groupby(["site_id", "year"])[
        "observed_gcc90"
    ].transform(normalize_group)
    frame["observed_relative_smooth"] = (
        frame.groupby(["site_id", "year"], group_keys=False)["observed_relative"]
        .apply(lambda values: values.rolling(15, center=True, min_periods=5).mean())
    )
    frame[
        [
            "site_id",
            "date",
            "year",
            "gsi_p05",
            "gsi_median",
            "gsi_p95",
            "observed_relative",
            "observed_relative_smooth",
        ]
    ].to_csv(OUTPUT / "f7-relative-seasonality.csv", index=False)
    fig, axes = plt.subplots(2, 1, figsize=(12.2, 7.6), sharex=True)
    names = (("SH-EN-ALERCE", "Alerce evergreen"), ("SH-DB-BEZA", "Bezà dry forest"))
    for axis, (site, label) in zip(axes, names, strict=True):
        subset = frame[frame.site_id == site].sort_values("date")
        axis.fill_between(
            subset.date,
            subset.gsi_p05,
            subset.gsi_p95,
            color="#0072B2",
            alpha=0.18,
            label="model 5th–95th percentile",
        )
        axis.plot(subset.date, subset.gsi_median, color="#0072B2", linewidth=1.8, label="model median GSI")
        axis.plot(
            subset.date,
            subset.observed_relative,
            color="#777777",
            linewidth=0.55,
            alpha=0.45,
            label="observed GCC90, daily",
        )
        axis.plot(
            subset.date,
            subset.observed_relative_smooth,
            color="#111111",
            linewidth=1.5,
            label="observed GCC90, 15-day mean",
        )
        axis.set_ylabel("Relative seasonal activity")
        axis.set_title(label)
        axis.set_ylim(-0.05, 1.05)
        axis.grid(axis="y", color="#D9E2EC", linewidth=0.7)
    axes[-1].xaxis.set_major_locator(mdates.MonthLocator(interval=3))
    axes[-1].xaxis.set_major_formatter(mdates.DateFormatter("%b %Y"))
    axes[-1].set_xlabel("Date")
    axes[0].legend(frameon=False, ncol=4)
    fig.suptitle("Candidate F7 — Southern Hemisphere model and observed seasonal trends")
    fig.text(
        0.5,
        0.925,
        "Observed GCC is normalized within site-year; amplitude agreement is not evaluated.",
        ha="center",
    )
    fig.subplots_adjust(top=0.87, hspace=0.25)
    save(fig, "f7-hemisphere-seasonality")


def f8_beza_products(ensemble: pd.DataFrame, products: pd.DataFrame) -> None:
    model = ensemble[(ensemble.site_id == "SH-DB-BEZA") & ensemble.year.isin([2024, 2025])].copy()
    model["date"] = pd.to_datetime(model.date)
    products = products[products.year.isin([2024, 2025])].copy()
    products["date"] = pd.to_datetime(products.date)
    for field in ("smooth_gcc_mean", "smooth_gcc_90"):
        products[f"{field}_relative"] = products.groupby("year")[field].transform(normalize_group)
    fig, axes = plt.subplots(2, 1, figsize=(12.2, 7.6), sharex=False)
    for axis, year in zip(axes, (2024, 2025), strict=True):
        year_model = model[model.year == year]
        year_obs = products[products.year == year]
        axis.fill_between(
            year_model.date,
            year_model.gsi_p05,
            year_model.gsi_p95,
            color="#0072B2",
            alpha=0.18,
            label="model 5th–95th percentile",
        )
        axis.plot(year_model.date, year_model.gsi_median, color="#0072B2", linewidth=1.8, label="model median GSI")
        axis.plot(
            year_obs.date,
            year_obs.smooth_gcc_mean_relative,
            color="#D55E00",
            linewidth=1.3,
            label="observed gcc_mean",
        )
        axis.plot(
            year_obs.date,
            year_obs.smooth_gcc_90_relative,
            color="#009E73",
            linestyle=(0, (5, 2)),
            linewidth=1.3,
            label="observed gcc_90",
        )
        axis.set_title(str(year))
        axis.set_ylabel("Relative seasonal activity")
        axis.set_ylim(-0.05, 1.05)
        axis.grid(axis="y", color="#D9E2EC", linewidth=0.7)
        axis.xaxis.set_major_locator(mdates.MonthLocator(interval=2))
        axis.xaxis.set_major_formatter(mdates.DateFormatter("%b"))
    axes[-1].set_xlabel("Calendar month")
    axes[0].legend(frameon=False, ncol=4)
    fig.suptitle("Candidate F8 — Bezà observed products and model chronology disagree")
    fig.text(
        0.5,
        0.925,
        "Both observation products lead to the same no-calibration decision; each series is shown on a relative annual scale.",
        ha="center",
    )
    fig.subplots_adjust(top=0.87, hspace=0.26)
    save(fig, "f8-beza-observed-modeled")


def write_manifest(paths: list[tuple[Path, str]]) -> None:
    with (OUTPUT / "source-manifest.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=("path", "sha256", "role"),
            lineterminator="\n",
        )
        writer.writeheader()
        for path, role in paths:
            writer.writerow(
                {
                    "path": path.relative_to(ROOT),
                    "sha256": digest(path),
                    "role": role,
                }
            )


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    daily_path = CAL06 / "daily-climatology.csv"
    configs_path = CAL04B / "candidate-configurations.csv"
    accepted_path = CAL04B / "accepted-calibration-ensemble.csv"
    ensemble_path = CAL07C / "ensemble-daily.csv"
    products_path = CAL07F / "daily-product-curves.csv"

    daily = pd.read_csv(daily_path)
    configs = pd.read_csv(configs_path)
    accepted_frame = pd.read_csv(accepted_path)
    accepted = set(accepted_frame.candidate_id)
    ensemble = pd.read_csv(ensemble_path)
    products = pd.read_csv(products_path)

    f1_coefficient_response(daily, configs)
    f2_forest_class(daily)
    f3_litter_residue_frost(daily)
    f4_temperate_timing()
    f5_source_decay()
    f6_canopy_gradient(daily)
    f7_hemisphere(ensemble)
    f8_beza_products(ensemble, products)

    write_manifest(
        [
            (daily_path, "CAL-06 retained model daily climatology"),
            (configs_path, "CAL-04B candidate coefficient definitions"),
            (accepted_path, "CAL-04B accepted frozen ensemble"),
            (CAL05 / "terminal-stock-ridge-design.csv", "CAL-05 synthetic source-decay design"),
            (ensemble_path, "CAL-07C retained Southern Hemisphere ensemble daily series"),
            (products_path, "CAL-07F retained Bezà observation products"),
            (HARVARD_SNOW, "CAL-06 admitted Harvard snow observations"),
            (MARCELL_SNOW, "CAL-06 admitted Marcell snow observations"),
            (F4_SUMMARY, "CAL-09 retained temperate timing figure rows"),
        ]
    )
    print("PASS candidate_figures=8")


if __name__ == "__main__":
    main()
