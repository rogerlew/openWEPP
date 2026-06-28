#!/usr/bin/env python3
"""Analyze the SNOWDENSITY-10.3.21 post-partition snow residual.

This diagnostic consumes the current no-env default profile from the
SNOWDENSITY-10.3.20 real WAT/trace artifact by default, compares it with the
pre-partition 10.3.18 profile, and decomposes the remaining forcing-robust
rubric failures by signature, site, residual component, persistence direction,
and candidate mechanism class. It makes no promotion, activation, or frost
threshold decision.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA = "snowdensity10-3-21-post-partition-residual-decomposition-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 ADR-0028 ADR-0017"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_21_post_partition_residual"
DEFAULT_CURRENT_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001"
    / "artifacts/sublimation-stage-b-unlock.json"
)
DEFAULT_PRE_PARTITION_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001"
    / "artifacts/cross-snotel-mechanism-rubric.json"
)
DEFAULT_MODEL = "activated_bundle"
PRE_PARTITION_MODEL = "activated_bundle"
POST_PARTITION_10_3_18_MODEL = "harder_pomeroy_partition"
LEGACY_FLAG_MODEL = "legacy_baseline"
ARTIFACT_STEM = "post-partition-residual-decomposition"
LABEL_SCORE = {"fail": 0, "marginal": 1, "pass": 2, "strong": 3}


SIGNATURE_CLASS = {
    "long_term_cold_season_bulk_density": "density",
    "seasonal_densification_trajectory": "density",
    "seasonal_depth_swe_slope": "depth_density_geometry",
    "seasonal_peak_swe_date": "timing",
    "seasonal_peak_depth_date": "timing",
    "seasonal_ablation_meltout_date": "timing",
    "long_term_snow_cover_duration": "persistence",
    "seasonal_accumulation_onset_date": "timing",
    "cross_cutting_bias_sign_consistency": "bias",
}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--current-report", type=Path, default=DEFAULT_CURRENT_REPORT)
    parser.add_argument("--pre-partition-report", type=Path, default=DEFAULT_PRE_PARTITION_REPORT)
    parser.add_argument("--current-model", default=DEFAULT_MODEL)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    args = parser.parse_args(argv)

    report = analyze(
        current_report_path=args.current_report.resolve(),
        pre_partition_report_path=args.pre_partition_report.resolve(),
        current_model=args.current_model,
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def analyze(
    current_report_path: Path,
    pre_partition_report_path: Path,
    current_model: str,
    output_dir: Path,
    package_artifacts_dir: Path,
) -> dict[str, Any]:
    current = read_json(current_report_path)
    pre_partition = read_json(pre_partition_report_path)
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    fail_rows = robust_fail_rows(current, current_model)
    limited_rows = forcing_limited_rows(current, current_model)
    clusters = cluster_failures(fail_rows)
    split = over_under_persistence_split(fail_rows)
    before_after = before_after_partition(pre_partition, current, current_model)
    mechanism_read = mechanism_family_read(fail_rows, clusters)
    frost_input = frost_threshold_input(current, pre_partition, fail_rows, clusters, split)
    summary = summarize(current, pre_partition, current_model, fail_rows, clusters, split)

    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Reused Ran",
        "diagnostic_only": True,
        "promotion_or_activation_decision_made": False,
        "frost_threshold_decision_made": False,
        "current_default_source": {
            "path": rel(current_report_path),
            "schema": current.get("schema"),
            "evidence_class": current.get("evidence_class"),
            "runtime_coupling": current.get("runtime_coupling"),
            "model_id": current_model,
            "robust_fail_count": summary["current_default_robust_fail_count"],
            "robust_ordinal_score": summary["current_default_robust_ordinal_score"],
        },
        "pre_partition_source": {
            "path": rel(pre_partition_report_path),
            "schema": pre_partition.get("schema"),
            "evidence_class": pre_partition.get("evidence_class"),
        },
        "authority": {
            "rubric": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050",
            "admission_authority_for_future_candidates": "ADR-0028",
            "legacy_and_pysnobal_are_flags_not_targets": True,
            "forcing_robust_cells_carry_verdicts": True,
            "forcing_limited_absolute_swe_depth_report_only": True,
            "new_contract_gate_authority_added": False,
        },
        "protected_boundaries": {
            "production_default_changed": False,
            "selector_added": False,
            "density_cap_changed": False,
            "output_schema_changed": False,
            "fixture_inputs_changed": False,
            "frost_physics_changed": False,
            "site_calibration_performed": False,
            "legacy_or_pysnobal_used_as_target": False,
        },
        "summary": summary,
        "before_after_partition_comparison": before_after,
        "robust_fail_rows": fail_rows,
        "residual_clusters": clusters,
        "mass_density_depth_decomposition": component_decomposition(current, current_model),
        "over_under_persistence_split": split,
        "forcing_limited_report_only": limited_rows,
        "mechanism_class_read": mechanism_read,
        "frost_attribution_threshold_input": frost_input,
        "raw_outputs": {
            "output_dir": rel(output_dir),
            "package_json": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
        },
    }
    write_report(report, output_dir, package_artifacts_dir)
    return report


def robust_fail_rows(report: dict[str, Any], model_id: str) -> list[dict[str, Any]]:
    rows = []
    for site in report["sites"]:
        model = site["models"][model_id]
        residual = residual_snapshot(model)
        for cell in model["rubric_profile"]["cells"]:
            if not cell.get("forcing_robust") or cell.get("ordinal_label") != "fail":
                continue
            component = SIGNATURE_CLASS.get(cell["cell_id"], "other")
            direction = residual_direction(cell, residual)
            mechanism = mechanism_class(site, cell, component, direction)
            rows.append(
                {
                    "corpus": site["corpus"],
                    "site_id": site["site_id"],
                    "snow_climate": site["snow_climate"],
                    "cell_id": cell["cell_id"],
                    "timescale": cell["timescale"],
                    "signature": cell["signature"],
                    "component_class": component,
                    "ordinal_label": cell["ordinal_label"],
                    "ordinal_score": cell["ordinal_score"],
                    "direction": direction,
                    "mechanism_class_signal": mechanism["class"],
                    "mechanism_class_read": mechanism["read"],
                    "frost_threshold_relevance": mechanism["frost_threshold_relevance"],
                    "cell_metrics": compact_metrics(cell.get("metrics", {})),
                    "site_residuals": residual,
                }
            )
    rows.sort(key=lambda row: (row["component_class"], row["cell_id"], row["site_id"]))
    return rows


def forcing_limited_rows(report: dict[str, Any], model_id: str) -> dict[str, Any]:
    rows = []
    counts = Counter()
    for site in report["sites"]:
        model = site["models"][model_id]
        for cell in model["rubric_profile"]["cells"]:
            if cell.get("forcing_robust"):
                continue
            label = cell.get("ordinal_label")
            counts[label] += 1
            if label in {"fail", "marginal"}:
                rows.append(
                    {
                        "corpus": site["corpus"],
                        "site_id": site["site_id"],
                        "snow_climate": site["snow_climate"],
                        "cell_id": cell["cell_id"],
                        "signature": cell["signature"],
                        "ordinal_label": label,
                        "metric": cell.get("metric"),
                        "metrics": compact_metrics(cell.get("metrics", {})),
                        "report_only_reason": (
                            "Forcing-limited absolute SWE/depth magnitude cells "
                            "are reported under INV-SNOWFREEZE-050 but do not carry "
                            "promotion or frost-threshold verdicts."
                        ),
                    }
                )
    return {
        "counts_by_label": dict(sorted(counts.items())),
        "report_only_fail_or_marginal_rows": rows,
    }


def cluster_failures(rows: list[dict[str, Any]]) -> dict[str, Any]:
    by_component = counter_rows(rows, "component_class")
    by_cell = counter_rows(rows, "cell_id")
    by_site = counter_rows(rows, "site_id")
    by_climate = counter_rows(rows, "snow_climate")
    by_mechanism = counter_rows(rows, "mechanism_class_signal")
    max_site = max(by_site.values(), default=0)
    concentration = "diffuse_by_site" if max_site <= 2 else "site_concentrated"
    signature_concentration = "signature_concentrated" if by_cell.get("seasonal_densification_trajectory", 0) >= 6 else "signature_diffuse"
    clusters = [
        density_trajectory_cluster(rows),
        timing_under_persistence_cluster(rows),
        depth_swe_slope_cluster(rows),
    ]
    return {
        "fail_count": len(rows),
        "by_component_class": by_component,
        "by_cell_id": by_cell,
        "by_site_id": by_site,
        "by_snow_climate": by_climate,
        "by_mechanism_class_signal": by_mechanism,
        "site_concentration_read": concentration,
        "signature_concentration_read": signature_concentration,
        "residual_clusters": [cluster for cluster in clusters if cluster["fail_count"] > 0],
    }


def density_trajectory_cluster(rows: list[dict[str, Any]]) -> dict[str, Any]:
    subset = [row for row in rows if row["cell_id"] == "seasonal_densification_trajectory"]
    beta_under = sum(1 for row in subset if numeric(row["cell_metrics"].get("beta")) is not None and row["cell_metrics"]["beta"] < 1.0)
    beta_over = sum(1 for row in subset if numeric(row["cell_metrics"].get("beta")) is not None and row["cell_metrics"]["beta"] > 1.0)
    return {
        "cluster_id": "density_trajectory_diffuse",
        "fail_count": len(subset),
        "site_ids": sorted({row["site_id"] for row in subset}),
        "climates": sorted({row["snow_climate"] for row in subset}),
        "dominant_direction": "mixed_density_shape",
        "beta_under_count": beta_under,
        "beta_over_count": beta_over,
        "mechanism_class_signal": "irreducible_or_new_density_structure",
        "read": (
            "Densification trajectory is the dominant residual signature and is diffuse "
            "across SNOTEL and cancov sites. Because Anderson/SNOBAL/CoE density "
            "variants have been exhausted, this does not point to another in-family "
            "density lever."
        ),
    }


def timing_under_persistence_cluster(rows: list[dict[str, Any]]) -> dict[str, Any]:
    subset = [
        row
        for row in rows
        if row["component_class"] in {"timing", "persistence"}
        and row["direction"] == "under_persistence"
    ]
    offsets = [
        row["cell_metrics"].get("median_offset_days")
        for row in subset
        if row["cell_metrics"].get("median_offset_days") is not None
    ]
    return {
        "cluster_id": "mountain_under_persistence_timing",
        "fail_count": len(subset),
        "site_ids": sorted({row["site_id"] for row in subset}),
        "climates": sorted({row["snow_climate"] for row in subset}),
        "median_offsets_days": offsets,
        "dominant_direction": "under_persistence",
        "mechanism_class_signal": "wind_redistribution_or_forcing_representativeness",
        "read": (
            "All robust timing failures are early modeled peak or meltout dates. "
            "This keeps an under-persistence tail, but it is not an over-persistence "
            "guardrail blocker and is concentrated in mountain SNOTEL timing cells."
        ),
    }


def depth_swe_slope_cluster(rows: list[dict[str, Any]]) -> dict[str, Any]:
    subset = [row for row in rows if row["cell_id"] == "seasonal_depth_swe_slope"]
    ratios = [
        row["cell_metrics"].get("slope_ratio")
        for row in subset
        if row["cell_metrics"].get("slope_ratio") is not None
    ]
    return {
        "cluster_id": "cancov_depth_swe_slope_geometry",
        "fail_count": len(subset),
        "site_ids": sorted({row["site_id"] for row in subset}),
        "climates": sorted({row["snow_climate"] for row in subset}),
        "slope_ratios": ratios,
        "dominant_direction": "over_depth_per_swe" if any(value > 1.0 for value in ratios) else "under_depth_per_swe",
        "mechanism_class_signal": "canopy_snow_interception_or_subcanopy_longwave",
        "read": (
            "The depth-SWE slope failures are confined to the humid-New-England "
            "paired set. They are frost-relevant because snow depth insulates frost, "
            "but the open-stratum member keeps this from being a pure canopy-only "
            "diagnosis."
        ),
    }


def component_decomposition(report: dict[str, Any], model_id: str) -> dict[str, Any]:
    rows = []
    aggregate: dict[str, list[float]] = defaultdict(list)
    direction_counts: dict[str, Counter[str]] = defaultdict(Counter)
    for site in report["sites"]:
        model = site["models"][model_id]
        residual = residual_snapshot(model)
        row = {
            "site_id": site["site_id"],
            "corpus": site["corpus"],
            "snow_climate": site["snow_climate"],
            "paired_count": residual.get("paired_count"),
            "components": {},
        }
        for component in ("swe", "depth", "density"):
            comp = residual[component]
            median_signed = comp.get("median_signed")
            mean_signed = comp.get("mean_signed")
            direction = signed_direction(median_signed)
            row["components"][component] = {
                "median_signed": median_signed,
                "mean_signed": mean_signed,
                "mean_abs": comp.get("mean_abs"),
                "modeled_over_observed_count": comp.get("modeled_over_observed_count"),
                "modeled_under_observed_count": comp.get("modeled_under_observed_count"),
                "direction": direction,
            }
            if median_signed is not None:
                aggregate[component].append(median_signed)
            direction_counts[component][direction] += 1
        rows.append(row)
    return {
        "site_rows": rows,
        "median_of_site_median_signed": {
            component: median(values) for component, values in aggregate.items()
        },
        "site_direction_counts": {
            component: dict(sorted(counts.items()))
            for component, counts in direction_counts.items()
        },
        "interpretation": (
            "SWE/depth/density are direct corpus quantities. Component directions "
            "are site-level medians and are used for diagnosis only; forcing-limited "
            "absolute SWE/depth magnitude cells remain report-only."
        ),
    }


def over_under_persistence_split(rows: list[dict[str, Any]]) -> dict[str, Any]:
    counts = Counter(row["direction"] for row in rows)
    timing_rows = [
        row
        for row in rows
        if row["component_class"] in {"timing", "persistence"}
    ]
    timing_counts = Counter(row["direction"] for row in timing_rows)
    over_tail = counts.get("over_persistence", 0)
    under_tail = counts.get("under_persistence", 0)
    density_structure = sum(
        count
        for direction, count in counts.items()
        if direction
        in {
            "density_shape_under_beta",
            "density_shape_over_beta",
            "mixed_density_shape",
            "over_depth_per_swe",
            "under_depth_per_swe",
        }
    )
    if under_tail > over_tail and timing_counts.get("under_persistence", 0) > 0:
        read = "under_persistence_tail_present_not_sole_constraint"
    elif over_tail > under_tail:
        read = "over_persistence_tail_present"
    else:
        read = "persistence_tail_not_primary"
    return {
        "all_robust_fail_direction_counts": dict(sorted(counts.items())),
        "timing_or_duration_fail_direction_counts": dict(sorted(timing_counts.items())),
        "under_persistence_fail_count": under_tail,
        "over_persistence_fail_count": over_tail,
        "density_structure_fail_count": density_structure,
        "binding_constraint_read": read,
        "interpretation": (
            "The post-partition robust fail set has no over-persistence timing tail. "
            "All timing failures are early/under-persistent, while most fail cells "
            "are density-structure signatures."
        ),
    }


def before_after_partition(
    pre_report: dict[str, Any], current_report: dict[str, Any], current_model: str
) -> dict[str, Any]:
    pre_default = model_aggregate(pre_report, PRE_PARTITION_MODEL)
    pre_hp = model_aggregate(pre_report, POST_PARTITION_10_3_18_MODEL)
    current = model_aggregate(current_report, current_model)
    legacy = model_aggregate(pre_report, LEGACY_FLAG_MODEL)
    pre_fail_map = fail_map(pre_report, PRE_PARTITION_MODEL)
    post_fail_map = fail_map(current_report, current_model)
    resolved = sorted(set(pre_fail_map) - set(post_fail_map))
    introduced = sorted(set(post_fail_map) - set(pre_fail_map))
    persisted = sorted(set(pre_fail_map) & set(post_fail_map))
    return {
        "pre_partition_activated_bundle": pre_default,
        "post_partition_10_3_18_harder_pomeroy_profile": pre_hp,
        "current_default_10_3_20_profile": current,
        "legacy_flag_profile": legacy,
        "current_default_beats_legacy_flag_profile": (
            current["robust_fail_count"] <= legacy["robust_fail_count"]
            and current["robust_ordinal_score"] >= legacy["robust_ordinal_score"]
        ),
        "resolved_fail_cells_after_partition": [pre_fail_map[key] for key in resolved],
        "introduced_fail_cells_after_partition": [post_fail_map[key] for key in introduced],
        "persisted_fail_cells_after_partition": [post_fail_map[key] for key in persisted],
        "read": (
            "Harder-Pomeroy partition improved the activated bundle from 17/172 to "
            "15/179 and the current default reconfirms 15/179. The remaining fail "
            "set is not the same as the pre-partition activated bundle."
        ),
    }


def mechanism_family_read(
    rows: list[dict[str, Any]], clusters: dict[str, Any]
) -> dict[str, Any]:
    mechanism_counts = clusters["by_mechanism_class_signal"]
    clear_new = {
        key: value
        for key, value in mechanism_counts.items()
        if key
        in {
            "canopy_snow_interception_or_subcanopy_longwave",
            "wind_redistribution_or_forcing_representativeness",
        }
    }
    return {
        "mechanism_family_exhausted_input": True,
        "snobal_coe_anderson_family_read": (
            "The remaining robust fail set is evaluated after the adopted "
            "holding-capacity, density-compaction, and Harder-Pomeroy partition "
            "changes, and after rejected spring/shallow/sublimation variants. "
            "No further in-family lever is identified by this diagnostic."
        ),
        "new_mechanism_class_counts": clear_new,
        "residual_rows_by_new_mechanism_class": {
            key: [
                {
                    "site_id": row["site_id"],
                    "cell_id": row["cell_id"],
                    "direction": row["direction"],
                }
                for row in rows
                if row["mechanism_class_signal"] == key
            ]
            for key in clear_new
        },
        "candidate_framing_under_adr0028": (
            "Any new canopy, sub-canopy longwave, or wind redistribution lever is "
            "a later opt-in candidate only if it has defensible physics, improves "
            "forcing-robust rubric cells across regimes without fixture fitting, "
            "and preserves conservation."
        ),
    }


def frost_threshold_input(
    current_report: dict[str, Any],
    pre_report: dict[str, Any],
    rows: list[dict[str, Any]],
    clusters: dict[str, Any],
    split: dict[str, Any],
) -> dict[str, Any]:
    current = model_aggregate(current_report, DEFAULT_MODEL)
    legacy = model_aggregate(pre_report, LEGACY_FLAG_MODEL)
    cancov_mechanism_count = sum(
        1
        for row in rows
        if row["mechanism_class_signal"]
        == "canopy_snow_interception_or_subcanopy_longwave"
    )
    wind_or_forcing_count = sum(
        1
        for row in rows
        if row["mechanism_class_signal"]
        == "wind_redistribution_or_forcing_representativeness"
    )
    density_irreducible_count = sum(
        1
        for row in rows
        if row["mechanism_class_signal"] == "irreducible_or_new_density_structure"
    )
    return {
        "decision_scope": "operator_input_only_no_frost_unblock_decision",
        "current_default_vs_legacy_flag": {
            "current_default": current,
            "legacy_flag": legacy,
            "current_default_beats_legacy": (
                current["robust_fail_count"] <= legacy["robust_fail_count"]
                and current["robust_ordinal_score"] >= legacy["robust_ordinal_score"]
            ),
        },
        "snow_good_enough_evidence": [
            "Current no-env default is 15/179 and beats the legacy flag profile 16/176.",
            "The SNOBAL/CoE/Anderson mechanism family has no remaining promoted candidate.",
            "Residual fails are diffuse by site; no site has more than two robust fails.",
            "No over-persistence timing tail remains after partition.",
        ],
        "one_more_lever_evidence": [
            "Humid-New-England depth-SWE slope failures point to a possible canopy/sub-canopy class.",
            "Mountain timing under-persistence points to wind redistribution or forcing/representativeness limits.",
            "Density trajectory failures dominate count, but explored density-family variants did not recover them.",
        ],
        "counts": {
            "total_robust_fails": len(rows),
            "canopy_or_subcanopy_signal_rows": cancov_mechanism_count,
            "wind_or_forcing_signal_rows": wind_or_forcing_count,
            "density_irreducible_or_new_structure_rows": density_irreducible_count,
            "under_persistence_fail_count": split["under_persistence_fail_count"],
            "over_persistence_fail_count": split["over_persistence_fail_count"],
        },
        "threshold_input_read": (
            "MIXED-NO-SINGLE-GLOBAL-SNOW-LEVER: the residual is not zero, but it "
            "is mostly density-structure plus under-persistent mountain timing, "
            "with only a small mechanism-coherent cancov forest geometry cluster. "
            "This supports an operator frost-threshold decision rather than an "
            "automatic snow promotion or automatic frost unblock."
        ),
        "not_a_decision": True,
    }


def summarize(
    current_report: dict[str, Any],
    pre_report: dict[str, Any],
    current_model: str,
    rows: list[dict[str, Any]],
    clusters: dict[str, Any],
    split: dict[str, Any],
) -> dict[str, Any]:
    current = model_aggregate(current_report, current_model)
    pre = model_aggregate(pre_report, PRE_PARTITION_MODEL)
    legacy = model_aggregate(pre_report, LEGACY_FLAG_MODEL)
    return {
        "disposition": "DIAGNOSTIC-COMPLETE-NO-PROMOTION-NO-FROST-DECISION",
        "current_default_robust_fail_count": current["robust_fail_count"],
        "current_default_robust_ordinal_score": current["robust_ordinal_score"],
        "pre_partition_activated_robust_fail_count": pre["robust_fail_count"],
        "pre_partition_activated_robust_ordinal_score": pre["robust_ordinal_score"],
        "legacy_flag_robust_fail_count": legacy["robust_fail_count"],
        "legacy_flag_robust_ordinal_score": legacy["robust_ordinal_score"],
        "robust_fail_count": len(rows),
        "dominant_failed_signature": most_common_key(clusters["by_cell_id"]),
        "dominant_component_class": most_common_key(clusters["by_component_class"]),
        "site_concentration_read": clusters["site_concentration_read"],
        "signature_concentration_read": clusters["signature_concentration_read"],
        "under_persistence_binding_read": split["binding_constraint_read"],
        "forcing_limited_absolute_swe_depth_report_only": True,
        "production_or_frost_change_authorized": False,
    }


def residual_direction(cell: dict[str, Any], residual: dict[str, Any]) -> str:
    cell_id = cell["cell_id"]
    metrics = cell.get("metrics", {})
    if cell_id in {"seasonal_peak_swe_date", "seasonal_peak_depth_date", "seasonal_ablation_meltout_date", "seasonal_accumulation_onset_date"}:
        offset = metrics.get("median_offset_days")
        if offset is None:
            return "timing_unknown"
        if offset < 0.0:
            return "under_persistence"
        if offset > 0.0:
            return "over_persistence"
        return "timing_no_bias"
    if cell_id == "long_term_snow_cover_duration":
        modeled_minus_observed = metrics.get("modeled_minus_observed_days")
        if modeled_minus_observed is None:
            modeled_minus_observed = metrics.get("median_signed_days")
        if modeled_minus_observed is None:
            return "duration_unknown"
        if modeled_minus_observed < 0.0:
            return "under_persistence"
        if modeled_minus_observed > 0.0:
            return "over_persistence"
        return "duration_no_bias"
    if cell_id == "seasonal_depth_swe_slope":
        ratio = metrics.get("slope_ratio")
        if ratio is None:
            return "depth_swe_slope_unknown"
        if ratio > 1.0:
            return "over_depth_per_swe"
        if ratio < 1.0:
            return "under_depth_per_swe"
        return "depth_swe_slope_no_bias"
    if cell_id == "seasonal_densification_trajectory":
        beta = metrics.get("beta")
        if beta is None:
            return "mixed_density_shape"
        if beta < 1.0:
            return "density_shape_under_beta"
        if beta > 1.0:
            return "density_shape_over_beta"
        return "mixed_density_shape"
    if cell_id == "long_term_cold_season_bulk_density":
        return signed_direction(residual["density"].get("median_signed"))
    if cell_id == "cross_cutting_bias_sign_consistency":
        return site_mass_depth_direction(residual)
    return "unclassified"


def mechanism_class(
    site: dict[str, Any], cell: dict[str, Any], component: str, direction: str
) -> dict[str, str]:
    corpus = site["corpus"]
    climate = site["snow_climate"]
    cell_id = cell["cell_id"]
    if cell_id == "seasonal_depth_swe_slope" and corpus == "cancov_forest":
        return {
            "class": "canopy_snow_interception_or_subcanopy_longwave",
            "read": (
                "Forest-paired depth/SWE geometry can be affected by interception, "
                "canopy unloading, sublimation from canopy storage, and sub-canopy "
                "longwave. The open member means representativeness remains plausible."
            ),
            "frost_threshold_relevance": "high_depth_insulation_relevance",
        }
    if component in {"timing", "persistence"} and direction == "under_persistence":
        return {
            "class": "wind_redistribution_or_forcing_representativeness",
            "read": (
                "Early peak or meltout in mountain SNOTEL profiles can indicate "
                "missing wind redistribution/exposure physics or forcing/point-vs-"
                "hillslope representativeness rather than another density-family lever."
            ),
            "frost_threshold_relevance": "moderate_snow_duration_relevance",
        }
    if cell_id == "seasonal_densification_trajectory":
        if corpus == "cancov_forest" and ("conifer" in climate or "hardwood" in climate):
            relevance = "high_forest_depth_density_relevance"
        else:
            relevance = "moderate_density_relevance"
        return {
            "class": "irreducible_or_new_density_structure",
            "read": (
                "The signature names a density trajectory shape error. Existing "
                "SNOBAL/Anderson density-family levers did not recover it; no "
                "specific new mechanism is isolated by this cell alone."
            ),
            "frost_threshold_relevance": relevance,
        }
    return {
        "class": "forcing_limited_or_irreducible",
        "read": (
            "No specific new conserving mechanism class is isolated by this robust "
            "cell without additional independent forcing or site structure evidence."
        ),
        "frost_threshold_relevance": "diagnostic_context_only",
    }


def residual_snapshot(model: dict[str, Any]) -> dict[str, Any]:
    source = model.get("residual_decomposition", {})
    result = {"paired_count": source.get("paired_count")}
    for component in ("swe", "depth", "density"):
        comp = source.get(component, {})
        result[component] = {
            "median_signed": comp.get("median_signed"),
            "mean_signed": comp.get("mean_signed"),
            "mean_abs": comp.get("mean_abs"),
            "median_abs": comp.get("median_abs"),
            "fail_count": comp.get("fail_count"),
            "modeled_over_observed_count": comp.get("modeled_over_observed_count"),
            "modeled_under_observed_count": comp.get("modeled_under_observed_count"),
        }
    return result


def compact_metrics(metrics: dict[str, Any]) -> dict[str, Any]:
    keep = [
        "kge",
        "r",
        "beta",
        "gamma",
        "paired_count",
        "median_offset_days",
        "iqr_offset_days",
        "annual_count",
        "slope_ratio",
        "modeled_slope_depth_per_swe",
        "observed_slope_depth_per_swe",
        "median_signed_bias",
        "iqr_bias",
        "modeled_minus_observed_days",
    ]
    return {key: metrics[key] for key in keep if key in metrics}


def model_aggregate(report: dict[str, Any], model_id: str) -> dict[str, Any]:
    aggregate = report["model_summaries"][model_id]["aggregate"]
    return {
        "model_id": model_id,
        "robust_fail_count": aggregate["robust_fail_count"],
        "robust_ordinal_score": aggregate["robust_ordinal_score"],
        "robust_available_cell_count": aggregate["robust_available_cell_count"],
    }


def fail_map(report: dict[str, Any], model_id: str) -> dict[tuple[str, str], dict[str, Any]]:
    result = {}
    for site in report["sites"]:
        model = site["models"][model_id]
        for cell in model["rubric_profile"]["cells"]:
            if cell.get("forcing_robust") and cell.get("ordinal_label") == "fail":
                key = (site["site_id"], cell["cell_id"])
                result[key] = {
                    "site_id": site["site_id"],
                    "snow_climate": site["snow_climate"],
                    "cell_id": cell["cell_id"],
                    "signature": cell["signature"],
                }
    return result


def counter_rows(rows: list[dict[str, Any]], key: str) -> dict[str, int]:
    return dict(sorted(Counter(row[key] for row in rows).items()))


def most_common_key(counts: dict[str, int]) -> str | None:
    if not counts:
        return None
    return sorted(counts.items(), key=lambda item: (-item[1], item[0]))[0][0]


def site_mass_depth_direction(residual: dict[str, Any]) -> str:
    swe = residual["swe"].get("median_signed")
    depth = residual["depth"].get("median_signed")
    if swe is not None and depth is not None:
        if swe < 0.0 and depth < 0.0:
            return "under_mass_and_depth"
        if swe > 0.0 and depth > 0.0:
            return "over_mass_and_depth"
    return "mixed_mass_depth"


def signed_direction(value: float | None) -> str:
    if value is None:
        return "unknown"
    if value < 0.0:
        return "under"
    if value > 0.0:
        return "over"
    return "near_zero"


def numeric(value: Any) -> float | None:
    if isinstance(value, int | float):
        return float(value)
    return None


def median(values: list[float]) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    mid = len(ordered) // 2
    if len(ordered) % 2:
        return ordered[mid]
    return (ordered[mid - 1] + ordered[mid]) / 2.0


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_report(report: dict[str, Any], output_dir: Path, package_artifacts_dir: Path) -> None:
    for directory in (output_dir, package_artifacts_dir):
        write_json(directory / f"{ARTIFACT_STEM}.json", report)
        (directory / f"{ARTIFACT_STEM}.md").write_text(render_markdown(report), encoding="utf-8")


def write_json(path: Path, document: dict[str, Any]) -> None:
    path.write_text(json.dumps(document, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.21 Post-Partition Residual Decomposition",
        "",
        "Evidence mode: Static diagnostic over reused real WAT/trace evidence.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Current default robust fail/score: `{summary['current_default_robust_fail_count']}` / `{summary['current_default_robust_ordinal_score']}`",
        f"- Pre-partition activated robust fail/score: `{summary['pre_partition_activated_robust_fail_count']}` / `{summary['pre_partition_activated_robust_ordinal_score']}`",
        f"- Dominant failed signature: `{summary['dominant_failed_signature']}`",
        f"- Dominant component class: `{summary['dominant_component_class']}`",
        f"- Site concentration: `{summary['site_concentration_read']}`",
        f"- Under-persistence read: `{summary['under_persistence_binding_read']}`",
        "- Production/frost/default changes authorized: `false`",
        "",
        "## Robust Fail Clusters",
        "",
        "| Cluster | Fails | Direction | Mechanism signal | Read |",
        "|---|---:|---|---|---|",
    ]
    for cluster in report["residual_clusters"]["residual_clusters"]:
        lines.append(
            "| {cluster_id} | {fail_count} | {direction} | {mechanism} | {read} |".format(
                cluster_id=cluster["cluster_id"],
                fail_count=cluster["fail_count"],
                direction=cluster["dominant_direction"],
                mechanism=cluster["mechanism_class_signal"],
                read=cluster["read"],
            )
        )
    lines.extend(
        [
            "",
            "## Robust Fail Rows",
            "",
            "| Site | Climate | Cell | Component | Direction | Mechanism signal |",
            "|---|---|---|---|---|---|",
        ]
    )
    for row in report["robust_fail_rows"]:
        lines.append(
            "| {site} | {climate} | `{cell}` | {component} | {direction} | {mechanism} |".format(
                site=row["site_id"],
                climate=row["snow_climate"],
                cell=row["cell_id"],
                component=row["component_class"],
                direction=row["direction"],
                mechanism=row["mechanism_class_signal"],
            )
        )
    frost = report["frost_attribution_threshold_input"]
    lines.extend(
        [
            "",
            "## Frost-Threshold Input",
            "",
            f"- Threshold input read: `{frost['threshold_input_read']}`",
            f"- Decision made: `{not frost['not_a_decision']}`",
            "",
            "Snow good-enough evidence:",
        ]
    )
    for item in frost["snow_good_enough_evidence"]:
        lines.append(f"- {item}")
    lines.append("")
    lines.append("One-more-lever evidence:")
    for item in frost["one_more_lever_evidence"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "Forcing-limited absolute SWE/depth magnitude cells are report-only under INV-SNOWFREEZE-050. Legacy and PySnobal profiles remain ADR-0017 flags, not targets.",
            "",
        ]
    )
    return "\n".join(lines)


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
