#!/usr/bin/env python3
"""Run PARADIGM-2 Stage 3-Decouple observed guardrails.

The decoupled arm keeps the current bulk snow-density path and adds only the
Stage 3 thermal/liquid/meltwater-temperature capability.  Therefore the primary
gate is strict equality with the current no-env default snow rubric while the
typed meltwater-temperature source remains opt-in and diagnostic.
"""

from __future__ import annotations

import argparse
import json
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Callable


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import cross_snotel_mechanism_rubric as cross  # noqa: E402
from paradigm2_stage3_liquid_routing_meltwater_temperature import (  # noqa: E402
    RUNOFF_TIMING_CELL_IDS,
    compare_models,
)


SCHEMA = "paradigm2-stage3-decouple-water-temperature-gates-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-081 INV-SNOWFREEZE-050 ADR-0029"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/paradigm2_stage3_decouple_water_temperature"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/release/openwepp-cli-hill"
ARTIFACT_STEM = "paradigm2-stage3-decouple-observed-guardrails"
STAGE3_ENV = "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL"
MODEL_DEFAULT = "activated_bundle"
MODEL_DECOUPLED = "stage3_decoupled_bulk_equivalent"
EXPECTED_DEFAULT_FAIL_COUNT = 15
EXPECTED_DEFAULT_SCORE = 179


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
    decoupled_vs_default = compare_models(site_reports, MODEL_DEFAULT, MODEL_DECOUPLED)
    timing_vs_default = compare_models(
        site_reports,
        MODEL_DEFAULT,
        MODEL_DECOUPLED,
        lambda cell: cell["cell_id"] in RUNOFF_TIMING_CELL_IDS,
    )
    gates = evaluate_gates(model_summaries, site_reports, decoupled_vs_default, timing_vs_default)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran" if run_models else "Static + Reused",
        "diagnostic_only": True,
        "activation_authorized": False,
        "default_changed": False,
        "authority": {
            "rubric": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050",
            "stage3_decouple_contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-081",
            "comparison_policy": (
                "The decoupled arm is scored directly against the current no-env "
                "bulk default because it must preserve aggregate snow density, "
                "depth, SWE, and runoff timing while adding meltwater temperature."
            ),
            "fixture_fitting_used": False,
            "promotion_decision_made": False,
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
            "runoff_timing_cell_ids": sorted(RUNOFF_TIMING_CELL_IDS),
        },
        "summary": summarize(
            model_summaries,
            gates,
            decoupled_vs_default,
            timing_vs_default,
            elapsed_seconds,
        ),
        "gates": {
            name: {"passed": gate.passed, "reason": gate.reason}
            for name, gate in gates.items()
        },
        "model_summaries": model_summaries,
        "decoupled_vs_current_default": decoupled_vs_default,
        "runoff_timing_vs_current_default": timing_vs_default,
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
            model_id=MODEL_DEFAULT,
            mechanism="current no-env default activated bundle",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: None,
                cross.PHASE_ENV: None,
                STAGE3_ENV: None,
            },
            source="openwepp-cli-hill direct-production executor",
            note="bulk default reference and hard no-regress comparator",
            lever_rank_eligible=False,
        ),
        cross.ModelSpec(
            model_id=MODEL_DECOUPLED,
            mechanism="Stage 3-Decouple bulk-equivalent thermal liquid arm",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: None,
                cross.PHASE_ENV: None,
                STAGE3_ENV: "layered_thermal_liquid_v1",
            },
            source=f"{STAGE3_ENV}=layered_thermal_liquid_v1",
            note=(
                "PARADIGM-2 Stage 3-Decouple opt-in capability; no density "
                "selector override and default unchanged"
            ),
            lever_rank_eligible=False,
        ),
    ]


def evaluate_gates(
    model_summaries: dict[str, Any],
    site_reports: list[dict[str, Any]],
    decoupled_vs_default: dict[str, Any],
    timing_vs_default: dict[str, Any],
) -> dict[str, GateResult]:
    failures = availability_failures(site_reports)
    if failures:
        reason = "model run failed or unavailable: " + "; ".join(failures)
        return {
            "candidate_availability": GateResult(False, reason),
            "snow_guardrail_equals_default": GateResult(False, reason),
            "runoff_timing_guardrail_vs_default": GateResult(False, reason),
        }

    default = model_summaries[MODEL_DEFAULT]["aggregate"]
    candidate = model_summaries[MODEL_DECOUPLED]["aggregate"]
    exact_default_profile = (
        default["robust_fail_count"] == EXPECTED_DEFAULT_FAIL_COUNT
        and default["robust_ordinal_score"] == EXPECTED_DEFAULT_SCORE
        and candidate["robust_fail_count"] == default["robust_fail_count"]
        and candidate["robust_ordinal_score"] == default["robust_ordinal_score"]
        and decoupled_vs_default["better_robust_cells"] == 0
        and decoupled_vs_default["worse_robust_cells"] == 0
        and decoupled_vs_default["robust_fail_delta_base_minus_candidate"] == 0
        and decoupled_vs_default["robust_score_delta_candidate_minus_base"] == 0
    )
    snow_reason = (
        "default/candidate robust profile="
        f"{default['robust_fail_count']}/{default['robust_ordinal_score']} "
        f"vs {candidate['robust_fail_count']}/{candidate['robust_ordinal_score']}; "
        "better/equal/worse="
        f"{decoupled_vs_default['better_robust_cells']}/"
        f"{decoupled_vs_default['equal_robust_cells']}/"
        f"{decoupled_vs_default['worse_robust_cells']}; fail_delta="
        f"{decoupled_vs_default['robust_fail_delta_base_minus_candidate']}; score_delta="
        f"{decoupled_vs_default['robust_score_delta_candidate_minus_base']}"
    )
    timing_pass = (
        timing_vs_default["scored_candidate_cell_count"] > 0
        and timing_vs_default["worse_robust_cells"] == 0
    )
    timing_reason = (
        "decoupled-vs-default timing/runoff cells scored="
        f"{timing_vs_default['scored_candidate_cell_count']}; better/equal/worse="
        f"{timing_vs_default['better_robust_cells']}/"
        f"{timing_vs_default['equal_robust_cells']}/"
        f"{timing_vs_default['worse_robust_cells']}"
    )
    return {
        "candidate_availability": GateResult(True, "all direct-runtime model runs completed"),
        "snow_guardrail_equals_default": GateResult(exact_default_profile, snow_reason),
        "runoff_timing_guardrail_vs_default": GateResult(timing_pass, timing_reason),
    }


def availability_failures(site_reports: list[dict[str, Any]]) -> list[str]:
    failures = []
    for site in site_reports:
        model = site["models"][MODEL_DECOUPLED]
        if model.get("availability") == "current_direct_runtime":
            continue
        reason = model.get("unavailable_reason") or f"availability={model.get('availability')}"
        failures.append(f"{site['site_id']} {MODEL_DECOUPLED} {reason}")
    return failures


def summarize(
    model_summaries: dict[str, Any],
    gates: dict[str, GateResult],
    decoupled_vs_default: dict[str, Any],
    timing_vs_default: dict[str, Any],
    elapsed_seconds: float,
) -> dict[str, Any]:
    default = model_summaries[MODEL_DEFAULT]["aggregate"]
    candidate = model_summaries[MODEL_DECOUPLED]["aggregate"]
    return {
        "observed_gates_passed": all(gate.passed for gate in gates.values()),
        "promotion_decision_made": False,
        "activation_authorized": False,
        "elapsed_seconds": elapsed_seconds,
        "current_default_robust_fail_count": default["robust_fail_count"],
        "current_default_robust_ordinal_score": default["robust_ordinal_score"],
        "decoupled_robust_fail_count": candidate["robust_fail_count"],
        "decoupled_robust_ordinal_score": candidate["robust_ordinal_score"],
        "decoupled_vs_default_better_robust_cells": decoupled_vs_default[
            "better_robust_cells"
        ],
        "decoupled_vs_default_worse_robust_cells": decoupled_vs_default[
            "worse_robust_cells"
        ],
        "timing_vs_default_worse_robust_cells": timing_vs_default["worse_robust_cells"],
    }


def write_report(report: dict[str, Any], output_dir: Path, package_artifacts_dir: Path) -> None:
    output_json = output_dir / f"{ARTIFACT_STEM}.json"
    output_md = output_dir / f"{ARTIFACT_STEM}.md"
    package_json = package_artifacts_dir / f"{ARTIFACT_STEM}.json"
    package_md = package_artifacts_dir / f"{ARTIFACT_STEM}.md"
    text = json.dumps(report, indent=2, sort_keys=True)
    output_json.write_text(text + "\n", encoding="utf-8")
    package_json.write_text(text + "\n", encoding="utf-8")
    markdown = render_markdown(report)
    output_md.write_text(markdown, encoding="utf-8")
    package_md.write_text(markdown, encoding="utf-8")


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    rows = [
        "# PARADIGM-2 Stage 3-Decouple Observed Guardrails",
        "",
        f"Schema: `{report['schema']}`",
        f"Contract: `{report['contract']}`",
        f"Evidence: `{report['evidence_class']}`",
        "",
        "## Summary",
        "",
        f"- Observed gates passed: `{summary['observed_gates_passed']}`",
        f"- Current default robust profile: `{summary['current_default_robust_fail_count']}` fails / `{summary['current_default_robust_ordinal_score']}` score",
        f"- Decoupled robust profile: `{summary['decoupled_robust_fail_count']}` fails / `{summary['decoupled_robust_ordinal_score']}` score",
        f"- Decoupled vs default better robust cells: `{summary['decoupled_vs_default_better_robust_cells']}`",
        f"- Decoupled vs default worse robust cells: `{summary['decoupled_vs_default_worse_robust_cells']}`",
        f"- Runoff/timing worse robust cells: `{summary['timing_vs_default_worse_robust_cells']}`",
        f"- Real-run elapsed seconds: `{summary['elapsed_seconds']:.3f}`",
        "",
        "## Gates",
        "",
    ]
    for name, gate in report["gates"].items():
        label = "PASS" if gate["passed"] else "FAIL"
        rows.append(f"- `{name}`: `{label}` - {gate['reason']}")
    rows.extend(
        [
            "",
            "## Boundary",
            "",
            "- The decoupled arm is compared directly to the current no-env bulk default.",
            "- `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL` is not set for the candidate.",
            "- `event_rain_on_snow_response` remains unavailable in the daily observed corpus.",
            "- No default activation or full in-stream temperature routing is authorized.",
            "",
            "## Raw Outputs",
            "",
            f"- Output directory: `{report['raw_outputs']['output_dir']}`",
            f"- JSON artifact: `{report['raw_outputs']['package_json']}`",
        ]
    )
    return "\n".join(rows) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())
