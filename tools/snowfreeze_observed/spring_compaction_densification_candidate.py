#!/usr/bin/env python3
"""Run the SNOWDENSITY-10.3.11 spring densification candidate through coupled WAT."""

from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import json
import os
import subprocess
import sys
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


SCHEMA = "snowdensity10-3-11-spring-compaction-densification-candidate-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-068 OBL-SNOWFREEZE-P-043"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_11_spring_compaction_densification"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_PRIOR_1038_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001"
    / "artifacts/liquid-holding-capacity-coupled-wat.json"
)
DEFAULT_PRIOR_10310_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001"
    / "artifacts/spring-pack-depletion-compaction-adjudication.json"
)
MELT_ENV = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL"
DENSITY_ENV = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL"
SNOW_TRACE_ENV = "OPENWEPP_R7H_SNOW_TRACE_PATH"
MELT_MODEL = "coe_liquid_holding_capacity_v1"
DENSITY_BASELINE = "physics_bulk_density_compaction_v1"
SPRING_CANDIDATE = "physics_bulk_spring_densification_v1"
ARTIFACT_STEM = "spring-compaction-densification-candidate"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--prior-10-3-8-report", type=Path, default=DEFAULT_PRIOR_1038_REPORT)
    parser.add_argument("--prior-10-3-10-report", type=Path, default=DEFAULT_PRIOR_10310_REPORT)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = adjudicate(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        prior_1038_report=args.prior_10_3_8_report.resolve(),
        prior_10310_report=args.prior_10_3_10_report.resolve(),
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    prior_1038_report: Path,
    prior_10310_report: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")
    prior_1038 = read_json(prior_1038_report)
    prior_10310 = read_json(prior_10310_report)
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    surfaces = []
    trace_paths: dict[str, Path] = {}
    prior_by_surface = {item["surface_id"]: item for item in prior_1038["surfaces"]}
    prior_10310_by_surface = {item["surface_id"]: item for item in prior_10310["surfaces"]}
    for surface in phase.SURFACES:
        model_results = {}
        for density_model in [DENSITY_BASELINE, SPRING_CANDIDATE]:
            trace_path = output_dir / "traces" / f"{surface.surface_id}_{density_model}.jsonl"
            trace_paths[f"{surface.surface_id}:{density_model}"] = trace_path
            model_results[density_model] = run_and_analyze(
                surface=surface,
                output_dir=output_dir,
                hill_binary=hill_binary,
                density_model=density_model,
                trace_path=trace_path,
                run_model=run_models,
            )
        surfaces.append(
            build_surface_report(
                surface=surface,
                model_results=model_results,
                prior_1038_surface=prior_by_surface[surface.surface_id],
                prior_10310_surface=prior_10310_by_surface[surface.surface_id],
            )
        )

    trace_proof = build_trace_proof(trace_paths)
    summary = summarize(surfaces, trace_proof, prior_1038, prior_10310)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "diagnostic_selector": {
            "melt_env": MELT_ENV,
            "melt_model": MELT_MODEL,
            "density_env": DENSITY_ENV,
            "density_baseline": DENSITY_BASELINE,
            "spring_candidate": SPRING_CANDIDATE,
            "trace_env": SNOW_TRACE_ENV,
            "trace_proof": trace_proof,
        },
        "source_reports": {
            "prior_10_3_8_coupled_wat": rel(prior_1038_report),
            "prior_10_3_10_cap_adjudication": rel(prior_10310_report),
            "prior_10_3_8_candidate_fail_count": prior_1038["summary"][
                "candidate_snow_control_fail_count"
            ],
            "prior_10_3_10_compaction_only_failure_count": prior_10310["summary"][
                "compaction_only_feasible_failure_count"
            ],
        },
        "protected_boundaries": {
            "default_activation_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "density_cap_changed": False,
            "observed_depth_or_density_consumed_by_runtime": False,
            "melt_radiation_canopy_phase_rain_heat_longwave_frost_changed": False,
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
    density_model: str,
    trace_path: Path,
    run_model: bool,
) -> dict[str, Any]:
    run_dir = output_dir / "runs" / surface.surface_id / density_model
    run_dir.mkdir(parents=True, exist_ok=True)
    trace_path.parent.mkdir(parents=True, exist_ok=True)
    if trace_path.exists() and run_model:
        trace_path.unlink()
    run_stem = observed_harness.discover_run_stem(surface.fixture_dir)
    run_id = f"{surface.surface_id}_{density_model}"
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
        DENSITY_ENV: density_model,
        SNOW_TRACE_ENV: str(trace_path),
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
                f"openwepp-cli-hill failed for {surface.surface_id}/{density_model} "
                f"with exit code {completed.returncode}; see {run_dir}"
            )
    wat_path = run_dir / f"{run_id}.wat.parquet"
    if not wat_path.is_file():
        raise FileNotFoundError(f"expected WAT parquet output {wat_path}")
    modeled = observed_harness.load_modeled_wat(wat_path)
    observations = phase.load_observations(surface)
    pairs = phase.pair_observations(observations, modeled, surface.observation_kind)
    march_april_pairs = [
        march_april.annotate_pair(pair)
        for pair in pairs
        if dt.date.fromisoformat(pair["date"]).month in march_april.MARCH_APRIL_MONTHS
    ]
    spring_evaluations = [spring10.evaluate_pair(pair) for pair in march_april_pairs]
    return {
        "model": density_model,
        "env": {MELT_ENV: MELT_MODEL, DENSITY_ENV: density_model},
        "command": [str(value) for value in command],
        "run_dir": rel(run_dir),
        "runfile": rel(runfile_path),
        "wat": rel(wat_path),
        "trace": rel(trace_path),
        "observation_row_count": len(observations),
        "paired_row_count": len(pairs),
        "residuals": phase.residual_summary(pairs),
        "wat_summary": phase.wat_summary(modeled),
        "march_april_compaction_adjudication": spring10.evaluation_summary(spring_evaluations),
    }


def build_surface_report(
    surface: phase.Surface,
    model_results: dict[str, dict[str, Any]],
    prior_1038_surface: dict[str, Any],
    prior_10310_surface: dict[str, Any],
) -> dict[str, Any]:
    density_baseline = model_results[DENSITY_BASELINE]
    candidate = model_results[SPRING_CANDIDATE]
    candidate_vs_density = phase.paired_residual_delta(
        density_baseline["residuals"], candidate["residuals"]
    )
    prior_candidate = prior_1038_surface["candidate"]
    candidate_vs_prior = phase.paired_residual_delta(
        prior_candidate["residuals"], candidate["residuals"]
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
        "prior_10_3_8_candidate": prior_candidate,
        "prior_10_3_10_cap_adjudication": prior_10310_surface["adjudication"],
        "density_baseline": density_baseline,
        "candidate": candidate,
        "candidate_vs_density_baseline": candidate_vs_density,
        "candidate_vs_prior_10_3_8": candidate_vs_prior,
        "impact_class": classify_surface_impact(surface, candidate_vs_density),
    }


def classify_surface_impact(surface: phase.Surface, delta: dict[str, Any]) -> str:
    if surface.verdict_scope != "paired_observation":
        return "OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY"
    fail_delta = delta["fail_count_delta_default_minus_opt_in"]
    abs_reduction = delta["mean_abs_depth_reduction_m"]
    if fail_delta is None or abs_reduction is None:
        return "NO-PAIRED-SNOW-DEPTH"
    if fail_delta > 0 and abs_reduction >= 0.0:
        return "IMPROVED"
    if fail_delta == 0 and abs_reduction > 0.0:
        return "MARGINAL-IMPROVEMENT"
    if fail_delta < 0 or abs_reduction < 0.0:
        return "WORSE"
    return "NO-WAT-CHANGE"


def summarize(
    surfaces: list[dict[str, Any]],
    trace_proof: dict[str, Any],
    prior_1038: dict[str, Any],
    prior_10310: dict[str, Any],
) -> dict[str, Any]:
    paired = [item for item in surfaces if item["verdict_scope"] == "paired_observation"]
    blocked = [item for item in surfaces if item["verdict_scope"] != "paired_observation"]
    density_pairs = sum(item["density_baseline"]["residuals"]["paired_count"] for item in paired)
    candidate_pairs = sum(item["candidate"]["residuals"]["paired_count"] for item in paired)
    density_fail = sum(
        item["density_baseline"]["residuals"]["snow_control_fail_count"] for item in paired
    )
    candidate_fail = sum(item["candidate"]["residuals"]["snow_control_fail_count"] for item in paired)
    prior_fail = prior_1038["summary"]["candidate_snow_control_fail_count"]
    improved = [item for item in paired if item["impact_class"] == "IMPROVED"]
    marginal = [item for item in paired if item["impact_class"] == "MARGINAL-IMPROVEMENT"]
    worse = [item for item in paired if item["impact_class"] == "WORSE"]
    density_trace_ok = trace_proof["density_baseline_trace_selected_count"] > 0
    candidate_trace_ok = trace_proof["candidate_trace_selected_count"] > 0
    under_density = spring_class_count(paired, "density_baseline", spring10.CLASS_UNDER_PERSISTENCE)
    under_candidate = spring_class_count(paired, "candidate", spring10.CLASS_UNDER_PERSISTENCE)
    compaction_density = spring_class_count(paired, "density_baseline", spring10.CLASS_COMPACTION_ONLY)
    compaction_candidate = spring_class_count(paired, "candidate", spring10.CLASS_COMPACTION_ONLY)
    no_worse_gate = (
        density_trace_ok
        and candidate_trace_ok
        and candidate_fail <= density_fail
        and under_candidate <= under_density
        and not worse
    )

    if not paired:
        disposition = "SPRING-DENSIFICATION-HOLD"
        blocker = "NO-PAIRED-SNOW-DEPTH-SURFACES"
    elif not density_trace_ok or not candidate_trace_ok:
        disposition = "SPRING-DENSIFICATION-HOLD"
        blocker = "DENSITY-OPT-IN-DIRECT-TRACE-ABSENT"
    elif candidate_fail < density_fail and under_candidate <= under_density and not worse:
        disposition = "SPRING-DENSIFICATION-IMPROVES"
        blocker = "SNOW-CONTROL-NOT-CLEARED" if candidate_fail > 0 else None
    elif candidate_fail == density_fail and under_candidate <= under_density and not worse:
        disposition = "SPRING-DENSIFICATION-NEUTRAL"
        blocker = "SNOW-CONTROL-NOT-CLEARED" if candidate_fail > 0 else None
    else:
        disposition = "SPRING-DENSIFICATION-NON-PROMOTION"
        blocker = "COUPLED-SNOW-CONTROL-OR-UNDER-PERSISTENCE-WORSENED"

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
        "prior_10_3_8_snow_control_fail_count": prior_fail,
        "density_baseline_paired_row_count": density_pairs,
        "candidate_paired_row_count": candidate_pairs,
        "density_baseline_snow_control_fail_count": density_fail,
        "candidate_snow_control_fail_count": candidate_fail,
        "snow_control_fail_delta_density_minus_candidate": density_fail - candidate_fail,
        "snow_control_fail_delta_prior_10_3_8_minus_candidate": prior_fail - candidate_fail,
        "prior_10_3_10_compaction_only_failure_count": prior_10310["summary"][
            "compaction_only_feasible_failure_count"
        ],
        "density_baseline_march_april_compaction_only_failure_count": compaction_density,
        "candidate_march_april_compaction_only_failure_count": compaction_candidate,
        "march_april_compaction_only_delta_density_minus_candidate": compaction_density
        - compaction_candidate,
        "density_baseline_under_persistence_failure_count": under_density,
        "candidate_under_persistence_failure_count": under_candidate,
        "observation_blocked_surface_ids": [item["surface_id"] for item in blocked],
    }


def spring_class_count(surfaces: list[dict[str, Any]], arm: str, class_name: str) -> int:
    total = 0
    for surface in surfaces:
        counts = surface[arm]["march_april_compaction_adjudication"]["class_counts_failures"]
        total += int(counts.get(class_name, 0))
    return total


def build_trace_proof(trace_paths: dict[str, Path]) -> dict[str, Any]:
    by_model = {DENSITY_BASELINE: {}, SPRING_CANDIDATE: {}}
    density_selected = 0
    candidate_selected = 0
    for key, path in trace_paths.items():
        _surface_id, model = key.split(":", 1)
        counts = count_trace_density_models(path)
        by_model[model][rel(path)] = counts
        density_selected += counts.get(DENSITY_BASELINE, 0)
        candidate_selected += counts.get(SPRING_CANDIDATE, 0)
    return {
        "density_baseline_trace_selected_count": density_selected,
        "candidate_trace_selected_count": candidate_selected,
        "trace_counts_by_path": by_model,
    }


def count_trace_density_models(path: Path) -> dict[str, int]:
    if not path.is_file():
        return {}
    counts: dict[str, int] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        model = str(row.get("snow_density_model", "missing"))
        counts[model] = counts.get(model, 0) + 1
    return counts


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


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.11 Spring Compaction/Densification Candidate",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Blocker: `{summary['blocker']}`",
        f"- Coupled no-worse gate passed: `{summary['coupled_no_worse_gate_passed']}`",
        f"- Prior 10.3.8 fail count: `{summary['prior_10_3_8_snow_control_fail_count']}`",
        f"- Density-baseline fail count: `{summary['density_baseline_snow_control_fail_count']}`",
        f"- Candidate fail count: `{summary['candidate_snow_control_fail_count']}`",
        f"- Density-minus-candidate fail delta: `{summary['snow_control_fail_delta_density_minus_candidate']}`",
        f"- Candidate under-persistence count: `{summary['candidate_under_persistence_failure_count']}`",
        f"- Trace proof: `{report['diagnostic_selector']['trace_proof']}`",
        "",
        "## Surface Results",
        "",
        "| Surface | Scope | Impact | Density fails | Candidate fails | Density compaction-only | Candidate compaction-only | Under-persistence candidate |",
        "|---|---|---|---:|---:|---:|---:|---:|",
    ]
    for item in report["surfaces"]:
        density_counts = item["density_baseline"]["march_april_compaction_adjudication"][
            "class_counts_failures"
        ]
        candidate_counts = item["candidate"]["march_april_compaction_adjudication"][
            "class_counts_failures"
        ]
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{item['surface_id']}`",
                    item["verdict_scope"],
                    item["impact_class"],
                    str(item["density_baseline"]["residuals"]["snow_control_fail_count"]),
                    str(item["candidate"]["residuals"]["snow_control_fail_count"]),
                    str(density_counts.get(spring10.CLASS_COMPACTION_ONLY, 0)),
                    str(candidate_counts.get(spring10.CLASS_COMPACTION_ONLY, 0)),
                    str(candidate_counts.get(spring10.CLASS_UNDER_PERSISTENCE, 0)),
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
            "- Density cap changed: `false`.",
            "- Runtime calculation consumes observed depth/density: `false`.",
            "- Observation-blocked surfaces are diagnostic-only and carry no defect verdict.",
            "",
        ]
    )
    return "\n".join(lines)


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
