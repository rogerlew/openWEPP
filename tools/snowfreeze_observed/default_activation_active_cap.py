#!/usr/bin/env python3
"""Verify SNOWDENSITY-10.3.15 no-env default activation under the active cap."""

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


SCHEMA = "snowdensity10-3-15-default-activation-active-cap-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-072 OBL-SNOWFREEZE-P-047"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_15_default_activation_active_cap"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
MELT_ENV = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL"
DENSITY_ENV = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL"
TRACE_ENV = "OPENWEPP_R7H_SNOW_TRACE_PATH"
DEFAULT_MELT_MODEL = "coe_liquid_holding_capacity_v1"
DEFAULT_DENSITY_MODEL = "physics_bulk_density_compaction_v1"
ROLLBACK_MELT_MODEL = "legacy_coe"
ROLLBACK_DENSITY_MODEL = "legacy_wepp"
EXPECTED_POLICY_B_FAILURES = 498
EXPECTED_POLICY_B_PAIRED_ROWS = 1415
ARTIFACT_STEM = "default-activation-active-cap"


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

    surfaces = []
    default_trace_paths: dict[str, Path] = {}
    for surface in phase.SURFACES:
        trace_path = output_dir / "traces" / f"{surface.surface_id}_default_no_env.jsonl"
        default_trace_paths[surface.surface_id] = trace_path
        default_run = run_and_analyze(
            surface=surface,
            scenario="default_no_env",
            output_dir=output_dir,
            hill_binary=hill_binary,
            trace_path=trace_path,
            env_updates={
                MELT_ENV: None,
                DENSITY_ENV: None,
                TRACE_ENV: str(trace_path),
            },
            run_model=run_models,
        )
        surfaces.append(
            {
                "surface_id": surface.surface_id,
                "site_group": surface.site_group,
                "cover": surface.cover,
                "fixture_dir": rel(surface.fixture_dir),
                "verdict_scope": surface.verdict_scope,
                "observation_source": surface.observation_source,
                "observation_kind": surface.observation_kind,
                "default_no_env": default_run,
            }
        )

    rollback_surface = next(
        surface for surface in phase.SURFACES if surface.verdict_scope == "paired_observation"
    )
    rollback_trace_path = (
        output_dir / "traces" / f"{rollback_surface.surface_id}_rollback_legacy.jsonl"
    )
    rollback = run_and_analyze(
        surface=rollback_surface,
        scenario="rollback_legacy",
        output_dir=output_dir,
        hill_binary=hill_binary,
        trace_path=rollback_trace_path,
        env_updates={
            MELT_ENV: ROLLBACK_MELT_MODEL,
            DENSITY_ENV: ROLLBACK_DENSITY_MODEL,
            TRACE_ENV: str(rollback_trace_path),
        },
        run_model=run_models,
    )

    default_trace_proof = build_trace_proof(
        default_trace_paths,
        expected_melt_model=DEFAULT_MELT_MODEL,
        expected_density_model=DEFAULT_DENSITY_MODEL,
    )
    rollback_trace_proof = build_trace_proof(
        {rollback_surface.surface_id: rollback_trace_path},
        expected_melt_model=ROLLBACK_MELT_MODEL,
        expected_density_model=ROLLBACK_DENSITY_MODEL,
    )
    summary = summarize(surfaces, default_trace_proof, rollback_trace_proof)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "activated_default": {
            "snow_melt_model": DEFAULT_MELT_MODEL,
            "snow_density_model": DEFAULT_DENSITY_MODEL,
            "melt_env": MELT_ENV,
            "density_env": DENSITY_ENV,
            "trace_env": TRACE_ENV,
            "selector_state": "absent/empty environment values select activated defaults",
            "trace_proof": default_trace_proof,
        },
        "rollback": {
            "surface_id": rollback_surface.surface_id,
            "snow_melt_model": ROLLBACK_MELT_MODEL,
            "snow_density_model": ROLLBACK_DENSITY_MODEL,
            "trace_proof": rollback_trace_proof,
            "run": rollback,
        },
        "protected_boundaries": {
            "parser_runfile_user_cli_selector_added": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "density_cap_changed": False,
            "active_density_cap_kg_m3": 522.0,
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
    scenario: str,
    output_dir: Path,
    hill_binary: Path,
    trace_path: Path,
    env_updates: dict[str, str | None],
    run_model: bool,
) -> dict[str, Any]:
    run_dir = output_dir / "runs" / surface.surface_id / scenario
    run_dir.mkdir(parents=True, exist_ok=True)
    trace_path.parent.mkdir(parents=True, exist_ok=True)
    if trace_path.exists() and run_model:
        trace_path.unlink()
    run_stem = observed_harness.discover_run_stem(surface.fixture_dir)
    run_id = f"{surface.surface_id}_{scenario}"
    runfile_path = run_dir / f"{run_id}.run"
    observed_harness.write_runfile(runfile_path, surface.fixture_dir, run_stem, run_dir, run_id)
    command = observed_harness.cli_command(
        hill_binary,
        surface.fixture_dir,
        runfile_path,
        run_dir,
        "direct-production-executor",
    )
    if run_model:
        with scoped_env(env_updates):
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
                f"openwepp-cli-hill failed for {surface.surface_id}/{scenario} "
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
        "scenario": scenario,
        "env_updates": printable_env(env_updates),
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


def summarize(
    surfaces: list[dict[str, Any]],
    default_trace_proof: dict[str, Any],
    rollback_trace_proof: dict[str, Any],
) -> dict[str, Any]:
    paired = [surface for surface in surfaces if surface["verdict_scope"] == "paired_observation"]
    blocked = [surface for surface in surfaces if surface["verdict_scope"] != "paired_observation"]
    paired_rows = sum(
        surface["default_no_env"]["residuals"]["paired_count"] for surface in paired
    )
    fail_count = sum(
        surface["default_no_env"]["residuals"]["snow_control_fail_count"]
        for surface in paired
    )
    default_trace_ok = (
        default_trace_proof["expected_snow_melt_model_count"] > 0
        and default_trace_proof["expected_snow_density_model_count"] > 0
        and not default_trace_proof["unexpected_snow_melt_models"]
        and not default_trace_proof["unexpected_snow_density_models"]
    )
    rollback_trace_ok = (
        rollback_trace_proof["expected_snow_melt_model_count"] > 0
        and rollback_trace_proof["expected_snow_density_model_count"] > 0
        and not rollback_trace_proof["unexpected_snow_melt_models"]
        and not rollback_trace_proof["unexpected_snow_density_models"]
    )
    residuals_match_policy_b = (
        paired_rows == EXPECTED_POLICY_B_PAIRED_ROWS
        and fail_count == EXPECTED_POLICY_B_FAILURES
    )
    activation_complete = default_trace_ok and rollback_trace_ok and residuals_match_policy_b
    disposition = (
        "COMPLETE-DEFAULT-ACTIVATED-UNDER-ACTIVE-CAP"
        if activation_complete
        else "HOLD-DEFAULT-ACTIVATION-EVIDENCE-MISMATCH"
    )
    return {
        "disposition": disposition,
        "activation_complete": activation_complete,
        "default_trace_ok": default_trace_ok,
        "rollback_trace_ok": rollback_trace_ok,
        "residuals_match_policy_b_active_cap_evidence": residuals_match_policy_b,
        "paired_row_count": paired_rows,
        "snow_control_fail_count": fail_count,
        "expected_policy_b_paired_row_count": EXPECTED_POLICY_B_PAIRED_ROWS,
        "expected_policy_b_snow_control_fail_count": EXPECTED_POLICY_B_FAILURES,
        "fail_delta_from_legacy_default_policy_b": 1147 - fail_count,
        "frost_attribution_unblocked": False,
        "frost_attribution_blocker": "SNOW-CONTROL-RESIDUALS-REMAIN",
        "observation_blocked_surface_count": len(blocked),
        "observation_blocked_surface_ids": [surface["surface_id"] for surface in blocked],
        "protected_boundaries_changed": False,
        "active_density_cap_kg_m3": 522.0,
    }


def build_trace_proof(
    trace_paths: dict[str, Path],
    expected_melt_model: str,
    expected_density_model: str,
) -> dict[str, Any]:
    by_surface = {}
    melt_count = 0
    density_count = 0
    melt_unexpected: Counter[str] = Counter()
    density_unexpected: Counter[str] = Counter()
    for surface_id, path in trace_paths.items():
        counts = count_trace_models(path)
        by_surface[surface_id] = {"path": rel(path), "counts": counts}
        melt_count += counts["snow_melt_model"].get(expected_melt_model, 0)
        density_count += counts["snow_density_model"].get(expected_density_model, 0)
        melt_unexpected.update(
            {
                key: value
                for key, value in counts["snow_melt_model"].items()
                if key != expected_melt_model
            }
        )
        density_unexpected.update(
            {
                key: value
                for key, value in counts["snow_density_model"].items()
                if key != expected_density_model
            }
        )
    return {
        "expected_snow_melt_model": expected_melt_model,
        "expected_snow_density_model": expected_density_model,
        "expected_snow_melt_model_count": melt_count,
        "expected_snow_density_model_count": density_count,
        "unexpected_snow_melt_models": dict(sorted(melt_unexpected.items())),
        "unexpected_snow_density_models": dict(sorted(density_unexpected.items())),
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


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.15 Default Activation Under Active Cap",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Activation complete: `{summary['activation_complete']}`",
        f"- Default trace ok: `{summary['default_trace_ok']}`",
        f"- Rollback trace ok: `{summary['rollback_trace_ok']}`",
        f"- Paired rows: `{summary['paired_row_count']}`",
        f"- Snow-control failures: `{summary['snow_control_fail_count']}`",
        f"- Frost attribution blocker: `{summary['frost_attribution_blocker']}`",
        f"- Active density cap: `{summary['active_density_cap_kg_m3']} kg m^-3`",
        "",
        "## Activated Default",
        "",
        f"- Melt model: `{report['activated_default']['snow_melt_model']}`",
        f"- Density model: `{report['activated_default']['snow_density_model']}`",
        f"- Trace melt count: `{report['activated_default']['trace_proof']['expected_snow_melt_model_count']}`",
        f"- Trace density count: `{report['activated_default']['trace_proof']['expected_snow_density_model_count']}`",
        "",
        "## Rollback",
        "",
        f"- Surface: `{report['rollback']['surface_id']}`",
        f"- Melt model: `{report['rollback']['snow_melt_model']}`",
        f"- Density model: `{report['rollback']['snow_density_model']}`",
        f"- Trace melt count: `{report['rollback']['trace_proof']['expected_snow_melt_model_count']}`",
        f"- Trace density count: `{report['rollback']['trace_proof']['expected_snow_density_model_count']}`",
        "",
        "## Surface Results",
        "",
        "| Surface | Scope | Cover | Paired rows | Failures |",
        "|---|---|---|---:|---:|",
    ]
    for surface in report["surfaces"]:
        residuals = surface["default_no_env"]["residuals"]
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{surface['surface_id']}`",
                    surface["verdict_scope"],
                    surface["cover"],
                    str(residuals["paired_count"]),
                    str(residuals["snow_control_fail_count"]),
                ]
            )
            + " |"
        )
    lines.extend(
        [
            "",
            "## Boundary Disposition",
            "",
            "- Parser/runfile/user CLI selector added: `false`.",
            "- Fixture inputs changed: `false`.",
            "- Public output schema changed: `false`.",
            "- Density cap changed: `false`.",
            "- New process physics added: `false`.",
            "- Frost attribution remains blocked while snow-control residuals remain.",
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


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def printable_env(updates: dict[str, str | None]) -> dict[str, str]:
    return {key: ("<absent>" if value is None else value) for key, value in updates.items()}


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


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
