#!/usr/bin/env python3
"""Audit modeled snow-depth fidelity before frost-depth attribution."""

from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OBSERVATIONS = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
SCHEMA = "snowfreeze-observed-snow-depth-audit-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-048"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reports", nargs="+", type=Path)
    parser.add_argument("--observations-dir", type=Path, default=DEFAULT_OBSERVATIONS)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    args = parser.parse_args()

    audit = audit_reports(
        args.observations_dir.resolve(), [path.resolve() for path in args.reports]
    )
    write_json(args.output_json.resolve(), audit)
    args.output_md.resolve().write_text(render_markdown(audit), encoding="utf-8")
    return 0


def audit_reports(observations_dir: Path, report_paths: list[Path]) -> dict[str, Any]:
    manifest_path = observations_dir / "manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    sites_by_id = {site["site_id"]: site for site in manifest["sites"]}
    provenance_by_id = {
        source["source_id"]: source for source in manifest.get("sources", [])
    }

    sites = []
    for report_path in sorted(report_paths):
        report = json.loads(report_path.read_text(encoding="utf-8"))
        site_id = report["site_id"]
        site_record = sites_by_id.get(site_id)
        if site_record is None:
            raise ValueError(f"{report_path} site {site_id} is absent from {manifest_path}")
        observations = load_observations(observations_dir / site_record["observation_file"])
        provenance = provenance_by_id.get(site_record.get("source_id"), {})
        sites.append(audit_site(report_path, report, site_record, provenance, observations))

    return {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "site_count": len(sites),
        "summary": summarize(sites),
        "sites": sites,
    }


def audit_site(
    report_path: Path,
    report: dict[str, Any],
    site_record: dict[str, Any],
    provenance: dict[str, Any],
    observations: list[dict[str, str]],
) -> dict[str, Any]:
    metrics = report.get("metrics") or {}
    residuals = metrics.get("snow_depth_residuals") or []
    observed_snow_rows = sum(1 for row in observations if row.get("observed_snow_depth_m", ""))
    source_semantics = str(provenance.get("parser_assumptions") or "")
    source_depth_semantics_ok = source_mentions_physical_snow_depth(source_semantics)
    modeled_source = str(report.get("modeled_snow_depth_source") or "")
    modeled_depth_lineage_ok = (
        "Snow-Depth" in modeled_source and "snow.runtime_depth_m" in modeled_source
    )
    timing_rescue_count = int(metrics.get("snow_depth_best_offset_rescue_count") or 0)
    failed_count = int(metrics.get("snow_depth_control_fail_count") or 0)
    paired_count = int(metrics.get("snow_depth_control_count") or 0)
    alias_better_count = int(metrics.get("snow_water_alias_abs_better_count") or 0)
    alias_primary = paired_count > 0 and alias_better_count == paired_count
    timing_primary = failed_count > 0 and timing_rescue_count == failed_count
    over_count = int(metrics.get("snow_depth_modeled_over_observed_count") or 0)
    under_count = int(metrics.get("snow_depth_modeled_under_observed_count") or 0)

    if observed_snow_rows == 0:
        route = "INSUFFICIENT-PAIRED-SNOW-DATA"
        reason = "No observed snow-depth rows are available for this site."
    elif paired_count == 0:
        route = "INSUFFICIENT-MATCHED-SNOW-DATA"
        reason = "Observed snow-depth rows exist, but none match modeled WAT dates."
    elif not source_depth_semantics_ok:
        route = "CORRESPONDENCE-BLOCKED-SOURCE-SEMANTICS"
        reason = "Source provenance does not prove physical snowpack-depth semantics."
    elif not modeled_depth_lineage_ok:
        route = "CORRESPONDENCE-BLOCKED-MODELED-LINEAGE"
        reason = "Modeled snow-depth lineage is not WAT Snow-Depth from snow.runtime_depth_m."
    elif alias_primary:
        route = "CORRESPONDENCE-BLOCKED-SWE-ALIAS-RISK"
        reason = "SWE alias residuals are better on every paired row; depth-vs-SWE must be rechecked."
    elif timing_primary:
        route = "CORRESPONDENCE-BLOCKED-TIMING-STAGE-RISK"
        reason = "Adjacent-day modeled snow depth would rescue every failed same-day row."
    elif failed_count > 0:
        route = "SNOW-DEPTH-FIDELITY-ISSUE"
        reason = (
            "Like-for-like depth evidence is present, aliases/timing do not explain "
            "the failures, and TOL-SNOWFREEZE-009 fails."
        )
    else:
        route = "FROST-READY-SNOW-CONTROL-PASSED"
        reason = "Paired snow-depth control passes for current rows."

    nonzero_direction_count = over_count + under_count
    direction = "mixed"
    if over_count > 0 and under_count == 0:
        direction = "modeled-over-observed"
    elif under_count > 0 and over_count == 0:
        direction = "modeled-under-observed"
    elif nonzero_direction_count > 0 and over_count / nonzero_direction_count >= 0.80:
        direction = "dominant-modeled-over-observed"
    elif nonzero_direction_count > 0 and under_count / nonzero_direction_count >= 0.80:
        direction = "dominant-modeled-under-observed"
    elif over_count == 0 and under_count == 0 and paired_count > 0:
        direction = "zero-residual"
    elif paired_count == 0:
        direction = "no-paired-residuals"

    return {
        "site_id": report["site_id"],
        "source_id": site_record.get("source_id"),
        "report_path": str(report_path.relative_to(REPO_ROOT)),
        "source_parser_assumptions": source_semantics,
        "source_depth_semantics_ok": source_depth_semantics_ok,
        "modeled_depth_lineage_ok": modeled_depth_lineage_ok,
        "route": route,
        "reason": reason,
        "dominant_direction": direction,
        "paired_count": paired_count,
        "observed_snow_depth_row_count": observed_snow_rows,
        "failed_count": failed_count,
        "timing_rescue_count": timing_rescue_count,
        "snow_water_alias_better_count": alias_better_count,
        "metrics": {
            "mean_signed_snow_depth_residual_m": metrics.get(
                "mean_signed_snow_depth_residual_m"
            ),
            "median_signed_snow_depth_residual_m": metrics.get(
                "median_signed_snow_depth_residual_m"
            ),
            "min_signed_snow_depth_residual_m": metrics.get(
                "min_signed_snow_depth_residual_m"
            ),
            "max_signed_snow_depth_residual_m": metrics.get(
                "max_signed_snow_depth_residual_m"
            ),
            "max_abs_snow_depth_residual_m": metrics.get(
                "max_abs_snow_depth_residual_m"
            ),
            "mean_abs_snow_depth_residual_m": metrics.get(
                "mean_abs_snow_depth_residual_m"
            ),
            "snow_depth_modeled_over_observed_count": over_count,
            "snow_depth_modeled_under_observed_count": under_count,
            "snow_depth_control_fail_count": failed_count,
            "snow_depth_best_offset_rescue_count": timing_rescue_count,
            "snow_water_alias_abs_better_count": alias_better_count,
            "max_abs_snow_water_alias_residual_m": metrics.get(
                "max_abs_snow_water_alias_residual_m"
            ),
            "mean_abs_snow_water_alias_residual_m": metrics.get(
                "mean_abs_snow_water_alias_residual_m"
            ),
        },
        "sample_snow_depth_residuals": residuals[:20],
    }


def source_mentions_physical_snow_depth(parser_assumptions: str) -> bool:
    lowered = parser_assumptions.lower()
    return (
        ("snow-depth" in lowered or "snow_cm" in lowered or "snow depth" in lowered)
        and ("cm" in lowered or "centimeter" in lowered)
    )


def summarize(sites: list[dict[str, Any]]) -> dict[str, Any]:
    route_counts = count_values(site["route"] for site in sites)
    direction_counts = count_values(site["dominant_direction"] for site in sites)
    next_route = "snow-depth fidelity"
    if route_counts == {"FROST-READY-SNOW-CONTROL-PASSED": len(sites)}:
        next_route = "frost mechanism adjudication"
    elif any(route.startswith("CORRESPONDENCE-BLOCKED") for route in route_counts):
        next_route = "correspondence blocker resolution"
    elif route_counts.get("SNOW-DEPTH-FIDELITY-ISSUE", 0) == 0:
        next_route = "additional paired snow observations or fixtures"
    return {
        "route_counts": route_counts,
        "direction_counts": direction_counts,
        "next_route": next_route,
        "frost_attribution_authorized": False,
        "qwet_authorized": False,
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


def render_markdown(audit: dict[str, Any]) -> str:
    lines = [
        "# Snow-Depth Fidelity Audit",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{audit['schema']}`",
        f"- Contract: `{audit['contract']}`",
        f"- Site count: `{audit['site_count']}`",
        f"- Route counts: `{audit['summary']['route_counts']}`",
        f"- Direction counts: `{audit['summary']['direction_counts']}`",
        f"- Next route: `{audit['summary']['next_route']}`",
        f"- Frost attribution authorized: `{audit['summary']['frost_attribution_authorized']}`",
        f"- Qwet authorized: `{audit['summary']['qwet_authorized']}`",
        "",
        "## Site Audit",
        "",
        "| Site | Route | Direction | Pairs | Failures | Timing rescues | SWE alias better | Mean signed m | Median signed m | Max abs m | Reason |",
        "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
    ]
    for site in audit["sites"]:
        metrics = site["metrics"]
        lines.append(
            "| {site_id} | {route} | {direction} | {pairs} | {failures} | {timing} | {alias} | {mean_signed} | {median_signed} | {max_abs} | {reason} |".format(
                site_id=site["site_id"],
                route=site["route"],
                direction=site["dominant_direction"],
                pairs=site["paired_count"],
                failures=site["failed_count"],
                timing=site["timing_rescue_count"],
                alias=site["snow_water_alias_better_count"],
                mean_signed=fmt(metrics["mean_signed_snow_depth_residual_m"]),
                median_signed=fmt(metrics["median_signed_snow_depth_residual_m"]),
                max_abs=fmt(metrics["max_abs_snow_depth_residual_m"]),
                reason=site["reason"].replace("|", "\\|"),
            )
        )
    lines.extend(
        [
            "",
            "## Disposition",
            "",
            "Frost heat-flow, frozen-K, SFCC, impedance, and migration/fringe "
            "work remain unauthorized by these field residuals. The next "
            "authorized route is snow-depth fidelity unless correspondence "
            "blockers are reported above.",
            "",
        ]
    )
    return "\n".join(lines)


def fmt(value: Any) -> str:
    return "n/a" if value is None else str(value)


if __name__ == "__main__":
    raise SystemExit(main())
