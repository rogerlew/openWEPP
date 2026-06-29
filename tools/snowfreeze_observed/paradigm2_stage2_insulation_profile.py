#!/usr/bin/env python3
"""Run PARADIGM-2 Stage 2 snow-frost insulation-profile diagnostics."""

from __future__ import annotations

import argparse
from collections import Counter
from contextlib import contextmanager
import json
import os
import sys
import time
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import cross_snotel_mechanism_rubric as cross  # noqa: E402
import non_snotel_rubric_baseline as frost_rubric  # noqa: E402
import observed_harness  # noqa: E402


SCHEMA = "paradigm2-stage2-snow-frost-insulation-profile-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-079 INV-SNOWFREEZE-050 ADR-0029"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/paradigm2_stage2_insulation_profile"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_FROST_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
ARTIFACT_STEM = "paradigm2-stage2-gradient-entry-gate"
FROST_ARTIFACT_STEM = "paradigm2-stage2-frost-rubric"
MODEL_ID = "paradigm2_stage1_layered_density"
FROST_BULK_MODEL_ID = "stage1_layered_density_bulk_snow_frost_handoff"
FROST_CANDIDATE_MODEL_ID = "stage2_layered_resistance_v1"
INSULATION_ENV = "OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL"
GRADIENT_TOLERANCE_KG_M3 = 1.0e-9
MATERIAL_GRADIENT_KG_M3 = 10.0
FROST_PRIMARY_CELL_IDS = {
    "frost_measurement_correspondence",
    "frost_isotherm_upper_bound",
    "frost_onset_timing",
    "frost_thaw_timing",
    "frost_frozen_duration",
}
FROST_LIMITED_CELL_IDS = {"frost_max_depth_bias", "frost_depth_timeseries"}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--frost-observations-dir", type=Path, default=DEFAULT_FROST_OBSERVATIONS)
    parser.add_argument(
        "--mode",
        choices=["all", "gradient", "frost"],
        default="all",
        help="diagnostic phase to run; all writes both package artifacts",
    )
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    summaries = {}
    if args.mode in {"all", "gradient"}:
        report = diagnose(
            output_dir=args.output_dir.resolve(),
            package_artifacts_dir=args.package_artifacts_dir.resolve(),
            hill_binary=args.hill_binary.resolve(),
            run_models=not args.skip_model_runs,
        )
        summaries["gradient"] = report["summary"]
    if args.mode in {"all", "frost"}:
        frost_report = diagnose_frost(
            output_dir=args.output_dir.resolve(),
            package_artifacts_dir=args.package_artifacts_dir.resolve(),
            observations_dir=args.frost_observations_dir.resolve(),
            hill_binary=args.hill_binary.resolve(),
            run_models=not args.skip_model_runs,
        )
        summaries["frost"] = frost_report["summary"]
    print(json.dumps(summaries, indent=2, sort_keys=True))
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
    models = [model_spec()]
    started = time.perf_counter()
    site_reports = [
        cross.score_site(site, models, output_dir, hill_binary, None, run_models)
        for site in sites
    ]
    elapsed_seconds = time.perf_counter() - started
    gradient = build_gradient_proof(site_reports)
    gate = gradient_gate(gradient)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": (
            "Static + Ran"
            if run_models
            else "Static + Ran (completed outputs reused)"
        ),
        "default_changed": False,
        "diagnostic_only": True,
        "activation_authorized": False,
        "authority": {
            "stage": "Paradigm 2 Stage 2 entry gate",
            "gradient_rule": "multi-layer rows must demonstrate basal density greater than surface density before frost coupling is evaluated",
            "fixture_fitting_used": False,
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
        "summary": {
            "gate_passed": gate["passed"],
            "gate_reason": gate["reason"],
            "candidate_trace_row_count": gradient["candidate_trace_row_count"],
            "multi_layer_rows_after": gradient["multi_layer_rows_after"],
            "positive_gradient_rows_after": gradient["positive_gradient_rows_after"],
            "negative_gradient_rows_after": gradient["negative_gradient_rows_after"],
            "max_gradient_after_kg_m3": gradient["max_gradient_after_kg_m3"],
            "min_gradient_after_kg_m3": gradient["min_gradient_after_kg_m3"],
            "elapsed_seconds": elapsed_seconds,
        },
        "gates": {"gradient_entry_gate": gate},
        "gradient_proof": gradient,
        "sites": site_reports,
        "raw_outputs": {
            "output_dir": cross.rel(output_dir),
            "package_json": cross.rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": cross.rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
        },
    }
    write_report(report, output_dir, package_artifacts_dir)
    return report


def model_spec() -> cross.ModelSpec:
    return cross.ModelSpec(
        model_id=MODEL_ID,
        mechanism="Paradigm 2 Stage 1 n-layer density under local overburden",
        availability="current_direct_runtime",
        env={
            cross.MELT_ENV: None,
            cross.DENSITY_ENV: "physics_bulk_multilayer_density_v1",
            cross.PHASE_ENV: None,
        },
        source="OPENWEPP_SNOWDENSITY09_DENSITY_MODEL opt-in",
        note="Stage 2 entry-gate diagnostic; default unchanged",
        lever_rank_eligible=False,
    )


def diagnose_frost(
    output_dir: Path,
    package_artifacts_dir: Path,
    observations_dir: Path,
    hill_binary: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")
    observed_harness.validate_observations(observations_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((observations_dir / "manifest.json").read_text(encoding="utf-8"))
    started = time.perf_counter()
    by_model: dict[str, list[dict[str, Any]]] = {model["model_id"]: [] for model in frost_model_specs()}
    for model in frost_model_specs():
        for site in manifest["sites"]:
            site_id = site["site_id"]
            site_output = output_dir / "frost_site_reports" / model["model_id"] / site_id
            with scoped_env(model["env"]):
                observed_harness.compare_site(
                    site_id=site_id,
                    observations_dir=observations_dir,
                    output_dir=site_output,
                    binary=hill_binary,
                    no_run=not run_models,
                    runtime="direct-production-executor",
                )
            report = json.loads((site_output / "comparison_report.json").read_text(encoding="utf-8"))
            observations = frost_rubric.load_csv(observations_dir / site["observation_file"])
            modeled = observed_harness.load_modeled_wat(Path(report["wat_output"]))
            by_model[model["model_id"]].append(
                frost_rubric.build_site_profile(
                    site,
                    report,
                    observations,
                    modeled,
                    model["model_id"],
                )
            )
    elapsed_seconds = time.perf_counter() - started
    aggregate = {
        model_id: aggregate_frost_profiles(profiles)
        for model_id, profiles in by_model.items()
    }
    comparison = compare_frost_models(
        by_model[FROST_BULK_MODEL_ID],
        by_model[FROST_CANDIDATE_MODEL_ID],
    )
    gate = frost_gate(aggregate, comparison)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": (
            "Static + Ran"
            if run_models
            else "Static + Ran (completed outputs reused)"
        ),
        "default_changed": False,
        "diagnostic_only": True,
        "activation_authorized": False,
        "authority": {
            "stage": "Paradigm 2 Stage 2 frost-primary candidate gate",
            "primary_gate": "forcing-robust frost signatures under INV-SNOWFREEZE-050",
            "limited_frost_depth_cells_report_only": sorted(FROST_LIMITED_CELL_IDS),
            "fixture_fitting_used": False,
        },
        "protected_boundaries": {
            "production_default_changed": False,
            "density_cap_changed": False,
            "output_schema_changed": False,
            "fixture_inputs_changed": False,
            "frost_output_schema_changed": False,
            "per_layer_thermal_solve_added": False,
            "parser_runfile_user_cli_selector_added": False,
            "site_calibration_performed": False,
        },
        "inputs": {
            "hill_binary": cross.rel(hill_binary),
            "observations_dir": cross.rel(observations_dir),
            "site_count": len(manifest["sites"]),
            "models": frost_model_specs_for_record(),
        },
        "summary": {
            "gate_passed": gate["passed"],
            "gate_reason": gate["reason"],
            "bulk_robust_fail_count": aggregate[FROST_BULK_MODEL_ID]["primary_robust_fail_count"],
            "candidate_robust_fail_count": aggregate[FROST_CANDIDATE_MODEL_ID][
                "primary_robust_fail_count"
            ],
            "bulk_robust_score": aggregate[FROST_BULK_MODEL_ID]["primary_robust_ordinal_score"],
            "candidate_robust_score": aggregate[FROST_CANDIDATE_MODEL_ID][
                "primary_robust_ordinal_score"
            ],
            "primary_improved_cell_count": comparison["primary_improved_cell_count"],
            "primary_worsened_cell_count": comparison["primary_worsened_cell_count"],
            "elapsed_seconds": elapsed_seconds,
        },
        "gates": {"frost_primary_gate": gate},
        "aggregate": aggregate,
        "comparison": comparison,
        "sites_by_model": by_model,
        "raw_outputs": {
            "output_dir": cross.rel(output_dir),
            "package_json": cross.rel(package_artifacts_dir / f"{FROST_ARTIFACT_STEM}.json"),
            "package_markdown": cross.rel(package_artifacts_dir / f"{FROST_ARTIFACT_STEM}.md"),
        },
    }
    write_frost_report(report, output_dir, package_artifacts_dir)
    return report


def frost_model_specs() -> list[dict[str, Any]]:
    common = {
        cross.DENSITY_ENV: "physics_bulk_multilayer_density_v1",
        cross.MELT_ENV: None,
        cross.PHASE_ENV: None,
    }
    return [
        {
            "model_id": FROST_BULK_MODEL_ID,
            "mechanism": "Stage 1 layered snow density with existing bulk snow-frost handoff",
            "env": {**common, INSULATION_ENV: "bulk_depth_density"},
            "source": "rollback/current handoff",
        },
        {
            "model_id": FROST_CANDIDATE_MODEL_ID,
            "mechanism": "Stage 2 layered resistance snow-frost handoff",
            "env": {**common, INSULATION_ENV: "layered_resistance_v1"},
            "source": f"{INSULATION_ENV}=layered_resistance_v1",
        },
    ]


def frost_model_specs_for_record() -> list[dict[str, Any]]:
    return [
        {
            **model,
            "env": {key: value for key, value in model["env"].items() if value is not None},
        }
        for model in frost_model_specs()
    ]


@contextmanager
def scoped_env(values: dict[str, str | None]) -> Any:
    previous = {key: os.environ.get(key) for key in values}
    try:
        for key, value in values.items():
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


def aggregate_frost_profiles(site_profiles: list[dict[str, Any]]) -> dict[str, Any]:
    counts: Counter[str] = Counter()
    primary_counts: Counter[str] = Counter()
    limited_counts: Counter[str] = Counter()
    primary_score = 0
    primary_available = 0
    primary_fail = 0
    for site in site_profiles:
        for cell in site["rubric_profile"]["cells"]:
            label = cell["ordinal_label"]
            counts[label] += 1
            if cell["cell_id"] in FROST_PRIMARY_CELL_IDS and label in cross.LABEL_SCORE:
                primary_available += 1
                primary_counts[label] += 1
                primary_score += cross.LABEL_SCORE[label]
                primary_fail += int(label == "fail")
            elif cell["cell_id"] in FROST_LIMITED_CELL_IDS:
                limited_counts[label] += 1
    return {
        "counts_by_label": dict(sorted(counts.items())),
        "primary_cell_ids": sorted(FROST_PRIMARY_CELL_IDS),
        "limited_report_only_cell_ids": sorted(FROST_LIMITED_CELL_IDS),
        "primary_robust_counts_by_label": dict(sorted(primary_counts.items())),
        "limited_report_only_counts_by_label": dict(sorted(limited_counts.items())),
        "primary_robust_available_cell_count": primary_available,
        "primary_robust_fail_count": primary_fail,
        "primary_robust_ordinal_score": primary_score,
        "site_count": len(site_profiles),
    }


def compare_frost_models(
    bulk_sites: list[dict[str, Any]],
    candidate_sites: list[dict[str, Any]],
) -> dict[str, Any]:
    candidate_by_site = {site["site_id"]: site for site in candidate_sites}
    primary_rows = []
    limited_rows = []
    for bulk_site in bulk_sites:
        candidate_site = candidate_by_site[bulk_site["site_id"]]
        bulk_cells = cells_by_id(bulk_site)
        candidate_cells = cells_by_id(candidate_site)
        for cell_id in sorted(FROST_PRIMARY_CELL_IDS | FROST_LIMITED_CELL_IDS):
            bulk_cell = bulk_cells.get(cell_id)
            candidate_cell = candidate_cells.get(cell_id)
            if bulk_cell is None or candidate_cell is None:
                continue
            row = frost_delta_row(bulk_site["site_id"], bulk_cell, candidate_cell)
            if cell_id in FROST_PRIMARY_CELL_IDS:
                primary_rows.append(row)
            else:
                limited_rows.append(row)
    return {
        "primary_rows": primary_rows,
        "limited_report_only_rows": limited_rows,
        "primary_improved_cell_count": sum(1 for row in primary_rows if row["score_delta"] > 0),
        "primary_worsened_cell_count": sum(1 for row in primary_rows if row["score_delta"] < 0),
        "primary_unchanged_cell_count": sum(1 for row in primary_rows if row["score_delta"] == 0),
        "limited_improved_cell_count": sum(1 for row in limited_rows if row["score_delta"] > 0),
        "limited_worsened_cell_count": sum(1 for row in limited_rows if row["score_delta"] < 0),
    }


def cells_by_id(site: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {cell["cell_id"]: cell for cell in site["rubric_profile"]["cells"]}


def frost_delta_row(
    site_id: str,
    bulk_cell: dict[str, Any],
    candidate_cell: dict[str, Any],
) -> dict[str, Any]:
    bulk_label = bulk_cell["ordinal_label"]
    candidate_label = candidate_cell["ordinal_label"]
    return {
        "site_id": site_id,
        "cell_id": bulk_cell["cell_id"],
        "tier": bulk_cell["tier"],
        "signature": bulk_cell["signature"],
        "bulk_label": bulk_label,
        "candidate_label": candidate_label,
        "bulk_score": cross.LABEL_SCORE.get(bulk_label),
        "candidate_score": cross.LABEL_SCORE.get(candidate_label),
        "score_delta": (
            cross.LABEL_SCORE[candidate_label] - cross.LABEL_SCORE[bulk_label]
            if bulk_label in cross.LABEL_SCORE and candidate_label in cross.LABEL_SCORE
            else 0
        ),
        "bulk_metrics": bulk_cell.get("metrics", {}),
        "candidate_metrics": candidate_cell.get("metrics", {}),
    }


def frost_gate(aggregate: dict[str, Any], comparison: dict[str, Any]) -> dict[str, Any]:
    bulk = aggregate[FROST_BULK_MODEL_ID]
    candidate = aggregate[FROST_CANDIDATE_MODEL_ID]
    fail_delta = candidate["primary_robust_fail_count"] - bulk["primary_robust_fail_count"]
    score_delta = (
        candidate["primary_robust_ordinal_score"] - bulk["primary_robust_ordinal_score"]
    )
    passed = (
        candidate["primary_robust_available_cell_count"] > 0
        and comparison["primary_worsened_cell_count"] == 0
        and (fail_delta < 0 or (fail_delta == 0 and score_delta > 0))
    )
    reason = (
        "available={available}; fail_delta={fail_delta}; score_delta={score_delta}; "
        "improved_cells={improved}; worsened_cells={worsened}"
    ).format(
        available=candidate["primary_robust_available_cell_count"],
        fail_delta=fail_delta,
        score_delta=score_delta,
        improved=comparison["primary_improved_cell_count"],
        worsened=comparison["primary_worsened_cell_count"],
    )
    return {"passed": passed, "reason": reason}


def build_gradient_proof(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    total_rows = 0
    multi_layer_rows = 0
    positive_rows = 0
    material_positive_rows = 0
    negative_rows = 0
    zero_rows = 0
    max_gradient = 0.0
    min_gradient = 0.0
    by_site = {}
    for site in site_reports:
        model = site["models"][MODEL_ID]
        trace_path = path_from_rel(model.get("trace"))
        proof = trace_gradient_proof(trace_path)
        by_site[site["site_id"]] = {
            "corpus": site["corpus"],
            "trace": model.get("trace"),
            **proof,
        }
        total_rows += proof["row_count"]
        multi_layer_rows += proof["multi_layer_rows_after"]
        positive_rows += proof["positive_gradient_rows_after"]
        material_positive_rows += proof["material_positive_gradient_rows_after"]
        negative_rows += proof["negative_gradient_rows_after"]
        zero_rows += proof["zero_gradient_rows_after"]
        max_gradient = max(max_gradient, proof["max_gradient_after_kg_m3"])
        min_gradient = min(min_gradient, proof["min_gradient_after_kg_m3"])
    return {
        "candidate_trace_row_count": total_rows,
        "multi_layer_rows_after": multi_layer_rows,
        "positive_gradient_rows_after": positive_rows,
        "material_positive_gradient_rows_after": material_positive_rows,
        "negative_gradient_rows_after": negative_rows,
        "zero_gradient_rows_after": zero_rows,
        "max_gradient_after_kg_m3": max_gradient,
        "min_gradient_after_kg_m3": min_gradient,
        "gradient_tolerance_kg_m3": GRADIENT_TOLERANCE_KG_M3,
        "material_gradient_kg_m3": MATERIAL_GRADIENT_KG_M3,
        "by_site": by_site,
    }


def trace_gradient_proof(path: Path | None) -> dict[str, Any]:
    if path is None or not path.is_file():
        return empty_gradient_proof()
    proof = empty_gradient_proof()
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            proof["row_count"] += 1
            layer_count = int(number(row, "snow_layer_count_after"))
            if layer_count < 2:
                continue
            proof["multi_layer_rows_after"] += 1
            gradient = number(row, "snow_layer_density_gradient_after_kg_m3")
            proof["max_gradient_after_kg_m3"] = max(
                proof["max_gradient_after_kg_m3"],
                gradient,
            )
            proof["min_gradient_after_kg_m3"] = min(
                proof["min_gradient_after_kg_m3"],
                gradient,
            )
            if gradient > GRADIENT_TOLERANCE_KG_M3:
                proof["positive_gradient_rows_after"] += 1
                if gradient >= MATERIAL_GRADIENT_KG_M3:
                    proof["material_positive_gradient_rows_after"] += 1
            elif gradient < -GRADIENT_TOLERANCE_KG_M3:
                proof["negative_gradient_rows_after"] += 1
            else:
                proof["zero_gradient_rows_after"] += 1
    return proof


def empty_gradient_proof() -> dict[str, Any]:
    return {
        "row_count": 0,
        "multi_layer_rows_after": 0,
        "positive_gradient_rows_after": 0,
        "material_positive_gradient_rows_after": 0,
        "negative_gradient_rows_after": 0,
        "zero_gradient_rows_after": 0,
        "max_gradient_after_kg_m3": 0.0,
        "min_gradient_after_kg_m3": 0.0,
    }


def gradient_gate(gradient: dict[str, Any]) -> dict[str, Any]:
    passed = (
        gradient["candidate_trace_row_count"] > 0
        and gradient["multi_layer_rows_after"] > 0
        and gradient["positive_gradient_rows_after"] > 0
        and gradient["material_positive_gradient_rows_after"] > 0
        and gradient["max_gradient_after_kg_m3"] >= MATERIAL_GRADIENT_KG_M3
    )
    reason = (
        "rows={rows}; multi_layer={multi}; positive={positive}; "
        "material_positive={material}; negative={negative}; max={max_gradient}; "
        "min={min_gradient}; material_threshold={threshold}"
    ).format(
        rows=gradient["candidate_trace_row_count"],
        multi=gradient["multi_layer_rows_after"],
        positive=gradient["positive_gradient_rows_after"],
        material=gradient["material_positive_gradient_rows_after"],
        negative=gradient["negative_gradient_rows_after"],
        max_gradient=gradient["max_gradient_after_kg_m3"],
        min_gradient=gradient["min_gradient_after_kg_m3"],
        threshold=MATERIAL_GRADIENT_KG_M3,
    )
    return {"passed": passed, "reason": reason}


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


def write_frost_report(
    report: dict[str, Any],
    output_dir: Path,
    package_artifacts_dir: Path,
) -> None:
    output_json = output_dir / f"{FROST_ARTIFACT_STEM}.json"
    output_md = output_dir / f"{FROST_ARTIFACT_STEM}.md"
    package_json = package_artifacts_dir / f"{FROST_ARTIFACT_STEM}.json"
    package_md = package_artifacts_dir / f"{FROST_ARTIFACT_STEM}.md"
    text = json.dumps(report, indent=2, sort_keys=True)
    output_json.write_text(text + "\n", encoding="utf-8")
    package_json.write_text(text + "\n", encoding="utf-8")
    markdown = render_frost_markdown(report)
    output_md.write_text(markdown, encoding="utf-8")
    package_md.write_text(markdown, encoding="utf-8")


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    rows = [
        "# PARADIGM-2 Stage 2 Gradient Entry Gate",
        "",
        f"Schema: `{report['schema']}`",
        f"Contract: `{report['contract']}`",
        f"Evidence: `{report['evidence_class']}`",
        "",
        "## Summary",
        "",
        f"- Gate passed: `{summary['gate_passed']}`",
        f"- Reason: {summary['gate_reason']}",
        f"- Candidate trace rows: `{summary['candidate_trace_row_count']}`",
        f"- Multi-layer rows after snow step: `{summary['multi_layer_rows_after']}`",
        f"- Positive gradient rows: `{summary['positive_gradient_rows_after']}`",
        f"- Negative gradient rows: `{summary['negative_gradient_rows_after']}`",
        f"- Max gradient: `{summary['max_gradient_after_kg_m3']} kg m^-3`",
        f"- Min gradient: `{summary['min_gradient_after_kg_m3']} kg m^-3`",
        "",
        "## Site Matrix",
        "",
        "| Site | Corpus | Multi-layer rows | Positive | Material positive | Negative | Max gradient | Min gradient |",
        "|---|---|---:|---:|---:|---:|---:|---:|",
    ]
    for site_id, site in report["gradient_proof"]["by_site"].items():
        rows.append(
            "| {site_id} | {corpus} | {multi} | {positive} | {material} | {negative} | {max_gradient} | {min_gradient} |".format(
                site_id=site_id,
                corpus=site["corpus"],
                multi=site["multi_layer_rows_after"],
                positive=site["positive_gradient_rows_after"],
                material=site["material_positive_gradient_rows_after"],
                negative=site["negative_gradient_rows_after"],
                max_gradient=site["max_gradient_after_kg_m3"],
                min_gradient=site["min_gradient_after_kg_m3"],
            )
        )
    rows.append("")
    return "\n".join(rows)


def render_frost_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    rows = [
        "# PARADIGM-2 Stage 2 Frost-Primary Rubric",
        "",
        f"Schema: `{report['schema']}`",
        f"Contract: `{report['contract']}`",
        f"Evidence: `{report['evidence_class']}`",
        "",
        "## Summary",
        "",
        f"- Gate passed: `{summary['gate_passed']}`",
        f"- Reason: {summary['gate_reason']}",
        f"- Bulk robust fails/score: `{summary['bulk_robust_fail_count']}` / `{summary['bulk_robust_score']}`",
        f"- Candidate robust fails/score: `{summary['candidate_robust_fail_count']}` / `{summary['candidate_robust_score']}`",
        f"- Primary improved/worsened cells: `{summary['primary_improved_cell_count']}` / `{summary['primary_worsened_cell_count']}`",
        "",
        "## Aggregate",
        "",
        "| Model | Primary robust counts | Primary score | Limited report-only counts |",
        "|---|---|---:|---|",
    ]
    for model_id, aggregate in report["aggregate"].items():
        rows.append(
            "| {model} | {primary} | {score} | {limited} |".format(
                model=model_id,
                primary=aggregate["primary_robust_counts_by_label"],
                score=aggregate["primary_robust_ordinal_score"],
                limited=aggregate["limited_report_only_counts_by_label"],
            )
        )
    rows.extend(
        [
            "",
            "## Primary Cell Deltas",
            "",
            "| Site | Cell | Bulk | Candidate | Delta |",
            "|---|---|---:|---:|---:|",
        ]
    )
    for row in report["comparison"]["primary_rows"]:
        rows.append(
            "| {site} | {cell} | {bulk} | {candidate} | {delta} |".format(
                site=row["site_id"],
                cell=row["cell_id"],
                bulk=row["bulk_label"],
                candidate=row["candidate_label"],
                delta=row["score_delta"],
            )
        )
    rows.extend(
        [
            "",
            "## Limited Frost-Depth Cells",
            "",
            "| Site | Cell | Bulk | Candidate | Delta |",
            "|---|---|---:|---:|---:|",
        ]
    )
    for row in report["comparison"]["limited_report_only_rows"]:
        rows.append(
            "| {site} | {cell} | {bulk} | {candidate} | {delta} |".format(
                site=row["site_id"],
                cell=row["cell_id"],
                bulk=row["bulk_label"],
                candidate=row["candidate_label"],
                delta=row["score_delta"],
            )
        )
    rows.append("")
    return "\n".join(rows)


def path_from_rel(value: Any) -> Path | None:
    if not isinstance(value, str) or not value:
        return None
    path = Path(value)
    if path.is_absolute():
        return path
    return REPO_ROOT / path


def number(row: dict[str, Any], key: str) -> float:
    value = row.get(key, 0.0)
    if value is None:
        return 0.0
    return float(value)


if __name__ == "__main__":
    raise SystemExit(main())
