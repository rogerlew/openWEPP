#!/usr/bin/env python3
"""Terminal validator for CAL-06 retained evidence."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import statistics
import xml.etree.ElementTree as ET
from collections import Counter, defaultdict
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
EXPECTED_FIGURES = {
    "cal06-canopy-chronology.svg",
    "cal06-seasonal-ordering-amplitude.svg",
    "cal06-snow-response.svg",
    "cal06-litter-residue-frost.svg",
    "cal06-downstream-consequences.svg",
    "cal06-congruence-verdict-matrix.svg",
}
FIGURE_SOURCES = {
    "cal06-canopy-chronology.svg": ("daily-climatology.csv",),
    "cal06-seasonal-ordering-amplitude.svg": ("ensemble-summary.csv",),
    "cal06-snow-response.svg": (
        "daily-climatology.csv",
    ),
    "cal06-litter-residue-frost.svg": (
        "daily-climatology.csv",
    ),
    "cal06-downstream-consequences.svg": ("daily-climatology.csv",),
    "cal06-congruence-verdict-matrix.svg": ("verdict-matrix.csv",),
}
EXPECTED_SIDECARS = {
    name.removesuffix(".svg") + ".md" for name in EXPECTED_FIGURES
}
SIDECAR_REQUIRED_TEXT = {
    "cal06-canopy-chronology.md": ("37-member", "model response"),
    "cal06-seasonal-ordering-amplitude.md": ("37/37", "bounded"),
    "cal06-snow-response.md": (
        "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION",
        "NOT_EVALUATED_SCALE_MISMATCH",
    ),
    "cal06-litter-residue-frost.md": (
        "NULL_AUTHORITY_MISSING",
        "NOT_ADVANCED",
    ),
    "cal06-downstream-consequences.md": (
        "NOT_ADVANCED",
        "NULL_NOT_EMITTED",
    ),
    "cal06-congruence-verdict-matrix.md": (
        "13 cells",
        "DOWNSTREAM ADVANCEMENT",
    ),
}


def rows(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def median(values: list[dict[str, str]], field: str) -> str:
    numbers = [float(row[field]) for row in values if row[field] != ""]
    return "" if not numbers else str(statistics.median(numbers))


def require_same(actual: str, expected: str, message: str) -> None:
    if actual == "" or expected == "":
        require(actual == expected, message)
    else:
        require(
            math.isclose(float(actual), float(expected), rel_tol=1.0e-12, abs_tol=1.0e-12),
            f"{message}: {actual} != {expected}",
        )


def main() -> int:
    manifest = json.loads((ARTIFACTS / "result-manifest.json").read_text(encoding="utf-8"))
    require(manifest["planned_runs"] == 261, "planned run count drifted")
    require(manifest["passed_runs"] == 261, "not all planned runs passed")
    require(manifest["forest_member_runs"] == 259, "forest/member inventory drifted")
    require(manifest["open_control_runs"] == 2, "open-control inventory drifted")
    require(manifest["accepted_member_count"] == 37, "accepted ensemble was collapsed")
    require(
        manifest["observation_operator"]["harvard_profile_density"]
        == "NOT_EVALUATED_SCALE_MISMATCH",
        "Harvard vertical profile/bulk-density scale boundary drifted",
    )
    for name, identity in manifest["outputs"].items():
        path = ARTIFACTS / name
        require(path.stat().st_size == identity["bytes"], f"{name} byte count drift")
        require(sha256(path) == identity["sha256"], f"{name} digest drift")

    runs = rows("run-results.csv")
    require(len(runs) == 261, "run-results inventory must contain 261 rows")
    counts = Counter((row["site"], row["stratum"]) for row in runs)
    expected = {
        ("marcell", "conifer"): 37,
        ("marcell", "deciduous"): 37,
        ("marcell", "mixed"): 37,
        ("marcell", "open"): 1,
        ("harvard", "deciduous"): 37,
        ("harvard", "mixed"): 37,
        ("harvard", "open"): 1,
        ("hubbard_brook", "deciduous"): 37,
        ("hubbard_brook", "mixed"): 37,
    }
    require(dict(counts) == expected, f"lane/member counts differ: {counts}")
    members: dict[tuple[str, str], set[str]] = defaultdict(set)
    for row in runs:
        require(row["run_state"] == "PASS", "non-pass run retained")
        require(int(row["day_count"]) == 16_437, "daily chronology incomplete")
        members[(row["site"], row["stratum"])].add(row["member_id"])
        if row["member_id"] != "OPEN-CONTROL":
            require(
                row["predictive_needle_source"] == "NULL_AUTHORITY_MISSING",
                "needle missing-source semantics drifted",
            )
            require(
                row["predictive_fine_woody_source"] == "NULL_AUTHORITY_MISSING",
                "fine-woody missing-source semantics drifted",
            )
        require(
            row["erosion_output"] == "NULL_NOT_EMITTED",
            "erosion absence must not be converted to zero",
        )
    reference_members = members[("marcell", "conifer")]
    require(len(reference_members) == 37, "full member inventory missing")
    for key, values in members.items():
        if "open" not in key:
            require(values == reference_members, f"{key} ensemble membership differs")

    operands = rows("run-period-operands.csv")
    by_run: dict[tuple[str, str, str], list[dict[str, str]]] = defaultdict(list)
    for row in operands:
        by_run[(row["site"], row["stratum"], row["member_id"])].append(row)
    require(len(by_run) == 261, "period operands do not cover all runs")
    for run in runs:
        key = (run["site"], run["stratum"], run["member_id"])
        values = by_run[key]
        all_rows = [row for row in values if row["period_type"] == "ALL"]
        calendar = [row for row in values if row["period_type"] == "CALENDAR_YEAR"]
        water = [row for row in values if row["period_type"] == "WATER_YEAR"]
        require(len(all_rows) == 1, f"{key}: expected one ALL operand row")
        require(len(calendar) == 45, f"{key}: expected 45 calendar-year rows")
        require(len(water) == 46, f"{key}: expected 46 water-year rows")
        overall = all_rows[0]
        require(int(overall["day_count"]) == int(run["day_count"]), f"{key}: day count")
        if overall["winter_cover_count"] == "":
            winter_mean = ""
        else:
            winter_mean = str(
                float(overall["winter_cover_sum"])
                / int(overall["winter_cover_count"])
            )
        reconstructed = {
            "winter_cover_mean": winter_mean,
            "summer_cover_max": overall["summer_cover_max"],
            "summer_lai_max": overall["summer_lai_max"],
            "cover_amplitude": (
                ""
                if overall["cover_min"] == ""
                else str(float(overall["cover_max"]) - float(overall["cover_min"]))
            ),
            "annual_leaf_litter_median_kg_m2": median(
                calendar, "annual_leaf_litter_kg_m2"
            ),
            "annual_interception_median_mm": median(
                calendar, "annual_interception_mm"
            ),
            "annual_et_median_mm": median(calendar, "annual_et_mm"),
            "annual_runoff_median_mm": median(calendar, "annual_runoff_mm"),
            "peak_swe_median_mm": median(water, "peak_swe_mm"),
            "peak_snow_depth_median_mm": median(water, "peak_snow_depth_mm"),
            "peak_snow_density_median_kg_m3": median(
                water, "peak_snow_density_kg_m3"
            ),
            "meltout_median_day_of_year": median(water, "meltout_day_of_year"),
            "frost_onset_median_day_of_year": median(
                water, "frost_onset_day_of_year"
            ),
            "frost_thaw_median_day_of_year": median(
                water, "frost_thaw_day_of_year"
            ),
        }
        for field, expected_value in reconstructed.items():
            require_same(run[field], expected_value, f"{key}: {field} reconstruction")

    summary_fields = (
        "winter_cover_mean",
        "summer_cover_max",
        "summer_lai_max",
        "cover_amplitude",
        "annual_leaf_litter_median_kg_m2",
        "annual_interception_median_mm",
        "annual_et_median_mm",
        "annual_runoff_median_mm",
        "peak_swe_median_mm",
        "peak_snow_depth_median_mm",
        "peak_snow_density_median_kg_m3",
        "meltout_median_day_of_year",
        "frost_onset_median_day_of_year",
        "frost_thaw_median_day_of_year",
    )
    lane_runs: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for run in runs:
        lane_runs[(run["site"], run["stratum"])].append(run)
    ensemble = {
        (row["site"], row["stratum"]): row for row in rows("ensemble-summary.csv")
    }
    require(set(ensemble) == set(lane_runs), "ensemble-summary lane inventory drifted")
    for key, group in lane_runs.items():
        summary = ensemble[key]
        require(int(summary["member_count"]) == len(group), f"{key}: member count")
        for field in summary_fields:
            values = [float(row[field]) for row in group if row[field] != ""]
            expected_values = {
                "min": "" if not values else str(min(values)),
                "median": "" if not values else str(statistics.median(values)),
                "max": "" if not values else str(max(values)),
            }
            for suffix, expected_value in expected_values.items():
                require_same(
                    summary[f"{field}_{suffix}"],
                    expected_value,
                    f"{key}: ensemble {field}_{suffix}",
                )

    daily = rows("daily-climatology.csv")
    require(len(daily) == 261 * 366, "daily climatology must retain every run/day-of-year")
    daily_counts = Counter(
        (row["site"], row["stratum"], row["member_id"]) for row in daily
    )
    require(set(daily_counts.values()) == {366}, "each run needs 366 climatology rows")

    scores = rows("observation-scores.csv")
    require(len(scores) == 261 * 3, "every run needs three observation score rows")
    require(
        all(
            row["verdict"]
            in {
                "BOUNDED_NO_SOURCE_UNCERTAINTY",
                "NOT_EVALUATED",
                "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION",
            }
            for row in scores
        ),
        "observation scoring invented a support verdict",
    )
    harvard_swe = [
        row
        for row in scores
        if row["site"] == "harvard"
        and row["stratum"] in {"deciduous", "open"}
        and row["quantity"] == "swe"
    ]
    require(
        harvard_swe
        and all(
            row["verdict"] == "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION"
            and int(row["matched_count"]) == 0
            for row in harvard_swe
        ),
        "Harvard SWE source contradiction must remain excluded",
    )
    require(
        all(
            row["verdict"] == "NOT_EVALUATED"
            for row in scores
            if row["site"] == "harvard"
            and row["stratum"] == "mixed"
            and row["quantity"] == "swe"
        ),
        "unbound Harvard mixed SWE must remain not evaluated",
    )
    score_groups: dict[tuple[str, str, str], list[dict[str, str]]] = defaultdict(list)
    for row in scores:
        if int(row["matched_count"]) > 0:
            score_groups[(row["site"], row["stratum"], row["quantity"])].append(row)
    score_summary = {
        (row["site"], row["stratum"], row["quantity"]): row
        for row in rows("observation-score-summary.csv")
    }
    require(
        set(score_summary) == set(score_groups),
        "observation-score-summary group inventory drifted",
    )
    for key, group in score_groups.items():
        summary = score_summary[key]
        require(int(summary["member_count"]) == len(group), f"{key}: score members")
        group_verdicts = {row["verdict"] for row in group}
        require(
            group_verdicts == {summary["verdict"]},
            f"{key}: score-summary verdict drifted",
        )
        counts = [int(row["matched_count"]) for row in group]
        require(
            int(summary["matched_count_per_member_min"]) == min(counts)
            and int(summary["matched_count_per_member_max"]) == max(counts),
            f"{key}: matched-count range drifted",
        )
        for field in ("bias", "mae", "rmse"):
            values = [float(row[field]) for row in group]
            expected_values = {
                "min": str(min(values)),
                "median": str(statistics.median(values)),
                "max": str(max(values)),
            }
            for suffix, expected_value in expected_values.items():
                require_same(
                    summary[f"{field}_{suffix}"],
                    expected_value,
                    f"{key}: score {field}_{suffix}",
                )

    verdicts = rows("verdict-matrix.csv")
    contract = rows("cell-contract.csv")
    require(len(verdicts) == len(contract) == 13, "verdict/contract cell count differs")
    require(
        {row["cell_id"] for row in verdicts}
        == {row["cell_id"] for row in contract},
        "verdict cells differ from prospective contract",
    )
    require(
        {row["status"] for row in verdicts} <= {"BOUNDED", "NOT_EVALUATED"},
        "unsupported promoted verdict present",
    )
    require(
        all(
            row["advancement"] == "NOT_ADVANCED"
            for row in verdicts
            if row["cell_id"]
            in {"CAL06-RES-001", "CAL06-FROST-001", "CAL06-ET-001", "CAL06-RUN-001", "CAL06-ERO-001"}
        ),
        "downstream gate was bypassed",
    )
    verdict_by_cell = {row["cell_id"]: row for row in verdicts}
    marcell_match_count = sum(
        int(row["matched_count"])
        for row in scores
        if row["site"] == "marcell"
        and row["verdict"] != "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION"
    )
    require(marcell_match_count == 31_542, "Marcell-only snow match count drifted")
    require(
        str(marcell_match_count)
        in verdict_by_cell["CAL06-SNOW-001"]["quantitative_result"],
        "CAL06-SNOW-001 does not report its cell-scoped count",
    )

    actual_figures = {path.name for path in FIGURES.glob("*.svg")}
    require(actual_figures == EXPECTED_FIGURES, "figure inventory differs")
    for path in sorted(FIGURES.glob("*.svg")):
        root = ET.parse(path).getroot()
        namespace = "{http://www.w3.org/2000/svg}"
        require(root.get("role") == "img", f"{path.name}: missing role=img")
        require(root.get("aria-labelledby") == "title desc", f"{path.name}: missing labels")
        require(root.find(f"{namespace}title") is not None, f"{path.name}: missing title")
        require(root.find(f"{namespace}desc") is not None, f"{path.name}: missing description")
        metadata = root.find(f"{namespace}metadata[@id='source-bindings']")
        require(metadata is not None and metadata.text, f"{path.name}: missing source binding")
        actual_bindings = dict(
            item.split(":", 1) for item in metadata.text.split(";")
        )
        expected_bindings = {
            name: sha256(ARTIFACTS / name) for name in FIGURE_SOURCES[path.name]
        }
        require(
            actual_bindings == expected_bindings,
            f"{path.name}: source/digest binding drifted",
        )
        visible_text = " ".join(
            element.text or "" for element in root.findall(f".//{namespace}text")
        )
        for ancillary_phrase in (
            "Verdict:",
            "SOURCE COMPLETENESS",
            "GATED CONSEQUENCE",
            "Exact-date observation",
            "Overall:",
        ):
            require(
                ancillary_phrase not in visible_text,
                f"{path.name}: ancillary prose remains inside plot",
            )
        if path.name == "cal06-downstream-consequences.svg":
            require(
                "conifer" not in visible_text,
                "downstream legend advertises unavailable Harvard conifer lane",
            )
            for stratum in ("open", "deciduous", "mixed"):
                require(
                    stratum in visible_text,
                    f"downstream legend is missing plotted Harvard {stratum} lane",
                )
    actual_sidecars = {
        path.name for path in FIGURES.glob("cal06-*.md")
    }
    require(actual_sidecars == EXPECTED_SIDECARS, "figure sidecar inventory differs")
    for name in sorted(actual_sidecars):
        content = (FIGURES / name).read_text(encoding="utf-8")
        require("# Caption" in content, f"{name}: caption heading missing")
        require(
            "## Ancillary information" in content,
            f"{name}: ancillary-information heading missing",
        )
        require("## Source data" in content, f"{name}: source-data heading missing")
        require(
            name.removesuffix(".md") + ".svg" in content,
            f"{name}: paired SVG is not named",
        )
        for required_text in SIDECAR_REQUIRED_TEXT[name]:
            require(
                required_text in content,
                f"{name}: required ancillary semantics missing: {required_text}",
            )

    required_docs = (
        "science-summary.md",
        "calibration-readiness-matrix.md",
        "consumer-lineage.md",
        "observation-operator-disposition.md",
        "run-period-operands.csv",
        "ensemble-summary.csv",
        "observation-score-summary.csv",
        "verdict-matrix.csv",
    )
    for name in required_docs:
        require((ARTIFACTS / name).stat().st_size > 0, f"{name} missing/empty")

    print(
        "PASS: 261 runs; 259 forest/member executions; 2 open controls; "
        "95,526 climatology rows; 783 observation scores; 13 verdict cells; "
        "6 SVG figures; 6 Markdown sidecars"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
