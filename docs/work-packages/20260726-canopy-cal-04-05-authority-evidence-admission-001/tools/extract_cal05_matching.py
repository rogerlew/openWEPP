#!/usr/bin/env python3
"""Build the reviewable HF324 litter-flux/organic-stock plot join."""

from __future__ import annotations

import argparse
import csv
import statistics
from collections import defaultdict
from pathlib import Path

FIELDS = [
    "project",
    "plot",
    "site",
    "litter_years",
    "litter_row_count",
    "mean_foliar_g_c_m2_yr",
    "mean_pooled_nonfoliar_g_c_m2_yr",
    "mean_total_g_c_m2_yr",
    "stock_year",
    "stock_replicate_count",
    "mean_organic_horizon_c_mass_kg_c_m2",
    "stock_use_not",
    "admissibility",
]


def number(value: str) -> float | None:
    return None if value in {"", "NA"} else float(value)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("litterfall_source", type=Path)
    parser.add_argument("soil_carbon_source", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    litter: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    with args.litterfall_source.open(newline="", encoding="latin-1") as stream:
        for row in csv.DictReader(stream):
            if (
                row["type"] == "hardwood"
                and number(row["foliar.gcm2"]) is not None
                and number(row["nonfoliar.gcm2"]) is not None
                and number(row["total.gcm2"]) is not None
            ):
                litter[(row["project"], row["plot"])].append(row)

    stock: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    with args.soil_carbon_source.open(newline="", encoding="latin-1") as stream:
        for row in csv.DictReader(stream):
            if (
                row["type"] == "hardwood"
                and row["horizon"] == "organic"
                and row["use.not"] == "1"
                and number(row["c.mass.rocks"]) is not None
            ):
                stock[(row["project"], row["plot"])].append(row)

    output: list[dict[str, object]] = []
    for key in sorted(litter.keys() & stock.keys()):
        litter_rows = litter[key]
        stock_rows = stock[key]
        output.append(
            {
                "project": key[0],
                "plot": key[1],
                "site": litter_rows[0]["site"],
                "litter_years": (
                    f"{min(int(row['year']) for row in litter_rows)}-"
                    f"{max(int(row['year']) for row in litter_rows)}"
                ),
                "litter_row_count": len(litter_rows),
                "mean_foliar_g_c_m2_yr": statistics.fmean(
                    float(row["foliar.gcm2"]) for row in litter_rows
                ),
                "mean_pooled_nonfoliar_g_c_m2_yr": statistics.fmean(
                    float(row["nonfoliar.gcm2"]) for row in litter_rows
                ),
                "mean_total_g_c_m2_yr": statistics.fmean(
                    float(row["total.gcm2"]) for row in litter_rows
                ),
                "stock_year": ",".join(sorted({row["year"] for row in stock_rows})),
                "stock_replicate_count": len(stock_rows),
                "mean_organic_horizon_c_mass_kg_c_m2": statistics.fmean(
                    float(row["c.mass.rocks"]) for row in stock_rows
                ),
                "stock_use_not": "1",
                "admissibility": "PARTIAL_POOLED_NONFOLIAR",
            }
        )

    if len(output) != 28:
        raise SystemExit(f"expected 28 EMS hardwood joins, found {len(output)}")
    with args.output.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(output)
    print("wrote 28 plot-level partial CAL-05 joins")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
