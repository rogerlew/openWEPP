#!/usr/bin/env python3
"""Copyright-safe Figure 9 Zone 1 / Zone 2 taxonomy harness.

Reads the local Papanicolaou 2018 supplemental Figure_9 workbook, verifies its
sha256, parses the derived stream-power table, and emits scalar taxonomy
summaries only. The workbook rows remain in the ignored local reference cache.
"""

import argparse
import hashlib
import json
import math
import sys

import numpy as np
import pandas as pd


FIG9_SHA256 = "ec198018d34414298b08419ba1b303de86ebd11b642a03731a0e68e2b04b8f28"

SECTIONS = {
    "bare": {
        "label": "Bare",
        "published_i_star": 0.16,
        "published_psi_star": 0.004,
    },
    "isolated": {
        "label": "Clods",
        "published_i_star": 0.33,
        "published_psi_star": 0.017,
    },
    "vegetation": {
        "label": "Vegetation",
        "published_i_star": 0.68,
        "published_psi_star": 0.022,
    },
}


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()


def section_starts(raw):
    starts = {}
    for key, spec in SECTIONS.items():
        label = spec["label"]
        matches = []
        for row_idx, row in raw.iterrows():
            for value in row:
                if isinstance(value, str) and value.strip() == label:
                    matches.append(row_idx)
        if len(matches) != 1:
            raise ValueError(f"expected exactly one section label {label!r}, found {matches}")
        starts[key] = matches[0]
    return starts


def parse_section(raw, key, starts):
    ordered = sorted(starts.items(), key=lambda item: item[1])
    start = starts[key]
    later = [idx for _, idx in ordered if idx > start]
    end = min(later) if later else len(raw)
    data = raw.iloc[start + 2 : end, 0:7].copy()
    data.columns = ["P_mm_h", "Q_m2_s_m", "S_pct", "I_star", "Q_star", "S_star", "Psi_star"]
    for column in data.columns:
        data[column] = pd.to_numeric(data[column], errors="coerce")
    return data.dropna(subset=["P_mm_h", "Q_m2_s_m", "S_pct", "I_star", "Q_star", "S_star", "Psi_star"])


def fit_power_law(subset):
    x = np.log(subset["I_star"].to_numpy(dtype=float))
    y = np.log(subset["Psi_star"].to_numpy(dtype=float))
    slope, intercept = np.polyfit(x, y, 1)
    predicted = slope * x + intercept
    ss_res = float(np.sum((y - predicted) ** 2))
    ss_tot = float(np.sum((y - np.mean(y)) ** 2))
    r2 = 1.0 if ss_tot == 0.0 else 1.0 - ss_res / ss_tot
    return {"k": float(math.exp(intercept)), "l": float(slope), "r2": r2, "n": int(len(subset))}


def fit_by_slope(data, threshold, zone):
    fits = []
    for slope, group in data.groupby("S_pct"):
        group = group.sort_values("I_star")
        if zone == "zone1":
            subset = group[group["I_star"] < threshold]
        else:
            subset = group[group["I_star"] >= threshold]
        if len(subset) < 2:
            continue
        fit = fit_power_law(subset)
        fit["S_pct"] = float(slope)
        fits.append(fit)
    return fits


def summarize_fits(fits):
    if not fits:
        return {"fit_count": 0}
    l_values = [fit["l"] for fit in fits]
    r2_values = [fit["r2"] for fit in fits]
    return {
        "fit_count": len(fits),
        "l_min": min(l_values),
        "l_max": max(l_values),
        "l_mean": sum(l_values) / len(l_values),
        "r2_min": min(r2_values),
    }


def nearest_threshold_support(data, i_star, psi_star):
    nearest_i = data.iloc[(data["I_star"] - i_star).abs().argsort()[:1]].iloc[0]
    nearest_psi = data.iloc[(data["Psi_star"] - psi_star).abs().argsort()[:1]].iloc[0]
    return {
        "nearest_i_star": float(nearest_i["I_star"]),
        "nearest_i_star_abs_error": float(abs(nearest_i["I_star"] - i_star)),
        "nearest_i_star_p_mm_h": float(nearest_i["P_mm_h"]),
        "nearest_psi_star": float(nearest_psi["Psi_star"]),
        "nearest_psi_star_abs_error": float(abs(nearest_psi["Psi_star"] - psi_star)),
        "nearest_psi_star_rel_error": float(abs(nearest_psi["Psi_star"] - psi_star) / psi_star),
        "nearest_psi_star_s_pct": float(nearest_psi["S_pct"]),
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fig9", required=True, help="path to gitignored Figure_9.xlsx")
    parser.add_argument("--no-assert", action="store_true", help="emit metrics without failing assertions")
    args = parser.parse_args()

    got = sha256(args.fig9)
    if got != FIG9_SHA256:
        print(json.dumps({"error": "sha256 mismatch", "expected": FIG9_SHA256, "got": got}))
        sys.exit(3)

    raw = pd.read_excel(args.fig9, sheet_name="Results", header=None)
    starts = section_starts(raw)
    result = {
        "source": {
            "fig9_sha256": got,
            "sheet": "Results",
            "normalizing_intensity_mm_h": float(raw.iloc[0, 1]),
            "normalizing_discharge_m2_s_m": float(raw.iloc[1, 1]),
            "normalizing_slope_pct": float(raw.iloc[2, 1]),
        },
        "sections": {},
    }

    for key, spec in SECTIONS.items():
        data = parse_section(raw, key, starts)
        threshold = spec["published_i_star"]
        zone1_fits = fit_by_slope(data, threshold, "zone1")
        zone2_fits = fit_by_slope(data, threshold, "zone2")
        result["sections"][key] = {
            "workbook_label": spec["label"],
            "rows": int(len(data)),
            "slope_count": int(data["S_pct"].nunique()),
            "published_threshold": {
                "I_star": spec["published_i_star"],
                "Psi_star": spec["published_psi_star"],
            },
            "threshold_support": nearest_threshold_support(
                data, spec["published_i_star"], spec["published_psi_star"]
            ),
            "zone1_fit_summary": summarize_fits(zone1_fits),
            "zone2_fit_summary": summarize_fits(zone2_fits),
            "zone1_fit_s_pct": [fit["S_pct"] for fit in zone1_fits],
            "zone2_fit_s_pct": [fit["S_pct"] for fit in zone2_fits],
        }

    if not args.no_assert:
        thresholds = [result["sections"][key]["published_threshold"]["I_star"] for key in SECTIONS]
        if thresholds != sorted(thresholds):
            raise AssertionError("published critical intensity ordering is not bare < isolated < vegetation")

        for key, section in result["sections"].items():
            support = section["threshold_support"]
            if support["nearest_i_star_abs_error"] > 0.02:
                raise AssertionError(f"{key}: published I* threshold is not supported by the workbook grid")
            if support["nearest_psi_star_rel_error"] > 0.10:
                raise AssertionError(
                    f"{key}: published Psi* threshold is not supported within 10% by the workbook grid"
                )
            zone2 = section["zone2_fit_summary"]
            if zone2["fit_count"] == 0:
                raise AssertionError(f"{key}: missing Zone 2 fit support")
            if not (0.95 <= zone2["l_min"] <= zone2["l_max"] <= 1.15):
                raise AssertionError(f"{key}: Zone 2 l range is not near-linear")
            if zone2["r2_min"] < 0.98:
                raise AssertionError(f"{key}: Zone 2 fit is not log-linear enough")

        for key in ["isolated", "vegetation"]:
            zone1 = result["sections"][key]["zone1_fit_summary"]
            if zone1["fit_count"] == 0:
                raise AssertionError(f"{key}: missing Zone 1 fit support")
            if zone1["l_mean"] <= 1.20:
                raise AssertionError(f"{key}: Zone 1 l mean does not show roughness-driven nonlinearity")

    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
