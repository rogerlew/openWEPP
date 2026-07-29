#!/usr/bin/env python3
"""Independently validate CAL-07F retained evidence."""

from __future__ import annotations

import csv
from datetime import date, timedelta
import hashlib
import math
from pathlib import Path
import statistics
import sys
import xml.etree.ElementTree as ET


PKG = Path(__file__).resolve().parents[1]
ROOT = PKG.parents[2]
ART = PKG / "artifacts"
PRODUCTS = {"gcc_mean", "gcc_90"}
SCENARIOS = (
    ROOT
    / "docs/work-packages"
    / "20260729-canopy-cal-07d-transition-chronology-attribution-001"
    / "artifacts/scenario-event-screen.csv"
)


def rows(name: str) -> list[dict[str, str]]:
    with (ART / name).open(encoding="utf-8", newline="") as stream:
        result = list(csv.DictReader(stream))
    if not result:
        raise ValueError(f"{name}: no rows")
    if any(None in row for row in result):
        raise ValueError(f"{name}: malformed row")
    return result


def boolean(value: str) -> bool:
    if value not in {"True", "False"}:
        raise ValueError(f"invalid boolean: {value}")
    return value == "True"


def require_unique(
    name: str, data: list[dict[str, str]], fields: tuple[str, ...]
) -> None:
    keys = [tuple(row[field] for field in fields) for row in data]
    if len(keys) != len(set(keys)):
        raise ValueError(f"{name}: duplicate key {fields}")


def validate_dependencies() -> None:
    manifest = rows("dependency-manifest.csv")
    if len(manifest) != 6:
        raise ValueError("dependency manifest must have six rows")
    for row in manifest:
        path = ROOT / row["path"]
        if not path.is_file():
            raise ValueError(f"missing dependency: {row['path']}")
        if path.stat().st_size != int(row["bytes"]):
            raise ValueError(f"dependency size mismatch: {row['path']}")
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != row["sha256"]:
            raise ValueError(f"dependency hash mismatch: {row['path']}")


def validate_curves() -> None:
    curves = rows("daily-product-curves.csv")
    if len(curves) != 731:
        raise ValueError("daily curves must contain 731 rows")
    days = [date.fromisoformat(row["date"]) for row in curves]
    if days[0] != date(2024, 1, 1) or days[-1] != date(2025, 12, 31):
        raise ValueError("daily curve endpoints mismatch")
    if any(current != previous + timedelta(days=1) for previous, current in zip(days, days[1:])):
        raise ValueError("daily curves are not consecutive")
    if any(
        row["outlierflag_gcc_mean"] != "0"
        or row["outlierflag_gcc_90"] != "0"
        for row in curves
    ):
        raise ValueError("unexpected daily GCC outlier flag")
    if sum(row["int_flag"] == "1" for row in curves) != 21:
        raise ValueError("interpolation-flag count mismatch")
    for row in curves:
        for field in (
            "smooth_gcc_mean",
            "smooth_ci_gcc_mean",
            "smooth_gcc_90",
            "smooth_ci_gcc_90",
        ):
            if not math.isfinite(float(row[field])):
                raise ValueError(f"nonfinite {field}")


def validate_source_transitions() -> None:
    source = rows("source-transition-audit.csv")
    if len(source) != 24:
        raise ValueError("source transition audit must contain 24 rows")
    require_unique(
        "source-transition-audit.csv",
        source,
        ("product", "event_id", "source_level"),
    )
    if {row["product"] for row in source} != PRODUCTS:
        raise ValueError("source products mismatch")
    exact = 0
    for row in source:
        observed = float(row["observed_ordinal"])
        if observed != date.fromisoformat(row["observed_date"]).toordinal():
            raise ValueError("observed ordinal mismatch")
        start = date.fromisoformat(row["ci_start"]).toordinal()
        end = date.fromisoformat(row["ci_end"]).toordinal()
        if not start <= observed <= end:
            raise ValueError("observed date outside reported CI")
        daily = float(row["daily_crossing_ordinal"])
        residual = daily - observed
        if abs(residual - float(row["daily_minus_transition_days"])) > 1e-9:
            raise ValueError("daily transition residual mismatch")
        if abs(residual) > 5.0:
            raise ValueError("daily curve crossing more than five days from transition")
        if int(row["daily_same_direction_crossing_count"]) < 1:
            raise ValueError("source transition has no daily curve crossing")
        if abs(residual) < 1e-12:
            exact += 1
        elif not start <= daily <= end:
            raise ValueError("nonexact daily crossing outside reported CI")
    if exact != 23:
        raise ValueError(f"expected 23 exact daily transition dates, found {exact}")


def validate_comparisons() -> tuple[list[dict[str, str]], set[str]]:
    comparisons = rows("member-comparisons.csv")
    if len(comparisons) != 888:
        raise ValueError("relative comparison inventory must contain 888 rows")
    require_unique(
        "member-comparisons.csv",
        comparisons,
        ("product", "member", "event_id", "source_level"),
    )
    members = {row["member"] for row in comparisons}
    if len(members) != 37:
        raise ValueError("relative comparison member inventory mismatch")
    for row in comparisons:
        observed = date.fromisoformat(row["observed_date"]).toordinal()
        start = date.fromisoformat(row["ci_start"]).toordinal()
        end = date.fromisoformat(row["ci_end"]).toordinal()
        if row["selected_crossing_ordinal"]:
            selected = float(row["selected_crossing_ordinal"])
            residual = selected - observed
            if abs(residual - float(row["residual_days"])) > 1e-9:
                raise ValueError("relative residual arithmetic mismatch")
            expected_ci = start <= selected <= end
            if boolean(row["inside_observed_ci"]) != expected_ci:
                raise ValueError("relative CI classification mismatch")
            window_start = date.fromisoformat(row["window_start"]).toordinal()
            window_end = date.fromisoformat(row["window_end"]).toordinal()
            if not window_start - 1 <= selected <= window_end + 1:
                raise ValueError("selected crossing outside reported seasonal window")
            if int(row["candidate_count"]) < 1:
                raise ValueError("selected crossing without candidate")
        else:
            if row["residual_days"] or row["selected_crossing_date"]:
                raise ValueError("partial unmatched comparison")
            if boolean(row["inside_observed_ci"]):
                raise ValueError("unmatched comparison marked inside CI")
            if int(row["candidate_count"]) != 0:
                raise ValueError("unmatched comparison has candidate")
    return comparisons, members


def validate_absolute(members: set[str]) -> None:
    absolute = rows("absolute-comparisons.csv")
    if len(absolute) != 296:
        raise ValueError("absolute comparison inventory must contain 296 rows")
    require_unique(
        "absolute-comparisons.csv",
        absolute,
        ("product", "member", "event_id", "source_level"),
    )
    if {row["member"] for row in absolute} != members:
        raise ValueError("absolute member inventory mismatch")
    for row in absolute:
        if row["selected_crossing_ordinal"]:
            residual = float(row["selected_crossing_ordinal"]) - date.fromisoformat(
                row["observed_date"]
            ).toordinal()
            if abs(residual - float(row["residual_days"])) > 1e-9:
                raise ValueError("absolute residual arithmetic mismatch")


def validate_summaries(
    comparisons: list[dict[str, str]], members: set[str]
) -> list[dict[str, str]]:
    summaries = rows("member-summary.csv")
    if len(summaries) != 74:
        raise ValueError("member summary must contain 74 rows")
    require_unique("member-summary.csv", summaries, ("product", "member"))
    if {row["member"] for row in summaries} != members:
        raise ValueError("summary member inventory mismatch")
    for summary in summaries:
        group = [
            row
            for row in comparisons
            if row["product"] == summary["product"]
            and row["member"] == summary["member"]
        ]
        residuals = [
            float(row["residual_days"]) for row in group if row["residual_days"]
        ]
        if len(group) != 12:
            raise ValueError("member summary group must have 12 comparisons")
        if int(summary["crossing_count"]) != len(residuals):
            raise ValueError("summary crossing count mismatch")
        if int(summary["missing_count"]) != 12 - len(residuals):
            raise ValueError("summary missing count mismatch")
        if boolean(summary["complete_12"]) != (len(residuals) == 12):
            raise ValueError("summary completeness mismatch")
        hits = sum(boolean(row["inside_observed_ci"]) for row in group)
        if int(summary["interval_hit_count"]) != hits:
            raise ValueError("summary interval-hit mismatch")
        if residuals:
            median_abs = statistics.median(abs(value) for value in residuals)
            if abs(
                median_abs - float(summary["median_absolute_residual_days"])
            ) > 1e-9:
                raise ValueError("summary median absolute residual mismatch")
        expected_crossing_pass = len(residuals) == 12
        if boolean(summary["crossing_sufficiency_pass"]) != expected_crossing_pass:
            raise ValueError("crossing-sufficiency flag is not biconditional")
        expected_uncertainty_pass = (
            len(residuals) == 12 and hits >= 8 and median_abs <= 21
        )
        if boolean(summary["uncertainty_fit_pass"]) != expected_uncertainty_pass:
            raise ValueError("uncertainty-fit flag is not biconditional")
        rising = [
            float(row["residual_days"])
            for row in group
            if row["residual_days"] and row["direction"] == "rising"
        ]
        falling = [
            float(row["residual_days"])
            for row in group
            if row["residual_days"] and row["direction"] == "falling"
        ]
        expected_direction_pass = (
            len(residuals) == 12
            and bool(rising)
            and bool(falling)
            and abs(statistics.median(rising)) <= 21
            and abs(statistics.median(falling)) <= 21
        )
        if (
            boolean(summary["direction_coherence_pass"])
            != expected_direction_pass
        ):
            raise ValueError("direction-coherence flag is not biconditional")
    return summaries


def rank(values: dict[str, float]) -> dict[str, int]:
    ordered = sorted(values, key=lambda member: (values[member], member))
    return {member: index + 1 for index, member in enumerate(ordered)}


def validate_rank_and_operator(
    summaries: list[dict[str, str]],
) -> tuple[float, float, bool]:
    retained = rows("product-rank-comparison.csv")
    if len(retained) != 37:
        raise ValueError("rank comparison must contain 37 rows")
    require_unique("product-rank-comparison.csv", retained, ("member",))
    members = {row["member"] for row in summaries}
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
    for row in retained:
        member = row["member"]
        for product in PRODUCTS:
            if abs(
                float(row[f"{product}_score_days"]) - scores[product][member]
            ) > 1e-9:
                raise ValueError("retained product score mismatch")
            if int(row[f"{product}_rank"]) != ranks[product][member]:
                raise ValueError("retained product rank mismatch")
    n = len(members)
    squared = sum(
        (ranks["gcc_mean"][member] - ranks["gcc_90"][member]) ** 2
        for member in members
    )
    correlation = 1.0 - (6.0 * squared / (n * (n * n - 1)))
    top_count = math.ceil(n / 4)
    top_sets = {
        product: {
            member
            for member in members
            if ranks[product][member] <= top_count
        }
        for product in PRODUCTS
    }
    overlap = len(top_sets["gcc_mean"] & top_sets["gcc_90"]) / top_count
    joint = {
        member
        for member in members
        if all(
            boolean(
                next(
                    row["crossing_sufficiency_pass"]
                    for row in summaries
                    if row["product"] == product and row["member"] == member
                )
            )
            and boolean(
                next(
                    row["uncertainty_fit_pass"]
                    for row in summaries
                    if row["product"] == product and row["member"] == member
                )
            )
            and boolean(
                next(
                    row["direction_coherence_pass"]
                    for row in summaries
                    if row["product"] == product and row["member"] == member
                )
            )
            for product in PRODUCTS
        )
    }
    operator_pass = not joint or (correlation >= 0.90 and overlap >= 0.75)
    return correlation, overlap, operator_pass


def validate_parameter_plausibility() -> tuple[bool, list[str]]:
    with SCENARIOS.open(encoding="utf-8", newline="") as stream:
        scenario_rows = list(csv.DictReader(stream))
    qualifying: list[str] = []
    scenarios = sorted({row["scenario"] for row in scenario_rows})
    for scenario in scenarios:
        if scenario in {"BASE", "SC_PLANT_GENERALIZED_DEFAULT"}:
            continue
        selected = [
            row
            for row in scenario_rows
            if row["scenario"] == scenario
            and row["operator"] == "EVENT_YEAR_RELATIVE"
            and row["residual_days"]
        ]
        if len(selected) != 148:
            continue
        medians = {
            direction: statistics.median(
                float(row["residual_days"])
                for row in selected
                if row["direction"] == direction
            )
            for direction in ("rising", "falling")
        }
        if all(abs(value) <= 21 for value in medians.values()):
            qualifying.append(scenario)
    return bool(qualifying), qualifying


def validate_decision(summaries: list[dict[str, str]]) -> None:
    decision = rows("decision-screen.csv")
    if len(decision) != 7:
        raise ValueError("decision screen must contain seven rows")
    require_unique("decision-screen.csv", decision, ("criterion",))
    mapping = {row["criterion"]: row for row in decision}
    required = {
        "OPERATOR_INDEPENDENCE",
        "CROSSING_SUFFICIENCY",
        "UNCERTAINTY_FIT",
        "DIRECTION_COHERENCE",
        "PARAMETER_PLAUSIBILITY",
        "EMPIRICAL_ROLE",
        "CALIBRATION_ROUND",
    }
    if set(mapping) != required:
        raise ValueError("decision criteria mismatch")
    for criterion, field in (
        ("CROSSING_SUFFICIENCY", "crossing_sufficiency_pass"),
        ("UNCERTAINTY_FIT", "uncertainty_fit_pass"),
        ("DIRECTION_COHERENCE", "direction_coherence_pass"),
    ):
        members_by_product = {
            product: {
                row["member"]
                for row in summaries
                if row["product"] == product and boolean(row[field])
            }
            for product in PRODUCTS
        }
        expected = bool(
            members_by_product["gcc_mean"] & members_by_product["gcc_90"]
        )
        if (mapping[criterion]["status"] == "PASS") != expected:
            raise ValueError(f"decision reduction mismatch: {criterion}")
    correlation, overlap, operator_pass = validate_rank_and_operator(summaries)
    if (mapping["OPERATOR_INDEPENDENCE"]["status"] == "PASS") != operator_pass:
        raise ValueError("decision reduction mismatch: OPERATOR_INDEPENDENCE")
    if f"spearman={correlation:.6f}" not in mapping["OPERATOR_INDEPENDENCE"]["metric"]:
        raise ValueError("operator decision Spearman metric mismatch")
    if f"top_quartile_overlap={overlap:.3f}" not in mapping["OPERATOR_INDEPENDENCE"]["metric"]:
        raise ValueError("operator decision overlap metric mismatch")
    parameter_pass, qualifying = validate_parameter_plausibility()
    if (mapping["PARAMETER_PLAUSIBILITY"]["status"] == "PASS") != parameter_pass:
        raise ValueError("decision reduction mismatch: PARAMETER_PLAUSIBILITY")
    retained_scenarios = mapping["PARAMETER_PLAUSIBILITY"]["metric"].split(
        "=", maxsplit=1
    )[1]
    expected_scenarios = "|".join(qualifying) if qualifying else "none"
    if retained_scenarios != expected_scenarios:
        raise ValueError("parameter-plausibility scenario metric mismatch")
    required_statuses = [
        mapping[name]["status"]
        for name in required
        if name != "CALIBRATION_ROUND"
    ]
    expected_recommend = all(status == "PASS" for status in required_statuses)
    if (
        mapping["CALIBRATION_ROUND"]["status"] == "RECOMMEND"
    ) != expected_recommend:
        raise ValueError("final calibration decision reduction mismatch")
    if mapping["CALIBRATION_ROUND"]["status"] != "DO_NOT_RECOMMEND":
        raise ValueError("retained disposition is not the executed stop-loss")


def validate_figures_and_docs() -> None:
    figures = sorted((ART / "figures").glob("*.svg"))
    if len(figures) != 3:
        raise ValueError("expected three SVG figures")
    namespace = {"svg": "http://www.w3.org/2000/svg"}
    for figure in figures:
        tree = ET.parse(figure)
        root = tree.getroot()
        if root.get("role") != "img":
            raise ValueError(f"{figure.name}: missing image role")
        if root.find("svg:title", namespace) is None:
            raise ValueError(f"{figure.name}: missing SVG title")
        if root.find("svg:desc", namespace) is None:
            raise ValueError(f"{figure.name}: missing SVG description")
        sidecar = figure.with_suffix(".md")
        text = sidecar.read_text(encoding="utf-8")
        if figure.name not in text:
            raise ValueError(f"{sidecar.name}: does not embed figure")
        for heading in ("## Caption", "## Ancillary information"):
            if heading not in text:
                raise ValueError(f"{sidecar.name}: missing {heading}")
    required_docs = {
        "source-custody.md",
        "calibration-readiness-matrix.md",
        "science-summary.md",
        "ecosystem-model-limitation-adjudication.md",
        "decision-rationale.md",
        "gate-evidence.md",
        "line-count-governance.md",
        "finding-disposition.md",
        "exact-diff-reconciliation.md",
        "final-disposition.md",
        "review-agent-a.md",
        "verification-agent-a.md",
        "review-agent-b.md",
        "verification-agent-b.md",
    }
    missing = sorted(name for name in required_docs if not (ART / name).is_file())
    if missing:
        raise ValueError(f"missing required documents: {missing}")
    if (PKG / "prompts/active/kickoff.md").exists():
        raise ValueError("kickoff prompt remains active after closure")
    if not (PKG / "prompts/archived/kickoff.md").is_file():
        raise ValueError("archived kickoff prompt is missing")
    package_text = (PKG / "package.md").read_text(encoding="utf-8")
    if "complete / do not calibrate / ecosystem-model limitation adjudicated" not in package_text:
        raise ValueError("package terminal status mismatch")


def main() -> int:
    validate_dependencies()
    validate_curves()
    validate_source_transitions()
    comparisons, members = validate_comparisons()
    validate_absolute(members)
    summaries = validate_summaries(comparisons, members)
    validate_decision(summaries)
    validate_figures_and_docs()
    print(
        "CAL-07F validation PASS: 6 dependencies, 731 daily rows, "
        "24 source transitions, 888 relative comparisons, 296 absolute "
        "comparisons, 74 member summaries, 3 figure/sidecar pairs, "
        "decision=DO_NOT_RECOMMEND"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, ET.ParseError) as error:
        print(f"CAL-07F validation FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
