#!/usr/bin/env python3
"""Compare the opt-in winter-thaw state-loss candidate against legacy CoE melt.

This is SNOWDENSITY-10.3.7 evidence tooling. It runs
``openwepp-snowbench coe-melt`` for ``legacy_coe`` and
``coe_winter_thaw_state_loss_v1`` over the same surfaces used by
SNOWDENSITY-10.3.6, then reuses that package's event-window analysis to compare
observed thaw-ablation under-response. It does not change production defaults,
fixtures, public output schemas, rain heat, sub-canopy longwave, or coefficients.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import maritime_overaccumulation_diagnosis as maritime  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402
import winter_thaw_melt_response as thaw  # noqa: E402


SCHEMA = "snowdensity10-3-7-winter-thaw-melt-response-correction-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-002 INV-SNOWFREEZE-066 OBL-SNOWFREEZE-P-041"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_7_winter_thaw_melt_response_correction"
PACKAGE_ARTIFACTS = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/artifacts"
)
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
LEGACY_MODEL = "legacy_coe"
CANDIDATE_MODEL = "coe_winter_thaw_state_loss_v1"
MODELS = [LEGACY_MODEL, CANDIDATE_MODEL]
CONSERVATION_TOL_M = 1.0e-9


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--skip-runs", action="store_true")
    args = parser.parse_args(argv)

    report = diagnose(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        run_snowbench=not args.skip_runs,
    )
    print(
        json.dumps(
            {
                "schema": report["schema"],
                "disposition": report["summary"]["disposition"],
                "under_ablation_delta": report["summary"]["opt_in_vs_legacy"][
                    "under_ablation_interval_count_delta"
                ],
                "aggregate_depth_loss_deficit_delta_m": report["summary"][
                    "opt_in_vs_legacy"
                ]["aggregate_depth_loss_deficit_delta_m"],
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


def diagnose(
    output_dir: Path,
    package_artifacts_dir: Path,
    snowbench_binary: Path,
    run_snowbench: bool,
) -> dict[str, Any]:
    if run_snowbench and not snowbench_binary.is_file():
        raise FileNotFoundError(f"openwepp-snowbench binary not found: {snowbench_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    model_reports = {
        model: analyze_model(
            model=model,
            output_dir=output_dir / "models" / model,
            snowbench_binary=snowbench_binary,
            run_snowbench=run_snowbench,
        )
        for model in MODELS
    }
    comparison = compare_summaries(
        model_reports[CANDIDATE_MODEL]["summary"],
        model_reports[LEGACY_MODEL]["summary"],
    )
    conservation = conservation_gate(model_reports)
    disposition = disposition_from_comparison(comparison)
    if disposition == "WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES" and not conservation[
        "candidate_conservation_passed"
    ]:
        disposition = "WINTER-THAW-MELT-RESPONSE-CANDIDATE-HOLD"
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "runtime_coupling": "diagnostic snowbench replay of typed CoE melt path; opt-in only",
        "evidence_mode": "Static/Ran",
        "models": MODELS,
        "legacy_model": LEGACY_MODEL,
        "candidate_model": CANDIDATE_MODEL,
        "no_tuning": True,
        "no_site_constants": True,
        "default_activation_changed": False,
        "parser_runfile_user_cli_selector_added": False,
        "fixture_inputs_changed": False,
        "public_output_schema_changed": False,
        "rain_heat_changed": False,
        "subcanopy_longwave_changed": False,
        "snowbench_binary": str(snowbench_binary),
        "output_dir": str(output_dir),
        "summary": {
            "disposition": disposition,
            "opt_in_vs_legacy": comparison,
            "conservation_gate": conservation,
            "closure_rule": (
                "candidate must reduce both under_ablation_interval_count and "
                "aggregate_depth_loss_deficit_m relative to legacy_coe and pass "
                "the produced-artifact conservation/routing gate"
            ),
        },
        "model_reports": model_reports,
        "static_scope_scan": static_scope_scan_record(),
    }
    rubric.write_json(output_dir / "winter-thaw-melt-response-correction.json", report)
    (output_dir / "winter-thaw-melt-response-correction.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    rubric.write_json(
        package_artifacts_dir / "winter-thaw-melt-response-correction.json",
        report,
    )
    (package_artifacts_dir / "winter-thaw-melt-response-correction.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return report


def analyze_model(
    model: str,
    output_dir: Path,
    snowbench_binary: Path,
    run_snowbench: bool,
) -> dict[str, Any]:
    surfaces = []
    for surface in maritime.SURFACES:
        run_dir = output_dir / "runs" / surface.surface_id
        if run_snowbench:
            run_coe_melt_model(surface.fixture_dir, run_dir, snowbench_binary, model)
        surfaces.append(thaw.analyze_surface(surface, run_dir))
    return {
        "schema": "snowdensity10-3-7-model-event-window-profile-v1",
        "model_id": model,
        "runtime_coupling": "diagnostic snowbench replay only",
        "output_dir": str(output_dir),
        "summary": thaw.summarize(surfaces),
        "surfaces": surfaces,
    }


def run_coe_melt_model(
    fixture_dir: Path,
    output_dir: Path,
    snowbench_binary: Path,
    model: str,
) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    command = [
        str(snowbench_binary),
        "coe-melt",
        "--run-dir",
        str(fixture_dir),
        "--output-dir",
        str(output_dir),
        "--model",
        model,
    ]
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    (output_dir / "openwepp-snowbench.stdout").write_text(completed.stdout, encoding="utf-8")
    (output_dir / "openwepp-snowbench.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(
            f"openwepp-snowbench coe-melt failed for {fixture_dir.name} {model} "
            f"with exit code {completed.returncode}"
        )


def compare_summaries(candidate: dict[str, Any], legacy: dict[str, Any]) -> dict[str, Any]:
    legacy_deficit = float(legacy["total_depth_loss_deficit_m"])
    candidate_deficit = float(candidate["total_depth_loss_deficit_m"])
    legacy_under = int(legacy["under_ablation_interval_count"])
    candidate_under = int(candidate["under_ablation_interval_count"])
    return {
        "legacy_under_ablation_interval_count": legacy_under,
        "candidate_under_ablation_interval_count": candidate_under,
        "under_ablation_interval_count_delta": candidate_under - legacy_under,
        "legacy_aggregate_depth_loss_deficit_m": legacy_deficit,
        "candidate_aggregate_depth_loss_deficit_m": candidate_deficit,
        "aggregate_depth_loss_deficit_m": candidate_deficit,
        "aggregate_depth_loss_deficit_delta_m": candidate_deficit - legacy_deficit,
        "legacy_total_modeled_depth_loss_m": legacy["total_modeled_depth_loss_m"],
        "candidate_total_modeled_depth_loss_m": candidate["total_modeled_depth_loss_m"],
        "legacy_total_raw_melt_m": legacy["total_raw_melt_m"],
        "candidate_total_raw_melt_m": candidate["total_raw_melt_m"],
        "legacy_total_routed_melt_m": legacy["total_routed_melt_m"],
        "candidate_total_routed_melt_m": candidate["total_routed_melt_m"],
        "legacy_total_snowpack_swe_loss_m": legacy["total_snowpack_swe_loss_m"],
        "candidate_total_snowpack_swe_loss_m": candidate["total_snowpack_swe_loss_m"],
        "legacy_total_rain_retained_m": legacy["total_rain_retained_m"],
        "candidate_total_rain_retained_m": candidate["total_rain_retained_m"],
        "legacy_total_rain_released_m": legacy["total_rain_released_m"],
        "candidate_total_rain_released_m": candidate["total_rain_released_m"],
        "legacy_max_abs_swe_balance_residual_m": legacy[
            "max_abs_swe_balance_residual_m"
        ],
        "candidate_max_abs_swe_balance_residual_m": candidate[
            "max_abs_swe_balance_residual_m"
        ],
        "legacy_max_abs_routed_state_loss_residual_m": legacy[
            "max_abs_routed_state_loss_residual_m"
        ],
        "candidate_max_abs_routed_state_loss_residual_m": candidate[
            "max_abs_routed_state_loss_residual_m"
        ],
        "legacy_min_state_loss_available_storage_margin_m": legacy[
            "min_state_loss_available_storage_margin_m"
        ],
        "candidate_min_state_loss_available_storage_margin_m": candidate[
            "min_state_loss_available_storage_margin_m"
        ],
        "under_ablation_improved": candidate_under < legacy_under,
        "deficit_improved": candidate_deficit < legacy_deficit,
    }


def conservation_gate(model_reports: dict[str, dict[str, Any]]) -> dict[str, Any]:
    by_model = {}
    for model, model_report in model_reports.items():
        summary = model_report["summary"]
        max_swe = float(summary["max_abs_swe_balance_residual_m"])
        max_route = float(summary["max_abs_routed_state_loss_residual_m"])
        min_margin = summary["min_state_loss_available_storage_margin_m"]
        min_margin_value = float(min_margin) if min_margin is not None else 0.0
        by_model[model] = {
            "max_abs_swe_balance_residual_m": max_swe,
            "max_abs_routed_state_loss_residual_m": max_route,
            "min_state_loss_available_storage_margin_m": min_margin,
            "swe_balance_passed": max_swe <= CONSERVATION_TOL_M,
            "routed_state_loss_passed": max_route <= CONSERVATION_TOL_M,
            "state_loss_available_storage_passed": min_margin_value >= -CONSERVATION_TOL_M,
        }
        by_model[model]["passed"] = (
            by_model[model]["swe_balance_passed"]
            and by_model[model]["routed_state_loss_passed"]
            and by_model[model]["state_loss_available_storage_passed"]
        )
    return {
        "tolerance_m": CONSERVATION_TOL_M,
        "rule": (
            "daily emitted rows must close prior SWE + snow input + retained rain "
            "- SWE loss - after SWE, route SWE state loss as routed melt after "
            "separating released rain, and never lose more SWE than prior SWE plus "
            "same-day snow/rain input"
        ),
        "models": by_model,
        "candidate_conservation_passed": by_model[CANDIDATE_MODEL]["passed"],
    }


def disposition_from_comparison(comparison: dict[str, Any]) -> str:
    if comparison["under_ablation_improved"] and comparison["deficit_improved"]:
        return "WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES"
    return "WINTER-THAW-MELT-RESPONSE-CANDIDATE-HOLD"


def static_scope_scan_record() -> dict[str, Any]:
    return {
        "evidence_class": "Static",
        "production_default": "legacy_coe remains default; candidate is selected only by explicit typed/snowbench model id.",
        "scope": "State-loss application under positive thaw only; CoE melt terms are unchanged.",
        "rain_heat": "Rain heat remains out of scope; no dmelt correction is made.",
        "longwave": "Sub-canopy longwave remains out of scope; no correction is made.",
        "defaults": "The tool does not change production defaults.",
    }


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    comparison = summary["opt_in_vs_legacy"]
    conservation = summary["conservation_gate"]
    lines = [
        "# SNOWDENSITY-10.3.7 Winter-Thaw Melt Response Correction",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Contract: `{report['contract']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        f"- Legacy model: `{report['legacy_model']}`",
        f"- Candidate model: `{report['candidate_model']}`",
        f"- Disposition: `{summary['disposition']}`",
        f"- Default activation changed: `{report['default_activation_changed']}`",
        f"- Parser/runfile/user CLI selector added: `{report['parser_runfile_user_cli_selector_added']}`",
        f"- Public output schema changed: `{report['public_output_schema_changed']}`",
        "- Rain heat and sub-canopy longwave remain out of scope.",
        "",
        "## Improvement Gate",
        "",
        "| Metric | Legacy | Candidate | Delta |",
        "|---|---:|---:|---:|",
        "| `under_ablation_interval_count` | {legacy} | {candidate} | {delta} |".format(
            legacy=comparison["legacy_under_ablation_interval_count"],
            candidate=comparison["candidate_under_ablation_interval_count"],
            delta=comparison["under_ablation_interval_count_delta"],
        ),
        "| `aggregate_depth_loss_deficit_m` | {legacy} | {candidate} | {delta} |".format(
            legacy=fmt(comparison["legacy_aggregate_depth_loss_deficit_m"]),
            candidate=fmt(comparison["candidate_aggregate_depth_loss_deficit_m"]),
            delta=fmt(comparison["aggregate_depth_loss_deficit_delta_m"]),
        ),
        "| `total_modeled_depth_loss_m` | {legacy} | {candidate} | |".format(
            legacy=fmt(comparison["legacy_total_modeled_depth_loss_m"]),
            candidate=fmt(comparison["candidate_total_modeled_depth_loss_m"]),
        ),
        "| `total_raw_melt_m` | {legacy} | {candidate} | |".format(
            legacy=fmt(comparison["legacy_total_raw_melt_m"]),
            candidate=fmt(comparison["candidate_total_raw_melt_m"]),
        ),
        "| `total_routed_melt_m` | {legacy} | {candidate} | |".format(
            legacy=fmt(comparison["legacy_total_routed_melt_m"]),
            candidate=fmt(comparison["candidate_total_routed_melt_m"]),
        ),
        "| `total_snowpack_swe_loss_m` | {legacy} | {candidate} | |".format(
            legacy=fmt(comparison["legacy_total_snowpack_swe_loss_m"]),
            candidate=fmt(comparison["candidate_total_snowpack_swe_loss_m"]),
        ),
        "| `total_rain_retained_m` | {legacy} | {candidate} | |".format(
            legacy=fmt(comparison["legacy_total_rain_retained_m"]),
            candidate=fmt(comparison["candidate_total_rain_retained_m"]),
        ),
        "| `total_rain_released_m` | {legacy} | {candidate} | |".format(
            legacy=fmt(comparison["legacy_total_rain_released_m"]),
            candidate=fmt(comparison["candidate_total_rain_released_m"]),
        ),
        "",
        "## Conservation Gate",
        "",
        f"- Tolerance: `{conservation['tolerance_m']}` m",
        f"- Candidate conservation passed: `{conservation['candidate_conservation_passed']}`",
        "",
        "| Model | SWE balance residual m | Routed state-loss residual m | Min storage margin m | Passed |",
        "|---|---:|---:|---:|---:|",
    ]
    for model in report["models"]:
        gate = conservation["models"][model]
        lines.append(
            "| `{model}` | {swe} | {routed} | {margin} | `{passed}` |".format(
                model=model,
                swe=fmt(gate["max_abs_swe_balance_residual_m"]),
                routed=fmt(gate["max_abs_routed_state_loss_residual_m"]),
                margin=fmt(gate["min_state_loss_available_storage_margin_m"]),
                passed=gate["passed"],
            )
        )
    lines.extend(
        [
        "",
        ]
    )
    for model in report["models"]:
        model_report = report["model_reports"][model]
        model_summary = model_report["summary"]
        lines.extend(
            [
                f"## `{model}` Surface Summary",
                "",
                "| Metric | Value |",
                "|---|---:|",
                f"| `paired_surface_count` | {model_summary['paired_surface_count']} |",
                f"| `thaw_observed_ablation_interval_count` | {model_summary['thaw_observed_ablation_interval_count']} |",
                f"| `under_ablation_interval_count` | {model_summary['under_ablation_interval_count']} |",
                f"| `under_ablation_fraction` | {fmt(model_summary['under_ablation_fraction'])} |",
                f"| `total_depth_loss_deficit_m` | {fmt(model_summary['total_depth_loss_deficit_m'])} |",
                f"| `total_raw_melt_m` | {fmt(model_summary['total_raw_melt_m'])} |",
                f"| `total_routed_melt_m` | {fmt(model_summary['total_routed_melt_m'])} |",
                f"| `total_snowpack_swe_loss_m` | {fmt(model_summary['total_snowpack_swe_loss_m'])} |",
                "",
            ]
        )
    return "\n".join(lines) + "\n"


def fmt(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


if __name__ == "__main__":
    raise SystemExit(main())
