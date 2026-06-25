#!/usr/bin/env python3
"""Classify observed snow/frost comparison reports without tuning physics."""

from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
SCHEMA = "snowfreeze-observed-residual-classification-v1"
MISSING_MODELED_SNOW = "UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reports", nargs="+", type=Path)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    args = parser.parse_args()

    observations_dir = args.observations_dir.resolve()
    classification = classify_reports(observations_dir, [path.resolve() for path in args.reports])
    write_json(args.output_json.resolve(), classification)
    args.output_md.resolve().write_text(render_markdown(classification), encoding="utf-8")
    return 0


def classify_reports(observations_dir: Path, report_paths: list[Path]) -> dict[str, Any]:
    manifest_path = observations_dir / "manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    sites_by_id = {site["site_id"]: site for site in manifest["sites"]}
    site_results = []
    for report_path in sorted(report_paths):
        report = json.loads(report_path.read_text(encoding="utf-8"))
        site_id = report["site_id"]
        site_record = sites_by_id.get(site_id)
        if site_record is None:
            raise ValueError(f"{report_path} site {site_id} is absent from {manifest_path}")
        observations = load_observations(observations_dir / site_record["observation_file"])
        site_results.append(classify_site(report_path, report, site_record, observations))

    return {
        "schema": SCHEMA,
        "measurement_contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047",
        "snow_control_tolerance": "TOL-SNOWFREEZE-009",
        "site_count": len(site_results),
        "defect_attribution_eligible_count": sum(
            1 for result in site_results if result["defect_attribution_eligible"]
        ),
        "openwepp_defective_count": sum(
            1 for result in site_results if result["primary_classification"] == "OPENWEPP-DEFECTIVE"
        ),
        "summary": summarize(site_results),
        "sites": site_results,
    }


def classify_site(
    report_path: Path,
    report: dict[str, Any],
    site_record: dict[str, Any],
    observations: list[dict[str, str]],
) -> dict[str, Any]:
    metrics = report.get("metrics") or {}
    method_counts = count_values(row["method"] for row in observations)
    observed_snow_rows = sum(1 for row in observations if row.get("observed_snow_depth_m", ""))
    snow_status = report.get("snow_control_status", "ABSENT")
    has_modeled_snow_depth = snow_status != MISSING_MODELED_SNOW
    verdict = report.get("verdict")
    matched_count = int(metrics.get("matched_count") or 0)

    if verdict == "SOURCE-BLOCKED":
        primary = "SOURCE-BLOCKED"
        residual_family = "source-blocked"
        reason = "Normalized observations are unavailable for this site."
    elif verdict == "HARNESS-SURFACE-MISMATCH" or matched_count == 0:
        primary = "HARNESS-SURFACE-MISMATCH"
        residual_family = "harness-surface-mismatch"
        reason = "The harness did not produce matched modeled/observed rows."
    elif not has_modeled_snow_depth and observed_snow_rows > 0:
        primary = "SNOW-CONTROL-BLOCKED"
        residual_family = "snow-confounded"
        reason = (
            "Observed snow depth exists but modeled snow depth is absent; "
            "TOL-SNOWFREEZE-009 cannot be evaluated."
        )
    elif not has_modeled_snow_depth:
        primary = "INCONCLUSIVE"
        residual_family = "snow-control-missing"
        reason = (
            "No modeled snow-depth diagnostic is available, and this source "
            "does not provide paired snow-depth rows."
        )
    else:
        primary, residual_family, reason = classify_after_snow_gate(metrics)

    defect_eligible = primary in {
        "HEAT-FLOW-THERMAL-CANDIDATE",
        "LOWER-BOUNDARY-QDRY-CANDIDATE",
        "FROZEN-K-INFILTRATION-CANDIDATE",
        "MIGRATION-FRINGE-CANDIDATE",
    }

    return {
        "site_id": report["site_id"],
        "source_id": site_record.get("source_id"),
        "fixture": site_record.get("fixture"),
        "report_path": str(report_path.relative_to(REPO_ROOT)),
        "runtime": report.get("runtime"),
        "harness_verdict": verdict,
        "primary_classification": primary,
        "residual_family": residual_family,
        "reason": reason,
        "defect_attribution_eligible": defect_eligible,
        "snow_control_status": snow_status,
        "has_modeled_snow_depth": has_modeled_snow_depth,
        "observed_snow_depth_row_count": observed_snow_rows,
        "method_counts": method_counts,
        "metrics": {
            "observation_count": metrics.get("observation_count"),
            "matched_count": metrics.get("matched_count"),
            "frost_depth_residual_count": metrics.get("frost_depth_residual_count"),
            "max_abs_residual_m": metrics.get("max_abs_residual_m"),
            "mean_abs_residual_m": metrics.get("mean_abs_residual_m"),
            "isotherm_upper_bound_count": metrics.get("isotherm_upper_bound_count"),
            "isotherm_upper_bound_exceedance_count": metrics.get(
                "isotherm_upper_bound_exceedance_count"
            ),
            "max_isotherm_upper_bound_margin_m": metrics.get(
                "max_isotherm_upper_bound_margin_m"
            ),
        },
    }


def classify_after_snow_gate(metrics: dict[str, Any]) -> tuple[str, str, str]:
    if int(metrics.get("isotherm_upper_bound_exceedance_count") or 0) > 0:
        return (
            "HEAT-FLOW-THERMAL-CANDIDATE",
            "heat-flow/thermal-property shaped",
            "Modeled frost exceeds a soil-temperature isotherm upper-bound after snow control.",
        )
    if int(metrics.get("frost_depth_residual_count") or 0) > 0:
        return (
            "RESIDUAL-CLASSIFIED-AFTER-SNOW-GATE",
            "requires SNOWFROST-FIDELITY-B/C/D discrimination",
            "Frost-tube residuals remain after snow control; benchmarks must discriminate mechanism.",
        )
    return (
        "PASS",
        "no actionable residual",
        "No residual family is exposed by the matched rows.",
    )


def summarize(site_results: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "primary_classification_counts": count_values(
            result["primary_classification"] for result in site_results
        ),
        "residual_family_counts": count_values(result["residual_family"] for result in site_results),
        "snow_control_blocked_sites": [
            result["site_id"]
            for result in site_results
            if result["primary_classification"] == "SNOW-CONTROL-BLOCKED"
        ],
        "inconclusive_sites": [
            result["site_id"]
            for result in site_results
            if result["primary_classification"] == "INCONCLUSIVE"
        ],
        "next_action": (
            "Add a modeled snow-depth diagnostic and rerun SNOWFROST-FIDELITY-A "
            "classification before field residuals are attributed to frost physics. "
            "No Qwet, SFCC, frozen-K, or heat-flow tuning is authorized by these "
            "classifications."
        ),
    }


def load_observations(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def count_values(values: Any) -> dict[str, int]:
    counts: dict[str, int] = {}
    for value in values:
        counts[str(value)] = counts.get(str(value), 0) + 1
    return dict(sorted(counts.items()))


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def render_markdown(classification: dict[str, Any]) -> str:
    lines = [
        "# Residual Classification",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{classification['schema']}`",
        f"- Measurement contract: `{classification['measurement_contract']}`",
        f"- Snow-control tolerance: `{classification['snow_control_tolerance']}`",
        f"- Site count: `{classification['site_count']}`",
        f"- Defect-attribution eligible sites: `{classification['defect_attribution_eligible_count']}`",
        f"- `OPENWEPP-DEFECTIVE` sites: `{classification['openwepp_defective_count']}`",
        "",
        "## Summary",
        "",
        f"- Primary classifications: `{classification['summary']['primary_classification_counts']}`",
        f"- Residual families: `{classification['summary']['residual_family_counts']}`",
        f"- Next action: {classification['summary']['next_action']}",
        "",
        "## Site Classifications",
        "",
        "| Site | Harness | Primary | Family | Matched | Frost residuals | Max abs residual m | Isotherm exceedances | Snow rows | Reason |",
        "| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |",
    ]
    for site in classification["sites"]:
        metrics = site["metrics"]
        lines.append(
            "| {site_id} | {harness} | {primary} | {family} | {matched} | {frost_rows} | {max_abs} | {iso_exceed} | {snow_rows} | {reason} |".format(
                site_id=site["site_id"],
                harness=site["harness_verdict"],
                primary=site["primary_classification"],
                family=site["residual_family"],
                matched=fmt(metrics["matched_count"]),
                frost_rows=fmt(metrics["frost_depth_residual_count"]),
                max_abs=fmt(metrics["max_abs_residual_m"]),
                iso_exceed=fmt(metrics["isotherm_upper_bound_exceedance_count"]),
                snow_rows=site["observed_snow_depth_row_count"],
                reason=site["reason"].replace("|", "\\|"),
            )
        )
    lines.extend(
        [
            "",
            "## Disposition",
            "",
            "No site is eligible for frost-model defect attribution in this pass. "
            "The direct harness produces metric-bearing reports, but modeled "
            "snow depth is absent, so `TOL-SNOWFREEZE-009` cannot be evaluated. "
            "Current field residuals are evidence for the next diagnostic gate, "
            "not authority to tune heat flow, frozen conductivity, or migration heat.",
            "",
        ]
    )
    return "\n".join(lines)


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    return str(value)


if __name__ == "__main__":
    raise SystemExit(main())
