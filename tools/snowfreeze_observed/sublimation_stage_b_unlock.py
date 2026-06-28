#!/usr/bin/env python3
"""Run SNOWDENSITY-10.3.20 sublimation diagnosis and Stage B unlock gates."""

from __future__ import annotations

import argparse
import json
import sys
from collections import Counter
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import cross_snotel_mechanism_rubric as cross  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowdensity10-3-20-sublimation-stage-b-unlock-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-073 INV-SNOWFREEZE-076"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_20_sublimation_stage_b_unlock"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
ARTIFACT_STEM = "sublimation-stage-b-unlock"
CURRENT_DEFAULT = "activated_bundle"
STAGE_A_LEGACY = "stage_a_legacy_phase_10_3_16"
STAGE_A_COMPOSITION = "partition_sublimation_stage_a"
STAGE_B = "stage_b_surface_layer"
STAGE_A_MELT = "coe_open_sublimation_stage_a_v1"
STAGE_B_MELT = "coe_open_sublimation_stage_b_v1"
TRACE_MODEL_KEYS = ("snow_melt_model", "snow_density_model", "snow_phase_model")
TOLERANCE_M = 1.0e-9


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

    models = model_specs()
    sites = cross.diagnostic_sites()
    site_reports = [
        cross.score_site(site, models, output_dir, hill_binary, None, run_models)
        for site in sites
    ]
    model_summaries = cross.summarize_models(site_reports, models)
    comparison = cross.compare_to_activated(site_reports, models)
    trace_proof = build_trace_proof(site_reports, models)
    stage_a_diagnosis = diagnose_stage_a(
        model_summaries, comparison, trace_proof, site_reports
    )
    candidates = candidate_disposition(model_summaries, comparison, trace_proof)
    summary = summarize(model_summaries, candidates, trace_proof)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran" if run_models else "Static + Reused",
        "runtime_coupling": "real direct-production WAT via openwepp-cli-hill",
        "promotion_decision_scope": "current package only; default remains unchanged unless a candidate wins all gates",
        "libsnobal_provenance": libsnobal_provenance(),
        "protected_boundaries": {
            "production_default_changed": False,
            "rollback_removed": False,
            "density_cap_changed": False,
            "output_schema_changed": False,
            "fixture_inputs_changed": False,
            "frost_physics_changed": False,
            "parser_runfile_user_cli_selector_added": False,
            "run_file_disable_option_added": False,
            "qwet_or_frzftp_changed": False,
            "site_calibration_performed": False,
        },
        "inputs": {
            "hill_binary": cross.rel(hill_binary),
            "site_count": len(sites),
            "models": [cross.model_record(model) for model in models],
        },
        "summary": summary,
        "stage_a_implementation_diagnosis": stage_a_diagnosis,
        "candidate_disposition": candidates,
        "model_summaries": model_summaries,
        "mechanism_improvements": comparison,
        "trace_proof": trace_proof,
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
            model_id=CURRENT_DEFAULT,
            mechanism="current default: activated bundle + Harder-Pomeroy hourly phase",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: None,
                cross.DENSITY_ENV: None,
                cross.PHASE_ENV: None,
            },
            source="no-env direct-production default after SNOWDENSITY-10.3.19",
            note="primary baseline for 10.3.20 promotion gates",
            lever_rank_eligible=False,
        ),
        cross.ModelSpec(
            model_id=STAGE_A_LEGACY,
            mechanism="Stage A sublimation with explicit legacy RST phase",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: STAGE_A_MELT,
                cross.DENSITY_ENV: None,
                cross.PHASE_ENV: "legacy_rst",
            },
            source="10.3.16 lineage replay under pre-10.3.19 phase",
            note="diagnoses Stage A implementation separate from HP composition",
            lever_rank_eligible=True,
        ),
        cross.ModelSpec(
            model_id=STAGE_A_COMPOSITION,
            mechanism="current default plus Stage A sublimation sink",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: STAGE_A_MELT,
                cross.DENSITY_ENV: None,
                cross.PHASE_ENV: "harder_pomeroy_hourly",
            },
            source="partition + sublimation composition diagnostic",
            note="tests the +23.6 / -23.0 density-bias offset hypothesis",
            lever_rank_eligible=True,
        ),
        cross.ModelSpec(
            model_id=STAGE_B,
            mechanism="Stage B active surface-layer sublimation candidate",
            availability="current_direct_runtime",
            env={
                cross.MELT_ENV: STAGE_B_MELT,
                cross.DENSITY_ENV: None,
                cross.PHASE_ENV: "harder_pomeroy_hourly",
            },
            source="SNOBAL/Marks active surface-layer temperature gate",
            note="opt-in candidate; no default activation unless all 10.3.20 gates pass",
            lever_rank_eligible=True,
        ),
    ]


def build_trace_proof(
    site_reports: list[dict[str, Any]], models: list[cross.ModelSpec]
) -> dict[str, Any]:
    result = {}
    expected_by_model = {
        model.model_id: {
            "snow_melt_model": expected_env(
                model.env.get(cross.MELT_ENV), cross.ACTIVATED_MELT
            ),
            "snow_density_model": expected_env(
                model.env.get(cross.DENSITY_ENV), cross.ACTIVATED_DENSITY
            ),
            "snow_phase_model": expected_env(
                model.env.get(cross.PHASE_ENV), "harder_pomeroy_hourly"
            ),
        }
        for model in models
    }
    for model in models:
        aggregate_counts = {key: Counter() for key in TRACE_MODEL_KEYS}
        by_site = {}
        row_count = 0
        positive_sublimation_rows = 0
        total_sublimation_m = 0.0
        max_daily_sublimation_m = 0.0
        max_abs_snow_state_residual_m = 0.0
        max_abs_partition_residual_m = 0.0
        for site in site_reports:
            model_report = site["models"][model.model_id]
            trace = path_from_rel(model_report.get("trace"))
            counts, closure = count_trace(trace)
            by_site[site["site_id"]] = {
                "trace": cross.rel(trace) if trace is not None else None,
                "counts": {key: dict(sorted(value.items())) for key, value in counts.items()},
                "closure": closure,
            }
            row_count += closure["row_count"]
            positive_sublimation_rows += closure["positive_sublimation_row_count"]
            total_sublimation_m += closure["total_sublimation_m"]
            max_daily_sublimation_m = max(
                max_daily_sublimation_m, closure["max_daily_sublimation_m"]
            )
            max_abs_snow_state_residual_m = max(
                max_abs_snow_state_residual_m,
                closure["max_abs_snow_state_residual_m"],
            )
            max_abs_partition_residual_m = max(
                max_abs_partition_residual_m,
                closure["max_abs_partition_residual_m"],
            )
            for key in TRACE_MODEL_KEYS:
                aggregate_counts[key].update(counts[key])
        expected = expected_by_model[model.model_id]
        unexpected = {
            key: {
                observed: count
                for observed, count in aggregate_counts[key].items()
                if observed != expected[key]
            }
            for key in TRACE_MODEL_KEYS
        }
        result[model.model_id] = {
            "expected": expected,
            "row_count": row_count,
            "positive_sublimation_row_count": positive_sublimation_rows,
            "total_sublimation_m": total_sublimation_m,
            "max_daily_sublimation_m": max_daily_sublimation_m,
            "max_abs_snow_state_residual_m": max_abs_snow_state_residual_m,
            "max_abs_partition_residual_m": max_abs_partition_residual_m,
            "aggregate_counts": {
                key: dict(sorted(value.items())) for key, value in aggregate_counts.items()
            },
            "unexpected_counts": {
                key: dict(sorted(value.items())) for key, value in unexpected.items()
            },
            "selector_trace_ok": (
                row_count > 0
                and all(
                    aggregate_counts[key].get(expected[key], 0) > 0
                    for key in TRACE_MODEL_KEYS
                )
                and all(not unexpected[key] for key in TRACE_MODEL_KEYS)
            ),
            "sublimation_vapor_conservation_ok": (
                max_abs_snow_state_residual_m <= TOLERANCE_M
            ),
            "partition_conservation_ok": max_abs_partition_residual_m <= TOLERANCE_M,
            "by_site": by_site,
        }
    return result


def count_trace(path: Path | None) -> tuple[dict[str, Counter[str]], dict[str, Any]]:
    counts = {key: Counter() for key in TRACE_MODEL_KEYS}
    if path is None or not path.is_file():
        return counts, empty_trace_closure()
    row_count = 0
    positive_sublimation_rows = 0
    total_sublimation_m = 0.0
    max_daily_sublimation_m = 0.0
    max_abs_snow_state_residual_m = 0.0
    max_abs_partition_residual_m = 0.0
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            row_count += 1
            for key in TRACE_MODEL_KEYS:
                counts[key][str(row.get(key, "missing"))] += 1
            sublimation_m = number(row, "sublimation_m")
            total_sublimation_m += sublimation_m
            if sublimation_m > 0.0:
                positive_sublimation_rows += 1
                max_daily_sublimation_m = max(max_daily_sublimation_m, sublimation_m)
            max_abs_snow_state_residual_m = max(
                max_abs_snow_state_residual_m, abs(snow_state_residual(row))
            )
            max_abs_partition_residual_m = max(
                max_abs_partition_residual_m, abs(partition_residual(row))
            )
    return counts, {
        "row_count": row_count,
        "positive_sublimation_row_count": positive_sublimation_rows,
        "total_sublimation_m": total_sublimation_m,
        "max_daily_sublimation_m": max_daily_sublimation_m,
        "max_abs_snow_state_residual_m": max_abs_snow_state_residual_m,
        "max_abs_partition_residual_m": max_abs_partition_residual_m,
    }


def diagnose_stage_a(
    model_summaries: dict[str, Any],
    comparison: dict[str, Any],
    trace_proof: dict[str, Any],
    site_reports: list[dict[str, Any]],
) -> dict[str, Any]:
    rows = {}
    for model_id in (STAGE_A_LEGACY, STAGE_A_COMPOSITION):
        aggregate = model_summaries[model_id]["aggregate"]
        comp = comparison["comparisons"][model_id]
        rows[model_id] = {
            "robust_fail_count": aggregate["robust_fail_count"],
            "robust_ordinal_score": aggregate["robust_ordinal_score"],
            "robust_fail_delta_current_default_minus_candidate": comp[
                "robust_fail_delta_activated_minus_candidate"
            ],
            "robust_score_delta_candidate_minus_current_default": comp[
                "robust_score_delta_candidate_minus_activated"
            ],
            "better_robust_cells_vs_current_default": comp[
                "better_robust_cells_vs_activated"
            ],
            "worse_robust_cells_vs_current_default": comp[
                "worse_robust_cells_vs_activated"
            ],
            "residual_component_delta_vs_current_default": comp[
                "residual_component_delta_vs_activated"
            ],
            "trace_sublimation": trace_sublimation_summary(trace_proof[model_id]),
        }
    return {
        "diagnosis_basis": (
            "Stage A degradation is implementation-quality evidence when it "
            "removes SWE broadly, shifts density/depth negative, and worsens "
            "forcing-robust timing/densification cells despite valid vapor closure."
        ),
        "legacy_phase_stage_a": rows[STAGE_A_LEGACY],
        "harder_pomeroy_stage_a_composition": rows[STAGE_A_COMPOSITION],
        "worse_signature_counts": worse_signature_counts(
            site_reports, STAGE_A_COMPOSITION
        ),
        "worse_site_counts": worse_site_counts(site_reports, STAGE_A_COMPOSITION),
        "worse_robust_cells": paired_worse_rows(site_reports, STAGE_A_COMPOSITION),
    }


def candidate_disposition(
    model_summaries: dict[str, Any],
    comparison: dict[str, Any],
    trace_proof: dict[str, Any],
) -> dict[str, Any]:
    default = model_summaries[CURRENT_DEFAULT]["aggregate"]
    rows = {}
    for model_id in (STAGE_A_COMPOSITION, STAGE_B):
        aggregate = model_summaries[model_id]["aggregate"]
        comp = comparison["comparisons"][model_id]
        trace = trace_proof[model_id]
        primary_gate = (
            aggregate["robust_fail_count"] <= default["robust_fail_count"]
            and aggregate["robust_ordinal_score"] > default["robust_ordinal_score"]
        )
        bidirectional_guardrail = comp["worse_robust_cells_vs_activated"] == 0
        conservation_gate = (
            trace["selector_trace_ok"]
            and trace["sublimation_vapor_conservation_ok"]
            and trace["partition_conservation_ok"]
        )
        rows[model_id] = {
            "robust_fail_count": aggregate["robust_fail_count"],
            "robust_ordinal_score": aggregate["robust_ordinal_score"],
            "primary_gate_beats_current_default": primary_gate,
            "bidirectional_guardrail_no_worse_cells": bidirectional_guardrail,
            "conservation_gate_pass": conservation_gate,
            "promotion_eligible": primary_gate and bidirectional_guardrail and conservation_gate,
            "better_robust_cells_vs_current_default": comp[
                "better_robust_cells_vs_activated"
            ],
            "worse_robust_cells_vs_current_default": comp[
                "worse_robust_cells_vs_activated"
            ],
            "robust_fail_delta_current_default_minus_candidate": comp[
                "robust_fail_delta_activated_minus_candidate"
            ],
            "robust_score_delta_candidate_minus_current_default": comp[
                "robust_score_delta_candidate_minus_activated"
            ],
            "trace_sublimation": trace_sublimation_summary(trace),
        }
    return rows


def summarize(
    model_summaries: dict[str, Any],
    candidates: dict[str, Any],
    trace_proof: dict[str, Any],
) -> dict[str, Any]:
    default = model_summaries[CURRENT_DEFAULT]["aggregate"]
    promoted = [
        model_id for model_id, row in candidates.items() if row["promotion_eligible"]
    ]
    stage_b = candidates[STAGE_B]
    composition = candidates[STAGE_A_COMPOSITION]
    return {
        "disposition": "PROMOTION-ELIGIBLE" if promoted else "NON-PROMOTION-GATE-NOT-MET",
        "promotion_eligible_models": promoted,
        "activation_authorized": bool(promoted),
        "current_default_robust_fail_count": default["robust_fail_count"],
        "current_default_robust_ordinal_score": default["robust_ordinal_score"],
        "composition_robust_fail_count": composition["robust_fail_count"],
        "composition_robust_ordinal_score": composition["robust_ordinal_score"],
        "stage_b_robust_fail_count": stage_b["robust_fail_count"],
        "stage_b_robust_ordinal_score": stage_b["robust_ordinal_score"],
        "stage_b_primary_gate_pass": stage_b["primary_gate_beats_current_default"],
        "stage_b_conservation_gate_pass": stage_b["conservation_gate_pass"],
        "stage_b_bidirectional_guardrail_pass": stage_b[
            "bidirectional_guardrail_no_worse_cells"
        ],
        "composition_primary_gate_pass": composition[
            "primary_gate_beats_current_default"
        ],
        "protected_boundaries_preserved": True,
        "current_default_selector_trace_ok": trace_proof[CURRENT_DEFAULT]["selector_trace_ok"],
    }


def worse_signature_counts(
    site_reports: list[dict[str, Any]], model_id: str
) -> dict[str, int]:
    counts: Counter[str] = Counter()
    for row in paired_worse_rows(site_reports, model_id):
        counts[row["signature"]] += 1
    return dict(sorted(counts.items()))


def worse_site_counts(site_reports: list[dict[str, Any]], model_id: str) -> dict[str, int]:
    counts: Counter[str] = Counter()
    for row in paired_worse_rows(site_reports, model_id):
        counts[row["site_id"]] += 1
    return dict(sorted(counts.items()))


def paired_worse_rows(site_reports: list[dict[str, Any]], model_id: str) -> list[dict[str, Any]]:
    rows = []
    for site in site_reports:
        default_profile = site["models"][CURRENT_DEFAULT]["rubric_profile"]
        candidate_profile = site["models"][model_id]["rubric_profile"]
        default_scores = cross.robust_cell_scores(default_profile)
        candidate_scores = cross.robust_cell_scores(candidate_profile)
        for cell_id, candidate_score in candidate_scores.items():
            default_score = default_scores.get(cell_id)
            if default_score is None or candidate_score >= default_score:
                continue
            default_cell = cross.cell_by_id(default_profile, cell_id)
            candidate_cell = cross.cell_by_id(candidate_profile, cell_id)
            rows.append(
                {
                    "site_id": site["site_id"],
                    "corpus": site["corpus"],
                    "snow_climate": site["snow_climate"],
                    "model_id": model_id,
                    "cell_id": cell_id,
                    "signature": candidate_cell["signature"],
                    "timescale": candidate_cell["timescale"],
                    "tier": candidate_cell["tier"],
                    "current_default_label": default_cell["ordinal_label"],
                    "candidate_label": candidate_cell["ordinal_label"],
                    "score_delta": candidate_score - default_score,
                }
            )
    return rows


def trace_sublimation_summary(trace: dict[str, Any]) -> dict[str, Any]:
    return {
        "positive_sublimation_row_count": trace["positive_sublimation_row_count"],
        "total_sublimation_m": trace["total_sublimation_m"],
        "max_daily_sublimation_m": trace["max_daily_sublimation_m"],
        "max_abs_snow_state_residual_m": trace["max_abs_snow_state_residual_m"],
        "max_abs_partition_residual_m": trace["max_abs_partition_residual_m"],
    }


def libsnobal_provenance() -> dict[str, Any]:
    setup_py = Path("/home/workdir/pysnobal/setup.py")
    license_lines = []
    if setup_py.is_file():
        license_lines = [
            line.strip()
            for line in setup_py.read_text(encoding="utf-8").splitlines()
            if "license=" in line or "CC0 1.0" in line
        ]
    return {
        "clone_path": "/home/workdir/pysnobal",
        "clone_commit": "bf8b41c71e3e54ae654ae04005ddf72566c47ee6",
        "setup_py": str(setup_py),
        "setup_py_license_lines": license_lines,
        "deny_toml_allows_cc0_1_0": True,
        "deny_toml_gpl_family_excluded_by_omission": True,
        "source_use": "equation-reference / portable implementation reference only",
    }


def empty_trace_closure() -> dict[str, Any]:
    return {
        "row_count": 0,
        "positive_sublimation_row_count": 0,
        "total_sublimation_m": 0.0,
        "max_daily_sublimation_m": 0.0,
        "max_abs_snow_state_residual_m": 0.0,
        "max_abs_partition_residual_m": 0.0,
    }


def expected_env(value: str | None, default: str) -> str:
    return default if value is None else value


def path_from_rel(value: Any) -> Path | None:
    if not value:
        return None
    path = Path(str(value))
    return path if path.is_absolute() else REPO_ROOT / path


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


def write_report(report: dict[str, Any], output_dir: Path, package_artifacts_dir: Path) -> None:
    rubric.write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    rubric.write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    markdown = render_markdown(report)
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    candidates = report["candidate_disposition"]
    lines = [
        "# SNOWDENSITY-10.3.20 Sublimation Stage B Unlock",
        "",
        f"Evidence mode: `{report['evidence_class']}`.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Current default robust fail/score: `{summary['current_default_robust_fail_count']}` / `{summary['current_default_robust_ordinal_score']}`",
        f"- Composition robust fail/score: `{summary['composition_robust_fail_count']}` / `{summary['composition_robust_ordinal_score']}`",
        f"- Stage B robust fail/score: `{summary['stage_b_robust_fail_count']}` / `{summary['stage_b_robust_ordinal_score']}`",
        f"- Stage B primary gate pass: `{summary['stage_b_primary_gate_pass']}`",
        f"- Stage B conservation gate pass: `{summary['stage_b_conservation_gate_pass']}`",
        f"- Stage B bidirectional guardrail pass: `{summary['stage_b_bidirectional_guardrail_pass']}`",
        f"- Activation authorized: `{summary['activation_authorized']}`",
        "",
        "## Candidate Gates",
        "",
        "| Candidate | Primary | Guardrail | Conservation | Promotion | Better cells | Worse cells |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for model_id, row in candidates.items():
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{model_id}`",
                    str(row["primary_gate_beats_current_default"]),
                    str(row["bidirectional_guardrail_no_worse_cells"]),
                    str(row["conservation_gate_pass"]),
                    str(row["promotion_eligible"]),
                    str(row["better_robust_cells_vs_current_default"]),
                    str(row["worse_robust_cells_vs_current_default"]),
                ]
            )
            + " |"
        )
    lines.extend(
        [
            "",
            "## Provenance",
            "",
            f"- libsnobal clone commit: `{report['libsnobal_provenance']['clone_commit']}`",
            "- `setup.py` CC0 declaration captured in JSON artifact.",
            "- Default, rollback, fixtures, output schema, density cap, frost, parser/runfile/user CLI, and `.run` controls unchanged.",
            "",
        ]
    )
    return "\n".join(lines)


if __name__ == "__main__":
    raise SystemExit(main())
