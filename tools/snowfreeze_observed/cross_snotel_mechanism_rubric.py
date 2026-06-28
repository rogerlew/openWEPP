#!/usr/bin/env python3
"""Run the SNOWDENSITY-10.3.18 cross-SNOTEL mechanism rubric diagnostic.

This diagnostic scores current executable snow mechanisms against the
``INV-SNOWFREEZE-050`` SWE/depth/density rubric across the five SNOTEL fixtures
and the bound ``cancov_forest`` SWE/depth/density strata. Legacy/PySnobal
profiles remain ADR-0017 flag profiles, never targets.
"""

from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import json
import os
import subprocess
import sys
from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterator


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import observed_harness  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowdensity10-3-18-cross-snotel-mechanism-rubric-v1"
CONTRACT = (
    "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 REF-SNOWFREEZE-FROST-OBS ADR-0017"
)
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_18_cross_snotel_mechanism_rubric"
DEFAULT_HILL_BINARY = REPO_ROOT / "target/debug/openwepp-cli-hill"
DEFAULT_H_REPORT = REPO_ROOT / "target/snowfrost_fidelity_h/three_way_comparison.json"
MELT_ENV = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL"
DENSITY_ENV = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL"
PHASE_ENV = "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL"
TRACE_ENV = "OPENWEPP_R7H_SNOW_TRACE_PATH"
LEGACY_MELT = "legacy_coe"
LEGACY_DENSITY = "legacy_wepp"
ACTIVATED_MELT = "coe_liquid_holding_capacity_v1"
ACTIVATED_DENSITY = "physics_bulk_density_compaction_v1"
LABEL_SCORE = {"fail": 0, "marginal": 1, "pass": 2, "strong": 3}
COMPONENTS = ("swe", "depth", "density")
ARTIFACT_STEM = "cross-snotel-mechanism-rubric"


@dataclass(frozen=True)
class DiagnosticSite:
    site_id: str
    corpus: str
    fixture_dir: Path
    observation_file: Path
    snow_climate: str
    observation_filter: dict[str, str]
    flag_profile_source: str


@dataclass(frozen=True)
class ModelSpec:
    model_id: str
    mechanism: str
    availability: str
    env: dict[str, str | None]
    source: str
    note: str
    lever_rank_eligible: bool


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--hill-binary", type=Path, default=DEFAULT_HILL_BINARY)
    parser.add_argument("--h-report", type=Path, default=DEFAULT_H_REPORT)
    parser.add_argument("--skip-model-runs", action="store_true")
    args = parser.parse_args(argv)

    report = diagnose(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        hill_binary=args.hill_binary.resolve(),
        h_report_path=args.h_report.resolve(),
        run_models=not args.skip_model_runs,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def diagnose(
    output_dir: Path,
    package_artifacts_dir: Path,
    hill_binary: Path,
    h_report_path: Path,
    run_models: bool,
) -> dict[str, Any]:
    if run_models and not hill_binary.is_file():
        raise FileNotFoundError(f"openwepp-cli-hill binary not found: {hill_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    sites = diagnostic_sites()
    models = model_specs()
    h_report = read_json(h_report_path) if h_report_path.is_file() else None
    site_reports = [
        score_site(site, models, output_dir, hill_binary, h_report, run_models)
        for site in sites
    ]
    model_summaries = summarize_models(site_reports, models)
    improvement_matrix = compare_to_activated(site_reports, models)
    lever_rank = rank_levers(model_summaries, improvement_matrix, models)
    humid_ne = humid_new_england_read(site_reports)
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran" if run_models else "Static + Reused",
        "diagnostic_only": True,
        "no_promotion_or_activation_decision": True,
        "authority": {
            "rubric": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050",
            "forcing_robust_verdict_rule": "R cells carry diagnostic verdict weight; L cells are reported only",
            "legacy_and_pysnobal_are_flags_not_targets": True,
            "new_contract_authority_needed": False,
            "site_calibration_or_fixture_fitting": False,
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
            "hill_binary": rel(hill_binary),
            "h_snotel_flag_report": rel(h_report_path) if h_report_path.is_file() else None,
            "site_count": len(sites),
            "models": [model_record(model) for model in models],
        },
        "summary": summarize_report(model_summaries, improvement_matrix, lever_rank, humid_ne),
        "model_summaries": model_summaries,
        "mechanism_improvements": improvement_matrix,
        "humid_new_england_representativeness": humid_ne,
        "ranked_next_global_lever": lever_rank,
        "sites": site_reports,
        "matrix": flatten_matrix(site_reports),
        "raw_outputs": {
            "output_dir": rel(output_dir),
            "package_json": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.json"),
            "package_markdown": rel(package_artifacts_dir / f"{ARTIFACT_STEM}.md"),
        },
    }
    write_report(report, output_dir, package_artifacts_dir)
    return report


def diagnostic_sites() -> list[DiagnosticSite]:
    snotel_sites = [
        DiagnosticSite(
            site_id=site.site_id,
            corpus="snotel_observed",
            fixture_dir=REPO_ROOT / "tests/fixtures/snotel_observed" / site.site_id,
            observation_file=(
                REPO_ROOT
                / "tests/fixtures/snotel_observed/observations/sites"
                / f"{site.site_id}.csv"
            ),
            snow_climate=site.snow_climate,
            observation_filter={},
            flag_profile_source="snowfrost_fidelity_h",
        )
        for site in rubric.SITES
    ]
    cancov_root = REPO_ROOT / "tests/fixtures/cancov_forest"
    cancov = [
        DiagnosticSite(
            site_id="harvard_open",
            corpus="cancov_forest",
            fixture_dir=cancov_root / "harvard_open_ma",
            observation_file=cancov_root / "observations/sites/harvard_hf237_strata.csv",
            snow_climate="humid_new_england_open",
            observation_filter={
                "binding_status": "bound",
                "model_fixture": "harvard_open_ma",
                "observed_stratum": "open",
            },
            flag_profile_source="not_available",
        ),
        DiagnosticSite(
            site_id="harvard_hardwood",
            corpus="cancov_forest",
            fixture_dir=cancov_root / "harvard_deciduous_ma",
            observation_file=cancov_root / "observations/sites/harvard_hf237_strata.csv",
            snow_climate="humid_new_england_hardwood",
            observation_filter={
                "binding_status": "bound",
                "model_fixture": "harvard_deciduous_ma",
                "observed_stratum": "hardwood",
            },
            flag_profile_source="not_available",
        ),
        DiagnosticSite(
            site_id="marcell_conifer",
            corpus="cancov_forest",
            fixture_dir=cancov_root / "marcell_conifer_mn",
            observation_file=cancov_root
            / "observations/sites/marcell_rds_2021_0016_stratum_means.csv",
            snow_climate="laurentian_continental_conifer",
            observation_filter={
                "binding_status": "bound",
                "model_fixture": "marcell_conifer_mn",
                "observed_stratum": "conifer",
            },
            flag_profile_source="not_available",
        ),
        DiagnosticSite(
            site_id="marcell_deciduous",
            corpus="cancov_forest",
            fixture_dir=cancov_root / "marcell_deciduous_mn",
            observation_file=cancov_root
            / "observations/sites/marcell_rds_2021_0016_stratum_means.csv",
            snow_climate="laurentian_continental_deciduous",
            observation_filter={
                "binding_status": "bound",
                "model_fixture": "marcell_deciduous_mn",
                "observed_stratum": "deciduous",
            },
            flag_profile_source="not_available",
        ),
        DiagnosticSite(
            site_id="marcell_open",
            corpus="cancov_forest",
            fixture_dir=cancov_root / "marcell_open_mn",
            observation_file=cancov_root
            / "observations/sites/marcell_rds_2021_0016_stratum_means.csv",
            snow_climate="laurentian_continental_open",
            observation_filter={
                "binding_status": "bound",
                "model_fixture": "marcell_open_mn",
                "observed_stratum": "open",
            },
            flag_profile_source="not_available",
        ),
    ]
    return snotel_sites + cancov


def model_specs() -> list[ModelSpec]:
    return [
        ModelSpec(
            model_id="legacy_baseline",
            mechanism="legacy_coe + legacy_wepp rollback",
            availability="current_direct_runtime",
            env={MELT_ENV: LEGACY_MELT, DENSITY_ENV: LEGACY_DENSITY},
            source="openwepp-cli-hill direct-production executor",
            note="ADR-0017 flag profile, not a target",
            lever_rank_eligible=False,
        ),
        ModelSpec(
            model_id="activated_bundle",
            mechanism="coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1",
            availability="current_direct_runtime",
            env={MELT_ENV: None, DENSITY_ENV: None},
            source="openwepp-cli-hill direct-production executor",
            note="current no-env activated default under active cap",
            lever_rank_eligible=False,
        ),
        ModelSpec(
            model_id="harder_pomeroy_partition",
            mechanism="Harder-Pomeroy hourly phase partition",
            availability="current_direct_runtime",
            env={PHASE_ENV: "harder_pomeroy_hourly", MELT_ENV: None, DENSITY_ENV: None},
            source="OPENWEPP_SNOWDENSITY1035_PHASE_MODEL opt-in",
            note="phase-only opt-in scored against the activated bundle context",
            lever_rank_eligible=True,
        ),
        ModelSpec(
            model_id="open_sublimation_stage_a_10_3_16",
            mechanism="open-surface sublimation Stage A",
            availability="current_direct_runtime",
            env={MELT_ENV: "coe_open_sublimation_stage_a_v1", DENSITY_ENV: None},
            source="OPENWEPP_SNOWDENSITY1038_MELT_MODEL opt-in",
            note="10.3.16 non-promoted opt-in profile",
            lever_rank_eligible=True,
        ),
        ModelSpec(
            model_id="shallow_pack_guard_10_3_17",
            mechanism="shallow-pack compaction guard",
            availability="current_direct_runtime",
            env={MELT_ENV: None, DENSITY_ENV: "physics_bulk_shallow_guard_v1"},
            source="OPENWEPP_SNOWDENSITY09_DENSITY_MODEL opt-in",
            note="10.3.17 non-promoted opt-in profile folded into the model list",
            lever_rank_eligible=True,
        ),
        ModelSpec(
            model_id="spring_densification_10_3_11",
            mechanism="spring densification",
            availability="archival_not_current_selector",
            env={},
            source="20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001",
            note="rejected candidate; current selector path no longer accepts physics_bulk_spring_densification_v1",
            lever_rank_eligible=False,
        ),
        ModelSpec(
            model_id="winter_thaw_state_loss_10_3_7",
            mechanism="winter-thaw positive state-loss melt response",
            availability="archival_snowbench_only",
            env={},
            source="20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001",
            note="diagnostic snowbench/coupled-gate lineage; not callable through current activated selector",
            lever_rank_eligible=False,
        ),
        ModelSpec(
            model_id="pysnobal_reference",
            mechanism="PySnobal process reference",
            availability="snotel_h_flag_profile",
            env={},
            source="SNOWFROST-FIDELITY-H SNOTEL ground-temperature lane",
            note="ADR-0017 process-reference flag, never a target",
            lever_rank_eligible=False,
        ),
    ]


def score_site(
    site: DiagnosticSite,
    models: list[ModelSpec],
    output_dir: Path,
    hill_binary: Path,
    h_report: dict[str, Any] | None,
    run_models: bool,
) -> dict[str, Any]:
    observations = load_filtered_observations(site)
    site_models = {}
    for model in models:
        if model.availability == "current_direct_runtime":
            site_models[model.model_id] = score_direct_model(
                site, observations, model, output_dir, hill_binary, run_models
            )
        elif model.model_id == "pysnobal_reference":
            site_models[model.model_id] = score_pysnobal_flag(site, observations, h_report)
        else:
            site_models[model.model_id] = score_unavailable_model(site, observations, model)
    return {
        "site_id": site.site_id,
        "corpus": site.corpus,
        "snow_climate": site.snow_climate,
        "fixture_dir": rel(site.fixture_dir),
        "observation_file": rel(site.observation_file),
        "observation_filter": site.observation_filter,
        "observation_row_count": len(observations),
        "models": site_models,
    }


def score_direct_model(
    site: DiagnosticSite,
    observations: list[dict[str, str]],
    model: ModelSpec,
    output_dir: Path,
    hill_binary: Path,
    run_models: bool,
) -> dict[str, Any]:
    run_dir = output_dir / "runs" / site.corpus / site.site_id / model.model_id
    run_dir.mkdir(parents=True, exist_ok=True)
    trace_path = output_dir / "traces" / site.corpus / f"{site.site_id}_{model.model_id}.jsonl"
    trace_path.parent.mkdir(parents=True, exist_ok=True)
    run_stem = observed_harness.discover_run_stem(site.fixture_dir)
    run_id = f"{site.site_id}_{model.model_id}"
    runfile_path = run_dir / f"{run_id}.run"
    observed_harness.write_runfile(runfile_path, site.fixture_dir, run_stem, run_dir, run_id)
    command = observed_harness.cli_command(
        hill_binary,
        site.fixture_dir,
        runfile_path,
        run_dir,
        "direct-production-executor",
    )
    env = dict(model.env)
    env[TRACE_ENV] = str(trace_path)
    if trace_path.exists() and run_models:
        trace_path.unlink()
    if run_models:
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
            return direct_failure_result(model, run_dir, runfile_path, trace_path, completed)
    wat_path = run_dir / f"{run_id}.wat.parquet"
    if not wat_path.is_file():
        return unavailable_result(
            observations,
            model,
            f"expected WAT output missing: {wat_path}",
            {
                "run_dir": rel(run_dir),
                "runfile": rel(runfile_path),
                "trace": rel(trace_path),
                "command": [str(value) for value in command],
            },
        )
    modeled = observed_harness.load_modeled_wat(wat_path)
    metrics = rubric.model_metrics(observations, modeled, model.model_id)
    profile = rubric.rubric_profile(observations, modeled, model.model_id)
    return {
        "model_id": model.model_id,
        "mechanism": model.mechanism,
        "availability": model.availability,
        "source": model.source,
        "note": model.note,
        "env": printable_env(env),
        "run_dir": rel(run_dir),
        "runfile": rel(runfile_path),
        "wat": rel(wat_path),
        "trace": rel(trace_path),
        "metrics": metrics,
        "rubric_profile": profile,
        "residual_decomposition": residual_decomposition(metrics),
    }


def score_pysnobal_flag(
    site: DiagnosticSite,
    observations: list[dict[str, str]],
    h_report: dict[str, Any] | None,
) -> dict[str, Any]:
    spec = next(model for model in model_specs() if model.model_id == "pysnobal_reference")
    if site.corpus != "snotel_observed":
        return unavailable_result(
            observations,
            spec,
            "PySnobal bridge/profile is installed only for the five SNOTEL fixtures in H",
            {},
        )
    if h_report is None:
        return unavailable_result(observations, spec, "H SNOTEL flag profile JSON missing", {})
    for h_site in h_report.get("sites", []):
        if h_site.get("site_id") != site.site_id:
            continue
        model = h_site.get("models", {}).get("pysnobal")
        if model is None:
            break
        return {
            "model_id": spec.model_id,
            "mechanism": spec.mechanism,
            "availability": spec.availability,
            "source": spec.source,
            "note": spec.note,
            "env": {},
            "series_source": model.get("series_source"),
            "metrics": model["metrics"],
            "rubric_profile": reidentify_profile(model["rubric_profile"], spec.model_id),
            "residual_decomposition": residual_decomposition(model["metrics"]),
        }
    return unavailable_result(observations, spec, "PySnobal profile absent for site", {})


def score_unavailable_model(
    site: DiagnosticSite,
    observations: list[dict[str, str]],
    model: ModelSpec,
) -> dict[str, Any]:
    return unavailable_result(observations, model, model.note, {"site": site.site_id})


def direct_failure_result(
    model: ModelSpec,
    run_dir: Path,
    runfile_path: Path,
    trace_path: Path,
    completed: subprocess.CompletedProcess[str],
) -> dict[str, Any]:
    observations: list[dict[str, str]] = []
    profile = rubric.rubric_profile(observations, {}, model.model_id)
    return {
        "model_id": model.model_id,
        "mechanism": model.mechanism,
        "availability": "run_failed",
        "source": model.source,
        "note": model.note,
        "env": printable_env(model.env),
        "run_dir": rel(run_dir),
        "runfile": rel(runfile_path),
        "trace": rel(trace_path),
        "stdout": rel(run_dir / "openwepp-cli-hill.stdout"),
        "stderr": rel(run_dir / "openwepp-cli-hill.stderr"),
        "returncode": completed.returncode,
        "metrics": empty_metrics(model.model_id),
        "rubric_profile": profile,
        "residual_decomposition": {},
    }


def unavailable_result(
    observations: list[dict[str, str]],
    model: ModelSpec,
    reason: str,
    artifacts: dict[str, Any],
) -> dict[str, Any]:
    profile = rubric.rubric_profile(observations, {}, model.model_id)
    return {
        "model_id": model.model_id,
        "mechanism": model.mechanism,
        "availability": model.availability,
        "source": model.source,
        "note": model.note,
        "unavailable_reason": reason,
        "env": printable_env(model.env),
        "artifacts": artifacts,
        "metrics": empty_metrics(model.model_id),
        "rubric_profile": profile,
        "residual_decomposition": {},
    }


def load_filtered_observations(site: DiagnosticSite) -> list[dict[str, str]]:
    rows = rubric.read_csv_dicts(site.observation_file)
    if not site.observation_filter:
        return rows
    return [
        row
        for row in rows
        if all(row.get(key) == value for key, value in site.observation_filter.items())
    ]


def reidentify_profile(profile: dict[str, Any], model_id: str) -> dict[str, Any]:
    updated = dict(profile)
    updated["model_id"] = model_id
    return updated


def summarize_models(
    site_reports: list[dict[str, Any]], models: list[ModelSpec]
) -> dict[str, Any]:
    summaries = {}
    for model in models:
        site_profiles = []
        residual_components = []
        for site in site_reports:
            model_report = site["models"][model.model_id]
            profile = model_report["rubric_profile"]
            site_profiles.append(
                {
                    "site_id": site["site_id"],
                    "corpus": site["corpus"],
                    "snow_climate": site["snow_climate"],
                    "rubric_profile": profile,
                    "availability": model_report["availability"],
                }
            )
            residual_components.append(model_report.get("residual_decomposition", {}))
        summaries[model.model_id] = {
            "model_id": model.model_id,
            "mechanism": model.mechanism,
            "availability": model.availability,
            "source": model.source,
            "lever_rank_eligible": model.lever_rank_eligible,
            "aggregate": aggregate_profile(site_profiles),
            "residual_decomposition": aggregate_residual_decomposition(residual_components),
            "site_summaries": [
                {
                    "site_id": item["site_id"],
                    "corpus": item["corpus"],
                    "snow_climate": item["snow_climate"],
                    "availability": item["availability"],
                    **profile_score_summary(item["rubric_profile"]),
                }
                for item in site_profiles
            ],
        }
    return summaries


def aggregate_profile(site_profiles: list[dict[str, Any]]) -> dict[str, Any]:
    counts: Counter[str] = Counter()
    robust_counts: Counter[str] = Counter()
    limited_counts: Counter[str] = Counter()
    robust_score = 0
    robust_available = 0
    robust_fail = 0
    for site in site_profiles:
        profile = site["rubric_profile"]
        counts.update(profile["summary"]["counts_by_label"])
        robust_counts.update(profile["summary"]["forcing_robust_counts_by_label"])
        limited_counts.update(profile["summary"]["forcing_limited_counts_by_label"])
        for cell in profile["cells"]:
            label = cell["ordinal_label"]
            if not cell["forcing_robust"] or label not in LABEL_SCORE:
                continue
            robust_available += 1
            robust_score += LABEL_SCORE[label]
            robust_fail += int(label == "fail")
    return {
        "counts_by_label": dict(sorted(counts.items())),
        "forcing_robust_counts_by_label": dict(sorted(robust_counts.items())),
        "forcing_limited_counts_by_label": dict(sorted(limited_counts.items())),
        "robust_available_cell_count": robust_available,
        "robust_fail_count": robust_fail,
        "robust_ordinal_score": robust_score,
    }


def compare_to_activated(
    site_reports: list[dict[str, Any]], models: list[ModelSpec]
) -> dict[str, Any]:
    candidates = [model for model in models if model.model_id != "activated_bundle"]
    comparisons = {}
    signature_improvements = []
    for model in candidates:
        better = equal = worse = unpaired = 0
        robust_fail_delta = 0
        score_delta = 0
        component_deltas = []
        for site in site_reports:
            activated = site["models"]["activated_bundle"]
            candidate = site["models"][model.model_id]
            activated_scores = robust_cell_scores(activated["rubric_profile"])
            candidate_scores = robust_cell_scores(candidate["rubric_profile"])
            robust_fail_delta += robust_fail_count(activated["rubric_profile"]) - robust_fail_count(
                candidate["rubric_profile"]
            )
            score_delta += robust_score(candidate["rubric_profile"]) - robust_score(
                activated["rubric_profile"]
            )
            for cell_id, candidate_score in candidate_scores.items():
                activated_score = activated_scores.get(cell_id)
                if activated_score is None:
                    unpaired += 1
                    continue
                if candidate_score > activated_score:
                    better += 1
                    signature_improvements.append(
                        improvement_row(site, model, activated["rubric_profile"], candidate["rubric_profile"], cell_id)
                    )
                elif candidate_score == activated_score:
                    equal += 1
                else:
                    worse += 1
            component_deltas.append(
                residual_component_delta(
                    activated.get("residual_decomposition", {}),
                    candidate.get("residual_decomposition", {}),
                )
            )
        comparisons[model.model_id] = {
            "model_id": model.model_id,
            "mechanism": model.mechanism,
            "better_robust_cells_vs_activated": better,
            "equal_robust_cells_vs_activated": equal,
            "worse_robust_cells_vs_activated": worse,
            "unpaired_robust_cells": unpaired,
            "robust_fail_delta_activated_minus_candidate": robust_fail_delta,
            "robust_score_delta_candidate_minus_activated": score_delta,
            "residual_component_delta_vs_activated": aggregate_component_deltas(component_deltas),
        }
    return {
        "baseline_model": "activated_bundle",
        "comparisons": comparisons,
        "signature_improvements": signature_improvements,
    }


def improvement_row(
    site: dict[str, Any],
    model: ModelSpec,
    activated_profile: dict[str, Any],
    candidate_profile: dict[str, Any],
    cell_id: str,
) -> dict[str, Any]:
    activated_cell = cell_by_id(activated_profile, cell_id)
    candidate_cell = cell_by_id(candidate_profile, cell_id)
    return {
        "site_id": site["site_id"],
        "corpus": site["corpus"],
        "snow_climate": site["snow_climate"],
        "model_id": model.model_id,
        "mechanism": model.mechanism,
        "cell_id": cell_id,
        "signature": candidate_cell["signature"],
        "timescale": candidate_cell["timescale"],
        "tier": candidate_cell["tier"],
        "activated_label": activated_cell["ordinal_label"],
        "candidate_label": candidate_cell["ordinal_label"],
        "score_delta": LABEL_SCORE[candidate_cell["ordinal_label"]]
        - LABEL_SCORE[activated_cell["ordinal_label"]],
    }


def rank_levers(
    model_summaries: dict[str, Any],
    improvement_matrix: dict[str, Any],
    models: list[ModelSpec],
) -> list[dict[str, Any]]:
    eligible = {model.model_id for model in models if model.lever_rank_eligible}
    rows = []
    for model_id in eligible:
        comparison = improvement_matrix["comparisons"][model_id]
        summary = model_summaries[model_id]["aggregate"]
        rows.append(
            {
                "model_id": model_id,
                "mechanism": model_summaries[model_id]["mechanism"],
                "rank_basis": (
                    "supported current direct-runtime candidates only; legacy/PySnobal and "
                    "archival rejected candidates are flags, not promotion targets"
                ),
                "robust_fail_count": summary["robust_fail_count"],
                "robust_ordinal_score": summary["robust_ordinal_score"],
                "better_robust_cells_vs_activated": comparison[
                    "better_robust_cells_vs_activated"
                ],
                "worse_robust_cells_vs_activated": comparison[
                    "worse_robust_cells_vs_activated"
                ],
                "robust_fail_delta_activated_minus_candidate": comparison[
                    "robust_fail_delta_activated_minus_candidate"
                ],
                "robust_score_delta_candidate_minus_activated": comparison[
                    "robust_score_delta_candidate_minus_activated"
                ],
            }
        )
    rows.sort(
        key=lambda row: (
            -row["robust_score_delta_candidate_minus_activated"],
            -row["robust_fail_delta_activated_minus_candidate"],
            row["worse_robust_cells_vs_activated"],
            row["model_id"],
        )
    )
    for index, row in enumerate(rows, start=1):
        row["rank"] = index
    return rows


def humid_new_england_read(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    hne_sites = [
        site
        for site in site_reports
        if site["corpus"] == "cancov_forest" and site["snow_climate"].startswith("humid_new_england")
    ]
    snotel_sites = [site for site in site_reports if site["corpus"] == "snotel_observed"]
    hne_fails = activated_fail_cell_ids(hne_sites)
    snotel_fails = activated_fail_cell_ids(snotel_sites)
    overlap = sorted(hne_fails & snotel_fails)
    union = hne_fails | snotel_fails
    jaccard = len(overlap) / len(union) if union else None
    if jaccard is None:
        read = "INCONCLUSIVE"
    elif jaccard >= 0.50:
        read = "PARTIALLY-REPRESENTATIVE"
    else:
        read = "NOT-REPRESENTATIVE"
    return {
        "humid_new_england_site_ids": [site["site_id"] for site in hne_sites],
        "snotel_site_ids": [site["site_id"] for site in snotel_sites],
        "activated_humid_new_england_fail_cell_ids": sorted(hne_fails),
        "activated_snotel_fail_cell_ids": sorted(snotel_fails),
        "shared_fail_cell_ids": overlap,
        "fail_cell_jaccard": jaccard,
        "representative_read": read,
        "interpretation": (
            "Representative here means the activated-bundle forcing-robust fail "
            "signature set overlaps across humid-New-England cancov strata and "
            "the mountain SNOTEL corpus. It is diagnostic, not a verdict."
        ),
    }


def summarize_report(
    model_summaries: dict[str, Any],
    improvement_matrix: dict[str, Any],
    lever_rank: list[dict[str, Any]],
    humid_ne: dict[str, Any],
) -> dict[str, Any]:
    activated = model_summaries["activated_bundle"]["aggregate"]
    return {
        "disposition": "DIAGNOSTIC-COMPLETE-NO-PROMOTION-DECISION",
        "activated_bundle_robust_fail_count": activated["robust_fail_count"],
        "activated_bundle_robust_ordinal_score": activated["robust_ordinal_score"],
        "model_count": len(model_summaries),
        "supported_direct_runtime_model_count": sum(
            1
            for model in model_summaries.values()
            if model["availability"] == "current_direct_runtime"
        ),
        "signature_improvement_count": len(improvement_matrix["signature_improvements"]),
        "top_supported_next_lever": lever_rank[0]["model_id"] if lever_rank else None,
        "humid_new_england_representative_read": humid_ne["representative_read"],
        "absolute_swe_depth_cells_are_report_only": True,
        "activation_authorized": False,
        "promotion_decision_made": False,
    }


def residual_decomposition(metrics: dict[str, Any]) -> dict[str, Any]:
    result = {"paired_count": metrics.get("paired_count", 0)}
    for component in COMPONENTS:
        summary = metrics.get(component, {})
        result[component] = {
            "median_signed": summary.get("median_signed"),
            "mean_signed": summary.get("mean_signed"),
            "mean_abs": summary.get("mean_abs"),
            "median_abs": summary.get("median_abs"),
            "fail_count": summary.get("fail_count"),
            "modeled_over_observed_count": summary.get("modeled_over_observed_count"),
            "modeled_under_observed_count": summary.get("modeled_under_observed_count"),
        }
    return result


def aggregate_residual_decomposition(items: list[dict[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for component in COMPONENTS:
        result[component] = {
            "site_count": sum(1 for item in items if item.get(component)),
            "median_signed_values": [
                item[component]["median_signed"]
                for item in items
                if item.get(component, {}).get("median_signed") is not None
            ],
            "mean_abs_values": [
                item[component]["mean_abs"]
                for item in items
                if item.get(component, {}).get("mean_abs") is not None
            ],
        }
        result[component]["median_of_median_signed"] = rubric.median(
            result[component]["median_signed_values"]
        )
        result[component]["median_of_mean_abs"] = rubric.median(
            result[component]["mean_abs_values"]
        )
    return result


def residual_component_delta(
    activated: dict[str, Any],
    candidate: dict[str, Any],
) -> dict[str, Any]:
    delta = {}
    for component in COMPONENTS:
        active_component = activated.get(component, {})
        candidate_component = candidate.get(component, {})
        active_mae = active_component.get("mean_abs")
        candidate_mae = candidate_component.get("mean_abs")
        active_signed = active_component.get("median_signed")
        candidate_signed = candidate_component.get("median_signed")
        delta[component] = {
            "mean_abs_delta_candidate_minus_activated": numeric_delta(candidate_mae, active_mae),
            "median_signed_delta_candidate_minus_activated": numeric_delta(
                candidate_signed, active_signed
            ),
        }
    return delta


def aggregate_component_deltas(items: list[dict[str, Any]]) -> dict[str, Any]:
    result = {}
    for component in COMPONENTS:
        mae_deltas = [
            item[component]["mean_abs_delta_candidate_minus_activated"]
            for item in items
            if item.get(component, {}).get("mean_abs_delta_candidate_minus_activated") is not None
        ]
        signed_deltas = [
            item[component]["median_signed_delta_candidate_minus_activated"]
            for item in items
            if item.get(component, {}).get("median_signed_delta_candidate_minus_activated")
            is not None
        ]
        result[component] = {
            "median_mean_abs_delta_candidate_minus_activated": rubric.median(mae_deltas),
            "median_signed_delta_candidate_minus_activated": rubric.median(signed_deltas),
            "improved_mean_abs_site_count": sum(1 for value in mae_deltas if value < 0.0),
            "worse_mean_abs_site_count": sum(1 for value in mae_deltas if value > 0.0),
        }
    return result


def flatten_matrix(site_reports: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for site in site_reports:
        for model_id, model in site["models"].items():
            for cell in model["rubric_profile"]["cells"]:
                rows.append(
                    {
                        "corpus": site["corpus"],
                        "site_id": site["site_id"],
                        "snow_climate": site["snow_climate"],
                        "model_id": model_id,
                        "mechanism": model["mechanism"],
                        "cell_id": cell["cell_id"],
                        "timescale": cell["timescale"],
                        "signature": cell["signature"],
                        "tier": cell["tier"],
                        "forcing_robust": cell["forcing_robust"],
                        "ordinal_label": cell["ordinal_label"],
                        "ordinal_score": cell["ordinal_score"],
                        "adr017_cell_verdict": cell["adr017_cell_verdict"],
                    }
                )
    return rows


def activated_fail_cell_ids(sites: list[dict[str, Any]]) -> set[str]:
    fail_cells = set()
    for site in sites:
        profile = site["models"]["activated_bundle"]["rubric_profile"]
        for cell in profile["cells"]:
            if cell["forcing_robust"] and cell["ordinal_label"] == "fail":
                fail_cells.add(cell["cell_id"])
    return fail_cells


def profile_score_summary(profile: dict[str, Any]) -> dict[str, Any]:
    return {
        "paired_count": profile["paired_count"],
        "robust_available_cell_count": robust_available(profile),
        "robust_fail_count": robust_fail_count(profile),
        "robust_ordinal_score": robust_score(profile),
        "forcing_robust_counts_by_label": profile["summary"][
            "forcing_robust_counts_by_label"
        ],
    }


def robust_cell_scores(profile: dict[str, Any]) -> dict[str, int]:
    scores = {}
    for cell in profile["cells"]:
        label = cell["ordinal_label"]
        if cell["forcing_robust"] and label in LABEL_SCORE:
            scores[cell["cell_id"]] = LABEL_SCORE[label]
    return scores


def robust_available(profile: dict[str, Any]) -> int:
    return sum(
        1
        for cell in profile["cells"]
        if cell["forcing_robust"] and cell["ordinal_label"] in LABEL_SCORE
    )


def robust_fail_count(profile: dict[str, Any]) -> int:
    return sum(
        1
        for cell in profile["cells"]
        if cell["forcing_robust"] and cell["ordinal_label"] == "fail"
    )


def robust_score(profile: dict[str, Any]) -> int:
    return sum(
        LABEL_SCORE[cell["ordinal_label"]]
        for cell in profile["cells"]
        if cell["forcing_robust"] and cell["ordinal_label"] in LABEL_SCORE
    )


def cell_by_id(profile: dict[str, Any], cell_id: str) -> dict[str, Any]:
    for cell in profile["cells"]:
        if cell["cell_id"] == cell_id:
            return cell
    raise KeyError(cell_id)


def numeric_delta(left: float | None, right: float | None) -> float | None:
    if left is None or right is None:
        return None
    return left - right


def empty_metrics(model_id: str) -> dict[str, Any]:
    empty_component = {
        "count": 0,
        "mean_signed": None,
        "median_signed": None,
        "mean_abs": None,
        "median_abs": None,
        "max_abs": None,
        "pass_count": 0,
        "fail_count": 0,
        "modeled_over_observed_count": 0,
        "modeled_under_observed_count": 0,
    }
    return {
        "model_id": model_id,
        "paired_count": 0,
        "swe": dict(empty_component),
        "depth": dict(empty_component),
        "density": dict(empty_component),
        "sample_pairs": [],
    }


def model_record(model: ModelSpec) -> dict[str, Any]:
    return {
        "model_id": model.model_id,
        "mechanism": model.mechanism,
        "availability": model.availability,
        "env": printable_env(model.env),
        "source": model.source,
        "note": model.note,
        "lever_rank_eligible": model.lever_rank_eligible,
    }


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


def printable_env(env: dict[str, str | None]) -> dict[str, str]:
    return {key: ("<absent>" if value is None else value) for key, value in sorted(env.items())}


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def write_report(report: dict[str, Any], output_dir: Path, package_artifacts_dir: Path) -> None:
    rubric.write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(render_markdown(report), encoding="utf-8")
    rubric.write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.18 Cross-SNOTEL Mechanism Rubric",
        "",
        "Evidence mode: Static/Ran diagnostic. No promotion or activation decision.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Models scored: `{summary['model_count']}`",
        f"- Supported direct-runtime models: `{summary['supported_direct_runtime_model_count']}`",
        f"- Activated robust fail count: `{summary['activated_bundle_robust_fail_count']}`",
        f"- Activated robust score: `{summary['activated_bundle_robust_ordinal_score']}`",
        f"- Signature improvements vs activated: `{summary['signature_improvement_count']}`",
        f"- Top supported next lever: `{summary['top_supported_next_lever']}`",
        f"- Humid-New-England read: `{summary['humid_new_england_representative_read']}`",
        "",
        "## Model Summary",
        "",
        "| Model | Availability | Robust fail | Robust score | SWE median bias | Depth median bias | Density median bias |",
        "|---|---|---:|---:|---:|---:|---:|",
    ]
    for model_id, model in report["model_summaries"].items():
        aggregate = model["aggregate"]
        residual = model["residual_decomposition"]
        lines.append(
            "| `{model}` | `{availability}` | {fail} | {score} | {swe} | {depth} | {density} |".format(
                model=model_id,
                availability=model["availability"],
                fail=aggregate["robust_fail_count"],
                score=aggregate["robust_ordinal_score"],
                swe=fmt(residual["swe"]["median_of_median_signed"]),
                depth=fmt(residual["depth"]["median_of_median_signed"]),
                density=fmt(residual["density"]["median_of_median_signed"]),
            )
        )
    lines.extend(
        [
            "",
            "## Supported Lever Rank",
            "",
            "| Rank | Model | Robust score delta | Robust fail delta | Better cells | Worse cells |",
            "|---:|---|---:|---:|---:|---:|",
        ]
    )
    for row in report["ranked_next_global_lever"]:
        lines.append(
            "| {rank} | `{model}` | {score_delta} | {fail_delta} | {better} | {worse} |".format(
                rank=row["rank"],
                model=row["model_id"],
                score_delta=row["robust_score_delta_candidate_minus_activated"],
                fail_delta=row["robust_fail_delta_activated_minus_candidate"],
                better=row["better_robust_cells_vs_activated"],
                worse=row["worse_robust_cells_vs_activated"],
            )
        )
    lines.extend(
        [
            "",
            "## Signature Improvements",
            "",
            "| Site | Climate | Model | Cell | Activated | Candidate |",
            "|---|---|---|---|---|---|",
        ]
    )
    for row in report["mechanism_improvements"]["signature_improvements"][:80]:
        lines.append(
            "| `{site}` | `{climate}` | `{model}` | `{cell}` | `{active}` | `{candidate}` |".format(
                site=row["site_id"],
                climate=row["snow_climate"],
                model=row["model_id"],
                cell=row["cell_id"],
                active=row["activated_label"],
                candidate=row["candidate_label"],
            )
        )
    lines.extend(
        [
            "",
            "Absolute SWE/depth magnitude cells are forcing-limited report-only cells under INV-SNOWFREEZE-050. Legacy and PySnobal profiles are ADR-0017 flags, not targets.",
            "",
        ]
    )
    return "\n".join(lines)


def fmt(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
