#!/usr/bin/env python3
"""Reconstruct the native-forest canopy-phenology assurance summary."""

from __future__ import annotations

import argparse
import csv
import json
import math
from pathlib import Path
from typing import Any


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as stream:
        return list(csv.DictReader(stream))


def number(identifier: str, value: int | float, unit: str, precision: str) -> dict[str, Any]:
    if isinstance(value, float) and not math.isfinite(value):
        raise ValueError(f"{identifier} is not finite")
    return {"id": identifier, "value": value, "unit_id": unit, "precision": precision}


def build(args: argparse.Namespace) -> dict[str, Any]:
    accepted = rows(args.accepted_ensemble)
    configs = rows(args.candidate_configurations)
    harvard = rows(args.harvard_holdout)
    ridge = rows(args.litter_ridge)
    gradient = rows(args.gradient_summary)
    beza = rows(args.beza_members)
    elliot = rows(args.elliot_comparison)

    accepted_ids = {row["candidate_id"] for row in accepted}
    accepted_configs = [row for row in configs if row["candidate_id"] in accepted_ids]
    if len(configs) != 9_261 or len(accepted_configs) != len(accepted):
        raise ValueError("the complete configuration grid and accepted configuration join must close")
    harvard_rows = [row for row in harvard if row["candidate_id"] in accepted_ids]
    if len(accepted) != 37 or len(harvard_rows) != 37:
        raise ValueError("the frozen accepted ensemble and Harvard evaluation must each have 37 members")

    harvard_scores = [float(row["aggregate_score"]) for row in harvard_rows]
    harvard_coverages = [float(row["interval_coverage_fraction"]) for row in harvard_rows]
    ridge_differences = [float(row["absolute_difference_kg_m2"]) for row in ridge]

    forest_gradient = [row for row in gradient if row["stratum"] != "open"]
    open_controls = [row for row in gradient if row["stratum"] == "open"]
    expected_gradient = {
        ("marcell", "conifer"),
        ("marcell", "deciduous"),
        ("marcell", "mixed"),
        ("harvard", "deciduous"),
        ("harvard", "mixed"),
        ("hubbard_brook", "deciduous"),
        ("hubbard_brook", "mixed"),
    }
    observed_gradient = {(row["site"], row["stratum"]) for row in forest_gradient}
    if observed_gradient != expected_gradient:
        raise ValueError("unexpected CAL-06 site/stratum inventory")
    if any(int(row["member_count"]) != 37 for row in forest_gradient):
        raise ValueError("every CAL-06 forest gradient must retain all 37 members")
    if len(open_controls) != 2 or any(int(row["member_count"]) != 1 for row in open_controls):
        raise ValueError("CAL-06 must retain exactly two one-member open controls")
    by_lane = {(row["site"], row["stratum"]): row for row in forest_gradient}
    ordering_pairs = [
        (("marcell", "deciduous"), ("marcell", "mixed")),
        (("marcell", "mixed"), ("marcell", "conifer")),
        (("harvard", "deciduous"), ("harvard", "mixed")),
        (("hubbard_brook", "deciduous"), ("hubbard_brook", "mixed")),
    ]
    for lower, upper in ordering_pairs:
        lower_max = float(by_lane[lower]["winter_cover_mean_max"])
        upper_min = float(by_lane[upper]["winter_cover_mean_min"])
        if not lower_max < upper_min:
            raise ValueError(f"winter canopy ordering is not separated: {lower} !< {upper}")
    ordering_member_count = min(int(row["member_count"]) for row in forest_gradient)
    gradient_run_count = sum(int(row["member_count"]) for row in gradient)

    joint = {}
    for row in beza:
        joint[(row["product"], row["member"])] = row
    best_mean = joint[("gcc_mean", "GSI-4831")]
    best_gcc90 = joint[("gcc_90", "GSI-4831")]
    complete_joint = sum(
        1
        for member in accepted_ids
        if joint[("gcc_mean", member)]["complete_12"] == "True"
        and joint[("gcc_90", member)]["complete_12"] == "True"
    )

    contradicted_elliot = sum(row["classification"] == "CONTRADICTED" for row in elliot)
    compared_elliot = len(elliot)

    values = [
        number("searched_configuration_count", len(configs), "member_count", "exact retained configuration-row count"),
        number("accepted_member_count", len(accepted), "member_count", "exact retained integer"),
        number("accepted_double_boundary_count", sum(row["boundary_flags"] == "DOUBLE_BOUNDARY" for row in accepted), "member_count", "count reconstructed from retained accepted flags"),
        number("accepted_upper_boundary_count", sum(row["boundary_flags"] == "UPPER_SUPPORT_BOUNDARY" for row in accepted), "member_count", "count reconstructed from retained accepted flags"),
        number("harvard_score_min_days", min(harvard_scores), "day", "minimum retained aggregate score"),
        number("harvard_score_max_days", max(harvard_scores), "day", "maximum retained aggregate score"),
        number("harvard_zero_coverage_members", sum(value == 0.0 for value in harvard_coverages), "member_count", "count reconstructed from retained coverage fractions"),
        number("harvard_max_coverage_percent", 100.0 * max(harvard_coverages), "percent", "scaled maximum retained fraction"),
        number("litter_ridge_pair_count", len(ridge), "pair_count", "exact retained row count"),
        number("litter_ridge_target_stock", float(ridge[0]["target_terminal_stock_kg_m2"]), "kg_m2", "common retained target"),
        number("litter_ridge_max_difference", max(ridge_differences), "kg_m2", "maximum absolute retained difference"),
        number("gradient_run_count", gradient_run_count, "run_count", "sum of retained site/stratum member counts"),
        number("gradient_ordering_member_count", ordering_member_count, "member_count", "minimum retained member count after explicit winter-cover separation checks"),
        number("beza_complete_member_count", complete_joint, "member_count", "members complete for all 12 transitions under both products"),
        number("beza_mean_interval_hits", int(best_mean["interval_hit_count"]), "transition_count", "retained best-member count"),
        number("beza_gcc90_interval_hits", int(best_gcc90["interval_hit_count"]), "transition_count", "retained best-member count"),
        number("beza_mean_penalized_error_days", float(best_mean["penalized_mean_absolute_residual_days"]), "day", "retained penalized mean absolute residual"),
        number("beza_gcc90_penalized_error_days", float(best_gcc90["penalized_mean_absolute_residual_days"]), "day", "retained penalized mean absolute residual"),
        number("elliot_compared_target_count", compared_elliot, "target_count", "retained comparison-row count"),
        number("elliot_contradicted_target_count", contradicted_elliot, "target_count", "count of retained CONTRADICTED rows"),
    ]
    coefficient_units = {
        "minimum_temperature_inactive_c": "degC",
        "minimum_temperature_unconstrained_c": "degC",
        "vapor_pressure_deficit_unconstrained_pa": "Pa",
        "vapor_pressure_deficit_inactive_pa": "Pa",
        "photoperiod_inactive_hours": "hour",
        "photoperiod_unconstrained_hours": "hour",
    }
    for field, unit in coefficient_units.items():
        observed = [float(row[field]) for row in accepted_configs]
        values.extend(
            [
                number(f"{field}_min", min(observed), unit, "minimum among 37 retained correlated configurations"),
                number(f"{field}_max", max(observed), unit, "maximum among 37 retained correlated configurations"),
            ]
        )
    return {
        "schema_version": 1,
        "result_id": "CANOPY-RESULT-SYNTHESIS",
        "values": values,
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--accepted-ensemble", type=Path, required=True)
    parser.add_argument("--candidate-configurations", type=Path, required=True)
    parser.add_argument("--harvard-holdout", type=Path, required=True)
    parser.add_argument("--litter-ridge", type=Path, required=True)
    parser.add_argument("--gradient-summary", type=Path, required=True)
    parser.add_argument("--beza-members", type=Path, required=True)
    parser.add_argument("--elliot-comparison", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    payload = json.dumps(build(args), indent=2, sort_keys=False) + "\n"
    if args.output is None:
        print(payload, end="")
    else:
        args.output.write_text(payload, encoding="utf-8")


if __name__ == "__main__":
    main()
