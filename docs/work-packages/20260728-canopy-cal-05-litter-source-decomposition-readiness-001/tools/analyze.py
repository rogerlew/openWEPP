#!/usr/bin/env python3
"""Independent CAL-05 reconstruction and source-native diagnostics."""

from __future__ import annotations

import csv
import math
import sys
from collections import defaultdict
from pathlib import Path

TOL = 1.0e-12


def reconstruct(producer: Path, output: Path) -> None:
    groups: dict[str, list[dict[str, str]]] = defaultdict(list)
    with producer.open(newline="", encoding="utf-8") as stream:
        for row in csv.DictReader(stream):
            groups[row["candidate_id"]].append(row)
    rows = []
    for candidate, trace in sorted(groups.items()):
        expected = 0.2
        expected_interrill = 0.0
        expected_rill = 0.0
        sse = 0.0
        maximum = 0.0
        first = ""
        for row in trace:
            tmax = float(row["tmax_c"])
            tmin = float(row["tmin_c"])
            precip = float(row["precipitation_m"])
            stress = float(row["water_stress_fraction"])
            tave = (tmax + tmin) / 2.0
            t1 = (tave + 6.1) ** 2
            temp = t1 * (2.0 * 1528.81 - t1) / 1528.81**2
            surface_water = 0.0 if tave <= 0 else min(precip / 0.004, 1.0)
            env = min(temp, stress)
            decay = math.exp(-env * float(row["rate_d-1"]))
            expected = (expected + float(row["source_kg_m2"])) * decay
            expected_interrill = (expected_interrill + float(row["source_kg_m2"])) * decay
            expected_rill = (expected_rill + float(row["source_kg_m2"])) * decay
            expected_values = {
                "surface_seed_kg_m2": expected / decay - float(row["source_kg_m2"]),
                "interrill_seed_kg_m2": expected_interrill / decay - float(row["source_kg_m2"]),
                "rill_seed_kg_m2": expected_rill / decay - float(row["source_kg_m2"]),
                "temperature_factor": temp,
                "surface_water_factor": surface_water,
                "flat_water_factor": stress,
                "environment_index": env,
                "decay_factor": decay,
                "surface_after_kg_m2": expected,
                "interrill_after_kg_m2": expected_interrill,
                "rill_after_kg_m2": expected_rill,
                "root_after_kg_m2": 0.0,
                "residue_depth_m": 0.0,
                "downstream_surface_kg_m2": expected,
                "downstream_environment_index": env,
                "downstream_decay_factor": decay,
                "partition_flat_kg_m2": expected,
                "partition_total_kg_m2": expected,
            }
            for field, expected_value in expected_values.items():
                delta = abs(expected_value - float(row[field]))
                sse += delta * delta
                maximum = max(maximum, delta)
                if delta > TOL and not first:
                    first = f"{row['year']}:{row['day']}:{field}"
        rows.append(
            {
                "candidate_id": candidate,
                "row_count": len(trace),
                "terminal_stock_kg_m2": f"{expected:.17g}",
                "reconstruction_sse": f"{sse:.17g}",
                "maximum_abs_difference_kg_m2": f"{maximum:.17g}",
                "first_divergence": first or "NONE",
                "state": "PASS" if maximum <= TOL else "FAIL",
            }
        )
    write_rows(output, rows)


def harvard(source: Path, output: Path) -> None:
    rows = []
    with source.open(newline="", encoding="utf-8") as stream:
        for row in csv.DictReader(stream):
            foliar = float(row["mean_foliar_g_c_m2_yr"])
            nonfoliar = float(row["mean_pooled_nonfoliar_g_c_m2_yr"])
            total = float(row["mean_total_g_c_m2_yr"])
            stock = float(row["mean_organic_horizon_c_mass_kg_c_m2"])
            if (
                row["litter_years"] != "2000-2011"
                or row["stock_year"] != "2014"
                or row["stock_use_not"] != "1"
                or int(row["litter_row_count"]) <= 0
                or int(row["stock_replicate_count"]) <= 0
                or any(not math.isfinite(v) or v < 0 for v in (foliar, nonfoliar, total))
                or not math.isfinite(stock)
                or stock <= 0
                or abs(foliar + nonfoliar - total) > 1.0e-7
            ):
                raise ValueError(f"invalid Harvard row {row['plot']}")
            rows.append(
                {
                    "project": row["project"],
                    "plot": row["plot"],
                    "flux_period": row["litter_years"],
                    "stock_year": row["stock_year"],
                    "litter_row_count": row["litter_row_count"],
                    "stock_replicate_count": row["stock_replicate_count"],
                    "stock_use_not": row["stock_use_not"],
                    "foliar_g_c_m2_yr": f"{foliar:.17g}",
                    "pooled_nonfoliar_g_c_m2_yr": f"{nonfoliar:.17g}",
                    "total_g_c_m2_yr": f"{total:.17g}",
                    "organic_horizon_stock_kg_c_m2": f"{stock:.17g}",
                    "pooled_nonfoliar_share": f"{nonfoliar / total:.17g}",
                    "descriptive_flux_stock_ratio_yr-1": f"{(total / 1000.0) / stock:.17g}",
                    "interpretation": "DESCRIPTIVE_NONCONTEMPORANEOUS_POOLED",
                }
            )
    if len(rows) != 28:
        raise ValueError(f"expected 28 Harvard rows, found {len(rows)}")
    write_rows(output, rows)


def write_rows(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


if __name__ == "__main__":
    reconstruct(Path(sys.argv[1]), Path(sys.argv[2]))
    harvard(Path(sys.argv[3]), Path(sys.argv[4]))
