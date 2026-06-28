#!/usr/bin/env python3
"""Adjudicate activation readiness for the best current snow-depth bundle."""

from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import json
import os
import subprocess
import sys
from collections import Counter
from pathlib import Path
from typing import Any, Iterator


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import march_april_residual_attribution as march_april  # noqa: E402
import observed_harness  # noqa: E402
import phase_partition_snowdepth_adjudication as phase  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402
import spring_pack_depletion_compaction_adjudication as spring10  # noqa: E402


SCHEMA = "snowdensity10-3-12-bundle-activation-adjudication-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-069 OBL-SNOWFREEZE-P-044"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_12_bundle_activation_adjudication"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_1038_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001"
    / "artifacts/liquid-holding-capacity-coupled-wat.json"
)
DEFAULT_10311_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001"
    / "artifacts/spring-compaction-densification-candidate.json"
)
MELT_ENV = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL"
DENSITY_ENV = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL"
TRACE_ENV = "OPENWEPP_R7H_SNOW_TRACE_PATH"
MELT_MODEL = "coe_liquid_holding_capacity_v1"
DENSITY_MODEL = "physics_bulk_density_compaction_v1"
REJECTED_DENSITY_MODEL = "physics_bulk_spring_densification_v1"
ARTIFACT_STEM = "bundle-activation-adjudication"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--prior-10-3-8-report", type=Path, default=DEFAULT_1038_REPORT)
    parser.add_argument("--prior-10-3-11-report", type=Path, default=DEFAULT_10311_REPORT)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = adjudicate(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        prior_1038_report=args.prior_10_3_8_report.resolve(),
        prior_10311_report=args.prior_10_3_11_report.resolve(),
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    prior_1038_report: Path,
    prior_10311_report: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")

    prior_1038 = read_json(prior_1038_report)
    prior_10311 = read_json(prior_10311_report)
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    prior_1038_by_surface = {item["surface_id"]: item for item in prior_1038["surfaces"]}
    prior_10311_by_surface = {item["surface_id"]: item for item in prior_10311["surfaces"]}
    surfaces = []
    trace_paths: dict[str, Path] = {}
    for surface in phase.SURFACES:
        trace_path = output_dir / "traces" / f"{surface.surface_id}_{MELT_MODEL}_{DENSITY_MODEL}.jsonl"
        trace_paths[surface.surface_id] = trace_path
        bundle = run_and_analyze(
            surface=surface,
            output_dir=output_dir,
            hill_binary=hill_binary,
            trace_path=trace_path,
            run_model=run_models,
        )
        surfaces.append(
            build_surface_report(
                surface=surface,
                bundle=bundle,
                prior_1038_surface=prior_1038_by_surface[surface.surface_id],
                prior_10311_surface=prior_10311_by_surface[surface.surface_id],
            )
        )

    trace_proof = build_trace_proof(trace_paths)
    summary = summarize(surfaces, prior_1038, prior_10311, trace_proof)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "bundle": {
            "snow_melt_model": MELT_MODEL,
            "snow_density_model": DENSITY_MODEL,
            "melt_env": MELT_ENV,
            "density_env": DENSITY_ENV,
            "trace_env": TRACE_ENV,
            "trace_proof": trace_proof,
        },
        "source_reports": {
            "prior_10_3_8_coupled_wat": rel(prior_1038_report),
            "prior_10_3_11_spring_densification": rel(prior_10311_report),
        },
        "protected_boundaries": {
            "default_activation_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "density_cap_changed": False,
            "observed_depth_or_density_consumed_by_runtime": False,
            "new_process_physics_added": False,
            "melt_radiation_canopy_phase_rain_heat_longwave_frost_changed": False,
            "qwet_or_frzftp_changed": False,
            "compatibility_runtime_changed": False,
        },
        "summary": summary,
        "surfaces": surfaces,
        "raw_outputs": {
            "output_dir": rel(output_dir),
            "package_json": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
        },
    }
    rubric.write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    rubric.write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(render_markdown(report), encoding="utf-8")
    return report


def run_and_analyze(
    surface: phase.Surface,
    output_dir: Path,
    hill_binary: Path,
    trace_path: Path,
    run_model: bool,
) -> dict[str, Any]:
    run_dir = output_dir / "runs" / surface.surface_id / "bundle"
    run_dir.mkdir(parents=True, exist_ok=True)
    trace_path.parent.mkdir(parents=True, exist_ok=True)
    if trace_path.exists() and run_model:
        trace_path.unlink()
    run_stem = observed_harness.discover_run_stem(surface.fixture_dir)
    run_id = f"{surface.surface_id}_bundle"
    runfile_path = run_dir / f"{run_id}.run"
    observed_harness.write_runfile(runfile_path, surface.fixture_dir, run_stem, run_dir, run_id)
    command = observed_harness.cli_command(
        hill_binary,
        surface.fixture_dir,
        runfile_path,
        run_dir,
        "direct-production-executor",
    )
    env = {
        MELT_ENV: MELT_MODEL,
        DENSITY_ENV: DENSITY_MODEL,
        TRACE_ENV: str(trace_path),
    }
    if run_model:
        with scoped_env(env):
            completed = subprocess.run(
                command,
                cwd=REPO_ROOT,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )
        (run_dir / "openwepp-cli-hill.stdout").write_text(completed.stdout, encoding="utf-8")
        (run_dir / "openwepp-cli-hill.stderr").write_text(completed.stderr, encoding="utf-8")
        if completed.returncode != 0:
            raise RuntimeError(
                f"openwepp-cli-hill failed for {surface.surface_id}/bundle "
                f"with exit code {completed.returncode}; see {run_dir}"
            )

    wat_path = run_dir / f"{run_id}.wat.parquet"
    if not wat_path.is_file():
        raise FileNotFoundError(f"expected WAT parquet output {wat_path}")
    modeled = observed_harness.load_modeled_wat(wat_path)
    observations = phase.load_observations(surface)
    pairs = phase.pair_observations(observations, modeled, surface.observation_kind)
    annotated_pairs = [march_april.annotate_pair(pair) for pair in pairs]
    march_april_pairs = [
        pair
        for pair in annotated_pairs
        if dt.date.fromisoformat(pair["date"]).month in march_april.MARCH_APRIL_MONTHS
    ]
    spring_evaluations = [spring10.evaluate_pair(pair) for pair in march_april_pairs]
    return {
        "model": {
            "snow_melt_model": MELT_MODEL,
            "snow_density_model": DENSITY_MODEL,
        },
        "env": env,
        "command": [str(value) for value in command],
        "run_dir": rel(run_dir),
        "runfile": rel(runfile_path),
        "wat": rel(wat_path),
        "trace": rel(trace_path),
        "observation_row_count": len(observations),
        "paired_row_count": len(pairs),
        "residuals": phase.residual_summary(pairs),
        "wat_summary": phase.wat_summary(modeled),
        "remaining_failures": failure_profile(annotated_pairs),
        "march_april_cap_adjudication": spring10.evaluation_summary(spring_evaluations),
        "failure_samples": sample_failures(annotated_pairs),
    }


def build_surface_report(
    surface: phase.Surface,
    bundle: dict[str, Any],
    prior_1038_surface: dict[str, Any],
    prior_10311_surface: dict[str, Any],
) -> dict[str, Any]:
    holding = prior_1038_surface["candidate"]
    spring = prior_10311_surface["candidate"]
    return {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "fixture_dir": rel(surface.fixture_dir),
        "verdict_scope": surface.verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "note": surface.note,
        "prior_default": prior_1038_surface["default"],
        "prior_holding_capacity_only": holding,
        "prior_spring_densification": spring,
        "bundle": bundle,
        "bundle_vs_holding_capacity_only": phase.paired_residual_delta(
            holding["residuals"], bundle["residuals"]
        ),
        "bundle_vs_spring_densification": phase.paired_residual_delta(
            spring["residuals"], bundle["residuals"]
        ),
    }


def failure_profile(pairs: list[dict[str, Any]]) -> dict[str, Any]:
    failures = [pair for pair in pairs if pair["depth_fail"]]
    by_month = Counter(dt.date.fromisoformat(pair["date"]).month for pair in failures)
    by_sign = Counter(residual_sign(pair["depth_residual_m"]) for pair in failures)
    by_water_year = Counter(str(pair["water_year"]) for pair in failures)
    return {
        "failure_count": len(failures),
        "counts_by_month": dict(sorted(by_month.items())),
        "counts_by_residual_sign": dict(sorted(by_sign.items())),
        "counts_by_water_year": dict(sorted(by_water_year.items())),
        "mean_signed_depth_residual_m": mean([pair["depth_residual_m"] for pair in failures]),
        "mean_abs_depth_residual_m": mean([abs(pair["depth_residual_m"]) for pair in failures]),
    }


def sample_failures(pairs: list[dict[str, Any]], limit: int = 8) -> list[dict[str, Any]]:
    samples = []
    for pair in pairs:
        if not pair["depth_fail"]:
            continue
        samples.append(
            {
                "date": pair["date"],
                "water_year": pair["water_year"],
                "observed_snow_depth_m": pair["observed_snow_depth_m"],
                "modeled_snow_depth_m": pair["modeled_snow_depth_m"],
                "modeled_snow_water_m": pair.get("modeled_snow_water_m"),
                "depth_residual_m": pair["depth_residual_m"],
                "depth_tolerance_m": pair["depth_tolerance_m"],
                "attribution": pair["attribution"],
            }
        )
        if len(samples) >= limit:
            break
    return samples


def summarize(
    surfaces: list[dict[str, Any]],
    prior_1038: dict[str, Any],
    prior_10311: dict[str, Any],
    trace_proof: dict[str, Any],
) -> dict[str, Any]:
    paired = [surface for surface in surfaces if surface["verdict_scope"] == "paired_observation"]
    blocked = [surface for surface in surfaces if surface["verdict_scope"] != "paired_observation"]
    paired_rows = sum(surface["bundle"]["residuals"]["paired_count"] for surface in paired)
    bundle_fail = sum(surface["bundle"]["residuals"]["snow_control_fail_count"] for surface in paired)
    holding_fail = prior_1038["summary"]["candidate_snow_control_fail_count"]
    default_fail = prior_1038["summary"]["default_snow_control_fail_count"]
    spring_fail = prior_10311["summary"]["candidate_snow_control_fail_count"]
    worse_vs_holding = [
        surface["surface_id"]
        for surface in paired
        if surface["bundle_vs_holding_capacity_only"]["fail_count_delta_default_minus_opt_in"] < 0
    ]
    bundle_trace_ok = (
        trace_proof["bundle_snow_melt_model_count"] > 0
        and trace_proof["bundle_snow_density_model_count"] > 0
    )
    march_counts = merge_counter(
        surface["bundle"]["march_april_cap_adjudication"]["class_counts_failures"]
        for surface in paired
    )
    residual_sign_counts = merge_counter(
        surface["bundle"]["remaining_failures"]["counts_by_residual_sign"] for surface in paired
    )
    month_counts = merge_counter(
        surface["bundle"]["remaining_failures"]["counts_by_month"] for surface in paired
    )

    policy_b_full_surface_scope = [
        "workspace regression/identity suite",
        "non-snow climate fixtures",
        "erosion and water-balance surfaces",
        "watershed routing surfaces",
    ]
    policy_b_snow_improves_default = bundle_fail < default_fail
    policy_b_full_surface_no_regression_evidence_present = False
    activation_ready = (
        bundle_trace_ok
        and paired_rows > 0
        and policy_b_snow_improves_default
        and not worse_vs_holding
        and policy_b_full_surface_no_regression_evidence_present
    )
    if not bundle_trace_ok:
        disposition = "HOLD-BUNDLE-TRACE-ABSENT"
        blocker = "DIRECT-SNOW-PARTITION-TRACE-ABSENT"
    elif activation_ready:
        disposition = "ACTIVATION-READY"
        blocker = None
    elif not policy_b_snow_improves_default:
        disposition = "RETIRE-OR-REWORK-BUNDLE"
        blocker = "BUNDLE-DOES-NOT-IMPROVE-CURRENT-DEFAULT"
    elif worse_vs_holding:
        disposition = "HOLD-OPT-IN-BUNDLE"
        blocker = "PAIRED-SURFACE-WORSENED-VS-HOLDING-CAPACITY"
    else:
        disposition = "HOLD-OPT-IN-BUNDLE"
        blocker = "POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING"

    return {
        "disposition": disposition,
        "blocker": blocker,
        "activation_policy": "POLICY-B",
        "activation_ready": activation_ready,
        "policy_b_zero_paired_snow_failures_required": False,
        "policy_b_gate_eligible_snow_strictly_better_than_default": policy_b_snow_improves_default,
        "policy_b_full_surface_no_regression_evidence_present": (
            policy_b_full_surface_no_regression_evidence_present
        ),
        "policy_b_full_surface_no_regression_scope": policy_b_full_surface_scope,
        "paired_snow_control_zero_failures": bundle_fail == 0,
        "frost_attribution_decoupled_from_activation": True,
        "frost_attribution_unblocked": False,
        "frost_attribution_blocker": (
            "SNOW-CONTROL-RESIDUALS-REMAIN" if bundle_fail > 0 else None
        ),
        "surface_count": len(surfaces),
        "paired_surface_count": len(paired),
        "observation_blocked_surface_count": len(blocked),
        "observation_blocked_surface_ids": [surface["surface_id"] for surface in blocked],
        "paired_row_count": paired_rows,
        "default_snow_control_fail_count": default_fail,
        "holding_capacity_only_snow_control_fail_count": holding_fail,
        "bundle_snow_control_fail_count": bundle_fail,
        "spring_densification_snow_control_fail_count": spring_fail,
        "fail_delta_default_minus_bundle": default_fail - bundle_fail,
        "fail_delta_holding_only_minus_bundle": holding_fail - bundle_fail,
        "fail_delta_bundle_minus_spring_densification": bundle_fail - spring_fail,
        "paired_surface_worse_vs_holding_count": len(worse_vs_holding),
        "paired_surface_worse_vs_holding_ids": worse_vs_holding,
        "bundle_failure_counts_by_month": dict(sorted(month_counts.items())),
        "bundle_failure_counts_by_residual_sign": dict(sorted(residual_sign_counts.items())),
        "bundle_march_april_cap_class_counts": dict(sorted(march_counts.items())),
    }


def build_trace_proof(trace_paths: dict[str, Path]) -> dict[str, Any]:
    by_surface = {}
    melt_count = 0
    density_count = 0
    for surface_id, path in trace_paths.items():
        counts = count_trace_models(path)
        by_surface[surface_id] = {"path": rel(path), "counts": counts}
        melt_count += counts["snow_melt_model"].get(MELT_MODEL, 0)
        density_count += counts["snow_density_model"].get(DENSITY_MODEL, 0)
    return {
        "bundle_snow_melt_model_count": melt_count,
        "bundle_snow_density_model_count": density_count,
        "trace_counts_by_surface": by_surface,
    }


def count_trace_models(path: Path) -> dict[str, dict[str, int]]:
    counts = {"snow_melt_model": {}, "snow_density_model": {}}
    if not path.is_file():
        return counts
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        for key in counts:
            value = str(row.get(key, "missing"))
            counts[key][value] = counts[key].get(value, 0) + 1
    return counts


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.12 Bundle Activation Adjudication",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Blocker: `{summary['blocker']}`",
        f"- Activation policy: `{summary['activation_policy']}`",
        f"- Activation ready: `{summary['activation_ready']}`",
        f"- Frost attribution unblocked: `{summary['frost_attribution_unblocked']}`",
        f"- Default failures: `{summary['default_snow_control_fail_count']}`",
        f"- Holding-capacity-only failures: `{summary['holding_capacity_only_snow_control_fail_count']}`",
        f"- Bundle failures: `{summary['bundle_snow_control_fail_count']}`",
        f"- Spring-densification failures: `{summary['spring_densification_snow_control_fail_count']}`",
        f"- Paired rows: `{summary['paired_row_count']}`",
        f"- Trace melt count: `{report['bundle']['trace_proof']['bundle_snow_melt_model_count']}`",
        f"- Trace density count: `{report['bundle']['trace_proof']['bundle_snow_density_model_count']}`",
        "",
        "## Activation Policy B",
        "",
        "- Zero paired snow-depth failures required for activation: "
        f"`{summary['policy_b_zero_paired_snow_failures_required']}`",
        "- Gate-eligible snow surfaces strictly better than current default: "
        f"`{summary['policy_b_gate_eligible_snow_strictly_better_than_default']}`",
        "- Full-surface no-regression evidence present: "
        f"`{summary['policy_b_full_surface_no_regression_evidence_present']}`",
        "- Full-surface no-regression scope: "
        f"`{summary['policy_b_full_surface_no_regression_scope']}`",
        "- Paired snow-control zero failures: "
        f"`{summary['paired_snow_control_zero_failures']}`",
        "- Frost attribution blocker: "
        f"`{summary['frost_attribution_blocker']}`",
        "",
        "## Remaining Failure Profile",
        "",
        f"- Counts by residual sign: `{summary['bundle_failure_counts_by_residual_sign']}`",
        f"- Counts by month: `{summary['bundle_failure_counts_by_month']}`",
        f"- March/April cap classes: `{summary['bundle_march_april_cap_class_counts']}`",
        "",
        "## Surface Results",
        "",
        "| Surface | Scope | Cover | Holding fails | Bundle fails | Spring fails | Bundle vs holding |",
        "|---|---|---|---:|---:|---:|---:|",
    ]
    for surface in report["surfaces"]:
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{surface['surface_id']}`",
                    surface["verdict_scope"],
                    surface["cover"],
                    str(surface["prior_holding_capacity_only"]["residuals"]["snow_control_fail_count"]),
                    str(surface["bundle"]["residuals"]["snow_control_fail_count"]),
                    str(surface["prior_spring_densification"]["residuals"]["snow_control_fail_count"]),
                    str(
                        surface["bundle_vs_holding_capacity_only"][
                            "fail_count_delta_default_minus_opt_in"
                        ]
                    ),
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
            "- Parser/runfile/user CLI selector added: `false`.",
            "- Fixture inputs changed: `false`.",
            "- Public output schema changed: `false`.",
            "- New process physics added: `false`.",
            "- Frost attribution remains blocked while snow-control residuals remain; "
            "this is separate from Policy-B default activation.",
            "",
        ]
    )
    return "\n".join(lines)


def residual_sign(value: float) -> str:
    if value > 0.0:
        return "MODELED_OVER_OBSERVED"
    if value < 0.0:
        return "MODELED_UNDER_OBSERVED"
    return "ZERO"


def merge_counter(items: Any) -> Counter:
    merged: Counter = Counter()
    for item in items:
        merged.update({str(key): int(value) for key, value in item.items()})
    return merged


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


@contextlib.contextmanager
def scoped_env(updates: dict[str, str | None]) -> Iterator[None]:
    previous = {key: os.environ.get(key) for key in updates}
    try:
        for key, value in updates.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value
        yield
    finally:
        for key, value in previous.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
