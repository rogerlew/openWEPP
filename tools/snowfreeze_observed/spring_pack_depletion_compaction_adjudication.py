#!/usr/bin/env python3
"""Adjudicate spring pack-depletion versus compaction feasibility.

This is SNOWDENSITY-10.3.10 diagnostic-only evidence tooling. It consumes the
10.3.8 coupled direct-production WAT candidate and paired March/April
observations, then asks a bounded physical question: can the modeled SWE fit
within the observed snow-depth tolerance by compaction alone under the existing
SC-SNOWFREEZE-001 density cap, or is pack depletion / patchy meltout required?
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import math
import sys
from collections import Counter
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import march_april_residual_attribution as march_april  # noqa: E402
import observed_harness  # noqa: E402
import phase_partition_snowdepth_adjudication as phase  # noqa: E402


SCHEMA = "snowdensity10-3-10-spring-pack-depletion-compaction-adjudication-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-003 INV-SNOWFREEZE-047 INV-SNOWFREEZE-050 INV-SNOWFREEZE-067"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_INPUT_REPORT = march_april.DEFAULT_INPUT_REPORT
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_10_spring_pack_depletion_compaction"
BASELINE_MODEL = march_april.BASELINE_MODEL
SNOW_DENSITY_CAP_KG_M3 = 522.0

CLASS_PASS = "PASS"
CLASS_UNDER_PERSISTENCE = "UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT"
CLASS_NO_SWE = "DEPTH_FAIL_NO_MODELED_SWE_STATE"
CLASS_COMPACTION_ONLY = "COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP"
CLASS_CAP_LIMITED_DEPLETION = "CAP_LIMITED_DEPLETION_REQUIRED"
CLASS_PATCHY_DEPLETION = "PATCHY_MELTOUT_OR_DEPLETION_REQUIRED"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-report", type=Path, default=DEFAULT_INPUT_REPORT)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    args = parser.parse_args(argv)

    report = adjudicate(
        input_report=args.input_report.resolve(),
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    input_report: Path,
    output_dir: Path,
    package_artifacts_dir: Path,
) -> dict[str, Any]:
    source_report = read_json(input_report)
    source_candidate = source_report["diagnostic_selector"]["opt_in_value"]
    if source_candidate != BASELINE_MODEL:
        raise ValueError(f"expected {BASELINE_MODEL}, got {source_candidate}")

    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)
    surface_by_id = {surface.surface_id: surface for surface in phase.SURFACES}
    surfaces = [
        analyze_surface(surface_by_id[source_surface["surface_id"]], source_surface)
        for source_surface in source_report["surfaces"]
    ]
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "source": {
            "input_report": rel(input_report),
            "source_schema": source_report["schema"],
            "baseline_model": BASELINE_MODEL,
            "runtime_coupling": "real direct-production WAT from SNOWDENSITY-10.3.8",
        },
        "adjudication_basis": {
            "density_cap_kg_m3": SNOW_DENSITY_CAP_KG_M3,
            "density_cap_authority": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-003 / REF-SNOWFREEZE-CH3-SNOWDENS-LIM",
            "month_window": sorted(march_april.MARCH_APRIL_MONTHS),
            "closure_depth": "observed_snow_depth_m + TOL-SNOWFREEZE-011 depth tolerance",
        },
        "protected_boundaries": {
            "production_physics_changed": False,
            "default_activation_changed": False,
            "selector_added": False,
            "parser_runfile_user_surface_changed": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "density_cap_changed": False,
            "radiation_canopy_phase_density_melt_rain_heat_longwave_frost_changed": False,
        },
        "summary": summarize(surfaces),
        "surfaces": surfaces,
    }
    write_json(package_artifacts_dir / "spring-pack-depletion-compaction-adjudication.json", report)
    write_json(output_dir / "spring-pack-depletion-compaction-adjudication.json", report)
    markdown = render_markdown(report)
    (package_artifacts_dir / "spring-pack-depletion-compaction-adjudication.md").write_text(
        markdown, encoding="utf-8"
    )
    (output_dir / "spring-pack-depletion-compaction-adjudication.md").write_text(
        markdown, encoding="utf-8"
    )
    return report


def analyze_surface(surface: phase.Surface, source_surface: dict[str, Any]) -> dict[str, Any]:
    candidate = source_surface["candidate"]
    wat_path = REPO_ROOT / candidate["wat"]
    if not wat_path.is_file():
        raise FileNotFoundError(f"10.3.8 WAT path is missing: {wat_path}")
    modeled = observed_harness.load_modeled_wat(wat_path)
    observations = phase.load_observations(surface)
    pairs = phase.pair_observations(observations, modeled, surface.observation_kind)
    annotated = [march_april.annotate_pair(pair) for pair in pairs]
    march_pairs = [
        pair
        for pair in annotated
        if dt.date.fromisoformat(pair["date"]).month in march_april.MARCH_APRIL_MONTHS
    ]
    evaluations = [evaluate_pair(pair) for pair in march_pairs]
    return {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "fixture_dir": rel(surface.fixture_dir),
        "verdict_scope": surface.verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "wat": candidate["wat"],
        "observation_row_count": len(observations),
        "march_april_paired_row_count": len(evaluations),
        "march_april_failure_count": sum(1 for row in evaluations if row["depth_fail"]),
        "adjudication": evaluation_summary(evaluations),
        "failure_samples": sample_failures(evaluations),
        "note": surface.note,
    }


def evaluate_pair(pair: dict[str, Any]) -> dict[str, Any]:
    row = dict(pair)
    modeled_swe = row.get("modeled_snow_water_m")
    observed_depth = row["observed_snow_depth_m"]
    modeled_depth = row["modeled_snow_depth_m"]
    tolerance = row["depth_tolerance_m"]
    allowed_depth = observed_depth + tolerance
    modeled_density = row.get("modeled_density_kg_m3")
    cap_depth = (
        1000.0 * modeled_swe / SNOW_DENSITY_CAP_KG_M3
        if modeled_swe is not None
        else None
    )
    required_density = (
        1000.0 * modeled_swe / allowed_depth
        if modeled_swe is not None and allowed_depth > 0.0
        else None
    )
    cap_swe_allowed = SNOW_DENSITY_CAP_KG_M3 * allowed_depth / 1000.0
    required_swe_depletion = (
        max(0.0, modeled_swe - cap_swe_allowed) if modeled_swe is not None else None
    )
    row.update(
        {
            "allowed_depth_to_clear_m": allowed_depth,
            "density_cap_kg_m3": SNOW_DENSITY_CAP_KG_M3,
            "modeled_cap_limited_depth_m": cap_depth,
            "required_density_to_clear_kg_m3": required_density,
            "density_headroom_to_cap_kg_m3": (
                SNOW_DENSITY_CAP_KG_M3 - modeled_density
                if modeled_density is not None
                else None
            ),
            "required_swe_depletion_to_clear_at_cap_m": required_swe_depletion,
        }
    )
    row["adjudication_class"] = classify_feasibility(row)
    return row


def classify_feasibility(row: dict[str, Any]) -> str:
    if not row["depth_fail"]:
        return CLASS_PASS
    if row["depth_residual_m"] < 0.0:
        return CLASS_UNDER_PERSISTENCE
    if row.get("modeled_snow_water_m") is None:
        return CLASS_NO_SWE
    cap_depth = row["modeled_cap_limited_depth_m"]
    if cap_depth is not None and cap_depth <= row["allowed_depth_to_clear_m"]:
        return CLASS_COMPACTION_ONLY
    if (
        row["observed_snow_depth_m"] <= march_april.PATCHY_OBS_DEPTH_M
        and row["modeled_snow_depth_m"] >= march_april.PATCHY_MODELED_DEPTH_M
    ):
        return CLASS_PATCHY_DEPLETION
    return CLASS_CAP_LIMITED_DEPLETION


def evaluation_summary(evaluations: list[dict[str, Any]]) -> dict[str, Any]:
    failures = [row for row in evaluations if row["depth_fail"]]
    counts = Counter(row["adjudication_class"] for row in evaluations)
    failure_counts = Counter(row["adjudication_class"] for row in failures)
    depletion_rows = [
        row
        for row in failures
        if row["adjudication_class"] in {CLASS_CAP_LIMITED_DEPLETION, CLASS_PATCHY_DEPLETION}
    ]
    compaction_rows = [
        row for row in failures if row["adjudication_class"] == CLASS_COMPACTION_ONLY
    ]
    required_losses = [
        row["required_swe_depletion_to_clear_at_cap_m"]
        for row in depletion_rows
        if row["required_swe_depletion_to_clear_at_cap_m"] is not None
    ]
    required_densities = [
        row["required_density_to_clear_kg_m3"]
        for row in compaction_rows
        if row["required_density_to_clear_kg_m3"] is not None
    ]
    return {
        "paired_count": len(evaluations),
        "failure_count": len(failures),
        "class_counts_all_rows": dict(sorted(counts.items())),
        "class_counts_failures": dict(sorted(failure_counts.items())),
        "compaction_only_feasible_failure_count": len(compaction_rows),
        "depletion_required_failure_count": len(depletion_rows),
        "depletion_required_fraction_of_failures": safe_div(len(depletion_rows), len(failures)),
        "compaction_only_fraction_of_failures": safe_div(len(compaction_rows), len(failures)),
        "mean_required_swe_depletion_to_clear_at_cap_m": mean(required_losses),
        "max_required_swe_depletion_to_clear_at_cap_m": max(required_losses) if required_losses else None,
        "row_sum_required_swe_depletion_to_clear_at_cap_m": sum(required_losses)
        if required_losses
        else 0.0,
        "mean_required_density_for_compaction_only_rows_kg_m3": mean(required_densities),
        "max_required_density_for_compaction_only_rows_kg_m3": max(required_densities)
        if required_densities
        else None,
    }


def summarize(surfaces: list[dict[str, Any]]) -> dict[str, Any]:
    paired = [surface for surface in surfaces if surface["verdict_scope"] == "paired_observation"]
    blocked = [surface for surface in surfaces if surface["verdict_scope"] != "paired_observation"]
    total_pairs = sum(surface["march_april_paired_row_count"] for surface in paired)
    total_failures = sum(surface["march_april_failure_count"] for surface in paired)
    class_counts = Counter()
    cover_counts = Counter()
    depletion_count = 0
    compaction_count = 0
    row_sum_loss = 0.0
    for surface in paired:
        surface_counts = surface["adjudication"]["class_counts_failures"]
        class_counts.update(surface_counts)
        depletion_count += surface["adjudication"]["depletion_required_failure_count"]
        compaction_count += surface["adjudication"]["compaction_only_feasible_failure_count"]
        row_sum_loss += surface["adjudication"]["row_sum_required_swe_depletion_to_clear_at_cap_m"]
        for class_name, count in surface_counts.items():
            if class_name in {CLASS_CAP_LIMITED_DEPLETION, CLASS_PATCHY_DEPLETION}:
                cover_counts[surface["cover"]] += count
    disposition = "SPRING-DEPLETION-FIRST" if depletion_count >= compaction_count else "SPRING-COMPACTION-FIRST"
    next_process = (
        "SPRING-SNOW-COVER-DEPLETION-AND-MASS-EXPORT-CANDIDATE"
        if disposition == "SPRING-DEPLETION-FIRST"
        else "SPRING-COMPACTION-DENSIFICATION-CANDIDATE"
    )
    return {
        "disposition": disposition,
        "candidate_model": BASELINE_MODEL,
        "density_cap_kg_m3": SNOW_DENSITY_CAP_KG_M3,
        "paired_surface_count": len(paired),
        "observation_blocked_surface_count": len(blocked),
        "march_april_paired_rows": total_pairs,
        "march_april_fail_count": total_failures,
        "class_counts_failures": dict(sorted(class_counts.items())),
        "compaction_only_feasible_failure_count": compaction_count,
        "depletion_required_failure_count": depletion_count,
        "depletion_required_fraction_of_failures": safe_div(depletion_count, total_failures),
        "compaction_only_fraction_of_failures": safe_div(compaction_count, total_failures),
        "depletion_required_by_cover": dict(sorted(cover_counts.items())),
        "row_sum_required_swe_depletion_to_clear_at_cap_m": row_sum_loss,
        "recommended_next_process": next_process,
        "remaining_blocker": "SNOW-CONTROL-NOT-CLEARED",
    }


def sample_failures(evaluations: list[dict[str, Any]]) -> list[dict[str, Any]]:
    failures = [row for row in evaluations if row["depth_fail"]]
    samples = sorted(failures, key=lambda row: abs(row["depth_residual_m"]), reverse=True)[:12]
    keep_keys = [
        "date",
        "adjudication_class",
        "observed_snow_depth_m",
        "modeled_snow_depth_m",
        "depth_residual_m",
        "allowed_depth_to_clear_m",
        "modeled_snow_water_m",
        "modeled_density_kg_m3",
        "required_density_to_clear_kg_m3",
        "modeled_cap_limited_depth_m",
        "required_swe_depletion_to_clear_at_cap_m",
        "observed_density_kg_m3",
        "observed_swe_depth_density_correspondence",
    ]
    return [{key: row.get(key) for key in keep_keys if key in row} for row in samples]


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# Spring Pack-Depletion and Compaction Adjudication",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Candidate baseline: `{summary['candidate_model']}`",
        f"- Density cap: `{summary['density_cap_kg_m3']} kg m^-3`",
        f"- Disposition: `{summary['disposition']}`",
        f"- Recommended next process: `{summary['recommended_next_process']}`",
        f"- Remaining blocker: `{summary['remaining_blocker']}`",
        "",
        "## Summary",
        "",
        "| Metric | Value |",
        "| --- | ---: |",
        f"| March/April paired rows | {summary['march_april_paired_rows']} |",
        f"| March/April failed rows | {summary['march_april_fail_count']} |",
        f"| Compaction-only feasible failures | {summary['compaction_only_feasible_failure_count']} |",
        f"| Depletion-required failures | {summary['depletion_required_failure_count']} |",
        f"| Depletion-required fraction | {fmt(summary['depletion_required_fraction_of_failures'])} |",
        f"| Row-sum SWE depletion required at cap (m) | {fmt(summary['row_sum_required_swe_depletion_to_clear_at_cap_m'])} |",
        "",
        "## Failure Classes",
        "",
        "| Class | Failed rows |",
        "| --- | ---: |",
    ]
    for class_name, count in summary["class_counts_failures"].items():
        lines.append(f"| `{class_name}` | {count} |")
    lines.extend(
        [
            "",
            "## Surface Results",
            "",
            "| Surface | Cover | Scope | Failures | Compaction-only | Depletion-required | Dominant class |",
            "| --- | --- | --- | ---: | ---: | ---: | --- |",
        ]
    )
    for surface in report["surfaces"]:
        counts = surface["adjudication"]["class_counts_failures"]
        dominant = dominant_class(counts)
        lines.append(
            "| {surface} | {cover} | {scope} | {failures} | {compaction} | {depletion} | `{dominant}` |".format(
                surface=surface["surface_id"],
                cover=surface["cover"],
                scope=surface["verdict_scope"],
                failures=surface["march_april_failure_count"],
                compaction=surface["adjudication"]["compaction_only_feasible_failure_count"],
                depletion=surface["adjudication"]["depletion_required_failure_count"],
                dominant=dominant,
            )
        )
    lines.extend(
        [
            "",
            "## Boundary Disposition",
            "",
            "- Diagnostic-only; no production physics or default behavior changed.",
            "- The `522 kg m^-3` cap is existing `SC-SNOWFREEZE-001` authority, not a fitted threshold.",
            "- Observation-blocked surfaces remain non-verdict surfaces.",
            "- Row-summed required SWE depletion is diagnostic row evidence, not a water-balance ledger.",
        ]
    )
    return "\n".join(lines) + "\n"


def dominant_class(counts: dict[str, Any]) -> str | None:
    if not counts:
        return None
    return max(counts.items(), key=lambda item: item[1])[0]


def finite_values(values: Any) -> list[float]:
    finite = []
    for value in values:
        if value is not None and math.isfinite(value):
            finite.append(value)
    return finite


def mean(values: Any) -> float | None:
    finite = finite_values(values)
    if not finite:
        return None
    return sum(finite) / len(finite)


def safe_div(numerator: int | float, denominator: int | float) -> float | None:
    if denominator == 0:
        return None
    return numerator / denominator


def fmt(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    raise SystemExit(main())
