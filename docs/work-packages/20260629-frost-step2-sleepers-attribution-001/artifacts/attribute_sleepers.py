#!/usr/bin/env python3
"""Attribute Step 2 Sleepers frost residuals under INV-SNOWFREEZE-050."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[4]
STEP1 = REPO_ROOT / "docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001"
OBS = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
IN_SCOPE = {
    "site1_sleepers_south_field_vt",
    "site2_sleepers_w9_hardwood_vt",
}
TOLERANCE_DAYS = 14


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    args = parser.parse_args()

    payload = analyze()
    args.output_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.output_md.write_text(render_markdown(payload), encoding="utf-8")
    return 0


def analyze() -> dict[str, Any]:
    routing = json.loads((STEP1 / "artifacts/current_snow_control_routing.json").read_text())
    routing_by_site = {site["site_id"]: site for site in routing["sites"]}
    site_reports = []
    for site_id in sorted(IN_SCOPE):
        route = routing_by_site.get(site_id)
        if route is None or route["route"] != "FORCING-LIMITED":
            raise SystemExit(f"{site_id} is not Step 1 FORCING-LIMITED")
        report_path = STEP1 / "artifacts/site_reports" / f"{site_id}.comparison_report.json"
        report = json.loads(report_path.read_text(encoding="utf-8"))
        site_reports.append(analyze_site(report_path, report, route))

    candidate_sites = [
        site["site_id"] for site in site_reports if site["candidate_defect_count"] > 0
    ]
    timing_agree_sites = [
        site["site_id"] for site in site_reports if site["candidate_defect_count"] == 0
    ]
    return {
        "schema": "frost-step2-sleepers-attribution-v1",
        "evidence_mode": "Ran",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047/048/050",
        "timing_tolerance_days": TOLERANCE_DAYS,
        "step1_systematic_timing_fraction_cutoff_note": (
            "Step 1 used a package-local >0.25 systematic-timing-fraction cutoff; "
            "only the 14-day timing tolerance is inherited here."
        ),
        "scope": {
            "included_sites": sorted(IN_SCOPE),
            "excluded_sites": {
                "site4_ggd498_morris_mn": "Step 1 BLOCKED",
                "site3_scan_mandan_nd": "Step 1 INCONCLUSIVE-NO-PAIRED-SNOW",
                "site5_reynolds_creek_us_rls_id": "Step 1 INCONCLUSIVE-NO-PAIRED-SNOW",
            },
        },
        "summary": {
            "candidate_defect_sites": candidate_sites,
            "timing_agreement_sites": timing_agree_sites,
            "gap_snowfreeze_002": (
                "open_narrowed_to_sleepers_timing_candidate_defects"
                if candidate_sites
                else "open_no_sleepers_timing_defects"
            ),
            "production_changes": False,
        },
        "sites": site_reports,
    }


def analyze_site(path: Path, report: dict[str, Any], route: dict[str, Any]) -> dict[str, Any]:
    metrics = report["metrics"]
    timing_rows = [classify_timing_row(row) for row in metrics["seasonal_metrics"]]
    candidate_defects = [
        item
        for row in timing_rows
        for item in row["signatures"]
        if item["attribution"] == "candidate-frost-model-defect"
    ]
    forcing_attributable = [
        item
        for row in timing_rows
        for item in row["signatures"]
        if item["attribution"] == "forcing-attributable"
    ]
    timing_pass = [
        item for row in timing_rows for item in row["signatures"] if item["verdict"] == "PASS"
    ]
    magnitude = magnitude_summary(report)
    return {
        "site_id": report["site_id"],
        "report_path": str(path.relative_to(REPO_ROOT)),
        "step1_route": route["route"],
        "step1_snow_direction": "modeled-over-observed",
        "step1_snow_mean_signed_depth_residual_m": route["mean_signed_snow_depth_residual_m"],
        "step1_snow_over_rows": route["modeled_over_observed_rows"],
        "step1_snow_under_rows": route["modeled_under_observed_rows"],
        "timing_summary": {
            "signature_count": sum(len(row["signatures"]) for row in timing_rows),
            "pass_count": len(timing_pass),
            "forcing_attributable_failure_count": len(forcing_attributable),
            "candidate_defect_count": len(candidate_defects),
        },
        "candidate_defect_count": len(candidate_defects),
        "candidate_defects": candidate_defects,
        "magnitude": magnitude,
        "timing_rows": timing_rows,
        "site_disposition": site_disposition(report["site_id"], candidate_defects, magnitude),
    }


def classify_timing_row(row: dict[str, Any]) -> dict[str, Any]:
    signatures = [
        classify_signature("onset", row.get("onset_residual_days")),
        classify_signature("thaw", row.get("thaw_residual_days")),
        classify_signature("frozen_duration", row.get("frozen_duration_residual_observation_days")),
    ]
    for signature in signatures:
        signature["water_year"] = row["water_year"]
    return {
        "water_year": row["water_year"],
        "observed_onset_date": row.get("observed_onset_date"),
        "modeled_onset_date": row.get("modeled_onset_date"),
        "observed_thaw_date": row.get("observed_thaw_date"),
        "modeled_thaw_date": row.get("modeled_thaw_date"),
        "signatures": signatures,
    }


def classify_signature(name: str, residual: int | None) -> dict[str, Any]:
    if residual is None:
        return {
            "signature": name,
            "residual": None,
            "verdict": "NO-DATA",
            "attribution": "not-attributed",
            "step3_pointer": None,
            "reason": "No comparable date pair for this signature.",
        }
    if abs(residual) <= TOLERANCE_DAYS:
        return {
            "signature": name,
            "residual": residual,
            "verdict": "PASS",
            "attribution": "agrees-within-tolerance",
            "step3_pointer": None,
            "reason": "Within +/-14 days.",
        }

    if snow_overprediction_explains(name, residual):
        return {
            "signature": name,
            "residual": residual,
            "verdict": "FAIL-FORCING-ATTRIBUTABLE",
            "attribution": "forcing-attributable",
            "step3_pointer": None,
            "reason": (
                "Residual direction matches deeper modeled snow: later onset, "
                "earlier thaw, shorter duration, or shallower frost."
            ),
        }

    return {
        "signature": name,
        "residual": residual,
        "verdict": "FAIL-CANDIDATE-DEFECT",
        "attribution": "candidate-frost-model-defect",
        "step3_pointer": step3_pointer(name, residual),
        "reason": "Residual direction is not explained by deeper modeled snow.",
    }


def snow_overprediction_explains(name: str, residual: int) -> bool:
    if name == "onset":
        return residual > 0
    if name == "thaw":
        return residual < 0
    if name == "frozen_duration":
        return residual < 0
    raise ValueError(name)


def step3_pointer(name: str, residual: int) -> str:
    if name == "onset" and residual < 0:
        return "residue-lifecycle handoff / static-vs-dynamic resdep"
    if name == "thaw" and residual > 0:
        return "residue-lifecycle handoff; compare against legacy-envelope outlier flag"
    if name == "frozen_duration" and residual > 0:
        return "residue-lifecycle handoff; compare against legacy-envelope outlier flag"
    return "absent Qwet evaporative term as secondary energy-balance check"


def magnitude_summary(report: dict[str, Any]) -> dict[str, Any]:
    metrics = report["metrics"]
    full_residuals = reconstruct_frost_residuals(report)
    signed = [row["residual_m"] for row in full_residuals]
    positive = sum(1 for value in signed if value > 0.0)
    negative = sum(1 for value in signed if value < 0.0)
    mean_signed = sum(signed) / len(signed) if signed else None
    seasonal_max_observed = metrics.get("seasonal_max_observed_m")
    seasonal_max_modeled = metrics.get("seasonal_max_modeled_on_observation_dates_m")
    seasonal_max_signed = (
        seasonal_max_modeled - seasonal_max_observed
        if seasonal_max_observed is not None and seasonal_max_modeled is not None
        else None
    )
    if (
        seasonal_max_signed is not None
        and seasonal_max_signed < 0.0
        and mean_signed is not None
        and mean_signed > 0.0
    ):
        tag = "FORCING-LIMITED-MIXED-SIGN"
        reason = (
            "Seasonal maximum modeled frost is shallower, but the full residual "
            "distribution has positive central tendency; magnitude remains "
            "forcing-limited and non-verdict-bearing."
        )
    elif seasonal_max_signed is not None and seasonal_max_signed < 0.0:
        tag = "FORCING-ATTRIBUTABLE"
        reason = "Seasonal maximum modeled frost is shallower, coherent with deeper modeled snow."
    elif seasonal_max_signed is not None and seasonal_max_signed > 0.0:
        tag = "FORCING-LIMITED-SIGN-INCOHERENT"
        reason = (
            "Seasonal maximum modeled frost is deeper despite deeper modeled snow; "
            "reported as a forcing-limited magnitude signal, not a verdict-bearing defect."
        )
    else:
        tag = "FORCING-LIMITED"
        reason = "Magnitude has no signed seasonal-max contrast."
    return {
        "tag": tag,
        "reason": reason,
        "residual_count": len(signed),
        "positive_residual_count": positive,
        "negative_residual_count": negative,
        "mean_signed_residual_m": mean_signed,
        "median_signed_residual_m": median(signed),
        "max_abs_residual_m": metrics.get("max_abs_residual_m"),
        "mean_abs_residual_m": metrics.get("mean_abs_residual_m"),
        "seasonal_max_observed_m": seasonal_max_observed,
        "seasonal_max_modeled_m": seasonal_max_modeled,
        "seasonal_max_signed_residual_m": seasonal_max_signed,
        "step3_pointer": (
            "legacy-envelope outlier flag plus residue-lifecycle handoff check"
            if tag in {"FORCING-LIMITED-SIGN-INCOHERENT", "FORCING-LIMITED-MIXED-SIGN"}
            else None
        ),
    }


def reconstruct_frost_residuals(report: dict[str, Any]) -> list[dict[str, Any]]:
    try:
        import pyarrow.parquet as pq
    except ImportError as error:
        raise RuntimeError("pyarrow is required; run with .venv/bin/python") from error

    wat_path = Path(report["wat_output"])
    if not wat_path.is_file():
        raise FileNotFoundError(wat_path)
    modeled = {}
    table = pq.read_table(wat_path)
    columns = table.to_pydict()
    for water_year, month, day, frdp_mm in zip(
        columns["water_year"], columns["month"], columns["day_of_month"], columns["frdp"]
    ):
        calendar_year = int(water_year) - 1 if int(month) >= 10 else int(water_year)
        modeled[dt.date(calendar_year, int(month), int(day))] = float(frdp_mm) / 1000.0

    obs_path = observation_path(report["site_id"])
    residuals = []
    with obs_path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            if row["method"] != "frost_tube" or row["censoring"] != "none":
                continue
            observed = parse_optional(row["observed_frost_depth_m"])
            if observed is None:
                continue
            date = dt.date.fromisoformat(row["date"])
            modeled_value = modeled.get(date)
            if modeled_value is None:
                continue
            residuals.append(
                {
                    "date": date.isoformat(),
                    "observed_m": observed,
                    "modeled_m": modeled_value,
                    "residual_m": modeled_value - observed,
                }
            )
    return residuals


def observation_path(site_id: str) -> Path:
    manifest = json.loads((OBS / "manifest.json").read_text(encoding="utf-8"))
    for site in manifest["sites"]:
        if site["site_id"] == site_id:
            return OBS / site["observation_file"]
    raise KeyError(site_id)


def parse_optional(value: str) -> float | None:
    if not value:
        return None
    parsed = float(value)
    return parsed if math.isfinite(parsed) else None


def median(values: list[float]) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    mid = len(ordered) // 2
    if len(ordered) % 2 == 1:
        return ordered[mid]
    return (ordered[mid - 1] + ordered[mid]) / 2.0


def site_disposition(
    site_id: str, candidate_defects: list[dict[str, Any]], magnitude: dict[str, Any]
) -> str:
    if candidate_defects:
        pointers = sorted({item["step3_pointer"] for item in candidate_defects})
        return (
            f"{site_id}: frost timing does not fully agree; "
            f"{len(candidate_defects)} timing cells are candidate frost-model defects. "
            f"Step 3 pointers: {', '.join(pointers)}."
        )
    return (
        f"{site_id}: frost timing agrees or is forcing-attributable; "
        f"magnitude remains {magnitude['tag']}."
    )


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# FROST STEP 2 Sleepers Attribution",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{payload['schema']}`",
        f"- Contract: `{payload['contract']}`",
        f"- Timing tolerance: `+/-{payload['timing_tolerance_days']} days`",
        f"- Candidate-defect sites: `{payload['summary']['candidate_defect_sites']}`",
        f"- GAP-SNOWFREEZE-002 disposition: `{payload['summary']['gap_snowfreeze_002']}`",
        "",
        "## Scope",
        "",
        f"- Included: `{payload['scope']['included_sites']}`",
        f"- Excluded: `{payload['scope']['excluded_sites']}`",
        "",
    ]
    for site in payload["sites"]:
        lines.extend(render_site(site))
    lines.extend(
        [
            "## Step 4 Note",
            "",
            payload["step1_systematic_timing_fraction_cutoff_note"],
            "If `INV-SNOWFREEZE-048/050` ratification inherits that cutoff, it must be adjudicated deliberately.",
            "",
        ]
    )
    return "\n".join(lines)


def render_site(site: dict[str, Any]) -> list[str]:
    lines = [
        f"## {site['site_id']}",
        "",
        f"- Step 1 route: `{site['step1_route']}`",
        f"- Step 1 snow direction: `{site['step1_snow_direction']}`",
        f"- Candidate timing defects: `{site['candidate_defect_count']}`",
        f"- Site disposition: {site['site_disposition']}",
        "",
        "### Timing Signatures",
        "",
        "| WY | Onset | Thaw | Duration |",
        "| ---: | --- | --- | --- |",
    ]
    for row in site["timing_rows"]:
        by_name = {item["signature"]: item for item in row["signatures"]}
        lines.append(
            "| {wy} | {onset} | {thaw} | {duration} |".format(
                wy=row["water_year"],
                onset=fmt_signature(by_name["onset"]),
                thaw=fmt_signature(by_name["thaw"]),
                duration=fmt_signature(by_name["frozen_duration"]),
            )
        )
    magnitude = site["magnitude"]
    lines.extend(
        [
            "",
            "### Magnitude",
            "",
            f"- Tag: `{magnitude['tag']}`",
            f"- Reason: {magnitude['reason']}",
            f"- Residual count: `{magnitude['residual_count']}`",
            f"- Mean signed residual m: `{fmt_number(magnitude['mean_signed_residual_m'])}`",
            f"- Median signed residual m: `{fmt_number(magnitude['median_signed_residual_m'])}`",
            f"- Max absolute residual m: `{fmt_number(magnitude['max_abs_residual_m'])}`",
            f"- Seasonal max signed residual m: `{fmt_number(magnitude['seasonal_max_signed_residual_m'])}`",
            "",
        ]
    )
    if site["candidate_defects"]:
        lines.extend(["### Step 3 Pointers", ""])
        for item in site["candidate_defects"]:
            lines.append(
                f"- WY `{item['water_year']}` `{item['signature']}` residual `{item['residual']}`: {item['step3_pointer']}."
            )
        lines.append("")
    return lines


def fmt_signature(item: dict[str, Any]) -> str:
    residual = item["residual"]
    residual_text = "n/a" if residual is None else str(residual)
    pointer = f"; {item['step3_pointer']}" if item.get("step3_pointer") else ""
    return f"`{residual_text}` {item['verdict']} ({item['attribution']}{pointer})"


def fmt_number(value: Any) -> str:
    if value is None:
        return "n/a"
    return f"{float(value):.6g}"


if __name__ == "__main__":
    raise SystemExit(main())
