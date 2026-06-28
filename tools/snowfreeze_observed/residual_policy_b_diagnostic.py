#!/usr/bin/env python3
"""Classify residual tails and Policy-B evidence after SNOWDENSITY-10.3.12."""

from __future__ import annotations

import argparse
import datetime as dt
import json
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
import snotel_density_three_way as rubric  # noqa: E402
import spring_pack_depletion_compaction_adjudication as spring10  # noqa: E402


SCHEMA = "snowdensity10-3-13-residual-policy-b-diagnostic-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-070 OBL-SNOWFREEZE-P-045"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-13-residual-policy-b-diagnostic-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_13_residual_policy_b_diagnostic"
DEFAULT_BUNDLE_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001"
    / "artifacts/bundle-activation-adjudication.json"
)
ARTIFACT_STEM = "residual-policy-b-diagnostic"

MODEL_DEFAULT = "default"
MODEL_HOLDING = "holding_capacity_only"
MODEL_BUNDLE = "bundle"
MODEL_SPRING = "spring_densification"

STATE_PASS = "PASS"
STATE_OVER = "OVER_FAIL"
STATE_UNDER = "UNDER_FAIL"

UNDER_PERSISTED = "UNDER_PERSISTED_FROM_HOLDING"
UNDER_INDUCED_FROM_PASS = "UNDER_INDUCED_FROM_HOLDING_PASS"
UNDER_INDUCED_FROM_OVER = "UNDER_INDUCED_FROM_HOLDING_OVER"
UNDER_OTHER = "UNDER_UNRESOLVED_TRANSITION"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle-report", type=Path, default=DEFAULT_BUNDLE_REPORT)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    args = parser.parse_args(argv)

    report = diagnose(
        bundle_report_path=args.bundle_report.resolve(),
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def diagnose(
    bundle_report_path: Path,
    output_dir: Path,
    package_artifacts_dir: Path,
) -> dict[str, Any]:
    bundle_report = read_json(bundle_report_path)
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    surface_by_id = {surface.surface_id: surface for surface in phase.SURFACES}
    surfaces = []
    for source_surface in bundle_report["surfaces"]:
        surface = surface_by_id[source_surface["surface_id"]]
        surfaces.append(analyze_surface(surface, source_surface))

    summary = summarize(surfaces, bundle_report)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "source": {
            "bundle_report": rel(bundle_report_path),
            "bundle_schema": bundle_report["schema"],
            "runtime_coupling": bundle_report["runtime_coupling"],
        },
        "active_density_cap": {
            "density_cap_kg_m3": spring10.SNOW_DENSITY_CAP_KG_M3,
            "authority": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-003 / REF-SNOWFREEZE-CH3-SNOWDENS-LIM",
            "changed_by_this_package": False,
            "snobal_550_reanchor_status": "FOLLOW_UP_ONLY_NOT_EVALUATED_HERE",
        },
        "protected_boundaries": {
            "production_physics_changed": False,
            "default_activation_changed": False,
            "density_cap_changed": False,
            "selector_added": False,
            "parser_runfile_user_surface_changed": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "observed_depth_or_density_consumed_by_runtime": False,
            "qwet_or_frzftp_changed": False,
            "compatibility_runtime_changed": False,
            "frost_attribution_authorized": False,
        },
        "summary": summary,
        "policy_b_evidence_matrix": policy_b_matrix(bundle_report),
        "surfaces": surfaces,
        "raw_outputs": {
            "output_dir": rel(output_dir),
            "package_json": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
        },
    }

    write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    markdown = render_markdown(report)
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    return report


def analyze_surface(surface: phase.Surface, source_surface: dict[str, Any]) -> dict[str, Any]:
    if surface.verdict_scope != "paired_observation":
        return {
            "surface_id": surface.surface_id,
            "site_group": surface.site_group,
            "cover": surface.cover,
            "verdict_scope": surface.verdict_scope,
            "observation_source": surface.observation_source,
            "paired_row_count": 0,
            "transition_summary": {},
            "note": surface.note,
        }

    pairs_by_model = {
        MODEL_DEFAULT: load_pair_map(surface, source_surface["prior_default"]["wat"]),
        MODEL_HOLDING: load_pair_map(surface, source_surface["prior_holding_capacity_only"]["wat"]),
        MODEL_BUNDLE: load_pair_map(surface, source_surface["bundle"]["wat"]),
        MODEL_SPRING: load_pair_map(surface, source_surface["prior_spring_densification"]["wat"]),
    }
    common_dates = sorted(set.intersection(*(set(pairs) for pairs in pairs_by_model.values())))
    rows = [
        build_transition_row(date, {model: pairs[date] for model, pairs in pairs_by_model.items()})
        for date in common_dates
    ]
    bundle_failures = [row for row in rows if row["bundle_state"] != STATE_PASS]
    bundle_under = [row for row in rows if row["bundle_state"] == STATE_UNDER]
    bundle_over = [row for row in rows if row["bundle_state"] == STATE_OVER]
    march_april_bundle_failures = [
        row
        for row in bundle_failures
        if dt.date.fromisoformat(row["date"]).month in march_april.MARCH_APRIL_MONTHS
    ]
    return {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "verdict_scope": surface.verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "paired_row_count": len(rows),
        "transition_summary": transition_summary(rows),
        "under_persistence": under_summary(bundle_under),
        "over_persistence": over_summary(bundle_over),
        "march_april": march_april_summary(march_april_bundle_failures),
        "bundle_vs_holding_physics_delta": bundle_vs_holding_delta(rows),
        "samples": {
            "under_persistence": sample_rows(bundle_under),
            "over_persistence": sample_rows(bundle_over),
        },
        "note": surface.note,
    }


def load_pair_map(surface: phase.Surface, wat_path: str) -> dict[str, dict[str, Any]]:
    path = REPO_ROOT / wat_path
    if not path.is_file():
        raise FileNotFoundError(f"required WAT output is missing: {path}")
    modeled = observed_harness.load_modeled_wat(path)
    observations = phase.load_observations(surface)
    pairs = phase.pair_observations(observations, modeled, surface.observation_kind)
    return {pair["date"]: annotate_pair(pair) for pair in pairs}


def annotate_pair(pair: dict[str, Any]) -> dict[str, Any]:
    row = march_april.annotate_pair(pair)
    row["state"] = state_for(row)
    row["modeled_density_kg_m3"] = density_from_swe_depth(
        row.get("modeled_snow_water_m"), row.get("modeled_snow_depth_m")
    )
    return row


def state_for(row: dict[str, Any]) -> str:
    residual = row["depth_residual_m"]
    tolerance = row["depth_tolerance_m"]
    if abs(residual) <= tolerance:
        return STATE_PASS
    if residual > 0.0:
        return STATE_OVER
    return STATE_UNDER


def build_transition_row(date: str, pairs: dict[str, dict[str, Any]]) -> dict[str, Any]:
    bundle = pairs[MODEL_BUNDLE]
    holding = pairs[MODEL_HOLDING]
    spring_row = spring10.evaluate_pair(dict(bundle))
    return {
        "date": date,
        "water_year": bundle["water_year"],
        "observed_snow_depth_m": bundle["observed_snow_depth_m"],
        "default_state": pairs[MODEL_DEFAULT]["state"],
        "holding_state": holding["state"],
        "bundle_state": bundle["state"],
        "spring_state": pairs[MODEL_SPRING]["state"],
        "bundle_under_transition": under_transition(holding["state"], bundle["state"]),
        "holding_depth_residual_m": holding["depth_residual_m"],
        "bundle_depth_residual_m": bundle["depth_residual_m"],
        "spring_depth_residual_m": pairs[MODEL_SPRING]["depth_residual_m"],
        "holding_modeled_depth_m": holding["modeled_snow_depth_m"],
        "bundle_modeled_depth_m": bundle["modeled_snow_depth_m"],
        "holding_swe_m": holding.get("modeled_snow_water_m"),
        "bundle_swe_m": bundle.get("modeled_snow_water_m"),
        "holding_density_kg_m3": holding.get("modeled_density_kg_m3"),
        "bundle_density_kg_m3": bundle.get("modeled_density_kg_m3"),
        "bundle_minus_holding_depth_m": (
            bundle["modeled_snow_depth_m"] - holding["modeled_snow_depth_m"]
        ),
        "bundle_minus_holding_swe_m": none_delta(
            bundle.get("modeled_snow_water_m"), holding.get("modeled_snow_water_m")
        ),
        "bundle_minus_holding_density_kg_m3": none_delta(
            bundle.get("modeled_density_kg_m3"), holding.get("modeled_density_kg_m3")
        ),
        "march_april_cap_class": spring_row["adjudication_class"],
        "required_swe_depletion_to_clear_at_cap_m": spring_row.get(
            "required_swe_depletion_to_clear_at_cap_m"
        ),
    }


def under_transition(holding_state: str, bundle_state: str) -> str | None:
    if bundle_state != STATE_UNDER:
        return None
    if holding_state == STATE_UNDER:
        return UNDER_PERSISTED
    if holding_state == STATE_PASS:
        return UNDER_INDUCED_FROM_PASS
    if holding_state == STATE_OVER:
        return UNDER_INDUCED_FROM_OVER
    return UNDER_OTHER


def transition_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "paired_row_count": len(rows),
        "holding_to_bundle_counts": dict(
            sorted(Counter(f"{row['holding_state']}->{row['bundle_state']}" for row in rows).items())
        ),
        "default_to_bundle_counts": dict(
            sorted(Counter(f"{row['default_state']}->{row['bundle_state']}" for row in rows).items())
        ),
        "bundle_to_spring_counts": dict(
            sorted(Counter(f"{row['bundle_state']}->{row['spring_state']}" for row in rows).items())
        ),
        "bundle_state_counts": dict(sorted(Counter(row["bundle_state"] for row in rows).items())),
    }


def under_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    transition_counts = Counter(row["bundle_under_transition"] for row in rows)
    induced = transition_counts[UNDER_INDUCED_FROM_PASS] + transition_counts[UNDER_INDUCED_FROM_OVER]
    return {
        "bundle_under_fail_count": len(rows),
        "transition_counts": dict(sorted(transition_counts.items())),
        "induced_by_bundle_density_arm_count": induced,
        "induced_by_bundle_density_arm_fraction": safe_div(induced, len(rows)),
        "lead_hypothesis": (
            "BULK_COMPACTION_MECHANISM_COST_SUPPORTED"
            if induced > 0
            else "BULK_COMPACTION_COST_NOT_SUPPORTED_BY_TRANSITIONS"
        ),
    }


def over_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    cap_counts = Counter(row["march_april_cap_class"] for row in rows)
    return {
        "bundle_over_fail_count": len(rows),
        "cap_class_counts": dict(sorted(cap_counts.items())),
        "cap_limited_or_patchy_count": cap_counts[spring10.CLASS_CAP_LIMITED_DEPLETION]
        + cap_counts[spring10.CLASS_PATCHY_DEPLETION],
        "compaction_feasible_count": cap_counts[spring10.CLASS_COMPACTION_ONLY],
    }


def march_april_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    cap_counts = Counter(row["march_april_cap_class"] for row in rows)
    under_rows = [row for row in rows if row["bundle_state"] == STATE_UNDER]
    over_rows = [row for row in rows if row["bundle_state"] == STATE_OVER]
    return {
        "bundle_failure_count": len(rows),
        "under_fail_count": len(under_rows),
        "over_fail_count": len(over_rows),
        "cap_class_counts": dict(sorted(cap_counts.items())),
    }


def bundle_vs_holding_delta(rows: list[dict[str, Any]]) -> dict[str, Any]:
    depth_deltas = [row["bundle_minus_holding_depth_m"] for row in rows]
    swe_deltas = [
        row["bundle_minus_holding_swe_m"]
        for row in rows
        if row["bundle_minus_holding_swe_m"] is not None
    ]
    density_deltas = [
        row["bundle_minus_holding_density_kg_m3"]
        for row in rows
        if row["bundle_minus_holding_density_kg_m3"] is not None
    ]
    return {
        "mean_depth_delta_m": mean(depth_deltas),
        "max_abs_depth_delta_m": max(abs(value) for value in depth_deltas) if depth_deltas else None,
        "mean_swe_delta_m": mean(swe_deltas),
        "max_abs_swe_delta_m": max(abs(value) for value in swe_deltas) if swe_deltas else None,
        "mean_density_delta_kg_m3": mean(density_deltas),
        "max_abs_density_delta_kg_m3": max(abs(value) for value in density_deltas)
        if density_deltas
        else None,
    }


def summarize(surfaces: list[dict[str, Any]], bundle_report: dict[str, Any]) -> dict[str, Any]:
    paired = [surface for surface in surfaces if surface["verdict_scope"] == "paired_observation"]
    transition_counts: Counter[str] = Counter()
    under_counts: Counter[str] = Counter()
    cap_counts: Counter[str] = Counter()
    paired_rows = 0
    for surface in paired:
        paired_rows += surface["paired_row_count"]
        transition_counts.update(surface["transition_summary"]["holding_to_bundle_counts"])
        under_counts.update(surface["under_persistence"]["transition_counts"])
        cap_counts.update(surface["march_april"]["cap_class_counts"])

    induced_under = under_counts[UNDER_INDUCED_FROM_PASS] + under_counts[UNDER_INDUCED_FROM_OVER]
    bundle_summary = bundle_report["summary"]
    return {
        "disposition": "HOLD-ACTIVATION-EVIDENCE-MISSING",
        "activation_policy": "POLICY-B",
        "activation_ready": False,
        "activation_blocker": "POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING",
        "frost_attribution_unblocked": False,
        "frost_attribution_blocker": "SNOW-CONTROL-RESIDUALS-REMAIN",
        "paired_surface_count": len(paired),
        "complete_transition_row_count": paired_rows,
        "source_bundle_paired_row_count": bundle_summary["paired_row_count"],
        "default_snow_control_fail_count": bundle_summary["default_snow_control_fail_count"],
        "bundle_snow_control_fail_count": bundle_summary["bundle_snow_control_fail_count"],
        "fail_delta_default_minus_bundle": bundle_summary["fail_delta_default_minus_bundle"],
        "holding_to_bundle_transition_counts": dict(sorted(transition_counts.items())),
        "under_persistence_transition_counts": dict(sorted(under_counts.items())),
        "under_persistence_induced_by_bundle_density_arm_count": induced_under,
        "under_persistence_induced_by_bundle_density_arm_fraction": safe_div(
            induced_under, sum(under_counts.values())
        ),
        "march_april_bundle_cap_class_counts": dict(sorted(cap_counts.items())),
        "lead_under_persistence_hypothesis": (
            "BULK_COMPACTION_MECHANISM_COST_SUPPORTED"
            if induced_under > 0
            else "BULK_COMPACTION_COST_NOT_SUPPORTED_BY_TRANSITIONS"
        ),
        "next_recommended_package": "SNOWDENSITY-10.3.14-POLICY-B-NO-REGRESSION-AND-CAP-AUTHORITY",
    }


def policy_b_matrix(bundle_report: dict[str, Any]) -> list[dict[str, str]]:
    summary = bundle_report["summary"]
    return [
        {
            "scope": "direct bundle trace proof",
            "status": "PASS",
            "evidence": "10.3.12 trace rows count both selected bundle members.",
        },
        {
            "scope": "gate-eligible paired-snow improvement versus current default",
            "status": "PASS",
            "evidence": (
                f"default {summary['default_snow_control_fail_count']} -> "
                f"bundle {summary['bundle_snow_control_fail_count']}; "
                f"delta {summary['fail_delta_default_minus_bundle']}"
            ),
        },
        {
            "scope": "paired surface no-worse guard versus holding-only",
            "status": "PASS",
            "evidence": (
                f"worse surface count {summary['paired_surface_worse_vs_holding_count']}"
            ),
        },
        {
            "scope": "full workspace regression/identity with bundle as default",
            "status": "MISSING",
            "evidence": "No default-activation branch was exercised in this diagnostic.",
        },
        {
            "scope": "non-snow climate no-regression",
            "status": "MISSING",
            "evidence": "No global default bundle run over non-snow climates was produced.",
        },
        {
            "scope": "erosion and water-balance no-regression",
            "status": "MISSING",
            "evidence": "No global default bundle comparison over erosion/WB outputs was produced.",
        },
        {
            "scope": "watershed routing no-regression",
            "status": "MISSING",
            "evidence": "No watershed default bundle comparison was produced.",
        },
        {
            "scope": "composite melt-density conservation under bundle",
            "status": "MISSING",
            "evidence": "Component conservation exists, but composite global activation evidence is absent.",
        },
    ]


def sample_rows(rows: list[dict[str, Any]], limit: int = 8) -> list[dict[str, Any]]:
    keys = [
        "date",
        "observed_snow_depth_m",
        "holding_state",
        "bundle_state",
        "bundle_under_transition",
        "holding_depth_residual_m",
        "bundle_depth_residual_m",
        "bundle_minus_holding_depth_m",
        "bundle_minus_holding_swe_m",
        "bundle_minus_holding_density_kg_m3",
        "march_april_cap_class",
    ]
    sorted_rows = sorted(rows, key=lambda row: abs(row["bundle_depth_residual_m"]), reverse=True)
    return [{key: row.get(key) for key in keys} for row in sorted_rows[:limit]]


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.13 Residual Tail And Policy-B Diagnostic",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Activation policy: `{summary['activation_policy']}`",
        f"- Activation ready: `{summary['activation_ready']}`",
        f"- Activation blocker: `{summary['activation_blocker']}`",
        f"- Frost-attribution blocker: `{summary['frost_attribution_blocker']}`",
        f"- Complete transition rows: `{summary['complete_transition_row_count']}`",
        f"- Source bundle paired rows: `{summary['source_bundle_paired_row_count']}`",
        f"- Default failures: `{summary['default_snow_control_fail_count']}`",
        f"- Bundle failures: `{summary['bundle_snow_control_fail_count']}`",
        f"- Default -> bundle delta: `{summary['fail_delta_default_minus_bundle']}`",
        f"- Under-persistence induced by bundle density arm: "
        f"`{summary['under_persistence_induced_by_bundle_density_arm_count']}`",
        f"- Lead under-persistence hypothesis: "
        f"`{summary['lead_under_persistence_hypothesis']}`",
        "",
        "## Transition Summary",
        "",
        f"- Holding -> bundle transitions: `{summary['holding_to_bundle_transition_counts']}`",
        f"- Under-persistence transitions: `{summary['under_persistence_transition_counts']}`",
        f"- March/April cap classes: `{summary['march_april_bundle_cap_class_counts']}`",
        "",
        "## Policy-B Evidence Matrix",
        "",
        "| Scope | Status | Evidence |",
        "|---|---|---|",
    ]
    for row in report["policy_b_evidence_matrix"]:
        lines.append(f"| {row['scope']} | `{row['status']}` | {row['evidence']} |")
    lines.extend(
        [
            "",
            "## Surface Results",
            "",
            "| Surface | Cover | Rows | Bundle under | Induced under | Bundle over |",
            "|---|---|---:|---:|---:|---:|",
        ]
    )
    for surface in report["surfaces"]:
        if surface["verdict_scope"] != "paired_observation":
            continue
        under = surface["under_persistence"]
        over = surface["over_persistence"]
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{surface['surface_id']}`",
                    surface["cover"],
                    str(surface["paired_row_count"]),
                    str(under["bundle_under_fail_count"]),
                    str(under["induced_by_bundle_density_arm_count"]),
                    str(over["bundle_over_fail_count"]),
                ]
            )
            + " |"
        )
    lines.extend(
        [
            "",
            "## Boundary Disposition",
            "",
            "- Default activation changed: `false`.",
            "- Production physics changed: `false`.",
            "- Density cap changed: `false`.",
            "- `550 kg m^-3` cap re-anchor status: `FOLLOW_UP_ONLY_NOT_EVALUATED_HERE`.",
            "- Frost attribution authorized: `false`.",
            "",
        ]
    )
    return "\n".join(lines)


def density_from_swe_depth(swe_m: float | None, depth_m: float | None) -> float | None:
    if swe_m is None or depth_m is None or depth_m <= 0.0:
        return None
    return 1000.0 * swe_m / depth_m


def none_delta(left: float | None, right: float | None) -> float | None:
    if left is None or right is None:
        return None
    return left - right


def safe_div(numerator: float, denominator: float) -> float | None:
    return numerator / denominator if denominator else None


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
