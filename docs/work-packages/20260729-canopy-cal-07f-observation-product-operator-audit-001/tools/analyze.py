#!/usr/bin/env python3
"""Execute the CAL-07F product/operator audit without refitting."""

from __future__ import annotations

import csv
from datetime import date
import hashlib
import math
from pathlib import Path
import statistics


PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"
CAL07 = (
    ROOT
    / "docs/work-packages"
    / "20260728-canopy-cal-07-southern-hemisphere-robustness-001"
)
CAL07D = (
    ROOT
    / "docs/work-packages"
    / "20260729-canopy-cal-07d-transition-chronology-attribution-001"
)
CAL07E = (
    ROOT
    / "docs/work-packages"
    / "20260729-canopy-cal-07e-literature-authority-review-001"
)
DAILY = CAL07 / "inputs/source/bezamahafaly_DB_1000_1day.csv"
ROI = CAL07 / "inputs/source/bezamahafaly_DB_1000_roi.csv"
META = CAL07 / "inputs/source/bezamahafaly_meta.json"
TRANSITIONS = (
    CAL07E
    / "inputs"
    / "bezamahafaly_DB_1000_2024_2025_gcc_mean_gcc_90_transition_subset.csv"
)
ALL_CROSSINGS = CAL07D / "artifacts/all-crossings.csv"
SCENARIOS = CAL07D / "artifacts/scenario-event-screen.csv"
PRODUCTS = ("gcc_mean", "gcc_90")
LEVELS = ("10", "25", "50")
MEMBERS = tuple(f"GSI-{value}" for value in ())
MISSING_PENALTY_DAYS = 183.0


def read_csv(path: Path, comments: bool = False) -> list[dict[str, str]]:
    if comments:
        lines = [
            line
            for line in path.read_text(encoding="utf-8").splitlines()
            if not line.startswith("#")
        ]
        return list(csv.DictReader(lines))
    with path.open(encoding="utf-8", newline="") as stream:
        return list(csv.DictReader(stream))


def write_csv(
    path: Path, fields: tuple[str, ...], rows: list[dict[str, object]]
) -> None:
    with path.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def ordinal(day: str) -> float:
    return float(date.fromisoformat(day).toordinal())


def iso_from_ordinal(value: float) -> str:
    return date.fromordinal(round(value)).isoformat()


def median_or_blank(values: list[float]) -> float | str:
    return statistics.median(values) if values else ""


def quantile(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    position = (len(ordered) - 1) * fraction
    lower = math.floor(position)
    upper = math.ceil(position)
    return ordered[lower] + (position - lower) * (ordered[upper] - ordered[lower])


def rank(values: dict[str, float]) -> dict[str, int]:
    ordered = sorted(values, key=lambda member: (values[member], member))
    return {member: index + 1 for index, member in enumerate(ordered)}


def spearman(rank_a: dict[str, int], rank_b: dict[str, int]) -> float:
    members = sorted(rank_a)
    n = len(members)
    squared = sum((rank_a[m] - rank_b[m]) ** 2 for m in members)
    return 1.0 - (6.0 * squared / (n * (n * n - 1)))


def source_transitions() -> list[dict[str, object]]:
    retained = read_csv(TRANSITIONS)
    results: list[dict[str, object]] = []
    for row in retained:
        product = row["gcc_value"]
        for level in LEVELS:
            observed = row[f"transition_{level}"]
            ci = sorted(
                (
                    row[f"transition_{level}_lower_ci"],
                    row[f"transition_{level}_upper_ci"],
                )
            )
            results.append(
                {
                    "product": product,
                    "event_id": f"{observed[:4]}-{row['direction']}",
                    "year": int(observed[:4]),
                    "direction": row["direction"],
                    "source_level": f"{int(level) / 100:.2f}",
                    "observed_date": observed,
                    "observed_ordinal": ordinal(observed),
                    "ci_start": ci[0],
                    "ci_end": ci[1],
                    "ci_start_ordinal": ordinal(ci[0]),
                    "ci_end_ordinal": ordinal(ci[1]),
                    "gcc_threshold": float(row[f"threshold_{level}"]),
                    "min_gcc": float(row["min_gcc"]),
                    "max_gcc": float(row["max_gcc"]),
                }
            )
    return sorted(
        results,
        key=lambda row: (
            str(row["product"]),
            int(row["year"]),
            str(row["direction"]),
            float(row["source_level"]),
        ),
    )


def daily_curves() -> list[dict[str, object]]:
    retained = read_csv(DAILY, comments=True)
    output: list[dict[str, object]] = []
    for row in retained:
        year = int(row["year"])
        if year not in {2024, 2025}:
            continue
        output.append(
            {
                "date": row["date"],
                "year": year,
                "doy": int(row["doy"]),
                "image_count": row["image_count"],
                "smooth_gcc_mean": float(row["smooth_gcc_mean"]),
                "smooth_ci_gcc_mean": float(row["smooth_ci_gcc_mean"]),
                "smooth_gcc_90": float(row["smooth_gcc_90"]),
                "smooth_ci_gcc_90": float(row["smooth_ci_gcc_90"]),
                "outlierflag_gcc_mean": row["outlierflag_gcc_mean"],
                "outlierflag_gcc_90": row["outlierflag_gcc_90"],
                "int_flag": row["int_flag"],
            }
        )
    return output


def curve_crossings(
    curve_rows: list[dict[str, object]],
    product: str,
    year: int,
    direction: str,
    threshold: float,
) -> list[float]:
    column = "smooth_gcc_mean" if product == "gcc_mean" else "smooth_gcc_90"
    year_rows = [row for row in curve_rows if int(row["year"]) == year]
    found: list[float] = []
    for previous, current in zip(year_rows, year_rows[1:]):
        old = float(previous[column])
        new = float(current[column])
        matches = (
            direction == "rising" and old < threshold <= new
        ) or (
            direction == "falling" and old >= threshold > new
        )
        if not matches or new == old:
            continue
        fraction = (threshold - old) / (new - old)
        found.append(ordinal(str(previous["date"])) + fraction)
    return found


def audit_source_curves(
    transitions: list[dict[str, object]],
    curves: list[dict[str, object]],
) -> None:
    for row in transitions:
        crossings = curve_crossings(
            curves,
            str(row["product"]),
            int(row["year"]),
            str(row["direction"]),
            float(row["gcc_threshold"]),
        )
        if crossings:
            selected = min(
                crossings,
                key=lambda value: abs(value - float(row["observed_ordinal"])),
            )
            row["daily_crossing_date"] = iso_from_ordinal(selected)
            row["daily_crossing_ordinal"] = selected
            row["daily_minus_transition_days"] = (
                selected - float(row["observed_ordinal"])
            )
            row["daily_same_direction_crossing_count"] = len(crossings)
        else:
            row["daily_crossing_date"] = ""
            row["daily_crossing_ordinal"] = ""
            row["daily_minus_transition_days"] = ""
            row["daily_same_direction_crossing_count"] = 0


def crossing_index() -> tuple[
    dict[tuple[str, str, str, str, str], list[float]], tuple[str, ...]
]:
    rows = read_csv(ALL_CROSSINGS)
    index: dict[tuple[str, str, str, str, str], list[float]] = {}
    members: set[str] = set()
    for row in rows:
        if row["scenario"] != "BASE":
            continue
        member = row["member_or_default"]
        members.add(member)
        year = row["event_id"][:4]
        if row["crossing_date"][:4] != year:
            continue
        key = (
            member,
            row["event_id"],
            row["operator"],
            f"{float(row['model_level']):.2f}",
            row["direction"],
        )
        index.setdefault(key, []).append(float(row["crossing_ordinal"]))
    return index, tuple(sorted(members))


def seasonal_windows(
    transitions: list[dict[str, object]],
) -> dict[tuple[str, int, str], tuple[float, float]]:
    windows: dict[tuple[str, int, str], tuple[float, float]] = {}
    for product in PRODUCTS:
        for year in (2024, 2025):
            falling_t10 = next(
                float(row["observed_ordinal"])
                for row in transitions
                if row["product"] == product
                and row["year"] == year
                and row["direction"] == "falling"
                and row["source_level"] == "0.10"
            )
            rising_t10 = next(
                float(row["observed_ordinal"])
                for row in transitions
                if row["product"] == product
                and row["year"] == year
                and row["direction"] == "rising"
                and row["source_level"] == "0.10"
            )
            boundary = 0.5 * (falling_t10 + rising_t10)
            windows[(product, year, "falling")] = (
                ordinal(f"{year}-01-01"),
                boundary,
            )
            windows[(product, year, "rising")] = (
                boundary,
                ordinal(f"{year}-12-31") + 0.999999,
            )
    return windows


def compare_members(
    transitions: list[dict[str, object]],
    operator: str,
    levels: tuple[str, ...],
) -> tuple[list[dict[str, object]], tuple[str, ...]]:
    index, members = crossing_index()
    windows = seasonal_windows(transitions)
    output: list[dict[str, object]] = []
    for source in transitions:
        source_level = str(source["source_level"])
        if source_level not in levels:
            continue
        model_level = "0.50" if operator == "ABSOLUTE_0_5" else source_level
        for member in members:
            key = (
                member,
                str(source["event_id"]),
                operator,
                model_level,
                str(source["direction"]),
            )
            window_start, window_end = windows[
                (
                    str(source["product"]),
                    int(source["year"]),
                    str(source["direction"]),
                )
            ]
            candidates = [
                value
                for value in index.get(key, [])
                if window_start <= value <= window_end
            ]
            if candidates:
                selected = min(
                    candidates,
                    key=lambda value: abs(
                        value - float(source["observed_ordinal"])
                    ),
                )
                residual = selected - float(source["observed_ordinal"])
                in_ci = (
                    float(source["ci_start_ordinal"])
                    <= selected
                    <= float(source["ci_end_ordinal"])
                )
                selected_date: object = iso_from_ordinal(selected)
                selected_ordinal: object = selected
            else:
                residual = ""
                in_ci = False
                selected_date = ""
                selected_ordinal = ""
            output.append(
                {
                    "product": source["product"],
                    "member": member,
                    "event_id": source["event_id"],
                    "year": source["year"],
                    "direction": source["direction"],
                    "operator": operator,
                    "source_level": source_level,
                    "observed_date": source["observed_date"],
                    "ci_start": source["ci_start"],
                    "ci_end": source["ci_end"],
                    "window_start": iso_from_ordinal(window_start),
                    "window_end": iso_from_ordinal(window_end),
                    "candidate_count": len(candidates),
                    "selected_crossing_date": selected_date,
                    "selected_crossing_ordinal": selected_ordinal,
                    "residual_days": residual,
                    "inside_observed_ci": in_ci,
                }
            )
    return output, members


def member_summaries(
    comparisons: list[dict[str, object]], members: tuple[str, ...]
) -> list[dict[str, object]]:
    output: list[dict[str, object]] = []
    for product in PRODUCTS:
        for member in members:
            rows = [
                row
                for row in comparisons
                if row["product"] == product and row["member"] == member
            ]
            residuals = [
                float(row["residual_days"])
                for row in rows
                if row["residual_days"] != ""
            ]
            rising = [
                float(row["residual_days"])
                for row in rows
                if row["residual_days"] != "" and row["direction"] == "rising"
            ]
            falling = [
                float(row["residual_days"])
                for row in rows
                if row["residual_days"] != "" and row["direction"] == "falling"
            ]
            penalized = [
                abs(float(row["residual_days"]))
                if row["residual_days"] != ""
                else MISSING_PENALTY_DAYS
                for row in rows
            ]
            complete = len(residuals) == 12
            interval_hits = sum(bool(row["inside_observed_ci"]) for row in rows)
            median_abs = median_or_blank([abs(value) for value in residuals])
            rising_median = median_or_blank(rising)
            falling_median = median_or_blank(falling)
            output.append(
                {
                    "product": product,
                    "member": member,
                    "comparison_count": len(rows),
                    "crossing_count": len(residuals),
                    "missing_count": 12 - len(residuals),
                    "complete_12": complete,
                    "interval_hit_count": interval_hits,
                    "median_absolute_residual_days": median_abs,
                    "rising_median_signed_residual_days": rising_median,
                    "falling_median_signed_residual_days": falling_median,
                    "penalized_mean_absolute_residual_days": statistics.fmean(
                        penalized
                    ),
                    "crossing_sufficiency_pass": complete,
                    "uncertainty_fit_pass": (
                        complete
                        and interval_hits >= 8
                        and float(median_abs) <= 21.0
                    ),
                    "direction_coherence_pass": (
                        complete
                        and rising_median != ""
                        and falling_median != ""
                        and abs(float(rising_median)) <= 21.0
                        and abs(float(falling_median)) <= 21.0
                    ),
                }
            )
    return output


def rank_comparison(
    summaries: list[dict[str, object]], members: tuple[str, ...]
) -> tuple[list[dict[str, object]], float, float]:
    scores = {
        product: {
            member: float(
                next(
                    row["penalized_mean_absolute_residual_days"]
                    for row in summaries
                    if row["product"] == product and row["member"] == member
                )
            )
            for member in members
        }
        for product in PRODUCTS
    }
    ranks = {product: rank(scores[product]) for product in PRODUCTS}
    correlation = spearman(ranks["gcc_mean"], ranks["gcc_90"])
    top_count = math.ceil(len(members) / 4)
    top_sets = {
        product: {
            member
            for member in members
            if ranks[product][member] <= top_count
        }
        for product in PRODUCTS
    }
    overlap = len(top_sets["gcc_mean"] & top_sets["gcc_90"]) / top_count
    output = [
        {
            "member": member,
            "gcc_mean_score_days": scores["gcc_mean"][member],
            "gcc_mean_rank": ranks["gcc_mean"][member],
            "gcc_90_score_days": scores["gcc_90"][member],
            "gcc_90_rank": ranks["gcc_90"][member],
            "rank_difference": ranks["gcc_90"][member]
            - ranks["gcc_mean"][member],
            "in_both_top_quartiles": (
                member in top_sets["gcc_mean"] & top_sets["gcc_90"]
            ),
        }
        for member in members
    ]
    return output, correlation, overlap


def parameter_plausibility() -> tuple[bool, str]:
    rows = read_csv(SCENARIOS)
    usable: list[str] = []
    for scenario in sorted({row["scenario"] for row in rows}):
        if scenario in {"BASE", "SC_PLANT_GENERALIZED_DEFAULT"}:
            continue
        selected = [
            row
            for row in rows
            if row["scenario"] == scenario
            and row["operator"] == "EVENT_YEAR_RELATIVE"
            and row["residual_days"] != ""
        ]
        if len(selected) != 148:
            continue
        direction_medians = {
            direction: statistics.median(
                float(row["residual_days"])
                for row in selected
                if row["direction"] == direction
            )
            for direction in ("rising", "falling")
        }
        if all(abs(value) <= 21.0 for value in direction_medians.values()):
            usable.append(scenario)
    return bool(usable), "|".join(usable) if usable else "none"


def decision_rows(
    summaries: list[dict[str, object]],
    rank_correlation: float,
    top_overlap: float,
    parameter_pass: bool,
    parameter_scenarios: str,
) -> list[dict[str, object]]:
    by_member = {
        member: {
            product: next(
                row
                for row in summaries
                if row["member"] == member and row["product"] == product
            )
            for product in PRODUCTS
        }
        for member in sorted({str(row["member"]) for row in summaries})
    }
    both_complete = [
        member
        for member, products in by_member.items()
        if all(bool(products[p]["crossing_sufficiency_pass"]) for p in PRODUCTS)
    ]
    both_uncertainty = [
        member
        for member, products in by_member.items()
        if all(bool(products[p]["uncertainty_fit_pass"]) for p in PRODUCTS)
    ]
    both_direction = [
        member
        for member, products in by_member.items()
        if all(bool(products[p]["direction_coherence_pass"]) for p in PRODUCTS)
    ]
    joint = sorted(
        set(both_complete) & set(both_uncertainty) & set(both_direction)
    )
    operator_pass = not joint or (
        rank_correlation >= 0.90 and top_overlap >= 0.75
    )
    empirical_role_pass = True
    rows = [
        {
            "criterion": "OPERATOR_INDEPENDENCE",
            "status": "PASS" if operator_pass else "FAIL",
            "metric": (
                f"spearman={rank_correlation:.6f};"
                f"top_quartile_overlap={top_overlap:.3f};"
                f"joint_candidates={len(joint)}"
            ),
            "reason": (
                "same no-calibration disposition in both products; "
                "no product selected by fit"
                if operator_pass
                else "candidate recommendation is product-sensitive"
            ),
        },
        {
            "criterion": "CROSSING_SUFFICIENCY",
            "status": "PASS" if both_complete else "FAIL",
            "metric": f"members_complete_in_both={len(both_complete)}",
            "reason": "requires all 12 same-year crossings in both products",
        },
        {
            "criterion": "UNCERTAINTY_FIT",
            "status": "PASS" if both_uncertainty else "FAIL",
            "metric": f"members_passing_in_both={len(both_uncertainty)}",
            "reason": "requires >=8/12 CI hits and median absolute residual <=21 d",
        },
        {
            "criterion": "DIRECTION_COHERENCE",
            "status": "PASS" if both_direction else "FAIL",
            "metric": f"members_passing_in_both={len(both_direction)}",
            "reason": "requires rising and falling median signed residual within +/-21 d",
        },
        {
            "criterion": "PARAMETER_PLAUSIBILITY",
            "status": "PASS" if parameter_pass else "FAIL",
            "metric": f"qualifying_cal07d_scenarios={parameter_scenarios}",
            "reason": "requires 148/148 matches and both direction medians within +/-21 d",
        },
        {
            "criterion": "EMPIRICAL_ROLE",
            "status": "PASS" if empirical_role_pass else "FAIL",
            "metric": "one fit year plus one diagnostic holdout is mechanically possible",
            "reason": "holdout would be internal only, not external validation",
        },
    ]
    recommendation = all(row["status"] == "PASS" for row in rows)
    rows.append(
        {
            "criterion": "CALIBRATION_ROUND",
            "status": "RECOMMEND" if recommendation else "DO_NOT_RECOMMEND",
            "metric": f"passed_required={sum(r['status'] == 'PASS' for r in rows)}/6",
            "reason": (
                "all prospective criteria pass"
                if recommendation
                else "one or more prospective criteria fail"
            ),
        }
    )
    return rows


def main() -> None:
    ART.mkdir(parents=True, exist_ok=True)
    dependencies = [
        (DAILY, "provisional daily GCC curves"),
        (ROI, "ROI identity and change interval"),
        (META, "site and camera metadata"),
        (TRANSITIONS, "provisional transition rows and confidence intervals"),
        (ALL_CROSSINGS, "validated CAL-07D crossing inventory"),
        (SCENARIOS, "validated CAL-07D counterfactual event screen"),
    ]
    write_csv(
        ART / "dependency-manifest.csv",
        ("path", "sha256", "bytes", "role"),
        [
            {
                "path": str(path.relative_to(ROOT)),
                "sha256": sha256(path),
                "bytes": path.stat().st_size,
                "role": role,
            }
            for path, role in dependencies
        ],
    )

    transitions = source_transitions()
    curves = daily_curves()
    audit_source_curves(transitions, curves)
    write_csv(
        ART / "daily-product-curves.csv",
        (
            "date",
            "year",
            "doy",
            "image_count",
            "smooth_gcc_mean",
            "smooth_ci_gcc_mean",
            "smooth_gcc_90",
            "smooth_ci_gcc_90",
            "outlierflag_gcc_mean",
            "outlierflag_gcc_90",
            "int_flag",
        ),
        curves,
    )
    write_csv(
        ART / "source-transition-audit.csv",
        (
            "product",
            "event_id",
            "year",
            "direction",
            "source_level",
            "observed_date",
            "observed_ordinal",
            "ci_start",
            "ci_end",
            "ci_start_ordinal",
            "ci_end_ordinal",
            "gcc_threshold",
            "min_gcc",
            "max_gcc",
            "daily_crossing_date",
            "daily_crossing_ordinal",
            "daily_minus_transition_days",
            "daily_same_direction_crossing_count",
        ),
        transitions,
    )

    comparisons, members = compare_members(
        transitions, "EVENT_YEAR_RELATIVE", ("0.10", "0.25", "0.50")
    )
    write_csv(
        ART / "member-comparisons.csv",
        (
            "product",
            "member",
            "event_id",
            "year",
            "direction",
            "operator",
            "source_level",
            "observed_date",
            "ci_start",
            "ci_end",
            "window_start",
            "window_end",
            "candidate_count",
            "selected_crossing_date",
            "selected_crossing_ordinal",
            "residual_days",
            "inside_observed_ci",
        ),
        comparisons,
    )
    absolute, _ = compare_members(transitions, "ABSOLUTE_0_5", ("0.50",))
    write_csv(
        ART / "absolute-comparisons.csv",
        (
            "product",
            "member",
            "event_id",
            "year",
            "direction",
            "operator",
            "source_level",
            "observed_date",
            "ci_start",
            "ci_end",
            "window_start",
            "window_end",
            "candidate_count",
            "selected_crossing_date",
            "selected_crossing_ordinal",
            "residual_days",
            "inside_observed_ci",
        ),
        absolute,
    )

    summaries = member_summaries(comparisons, members)
    write_csv(
        ART / "member-summary.csv",
        (
            "product",
            "member",
            "comparison_count",
            "crossing_count",
            "missing_count",
            "complete_12",
            "interval_hit_count",
            "median_absolute_residual_days",
            "rising_median_signed_residual_days",
            "falling_median_signed_residual_days",
            "penalized_mean_absolute_residual_days",
            "crossing_sufficiency_pass",
            "uncertainty_fit_pass",
            "direction_coherence_pass",
        ),
        summaries,
    )
    ranks, correlation, overlap = rank_comparison(summaries, members)
    write_csv(
        ART / "product-rank-comparison.csv",
        (
            "member",
            "gcc_mean_score_days",
            "gcc_mean_rank",
            "gcc_90_score_days",
            "gcc_90_rank",
            "rank_difference",
            "in_both_top_quartiles",
        ),
        ranks,
    )
    parameter_pass, parameter_scenarios = parameter_plausibility()
    decisions = decision_rows(
        summaries, correlation, overlap, parameter_pass, parameter_scenarios
    )
    write_csv(
        ART / "decision-screen.csv",
        ("criterion", "status", "metric", "reason"),
        decisions,
    )

    print(
        "CAL-07F analysis complete: "
        f"{len(curves)} daily rows, {len(transitions)} source transitions, "
        f"{len(comparisons)} relative comparisons, {len(absolute)} absolute "
        f"comparisons, {len(members)} members, "
        f"decision={decisions[-1]['status']}"
    )


if __name__ == "__main__":
    main()
