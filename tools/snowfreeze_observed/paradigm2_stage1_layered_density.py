#!/usr/bin/env python3
"""Run the PARADIGM-2 Stage 1 layered snow-density candidate gate."""

from __future__ import annotations

import argparse
import json
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import cross_snotel_mechanism_rubric as cross  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "paradigm2-stage1-layered-snow-density-gate-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-078 INV-SNOWFREEZE-050 ADR-0029"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/paradigm2_stage1_layered_density"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
ARTIFACT_STEM = "paradigm2-stage1-layered-density-rubric"
ACTIVATED = "activated_bundle"
CANDIDATE = "paradigm2_stage1_layered_density"
CURRENT_DEFAULT_FAILS = 15
CURRENT_DEFAULT_SCORE = 179
CONSERVATION_TOLERANCE_M = 1.0e-9


@dataclass(frozen=True)
class GateResult:
    passed: bool
    reason: str


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = diagnose(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def diagnose(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    sites = cross.diagnostic_sites()
    models = model_specs()
    started = time.perf_counter()
    site_reports = [
        cross.score_site(site, models, output_dir, hill_binary, None, run_models)
        for site in sites
    ]
    elapsed_seconds = time.perf_counter() - started
    model_summaries = cross.summarize_models(site_reports, models)
    comparison = cross.compare_to_activated(site_reports, models)
    conservation_proof = build_conservation_proof(site_reports)
    layer_proof = build_layer_proof(site_reports)
    gates = evaluate_gates(
        site_reports,
        model_summaries,
        comparison,
        conservation_proof,
        layer_proof,
    )
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": (
            "Static + Ran"
            if run_models
            else "Static + Ran (completed outputs reused)"
        ),
        "diagnostic_only": False,
        "activation_authorized": all(gate.passed for gate in gates.values()),
        "default_changed": False,
        "opt_in_selector": "physics_bulk_multilayer_density_v1",
        "authority": {
            "adr0029_stage": "Stage 1",
            "rubric": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050",
            "fixture_fitting_used": False,
            "constants_tuned": False,
        },
        "protected_boundaries": {
            "production_default_changed": False,
            "density_cap_changed": False,
            "output_schema_changed": False,
            "fixture_inputs_changed": False,
            "frost_physics_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "site_calibration_performed": False,
        },
        "inputs": {
            "hill_binary": cross.rel(hill_binary),
            "site_count": len(sites),
            "models": [cross.model_record(model) for model in models],
        },
        "summary": summarize(model_summaries, comparison, gates, elapsed_seconds),
        "gates": {
            name: {"passed": gate.passed, "reason": gate.reason}
            for name, gate in gates.items()
        },
        "model_summaries": model_summaries,
        "comparison_to_current_default": comparison,
        "conservation_proof": conservation_proof,
        "layer_persistence_proof": layer_proof,
        "sites": site_reports,
        "matrix": cross.flatten_matrix(site_reports),
        "raw_outputs": {
            "output_dir": cross.rel(output_dir),
            "package_json": cross.rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": cross.rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
        },
    }
    write_report(report, output_dir, package_artifacts_dir)
    return report


def model_specs() -> list[cross.ModelSpec]:
    return [
        cross.ModelSpec(
            model_id=ACTIVATED,
            mechanism="current default: activated bundle + Harder-Pomeroy hourly phase",
            availability="current_direct_runtime",
            env={cross.MELT_ENV: None, cross.DENSITY_ENV: None, cross.PHASE_ENV: None},
            source="openwepp-cli-hill direct-production executor",
            note="current no-env default; rollback baseline preserved",
            lever_rank_eligible=False,
        ),
        cross.ModelSpec(
            model_id=CANDIDATE,
            mechanism="Paradigm 2 Stage 1 n-layer density under local overburden",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: "physics_bulk_multilayer_density_v1",
                cross.PHASE_ENV: None,
            },
            source="OPENWEPP_SNOWDENSITY09_DENSITY_MODEL opt-in",
            note="PARADIGM-2 Stage 1 opt-in candidate; default unchanged",
            lever_rank_eligible=True,
        ),
    ]


def evaluate_gates(
    site_reports: list[dict[str, Any]],
    model_summaries: dict[str, Any],
    comparison: dict[str, Any],
    conservation_proof: dict[str, Any],
    layer_proof: dict[str, Any],
) -> dict[str, GateResult]:
    availability_failures = candidate_availability_failures(site_reports)
    if availability_failures:
        reason = "candidate run failed or was unavailable: " + "; ".join(availability_failures)
        return {
            "cross_snotel_primary": GateResult(False, reason),
            "bidirectional_densification_and_persistence": GateResult(False, reason),
            "whole_model_conservation": GateResult(False, reason),
            "layer_persistence_and_closure": GateResult(False, reason),
        }

    activated = model_summaries[ACTIVATED]["aggregate"]
    candidate = model_summaries[CANDIDATE]["aggregate"]
    candidate_vs_default = comparison["comparisons"][CANDIDATE]
    primary_pass = (
        candidate["robust_fail_count"] < CURRENT_DEFAULT_FAILS
        and candidate["robust_ordinal_score"] > CURRENT_DEFAULT_SCORE
    )
    primary_reason = (
        f"candidate robust profile {candidate['robust_fail_count']}/"
        f"{candidate['robust_ordinal_score']} vs required better than "
        f"{CURRENT_DEFAULT_FAILS}/{CURRENT_DEFAULT_SCORE}; rerun default observed "
        f"{activated['robust_fail_count']}/{activated['robust_ordinal_score']}"
    )
    bidirectional = bidirectional_densification_and_persistence_gate(
        site_reports,
        candidate_vs_default,
    )
    conservation = conservation_gate(conservation_proof)
    layer = layer_gate(layer_proof)
    return {
        "cross_snotel_primary": GateResult(primary_pass, primary_reason),
        "bidirectional_densification_and_persistence": bidirectional,
        "whole_model_conservation": conservation,
        "layer_persistence_and_closure": layer,
    }


def candidate_availability_failures(site_reports: list[dict[str, Any]]) -> list[str]:
    failures = []
    for site in site_reports:
        candidate = site["models"][CANDIDATE]
        if candidate.get("availability") == "current_direct_runtime":
            continue
        reason = candidate.get("unavailable_reason") or f"availability={candidate.get('availability')}"
        failures.append(f"{site['site_id']} {reason}")
    return failures


def bidirectional_densification_and_persistence_gate(
    site_reports: list[dict[str, Any]],
    candidate_vs_default: dict[str, Any],
) -> GateResult:
    improvements = []
    regressions = []
    for site in site_reports:
        activated_profile = site["models"][ACTIVATED]["rubric_profile"]
        candidate_profile = site["models"][CANDIDATE]["rubric_profile"]
        for cell in candidate_profile["cells"]:
            if not cell["forcing_robust"]:
                continue
            signature = str(cell["signature"])
            if "densification" not in signature and "persistence" not in signature:
                continue
            activated_cell = cross.cell_by_id(activated_profile, cell["cell_id"])
            delta = cross.LABEL_SCORE[cell["ordinal_label"]] - cross.LABEL_SCORE[
                activated_cell["ordinal_label"]
            ]
            row = f"{site['site_id']}:{cell['cell_id']}:{delta}"
            if delta > 0:
                improvements.append(row)
            elif delta < 0:
                regressions.append(row)
    worse = candidate_vs_default["worse_robust_cells_vs_activated"]
    passed = bool(improvements) and not regressions and worse == 0
    if passed:
        return GateResult(
            True,
            f"densification/persistence cells improved without regressions; worse robust cells={worse}; improvements={improvements}",
        )
    return GateResult(
        False,
        f"bidirectional evidence incomplete; improvements={improvements}; regressions={regressions}; worse robust cells={worse}",
    )


def conservation_gate(proof: dict[str, Any]) -> GateResult:
    passed = (
        proof["candidate_trace_row_count"] > 0
        and proof["max_abs_snow_state_residual_m"] <= CONSERVATION_TOLERANCE_M
        and proof["max_abs_partition_residual_m"] <= CONSERVATION_TOLERANCE_M
    )
    return GateResult(
        passed,
        "candidate trace rows={rows}; max snow-state residual={snow}; "
        "max partition residual={partition}; tolerance={tol}".format(
            rows=proof["candidate_trace_row_count"],
            snow=proof["max_abs_snow_state_residual_m"],
            partition=proof["max_abs_partition_residual_m"],
            tol=CONSERVATION_TOLERANCE_M,
        ),
    )


def layer_gate(proof: dict[str, Any]) -> GateResult:
    passed = (
        proof["candidate_trace_row_count"] > 0
        and proof["candidate_rows_with_layers_after"] > 0
        and proof["max_abs_layer_swe_after_residual_m"] <= CONSERVATION_TOLERANCE_M
        and proof["max_abs_layer_depth_after_residual_m"] <= CONSERVATION_TOLERANCE_M
    )
    return GateResult(
        passed,
        "layer rows={layer_rows}/{rows}; max layer SWE residual={swe}; "
        "max layer depth residual={depth}; tolerance={tol}".format(
            layer_rows=proof["candidate_rows_with_layers_after"],
            rows=proof["candidate_trace_row_count"],
            swe=proof["max_abs_layer_swe_after_residual_m"],
            depth=proof["max_abs_layer_depth_after_residual_m"],
            tol=CONSERVATION_TOLERANCE_M,
        ),
    )


def build_conservation_proof(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    by_site = {}
    total_rows = 0
    max_abs_snow_state_residual_m = 0.0
    max_abs_partition_residual_m = 0.0
    for site in site_reports:
        model = site["models"][CANDIDATE]
        trace_path = path_from_rel(model.get("trace"))
        closure = trace_closure(trace_path)
        by_site[site["site_id"]] = {
            "corpus": site["corpus"],
            "trace": model.get("trace"),
            **closure,
        }
        total_rows += closure["row_count"]
        max_abs_snow_state_residual_m = max(
            max_abs_snow_state_residual_m,
            closure["max_abs_snow_state_residual_m"],
        )
        max_abs_partition_residual_m = max(
            max_abs_partition_residual_m,
            closure["max_abs_partition_residual_m"],
        )
    return {
        "candidate_trace_row_count": total_rows,
        "max_abs_snow_state_residual_m": max_abs_snow_state_residual_m,
        "max_abs_partition_residual_m": max_abs_partition_residual_m,
        "tolerance_m": CONSERVATION_TOLERANCE_M,
        "passed": (
            total_rows > 0
            and max_abs_snow_state_residual_m <= CONSERVATION_TOLERANCE_M
            and max_abs_partition_residual_m <= CONSERVATION_TOLERANCE_M
        ),
        "by_site": by_site,
    }


def build_layer_proof(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    by_site = {}
    total_rows = 0
    rows_with_layers_after = 0
    max_abs_layer_swe_after_residual_m = 0.0
    max_abs_layer_depth_after_residual_m = 0.0
    for site in site_reports:
        model = site["models"][CANDIDATE]
        trace_path = path_from_rel(model.get("trace"))
        proof = trace_layer_proof(trace_path)
        by_site[site["site_id"]] = {
            "corpus": site["corpus"],
            "trace": model.get("trace"),
            **proof,
        }
        total_rows += proof["row_count"]
        rows_with_layers_after += proof["rows_with_layers_after"]
        max_abs_layer_swe_after_residual_m = max(
            max_abs_layer_swe_after_residual_m,
            proof["max_abs_layer_swe_after_residual_m"],
        )
        max_abs_layer_depth_after_residual_m = max(
            max_abs_layer_depth_after_residual_m,
            proof["max_abs_layer_depth_after_residual_m"],
        )
    return {
        "candidate_trace_row_count": total_rows,
        "candidate_rows_with_layers_after": rows_with_layers_after,
        "max_abs_layer_swe_after_residual_m": max_abs_layer_swe_after_residual_m,
        "max_abs_layer_depth_after_residual_m": max_abs_layer_depth_after_residual_m,
        "tolerance_m": CONSERVATION_TOLERANCE_M,
        "passed": (
            total_rows > 0
            and rows_with_layers_after > 0
            and max_abs_layer_swe_after_residual_m <= CONSERVATION_TOLERANCE_M
            and max_abs_layer_depth_after_residual_m <= CONSERVATION_TOLERANCE_M
        ),
        "by_site": by_site,
    }


def trace_closure(path: Path | None) -> dict[str, Any]:
    if path is None or not path.is_file():
        return {
            "row_count": 0,
            "max_abs_snow_state_residual_m": 0.0,
            "max_abs_partition_residual_m": 0.0,
        }
    row_count = 0
    max_abs_snow_state_residual_m = 0.0
    max_abs_partition_residual_m = 0.0
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            row_count += 1
            max_abs_snow_state_residual_m = max(
                max_abs_snow_state_residual_m,
                abs(snow_state_residual(row)),
            )
            max_abs_partition_residual_m = max(
                max_abs_partition_residual_m,
                abs(partition_residual(row)),
            )
    return {
        "row_count": row_count,
        "max_abs_snow_state_residual_m": max_abs_snow_state_residual_m,
        "max_abs_partition_residual_m": max_abs_partition_residual_m,
    }


def trace_layer_proof(path: Path | None) -> dict[str, Any]:
    if path is None or not path.is_file():
        return {
            "row_count": 0,
            "rows_with_layers_after": 0,
            "max_abs_layer_swe_after_residual_m": 0.0,
            "max_abs_layer_depth_after_residual_m": 0.0,
        }
    row_count = 0
    rows_with_layers_after = 0
    max_abs_layer_swe_after_residual_m = 0.0
    max_abs_layer_depth_after_residual_m = 0.0
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            row_count += 1
            layer_count_after = int(number(row, "snow_layer_count_after"))
            if layer_count_after > 0:
                rows_with_layers_after += 1
                max_abs_layer_swe_after_residual_m = max(
                    max_abs_layer_swe_after_residual_m,
                    abs(
                        number(row, "snow_layer_swe_sum_after_m")
                        - number(row, "runtime_swe_after_m")
                    ),
                )
                max_abs_layer_depth_after_residual_m = max(
                    max_abs_layer_depth_after_residual_m,
                    abs(
                        number(row, "snow_layer_depth_sum_after_m")
                        - number(row, "runtime_depth_after_m")
                    ),
                )
    return {
        "row_count": row_count,
        "rows_with_layers_after": rows_with_layers_after,
        "max_abs_layer_swe_after_residual_m": max_abs_layer_swe_after_residual_m,
        "max_abs_layer_depth_after_residual_m": max_abs_layer_depth_after_residual_m,
    }


def snow_state_residual(row: dict[str, Any]) -> float:
    return (
        number(row, "runtime_swe_before_m")
        + number(row, "accumulation_m")
        + number(row, "rain_retained_m")
        - number(row, "snowpack_swe_loss_m")
        - number(row, "sublimation_m")
        - number(row, "runtime_swe_after_m")
    )


def partition_residual(row: dict[str, Any]) -> float:
    return (
        number(row, "accumulation_m")
        + number(row, "rain_retained_m")
        + number(row, "rain_released_m")
        + number(row, "post_winter_rain_m")
        - number(row, "hyetograph_rainfall_m")
    )


def number(row: dict[str, Any], key: str) -> float:
    value = row.get(key)
    return float(value) if value is not None else 0.0


def path_from_rel(value: Any) -> Path | None:
    if not value:
        return None
    path = Path(str(value))
    return path if path.is_absolute() else REPO_ROOT / path


def summarize(
    model_summaries: dict[str, Any],
    comparison: dict[str, Any],
    gates: dict[str, GateResult],
    elapsed_seconds: float,
) -> dict[str, Any]:
    activated = model_summaries[ACTIVATED]["aggregate"]
    candidate = model_summaries[CANDIDATE]["aggregate"]
    return {
        "activated_default_robust_fail_count": activated["robust_fail_count"],
        "activated_default_robust_ordinal_score": activated["robust_ordinal_score"],
        "candidate_robust_fail_count": candidate["robust_fail_count"],
        "candidate_robust_ordinal_score": candidate["robust_ordinal_score"],
        "candidate_vs_default": comparison["comparisons"][CANDIDATE],
        "elapsed_seconds": elapsed_seconds,
        "activation_authorized": all(gate.passed for gate in gates.values()),
        "failed_gates": [name for name, gate in gates.items() if not gate.passed],
    }


def write_report(report: dict[str, Any], output_dir: Path, package_artifacts_dir: Path) -> None:
    for directory in (output_dir, package_artifacts_dir):
        directory.mkdir(parents=True, exist_ok=True)
        rubric.write_json(directory / f"{ARTIFACT_STEM}.json", report)
        (directory / f"{ARTIFACT_STEM}.md").write_text(markdown_report(report), encoding="utf-8")


def markdown_report(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# Paradigm 2 Stage 1 Layered Density Rubric",
        "",
        f"Evidence class: `{report['evidence_class']}`",
        "",
        "## Summary",
        "",
        f"- Activated default: `{summary['activated_default_robust_fail_count']}` robust fails, "
        f"`{summary['activated_default_robust_ordinal_score']}` robust score.",
        f"- Candidate: `{summary['candidate_robust_fail_count']}` robust fails, "
        f"`{summary['candidate_robust_ordinal_score']}` robust score.",
        f"- Activation authorized: `{str(report['activation_authorized']).lower()}`.",
        f"- Real-run elapsed seconds: `{summary['elapsed_seconds']:.3f}`.",
        "",
        "## Gates",
        "",
    ]
    for name, gate in report["gates"].items():
        lines.append(f"- `{name}`: `{'PASS' if gate['passed'] else 'FAIL'}` - {gate['reason']}")
    lines.extend(
        [
            "",
            "## Protected Boundaries",
            "",
            "- Production default, output schemas, fixtures, density cap, frost behavior, parser/runfile/user selectors, and site calibration are unchanged.",
            "",
            "## Raw Outputs",
            "",
            f"- Output directory: `{report['raw_outputs']['output_dir']}`",
            f"- JSON artifact: `{report['raw_outputs']['package_json']}`",
        ]
    )
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())
