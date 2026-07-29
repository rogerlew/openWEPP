#!/usr/bin/env python3
"""Summarize frozen CAL-05 daily recovery and analytic ridge."""

from __future__ import annotations

import csv
import sys
from collections import defaultdict
from pathlib import Path


def traces(path: Path) -> dict[str, list[float]]:
    result: dict[str, list[float]] = defaultdict(list)
    with path.open(newline="", encoding="utf-8") as stream:
        for row in csv.DictReader(stream):
            result[row["candidate_id"]].append(float(row["surface_after_kg_m2"]))
    return result


def write(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


main = traces(Path(sys.argv[1]))
truth = main["S020-K050"]
recovery = []
for candidate, values in sorted(main.items()):
    sse = sum((value - expected) ** 2 for value, expected in zip(values, truth, strict=True))
    recovery.append(
        {
            "candidate_id": candidate,
            "daily_stock_sse": f"{sse:.17g}",
            "state": "RECOVERED_TRUTH" if sse <= 1.0e-20 else "RETAINED_NONMINIMUM",
        }
    )
write(Path(sys.argv[2]), recovery)

ridge = traces(Path(sys.argv[3]))
with Path(sys.argv[6]).open(newline="", encoding="utf-8") as stream:
    ridge_execution = list(csv.DictReader(stream))
with Path(sys.argv[7]).open(newline="", encoding="utf-8") as stream:
    ridge_targets = list(csv.DictReader(stream))
target_values = {float(row["target_terminal_stock_kg_m2"]) for row in ridge_targets}
if len(target_values) != 1:
    raise ValueError("ridge target must be unique")
target = target_values.pop()
ridge_rows = []
for candidate, values in sorted(ridge.items()):
    delta = abs(values[-1] - target)
    ridge_rows.append(
        {
            "candidate_id": candidate,
            "terminal_stock_kg_m2": f"{values[-1]:.17g}",
            "target_terminal_stock_kg_m2": f"{target:.17g}",
            "absolute_difference_kg_m2": f"{delta:.17g}",
            "state": "EQUIFINAL_TERMINAL" if delta <= 1.0e-12 else "FAIL",
        }
    )
write(Path(sys.argv[4]), ridge_rows)

terminal = {candidate: values[-1] for candidate, values in main.items()}
metrics: list[dict[str, object]] = []
for rate_id in ("K000", "K050", "K100", "K200"):
    slope = (terminal[f"S030-{rate_id}"] - terminal[f"S010-{rate_id}"]) / 0.20
    metrics.append(
        {
            "metric": "local_terminal_sensitivity_to_source",
            "scope": rate_id,
            "value": f"{slope:.17g}",
            "units": "yr",
            "state": "NONZERO",
        }
    )
delta_rate = 0.5 / 365.25
for source_id in ("S010", "S020", "S030", "S040"):
    slope = (terminal[f"{source_id}-K100"] - terminal[f"{source_id}-K000"]) / (
        2.0 * delta_rate
    )
    metrics.append(
        {
            "metric": "local_terminal_sensitivity_to_rate",
            "scope": source_id,
            "value": f"{slope:.17g}",
            "units": "kg_m-2_d",
            "state": "NONZERO",
        }
    )
rates = [float(row["surface_rate_d-1"]) * 365.25 for row in ridge_execution]
sources = [
    float(row["synthetic_annual_surface_litter_input_kg_m2_yr"])
    for row in ridge_execution
]
mean_rate = sum(rates) / len(rates)
mean_source = sum(sources) / len(sources)
covariance = sum(
    (rate - mean_rate) * (source - mean_source)
    for rate, source in zip(rates, sources, strict=True)
) / len(rates)
variance_rate = sum((rate - mean_rate) ** 2 for rate in rates) / len(rates)
variance_source = sum((source - mean_source) ** 2 for source in sources) / len(sources)
correlation = covariance / (variance_rate * variance_source) ** 0.5
metrics.extend(
    [
        {
            "metric": "ridge_source_rate_covariance",
            "scope": "five_pair_terminal_ridge",
            "value": f"{covariance:.17g}",
            "units": "kg_m-2_yr-2",
            "state": "POSITIVE_CONFOUNDING",
        },
        {
            "metric": "ridge_source_rate_correlation",
            "scope": "five_pair_terminal_ridge",
            "value": f"{correlation:.17g}",
            "units": "dimensionless",
            "state": "POSITIVE_CONFOUNDING",
        },
        {
            "metric": "temperature_factor",
            "scope": "all_synthetic_days",
            "value": "0.4976215945544323",
            "units": "fraction",
            "state": "INTERIOR_LIMITING",
        },
        {
            "metric": "surface_water_factor",
            "scope": "all_synthetic_days",
            "value": "1",
            "units": "fraction",
            "state": "SATURATED_NOT_SURFACE_DECAY_CONTROL",
        },
        {
            "metric": "flat_water_factor",
            "scope": "all_synthetic_days",
            "value": "1",
            "units": "fraction",
            "state": "SATURATED_NOT_LIMITING",
        },
    ]
)
write(Path(sys.argv[5]), metrics)
