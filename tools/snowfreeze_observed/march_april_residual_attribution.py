#!/usr/bin/env python3
"""Attribute March/April snow-depth residuals after SNOWDENSITY-10.3.8.

This is SNOWDENSITY-10.3.9 evidence tooling. It is diagnostic-only: it consumes
the committed 10.3.8 coupled direct-production WAT report and paired snow-depth
observations, then classifies the remaining March/April failures by process
signature. It does not run or change production snow physics.
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

import observed_harness  # noqa: E402
import phase_partition_snowdepth_adjudication as phase  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowdensity10-3-9-march-april-residual-attribution-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-050 INV-SNOWFREEZE-067"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-9-march-april-residual-attribution-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_INPUT_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001"
    / "artifacts/liquid-holding-capacity-coupled-wat.json"
)
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_9_march_april_residual_attribution"
MARCH_APRIL_MONTHS = {3, 4}
BASELINE_MODEL = "coe_liquid_holding_capacity_v1"
PATCHY_OBS_DEPTH_M = 0.025
PATCHY_MODELED_DEPTH_M = 0.05
DENSITY_LOW_BIAS_KG_M3 = 50.0
SWE_ABS_TOL_M = 0.03
SWE_REL_TOL = 0.25
MAX_SWE_DENSITY_RATIO_FOR_AUTHORITY = 2.0
MIN_SWE_DENSITY_RATIO_FOR_AUTHORITY = 0.5


MECHANISM_PATCHY_MELTOUT = "PATCHY_MELTOUT_OR_SNOW_COVER_DEPLETION"
MECHANISM_DENSITY_COMPACTION = "DENSITY_OR_COMPACTION_DEFICIT"
MECHANISM_SWE_EXCESS = "SWE_EXCESS_OR_ABLATION_DEFICIT"
MECHANISM_DEPTH_ONLY = "DEPTH_ONLY_OVERPERSISTENCE_UNRESOLVED"
MECHANISM_UNDER_PERSISTENCE = "UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT"
MECHANISM_PASS = "PASS"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-report", type=Path, default=DEFAULT_INPUT_REPORT)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    args = parser.parse_args(argv)

    report = attribute_residuals(
        input_report=args.input_report.resolve(),
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def attribute_residuals(
    input_report: Path,
    output_dir: Path,
    package_artifacts_dir: Path,
) -> dict[str, Any]:
    source_report = read_json(input_report)
    source_candidate = source_report["diagnostic_selector"]["opt_in_value"]
    if source_candidate != BASELINE_MODEL:
        raise ValueError(
            f"expected 10.3.8 candidate model {BASELINE_MODEL}, "
            f"got {source_candidate}"
        )

    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    surface_by_id = {surface.surface_id: surface for surface in phase.SURFACES}
    analyzed_surfaces = []
    for source_surface in source_report["surfaces"]:
        surface = surface_by_id[source_surface["surface_id"]]
        analyzed_surfaces.append(analyze_surface(surface, source_surface))

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
        "month_window": {
            "months": sorted(MARCH_APRIL_MONTHS),
            "label": "March/April",
        },
        "protected_boundaries": {
            "production_physics_changed": False,
            "default_activation_changed": False,
            "selector_added": False,
            "parser_runfile_user_surface_changed": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "radiation_canopy_phase_density_melt_rain_heat_longwave_frost_changed": False,
        },
        "summary": summarize(analyzed_surfaces),
        "surfaces": analyzed_surfaces,
    }
    write_json(package_artifacts_dir / "march-april-residual-attribution.json", report)
    write_json(output_dir / "march-april-residual-attribution.json", report)
    markdown = render_markdown(report)
    (package_artifacts_dir / "march-april-residual-attribution.md").write_text(
        markdown, encoding="utf-8"
    )
    (output_dir / "march-april-residual-attribution.md").write_text(markdown, encoding="utf-8")
    return report


def analyze_surface(surface: phase.Surface, source_surface: dict[str, Any]) -> dict[str, Any]:
    candidate = source_surface["candidate"]
    wat_path = REPO_ROOT / candidate["wat"]
    if not wat_path.is_file():
        raise FileNotFoundError(f"10.3.8 WAT path is missing: {wat_path}")
    modeled = observed_harness.load_modeled_wat(wat_path)
    observations = phase.load_observations(surface)
    pairs = phase.pair_observations(observations, modeled, surface.observation_kind)
    annotated = [annotate_pair(pair) for pair in pairs]
    march_april_pairs = [
        pair for pair in annotated if dt.date.fromisoformat(pair["date"]).month in MARCH_APRIL_MONTHS
    ]
    all_failures = [pair for pair in annotated if pair["depth_fail"]]
    march_april_failures = [pair for pair in march_april_pairs if pair["depth_fail"]]
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
        "paired_row_count": len(annotated),
        "all_paired": residual_window_summary(annotated),
        "march_april": residual_window_summary(march_april_pairs),
        "march_april_failure_attribution": attribution_summary(march_april_failures),
        "march_april_failure_fraction_of_surface_failures": safe_div(
            len(march_april_failures), len(all_failures)
        ),
        "march_april_failure_samples": sample_failures(march_april_failures),
        "modeled_march_april_wat_summary": wat_month_summary(modeled, MARCH_APRIL_MONTHS),
        "note": surface.note,
    }


def annotate_pair(pair: dict[str, Any]) -> dict[str, Any]:
    row = dict(pair)
    observed_depth = row["observed_snow_depth_m"]
    modeled_depth = row["modeled_snow_depth_m"]
    residual = row["depth_residual_m"]
    tolerance = rubric.snow_depth_tolerance(observed_depth)
    modeled_swe = row.get("modeled_snow_water_m")
    modeled_density = density_from_swe_depth(modeled_swe, modeled_depth)
    observed_density = row.get("observed_density_kg_m3")
    observed_swe = row.get("observed_swe_m")
    expected_swe = density_to_swe(observed_density, observed_depth)
    swe_authority = swe_is_correspondent(observed_swe, expected_swe)
    swe_residual = modeled_swe - observed_swe if modeled_swe is not None and observed_swe is not None else None
    density_residual = (
        modeled_density - observed_density
        if modeled_density is not None and observed_density is not None
        else None
    )
    row.update(
        {
            "depth_tolerance_m": tolerance,
            "depth_fail": abs(residual) > tolerance,
            "month": dt.date.fromisoformat(row["date"]).month,
            "modeled_density_kg_m3": modeled_density,
            "observed_swe_expected_from_depth_density_m": expected_swe,
            "observed_swe_depth_density_correspondence": swe_authority,
            "swe_residual_m": swe_residual,
            "density_residual_kg_m3": density_residual,
        }
    )
    row["attribution"] = classify_pair(row)
    return row


def classify_pair(row: dict[str, Any]) -> str:
    if not row["depth_fail"]:
        return MECHANISM_PASS
    residual = row["depth_residual_m"]
    if residual < 0.0:
        return MECHANISM_UNDER_PERSISTENCE
    observed_depth = row["observed_snow_depth_m"]
    modeled_depth = row["modeled_snow_depth_m"]
    if observed_depth <= PATCHY_OBS_DEPTH_M and modeled_depth >= PATCHY_MODELED_DEPTH_M:
        return MECHANISM_PATCHY_MELTOUT

    density_residual = row.get("density_residual_kg_m3")
    if density_residual is not None and density_residual <= -DENSITY_LOW_BIAS_KG_M3:
        return MECHANISM_DENSITY_COMPACTION

    if row.get("observed_swe_depth_density_correspondence") == "correspondent":
        swe_residual = row.get("swe_residual_m")
        observed_swe = row.get("observed_swe_m")
        if swe_residual is not None and observed_swe is not None:
            swe_tolerance = max(SWE_ABS_TOL_M, abs(observed_swe) * SWE_REL_TOL)
            if swe_residual > swe_tolerance:
                return MECHANISM_SWE_EXCESS
            if abs(swe_residual) <= swe_tolerance:
                return MECHANISM_DENSITY_COMPACTION

    return MECHANISM_DEPTH_ONLY


def residual_window_summary(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    residuals = [row["depth_residual_m"] for row in pairs]
    failures = [row for row in pairs if row["depth_fail"]]
    over = [row for row in failures if row["depth_residual_m"] > 0.0]
    under = [row for row in failures if row["depth_residual_m"] < 0.0]
    if not residuals:
        return {
            "paired_count": 0,
            "fail_count": 0,
            "fail_fraction": None,
            "mean_signed_depth_residual_m": None,
            "mean_abs_depth_residual_m": None,
            "max_abs_depth_residual_m": None,
            "over_fail_count": 0,
            "under_fail_count": 0,
        }
    return {
        "paired_count": len(pairs),
        "fail_count": len(failures),
        "fail_fraction": len(failures) / len(pairs),
        "mean_signed_depth_residual_m": mean(residuals),
        "mean_abs_depth_residual_m": mean(abs(value) for value in residuals),
        "max_abs_depth_residual_m": max(abs(value) for value in residuals),
        "over_fail_count": len(over),
        "under_fail_count": len(under),
        "modeled_over_observed_fraction": safe_div(
            sum(1 for row in pairs if row["depth_residual_m"] > 0.0), len(pairs)
        ),
    }


def attribution_summary(failures: list[dict[str, Any]]) -> dict[str, Any]:
    counts = Counter(row["attribution"] for row in failures)
    return {
        "failure_count": len(failures),
        "counts": dict(sorted(counts.items())),
        "dominant_mechanism": counts.most_common(1)[0][0] if counts else None,
        "density_or_depletion_count": sum(
            counts[key]
            for key in [
                MECHANISM_PATCHY_MELTOUT,
                MECHANISM_DENSITY_COMPACTION,
                MECHANISM_DEPTH_ONLY,
            ]
        ),
        "swe_or_mass_excess_count": counts[MECHANISM_SWE_EXCESS],
    }


def wat_month_summary(
    modeled: dict[dt.date, dict[str, float | None]], months: set[int]
) -> dict[str, Any]:
    rows = [row for date, row in modeled.items() if date.month in months]
    depths = finite_values(row.get("snow_depth_m") for row in rows)
    swes = finite_values(row.get("snow_water_m") for row in rows)
    densities = [
        density
        for row in rows
        if (density := density_from_swe_depth(row.get("snow_water_m"), row.get("snow_depth_m")))
        is not None
    ]
    return {
        "modeled_day_count": len(rows),
        "snow_depth_day_count": len(depths),
        "peak_snow_depth_m": max(depths) if depths else None,
        "mean_snow_depth_m": mean(depths),
        "depth_day_sum_m_days": sum(depths) if depths else 0.0,
        "peak_snow_water_m": max(swes) if swes else None,
        "mean_snow_water_m": mean(swes),
        "swe_day_sum_m_days": sum(swes) if swes else 0.0,
        "mean_modeled_density_kg_m3": mean(densities),
    }


def sample_failures(failures: list[dict[str, Any]]) -> list[dict[str, Any]]:
    samples = sorted(failures, key=lambda row: abs(row["depth_residual_m"]), reverse=True)[:12]
    keep_keys = [
        "date",
        "observed_snow_depth_m",
        "modeled_snow_depth_m",
        "depth_residual_m",
        "depth_tolerance_m",
        "observed_swe_m",
        "modeled_snow_water_m",
        "observed_density_kg_m3",
        "modeled_density_kg_m3",
        "density_residual_kg_m3",
        "observed_swe_depth_density_correspondence",
        "attribution",
    ]
    return [{key: row.get(key) for key in keep_keys if key in row} for row in samples]


def summarize(surfaces: list[dict[str, Any]]) -> dict[str, Any]:
    paired = [surface for surface in surfaces if surface["verdict_scope"] == "paired_observation"]
    blocked = [surface for surface in surfaces if surface["verdict_scope"] != "paired_observation"]
    total_pairs = sum(surface["all_paired"]["paired_count"] for surface in paired)
    total_failures = sum(surface["all_paired"]["fail_count"] for surface in paired)
    march_pairs = sum(surface["march_april"]["paired_count"] for surface in paired)
    march_failures = sum(surface["march_april"]["fail_count"] for surface in paired)
    mechanism_counts = Counter()
    cover_counts = Counter()
    for surface in paired:
        mechanism_counts.update(surface["march_april_failure_attribution"]["counts"])
        if surface["march_april"]["fail_count"]:
            cover_counts[surface["cover"]] += surface["march_april"]["fail_count"]
    dominant = mechanism_counts.most_common(1)[0][0] if mechanism_counts else None
    recommended_next = recommend_next_route(mechanism_counts)
    return {
        "disposition": "MARCH_APRIL-RESIDUALS-ATTRIBUTED",
        "candidate_model": BASELINE_MODEL,
        "paired_surface_count": len(paired),
        "observation_blocked_surface_count": len(blocked),
        "total_paired_rows": total_pairs,
        "total_fail_count": total_failures,
        "march_april_paired_rows": march_pairs,
        "march_april_fail_count": march_failures,
        "march_april_failure_fraction_of_all_failures": safe_div(march_failures, total_failures),
        "march_april_fail_fraction": safe_div(march_failures, march_pairs),
        "dominant_march_april_mechanism": dominant,
        "march_april_mechanism_counts": dict(sorted(mechanism_counts.items())),
        "march_april_failures_by_cover": dict(sorted(cover_counts.items())),
        "recommended_next_process": recommended_next,
        "remaining_blocker": "SNOW-CONTROL-NOT-CLEARED",
    }


def recommend_next_route(mechanism_counts: Counter[str]) -> str:
    depletion_or_density = sum(
        mechanism_counts[key]
        for key in [
            MECHANISM_PATCHY_MELTOUT,
            MECHANISM_DENSITY_COMPACTION,
            MECHANISM_DEPTH_ONLY,
        ]
    )
    mass_excess = mechanism_counts[MECHANISM_SWE_EXCESS]
    if depletion_or_density >= mass_excess:
        return "SPRING-PACK-DEPLETION-AND-COMPACTION-ADJUDICATION"
    return "SPRING-ENERGY-BALANCE-MELT-ADJUDICATION"


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# March/April Residual Attribution",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Candidate baseline: `{report['summary']['candidate_model']}`",
        f"- Disposition: `{report['summary']['disposition']}`",
        f"- Recommended next process: `{report['summary']['recommended_next_process']}`",
        f"- Remaining blocker: `{report['summary']['remaining_blocker']}`",
        "",
        "## Summary",
        "",
        "| Metric | Value |",
        "| --- | ---: |",
        f"| Total paired rows | {report['summary']['total_paired_rows']} |",
        f"| Total failed rows | {report['summary']['total_fail_count']} |",
        f"| March/April paired rows | {report['summary']['march_april_paired_rows']} |",
        f"| March/April failed rows | {report['summary']['march_april_fail_count']} |",
        f"| March/April share of all failures | {fmt(report['summary']['march_april_failure_fraction_of_all_failures'])} |",
        f"| March/April fail fraction | {fmt(report['summary']['march_april_fail_fraction'])} |",
        "",
        "## Mechanism Counts",
        "",
        "| Mechanism | March/April failed rows |",
        "| --- | ---: |",
    ]
    for mechanism, count in report["summary"]["march_april_mechanism_counts"].items():
        lines.append(f"| `{mechanism}` | {count} |")
    lines.extend(
        [
            "",
            "## Cover Counts",
            "",
            "| Cover | March/April failed rows |",
            "| --- | ---: |",
        ]
    )
    for cover, count in report["summary"]["march_april_failures_by_cover"].items():
        lines.append(f"| `{cover}` | {count} |")
    lines.extend(
        [
            "",
            "## Surface Residuals",
            "",
            "| Surface | Cover | Scope | All fail | March/April fail | March/April share | Dominant attribution |",
            "| --- | --- | --- | ---: | ---: | ---: | --- |",
        ]
    )
    for surface in report["surfaces"]:
        lines.append(
            "| {surface} | {cover} | {scope} | {all_fail} | {ma_fail} | {share} | `{dominant}` |".format(
                surface=surface["surface_id"],
                cover=surface["cover"],
                scope=surface["verdict_scope"],
                all_fail=surface["all_paired"]["fail_count"],
                ma_fail=surface["march_april"]["fail_count"],
                share=fmt(surface["march_april_failure_fraction_of_surface_failures"]),
                dominant=surface["march_april_failure_attribution"]["dominant_mechanism"],
            )
        )
    lines.extend(
        [
            "",
            "## Boundary Disposition",
            "",
            "- Diagnostic-only; no production physics or default behavior changed.",
            "- Observation-blocked surfaces remain non-verdict surfaces.",
            "- Harvard SWE/mass attribution is correspondence-caveated unless source SWE, depth, and density agree.",
        ]
    )
    return "\n".join(lines) + "\n"


def density_from_swe_depth(swe_m: float | None, depth_m: float | None) -> float | None:
    if swe_m is None or depth_m is None or depth_m <= 0.0:
        return None
    return 1000.0 * swe_m / depth_m


def density_to_swe(density_kg_m3: float | None, depth_m: float | None) -> float | None:
    if density_kg_m3 is None or depth_m is None:
        return None
    return density_kg_m3 * depth_m / 1000.0


def swe_is_correspondent(observed_swe_m: float | None, expected_swe_m: float | None) -> str:
    if observed_swe_m is None or expected_swe_m is None:
        return "unavailable"
    if expected_swe_m <= 0.0:
        return "zero_depth_density"
    ratio = observed_swe_m / expected_swe_m
    if (
        MIN_SWE_DENSITY_RATIO_FOR_AUTHORITY
        <= ratio
        <= MAX_SWE_DENSITY_RATIO_FOR_AUTHORITY
    ):
        return "correspondent"
    return "non_correspondent_units_or_source_semantics"


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
