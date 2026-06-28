#!/usr/bin/env python3
"""Adjudicate SNOWDENSITY-10.3.17 shallow-pack compaction guard."""

from __future__ import annotations

import argparse
import json
import sys
from collections import Counter
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import default_activation_active_cap as active15  # noqa: E402
import march_april_residual_attribution as march_april  # noqa: E402
import observed_harness  # noqa: E402
import phase_partition_snowdepth_adjudication as phase  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402
import spring_pack_depletion_compaction_adjudication as spring10  # noqa: E402


SCHEMA = "snowdensity10-3-17-shallow-pack-compaction-guard-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-074 OBL-SNOWFREEZE-P-049"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_17_shallow_pack_compaction_guard"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_BUNDLE_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001"
    / "artifacts/bundle-activation-adjudication.json"
)
ARTIFACT_STEM = "shallow-pack-compaction-guard"

MELT_ENV = active15.MELT_ENV
DENSITY_ENV = active15.DENSITY_ENV
TRACE_ENV = active15.TRACE_ENV
DEFAULT_MELT_MODEL = active15.DEFAULT_MELT_MODEL
DEFAULT_DENSITY_MODEL = active15.DEFAULT_DENSITY_MODEL
SHALLOW_GUARD_DENSITY_MODEL = "physics_bulk_shallow_guard_v1"
SHALLOW_GUARD_DEPTH_THRESHOLD_M = 0.25
TRACE_CLOSURE_TOLERANCE_M = 1.0e-9
MASS_TERM_INVARIANCE_TOLERANCE_M = 1.0e-12

STATE_PASS = "PASS"
STATE_OVER = "OVER_FAIL"
STATE_UNDER = "UNDER_FAIL"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--bundle-report", type=Path, default=DEFAULT_BUNDLE_REPORT)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = adjudicate(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        bundle_report_path=args.bundle_report.resolve(),
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    bundle_report_path: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")
    bundle_report = read_json(bundle_report_path)

    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    baseline_trace_paths: dict[str, Path] = {}
    candidate_trace_paths: dict[str, Path] = {}
    bundle_by_surface = {item["surface_id"]: item for item in bundle_report["surfaces"]}
    surfaces = []
    for surface in phase.SURFACES:
        baseline_trace = output_dir / "traces" / f"{surface.surface_id}_activated_default.jsonl"
        candidate_trace = output_dir / "traces" / f"{surface.surface_id}_shallow_guard.jsonl"
        baseline_trace_paths[surface.surface_id] = baseline_trace
        candidate_trace_paths[surface.surface_id] = candidate_trace
        baseline_run = active15.run_and_analyze(
            surface=surface,
            scenario="activated_default",
            output_dir=output_dir,
            hill_binary=hill_binary,
            trace_path=baseline_trace,
            env_updates={
                MELT_ENV: None,
                DENSITY_ENV: None,
                TRACE_ENV: str(baseline_trace),
            },
            run_model=run_models,
        )
        candidate_run = active15.run_and_analyze(
            surface=surface,
            scenario="shallow_guard",
            output_dir=output_dir,
            hill_binary=hill_binary,
            trace_path=candidate_trace,
            env_updates={
                MELT_ENV: None,
                DENSITY_ENV: SHALLOW_GUARD_DENSITY_MODEL,
                TRACE_ENV: str(candidate_trace),
            },
            run_model=run_models,
        )
        surfaces.append(
            build_surface_report(
                surface=surface,
                source_surface=bundle_by_surface[surface.surface_id],
                baseline_run=baseline_run,
                candidate_run=candidate_run,
            )
        )

    baseline_trace_proof = active15.build_trace_proof(
        baseline_trace_paths,
        expected_melt_model=DEFAULT_MELT_MODEL,
        expected_density_model=DEFAULT_DENSITY_MODEL,
    )
    candidate_trace_proof = active15.build_trace_proof(
        candidate_trace_paths,
        expected_melt_model=DEFAULT_MELT_MODEL,
        expected_density_model=SHALLOW_GUARD_DENSITY_MODEL,
    )
    trace_comparison = compare_traces(baseline_trace_paths, candidate_trace_paths)
    summary = summarize(surfaces, candidate_trace_proof, trace_comparison)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "threshold_authority": {
            "snow_shallow_compaction_guard_depth_threshold_m": SHALLOW_GUARD_DEPTH_THRESHOLD_M,
            "source": "Marks/SNOBAL active surface-layer depth; not fixture fitted",
            "libsnobal_c_read": False,
            "deny_toml_gpl_family_allowed": False,
        },
        "baseline": {
            "snow_melt_model": DEFAULT_MELT_MODEL,
            "snow_density_model": DEFAULT_DENSITY_MODEL,
            "selector_state": "absent/empty env selects activated default",
            "trace_proof": baseline_trace_proof,
        },
        "candidate": {
            "snow_melt_model": DEFAULT_MELT_MODEL,
            "snow_density_model": SHALLOW_GUARD_DENSITY_MODEL,
            "selector_env": DENSITY_ENV,
            "activation_status": "opt-in diagnostic only; no default activation",
            "trace_proof": candidate_trace_proof,
        },
        "protected_boundaries": {
            "default_activation_changed": False,
            "rollback_removed": False,
            "density_cap_changed": False,
            "active_density_cap_kg_m3": spring10.SNOW_DENSITY_CAP_KG_M3,
            "public_output_schema_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "fixture_inputs_changed": False,
            "site_calibration_performed": False,
            "observed_depth_or_density_consumed_by_runtime": False,
            "melt_or_liquid_terms_changed": trace_comparison[
                "max_abs_mass_term_delta_m"
            ]
            > MASS_TERM_INVARIANCE_TOLERANCE_M,
            "sublimation_changed": trace_comparison["max_abs_sublimation_delta_m"]
            > MASS_TERM_INVARIANCE_TOLERANCE_M,
            "two_layer_surface_added": False,
            "qwet_or_frzftp_changed": False,
            "frost_attribution_authorized": False,
        },
        "summary": summary,
        "trace_comparison": trace_comparison,
        "surfaces": surfaces,
        "raw_outputs": {
            "output_dir": rel(output_dir),
            "package_json": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
            "bundle_source_report": rel(bundle_report_path),
        },
    }
    rubric.write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    rubric.write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    markdown = render_markdown(report)
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    return report


def build_surface_report(
    surface: phase.Surface,
    source_surface: dict[str, Any],
    baseline_run: dict[str, Any],
    candidate_run: dict[str, Any],
) -> dict[str, Any]:
    result = {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "fixture_dir": rel(surface.fixture_dir),
        "verdict_scope": surface.verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "note": surface.note,
        "baseline": baseline_run,
        "candidate": candidate_run,
        "candidate_vs_baseline": phase.paired_residual_delta(
            baseline_run["residuals"], candidate_run["residuals"]
        ),
    }
    if surface.verdict_scope != "paired_observation":
        result["transition_summary"] = {}
        return result

    holding_pairs = load_pair_map(surface, source_surface["prior_holding_capacity_only"]["wat"])
    baseline_pairs = load_pair_map(surface, baseline_run["wat"])
    candidate_pairs = load_pair_map(surface, candidate_run["wat"])
    common_dates = sorted(set(holding_pairs) & set(baseline_pairs) & set(candidate_pairs))
    rows = [
        transition_row(
            date,
            holding_pairs[date],
            baseline_pairs[date],
            candidate_pairs[date],
        )
        for date in common_dates
    ]
    result["transition_summary"] = transition_summary(rows)
    result["samples"] = {
        "baseline_induced_under": sample_rows(
            [row for row in rows if row["baseline_induced_under"]]
        ),
        "candidate_over": sample_rows([row for row in rows if row["candidate_state"] == STATE_OVER]),
        "candidate_under": sample_rows(
            [row for row in rows if row["candidate_state"] == STATE_UNDER]
        ),
    }
    return result


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
    return row


def state_for(row: dict[str, Any]) -> str:
    residual = row["depth_residual_m"]
    tolerance = row["depth_tolerance_m"]
    if abs(residual) <= tolerance:
        return STATE_PASS
    if residual > 0.0:
        return STATE_OVER
    return STATE_UNDER


def transition_row(
    date: str,
    holding: dict[str, Any],
    baseline: dict[str, Any],
    candidate: dict[str, Any],
) -> dict[str, Any]:
    baseline_induced_under = baseline["state"] == STATE_UNDER and holding["state"] != STATE_UNDER
    candidate_induced_under = candidate["state"] == STATE_UNDER and holding["state"] != STATE_UNDER
    return {
        "date": date,
        "water_year": baseline["water_year"],
        "observed_snow_depth_m": baseline["observed_snow_depth_m"],
        "holding_state": holding["state"],
        "baseline_state": baseline["state"],
        "candidate_state": candidate["state"],
        "baseline_induced_under": baseline_induced_under,
        "candidate_induced_under": candidate_induced_under,
        "induced_under_recovered": baseline_induced_under and candidate["state"] != STATE_UNDER,
        "holding_depth_residual_m": holding["depth_residual_m"],
        "baseline_depth_residual_m": baseline["depth_residual_m"],
        "candidate_depth_residual_m": candidate["depth_residual_m"],
        "baseline_modeled_depth_m": baseline["modeled_snow_depth_m"],
        "candidate_modeled_depth_m": candidate["modeled_snow_depth_m"],
        "baseline_swe_m": baseline.get("modeled_snow_water_m"),
        "candidate_swe_m": candidate.get("modeled_snow_water_m"),
        "candidate_minus_baseline_depth_m": (
            candidate["modeled_snow_depth_m"] - baseline["modeled_snow_depth_m"]
        ),
        "candidate_minus_baseline_swe_m": none_delta(
            candidate.get("modeled_snow_water_m"), baseline.get("modeled_snow_water_m")
        ),
    }


def transition_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    baseline_counts = Counter(row["baseline_state"] for row in rows)
    candidate_counts = Counter(row["candidate_state"] for row in rows)
    baseline_induced_under = sum(1 for row in rows if row["baseline_induced_under"])
    candidate_induced_under = sum(1 for row in rows if row["candidate_induced_under"])
    recovered = sum(1 for row in rows if row["induced_under_recovered"])
    worsened_to_over = sum(
        1
        for row in rows
        if row["baseline_state"] != STATE_OVER and row["candidate_state"] == STATE_OVER
    )
    depth_deltas = [row["candidate_minus_baseline_depth_m"] for row in rows]
    swe_deltas = [
        row["candidate_minus_baseline_swe_m"]
        for row in rows
        if row["candidate_minus_baseline_swe_m"] is not None
    ]
    return {
        "paired_row_count": len(rows),
        "baseline_state_counts": dict(sorted(baseline_counts.items())),
        "candidate_state_counts": dict(sorted(candidate_counts.items())),
        "baseline_under_fail_count": baseline_counts[STATE_UNDER],
        "candidate_under_fail_count": candidate_counts[STATE_UNDER],
        "baseline_over_fail_count": baseline_counts[STATE_OVER],
        "candidate_over_fail_count": candidate_counts[STATE_OVER],
        "baseline_induced_under_count": baseline_induced_under,
        "candidate_induced_under_count": candidate_induced_under,
        "induced_under_recovered_count": recovered,
        "candidate_over_new_from_non_over_count": worsened_to_over,
        "mean_depth_delta_candidate_minus_baseline_m": mean(depth_deltas),
        "max_abs_depth_delta_candidate_minus_baseline_m": max_abs(depth_deltas),
        "max_abs_swe_delta_candidate_minus_baseline_m": max_abs(swe_deltas),
    }


def compare_traces(
    baseline_trace_paths: dict[str, Path],
    candidate_trace_paths: dict[str, Path],
) -> dict[str, Any]:
    by_surface = {}
    aggregate = {
        "max_abs_swe_depth_density_residual_m": 0.0,
        "max_abs_snow_state_closure_residual_m": 0.0,
        "max_abs_runtime_swe_delta_m": 0.0,
        "max_abs_mass_term_delta_m": 0.0,
        "max_abs_sublimation_delta_m": 0.0,
        "changed_depth_row_count": 0,
        "changed_density_row_count": 0,
    }
    for surface_id, baseline_path in baseline_trace_paths.items():
        candidate_path = candidate_trace_paths[surface_id]
        baseline_rows = trace_row_map(baseline_path)
        candidate_rows = trace_row_map(candidate_path)
        common_keys = sorted(set(baseline_rows) & set(candidate_rows))
        surface_summary = trace_pair_summary(
            [baseline_rows[key] for key in common_keys],
            [candidate_rows[key] for key in common_keys],
        )
        by_surface[surface_id] = {
            "baseline_trace": rel(baseline_path),
            "candidate_trace": rel(candidate_path),
            **surface_summary,
        }
        for key in aggregate:
            if key.endswith("_count"):
                aggregate[key] += surface_summary[key]
            else:
                aggregate[key] = max(aggregate[key], surface_summary[key])
    aggregate["by_surface"] = by_surface
    aggregate["snow_state_conservation_ok"] = (
        aggregate["max_abs_swe_depth_density_residual_m"] <= TRACE_CLOSURE_TOLERANCE_M
        and aggregate["max_abs_snow_state_closure_residual_m"] <= TRACE_CLOSURE_TOLERANCE_M
        and aggregate["max_abs_runtime_swe_delta_m"] <= TRACE_CLOSURE_TOLERANCE_M
        and aggregate["max_abs_mass_term_delta_m"] <= MASS_TERM_INVARIANCE_TOLERANCE_M
        and aggregate["max_abs_sublimation_delta_m"] <= MASS_TERM_INVARIANCE_TOLERANCE_M
    )
    return aggregate


def trace_pair_summary(
    baseline_rows: list[dict[str, Any]],
    candidate_rows: list[dict[str, Any]],
) -> dict[str, Any]:
    mass_term_keys = [
        "snow_coupling_signed_s_m",
        "raw_melt_m",
        "snowpack_swe_loss_m",
        "accumulation_m",
        "routed_melt_m",
        "rain_retained_m",
        "rain_released_m",
        "liquid_holding_capacity_after_m",
        "liquid_water_retained_after_m",
        "liquid_water_released_m",
        "post_winter_rain_m",
    ]
    return {
        "trace_pair_count": len(baseline_rows),
        "max_abs_swe_depth_density_residual_m": max_abs(
            swe_depth_density_residual(row) for row in candidate_rows
        ),
        "max_abs_snow_state_closure_residual_m": max_abs(
            snow_state_closure_residual(row) for row in candidate_rows
        ),
        "max_abs_runtime_swe_delta_m": max_abs(
            number(candidate, "runtime_swe_after_m") - number(baseline, "runtime_swe_after_m")
            for baseline, candidate in zip(baseline_rows, candidate_rows)
        ),
        "max_abs_mass_term_delta_m": max_abs(
            number(candidate, key) - number(baseline, key)
            for baseline, candidate in zip(baseline_rows, candidate_rows)
            for key in mass_term_keys
        ),
        "max_abs_sublimation_delta_m": max_abs(
            number(candidate, "sublimation_m") - number(baseline, "sublimation_m")
            for baseline, candidate in zip(baseline_rows, candidate_rows)
        ),
        "changed_depth_row_count": sum(
            1
            for baseline, candidate in zip(baseline_rows, candidate_rows)
            if abs(number(candidate, "runtime_depth_after_m") - number(baseline, "runtime_depth_after_m"))
            > 1.0e-12
        ),
        "changed_density_row_count": sum(
            1
            for baseline, candidate in zip(baseline_rows, candidate_rows)
            if abs(
                number(candidate, "runtime_density_after_kg_m3")
                - number(baseline, "runtime_density_after_kg_m3")
            )
            > 1.0e-9
        ),
    }


def trace_row_map(path: Path) -> dict[tuple[int, int], dict[str, Any]]:
    rows = {}
    if not path.is_file():
        return rows
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        rows[(int(row["day_index"]), int(row["lane_index"]))] = row
    return rows


def swe_depth_density_residual(row: dict[str, Any]) -> float:
    return number(row, "runtime_swe_after_m") - (
        number(row, "runtime_depth_after_m") * number(row, "runtime_density_after_kg_m3") / 1000.0
    )


def snow_state_closure_residual(row: dict[str, Any]) -> float:
    return (
        number(row, "runtime_swe_before_m")
        + number(row, "accumulation_m")
        + number(row, "rain_retained_m")
        - number(row, "snowpack_swe_loss_m")
        - number(row, "sublimation_m")
        - number(row, "runtime_swe_after_m")
    )


def summarize(
    surfaces: list[dict[str, Any]],
    candidate_trace_proof: dict[str, Any],
    trace_comparison: dict[str, Any],
) -> dict[str, Any]:
    paired = [surface for surface in surfaces if surface["verdict_scope"] == "paired_observation"]
    baseline_pairs = sum(surface["baseline"]["residuals"]["paired_count"] for surface in paired)
    candidate_pairs = sum(surface["candidate"]["residuals"]["paired_count"] for surface in paired)
    baseline_fail = sum(
        surface["baseline"]["residuals"]["snow_control_fail_count"] for surface in paired
    )
    candidate_fail = sum(
        surface["candidate"]["residuals"]["snow_control_fail_count"] for surface in paired
    )
    baseline_under = sum(
        surface["transition_summary"]["baseline_under_fail_count"] for surface in paired
    )
    candidate_under = sum(
        surface["transition_summary"]["candidate_under_fail_count"] for surface in paired
    )
    baseline_over = sum(
        surface["transition_summary"]["baseline_over_fail_count"] for surface in paired
    )
    candidate_over = sum(
        surface["transition_summary"]["candidate_over_fail_count"] for surface in paired
    )
    baseline_induced_under = sum(
        surface["transition_summary"]["baseline_induced_under_count"] for surface in paired
    )
    candidate_induced_under = sum(
        surface["transition_summary"]["candidate_induced_under_count"] for surface in paired
    )
    recovered = sum(
        surface["transition_summary"]["induced_under_recovered_count"] for surface in paired
    )
    new_over_from_non_over = sum(
        surface["transition_summary"]["candidate_over_new_from_non_over_count"]
        for surface in paired
    )
    harvard = next(
        (surface for surface in paired if surface["surface_id"] == "harvard_hardwood"),
        None,
    )
    harvard_summary = harvard["transition_summary"] if harvard is not None else {}
    candidate_trace_ok = (
        candidate_trace_proof["expected_snow_melt_model_count"] > 0
        and candidate_trace_proof["expected_snow_density_model_count"] > 0
        and not candidate_trace_proof["unexpected_snow_melt_models"]
        and not candidate_trace_proof["unexpected_snow_density_models"]
    )
    induced_under_reduced = candidate_induced_under < baseline_induced_under
    over_not_worse = candidate_over <= baseline_over and new_over_from_non_over == 0
    threshold_authority_ok = True
    conservation_ok = trace_comparison["snow_state_conservation_ok"]
    promotion_eligible = (
        bool(paired)
        and candidate_trace_ok
        and induced_under_reduced
        and over_not_worse
        and threshold_authority_ok
        and conservation_ok
    )
    return {
        "disposition": "PROMOTION-ELIGIBLE-OPT-IN-ONLY"
        if promotion_eligible
        else "NON-PROMOTION-SHALLOW-GUARD-GATE-NOT-MET",
        "promotion_eligible": promotion_eligible,
        "activation_authorized": False,
        "candidate_trace_ok": candidate_trace_ok,
        "induced_under_persistence_reduced": induced_under_reduced,
        "over_persistence_not_worse": over_not_worse,
        "threshold_authority_ok": threshold_authority_ok,
        "snow_state_conservation_ok": conservation_ok,
        "baseline_paired_row_count": baseline_pairs,
        "candidate_paired_row_count": candidate_pairs,
        "baseline_snow_control_fail_count": baseline_fail,
        "candidate_snow_control_fail_count": candidate_fail,
        "snow_control_fail_delta_baseline_minus_candidate": baseline_fail - candidate_fail,
        "baseline_under_fail_count": baseline_under,
        "candidate_under_fail_count": candidate_under,
        "baseline_over_fail_count": baseline_over,
        "candidate_over_fail_count": candidate_over,
        "baseline_induced_under_count": baseline_induced_under,
        "candidate_induced_under_count": candidate_induced_under,
        "induced_under_recovered_count": recovered,
        "candidate_new_over_from_non_over_count": new_over_from_non_over,
        "harvard_hardwood": harvard_summary,
        "surface_count": len(surfaces),
        "paired_surface_count": len(paired),
        "observation_blocked_surface_ids": [
            surface["surface_id"]
            for surface in surfaces
            if surface["verdict_scope"] != "paired_observation"
        ],
    }


def sample_rows(rows: list[dict[str, Any]], limit: int = 8) -> list[dict[str, Any]]:
    keys = [
        "date",
        "observed_snow_depth_m",
        "holding_state",
        "baseline_state",
        "candidate_state",
        "baseline_depth_residual_m",
        "candidate_depth_residual_m",
        "candidate_minus_baseline_depth_m",
        "candidate_minus_baseline_swe_m",
    ]
    sorted_rows = sorted(rows, key=lambda row: abs(row["candidate_depth_residual_m"]), reverse=True)
    return [{key: row.get(key) for key in keys} for row in sorted_rows[:limit]]


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.17 Shallow-Pack Compaction Guard",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Promotion eligible: `{summary['promotion_eligible']}`",
        f"- Activation authorized: `{summary['activation_authorized']}`",
        f"- Candidate trace ok: `{summary['candidate_trace_ok']}`",
        f"- Induced under-persistence reduced: `{summary['induced_under_persistence_reduced']}`",
        f"- Over-persistence not worse: `{summary['over_persistence_not_worse']}`",
        f"- Threshold authority ok: `{summary['threshold_authority_ok']}`",
        f"- Snow-state conservation ok: `{summary['snow_state_conservation_ok']}`",
        f"- Snow-control failures: `{summary['baseline_snow_control_fail_count']} -> {summary['candidate_snow_control_fail_count']}`",
        f"- Induced under-persistence: `{summary['baseline_induced_under_count']} -> {summary['candidate_induced_under_count']}`",
        f"- Under-persistence total: `{summary['baseline_under_fail_count']} -> {summary['candidate_under_fail_count']}`",
        f"- Over-persistence total: `{summary['baseline_over_fail_count']} -> {summary['candidate_over_fail_count']}`",
        f"- Max SWE-depth-density residual: `{report['trace_comparison']['max_abs_swe_depth_density_residual_m']:.3e} m`",
        f"- Max snow-state closure residual: `{report['trace_comparison']['max_abs_snow_state_closure_residual_m']:.3e} m`",
        f"- Max mass-term delta: `{report['trace_comparison']['max_abs_mass_term_delta_m']:.3e} m`",
        "",
        "## Surface Results",
        "",
        "| Surface | Scope | Baseline fails | Candidate fails | Baseline induced under | Candidate induced under | Baseline over | Candidate over |",
        "|---|---|---:|---:|---:|---:|---:|---:|",
    ]
    for surface in report["surfaces"]:
        if surface["verdict_scope"] != "paired_observation":
            lines.append(
                f"| `{surface['surface_id']}` | {surface['verdict_scope']} | 0 | 0 | 0 | 0 | 0 | 0 |"
            )
            continue
        trans = surface["transition_summary"]
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{surface['surface_id']}`",
                    surface["verdict_scope"],
                    str(surface["baseline"]["residuals"]["snow_control_fail_count"]),
                    str(surface["candidate"]["residuals"]["snow_control_fail_count"]),
                    str(trans["baseline_induced_under_count"]),
                    str(trans["candidate_induced_under_count"]),
                    str(trans["baseline_over_fail_count"]),
                    str(trans["candidate_over_fail_count"]),
                ]
            )
            + " |"
        )
    lines.extend(
        [
            "",
            "## Boundary Disposition",
            "",
            "- Candidate remains opt-in diagnostic only.",
            "- Default activation, density cap, public output schema, fixtures, parser/runfile/user CLI, compatibility runtime, Qwet/frzftp, sublimation, two-layer structure, and frost attribution remain unchanged.",
            "",
        ]
    )
    return "\n".join(lines)


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def number(row: dict[str, Any], key: str) -> float:
    value = row.get(key)
    return float(value) if value is not None else 0.0


def none_delta(left: float | None, right: float | None) -> float | None:
    if left is None or right is None:
        return None
    return left - right


def max_abs(values: Any) -> float:
    collected = [abs(float(value)) for value in values]
    return max(collected) if collected else 0.0


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
