#!/usr/bin/env python3
"""Adjudicate CoE-bound density replay candidates against SNOTEL profiles.

This is SNOWDENSITY-06B evidence tooling. It runs the offline
``coe-bound-density`` snowbench command for fixed CoE melt boundaries and
scores the resulting density/depth/SWE series with the snow-frost rubric.
It does not activate a production runtime selector.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
import subprocess
from pathlib import Path
from typing import Any

import physics_bulk_adjudication as pba
import snotel_density_three_way as snotel


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snotel_observed/observations"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity06b_coe_bound_density"
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
DEFAULT_H_COMPARATOR = REPO_ROOT / "target/snowfrost_fidelity_h/three_way_comparison.json"
FIXTURE_ROOT = REPO_ROOT / "tests/fixtures/snotel_observed"
DEFAULT_BOUNDARIES = ["legacy_coe", "coe_shortwave_albedo_v1"]
DEFAULT_VARIANT = "density_compaction_v1"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--h-comparator-json", type=Path, default=DEFAULT_H_COMPARATOR)
    parser.add_argument("--boundary", action="append", default=[])
    parser.add_argument("--variant", default=DEFAULT_VARIANT)
    parser.add_argument("--site", action="append", default=[])
    args = parser.parse_args(argv)

    report = adjudicate(
        observations_dir=args.observations_dir.resolve(),
        output_dir=args.output_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        h_comparator_json=args.h_comparator_json.resolve(),
        boundaries=args.boundary or DEFAULT_BOUNDARIES,
        variant=args.variant,
        sites=snotel.selected_sites(set(args.site)),
    )
    snotel.write_json(args.output_dir / "coe_bound_density_adjudication.json", report)
    (args.output_dir / "coe_bound_density_adjudication.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return 0


def adjudicate(
    observations_dir: Path,
    output_dir: Path,
    snowbench_binary: Path,
    h_comparator_json: Path,
    boundaries: list[str],
    variant: str,
    sites: list[snotel.SnotelSite],
) -> dict[str, Any]:
    snotel.validate_observations(observations_dir, sites)
    if variant != DEFAULT_VARIANT:
        raise ValueError("SNOWDENSITY-06B only accepts density_compaction_v1")
    if not snowbench_binary.is_file():
        raise FileNotFoundError(f"openwepp-snowbench binary not found: {snowbench_binary}")
    if not h_comparator_json.is_file():
        raise FileNotFoundError(f"H comparator JSON is required: {h_comparator_json}")

    output_dir.mkdir(parents=True, exist_ok=True)
    h_report = read_json(h_comparator_json)
    comparator_summaries = pba.summarize_h_comparators(h_report)
    candidates = [
        summarize_profile(
            run_boundary_profile(
                observations_dir=observations_dir,
                output_dir=output_dir / "boundaries" / boundary,
                snowbench_binary=snowbench_binary,
                boundary=boundary,
                variant=variant,
                sites=sites,
            ),
            h_report,
        )
        for boundary in boundaries
    ]
    best_candidate = sorted(
        candidates,
        key=lambda item: (
            item["robust_fail_count"],
            -item["robust_ordinal_score"],
            item["density_cell_profile"]["fail_count"],
            -item["density_cell_profile"]["ordinal_score"],
            item["coe_boundary_model"],
        ),
    )[0]
    openwepp = comparator_summaries["openwepp_as_built"]
    legacy = comparator_summaries["legacy_as_built"]
    beats_openwepp = profile_beats(best_candidate, openwepp)
    beats_legacy = profile_beats(best_candidate, legacy)
    disposition = (
        "PROMOTION-CANDIDATE"
        if beats_openwepp and beats_legacy
        else "NON-PROMOTION"
    )
    return {
        "schema": "snowdensity06b-coe-bound-density-adjudication-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-058 INV-SNOWFREEZE-059 OBL-SNOWFREEZE-P-034",
        "runtime_coupling": "none; offline CoE-bound density replay only",
        "no_site_constants": True,
        "variant": variant,
        "boundary_set": boundaries,
        "h_comparator_json": str(h_comparator_json),
        "snowbench_binary": str(snowbench_binary),
        "summary": {
            "disposition": disposition,
            "best_model": best_candidate["model_id"],
            "best_boundary": best_candidate["coe_boundary_model"],
            "beats_openwepp_as_built": beats_openwepp,
            "beats_legacy_as_built": beats_legacy,
            "promotion_rule": (
                "candidate robust_fail_count must be lower, robust_ordinal_score "
                "not lower, density fail_count lower, and density ordinal_score "
                "not lower than comparator"
            ),
        },
        "comparators": comparator_summaries,
        "candidates": candidates,
    }


def run_boundary_profile(
    observations_dir: Path,
    output_dir: Path,
    snowbench_binary: Path,
    boundary: str,
    variant: str,
    sites: list[snotel.SnotelSite],
) -> dict[str, Any]:
    output_dir.mkdir(parents=True, exist_ok=True)
    site_reports = []
    for site in sites:
        model_dir = output_dir / "runs" / site.site_id
        run_snowbench(site, model_dir, snowbench_binary, boundary, variant)
        observations = snotel.read_csv_dicts(observations_dir / "sites" / f"{site.site_id}.csv")
        modeled = load_coe_bound_density_series(model_dir / "coe_bound_density_snow.csv")
        summary = read_json(model_dir / "coe_bound_density_summary.json")
        model_id = str(summary["model_id"])
        metrics = snotel.model_metrics(observations, modeled, model_id)
        rubric = snotel.rubric_profile(observations, modeled, model_id)
        site_reports.append(
            {
                "site_id": site.site_id,
                "station_triplet": site.triplet,
                "snow_climate": site.snow_climate,
                "model_id": model_id,
                "coe_boundary_model": boundary,
                "density_variant": variant,
                "run_dir": str(model_dir),
                "snow_csv": str(model_dir / "coe_bound_density_snow.csv"),
                "summary_json": str(model_dir / "coe_bound_density_summary.json"),
                "snowbench_summary": summary,
                "metrics": metrics,
                "rubric_profile": rubric,
            }
        )
    return {
        "schema": "snowdensity06b-coe-bound-density-snotel-profile-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-058 INV-SNOWFREEZE-059",
        "model_id": site_reports[0]["model_id"] if site_reports else "coe_bound_density",
        "coe_boundary_model": boundary,
        "density_variant": variant,
        "runtime_coupling": "none; offline CoE-bound density replay only",
        "no_site_constants": True,
        "output_dir": str(output_dir),
        "summary": summarize_sites(site_reports),
        "sites": site_reports,
    }


def run_snowbench(
    site: snotel.SnotelSite,
    model_dir: Path,
    snowbench_binary: Path,
    boundary: str,
    variant: str,
) -> None:
    model_dir.mkdir(parents=True, exist_ok=True)
    command = [
        str(snowbench_binary),
        "coe-bound-density",
        "--run-dir",
        str(FIXTURE_ROOT / site.site_id),
        "--output-dir",
        str(model_dir),
        "--model",
        boundary,
        "--variant",
        variant,
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
            f"openwepp-snowbench coe-bound-density failed for {site.site_id} "
            f"{boundary} with exit code {completed.returncode}"
        )


def load_coe_bound_density_series(path: Path) -> dict[dt.date, dict[str, float | None]]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    modeled: dict[dt.date, dict[str, float | None]] = {}
    for row in rows:
        date = dt.date.fromisoformat(row["date"])
        modeled[date] = {
            "snow_water_m": parse_float(row["snow_water_m"]),
            "snow_depth_m": parse_float(row["snow_depth_m"]),
            "snow_density_kg_m3": parse_float(row["snow_density_kg_m3"]),
        }
    return modeled


def summarize_sites(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    paired_count = 0
    for report in site_reports:
        paired_count += int(report["metrics"]["paired_count"])
        summary = report["rubric_profile"]["summary"]
        pba.merge_counts(counts, summary["counts_by_label"])
        pba.merge_counts(robust_counts, summary["forcing_robust_counts_by_label"])
    return {
        "site_count": len(site_reports),
        "paired_count": paired_count,
        "counts_by_label": dict(sorted(counts.items())),
        "forcing_robust_counts_by_label": dict(sorted(robust_counts.items())),
        "openwepp_defective_cells": 0,
        "observation_only_failures_are_unresolved": True,
    }


def summarize_profile(profile: dict[str, Any], h_report: dict[str, Any]) -> dict[str, Any]:
    site_profiles = [
        {
            "site_id": site["site_id"],
            "rubric_profile": site["rubric_profile"],
        }
        for site in profile["sites"]
    ]
    summary = pba.summarize_site_profiles(profile["model_id"], site_profiles)
    summary["coe_boundary_model"] = profile["coe_boundary_model"]
    summary["density_variant"] = profile["density_variant"]
    summary["profile_output_dir"] = profile["output_dir"]
    summary["cell_comparison"] = {
        comparator: pba.compare_cells(site_profiles, h_report, comparator)
        for comparator in ("openwepp_as_built", "legacy_as_built", "pysnobal")
    }
    summary["max_abs_coe_swe_identity_residual_m"] = max(
        site["snowbench_summary"]["summary"]["max_abs_coe_swe_identity_residual_m"]
        for site in profile["sites"]
    )
    summary["max_abs_unbounded_swe_residual_m"] = max(
        site["snowbench_summary"]["summary"]["max_abs_unbounded_swe_residual_m"]
        for site in profile["sites"]
    )
    return summary


def profile_beats(candidate: dict[str, Any], comparator: dict[str, Any]) -> bool:
    candidate_density = candidate["density_cell_profile"]
    comparator_density = comparator["density_cell_profile"]
    return (
        candidate["robust_fail_count"] < comparator["robust_fail_count"]
        and candidate["robust_ordinal_score"] >= comparator["robust_ordinal_score"]
        and candidate_density["fail_count"] < comparator_density["fail_count"]
        and candidate_density["ordinal_score"] >= comparator_density["ordinal_score"]
    )


def parse_float(value: str) -> float | None:
    if value == "":
        return None
    parsed = float(value)
    if not math.isfinite(parsed):
        return None
    return parsed


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# SNOWDENSITY-06B CoE-Bound Density Adjudication",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Disposition: `{report['summary']['disposition']}`",
        f"- Best model: `{report['summary']['best_model']}`",
        f"- Best boundary: `{report['summary']['best_boundary']}`",
        f"- Beats openWEPP as-built: `{report['summary']['beats_openwepp_as_built']}`",
        f"- Beats legacy as-built: `{report['summary']['beats_legacy_as_built']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        f"- No site constants: `{report['no_site_constants']}`",
        "",
        "## Comparator Summary",
        "",
        "| Model | Robust fail | Robust score | Density fail | Density score | Robust counts |",
        "|---|---:|---:|---:|---:|---|",
    ]
    for model_id in ("openwepp_as_built", "legacy_as_built", "pysnobal"):
        model = report["comparators"][model_id]
        lines.append(pba.summary_row(model_id, model))
    lines.extend(
        [
            "",
            "## Candidate Summary",
            "",
            "| Boundary | Model | Robust fail | Robust score | Density fail | Density score | Max CoE SWE residual | Max unbounded SWE residual | vs openWEPP | vs legacy |",
            "|---|---|---:|---:|---:|---:|---:|---:|---|---|",
        ]
    )
    for candidate in report["candidates"]:
        density = candidate["density_cell_profile"]
        lines.append(
            "| `{boundary}` | `{model}` | {fail} | {score} | {density_fail} | {density_score} | {identity:.3e} | {unbounded:.3e} | `{openwepp}` | `{legacy}` |".format(
                boundary=candidate["coe_boundary_model"],
                model=candidate["model_id"],
                fail=candidate["robust_fail_count"],
                score=candidate["robust_ordinal_score"],
                density_fail=density["fail_count"],
                density_score=density["ordinal_score"],
                identity=candidate["max_abs_coe_swe_identity_residual_m"],
                unbounded=candidate["max_abs_unbounded_swe_residual_m"],
                openwepp=json.dumps(candidate["cell_comparison"]["openwepp_as_built"], sort_keys=True),
                legacy=json.dumps(candidate["cell_comparison"]["legacy_as_built"], sort_keys=True),
            )
        )
    lines.extend(
        [
            "",
            "Disposition rule: a 06B candidate must improve whole-rubric and density-cell profiles against both openWEPP as-built and legacy as-built while preserving CoE SWE identity. Failure is non-promotion evidence, not authorization to retune melt, canopy, albedo, radiation, or site constants.",
            "",
        ]
    )
    return "\n".join(lines)


if __name__ == "__main__":
    raise SystemExit(main())
