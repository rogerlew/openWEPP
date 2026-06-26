#!/usr/bin/env python3
"""Adjudicate offline physics_bulk variants against SNOTEL rubric profiles.

This is SNOWDENSITY-04 evidence tooling. It runs global named variants only;
it does not fit per-site constants and does not couple physics_bulk into
production runtime.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import physics_bulk_snotel_profile
import snotel_density_three_way as snotel


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snotel_observed/observations"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity04_adjudication"
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
DEFAULT_H_COMPARATOR = REPO_ROOT / "target/snowfrost_fidelity_h/three_way_comparison.json"
DEFAULT_VARIANTS = [
    "candidate_v1",
    "slow_melt_v1",
    "dense_slow_melt_v1",
    "cold_dense_slow_melt_v1",
    "density_compaction_v1",
]
DENSITY_CELL_IDS = {
    "long_term_cold_season_bulk_density",
    "seasonal_densification_trajectory",
    "seasonal_depth_swe_slope",
    "cross_cutting_bias_sign_consistency",
}
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
    parser.add_argument("--variant", action="append", default=[])
    args = parser.parse_args(argv)

    variants = args.variant or DEFAULT_VARIANTS
    report = adjudicate(
        observations_dir=args.observations_dir.resolve(),
        output_dir=args.output_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        h_comparator_json=args.h_comparator_json.resolve(),
        variants=variants,
    )
    snotel.write_json(args.output_dir / "physics_bulk_adjudication.json", report)
    (args.output_dir / "physics_bulk_adjudication.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return 0


def adjudicate(
    observations_dir: Path,
    output_dir: Path,
    snowbench_binary: Path,
    h_comparator_json: Path,
    variants: list[str],
) -> dict[str, Any]:
    if not h_comparator_json.is_file():
        raise FileNotFoundError(f"H comparator JSON is required: {h_comparator_json}")
    h_report = read_json(h_comparator_json)
    sites = snotel.SITES
    output_dir.mkdir(parents=True, exist_ok=True)

    comparator_summaries = summarize_h_comparators(h_report)
    candidate_reports = []
    for variant in variants:
        variant_dir = output_dir / "variants" / variant
        profile = physics_bulk_snotel_profile.run_profile(
            observations_dir=observations_dir,
            output_dir=variant_dir,
            snowbench_binary=snowbench_binary,
            variant=variant,
            sites=sites,
        )
        snotel.write_json(variant_dir / "physics_bulk_snotel_profile.json", profile)
        (variant_dir / "physics_bulk_snotel_profile.md").write_text(
            physics_bulk_snotel_profile.render_markdown(profile),
            encoding="utf-8",
        )
        candidate_reports.append(summarize_candidate(profile, h_report))

    best_candidate = sorted(
        candidate_reports,
        key=lambda item: (
            item["robust_fail_count"],
            -item["robust_ordinal_score"],
            item["variant"],
        ),
    )[0]
    openwepp = comparator_summaries["openwepp_as_built"]
    legacy = comparator_summaries["legacy_as_built"]
    beats_openwepp = beats_comparator(best_candidate, openwepp)
    beats_legacy = beats_comparator(best_candidate, legacy)
    disposition = (
        "PROMOTION-CANDIDATE"
        if beats_openwepp and beats_legacy
        else "NON-PROMOTION"
    )
    return {
        "schema": "snowdensity04-physics-bulk-adjudication-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-051 OBL-SNOWFREEZE-P-026",
        "runtime_coupling": "none; offline snowbench candidate only",
        "no_site_constants": True,
        "variant_set": variants,
        "h_comparator_json": str(h_comparator_json),
        "snowbench_binary": str(snowbench_binary),
        "summary": {
            "disposition": disposition,
            "best_variant": best_candidate["variant"],
            "beats_openwepp_as_built": beats_openwepp,
            "beats_legacy_as_built": beats_legacy,
            "promotion_rule": "candidate robust_fail_count must be lower and robust_ordinal_score not lower than comparator",
        },
        "comparators": comparator_summaries,
        "candidates": candidate_reports,
    }


def summarize_h_comparators(h_report: dict[str, Any]) -> dict[str, Any]:
    by_model: dict[str, list[dict[str, Any]]] = {}
    for site in h_report["sites"]:
        for model_id, model in site["models"].items():
            by_model.setdefault(model_id, []).append(
                {
                    "site_id": site["site_id"],
                    "rubric_profile": model["rubric_profile"],
                }
            )
    return {
        model_id: summarize_site_profiles(model_id, site_profiles)
        for model_id, site_profiles in sorted(by_model.items())
    }


def summarize_candidate(profile: dict[str, Any], h_report: dict[str, Any]) -> dict[str, Any]:
    model_id = str(profile["model_id"])
    site_profiles = [
        {
            "site_id": site["site_id"],
            "rubric_profile": site["rubric_profile"],
        }
        for site in profile["sites"]
    ]
    summary = summarize_site_profiles(model_id, site_profiles)
    summary["variant"] = profile["variant"]
    summary["profile_output_dir"] = profile["output_dir"]
    summary["cell_comparison"] = {
        comparator: compare_cells(site_profiles, h_report, comparator)
        for comparator in ("openwepp_as_built", "legacy_as_built", "pysnobal")
    }
    return summary


def summarize_site_profiles(model_id: str, site_profiles: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    robust_score = 0
    robust_available = 0
    robust_fail = 0
    site_summaries = []
    for site in site_profiles:
        rubric = site["rubric_profile"]
        merge_counts(counts, rubric["summary"]["counts_by_label"])
        merge_counts(robust_counts, rubric["summary"]["forcing_robust_counts_by_label"])
        site_score = 0
        site_available = 0
        site_fail = 0
        for cell in rubric["cells"]:
            label = cell["ordinal_label"]
            if not cell["forcing_robust"] or label == "unavailable":
                continue
            site_available += 1
            site_score += LABEL_SCORE[label]
            site_fail += int(label == "fail")
        robust_score += site_score
        robust_available += site_available
        robust_fail += site_fail
        site_summaries.append(
            {
                "site_id": site["site_id"],
                "robust_counts_by_label": rubric["summary"]["forcing_robust_counts_by_label"],
                "robust_available_cell_count": site_available,
                "robust_fail_count": site_fail,
                "robust_ordinal_score": site_score,
            }
        )
    return {
        "model_id": model_id,
        "counts_by_label": dict(sorted(counts.items())),
        "forcing_robust_counts_by_label": dict(sorted(robust_counts.items())),
        "robust_available_cell_count": robust_available,
        "robust_fail_count": robust_fail,
        "robust_ordinal_score": robust_score,
        "density_cell_profile": summarize_density_cells(model_id, site_profiles),
        "site_summaries": site_summaries,
    }


def summarize_density_cells(
    model_id: str, site_profiles: list[dict[str, Any]]
) -> dict[str, Any]:
    counts: dict[str, int] = {}
    score = 0
    available = 0
    fail = 0
    site_summaries = []
    for site in site_profiles:
        site_score = 0
        site_available = 0
        site_fail = 0
        site_cells = []
        for cell in site["rubric_profile"]["cells"]:
            if cell["cell_id"] not in DENSITY_CELL_IDS:
                continue
            label = cell["ordinal_label"]
            site_cells.append(
                {
                    "cell_id": cell["cell_id"],
                    "ordinal_label": label,
                    "ordinal_score": cell["ordinal_score"],
                }
            )
            if label not in LABEL_SCORE:
                continue
            counts[label] = counts.get(label, 0) + 1
            site_available += 1
            site_score += LABEL_SCORE[label]
            site_fail += int(label == "fail")
        available += site_available
        score += site_score
        fail += site_fail
        site_summaries.append(
            {
                "site_id": site["site_id"],
                "available_cell_count": site_available,
                "fail_count": site_fail,
                "ordinal_score": site_score,
                "cells": site_cells,
            }
        )
    return {
        "model_id": model_id,
        "cell_ids": sorted(DENSITY_CELL_IDS),
        "counts_by_label": dict(sorted(counts.items())),
        "available_cell_count": available,
        "fail_count": fail,
        "ordinal_score": score,
        "site_summaries": site_summaries,
    }


def compare_cells(
    candidate_sites: list[dict[str, Any]],
    h_report: dict[str, Any],
    comparator: str,
) -> dict[str, int]:
    candidate_cells = robust_cell_scores(candidate_sites)
    comparator_cells = robust_cell_scores(
        [
            {
                "site_id": site["site_id"],
                "rubric_profile": site["models"][comparator]["rubric_profile"],
            }
            for site in h_report["sites"]
            if comparator in site["models"]
        ]
    )
    result = {"better": 0, "equal": 0, "worse": 0, "unpaired": 0}
    for key, candidate_score in candidate_cells.items():
        comparator_score = comparator_cells.get(key)
        if comparator_score is None:
            result["unpaired"] += 1
        elif candidate_score > comparator_score:
            result["better"] += 1
        elif candidate_score == comparator_score:
            result["equal"] += 1
        else:
            result["worse"] += 1
    return result


def robust_cell_scores(site_profiles: list[dict[str, Any]]) -> dict[tuple[str, str], int]:
    scores = {}
    for site in site_profiles:
        for cell in site["rubric_profile"]["cells"]:
            label = cell["ordinal_label"]
            if cell["forcing_robust"] and label in LABEL_SCORE:
                scores[(site["site_id"], cell["cell_id"])] = LABEL_SCORE[label]
    return scores


def beats_comparator(candidate: dict[str, Any], comparator: dict[str, Any]) -> bool:
    return (
        candidate["robust_fail_count"] < comparator["robust_fail_count"]
        and candidate["robust_ordinal_score"] >= comparator["robust_ordinal_score"]
    )


def merge_counts(target: dict[str, int], source: dict[str, int]) -> None:
    for key, value in source.items():
        target[key] = target.get(key, 0) + int(value)


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# Physics-Bulk Snowbench Adjudication",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Disposition: `{report['summary']['disposition']}`",
        f"- Best variant: `{report['summary']['best_variant']}`",
        f"- Beats openWEPP as-built: `{report['summary']['beats_openwepp_as_built']}`",
        f"- Beats legacy as-built: `{report['summary']['beats_legacy_as_built']}`",
        f"- No site constants: `{report['no_site_constants']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        "",
        "## Comparator Summary",
        "",
        "| Model | Robust fail | Robust score | Density fail | Density score | Robust counts |",
        "|---|---:|---:|---:|---:|---|",
    ]
    for model_id in ("openwepp_as_built", "legacy_as_built", "pysnobal"):
        model = report["comparators"][model_id]
        lines.append(summary_row(model_id, model))
    lines.extend(["", "## Candidate Summary", "", "| Variant | Model | Robust fail | Robust score | Density fail | Density score | Robust counts | vs openWEPP | vs legacy |", "|---|---|---:|---:|---:|---:|---|---|---|"])
    for candidate in report["candidates"]:
        density = candidate["density_cell_profile"]
        lines.append(
            "| `{variant}` | `{model}` | {fail} | {score} | {density_fail} | {density_score} | `{counts}` | `{openwepp}` | `{legacy}` |".format(
                variant=candidate["variant"],
                model=candidate["model_id"],
                fail=candidate["robust_fail_count"],
                score=candidate["robust_ordinal_score"],
                density_fail=density["fail_count"],
                density_score=density["ordinal_score"],
                counts=json.dumps(candidate["forcing_robust_counts_by_label"], sort_keys=True),
                openwepp=json.dumps(candidate["cell_comparison"]["openwepp_as_built"], sort_keys=True),
                legacy=json.dumps(candidate["cell_comparison"]["legacy_as_built"], sort_keys=True),
            )
        )
    lines.extend(
        [
            "",
            "Disposition rule: a candidate must reduce robust fail count and preserve or improve robust ordinal score against both openWEPP as-built and legacy as-built. SNOWDENSITY-06 additionally requires the density/densification robust-cell profile to improve without melt retuning. Comparator agreement is flag evidence only under ADR-0017.",
            "",
        ]
    )
    return "\n".join(lines)


def summary_row(model_id: str, model: dict[str, Any]) -> str:
    density = model["density_cell_profile"]
    return "| `{}` | {} | {} | {} | {} | `{}` |".format(
        model_id,
        model["robust_fail_count"],
        model["robust_ordinal_score"],
        density["fail_count"],
        density["ordinal_score"],
        json.dumps(model["forcing_robust_counts_by_label"], sort_keys=True),
    )


if __name__ == "__main__":
    raise SystemExit(main())
