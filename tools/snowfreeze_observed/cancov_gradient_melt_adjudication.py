#!/usr/bin/env python3
"""Adjudicate CoE melt variants across the canopy-gradient fixtures.

This is SNOWDENSITY-10.3.3 evidence tooling. It runs the diagnostic
``openwepp-snowbench coe-melt`` replay for ``legacy_coe`` and
``coe_shortwave_albedo_v1`` against the canopy-stratified Harvard and Marcell
fixtures, then scores the existing INV-SNOWFREEZE-050 rubric.

The tool is diagnostic-only. It does not change production activation,
coefficients, radiation, canopy, albedo, density, or frost behavior.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import snotel_density_three_way as rubric  # noqa: E402


FIXTURE_ROOT = REPO_ROOT / "tests/fixtures/cancov_forest"
OBSERVATION_ROOT = FIXTURE_ROOT / "observations"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_3_gradient_melt_adjudication"
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
MODELS = ["legacy_coe", "coe_shortwave_albedo_v1"]
LABEL_SCORE = {"fail": 0, "marginal": 1, "pass": 2, "strong": 3}


@dataclass(frozen=True)
class Comparison:
    comparison_id: str
    regime: str
    site: str
    observed_stratum: str
    fixture: str
    observation_rows: list[dict[str, str]]
    verdict_scope: str
    binding_status: str
    note: str


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--observations-dir", type=Path, default=OBSERVATION_ROOT)
    parser.add_argument("--fixture-root", type=Path, default=FIXTURE_ROOT)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--skip-runs", action="store_true")
    args = parser.parse_args(argv)

    report = adjudicate(
        observations_dir=args.observations_dir.resolve(),
        fixture_root=args.fixture_root.resolve(),
        output_dir=args.output_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        run_snowbench=not args.skip_runs,
    )
    rubric.write_json(args.output_dir / "gradient_melt_adjudication.json", report)
    (args.output_dir / "gradient_melt_adjudication.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return 0


def adjudicate(
    observations_dir: Path,
    fixture_root: Path,
    output_dir: Path,
    snowbench_binary: Path,
    run_snowbench: bool,
) -> dict[str, Any]:
    if run_snowbench and not snowbench_binary.is_file():
        raise FileNotFoundError(f"openwepp-snowbench binary not found: {snowbench_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)

    comparisons, unbound = load_comparisons(observations_dir)
    ensure_fixture_dirs(fixture_root, comparisons)
    model_profiles = [
        run_model_profile(
            comparisons=comparisons,
            fixture_root=fixture_root,
            output_dir=output_dir / "models" / model,
            snowbench_binary=snowbench_binary,
            model=model,
            run_snowbench=run_snowbench,
        )
        for model in MODELS
    ]
    model_summaries = {profile["model_id"]: summarize_profile(profile) for profile in model_profiles}
    verdict_summary = compare_models(model_summaries["coe_shortwave_albedo_v1"], model_summaries["legacy_coe"])
    return {
        "schema": "snowdensity10-3-3-gradient-melt-adjudication-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-057 INV-SNOWFREEZE-063",
        "runtime_coupling": "diagnostic snowbench replay only; no production activation",
        "no_site_constants": True,
        "no_coefficient_retuning": True,
        "snowbench_binary": str(snowbench_binary),
        "fixture_root": str(fixture_root),
        "observations_dir": str(observations_dir),
        "comparison_set": [comparison_metadata(item) for item in comparisons],
        "unbound_observations": unbound,
        "models": model_profiles,
        "model_summaries": model_summaries,
        "summary": verdict_summary,
    }


def load_comparisons(observations_dir: Path) -> tuple[list[Comparison], list[dict[str, Any]]]:
    harvard_rows = rubric.read_csv_dicts(observations_dir / "sites/harvard_hf237_strata.csv")
    marcell_rows = rubric.read_csv_dicts(
        observations_dir / "sites/marcell_rds_2021_0016_stratum_means.csv"
    )
    comparisons = [
        bound_comparison(
            "marcell_conifer",
            "conifer",
            "marcell",
            "conifer",
            "marcell_conifer_mn",
            marcell_rows,
        ),
        bound_comparison(
            "marcell_deciduous",
            "deciduous",
            "marcell",
            "deciduous",
            "marcell_deciduous_mn",
            marcell_rows,
        ),
        bound_comparison(
            "marcell_open",
            "open_pasture",
            "marcell",
            "open",
            "marcell_open_mn",
            marcell_rows,
        ),
        bound_comparison(
            "harvard_hardwood",
            "deciduous",
            "harvard",
            "hardwood",
            "harvard_deciduous_ma",
            harvard_rows,
        ),
        bound_comparison(
            "harvard_open",
            "open_pasture",
            "harvard",
            "open",
            "harvard_open_ma",
            harvard_rows,
        ),
        aggregate_comparison(
            "marcell_mixed_aggregate",
            "mixed",
            "marcell",
            "unweighted_site_strata",
            "marcell_mixed_mn",
            marcell_rows,
        ),
        aggregate_comparison(
            "harvard_mixed_aggregate",
            "mixed",
            "harvard",
            "unweighted_site_strata",
            "harvard_mixed_ma",
            harvard_rows,
        ),
    ]
    unbound = [
        {
            "source_id": "harvard_hf237",
            "site": "harvard",
            "observed_stratum": "hemlock",
            "binding_status": "unbound_no_pure_conifer_fixture",
            "row_count": sum(1 for row in harvard_rows if row["observed_stratum"] == "hemlock"),
            "verdict_scope": "excluded",
            "reason": "Harvard delineation has no pure hemlock/conifer hillslope; mixed proxy is not a verdict-bearing binding.",
        }
    ]
    for item in comparisons:
        if not item.observation_rows:
            raise ValueError(f"{item.comparison_id} has no observation rows")
    return comparisons, unbound


def bound_comparison(
    comparison_id: str,
    regime: str,
    site: str,
    observed_stratum: str,
    fixture: str,
    rows: list[dict[str, str]],
) -> Comparison:
    filtered = [
        dict(row)
        for row in rows
        if row.get("binding_status") == "bound"
        and row.get("model_fixture") == fixture
        and row.get("observed_stratum") == observed_stratum
    ]
    return Comparison(
        comparison_id=comparison_id,
        regime=regime,
        site=site,
        observed_stratum=observed_stratum,
        fixture=fixture,
        observation_rows=filtered,
        verdict_scope="verdict_bearing",
        binding_status="bound_exact_stratum",
        note="Exact observed stratum to modeled hillslope binding from SNOWDENSITY-10.3.2.",
    )


def aggregate_comparison(
    comparison_id: str,
    regime: str,
    site: str,
    observed_stratum: str,
    fixture: str,
    rows: list[dict[str, str]],
) -> Comparison:
    aggregate_rows = aggregate_rows_by_date(rows, site, fixture)
    return Comparison(
        comparison_id=comparison_id,
        regime=regime,
        site=site,
        observed_stratum=observed_stratum,
        fixture=fixture,
        observation_rows=aggregate_rows,
        verdict_scope="diagnostic_only",
        binding_status="unweighted_aggregate",
        note=(
            "Unweighted same-site stratum aggregate used only to produce a mixed-regime "
            "rubric profile; it is not a canopy-stratum verdict."
        ),
    )


def aggregate_rows_by_date(rows: list[dict[str, str]], site: str, fixture: str) -> list[dict[str, str]]:
    by_date: dict[str, list[dict[str, str]]] = {}
    for row in rows:
        if row.get("binding_status") not in {"bound", "unbound_no_pure_conifer_fixture"}:
            continue
        if row.get("observed_stratum") == "unknown":
            continue
        by_date.setdefault(row["date"], []).append(row)

    aggregate_rows = []
    for date, date_rows in sorted(by_date.items()):
        valid = [row for row in date_rows if has_required_snow_values(row)]
        if len(valid) < 2:
            continue
        depth = mean_float(valid, "observed_snow_depth_m")
        swe = mean_float(valid, "observed_swe_mm")
        if depth is None or swe is None or depth <= 0.0:
            continue
        density = swe / depth
        aggregate_rows.append(
            {
                "source_id": f"{site}_diagnostic_unweighted_stratum_aggregate",
                "observation_site": site,
                "observed_stratum": "unweighted_site_strata",
                "binding_status": "diagnostic_unweighted_aggregate",
                "model_fixture": fixture,
                "date": date,
                "water_year": str(rubric.water_year(dt.date.fromisoformat(date))),
                "observed_snow_depth_m": f"{depth:.6f}",
                "observed_swe_mm": f"{swe:.6f}",
                "observed_density_kg_m3": f"{density:.6f}",
                "sample_count": str(sum(int(row.get("sample_count", "1") or "1") for row in valid)),
                "source_record_id": f"{site}:diagnostic_unweighted_stratum_aggregate:{date}",
                "quality_flag": "diagnostic_unweighted_stratum_aggregate;not_verdict_bearing",
            }
        )
    return aggregate_rows


def has_required_snow_values(row: dict[str, str]) -> bool:
    return all(optional_float(row.get(key)) is not None for key in REQUIRED_OBS_KEYS)


REQUIRED_OBS_KEYS = ["observed_snow_depth_m", "observed_swe_mm", "observed_density_kg_m3"]


def optional_float(value: Any) -> float | None:
    if value is None:
        return None
    if isinstance(value, str) and value.strip() == "":
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def mean_float(rows: list[dict[str, str]], key: str) -> float | None:
    values = [optional_float(row.get(key)) for row in rows]
    finite = [value for value in values if value is not None]
    return sum(finite) / len(finite) if finite else None


def ensure_fixture_dirs(fixture_root: Path, comparisons: list[Comparison]) -> None:
    for comparison in comparisons:
        fixture_dir = fixture_root / comparison.fixture
        if not fixture_dir.is_dir():
            raise FileNotFoundError(fixture_dir)


def run_model_profile(
    comparisons: list[Comparison],
    fixture_root: Path,
    output_dir: Path,
    snowbench_binary: Path,
    model: str,
    run_snowbench: bool,
) -> dict[str, Any]:
    output_dir.mkdir(parents=True, exist_ok=True)
    reports = []
    for comparison in comparisons:
        run_dir = output_dir / "runs" / comparison.comparison_id
        if run_snowbench:
            run_coe_melt(fixture_root / comparison.fixture, run_dir, snowbench_binary, model)
        modeled = load_coe_melt_series(run_dir / "coe_melt_snow.csv")
        summary = read_json(run_dir / "coe_melt_summary.json")
        metrics = rubric.model_metrics(comparison.observation_rows, modeled, model)
        profile = rubric.rubric_profile(comparison.observation_rows, modeled, model)
        reports.append(
            {
                **comparison_metadata(comparison),
                "model_id": model,
                "run_dir": str(run_dir),
                "snow_csv": str(run_dir / "coe_melt_snow.csv"),
                "summary_json": str(run_dir / "coe_melt_summary.json"),
                "snowbench_summary": summary,
                "metrics": metrics,
                "rubric_profile": profile,
            }
        )
    return {
        "schema": "snowdensity10-3-3-gradient-model-profile-v1",
        "model_id": model,
        "runtime_coupling": "diagnostic snowbench replay only",
        "no_site_constants": True,
        "no_coefficient_retuning": True,
        "comparisons": reports,
        "summary": summarize_comparison_reports(reports),
    }


def run_coe_melt(fixture_dir: Path, output_dir: Path, snowbench_binary: Path, model: str) -> None:
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


def load_coe_melt_series(path: Path) -> dict[dt.date, dict[str, float | None]]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    modeled: dict[dt.date, dict[str, float | None]] = {}
    for row in rows:
        date = dt.date.fromisoformat(row["date"])
        modeled[date] = {
            "snow_water_m": parse_optional_float(row["snow_water_m"]),
            "snow_depth_m": parse_optional_float(row["snow_depth_m"]),
            "snow_density_kg_m3": parse_optional_float(row["snow_density_kg_m3"]),
        }
    return modeled


def parse_optional_float(value: str) -> float | None:
    if value == "":
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def summarize_comparison_reports(reports: list[dict[str, Any]]) -> dict[str, Any]:
    total = summarize_scope(reports)
    verdict = summarize_scope([item for item in reports if item["verdict_scope"] == "verdict_bearing"])
    diagnostic = summarize_scope([item for item in reports if item["verdict_scope"] != "verdict_bearing"])
    regimes = {
        regime: summarize_scope([item for item in reports if item["regime"] == regime])
        for regime in ["conifer", "mixed", "deciduous", "open_pasture"]
    }
    verdict_low_canopy = summarize_scope(
        [
            item
            for item in reports
            if item["verdict_scope"] == "verdict_bearing"
            and item["regime"] in {"deciduous", "open_pasture"}
        ]
    )
    return {
        "total": total,
        "verdict_bearing": verdict,
        "diagnostic_only": diagnostic,
        "verdict_bearing_low_canopy": verdict_low_canopy,
        "regimes": regimes,
    }


def summarize_scope(reports: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    paired_count = 0
    robust_score = 0
    robust_available = 0
    robust_fail = 0
    for report in reports:
        paired_count += int(report["metrics"]["paired_count"])
        summary = report["rubric_profile"]["summary"]
        merge_counts(counts, summary["counts_by_label"])
        merge_counts(robust_counts, summary["forcing_robust_counts_by_label"])
        for cell in report["rubric_profile"]["cells"]:
            label = cell["ordinal_label"]
            if not cell["forcing_robust"] or label == "unavailable":
                continue
            robust_available += 1
            robust_score += LABEL_SCORE[label]
            robust_fail += int(label == "fail")
    return {
        "comparison_count": len(reports),
        "paired_count": paired_count,
        "counts_by_label": dict(sorted(counts.items())),
        "forcing_robust_counts_by_label": dict(sorted(robust_counts.items())),
        "robust_available_cell_count": robust_available,
        "robust_fail_count": robust_fail,
        "robust_ordinal_score": robust_score,
        "openwepp_defective_cells": 0,
        "observation_only_failures_are_unresolved": True,
    }


def summarize_profile(profile: dict[str, Any]) -> dict[str, Any]:
    summary = dict(profile["summary"])
    summary["model_id"] = profile["model_id"]
    summary["comparison_summaries"] = []
    for comparison in profile["comparisons"]:
        scope = summarize_scope([comparison])
        summary["comparison_summaries"].append(
            {
                "comparison_id": comparison["comparison_id"],
                "fixture": comparison["fixture"],
                "regime": comparison["regime"],
                "verdict_scope": comparison["verdict_scope"],
                "paired_count": scope["paired_count"],
                "robust_fail_count": scope["robust_fail_count"],
                "robust_ordinal_score": scope["robust_ordinal_score"],
                "forcing_robust_counts_by_label": scope["forcing_robust_counts_by_label"],
            }
        )
    return summary


def compare_models(candidate: dict[str, Any], legacy: dict[str, Any]) -> dict[str, Any]:
    verdict_bearing = compare_scope(
        candidate["verdict_bearing"],
        legacy["verdict_bearing"],
    )
    low_canopy = compare_scope(
        candidate["verdict_bearing_low_canopy"],
        legacy["verdict_bearing_low_canopy"],
    )
    regimes = {
        regime: compare_scope(candidate["regimes"][regime], legacy["regimes"][regime])
        for regime in ["conifer", "mixed", "deciduous", "open_pasture"]
    }
    if low_canopy["candidate_earns_value"]:
        disposition = "EARNS-LOW-CANOPY-DIAGNOSTIC-VALUE"
    elif low_canopy["candidate_is_neutral"]:
        disposition = "LOW-CANOPY-NEUTRAL"
    else:
        disposition = "LOW-CANOPY-NON-PROMOTION"
    return {
        "disposition": disposition,
        "answer": answer_text(disposition, low_canopy, regimes),
        "promotion_authorized": False,
        "default_activation_authorized": False,
        "comparison_rule": (
            "Value outside high-evergreen requires low-canopy verdict-bearing "
            "robust failures not to increase and robust ordinal score to improve, "
            "or robust failures to decrease without score regression."
        ),
        "verdict_bearing": verdict_bearing,
        "verdict_bearing_low_canopy": low_canopy,
        "regimes": regimes,
    }


def compare_scope(candidate: dict[str, Any], legacy: dict[str, Any]) -> dict[str, Any]:
    fail_delta = legacy["robust_fail_count"] - candidate["robust_fail_count"]
    score_delta = candidate["robust_ordinal_score"] - legacy["robust_ordinal_score"]
    paired_delta = candidate["paired_count"] - legacy["paired_count"]
    earns = (fail_delta > 0 and score_delta >= 0) or (fail_delta >= 0 and score_delta > 0)
    neutral = fail_delta == 0 and score_delta == 0
    return {
        "candidate_model": "coe_shortwave_albedo_v1",
        "comparator_model": "legacy_coe",
        "candidate_robust_fail_count": candidate["robust_fail_count"],
        "comparator_robust_fail_count": legacy["robust_fail_count"],
        "robust_fail_delta_positive_is_better": fail_delta,
        "candidate_robust_ordinal_score": candidate["robust_ordinal_score"],
        "comparator_robust_ordinal_score": legacy["robust_ordinal_score"],
        "robust_ordinal_score_delta": score_delta,
        "candidate_forcing_robust_counts_by_label": candidate["forcing_robust_counts_by_label"],
        "comparator_forcing_robust_counts_by_label": legacy["forcing_robust_counts_by_label"],
        "paired_count_delta": paired_delta,
        "candidate_earns_value": earns,
        "candidate_is_neutral": neutral,
    }


def answer_text(disposition: str, low_canopy: dict[str, Any], regimes: dict[str, Any]) -> str:
    if disposition == "EARNS-LOW-CANOPY-DIAGNOSTIC-VALUE":
        return (
            "The shortwave/albedo CoE modernization earns diagnostic value outside "
            "the high-evergreen regime: low-canopy verdict-bearing robust failures "
            f"changed by {low_canopy['robust_fail_delta_positive_is_better']} and "
            f"robust score changed by {low_canopy['robust_ordinal_score_delta']}. "
            "This is not a production activation decision."
        )
    if disposition == "LOW-CANOPY-NEUTRAL":
        return (
            "The shortwave/albedo CoE modernization is neutral on the current "
            "low-canopy verdict-bearing evidence. It does not earn activation value."
        )
    return (
        "The shortwave/albedo CoE modernization does not earn low-canopy value on "
        "the current verdict-bearing evidence. It should stay opt-in/diagnostic "
        "while later packages diagnose partition, rain-on-snow heat, and canopy "
        "longwave mechanisms."
    )


def comparison_metadata(comparison: Comparison) -> dict[str, Any]:
    return {
        "comparison_id": comparison.comparison_id,
        "regime": comparison.regime,
        "site": comparison.site,
        "observed_stratum": comparison.observed_stratum,
        "fixture": comparison.fixture,
        "observation_row_count": len(comparison.observation_rows),
        "verdict_scope": comparison.verdict_scope,
        "binding_status": comparison.binding_status,
        "note": comparison.note,
    }


def merge_counts(target: dict[str, int], source: dict[str, int]) -> None:
    for key, value in source.items():
        target[key] = target.get(key, 0) + int(value)


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# SNOWDENSITY-10.3.3 Gradient Melt Adjudication",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Contract: `{report['contract']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        f"- No site constants: `{report['no_site_constants']}`",
        f"- No coefficient retuning: `{report['no_coefficient_retuning']}`",
        f"- Disposition: `{report['summary']['disposition']}`",
        f"- Answer: {report['summary']['answer']}",
        "",
        "## Verdict Summary",
        "",
        "| Scope | Fail delta | Score delta | Candidate fails | Legacy fails | Candidate score | Legacy score | Earns value |",
        "|---|---:|---:|---:|---:|---:|---:|---|",
    ]
    for scope_id, scope in [
        ("verdict_bearing", report["summary"]["verdict_bearing"]),
        ("verdict_bearing_low_canopy", report["summary"]["verdict_bearing_low_canopy"]),
    ]:
        lines.append(scope_row(scope_id, scope))
    lines.extend(
        [
            "",
            "## Regime Summary",
            "",
            "| Regime | Fail delta | Score delta | Candidate fails | Legacy fails | Candidate score | Legacy score | Scope note |",
            "|---|---:|---:|---:|---:|---:|---:|---|",
        ]
    )
    scope_notes = {
        "conifer": "Marcell conifer exact binding.",
        "mixed": "Diagnostic unweighted aggregate only; no mixed-stratum verdict.",
        "deciduous": "Marcell deciduous + Harvard hardwood exact bindings.",
        "open_pasture": "Marcell open + Harvard open exact bindings.",
    }
    for regime, scope in report["summary"]["regimes"].items():
        lines.append(
            "| `{regime}` | {fail_delta} | {score_delta} | {candidate_fail} | {legacy_fail} | {candidate_score} | {legacy_score} | {note} |".format(
                regime=regime,
                fail_delta=scope["robust_fail_delta_positive_is_better"],
                score_delta=scope["robust_ordinal_score_delta"],
                candidate_fail=scope["candidate_robust_fail_count"],
                legacy_fail=scope["comparator_robust_fail_count"],
                candidate_score=scope["candidate_robust_ordinal_score"],
                legacy_score=scope["comparator_robust_ordinal_score"],
                note=scope_notes[regime],
            )
        )
    lines.extend(
        [
            "",
            "## Comparison Set",
            "",
            "| Comparison | Regime | Fixture | Stratum | Scope | Rows |",
            "|---|---|---|---|---|---:|",
        ]
    )
    for item in report["comparison_set"]:
        lines.append(
            "| `{comparison_id}` | `{regime}` | `{fixture}` | `{observed_stratum}` | `{verdict_scope}` | {rows} |".format(
                comparison_id=item["comparison_id"],
                regime=item["regime"],
                fixture=item["fixture"],
                observed_stratum=item["observed_stratum"],
                verdict_scope=item["verdict_scope"],
                rows=item["observation_row_count"],
            )
        )
    lines.extend(
        [
            "",
            "## Model Summaries",
            "",
            "| Model | Verdict paired | Verdict robust fails | Verdict robust score | Low-canopy fails | Low-canopy score | Robust counts |",
            "|---|---:|---:|---:|---:|---:|---|",
        ]
    )
    for model in MODELS:
        summary = report["model_summaries"][model]
        verdict = summary["verdict_bearing"]
        low = summary["verdict_bearing_low_canopy"]
        lines.append(
            "| `{model}` | {paired} | {fail} | {score} | {low_fail} | {low_score} | `{counts}` |".format(
                model=model,
                paired=verdict["paired_count"],
                fail=verdict["robust_fail_count"],
                score=verdict["robust_ordinal_score"],
                low_fail=low["robust_fail_count"],
                low_score=low["robust_ordinal_score"],
                counts=json.dumps(verdict["forcing_robust_counts_by_label"], sort_keys=True),
            )
        )
    lines.extend(
        [
            "",
            "## Unbound Observations",
            "",
            "| Source | Stratum | Rows | Status | Reason |",
            "|---|---|---:|---|---|",
        ]
    )
    for item in report["unbound_observations"]:
        lines.append(
            "| `{source}` | `{stratum}` | {rows} | `{status}` | {reason} |".format(
                source=item["source_id"],
                stratum=item["observed_stratum"],
                rows=item["row_count"],
                status=item["binding_status"],
                reason=item["reason"],
            )
        )
    lines.extend(
        [
            "",
            "Promotion/default activation is not authorized by this diagnostic package.",
            "Mixed-regime rows are diagnostic unweighted aggregates and do not carry canopy-stratum verdicts.",
            "",
        ]
    )
    return "\n".join(lines)


def scope_row(scope_id: str, scope: dict[str, Any]) -> str:
    return (
        "| `{scope_id}` | {fail_delta} | {score_delta} | {candidate_fail} | {legacy_fail} | {candidate_score} | {legacy_score} | `{earns}` |"
    ).format(
        scope_id=scope_id,
        fail_delta=scope["robust_fail_delta_positive_is_better"],
        score_delta=scope["robust_ordinal_score_delta"],
        candidate_fail=scope["candidate_robust_fail_count"],
        legacy_fail=scope["comparator_robust_fail_count"],
        candidate_score=scope["candidate_robust_ordinal_score"],
        legacy_score=scope["comparator_robust_ordinal_score"],
        earns=scope["candidate_earns_value"],
    )


if __name__ == "__main__":
    raise SystemExit(main())
