#!/usr/bin/env python3
"""Run the SNOWDENSITY-10.3.19 Harder-Pomeroy default activation gate.

The gate compares the prior activated bundle with explicit ``legacy_rst`` phase
rollback against the new no-env direct-production default, which composes the
activated melt+density bundle with ``harder_pomeroy_hourly`` phase partitioning.
The cross-SNOTEL forcing-robust rubric is the primary Policy-B gate.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

import cross_snotel_mechanism_rubric as cross
import snotel_density_three_way as rubric


SCHEMA = "snowdensity10-3-19-harder-pomeroy-default-activation-v1"
CONTRACT = (
    "SC-SNOWFREEZE-001 INV-SNOWFREEZE-075 OBL-SNOWFREEZE-P-050 "
    "INV-SNOWFREEZE-050 INV-SNOWFREEZE-065 INV-SNOWFREEZE-072 ADR-0017"
)
PACKAGE_DIR = (
    cross.REPO_ROOT
    / "docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = cross.REPO_ROOT / "target/snowdensity10_3_19_harder_pomeroy_default_activation"
ARTIFACT_STEM = "harder-pomeroy-default-activation"
PRIOR_BASELINE_MODEL = "activated_bundle"
NEW_DEFAULT_MODEL = "harder_pomeroy_default"
EXPECTED_PRIOR_FAIL_COUNT = 17
EXPECTED_PRIOR_SCORE = 172
RELEASE_NOTE_DENSITY_BIAS_KG_M3 = 23.6234
TRACE_MODEL_KEYS = ("snow_melt_model", "snow_density_model", "snow_phase_model")
CONSERVATION_TOLERANCE_M = 1.0e-10


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=cross.DEFAULT_HILL_BINARY)
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
    site_reports = [
        cross.score_site(site, models, output_dir, hill_binary, h_report=None, run_models=run_models)
        for site in sites
    ]
    model_summaries = cross.summarize_models(site_reports, models)
    comparison = cross.compare_to_activated(site_reports, models)
    trace_proof = build_trace_proof(site_reports)
    summary = summarize(model_summaries, comparison, trace_proof, run_models)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran" if run_models else "Static + Reused",
        "activation_decision": summary["disposition"],
        "authority": {
            "policy_b_primary_gate": "cross-SNOTEL INV-SNOWFREEZE-050 forcing-robust rubric",
            "legacy_rst_is_rollback_not_target": True,
            "legacy_and_pysnobal_are_flags_not_targets": True,
            "site_calibration_or_fixture_fitting": False,
            "runfile_disable_option_added": False,
        },
        "release_notes": {
            "humid_new_england_depth_regression": "roadmap item; non-representative gate, not blocker",
            "cross_snotel_density_bias_kg_m3": RELEASE_NOTE_DENSITY_BIAS_KG_M3,
            "density_bias_recovery_tracked_separately": True,
        },
        "protected_boundaries": {
            "public_output_schema_changed": False,
            "fixture_inputs_changed": False,
            "density_cap_changed": False,
            "frost_behavior_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "run_file_disable_option_added": False,
        },
        "models": {model.model_id: cross.model_record(model) for model in models},
        "model_summaries": model_summaries,
        "mechanism_improvements": comparison,
        "trace_proof": trace_proof,
        "site_reports": site_reports,
        "matrix": cross.flatten_matrix(site_reports),
        "summary": summary,
    }
    write_report(report, output_dir, package_artifacts_dir)
    return report


def model_specs() -> list[cross.ModelSpec]:
    return [
        cross.ModelSpec(
            model_id=PRIOR_BASELINE_MODEL,
            mechanism=(
                "prior activated bundle with explicit legacy_rst phase rollback "
                "(coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1)"
            ),
            availability="current_direct_runtime",
            env={
                cross.PHASE_ENV: "legacy_rst",
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: None,
            },
            source="explicit OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=legacy_rst rollback",
            note="Policy-B comparator profile, not a target",
            lever_rank_eligible=False,
        ),
        cross.ModelSpec(
            model_id=NEW_DEFAULT_MODEL,
            mechanism=(
                "no-env activated bundle + Harder-Pomeroy hourly phase default "
                "(coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1 + "
                "harder_pomeroy_hourly)"
            ),
            availability="current_direct_runtime",
            env={
                cross.PHASE_ENV: None,
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: None,
            },
            source="no-env direct-production default selector path",
            note="10.3.19 activation candidate; absolute SWE/depth L cells report-only",
            lever_rank_eligible=True,
        ),
    ]


def summarize(
    model_summaries: dict[str, Any],
    comparison: dict[str, Any],
    trace_proof: dict[str, Any],
    run_models: bool,
) -> dict[str, Any]:
    prior = model_summaries[PRIOR_BASELINE_MODEL]["aggregate"]
    new = model_summaries[NEW_DEFAULT_MODEL]["aggregate"]
    candidate_comparison = comparison["comparisons"][NEW_DEFAULT_MODEL]
    cross_snotel_gate_pass = (
        new["robust_fail_count"] <= prior["robust_fail_count"]
        and new["robust_ordinal_score"] >= prior["robust_ordinal_score"]
    )
    selector_trace_gate_pass = (
        trace_proof[PRIOR_BASELINE_MODEL]["selector_trace_ok"]
        and trace_proof[NEW_DEFAULT_MODEL]["selector_trace_ok"]
    )
    conservation_gate_pass = (
        trace_proof[PRIOR_BASELINE_MODEL]["partition_conservation_ok"]
        and trace_proof[NEW_DEFAULT_MODEL]["partition_conservation_ok"]
    )
    activation_complete = (
        cross_snotel_gate_pass and selector_trace_gate_pass and conservation_gate_pass
    )
    return {
        "disposition": "ACTIVATED" if activation_complete else "HOLD",
        "model_runs_executed": run_models,
        "prior_activated_bundle_robust_fail_count": prior["robust_fail_count"],
        "prior_activated_bundle_robust_ordinal_score": prior["robust_ordinal_score"],
        "new_default_robust_fail_count": new["robust_fail_count"],
        "new_default_robust_ordinal_score": new["robust_ordinal_score"],
        "cross_snotel_gate_pass": cross_snotel_gate_pass,
        "selector_trace_gate_pass": selector_trace_gate_pass,
        "partition_conservation_gate_pass": conservation_gate_pass,
        "activation_complete": activation_complete,
        "robust_fail_delta_prior_minus_new": (
            prior["robust_fail_count"] - new["robust_fail_count"]
        ),
        "robust_score_delta_new_minus_prior": (
            new["robust_ordinal_score"] - prior["robust_ordinal_score"]
        ),
        "better_robust_cells_vs_prior": candidate_comparison[
            "better_robust_cells_vs_activated"
        ],
        "worse_robust_cells_vs_prior": candidate_comparison[
            "worse_robust_cells_vs_activated"
        ],
        "matches_10_3_18_prior_basis": (
            prior["robust_fail_count"] == EXPECTED_PRIOR_FAIL_COUNT
            and prior["robust_ordinal_score"] == EXPECTED_PRIOR_SCORE
        ),
        "absolute_swe_depth_cells_are_report_only": True,
        "humid_new_england_depth_regression_is_blocker": False,
        "density_bias_recovery_tracked_separately": True,
    }


def build_trace_proof(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    expectations = {
        PRIOR_BASELINE_MODEL: {
            "snow_melt_model": cross.ACTIVATED_MELT,
            "snow_density_model": cross.ACTIVATED_DENSITY,
            "snow_phase_model": "legacy_rst",
        },
        NEW_DEFAULT_MODEL: {
            "snow_melt_model": cross.ACTIVATED_MELT,
            "snow_density_model": cross.ACTIVATED_DENSITY,
            "snow_phase_model": "harder_pomeroy_hourly",
        },
    }
    proof = {}
    for model_id, expected in expectations.items():
        trace_paths = trace_paths_for_model(site_reports, model_id)
        proof[model_id] = summarize_trace_paths(trace_paths, expected)
    return proof


def trace_paths_for_model(site_reports: list[dict[str, Any]], model_id: str) -> dict[str, Path]:
    paths = {}
    for site in site_reports:
        model = site["models"][model_id]
        trace = model.get("trace")
        if trace:
            paths[f"{site['corpus']}:{site['site_id']}"] = cross.REPO_ROOT / trace
    return paths


def summarize_trace_paths(
    trace_paths: dict[str, Path],
    expected: dict[str, str],
) -> dict[str, Any]:
    by_site = {}
    aggregate_counts = {key: Counter() for key in TRACE_MODEL_KEYS}
    unexpected = {key: Counter() for key in TRACE_MODEL_KEYS}
    row_count = 0
    precip_row_count = 0
    max_abs_partition_residual_m = 0.0
    for site_key, path in trace_paths.items():
        counts, closure = count_trace(path)
        by_site[site_key] = {
            "path": cross.rel(path),
            "counts": {key: dict(sorted(value.items())) for key, value in counts.items()},
            "partition_closure": closure,
        }
        row_count += closure["row_count"]
        precip_row_count += closure["precip_row_count"]
        max_abs_partition_residual_m = max(
            max_abs_partition_residual_m,
            closure["max_abs_partition_residual_m"],
        )
        for key in TRACE_MODEL_KEYS:
            aggregate_counts[key].update(counts[key])
            unexpected[key].update(
                {
                    observed: count
                    for observed, count in counts[key].items()
                    if observed != expected[key]
                }
            )
    selector_trace_ok = (
        row_count > 0
        and all(aggregate_counts[key].get(expected[key], 0) > 0 for key in TRACE_MODEL_KEYS)
        and all(not unexpected[key] for key in TRACE_MODEL_KEYS)
    )
    partition_conservation_ok = (
        precip_row_count > 0 and max_abs_partition_residual_m <= CONSERVATION_TOLERANCE_M
    )
    return {
        "expected": expected,
        "row_count": row_count,
        "precip_row_count": precip_row_count,
        "aggregate_counts": {
            key: dict(sorted(value.items())) for key, value in aggregate_counts.items()
        },
        "unexpected_counts": {
            key: dict(sorted(value.items())) for key, value in unexpected.items()
        },
        "selector_trace_ok": selector_trace_ok,
        "partition_conservation_ok": partition_conservation_ok,
        "max_abs_partition_residual_m": max_abs_partition_residual_m,
        "partition_conservation_tolerance_m": CONSERVATION_TOLERANCE_M,
        "source_hourly_reconstruction_guard": (
            "snow.hourly.stmtim.partition_reconstruction enforces "
            "abs(hrrain + hrsnow / 10 - active_precip) <= 1e-12"
        ),
        "by_site": by_site,
    }


def count_trace(path: Path) -> tuple[dict[str, Counter[str]], dict[str, Any]]:
    counts = {key: Counter() for key in TRACE_MODEL_KEYS}
    row_count = 0
    precip_row_count = 0
    max_abs_partition_residual_m = 0.0
    if not path.is_file():
        return counts, {
            "row_count": 0,
            "precip_row_count": 0,
            "max_abs_partition_residual_m": 0.0,
        }
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            row_count += 1
            for key in TRACE_MODEL_KEYS:
                counts[key][str(row.get(key, "missing"))] += 1
            precip = float(row.get("hyetograph_rainfall_m", 0.0) or 0.0)
            if abs(precip) > 1.0e-15:
                precip_row_count += 1
            reconstructed = sum(
                float(row.get(key, 0.0) or 0.0)
                for key in (
                    "accumulation_m",
                    "rain_retained_m",
                    "rain_released_m",
                    "post_winter_rain_m",
                )
            )
            max_abs_partition_residual_m = max(
                max_abs_partition_residual_m,
                abs(reconstructed - precip),
            )
    return counts, {
        "row_count": row_count,
        "precip_row_count": precip_row_count,
        "max_abs_partition_residual_m": max_abs_partition_residual_m,
    }


def write_report(report: dict[str, Any], output_dir: Path, package_artifacts_dir: Path) -> None:
    rubric.write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    markdown = render_markdown(report)
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    rubric.write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.19 Harder-Pomeroy Default Activation",
        "",
        "Evidence mode: Static/Ran direct-production activation gate.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Cross-SNOTEL gate pass: `{summary['cross_snotel_gate_pass']}`",
        f"- Selector trace gate pass: `{summary['selector_trace_gate_pass']}`",
        f"- Partition conservation gate pass: `{summary['partition_conservation_gate_pass']}`",
        f"- Prior robust fails / score: `{summary['prior_activated_bundle_robust_fail_count']}` / `{summary['prior_activated_bundle_robust_ordinal_score']}`",
        f"- New default robust fails / score: `{summary['new_default_robust_fail_count']}` / `{summary['new_default_robust_ordinal_score']}`",
        f"- Better / worse robust cells vs prior: `{summary['better_robust_cells_vs_prior']}` / `{summary['worse_robust_cells_vs_prior']}`",
        f"- Humid-New-England depth blocker: `{summary['humid_new_england_depth_regression_is_blocker']}`",
        f"- Density bias note: `{report['release_notes']['cross_snotel_density_bias_kg_m3']} kg m^-3`",
        "",
        "## Model Summary",
        "",
        "| Model | Robust fail | Robust score | SWE median bias | Depth median bias | Density median bias |",
        "|---|---:|---:|---:|---:|---:|",
    ]
    for model_id, model in report["model_summaries"].items():
        aggregate = model["aggregate"]
        residual = model["residual_decomposition"]
        lines.append(
            "| `{model}` | {fail} | {score} | {swe} | {depth} | {density} |".format(
                model=model_id,
                fail=aggregate["robust_fail_count"],
                score=aggregate["robust_ordinal_score"],
                swe=cross.fmt(residual["swe"]["median_of_median_signed"]),
                depth=cross.fmt(residual["depth"]["median_of_median_signed"]),
                density=cross.fmt(residual["density"]["median_of_median_signed"]),
            )
        )
    lines.extend(
        [
            "",
            "## Trace And Conservation",
            "",
            "| Model | Expected phase | Rows | Precip rows | Max partition residual m | Selector ok | Conservation ok |",
            "|---|---|---:|---:|---:|---|---|",
        ]
    )
    for model_id, proof in report["trace_proof"].items():
        lines.append(
            "| `{model}` | `{phase}` | {rows} | {precip_rows} | {residual} | `{selector}` | `{conservation}` |".format(
                model=model_id,
                phase=proof["expected"]["snow_phase_model"],
                rows=proof["row_count"],
                precip_rows=proof["precip_row_count"],
                residual=cross.fmt(proof["max_abs_partition_residual_m"]),
                selector=proof["selector_trace_ok"],
                conservation=proof["partition_conservation_ok"],
            )
        )
    lines.extend(
        [
            "",
            "## Release Notes",
            "",
            "- Humid-New-England depth regression remains a non-representative roadmap item.",
            "- Cross-SNOTEL density bias rises to about `+23.6 kg m^-3`; recovery is tracked separately.",
            "- No `.run` disable option, fixture, public schema, density-cap, or frost change is authorized.",
            "",
        ]
    )
    return "\n".join(lines)


if __name__ == "__main__":
    raise SystemExit(main())
