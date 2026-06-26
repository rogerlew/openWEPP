#!/usr/bin/env python3
"""Adjudicate diagnostic CoE melt variants against SNOTEL rubric profiles.

This is SNOWDENSITY-05G evidence tooling. It runs the diagnostic snowbench
``coe-melt`` command for ``legacy_coe`` and ``coe_shortwave_albedo_v1`` across
the five SNOTEL fixtures, then scores the existing INV-SNOWFREEZE-050 rubric.
It does not activate the opt-in model in production runtime.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import subprocess
from pathlib import Path
from typing import Any

import snotel_density_three_way as snotel


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snotel_observed/observations"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity05g_coe_melt_adjudication"
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
DEFAULT_H_COMPARATOR = REPO_ROOT / "target/snowfrost_fidelity_h/three_way_comparison.json"
FIXTURE_ROOT = REPO_ROOT / "tests/fixtures/snotel_observed"
MODELS = ["legacy_coe", "coe_shortwave_albedo_v1"]
LABEL_SCORE = {
    "fail": 0,
    "marginal": 1,
    "pass": 2,
    "strong": 3,
}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--h-comparator-json", type=Path, default=DEFAULT_H_COMPARATOR)
    parser.add_argument("--site", action="append", default=[])
    args = parser.parse_args(argv)

    report = adjudicate(
        observations_dir=args.observations_dir.resolve(),
        output_dir=args.output_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        h_comparator_json=args.h_comparator_json.resolve(),
        sites=snotel.selected_sites(set(args.site)),
    )
    snotel.write_json(args.output_dir / "coe_melt_adjudication.json", report)
    (args.output_dir / "coe_melt_adjudication.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return 0


def adjudicate(
    observations_dir: Path,
    output_dir: Path,
    snowbench_binary: Path,
    h_comparator_json: Path,
    sites: list[snotel.SnotelSite],
) -> dict[str, Any]:
    snotel.validate_observations(observations_dir, sites)
    if not snowbench_binary.is_file():
        raise FileNotFoundError(f"openwepp-snowbench binary not found: {snowbench_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    model_profiles = [
        run_model_profile(
            observations_dir=observations_dir,
            output_dir=output_dir / "models" / model,
            snowbench_binary=snowbench_binary,
            model=model,
            sites=sites,
        )
        for model in MODELS
    ]
    summaries = {profile["model_id"]: summarize_profile(profile) for profile in model_profiles}
    opt_in = summaries["coe_shortwave_albedo_v1"]
    legacy = summaries["legacy_coe"]
    h_comparators = load_h_comparators(h_comparator_json)
    disposition = disposition_from_profiles(opt_in, legacy)
    return {
        "schema": "snowdensity05g-coe-melt-adjudication-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-052 INV-SNOWFREEZE-055 INV-SNOWFREEZE-057",
        "runtime_coupling": "diagnostic snowbench replay only; no production activation",
        "no_site_constants": True,
        "snowbench_binary": str(snowbench_binary),
        "h_comparator_json": str(h_comparator_json) if h_comparator_json.is_file() else None,
        "summary": {
            "disposition": disposition,
            "promotion_rule": (
                "opt-in robust_fail_count must be lower than diagnostic legacy "
                "and robust_ordinal_score must be at least diagnostic legacy"
            ),
            "beats_diagnostic_legacy": disposition == "PROMOTION-CANDIDATE",
            "opt_in_vs_diagnostic_legacy": compare_model_summaries(opt_in, legacy),
            "h_comparator_available": bool(h_comparators),
        },
        "models": model_profiles,
        "model_summaries": summaries,
        "h_comparators": h_comparators,
    }


def run_model_profile(
    observations_dir: Path,
    output_dir: Path,
    snowbench_binary: Path,
    model: str,
    sites: list[snotel.SnotelSite],
) -> dict[str, Any]:
    site_reports = []
    output_dir.mkdir(parents=True, exist_ok=True)
    for site in sites:
        model_dir = output_dir / "runs" / site.site_id
        run_snowbench(site, model_dir, snowbench_binary, model)
        observations = snotel.read_csv_dicts(observations_dir / "sites" / f"{site.site_id}.csv")
        modeled = load_coe_melt_series(model_dir / "coe_melt_snow.csv")
        summary = read_json(model_dir / "coe_melt_summary.json")
        metrics = snotel.model_metrics(observations, modeled, model)
        rubric = snotel.rubric_profile(observations, modeled, model)
        site_reports.append(
            {
                "site_id": site.site_id,
                "station_triplet": site.triplet,
                "snow_climate": site.snow_climate,
                "model_id": model,
                "run_dir": str(model_dir),
                "snow_csv": str(model_dir / "coe_melt_snow.csv"),
                "summary_json": str(model_dir / "coe_melt_summary.json"),
                "snowbench_summary": summary,
                "metrics": metrics,
                "rubric_profile": rubric,
            }
        )
    return {
        "schema": "snowdensity05g-coe-melt-snotel-profile-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-055 INV-SNOWFREEZE-057",
        "model_id": model,
        "runtime_coupling": "diagnostic snowbench replay only",
        "no_site_constants": True,
        "output_dir": str(output_dir),
        "sites": site_reports,
        "summary": summarize_sites(site_reports),
    }


def run_snowbench(
    site: snotel.SnotelSite,
    model_dir: Path,
    snowbench_binary: Path,
    model: str,
) -> None:
    model_dir.mkdir(parents=True, exist_ok=True)
    command = [
        str(snowbench_binary),
        "coe-melt",
        "--run-dir",
        str(FIXTURE_ROOT / site.site_id),
        "--output-dir",
        str(model_dir),
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
    (model_dir / "openwepp-snowbench.stdout").write_text(completed.stdout, encoding="utf-8")
    (model_dir / "openwepp-snowbench.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise RuntimeError(
            f"openwepp-snowbench coe-melt failed for {site.site_id} {model} "
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
    if parsed != parsed:
        return None
    return parsed


def summarize_sites(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    paired_count = 0
    robust_score = 0
    robust_available = 0
    robust_fail = 0
    for report in site_reports:
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
        "site_count": len(site_reports),
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
    summary["site_summaries"] = []
    for site in profile["sites"]:
        robust_score = 0
        robust_available = 0
        robust_fail = 0
        for cell in site["rubric_profile"]["cells"]:
            label = cell["ordinal_label"]
            if not cell["forcing_robust"] or label == "unavailable":
                continue
            robust_available += 1
            robust_score += LABEL_SCORE[label]
            robust_fail += int(label == "fail")
        summary["site_summaries"].append(
            {
                "site_id": site["site_id"],
                "robust_counts_by_label": site["rubric_profile"]["summary"][
                    "forcing_robust_counts_by_label"
                ],
                "robust_available_cell_count": robust_available,
                "robust_fail_count": robust_fail,
                "robust_ordinal_score": robust_score,
            }
        )
    return summary


def disposition_from_profiles(opt_in: dict[str, Any], legacy: dict[str, Any]) -> str:
    if (
        opt_in["robust_fail_count"] < legacy["robust_fail_count"]
        and opt_in["robust_ordinal_score"] >= legacy["robust_ordinal_score"]
    ):
        return "PROMOTION-CANDIDATE"
    return "NON-PROMOTION"


def compare_model_summaries(candidate: dict[str, Any], comparator: dict[str, Any]) -> dict[str, Any]:
    return {
        "candidate_robust_fail_count": candidate["robust_fail_count"],
        "comparator_robust_fail_count": comparator["robust_fail_count"],
        "candidate_robust_ordinal_score": candidate["robust_ordinal_score"],
        "comparator_robust_ordinal_score": comparator["robust_ordinal_score"],
        "candidate_robust_counts_by_label": candidate["forcing_robust_counts_by_label"],
        "comparator_robust_counts_by_label": comparator["forcing_robust_counts_by_label"],
    }


def load_h_comparators(path: Path) -> dict[str, Any]:
    if not path.is_file():
        return {}
    report = read_json(path)
    result = {}
    for model_id in ("openwepp_as_built", "legacy_as_built", "pysnobal"):
        site_profiles = []
        for site in report.get("sites", []):
            model = site.get("models", {}).get(model_id)
            if model is None:
                continue
            site_profiles.append(
                {
                    "site_id": site["site_id"],
                    "rubric_profile": model["rubric_profile"],
                }
            )
        if site_profiles:
            result[model_id] = summarize_h_site_profiles(model_id, site_profiles)
    return result


def summarize_h_site_profiles(model_id: str, site_profiles: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    robust_available = 0
    robust_fail = 0
    robust_score = 0
    for site in site_profiles:
        summary = site["rubric_profile"]["summary"]
        merge_counts(counts, summary["counts_by_label"])
        merge_counts(robust_counts, summary["forcing_robust_counts_by_label"])
        for cell in site["rubric_profile"]["cells"]:
            label = cell["ordinal_label"]
            if not cell["forcing_robust"] or label == "unavailable":
                continue
            robust_available += 1
            robust_score += LABEL_SCORE[label]
            robust_fail += int(label == "fail")
    return {
        "model_id": model_id,
        "counts_by_label": dict(sorted(counts.items())),
        "forcing_robust_counts_by_label": dict(sorted(robust_counts.items())),
        "robust_available_cell_count": robust_available,
        "robust_fail_count": robust_fail,
        "robust_ordinal_score": robust_score,
    }


def merge_counts(target: dict[str, int], source: dict[str, int]) -> None:
    for key, value in source.items():
        target[key] = target.get(key, 0) + int(value)


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# SNOWDENSITY-05G CoE Melt Adjudication",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Contract: `{report['contract']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        f"- No site constants: `{report['no_site_constants']}`",
        f"- Disposition: `{report['summary']['disposition']}`",
        f"- Beats diagnostic legacy: `{report['summary']['beats_diagnostic_legacy']}`",
        "",
        "## Model Summary",
        "",
        "| Model | Paired rows | Robust fail | Robust score | Robust counts |",
        "|---|---:|---:|---:|---|",
    ]
    for model_id in MODELS:
        summary = report["model_summaries"][model_id]
        lines.append(
            "| `{model}` | {paired} | {fail} | {score} | `{counts}` |".format(
                model=model_id,
                paired=summary["paired_count"],
                fail=summary["robust_fail_count"],
                score=summary["robust_ordinal_score"],
                counts=json.dumps(summary["forcing_robust_counts_by_label"], sort_keys=True),
            )
        )
    lines.extend(
        [
            "",
            "## Site Summary",
            "",
            "| Model | Site | Robust fail | Robust score | Robust counts |",
            "|---|---|---:|---:|---|",
        ]
    )
    for model_id in MODELS:
        for site in report["model_summaries"][model_id]["site_summaries"]:
            lines.append(
                "| `{model}` | `{site}` | {fail} | {score} | `{counts}` |".format(
                    model=model_id,
                    site=site["site_id"],
                    fail=site["robust_fail_count"],
                    score=site["robust_ordinal_score"],
                    counts=json.dumps(site["robust_counts_by_label"], sort_keys=True),
                )
            )
    if report["h_comparators"]:
        lines.extend(
            [
                "",
                "## H Comparator Context",
                "",
                "| Model | Robust fail | Robust score | Robust counts |",
                "|---|---:|---:|---|",
            ]
        )
        for model_id, summary in sorted(report["h_comparators"].items()):
            lines.append(
                "| `{model}` | {fail} | {score} | `{counts}` |".format(
                    model=model_id,
                    fail=summary["robust_fail_count"],
                    score=summary["robust_ordinal_score"],
                    counts=json.dumps(summary["forcing_robust_counts_by_label"], sort_keys=True),
                )
            )
    lines.extend(
        [
            "",
            "Disposition rule: the opt-in path is a promotion candidate only if it "
            "reduces forcing-robust failures relative to diagnostic legacy without "
            "lowering the forcing-robust ordinal score. Observation-only disagreement "
            "does not create an `OPENWEPP-DEFECTIVE` label under ADR-0017.",
            "",
        ]
    )
    return "\n".join(lines)


if __name__ == "__main__":
    raise SystemExit(main())
