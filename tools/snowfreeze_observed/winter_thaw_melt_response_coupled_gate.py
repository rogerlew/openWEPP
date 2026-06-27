#!/usr/bin/env python3
"""Run the SNOWDENSITY-10.3.7 opt-in melt model through coupled WAT.

This is the review-disposition gate for SNOWDENSITY-10.3.7. It exercises the
real ``openwepp-cli-hill --direct-production-executor`` WAT path for the same
snow-depth surfaces used by SNOWDENSITY-10.3.5c, changing only the package-bound
diagnostic melt selector ``OPENWEPP_SNOWDENSITY1037_MELT_MODEL``.
Default artifact: ``coupled-wat-melt-response.json``.
"""

from __future__ import annotations

import argparse
import contextlib
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any, Iterator


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import observed_harness  # noqa: E402
import phase_partition_snowdepth_adjudication as phase  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowdensity10-3-7-coupled-wat-melt-response-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-066 OBL-SNOWFREEZE-P-041"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_7_coupled_wat_melt_response"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_MELT_ENV = "OPENWEPP_SNOWDENSITY1037_MELT_MODEL"
SNOW_TRACE_ENV = "OPENWEPP_R7H_SNOW_TRACE_PATH"
DEFAULT_MODEL = "legacy_coe"
DEFAULT_CANDIDATE_MODEL = "coe_winter_thaw_state_loss_v1"
DEFAULT_ARTIFACT_STEM = "coupled-wat-melt-response"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--melt-env", default=DEFAULT_MELT_ENV)
    parser.add_argument("--candidate-model", default=DEFAULT_CANDIDATE_MODEL)
    parser.add_argument("--schema", default=SCHEMA)
    parser.add_argument("--contract", default=CONTRACT)
    parser.add_argument("--artifact-stem", default=DEFAULT_ARTIFACT_STEM)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = adjudicate(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        melt_env=args.melt_env,
        candidate_model=args.candidate_model,
        schema=args.schema,
        contract=args.contract,
        artifact_stem=args.artifact_stem,
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    melt_env: str,
    candidate_model: str,
    schema: str,
    contract: str,
    artifact_stem: str,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    surfaces = []
    trace_paths: dict[str, Path] = {}
    models = [DEFAULT_MODEL, candidate_model]
    for surface in phase.SURFACES:
        model_results = {}
        for model in models:
            trace_path = output_dir / "traces" / f"{surface.surface_id}_{model}.jsonl"
            trace_paths[f"{surface.surface_id}:{model}"] = trace_path
            model_results[model] = run_and_analyze(
                surface=surface,
                output_dir=output_dir,
                hill_binary=hill_binary,
                melt_env=melt_env,
                candidate_model=candidate_model,
                model=model,
                trace_path=trace_path,
                run_model=run_models,
            )
        surfaces.append(build_surface_report(surface, model_results))

    trace_proof = build_trace_proof(trace_paths, candidate_model)
    report = {
        "schema": schema,
        "contract": contract,
        "evidence_class": "Static + Ran",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "diagnostic_selector": {
            "env": melt_env,
            "default_behavior": "absent selector -> legacy_coe",
            "opt_in_value": candidate_model,
            "trace_env": SNOW_TRACE_ENV,
            "trace_proof": trace_proof,
        },
        "protected_boundaries": {
            "default_activation_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "density_phase_canopy_radiation_frost_changed": False,
        },
        "summary": summarize(surfaces, trace_proof),
        "surfaces": surfaces,
        "raw_outputs": {
            "output_dir": rel(output_dir),
            "package_json": rel(package_artifacts_dir / f"{artifact_stem}.json"),
            "package_markdown": rel(package_artifacts_dir / f"{artifact_stem}.md"),
        },
    }
    report["diagnostic_selector"]["env"] = melt_env
    rubric.write_json(package_artifacts_dir / f"{artifact_stem}.json", report)
    (package_artifacts_dir / f"{artifact_stem}.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    rubric.write_json(output_dir / f"{artifact_stem}.json", report)
    (output_dir / f"{artifact_stem}.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    return report


def run_and_analyze(
    surface: phase.Surface,
    output_dir: Path,
    hill_binary: Path,
    melt_env: str,
    candidate_model: str,
    model: str,
    trace_path: Path,
    run_model: bool,
) -> dict[str, Any]:
    run_dir = output_dir / "runs" / surface.surface_id / model
    run_dir.mkdir(parents=True, exist_ok=True)
    trace_path.parent.mkdir(parents=True, exist_ok=True)
    if trace_path.exists() and run_model:
        trace_path.unlink()
    run_stem = observed_harness.discover_run_stem(surface.fixture_dir)
    run_id = f"{surface.surface_id}_{model}"
    runfile_path = run_dir / f"{run_id}.run"
    observed_harness.write_runfile(runfile_path, surface.fixture_dir, run_stem, run_dir, run_id)
    command = observed_harness.cli_command(
        hill_binary,
        surface.fixture_dir,
        runfile_path,
        run_dir,
        "direct-production-executor",
    )
    env_value = None if model == DEFAULT_MODEL else candidate_model
    if run_model:
        with scoped_env({melt_env: env_value, SNOW_TRACE_ENV: str(trace_path)}):
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
                f"openwepp-cli-hill failed for {surface.surface_id}/{model} "
                f"with exit code {completed.returncode}; see {run_dir}"
            )
    wat_path = run_dir / f"{run_id}.wat.parquet"
    if not wat_path.is_file():
        raise FileNotFoundError(f"expected WAT parquet output {wat_path}")
    modeled = observed_harness.load_modeled_wat(wat_path)
    observations = phase.load_observations(surface)
    pairs = phase.pair_observations(observations, modeled, surface.observation_kind)
    return {
        "model": model,
        "env": None if model == DEFAULT_MODEL else {melt_env: candidate_model},
        "command": [str(value) for value in command],
        "run_dir": rel(run_dir),
        "runfile": rel(runfile_path),
        "wat": rel(wat_path),
        "trace": rel(trace_path),
        "observation_row_count": len(observations),
        "paired_row_count": len(pairs),
        "residuals": phase.residual_summary(pairs),
        "wat_summary": phase.wat_summary(modeled),
    }


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


def build_surface_report(
    surface: phase.Surface,
    model_results: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    default_result = model_results[DEFAULT_MODEL]
    candidate_model = next(model for model in model_results if model != DEFAULT_MODEL)
    candidate_result = model_results[candidate_model]
    deltas = wat_summary_delta(default_result["wat_summary"], candidate_result["wat_summary"])
    paired_delta = phase.paired_residual_delta(
        default_result["residuals"], candidate_result["residuals"]
    )
    return {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "fixture_dir": rel(surface.fixture_dir),
        "verdict_scope": surface.verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "note": surface.note,
        "default": default_result,
        "candidate": candidate_result,
        "wat_deltas": deltas,
        "paired_residual_delta": paired_delta,
        "impact_class": classify_surface_impact(surface, paired_delta, deltas),
    }


def wat_summary_delta(default: dict[str, Any], candidate: dict[str, Any]) -> dict[str, Any]:
    return {
        "peak_snow_depth_delta_candidate_minus_default_m": optional_delta(
            candidate.get("peak_snow_depth_m"), default.get("peak_snow_depth_m")
        ),
        "mean_snow_depth_delta_candidate_minus_default_m": optional_delta(
            candidate.get("mean_snow_depth_m"), default.get("mean_snow_depth_m")
        ),
        "depth_day_sum_delta_candidate_minus_default_m_days": optional_delta(
            candidate.get("depth_day_sum_m_days"), default.get("depth_day_sum_m_days")
        ),
        "peak_snow_water_delta_candidate_minus_default_m": optional_delta(
            candidate.get("peak_snow_water_m"), default.get("peak_snow_water_m")
        ),
        "swe_day_sum_delta_candidate_minus_default_m_days": optional_delta(
            candidate.get("swe_day_sum_m_days"), default.get("swe_day_sum_m_days")
        ),
    }


def classify_surface_impact(
    surface: phase.Surface,
    paired_delta: dict[str, Any],
    deltas: dict[str, Any],
) -> str:
    if surface.verdict_scope != "paired_observation":
        return "OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY"
    fail_delta = paired_delta["fail_count_delta_default_minus_opt_in"]
    abs_reduction = paired_delta["mean_abs_depth_reduction_m"]
    if fail_delta is None or abs_reduction is None:
        return "NO-PAIRED-SNOW-DEPTH"
    if fail_delta > 0 and abs_reduction >= 0.0:
        return "IMPROVED"
    if fail_delta == 0 and abs_reduction > 0.0:
        return "MARGINAL-IMPROVEMENT"
    if fail_delta < 0 or abs_reduction < 0.0:
        return "WORSE"
    if abs(deltas.get("depth_day_sum_delta_candidate_minus_default_m_days") or 0.0) > 1.0e-9:
        return "CHANGED-NO-RESIDUAL-IMPROVEMENT"
    return "NO-WAT-CHANGE"


def summarize(surfaces: list[dict[str, Any]], trace_proof: dict[str, Any]) -> dict[str, Any]:
    paired = [item for item in surfaces if item["verdict_scope"] == "paired_observation"]
    blocked = [item for item in surfaces if item["verdict_scope"] != "paired_observation"]
    default_pairs = sum(item["default"]["residuals"]["paired_count"] for item in paired)
    candidate_pairs = sum(item["candidate"]["residuals"]["paired_count"] for item in paired)
    default_fail = sum(item["default"]["residuals"]["snow_control_fail_count"] for item in paired)
    candidate_fail = sum(
        item["candidate"]["residuals"]["snow_control_fail_count"] for item in paired
    )
    improved = [item for item in paired if item["impact_class"] == "IMPROVED"]
    marginal = [item for item in paired if item["impact_class"] == "MARGINAL-IMPROVEMENT"]
    worse = [item for item in paired if item["impact_class"] == "WORSE"]
    trace_selected = trace_proof["candidate_trace_selected_count"] > 0
    default_fail_fraction = default_fail / default_pairs if default_pairs else None
    candidate_fail_fraction = candidate_fail / candidate_pairs if candidate_pairs else None
    no_worse_gate = trace_selected and candidate_fail <= default_fail and not worse

    if not paired:
        disposition = "WINTER-THAW-COUPLED-WAT-HOLD"
        blocker = "NO-PAIRED-SNOW-DEPTH-SURFACES"
    elif not trace_selected:
        disposition = "WINTER-THAW-COUPLED-WAT-HOLD"
        blocker = "MELT-OPT-IN-DIRECT-TRACE-ABSENT"
    elif candidate_fail < default_fail and not worse:
        disposition = "WINTER-THAW-COUPLED-WAT-IMPROVES"
        blocker = "SNOW-CONTROL-NOT-CLEARED" if candidate_fail > 0 else None
    elif candidate_fail == default_fail and not worse:
        disposition = "WINTER-THAW-COUPLED-WAT-NEUTRAL"
        blocker = "SNOW-CONTROL-NOT-CLEARED" if candidate_fail > 0 else None
    else:
        disposition = "WINTER-THAW-COUPLED-WAT-WORSE"
        blocker = "COUPLED-SNOW-CONTROL-WORSENED"

    return {
        "disposition": disposition,
        "blocker": blocker,
        "coupled_no_worse_gate_passed": no_worse_gate,
        "candidate_snow_control_passed": candidate_fail == 0 and candidate_pairs > 0,
        "surface_count": len(surfaces),
        "paired_surface_count": len(paired),
        "observation_blocked_surface_count": len(blocked),
        "paired_surface_improved_count": len(improved),
        "paired_surface_marginal_count": len(marginal),
        "paired_surface_worse_count": len(worse),
        "default_paired_row_count": default_pairs,
        "candidate_paired_row_count": candidate_pairs,
        "default_snow_control_fail_count": default_fail,
        "candidate_snow_control_fail_count": candidate_fail,
        "default_snow_control_fail_fraction": default_fail_fraction,
        "candidate_snow_control_fail_fraction": candidate_fail_fraction,
        "snow_control_fail_delta_default_minus_candidate": default_fail - candidate_fail,
        "observation_blocked_surface_ids": [item["surface_id"] for item in blocked],
    }


def build_trace_proof(trace_paths: dict[str, Path], candidate_model: str) -> dict[str, Any]:
    by_model = {DEFAULT_MODEL: {}, candidate_model: {}}
    default_selected = 0
    candidate_selected = 0
    for key, path in trace_paths.items():
        _surface_id, model = key.split(":", 1)
        counts = count_trace_melt_models(path)
        by_model[model][rel(path)] = counts
        default_selected += counts.get(DEFAULT_MODEL, 0)
        candidate_selected += counts.get(candidate_model, 0)
    return {
        "default_trace_selected_count": default_selected,
        "candidate_trace_selected_count": candidate_selected,
        "trace_counts_by_path": by_model,
    }


def count_trace_melt_models(path: Path) -> dict[str, int]:
    if not path.is_file():
        return {}
    counts: dict[str, int] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        model = str(row.get("snow_melt_model", "missing"))
        counts[model] = counts.get(model, 0) + 1
    return counts


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.7 Coupled WAT Melt Response Gate",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Blocker: `{summary['blocker']}`",
        f"- Coupled no-worse gate passed: `{summary['coupled_no_worse_gate_passed']}`",
        f"- Candidate snow-control passed: `{summary['candidate_snow_control_passed']}`",
        f"- Default fail count: `{summary['default_snow_control_fail_count']}`",
        f"- Candidate fail count: `{summary['candidate_snow_control_fail_count']}`",
        f"- Fail delta default-minus-candidate: `{summary['snow_control_fail_delta_default_minus_candidate']}`",
        f"- Trace proof: `{report['diagnostic_selector']['trace_proof']}`",
        "",
        "## Surface Results",
        "",
        "| Surface | Scope | Impact | Default fails | Candidate fails | Mean abs reduction m | Depth day-sum delta m days |",
        "|---|---|---|---:|---:|---:|---:|",
    ]
    for item in report["surfaces"]:
        delta = item["paired_residual_delta"]
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{item['surface_id']}`",
                    item["verdict_scope"],
                    item["impact_class"],
                    str(item["default"]["residuals"]["snow_control_fail_count"]),
                    str(item["candidate"]["residuals"]["snow_control_fail_count"]),
                    fmt_optional(delta.get("mean_abs_depth_reduction_m")),
                    fmt_optional(
                        item["wat_deltas"].get(
                            "depth_day_sum_delta_candidate_minus_default_m_days"
                        )
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
            "- Observation-blocked surfaces are diagnostic-only and carry no defect verdict.",
            "",
        ]
    )
    return "\n".join(lines)


def optional_delta(candidate: Any, default: Any) -> float | None:
    if candidate is None or default is None:
        return None
    return float(candidate) - float(default)


def fmt_optional(value: Any) -> str:
    if value is None:
        return "n/a"
    return f"{float(value):.6f}"


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
