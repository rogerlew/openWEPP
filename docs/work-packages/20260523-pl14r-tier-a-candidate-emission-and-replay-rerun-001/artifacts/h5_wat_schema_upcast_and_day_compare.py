#!/usr/bin/env python3
"""Upcast 20-column H5.wat candidate rows to canonical 25-column schema and compare day-by-day.

This tool is fixture-oriented and deterministic:
- Candidate rows with 20 columns are expanded to 25 columns by deriving
  `SoilWaterTotal = Total-Soil + frozwt` and injecting profile-capacity fields.
- Profile fields default to constants inferred from the baseline 25-column file,
  with optional CLI overrides.
"""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path
from typing import Iterable

WB13_COLUMNS = [
    "OFE",
    "J",
    "Y",
    "P",
    "RM",
    "Q",
    "Ep",
    "Es",
    "Er",
    "Dp",
    "UpStrmQ",
    "SubRIn",
    "latqcc",
    "Total-Soil",
    "frozwt",
    "Snow-Water",
    "QOFE",
    "Tile",
    "Irr",
    "Area",
    "SoilWaterTotal",
    "ProfileDepth",
    "ProfilePorosityCap",
    "ProfileFCStore",
    "ProfileWPStore",
]

NUMERIC_20 = 20
NUMERIC_25 = 25


def parse_numeric_rows(path: Path) -> list[list[float]]:
    rows: list[list[float]] = []
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        parts = stripped.split()
        if len(parts) not in {NUMERIC_20, NUMERIC_25}:
            continue
        try:
            nums = [float(part) for part in parts]
        except ValueError:
            continue
        # Data rows always start with OFE/J/Y integer-like fields.
        if not all(float(int(nums[idx])) == nums[idx] for idx in (0, 1, 2)):
            continue
        rows.append(nums)
    return rows


def find_data_start_index(path: Path) -> int:
    lines = path.read_text(encoding="utf-8", errors="ignore").splitlines(keepends=True)
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if not stripped:
            continue
        parts = stripped.split()
        if len(parts) not in {NUMERIC_20, NUMERIC_25}:
            continue
        try:
            nums = [float(part) for part in parts]
        except ValueError:
            continue
        if all(float(int(nums[i])) == nums[i] for i in (0, 1, 2)):
            return idx
    raise ValueError(f"No numeric data row found in {path}")


def infer_profile_constants(baseline_rows: list[list[float]]) -> tuple[float, float, float, float]:
    if not baseline_rows:
        raise ValueError("Cannot infer profile constants from empty baseline rows")

    depth_vals = {round(row[21], 10) for row in baseline_rows}
    porosity_vals = {round(row[22], 10) for row in baseline_rows}
    fc_vals = {round(row[23], 10) for row in baseline_rows}
    wp_vals = {round(row[24], 10) for row in baseline_rows}

    if not (len(depth_vals) == len(porosity_vals) == len(fc_vals) == len(wp_vals) == 1):
        raise ValueError(
            "Baseline profile fields are not constant for this fixture; explicit overrides are required"
        )

    return (
        next(iter(depth_vals)),
        next(iter(porosity_vals)),
        next(iter(fc_vals)),
        next(iter(wp_vals)),
    )


def upcast_candidate_rows(
    candidate_rows: list[list[float]],
    profile_depth: float,
    profile_porosity_cap: float,
    profile_fc_store: float,
    profile_wp_store: float,
) -> list[list[float]]:
    out: list[list[float]] = []
    for row in candidate_rows:
        if len(row) == NUMERIC_25:
            out.append(row)
            continue
        if len(row) != NUMERIC_20:
            raise ValueError(f"Unexpected candidate row width: {len(row)}")

        total_soil = row[13]
        frozwt = row[14]
        soil_water_total = total_soil + frozwt
        expanded = [
            *row,
            soil_water_total,
            profile_depth,
            profile_porosity_cap,
            profile_fc_store,
            profile_wp_store,
        ]
        out.append(expanded)
    return out


def format_row_25(row: list[float]) -> str:
    # Mirror canonical WB13 formatting contracts from openwepp-summary-accumulator.
    return " ".join(
        [
            str(int(row[0])),
            str(int(row[1])),
            str(int(row[2])),
            f"{row[3]:.2f}",
            f"{row[4]:.2f}",
            f"{row[5]:.7E}",
            f"{row[6]:.2f}",
            f"{row[7]:.2f}",
            f"{row[8]:.2f}",
            f"{row[9]:.2f}",
            f"{row[10]:.7E}",
            f"{row[11]:.2f}",
            f"{row[12]:.2f}",
            f"{row[13]:.2f}",
            f"{row[14]:.2f}",
            f"{row[15]:.2f}",
            f"{row[16]:.7E}",
            f"{row[17]:.2f}",
            f"{row[18]:.2f}",
            f"{row[19]:.2f}",
            f"{row[20]:.2f}",
            f"{row[21]:.2f}",
            f"{row[22]:.2f}",
            f"{row[23]:.2f}",
            f"{row[24]:.2f}",
        ]
    )


def key_for_row(row: list[float]) -> tuple[int, int, int]:
    return int(row[0]), int(row[1]), int(row[2])


def day_compare(
    baseline_rows: list[list[float]],
    candidate_rows: list[list[float]],
) -> dict:
    baseline_map = {key_for_row(row): row for row in baseline_rows}
    candidate_map = {key_for_row(row): row for row in candidate_rows}

    baseline_keys = set(baseline_map)
    candidate_keys = set(candidate_map)

    only_baseline = sorted(baseline_keys - candidate_keys)
    only_candidate = sorted(candidate_keys - baseline_keys)
    common = sorted(baseline_keys & candidate_keys)

    column_stats = []
    for idx, name in enumerate(WB13_COLUMNS):
        max_abs = 0.0
        max_rel = 0.0
        nonzero_days = 0
        max_key: tuple[int, int, int] | None = None

        for key in common:
            a_val = baseline_map[key][idx]
            b_val = candidate_map[key][idx]
            abs_diff = abs(a_val - b_val)
            denom = max(abs(a_val), abs(b_val), 1e-30)
            rel_diff = abs_diff / denom
            if abs_diff > max_abs:
                max_abs = abs_diff
                max_key = key
            if rel_diff > max_rel:
                max_rel = rel_diff
            if abs_diff > 0.0:
                nonzero_days += 1

        column_stats.append(
            {
                "column": name,
                "nonzero_days": nonzero_days,
                "max_abs_diff": max_abs,
                "max_rel_diff": max_rel,
                "max_abs_key": list(max_key) if max_key is not None else None,
            }
        )

    return {
        "baseline_row_count": len(baseline_rows),
        "candidate_row_count": len(candidate_rows),
        "common_row_count": len(common),
        "only_baseline_count": len(only_baseline),
        "only_candidate_count": len(only_candidate),
        "only_baseline_examples": [list(k) for k in only_baseline[:10]],
        "only_candidate_examples": [list(k) for k in only_candidate[:10]],
        "all_columns_exact": all(stat["nonzero_days"] == 0 for stat in column_stats),
        "column_stats": column_stats,
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--out-candidate", type=Path, required=True)
    parser.add_argument("--report-json", type=Path, required=True)

    parser.add_argument("--profile-depth", type=float, default=None)
    parser.add_argument("--profile-porosity-cap", type=float, default=None)
    parser.add_argument("--profile-fc-store", type=float, default=None)
    parser.add_argument("--profile-wp-store", type=float, default=None)
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    baseline_rows = parse_numeric_rows(args.baseline)
    candidate_rows = parse_numeric_rows(args.candidate)

    if not baseline_rows:
        raise ValueError(f"No numeric rows found in baseline file: {args.baseline}")
    if not candidate_rows:
        raise ValueError(f"No numeric rows found in candidate file: {args.candidate}")

    if any(len(row) != NUMERIC_25 for row in baseline_rows):
        raise ValueError("Baseline file must provide canonical 25-column data rows")

    inferred_depth, inferred_porosity, inferred_fc, inferred_wp = infer_profile_constants(baseline_rows)
    profile_depth = args.profile_depth if args.profile_depth is not None else inferred_depth
    profile_porosity = (
        args.profile_porosity_cap if args.profile_porosity_cap is not None else inferred_porosity
    )
    profile_fc = args.profile_fc_store if args.profile_fc_store is not None else inferred_fc
    profile_wp = args.profile_wp_store if args.profile_wp_store is not None else inferred_wp

    upcast_rows = upcast_candidate_rows(
        candidate_rows,
        profile_depth=profile_depth,
        profile_porosity_cap=profile_porosity,
        profile_fc_store=profile_fc,
        profile_wp_store=profile_wp,
    )

    header_lines = args.baseline.read_text(encoding="utf-8", errors="ignore").splitlines(keepends=True)
    data_start = find_data_start_index(args.baseline)
    prefix = "".join(header_lines[:data_start])

    args.out_candidate.parent.mkdir(parents=True, exist_ok=True)
    with args.out_candidate.open("w", encoding="utf-8") as handle:
        handle.write(prefix)
        for row in upcast_rows:
            handle.write(format_row_25(row))
            handle.write("\n")

    report = {
        "inputs": {
            "baseline": str(args.baseline),
            "candidate": str(args.candidate),
            "upcast_candidate": str(args.out_candidate),
        },
        "profile_constants": {
            "profile_depth": profile_depth,
            "profile_porosity_cap": profile_porosity,
            "profile_fc_store": profile_fc,
            "profile_wp_store": profile_wp,
            "inferred_from_baseline": {
                "profile_depth": inferred_depth,
                "profile_porosity_cap": inferred_porosity,
                "profile_fc_store": inferred_fc,
                "profile_wp_store": inferred_wp,
            },
        },
        "baseline_numeric_row_width": NUMERIC_25,
        "candidate_numeric_row_widths": sorted({len(row) for row in candidate_rows}),
        "upcast_numeric_row_width": NUMERIC_25,
        "day_compare": day_compare(baseline_rows, upcast_rows),
    }

    args.report_json.parent.mkdir(parents=True, exist_ok=True)
    args.report_json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
