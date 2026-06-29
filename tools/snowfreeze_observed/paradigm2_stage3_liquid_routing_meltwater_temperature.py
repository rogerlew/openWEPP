#!/usr/bin/env python3
"""Run PARADIGM-2 Stage 3 observed snow/runoff-timing guardrails.

The Stage 3 candidate is diagnostic-only: it keeps the CoE melt mass path
authoritative and adds per-layer liquid/energy diagnostics plus a typed
meltwater temperature.  Therefore this gate isolates Stage 3 from the already
known Stage 1 layered-density rubric movement by comparing:

* current no-env default, for reference;
* Stage 1 layered density with Stage 3 disabled, the rollback baseline; and
* Stage 1 layered density plus Stage 3 liquid routing enabled.

Promotion/default activation remains out of scope.
"""

from __future__ import annotations

import argparse
import json
import sys
import time
from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Callable


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import cross_snotel_mechanism_rubric as cross  # noqa: E402


SCHEMA = "paradigm2-stage3-liquid-routing-meltwater-temperature-gates-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-080 INV-SNOWFREEZE-050 ADR-0029"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/paradigm2_stage3_liquid_routing"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/release/openwepp-cli-hill"
ARTIFACT_STEM = "paradigm2-stage3-observed-guardrails"
STAGE3_ENV = "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL"
MODEL_DEFAULT = "activated_bundle"
MODEL_STAGE1 = "stage1_layered_density_disabled_stage3"
MODEL_STAGE3 = "stage3_layered_thermal_liquid_v1"
RUNOFF_TIMING_CELL_IDS = {
    "long_term_snow_cover_duration",
    "seasonal_peak_swe_date",
    "seasonal_peak_depth_date",
    "seasonal_ablation_meltout_date",
    "event_rain_on_snow_response",
}


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
    stage3_vs_stage1 = compare_models(site_reports, MODEL_STAGE1, MODEL_STAGE3)
    stage3_vs_default = compare_models(site_reports, MODEL_DEFAULT, MODEL_STAGE3)
    timing_vs_stage1 = compare_models(
        site_reports,
        MODEL_STAGE1,
        MODEL_STAGE3,
        lambda cell: cell["cell_id"] in RUNOFF_TIMING_CELL_IDS,
    )
    gates = evaluate_gates(site_reports, stage3_vs_stage1, timing_vs_stage1)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran" if run_models else "Static + Reused",
        "diagnostic_only": True,
        "activation_authorized": False,
        "default_changed": False,
        "authority": {
            "rubric": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050",
            "stage3_contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-080",
            "comparison_policy": (
                "Stage 3 no-regression is isolated against the Stage 1 layered "
                "density rollback baseline because Stage 3 requires that layer "
                "stack and does not change density or public melt mass."
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
        "summary": summarize(model_summaries, gates, stage3_vs_stage1, timing_vs_stage1, elapsed_seconds),
        "gates": {
            name: {"passed": gate.passed, "reason": gate.reason}
            for name, gate in gates.items()
        },
        "model_summaries": model_summaries,
        "stage3_vs_stage1_rollback": stage3_vs_stage1,
        "stage3_vs_current_default_reference": stage3_vs_default,
        "runoff_timing_vs_stage1_rollback": timing_vs_stage1,
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
            note="reference only; Stage 3 no-regression is isolated against Stage 1 rollback",
            lever_rank_eligible=False,
        ),
        cross.ModelSpec(
            model_id=MODEL_STAGE1,
            mechanism="Stage 1 layered density with Stage 3 disabled",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: "physics_bulk_multilayer_density_v1",
                cross.PHASE_ENV: None,
                STAGE3_ENV: None,
            },
            source="OPENWEPP_SNOWDENSITY09_DENSITY_MODEL opt-in",
            note="rollback baseline for Stage 3 isolation",
            lever_rank_eligible=False,
        ),
        cross.ModelSpec(
            model_id=MODEL_STAGE3,
            mechanism="Stage 3 layered thermal liquid routing and meltwater temperature",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: "physics_bulk_multilayer_density_v1",
                cross.PHASE_ENV: None,
                STAGE3_ENV: "layered_thermal_liquid_v1",
            },
            source=f"{STAGE3_ENV}=layered_thermal_liquid_v1",
            note="PARADIGM-2 Stage 3 opt-in diagnostic candidate; default unchanged",
            lever_rank_eligible=False,
        ),
    ]


def evaluate_gates(
    site_reports: list[dict[str, Any]],
    stage3_vs_stage1: dict[str, Any],
    timing_vs_stage1: dict[str, Any],
) -> dict[str, GateResult]:
    failures = availability_failures(site_reports)
    if failures:
        reason = "model run failed or unavailable: " + "; ".join(failures)
        return {
            "candidate_availability": GateResult(False, reason),
            "snow_guardrail_vs_stage1": GateResult(False, reason),
            "runoff_timing_guardrail_vs_stage1": GateResult(False, reason),
        }
    snow_pass = (
        stage3_vs_stage1["worse_robust_cells"] == 0
        and stage3_vs_stage1["robust_fail_delta_base_minus_candidate"] >= 0
        and stage3_vs_stage1["robust_score_delta_candidate_minus_base"] >= 0
    )
    snow_reason = (
        "stage3-vs-stage1 robust cells better/equal/worse="
        f"{stage3_vs_stage1['better_robust_cells']}/"
        f"{stage3_vs_stage1['equal_robust_cells']}/"
        f"{stage3_vs_stage1['worse_robust_cells']}; fail_delta="
        f"{stage3_vs_stage1['robust_fail_delta_base_minus_candidate']}; score_delta="
        f"{stage3_vs_stage1['robust_score_delta_candidate_minus_base']}"
    )
    timing_pass = (
        timing_vs_stage1["scored_candidate_cell_count"] > 0
        and timing_vs_stage1["worse_robust_cells"] == 0
    )
    timing_reason = (
        "stage3-vs-stage1 timing/runoff cells scored="
        f"{timing_vs_stage1['scored_candidate_cell_count']}; better/equal/worse="
        f"{timing_vs_stage1['better_robust_cells']}/"
        f"{timing_vs_stage1['equal_robust_cells']}/"
        f"{timing_vs_stage1['worse_robust_cells']}"
    )
    return {
        "candidate_availability": GateResult(True, "all direct-runtime model runs completed"),
        "snow_guardrail_vs_stage1": GateResult(snow_pass, snow_reason),
        "runoff_timing_guardrail_vs_stage1": GateResult(timing_pass, timing_reason),
    }


def availability_failures(site_reports: list[dict[str, Any]]) -> list[str]:
    failures = []
    for site in site_reports:
        for model_id in [MODEL_STAGE1, MODEL_STAGE3]:
            model = site["models"][model_id]
            if model.get("availability") == "current_direct_runtime":
                continue
            reason = model.get("unavailable_reason") or f"availability={model.get('availability')}"
            failures.append(f"{site['site_id']} {model_id} {reason}")
    return failures


def compare_models(
    site_reports: list[dict[str, Any]],
    base_model_id: str,
    candidate_model_id: str,
    cell_filter: Callable[[dict[str, Any]], bool] | None = None,
) -> dict[str, Any]:
    better = equal = worse = unpaired = scored = 0
    fail_delta = 0
    score_delta = 0
    counts = Counter()
    rows = []
    for site in site_reports:
        base_profile = site["models"][base_model_id]["rubric_profile"]
        candidate_profile = site["models"][candidate_model_id]["rubric_profile"]
        base_cells = {cell["cell_id"]: cell for cell in base_profile["cells"]}
        for candidate_cell in candidate_profile["cells"]:
            if not candidate_cell["forcing_robust"]:
                continue
            if cell_filter is not None and not cell_filter(candidate_cell):
                continue
            label = candidate_cell["ordinal_label"]
            counts[label] += 1
            if label not in cross.LABEL_SCORE:
                continue
            scored += 1
            base_cell = base_cells.get(candidate_cell["cell_id"])
            if base_cell is None or base_cell["ordinal_label"] not in cross.LABEL_SCORE:
                unpaired += 1
                continue
            base_score = cross.LABEL_SCORE[base_cell["ordinal_label"]]
            candidate_score = cross.LABEL_SCORE[label]
            delta = candidate_score - base_score
            fail_delta += int(base_cell["ordinal_label"] == "fail") - int(label == "fail")
            score_delta += delta
            if delta > 0:
                better += 1
            elif delta < 0:
                worse += 1
            else:
                equal += 1
            rows.append(
                {
                    "site_id": site["site_id"],
                    "corpus": site["corpus"],
                    "cell_id": candidate_cell["cell_id"],
                    "signature": candidate_cell["signature"],
                    "base_label": base_cell["ordinal_label"],
                    "candidate_label": label,
                    "score_delta": delta,
                }
            )
    return {
        "base_model_id": base_model_id,
        "candidate_model_id": candidate_model_id,
        "scored_candidate_cell_count": scored,
        "counts_by_candidate_label": dict(sorted(counts.items())),
        "better_robust_cells": better,
        "equal_robust_cells": equal,
        "worse_robust_cells": worse,
        "unpaired_robust_cells": unpaired,
        "robust_fail_delta_base_minus_candidate": fail_delta,
        "robust_score_delta_candidate_minus_base": score_delta,
        "rows": rows,
    }


def summarize(
    model_summaries: dict[str, Any],
    gates: dict[str, GateResult],
    stage3_vs_stage1: dict[str, Any],
    timing_vs_stage1: dict[str, Any],
    elapsed_seconds: float,
) -> dict[str, Any]:
    default = model_summaries[MODEL_DEFAULT]["aggregate"]
    stage1 = model_summaries[MODEL_STAGE1]["aggregate"]
    stage3 = model_summaries[MODEL_STAGE3]["aggregate"]
    return {
        "deferred_observed_gates_passed": all(gate.passed for gate in gates.values()),
        "promotion_decision_made": False,
        "activation_authorized": False,
        "elapsed_seconds": elapsed_seconds,
        "current_default_robust_fail_count": default["robust_fail_count"],
        "current_default_robust_ordinal_score": default["robust_ordinal_score"],
        "stage1_rollback_robust_fail_count": stage1["robust_fail_count"],
        "stage1_rollback_robust_ordinal_score": stage1["robust_ordinal_score"],
        "stage3_robust_fail_count": stage3["robust_fail_count"],
        "stage3_robust_ordinal_score": stage3["robust_ordinal_score"],
        "stage3_vs_stage1_worse_robust_cells": stage3_vs_stage1["worse_robust_cells"],
        "timing_vs_stage1_worse_robust_cells": timing_vs_stage1["worse_robust_cells"],
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
        "# PARADIGM-2 Stage 3 Observed Guardrails",
        "",
        f"Schema: `{report['schema']}`",
        f"Contract: `{report['contract']}`",
        f"Evidence: `{report['evidence_class']}`",
        "",
        "## Summary",
        "",
        f"- Deferred observed gates passed: `{summary['deferred_observed_gates_passed']}`",
        f"- Current default robust profile: `{summary['current_default_robust_fail_count']}` fails / `{summary['current_default_robust_ordinal_score']}` score",
        f"- Stage 1 rollback robust profile: `{summary['stage1_rollback_robust_fail_count']}` fails / `{summary['stage1_rollback_robust_ordinal_score']}` score",
        f"- Stage 3 robust profile: `{summary['stage3_robust_fail_count']}` fails / `{summary['stage3_robust_ordinal_score']}` score",
        f"- Stage 3 vs Stage 1 worse robust cells: `{summary['stage3_vs_stage1_worse_robust_cells']}`",
        f"- Runoff/timing worse robust cells: `{summary['timing_vs_stage1_worse_robust_cells']}`",
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
            "- Stage 3 is compared to the Stage 1 rollback baseline because it requires the Stage 1 layer stack.",
            "- The current default profile is reported as reference, not as Stage 3 isolation evidence.",
            "- `event_rain_on_snow_response` remains unavailable in the daily observed corpus.",
            "- No promotion/default activation decision is made by this diagnostic.",
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
