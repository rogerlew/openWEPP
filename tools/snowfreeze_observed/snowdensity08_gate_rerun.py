#!/usr/bin/env python3
"""Run the SNOWDENSITY-08 snow/frost gate rerun.

The package deliberately separates SNOTEL density evidence from non-SNOTEL
frost-site evidence.  SNOTEL can use the accepted CoE-bound density replay;
non-SNOTEL frost attribution requires an authorized coupled opt-in WAT path.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import coe_bound_density_adjudication as cba  # noqa: E402
import non_snotel_rubric_baseline as nsb  # noqa: E402
import snotel_density_three_way as snotel  # noqa: E402


SCHEMA = "snowdensity08-snow-frost-gate-rerun-v1"
CONTRACT = (
    "SC-SNOWFREEZE-001 INV-SNOWFREEZE-048 INV-SNOWFREEZE-050 "
    "INV-SNOWFREEZE-059 INV-SNOWFREEZE-060 INV-SNOWFREEZE-061 "
    "OBL-SNOWFREEZE-P-036"
)
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity08_gate_rerun"
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_H_COMPARATOR = REPO_ROOT / "target/snowfrost_fidelity_h/three_way_comparison.json"
DIRECT_PUBLICATION_BUILDER = (
    REPO_ROOT
    / "crates/openwepp-runner/src/hillslope/direct_publication/"
    / "day_input_and_helpers/00_builders_and_authority.rs"
)
PACKAGE_ARTIFACTS = (
    REPO_ROOT
    / "docs/work-packages/20260626-snowdensity-08-snow-frost-gate-rerun-001/artifacts"
)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--h-comparator-json", type=Path, default=DEFAULT_H_COMPARATOR)
    parser.add_argument("--skip-non-snotel-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = run_gate_rerun(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        hill_binary=args.hill_binary.resolve(),
        h_comparator_json=args.h_comparator_json.resolve(),
        skip_non_snotel_model_runs=args.skip_non_snotel_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def run_gate_rerun(
    output_dir: Path,
    package_artifacts_dir: Path,
    snowbench_binary: Path,
    hill_binary: Path,
    h_comparator_json: Path,
    skip_non_snotel_model_runs: bool,
) -> dict[str, Any]:
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    snotel_report = run_snotel_gate(output_dir, snowbench_binary, h_comparator_json)
    non_snotel_report = run_non_snotel_gate(output_dir, hill_binary, skip_non_snotel_model_runs)
    default_selector = direct_default_density_selector()
    report = build_decision_report(
        output_dir=output_dir,
        snotel_report=snotel_report,
        non_snotel_report=non_snotel_report,
        default_selector=default_selector,
    )
    write_json(output_dir / "snowdensity08_gate_rerun.json", report)
    (output_dir / "snowdensity08_gate_rerun.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    write_json(package_artifacts_dir / "snowdensity08_gate_rerun.json", report)
    (package_artifacts_dir / "snowdensity08_gate_rerun.md").write_text(
        render_markdown(report), encoding="utf-8"
    )
    return report


def run_snotel_gate(
    output_dir: Path,
    snowbench_binary: Path,
    h_comparator_json: Path,
) -> dict[str, Any]:
    report = cba.adjudicate(
        observations_dir=cba.DEFAULT_OBSERVATIONS.resolve(),
        output_dir=(output_dir / "snotel_coe_bound_density").resolve(),
        snowbench_binary=snowbench_binary,
        h_comparator_json=h_comparator_json,
        boundaries=cba.DEFAULT_BOUNDARIES,
        variant=cba.DEFAULT_VARIANT,
        sites=snotel.SITES,
    )
    write_json(output_dir / "snotel_coe_bound_density" / "coe_bound_density_adjudication.json", report)
    (output_dir / "snotel_coe_bound_density" / "coe_bound_density_adjudication.md").write_text(
        cba.render_markdown(report), encoding="utf-8"
    )
    return report


def run_non_snotel_gate(
    output_dir: Path,
    hill_binary: Path,
    skip_model_runs: bool,
) -> dict[str, Any]:
    nsb.run_baseline(
        observations_dir=nsb.DEFAULT_OBSERVATIONS.resolve(),
        output_dir=(output_dir / "non_snotel_default_path").resolve(),
        binary=hill_binary,
        runtime="direct-production-executor",
        skip_model_runs=skip_model_runs,
    )
    return read_json(output_dir / "non_snotel_default_path" / "non_snotel_rubric_baseline.json")


def direct_default_density_selector() -> dict[str, Any]:
    source = DIRECT_PUBLICATION_BUILDER.read_text(encoding="utf-8")
    legacy_marker = (
        "snow_density_model: openwepp_hillslope_orchestrator::"
        "SnowDensityModel::LegacyWepp"
    )
    opt_in_marker = "SnowDensityModel::PhysicsBulkDensityCompactionV1"
    return {
        "builder_path": str(DIRECT_PUBLICATION_BUILDER.relative_to(REPO_ROOT)),
        "legacy_wepp_marker_present": legacy_marker in source,
        "physics_bulk_marker_present": opt_in_marker in source,
        "non_snotel_runtime_opt_in_coupled": False,
        "reason": (
            "SNOWDENSITY-07 intentionally exposes the density model only to typed "
            "callers; the surface-driven direct publication builder remains "
            "legacy_wepp and there is no parser/runfile/CLI density selector."
        ),
    }


def build_decision_report(
    output_dir: Path,
    snotel_report: dict[str, Any],
    non_snotel_report: dict[str, Any],
    default_selector: dict[str, Any],
) -> dict[str, Any]:
    snotel_summary = snotel_report["summary"]
    non_snotel_summary = non_snotel_report["summary"]
    snotel_gate_cleared = (
        snotel_summary["disposition"] == "PROMOTION-CANDIDATE"
        and snotel_summary["beats_openwepp_as_built"]
        and snotel_summary["beats_legacy_as_built"]
    )
    non_snotel_status_counts = non_snotel_summary["snow_control_status_counts"]
    default_snow_control_passed = set(non_snotel_status_counts) == {"SNOW_CONTROL_PASSED"}
    non_snotel_runtime_opt_in_coupled = bool(
        default_selector["non_snotel_runtime_opt_in_coupled"]
    )
    frost_attribution_authorized = (
        snotel_gate_cleared
        and default_snow_control_passed
        and non_snotel_runtime_opt_in_coupled
    )
    disposition = (
        "COMPLETE-08-SNOW-FROST-GATE-CLEARED"
        if frost_attribution_authorized
        else "COMPLETE-08-SNOTEL-CLEARED-FROST-ATTRIBUTION-BLOCKED"
    )
    blocker = None
    if not snotel_gate_cleared:
        blocker = "SNOTEL-DENSITY-GATE-NOT-CLEARED"
    elif not non_snotel_runtime_opt_in_coupled:
        blocker = "NON-SNOTEL-COUPLED-OPT-IN-WAT-PATH-ABSENT"
    elif not default_snow_control_passed:
        blocker = "NON-SNOTEL-SNOW-CONTROL-FAILED"

    return {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "output_dir": str(output_dir.relative_to(REPO_ROOT)),
        "summary": {
            "disposition": disposition,
            "snotel_opt_in_density_gate_cleared": snotel_gate_cleared,
            "snotel_best_model": snotel_summary["best_model"],
            "snotel_best_boundary": snotel_summary["best_boundary"],
            "non_snotel_default_snow_control_passed": default_snow_control_passed,
            "non_snotel_runtime_opt_in_coupled": non_snotel_runtime_opt_in_coupled,
            "frost_attribution_authorized": frost_attribution_authorized,
            "blocker": blocker,
            "next_route": (
                "Build an authorized diagnostic coupled opt-in WAT/publication "
                "path for non-SNOTEL frost fixtures, or keep frost attribution "
                "blocked while snow-depth control fails on the default path."
            ),
            "production_physics_changed": False,
            "default_activation_changed": False,
            "no_site_constants": True,
        },
        "snotel": summarize_snotel(snotel_report),
        "non_snotel": summarize_non_snotel(non_snotel_report),
        "default_selector": default_selector,
        "raw_outputs": {
            "snotel_json": str(
                (output_dir / "snotel_coe_bound_density/coe_bound_density_adjudication.json")
                .relative_to(REPO_ROOT)
            ),
            "non_snotel_json": str(
                (output_dir / "non_snotel_default_path/non_snotel_rubric_baseline.json")
                .relative_to(REPO_ROOT)
            ),
        },
    }


def summarize_snotel(report: dict[str, Any]) -> dict[str, Any]:
    return {
        "summary": report["summary"],
        "comparators": {
            key: compact_profile(value) for key, value in report["comparators"].items()
        },
        "candidates": [compact_candidate(candidate) for candidate in report["candidates"]],
    }


def compact_candidate(candidate: dict[str, Any]) -> dict[str, Any]:
    compact = compact_profile(candidate)
    compact.update(
        {
            "coe_boundary_model": candidate["coe_boundary_model"],
            "density_variant": candidate["density_variant"],
            "max_abs_coe_swe_identity_residual_m": candidate[
                "max_abs_coe_swe_identity_residual_m"
            ],
            "max_abs_unbounded_swe_residual_m": candidate[
                "max_abs_unbounded_swe_residual_m"
            ],
            "cell_comparison": candidate["cell_comparison"],
        }
    )
    return compact


def compact_profile(profile: dict[str, Any]) -> dict[str, Any]:
    return {
        "model_id": profile["model_id"],
        "robust_fail_count": profile["robust_fail_count"],
        "robust_ordinal_score": profile["robust_ordinal_score"],
        "density_cell_profile": {
            "fail_count": profile["density_cell_profile"]["fail_count"],
            "ordinal_score": profile["density_cell_profile"]["ordinal_score"],
            "counts_by_label": profile["density_cell_profile"]["counts_by_label"],
        },
        "forcing_robust_counts_by_label": profile["forcing_robust_counts_by_label"],
    }


def summarize_non_snotel(report: dict[str, Any]) -> dict[str, Any]:
    return {
        "schema": report["schema"],
        "runtime": report["runtime"],
        "site_count": report["site_count"],
        "summary": report["summary"],
        "sites": [
            {
                "site_id": site["site_id"],
                "method": site["method"],
                "snow_control_status": site["snow_control_status"],
                "robust_counts_by_label": site["rubric_profile"]["summary"][
                    "forcing_robust_counts_by_label"
                ],
                "key_metrics": site["metrics"],
            }
            for site in report["sites"]
        ],
    }


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-08 Snow/Frost Gate Rerun",
        "",
        "Evidence class: Static + Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- SNOTEL density gate cleared: `{summary['snotel_opt_in_density_gate_cleared']}`",
        f"- Non-SNOTEL coupled opt-in WAT path: `{summary['non_snotel_runtime_opt_in_coupled']}`",
        f"- Frost attribution authorized: `{summary['frost_attribution_authorized']}`",
        f"- Blocker: `{summary['blocker']}`",
        f"- Next route: {summary['next_route']}",
        "",
        "## SNOTEL Summary",
        "",
        "| Model | Boundary | Robust fail | Robust score | Density fail | Density score | CoE SWE residual |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: |",
    ]
    for candidate in report["snotel"]["candidates"]:
        lines.append(
            "| `{model}` | `{boundary}` | {robust_fail} | {robust_score} | {density_fail} | {density_score} | {swe:.3e} |".format(
                model=candidate["model_id"],
                boundary=candidate["coe_boundary_model"],
                robust_fail=candidate["robust_fail_count"],
                robust_score=candidate["robust_ordinal_score"],
                density_fail=candidate["density_cell_profile"]["fail_count"],
                density_score=candidate["density_cell_profile"]["ordinal_score"],
                swe=candidate["max_abs_coe_swe_identity_residual_m"],
            )
        )
    lines.extend(
        [
            "",
            "## Non-SNOTEL Summary",
            "",
            f"- Runtime: `{report['non_snotel']['runtime']}`",
            f"- Snow-control counts: `{report['non_snotel']['summary']['snow_control_status_counts']}`",
            f"- Robust counts: `{report['non_snotel']['summary']['forcing_robust_rubric_counts_by_label']}`",
            "",
            "| Site | Snow control | Robust counts |",
            "| --- | --- | --- |",
        ]
    )
    for site in report["non_snotel"]["sites"]:
        lines.append(
            "| `{site_id}` | `{snow}` | `{robust}` |".format(
                site_id=site["site_id"],
                snow=site["snow_control_status"],
                robust=site["robust_counts_by_label"],
            )
        )
    lines.extend(
        [
            "",
            "## Disposition",
            "",
            "SNOTEL density evidence remains promotion-candidate evidence for the opt-in lineage, but frost attribution stays blocked. The current non-SNOTEL WAT rerun is still the default `legacy_wepp` density path, and the repository intentionally has no parser/runfile/CLI selector that can produce a coupled opt-in frost-site WAT surface. Per `INV-SNOWFREEZE-061`, offline snow-only depth cannot be substituted for a coupled frost run.",
            "",
        ]
    )
    return "\n".join(lines)


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    raise SystemExit(main())
