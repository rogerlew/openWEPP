#!/usr/bin/env python3
"""Adjudicate SNOWDENSITY-10.3.16 Stage A open-surface sublimation."""

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
import phase_partition_snowdepth_adjudication as phase  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402
import spring_pack_depletion_compaction_adjudication as spring10  # noqa: E402


SCHEMA = "snowdensity10-3-16-open-surface-ablation-stage-a-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-073 OBL-SNOWFREEZE-P-048"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_16_open_surface_ablation_stage_a"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
ARTIFACT_STEM = "open-surface-ablation-stage-a"

MELT_ENV = active15.MELT_ENV
DENSITY_ENV = active15.DENSITY_ENV
TRACE_ENV = active15.TRACE_ENV
DEFAULT_MELT_MODEL = active15.DEFAULT_MELT_MODEL
DEFAULT_DENSITY_MODEL = active15.DEFAULT_DENSITY_MODEL
STAGE_A_MELT_MODEL = "coe_open_sublimation_stage_a_v1"
OPEN_SURFACE_IDS = {"harvard_open", "sleepers_south_field"}
MAX_DAILY_LANE_SUBLIMATION_M = 0.03
MAX_MEAN_POSITIVE_SUBLIMATION_M = 0.01
TRACE_CLOSURE_TOLERANCE_M = 1.0e-9


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = adjudicate(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")

    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    default_trace_paths: dict[str, Path] = {}
    candidate_trace_paths: dict[str, Path] = {}
    surfaces = []
    for surface in phase.SURFACES:
        if surface.surface_id not in OPEN_SURFACE_IDS:
            continue
        default_trace = output_dir / "traces" / f"{surface.surface_id}_activated_default.jsonl"
        candidate_trace = output_dir / "traces" / f"{surface.surface_id}_stage_a.jsonl"
        default_trace_paths[surface.surface_id] = default_trace
        candidate_trace_paths[surface.surface_id] = candidate_trace
        default_run = active15.run_and_analyze(
            surface=surface,
            scenario="activated_default",
            output_dir=output_dir,
            hill_binary=hill_binary,
            trace_path=default_trace,
            env_updates={
                MELT_ENV: None,
                DENSITY_ENV: None,
                TRACE_ENV: str(default_trace),
            },
            run_model=run_models,
        )
        candidate_run = active15.run_and_analyze(
            surface=surface,
            scenario="stage_a_sublimation",
            output_dir=output_dir,
            hill_binary=hill_binary,
            trace_path=candidate_trace,
            env_updates={
                MELT_ENV: STAGE_A_MELT_MODEL,
                DENSITY_ENV: DEFAULT_DENSITY_MODEL,
                TRACE_ENV: str(candidate_trace),
            },
            run_model=run_models,
        )
        surfaces.append(
            {
                "surface_id": surface.surface_id,
                "site_group": surface.site_group,
                "cover": surface.cover,
                "fixture_dir": active15.rel(surface.fixture_dir),
                "verdict_scope": surface.verdict_scope,
                "observation_source": surface.observation_source,
                "observation_kind": surface.observation_kind,
                "activated_default": default_run,
                "stage_a": candidate_run,
                "tail_delta": tail_delta(default_run, candidate_run),
            }
        )

    default_trace_proof = active15.build_trace_proof(
        default_trace_paths,
        expected_melt_model=DEFAULT_MELT_MODEL,
        expected_density_model=DEFAULT_DENSITY_MODEL,
    )
    candidate_trace_proof = active15.build_trace_proof(
        candidate_trace_paths,
        expected_melt_model=STAGE_A_MELT_MODEL,
        expected_density_model=DEFAULT_DENSITY_MODEL,
    )
    trace_summary = summarize_traces(candidate_trace_paths)
    summary = summarize(surfaces, candidate_trace_proof, trace_summary)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "candidate": {
            "snow_melt_model": STAGE_A_MELT_MODEL,
            "snow_density_model": DEFAULT_DENSITY_MODEL,
            "selector_env": MELT_ENV,
            "activation_status": "opt-in diagnostic only; no default activation",
        },
        "baseline": {
            "snow_melt_model": DEFAULT_MELT_MODEL,
            "snow_density_model": DEFAULT_DENSITY_MODEL,
            "selector_state": "absent/empty env selects activated default",
            "trace_proof": default_trace_proof,
        },
        "trace_proof": candidate_trace_proof,
        "trace_summary": trace_summary,
        "protected_boundaries": {
            "default_activation_changed": False,
            "rollback_removed": False,
            "density_cap_changed": False,
            "public_output_schema_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "fixture_inputs_changed": False,
            "site_calibration_performed": False,
            "two_layer_surface_added": False,
            "qwet_or_frzftp_changed": False,
            "frost_attribution_authorized": False,
        },
        "summary": summary,
        "surfaces": surfaces,
        "raw_outputs": {
            "output_dir": active15.rel(output_dir),
            "package_json": active15.rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": active15.rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
        },
    }
    rubric.write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    rubric.write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    markdown = render_markdown(report)
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    return report


def tail_delta(default_run: dict[str, Any], candidate_run: dict[str, Any]) -> dict[str, Any]:
    default_counts = class_counts(default_run)
    candidate_counts = class_counts(candidate_run)
    return {
        "default_cap_limited_count": default_counts[spring10.CLASS_CAP_LIMITED_DEPLETION],
        "candidate_cap_limited_count": candidate_counts[spring10.CLASS_CAP_LIMITED_DEPLETION],
        "cap_limited_delta": default_counts[spring10.CLASS_CAP_LIMITED_DEPLETION]
        - candidate_counts[spring10.CLASS_CAP_LIMITED_DEPLETION],
        "default_under_persistence_count": default_counts[spring10.CLASS_UNDER_PERSISTENCE],
        "candidate_under_persistence_count": candidate_counts[spring10.CLASS_UNDER_PERSISTENCE],
        "under_persistence_delta": candidate_counts[spring10.CLASS_UNDER_PERSISTENCE]
        - default_counts[spring10.CLASS_UNDER_PERSISTENCE],
        "default_patchy_count": default_counts[spring10.CLASS_PATCHY_DEPLETION],
        "candidate_patchy_count": candidate_counts[spring10.CLASS_PATCHY_DEPLETION],
    }


def class_counts(run: dict[str, Any]) -> Counter[str]:
    return Counter(run["march_april_cap_adjudication"]["class_counts_failures"])


def summarize(
    surfaces: list[dict[str, Any]],
    candidate_trace_proof: dict[str, Any],
    trace_summary: dict[str, Any],
) -> dict[str, Any]:
    default_cap = sum(row["tail_delta"]["default_cap_limited_count"] for row in surfaces)
    candidate_cap = sum(row["tail_delta"]["candidate_cap_limited_count"] for row in surfaces)
    default_under = sum(row["tail_delta"]["default_under_persistence_count"] for row in surfaces)
    candidate_under = sum(row["tail_delta"]["candidate_under_persistence_count"] for row in surfaces)
    trace_ok = (
        candidate_trace_proof["expected_snow_melt_model_count"] > 0
        and candidate_trace_proof["expected_snow_density_model_count"] > 0
        and not candidate_trace_proof["unexpected_snow_melt_models"]
        and not candidate_trace_proof["unexpected_snow_density_models"]
    )
    cap_tail_reduced = candidate_cap < default_cap
    under_not_worse = candidate_under <= default_under
    magnitude_ok = (
        trace_summary["positive_sublimation_row_count"] > 0
        and trace_summary["max_daily_lane_sublimation_m"] <= MAX_DAILY_LANE_SUBLIMATION_M
        and trace_summary["mean_positive_daily_lane_sublimation_m"]
        <= MAX_MEAN_POSITIVE_SUBLIMATION_M
    )
    conservation_ok = (
        trace_summary["max_abs_snow_state_closure_residual_m"] <= TRACE_CLOSURE_TOLERANCE_M
        and trace_summary["max_abs_routed_liquid_excludes_sublimation_residual_m"]
        <= TRACE_CLOSURE_TOLERANCE_M
    )
    promotion_eligible = trace_ok and cap_tail_reduced and under_not_worse and magnitude_ok and conservation_ok
    return {
        "disposition": "PROMOTION-ELIGIBLE-OPT-IN-ONLY"
        if promotion_eligible
        else "NON-PROMOTION-STAGE-A-GATE-NOT-MET",
        "promotion_eligible": promotion_eligible,
        "activation_authorized": False,
        "candidate_trace_ok": trace_ok,
        "cap_limited_tail_reduced": cap_tail_reduced,
        "under_persistence_not_worse": under_not_worse,
        "sublimation_magnitude_ok": magnitude_ok,
        "snow_state_conservation_ok": conservation_ok,
        "default_open_cap_limited_count": default_cap,
        "candidate_open_cap_limited_count": candidate_cap,
        "cap_limited_delta_default_minus_candidate": default_cap - candidate_cap,
        "default_open_under_persistence_count": default_under,
        "candidate_open_under_persistence_count": candidate_under,
        "under_persistence_delta_candidate_minus_default": candidate_under - default_under,
        "surface_count": len(surfaces),
        "surface_ids": [row["surface_id"] for row in surfaces],
        "magnitude_envelope": {
            "max_daily_lane_sublimation_m": MAX_DAILY_LANE_SUBLIMATION_M,
            "max_mean_positive_daily_lane_sublimation_m": MAX_MEAN_POSITIVE_SUBLIMATION_M,
            "basis": "provisional Marks/SNOBAL open exposed-site magnitude sanity envelope; constants not fixture-fitted",
        },
    }


def summarize_traces(trace_paths: dict[str, Path]) -> dict[str, Any]:
    rows = []
    by_surface = {}
    for surface_id, path in trace_paths.items():
        surface_rows = [row for row in iter_trace_rows(path)]
        by_surface[surface_id] = trace_rows_summary(surface_rows, path)
        rows.extend(surface_rows)
    total_sublimation = sum(number(row, "sublimation_m") for row in rows)
    positive = [number(row, "sublimation_m") for row in rows if number(row, "sublimation_m") > 0.0]
    return {
        "trace_row_count": len(rows),
        "positive_sublimation_row_count": len(positive),
        "total_sublimation_m": total_sublimation,
        "max_daily_lane_sublimation_m": max(positive) if positive else 0.0,
        "mean_positive_daily_lane_sublimation_m": mean(positive),
        "max_abs_snow_state_closure_residual_m": max_abs(
            snow_state_closure_residual(row) for row in rows
        ),
        "max_abs_routed_liquid_excludes_sublimation_residual_m": max_abs(
            routed_liquid_excludes_sublimation_residual(row) for row in rows
        ),
        "by_surface": by_surface,
    }


def trace_rows_summary(rows: list[dict[str, Any]], path: Path) -> dict[str, Any]:
    positive = [number(row, "sublimation_m") for row in rows if number(row, "sublimation_m") > 0.0]
    return {
        "trace": active15.rel(path),
        "trace_row_count": len(rows),
        "positive_sublimation_row_count": len(positive),
        "total_sublimation_m": sum(number(row, "sublimation_m") for row in rows),
        "max_daily_lane_sublimation_m": max(positive) if positive else 0.0,
        "mean_positive_daily_lane_sublimation_m": mean(positive),
        "max_abs_snow_state_closure_residual_m": max_abs(
            snow_state_closure_residual(row) for row in rows
        ),
        "max_abs_routed_liquid_excludes_sublimation_residual_m": max_abs(
            routed_liquid_excludes_sublimation_residual(row) for row in rows
        ),
    }


def iter_trace_rows(path: Path) -> list[dict[str, Any]]:
    if not path.is_file():
        return []
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def snow_state_closure_residual(row: dict[str, Any]) -> float:
    return (
        number(row, "runtime_swe_before_m")
        + number(row, "accumulation_m")
        + number(row, "rain_retained_m")
        - number(row, "snowpack_swe_loss_m")
        - number(row, "sublimation_m")
        - number(row, "runtime_swe_after_m")
    )


def routed_liquid_excludes_sublimation_residual(row: dict[str, Any]) -> float:
    return (
        number(row, "routed_melt_m")
        - number(row, "rain_released_m")
        - number(row, "snowpack_swe_loss_m")
    )


def number(row: dict[str, Any], key: str) -> float:
    value = row.get(key)
    return float(value) if value is not None else 0.0


def max_abs(values: Any) -> float:
    collected = [abs(float(value)) for value in values]
    return max(collected) if collected else 0.0


def mean(values: list[float]) -> float:
    return sum(values) / len(values) if values else 0.0


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.16 Open-Surface Ablation Stage A",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Promotion eligible: `{summary['promotion_eligible']}`",
        f"- Activation authorized: `{summary['activation_authorized']}`",
        f"- Candidate trace ok: `{summary['candidate_trace_ok']}`",
        f"- Cap-limited tail reduced: `{summary['cap_limited_tail_reduced']}`",
        f"- Under-persistence not worse: `{summary['under_persistence_not_worse']}`",
        f"- Sublimation magnitude ok: `{summary['sublimation_magnitude_ok']}`",
        f"- Snow-state conservation ok: `{summary['snow_state_conservation_ok']}`",
        f"- Cap-limited open tail: `{summary['default_open_cap_limited_count']} -> {summary['candidate_open_cap_limited_count']}`",
        f"- Under-persistence open tail: `{summary['default_open_under_persistence_count']} -> {summary['candidate_open_under_persistence_count']}`",
        f"- Total trace sublimation: `{report['trace_summary']['total_sublimation_m']:.6f} m`",
        f"- Max daily-lane sublimation: `{report['trace_summary']['max_daily_lane_sublimation_m']:.6f} m`",
        f"- Max snow-state closure residual: `{report['trace_summary']['max_abs_snow_state_closure_residual_m']:.3e} m`",
        "",
        "## Surface Results",
        "",
        "| Surface | Cap-limited default | Cap-limited Stage A | Under default | Under Stage A |",
        "|---|---:|---:|---:|---:|",
    ]
    for surface in report["surfaces"]:
        delta = surface["tail_delta"]
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{surface['surface_id']}`",
                    str(delta["default_cap_limited_count"]),
                    str(delta["candidate_cap_limited_count"]),
                    str(delta["default_under_persistence_count"]),
                    str(delta["candidate_under_persistence_count"]),
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
            "- Default activation, density cap, public output schema, fixtures, parser/runfile/user CLI, Qwet/frzftp, and frost attribution remain unchanged.",
            "",
        ]
    )
    return "\n".join(lines)


if __name__ == "__main__":
    raise SystemExit(main())
