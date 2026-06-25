#!/usr/bin/env python3
"""Run offline physics_bulk snowbench over the SNOTEL corpus and score the rubric.

This tool is SNOWDENSITY-03 diagnostic evidence.  It does not calibrate per-site
constants and does not couple ``physics_bulk`` into production runtime.
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

import snotel_density_three_way as snotel


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snotel_observed/observations"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity03_physics_bulk"
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"
FIXTURE_ROOT = REPO_ROOT / "tests/fixtures/snotel_observed"
DEFAULT_VARIANT = "candidate_v1"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--variant", default=DEFAULT_VARIANT)
    parser.add_argument("--site", action="append", default=[])
    args = parser.parse_args(argv)

    sites = snotel.selected_sites(set(args.site))
    report = run_profile(
        observations_dir=args.observations_dir.resolve(),
        output_dir=args.output_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        variant=args.variant,
        sites=sites,
    )
    snotel.write_json(args.output_dir / "physics_bulk_snotel_profile.json", report)
    (args.output_dir / "physics_bulk_snotel_profile.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return 0


def run_profile(
    observations_dir: Path,
    output_dir: Path,
    snowbench_binary: Path,
    variant: str,
    sites: list[snotel.SnotelSite],
) -> dict[str, Any]:
    snotel.validate_observations(observations_dir, sites)
    if not snowbench_binary.is_file():
        raise FileNotFoundError(f"openwepp-snowbench binary not found: {snowbench_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    site_reports = []
    for site in sites:
        model_dir = output_dir / "runs" / site.site_id
        run_snowbench(site, model_dir, snowbench_binary, variant)
        observations = snotel.read_csv_dicts(observations_dir / "sites" / f"{site.site_id}.csv")
        modeled = load_physics_bulk_series(model_dir / "physics_bulk_snow.csv")
        summary = read_json(model_dir / "physics_bulk_summary.json")
        model_id = str(summary["model_id"])
        metrics = snotel.model_metrics(observations, modeled, model_id)
        rubric = snotel.rubric_profile(observations, modeled, model_id)
        site_reports.append(
            {
                "site_id": site.site_id,
                "station_triplet": site.triplet,
                "snow_climate": site.snow_climate,
                "model_id": model_id,
                "variant": summary["variant"],
                "run_dir": str(model_dir),
                "snow_csv": str(model_dir / "physics_bulk_snow.csv"),
                "summary_json": str(model_dir / "physics_bulk_summary.json"),
                "metrics": metrics,
                "rubric_profile": rubric,
            }
        )
    return {
        "schema": "snowdensity03-physics-bulk-snotel-profile-v1",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-051 OBL-SNOWFREEZE-P-026",
        "model_id": site_reports[0]["model_id"] if site_reports else "physics_bulk",
        "variant": variant,
        "runtime_coupling": "none; offline snowbench candidate only",
        "no_site_constants": True,
        "snowbench_binary": str(snowbench_binary),
        "output_dir": str(output_dir),
        "summary": summarize_sites(site_reports),
        "sites": site_reports,
    }


def run_snowbench(
    site: snotel.SnotelSite,
    model_dir: Path,
    snowbench_binary: Path,
    variant: str,
) -> None:
    model_dir.mkdir(parents=True, exist_ok=True)
    command = [
        str(snowbench_binary),
        "physics-bulk",
        "--run-dir",
        str(FIXTURE_ROOT / site.site_id),
        "--output-dir",
        str(model_dir),
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
            f"openwepp-snowbench physics-bulk failed for {site.site_id} with exit code {completed.returncode}"
        )


def load_physics_bulk_series(path: Path) -> dict[dt.date, dict[str, float | None]]:
    with path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        rows = list(reader)
    modeled: dict[dt.date, dict[str, float | None]] = {}
    for row in rows:
        date = dt.date.fromisoformat(row["date"])
        modeled[date] = {
            "snow_water_m": parse_float(row["snow_water_m"]),
            "snow_depth_m": parse_float(row["snow_depth_m"]),
            "snow_density_kg_m3": parse_float(row["snow_density_kg_m3"]),
        }
    return modeled


def read_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def parse_float(value: str) -> float | None:
    if value == "":
        return None
    parsed = float(value)
    if not math.isfinite(parsed):
        return None
    return parsed


def summarize_sites(site_reports: list[dict[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    robust_counts: dict[str, int] = {}
    paired_count = 0
    for report in site_reports:
        paired_count += int(report["metrics"]["paired_count"])
        summary = report["rubric_profile"]["summary"]
        merge_counts(counts, summary["counts_by_label"])
        merge_counts(robust_counts, summary["forcing_robust_counts_by_label"])
    return {
        "site_count": len(site_reports),
        "paired_count": paired_count,
        "counts_by_label": dict(sorted(counts.items())),
        "forcing_robust_counts_by_label": dict(sorted(robust_counts.items())),
        "openwepp_defective_cells": 0,
        "observation_only_failures_are_unresolved": True,
    }


def merge_counts(target: dict[str, int], source: dict[str, int]) -> None:
    for key, value in source.items():
        target[key] = target.get(key, 0) + int(value)


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# SNOWDENSITY-03 Physics-Bulk SNOTEL Profile",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Model: `{report['model_id']}`",
        f"- Variant: `{report['variant']}`",
        f"- Contract: `{report['contract']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        f"- No site constants: `{report['no_site_constants']}`",
        "",
        "| Site | Paired rows | Robust counts | All counts |",
        "|---|---:|---|---|",
    ]
    for site in report["sites"]:
        robust = site["rubric_profile"]["summary"]["forcing_robust_counts_by_label"]
        all_counts = site["rubric_profile"]["summary"]["counts_by_label"]
        lines.append(
            f"| `{site['site_id']}` | {site['metrics']['paired_count']} | `{json.dumps(robust, sort_keys=True)}` | `{json.dumps(all_counts, sort_keys=True)}` |"
        )
    lines.extend(
        [
            "",
            "Disposition: profile evidence only. Failures remain `UNRESOLVED` under ADR-0017 until SNOWDENSITY-04 adjudicates whether in-envelope changes improve forcing-robust cells without site tuning.",
            "",
        ]
    )
    return "\n".join(lines)


if __name__ == "__main__":
    raise SystemExit(main())
