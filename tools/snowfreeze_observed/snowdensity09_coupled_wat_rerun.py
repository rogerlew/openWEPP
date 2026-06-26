#!/usr/bin/env python3
"""Run SNOWDENSITY-09 default-vs-opt-in non-SNOTEL WAT rerun."""

from __future__ import annotations

import argparse
import contextlib
import json
import os
import sys
from pathlib import Path
from typing import Any, Iterator


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import non_snotel_rubric_baseline as nsb  # noqa: E402


SCHEMA = "snowdensity09-diagnostic-coupled-wat-rerun-v1"
CONTRACT = (
    "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 "
    "INV-SNOWFREEZE-050 INV-SNOWFREEZE-060 INV-SNOWFREEZE-061 "
    "INV-SNOWFREEZE-062 OBL-SNOWFREEZE-P-037"
)
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity09_coupled_wat_rerun"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_SNOWDENSITY08_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260626-snowdensity-08-snow-frost-gate-rerun-001/"
    / "artifacts/snowdensity08_gate_rerun.json"
)
PACKAGE_ARTIFACTS = (
    REPO_ROOT
    / "docs/work-packages/20260626-snowdensity-09-diagnostic-coupled-wat-rerun-001/artifacts"
)
DIAGNOSTIC_MODEL_ENV = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL"
SNOW_TRACE_ENV = "OPENWEPP_R7H_SNOW_TRACE_PATH"
OPT_IN_MODEL = "physics_bulk_density_compaction_v1"
LEGACY_MODEL = "legacy_wepp"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--snowdensity08-report", type=Path, default=DEFAULT_SNOWDENSITY08_REPORT)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = run_gate(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        snowdensity08_report=args.snowdensity08_report.resolve(),
        skip_model_runs=args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def run_gate(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    snowdensity08_report: Path,
    skip_model_runs: bool,
) -> dict[str, Any]:
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    default_trace = output_dir / "default_direct_snow_trace.jsonl"
    opt_in_trace = output_dir / "opt_in_direct_snow_trace.jsonl"
    default_report = run_non_snotel(
        output_dir / "non_snotel_default_path",
        hill_binary,
        skip_model_runs,
        model_id="openwepp_current_legacy_wepp_default",
        density_model_env=None,
        trace_path=default_trace,
    )
    opt_in_report = run_non_snotel(
        output_dir / "non_snotel_physics_bulk_density_compaction_v1",
        hill_binary,
        skip_model_runs,
        model_id="openwepp_diagnostic_physics_bulk_density_compaction_v1",
        density_model_env=OPT_IN_MODEL,
        trace_path=opt_in_trace,
    )
    snowdensity08 = read_json(snowdensity08_report)
    report = build_report(
        output_dir=output_dir,
        snowdensity08_report=snowdensity08_report,
        snowdensity08=snowdensity08,
        default_report=default_report,
        opt_in_report=opt_in_report,
        default_trace=read_trace(default_trace),
        opt_in_trace=read_trace(opt_in_trace),
    )
    write_json(output_dir / "snowdensity09_coupled_wat_rerun.json", report)
    (output_dir / "snowdensity09_coupled_wat_rerun.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    write_json(package_artifacts_dir / "snowdensity09_coupled_wat_rerun.json", report)
    (package_artifacts_dir / "snowdensity09_coupled_wat_rerun.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    return report


def run_non_snotel(
    output_dir: Path,
    hill_binary: Path,
    skip_model_runs: bool,
    model_id: str,
    density_model_env: str | None,
    trace_path: Path,
) -> dict[str, Any]:
    if trace_path.exists():
        trace_path.unlink()
    env_updates = {
        SNOW_TRACE_ENV: str(trace_path),
        DIAGNOSTIC_MODEL_ENV: density_model_env,
    }
    with scoped_env(env_updates):
        nsb.run_baseline(
            observations_dir=nsb.DEFAULT_OBSERVATIONS.resolve(),
            output_dir=output_dir.resolve(),
            binary=hill_binary,
            runtime="direct-production-executor",
            skip_model_runs=skip_model_runs,
            model_id=model_id,
        )
    return read_json(output_dir / "non_snotel_rubric_baseline.json")


@contextlib.contextmanager
def scoped_env(updates: dict[str, str | None]) -> Iterator[None]:
    previous: dict[str, str | None] = {key: os.environ.get(key) for key in updates}
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


def build_report(
    output_dir: Path,
    snowdensity08_report: Path,
    snowdensity08: dict[str, Any],
    default_report: dict[str, Any],
    opt_in_report: dict[str, Any],
    default_trace: list[dict[str, Any]],
    opt_in_trace: list[dict[str, Any]],
) -> dict[str, Any]:
    snotel_gate_cleared = bool(
        snowdensity08["summary"]["snotel_opt_in_density_gate_cleared"]
    )
    default_summary = default_report["summary"]
    opt_in_summary = opt_in_report["summary"]
    default_counts = default_summary["snow_control_gate_status_counts"]
    opt_in_counts = opt_in_summary["snow_control_gate_status_counts"]
    default_snow_control_passed = snow_control_gate_passed(default_summary)
    opt_in_snow_control_passed = snow_control_gate_passed(opt_in_summary)
    trace_proof = trace_model_proof(default_trace, opt_in_trace)
    coupled_opt_in_wat_path = trace_proof["opt_in_trace_selected_count"] > 0
    frost_attribution_authorized = (
        snotel_gate_cleared and opt_in_snow_control_passed and coupled_opt_in_wat_path
    )
    blocker = None
    if not snotel_gate_cleared:
        blocker = "SNOTEL-DENSITY-GATE-NOT-CLEARED"
    elif not coupled_opt_in_wat_path:
        blocker = "DIAGNOSTIC-OPT-IN-WAT-TRACE-ABSENT"
    elif not opt_in_snow_control_passed:
        blocker = "NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED"
    disposition = (
        "COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-AUTHORIZED"
        if frost_attribution_authorized
        else "COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED"
    )
    return {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "output_dir": str(output_dir.relative_to(REPO_ROOT)),
        "summary": {
            "disposition": disposition,
            "snotel_density_gate_cleared_from_snowdensity08": snotel_gate_cleared,
            "default_snow_control_passed": default_snow_control_passed,
            "opt_in_snow_control_passed": opt_in_snow_control_passed,
            "coupled_opt_in_wat_path_available": coupled_opt_in_wat_path,
            "frost_attribution_authorized": frost_attribution_authorized,
            "blocker": blocker,
            "production_physics_changed": False,
            "default_activation_changed": False,
            "parser_runfile_user_cli_activation_added": False,
            "output_schema_changed": False,
            "no_site_constants": True,
            "snow_control_gate_rule": (
                "only sites with observed snow-depth rows participate in the "
                "snow-control gate; sites without observed snow depth are "
                "reported as diagnostic-only out-of-gate evidence"
            ),
            "default_snow_control_gate_site_count": default_summary[
                "snow_control_gate_site_count"
            ],
            "opt_in_snow_control_gate_site_count": opt_in_summary[
                "snow_control_gate_site_count"
            ],
            "default_snow_control_out_of_gate_site_ids": default_summary[
                "snow_control_out_of_gate_site_ids"
            ],
            "opt_in_snow_control_out_of_gate_site_ids": opt_in_summary[
                "snow_control_out_of_gate_site_ids"
            ],
        },
        "diagnostic_selector": {
            "env": DIAGNOSTIC_MODEL_ENV,
            "default_behavior": LEGACY_MODEL,
            "opt_in_value": OPT_IN_MODEL,
            "trace_env": SNOW_TRACE_ENV,
            "trace_proof": trace_proof,
        },
        "snowdensity08_source": {
            "path": str(snowdensity08_report.relative_to(REPO_ROOT)),
            "snotel_best_model": snowdensity08["summary"]["snotel_best_model"],
            "snotel_best_boundary": snowdensity08["summary"]["snotel_best_boundary"],
        },
        "default_non_snotel": compact_non_snotel(default_report),
        "opt_in_non_snotel": compact_non_snotel(opt_in_report),
        "site_deltas": site_deltas(default_report, opt_in_report),
        "raw_outputs": {
            "default_json": str(
                (output_dir / "non_snotel_default_path/non_snotel_rubric_baseline.json")
                .relative_to(REPO_ROOT)
            ),
            "opt_in_json": str(
                (
                    output_dir
                    / "non_snotel_physics_bulk_density_compaction_v1/"
                    / "non_snotel_rubric_baseline.json"
                ).relative_to(REPO_ROOT)
            ),
            "default_trace": str((output_dir / "default_direct_snow_trace.jsonl").relative_to(REPO_ROOT)),
            "opt_in_trace": str((output_dir / "opt_in_direct_snow_trace.jsonl").relative_to(REPO_ROOT)),
        },
    }


def snow_control_gate_passed(summary: dict[str, Any]) -> bool:
    gate_counts = summary["snow_control_gate_status_counts"]
    return summary["snow_control_gate_site_count"] > 0 and set(gate_counts) == {
        "SNOW_CONTROL_PASSED"
    }


def compact_non_snotel(report: dict[str, Any]) -> dict[str, Any]:
    return {
        "summary": report["summary"],
        "sites": [
            {
                "site_id": site["site_id"],
                "snow_control_status": site["snow_control_status"],
                "metrics": site["metrics"],
                "rubric_summary": site["rubric_profile"]["summary"],
            }
            for site in report["sites"]
        ],
    }


def site_deltas(
    default_report: dict[str, Any],
    opt_in_report: dict[str, Any],
) -> list[dict[str, Any]]:
    by_site = {site["site_id"]: site for site in default_report["sites"]}
    deltas = []
    for opt_in_site in opt_in_report["sites"]:
        site_id = opt_in_site["site_id"]
        default_site = by_site[site_id]
        default_metrics = default_site["metrics"]
        opt_in_metrics = opt_in_site["metrics"]
        deltas.append(
            {
                "site_id": site_id,
                "default_snow_control_status": default_site["snow_control_status"],
                "opt_in_snow_control_status": opt_in_site["snow_control_status"],
                "default_mean_signed_snow_depth_residual_m": default_metrics.get(
                    "mean_signed_snow_depth_residual_m"
                ),
                "opt_in_mean_signed_snow_depth_residual_m": opt_in_metrics.get(
                    "mean_signed_snow_depth_residual_m"
                ),
                "default_max_abs_snow_depth_residual_m": default_metrics.get(
                    "max_abs_snow_depth_residual_m"
                ),
                "opt_in_max_abs_snow_depth_residual_m": opt_in_metrics.get(
                    "max_abs_snow_depth_residual_m"
                ),
                "default_rubric_summary": default_site["rubric_profile"]["summary"],
                "opt_in_rubric_summary": opt_in_site["rubric_profile"]["summary"],
            }
        )
    return deltas


def trace_model_proof(
    default_trace: list[dict[str, Any]],
    opt_in_trace: list[dict[str, Any]],
) -> dict[str, Any]:
    default_models = count_trace_models(default_trace)
    opt_in_models = count_trace_models(opt_in_trace)
    return {
        "default_trace_row_count": len(default_trace),
        "opt_in_trace_row_count": len(opt_in_trace),
        "default_models": default_models,
        "opt_in_models": opt_in_models,
        "default_trace_legacy_count": default_models.get(LEGACY_MODEL, 0),
        "default_trace_opt_in_count": default_models.get(OPT_IN_MODEL, 0),
        "opt_in_trace_selected_count": opt_in_models.get(OPT_IN_MODEL, 0),
    }


def count_trace_models(rows: list[dict[str, Any]]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in rows:
        model = str(row.get("snow_density_model", "missing"))
        counts[model] = counts.get(model, 0) + 1
    return counts


def read_trace(path: Path) -> list[dict[str, Any]]:
    if not path.is_file():
        return []
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-09 Diagnostic Coupled WAT Rerun",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Blocker: `{summary['blocker']}`",
        f"- SNOTEL density gate cleared: `{summary['snotel_density_gate_cleared_from_snowdensity08']}`",
        f"- Coupled opt-in WAT path available: `{summary['coupled_opt_in_wat_path_available']}`",
        f"- Opt-in snow-control passed: `{summary['opt_in_snow_control_passed']}`",
        f"- Frost attribution authorized: `{summary['frost_attribution_authorized']}`",
        f"- Default snow-control counts: `{report['default_non_snotel']['summary']['snow_control_status_counts']}`",
        f"- Default snow-control gate counts: `{report['default_non_snotel']['summary']['snow_control_gate_status_counts']}`",
        f"- Opt-in snow-control counts: `{report['opt_in_non_snotel']['summary']['snow_control_status_counts']}`",
        f"- Opt-in snow-control gate counts: `{report['opt_in_non_snotel']['summary']['snow_control_gate_status_counts']}`",
        f"- Diagnostic-only out-of-gate sites: `{summary['opt_in_snow_control_out_of_gate_site_ids']}`",
        f"- Trace proof: `{report['diagnostic_selector']['trace_proof']}`",
        "",
        "## Site Deltas",
        "",
        "| Site | Default status | Opt-in status | Default mean snow residual m | Opt-in mean snow residual m |",
        "|---|---|---|---:|---:|",
    ]
    for site in report["site_deltas"]:
        lines.append(
            "| {site_id} | `{default_snow_control_status}` | `{opt_in_snow_control_status}` | {default_mean_signed_snow_depth_residual_m} | {opt_in_mean_signed_snow_depth_residual_m} |".format(
                **site
            )
        )
    lines.append("")
    return "\n".join(lines)


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    raise SystemExit(main())
