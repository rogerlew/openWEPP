#!/usr/bin/env python3
"""Route FROST STEP 1 snow-control reports under INV-SNOWFREEZE-050."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path
from typing import Any


SCHEMA = "frost-step1-current-snow-control-routing-v1"
TOLERANCE_DAYS = 14


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reports-root", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    args = parser.parse_args()

    reports = sorted(args.reports_root.glob("site*/comparison_report.json"))
    if not reports:
        raise SystemExit(f"no comparison reports under {args.reports_root}")

    payload = build_report(reports)
    args.output_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.output_md.write_text(render_markdown(payload), encoding="utf-8")
    return 0


def build_report(report_paths: list[Path]) -> dict[str, Any]:
    sites = [route_site(path, json.loads(path.read_text(encoding="utf-8"))) for path in report_paths]
    return {
        "schema": SCHEMA,
        "evidence_mode": "Ran",
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047/048/050",
        "routing_counts": count_values(site["route"] for site in sites),
        "step2_unblocked_sites": [
            site["site_id"]
            for site in sites
            if site["step2_frost_magnitude_attribution_ready"]
        ],
        "sites": sites,
        "disposition": {
            "gap": "GAP-SNOWFREEZE-002",
            "status": "open_narrowed",
            "summary": (
                "Frost timing attribution is unblocked at paired sites whose only "
                "remaining snow-control failure is forcing-limited absolute "
                "magnitude; frost magnitude attribution is Step 2. Sites without "
                "paired snow depth remain inconclusive for snow control."
            ),
        },
    }


def route_site(path: Path, report: dict[str, Any]) -> dict[str, Any]:
    metrics = report.get("metrics") or {}
    snow_residuals = metrics.get("snow_depth_residuals") or []
    snow_regime = snow_regime_metrics(snow_residuals)
    paired = int(metrics.get("snow_depth_control_count") or 0)
    failures = int(metrics.get("snow_depth_control_fail_count") or 0)
    observed_snow_rows = int(metrics.get("observed_snow_depth_count") or 0)
    timing_rescues = int(metrics.get("snow_depth_best_offset_rescue_count") or 0)
    alias_better = int(metrics.get("snow_water_alias_abs_better_count") or 0)
    over = int(metrics.get("snow_depth_modeled_over_observed_count") or 0)
    under = int(metrics.get("snow_depth_modeled_under_observed_count") or 0)
    seasonal = metrics.get("seasonal_metrics") or []

    robust_timing_failures = count_timing_failures(seasonal)
    forcing_robust_snow_defect = False
    forcing_robust_reasons: list[str] = []

    if paired > 0 and failures > 0 and timing_rescues >= failures:
        forcing_robust_snow_defect = True
        forcing_robust_reasons.append("adjacent-day snow timing/stage explains every failure")
    if paired > 0 and alias_better >= paired:
        forcing_robust_snow_defect = True
        forcing_robust_reasons.append("SWE alias is better than physical depth for every paired row")
    if snow_regime["timing_failure_fraction"] is not None and (
        snow_regime["timing_failure_fraction"] > 0.25
    ):
        forcing_robust_snow_defect = True
        forcing_robust_reasons.append("systematic snow-cover onset/thaw offset exceeds 14 days")
    regime_mismatch_fraction = snow_regime["regime_mismatch_fraction"]
    if regime_mismatch_fraction is not None and regime_mismatch_fraction > 0.25:
        forcing_robust_snow_defect = True
        forcing_robust_reasons.append("snow-cover regime mismatch exceeds 25 percent")
    if robust_timing_failures > 0:
        forcing_robust_reasons.append(
            "frost timing residuals exist and are deferred to Step 2, not snow-control"
        )

    if observed_snow_rows == 0:
        route = "INCONCLUSIVE-NO-PAIRED-SNOW"
        snow_gate_status = "snow control cannot be established"
        attributable = "frost timing report-only until independent snow control exists"
        ready = False
    elif paired == 0:
        route = "INCONCLUSIVE-NO-MATCHED-SNOW"
        snow_gate_status = "observed snow rows exist but no modeled matches"
        attributable = "frost timing report-only until matched snow control exists"
        ready = False
    elif failures == 0:
        route = "PASS"
        snow_gate_status = "paired snow-depth gate clears"
        attributable = "timing and magnitude can move to Step 2 attribution"
        ready = True
    elif forcing_robust_snow_defect:
        route = "BLOCKED"
        snow_gate_status = "forcing-robust snow-control defect remains"
        attributable = "no frost attribution; snow defect would alias into frost"
        ready = False
    else:
        route = "FORCING-LIMITED"
        snow_gate_status = "magnitude-only snow-depth residual"
        attributable = "frost timing attributable; magnitude carries snow forcing uncertainty"
        ready = True

    return {
        "site_id": report["site_id"],
        "comparison_report": str(path),
        "harness_verdict": report.get("verdict"),
        "snow_control_status": report.get("snow_control_status"),
        "route": route,
        "snow_gate_status": snow_gate_status,
        "attributable_frost_signatures": attributable,
        "step2_frost_magnitude_attribution_ready": ready,
        "paired_snow_depth_rows": paired,
        "snow_depth_failures": failures,
        "observed_snow_depth_rows": observed_snow_rows,
        "modeled_over_observed_rows": over,
        "modeled_under_observed_rows": under,
        "timing_rescue_rows": timing_rescues,
        "snow_water_alias_better_rows": alias_better,
        "mean_signed_snow_depth_residual_m": metrics.get("mean_signed_snow_depth_residual_m"),
        "median_signed_snow_depth_residual_m": metrics.get("median_signed_snow_depth_residual_m"),
        "max_abs_snow_depth_residual_m": metrics.get("max_abs_snow_depth_residual_m"),
        "frost_depth_residual_rows": metrics.get("frost_depth_residual_count"),
        "max_abs_frost_depth_residual_m": metrics.get("max_abs_residual_m"),
        "isotherm_upper_bound_rows": metrics.get("isotherm_upper_bound_count"),
        "isotherm_upper_bound_exceedances": metrics.get("isotherm_upper_bound_exceedance_count"),
        "snow_regime_metrics": snow_regime,
        "seasonal_timing_failure_count": robust_timing_failures,
        "forcing_robust_notes": forcing_robust_reasons,
    }


def snow_regime_metrics(snow_residuals: list[dict[str, Any]]) -> dict[str, Any]:
    if not snow_residuals:
        return {
            "paired_rows": 0,
            "cover_agreement_fraction": None,
            "regime_mismatch_fraction": None,
            "modeled_only_cover_rows": 0,
            "observed_only_cover_rows": 0,
            "timing_failure_count": 0,
            "timing_check_count": 0,
            "timing_failure_fraction": None,
            "seasonal_offsets": [],
        }

    agreement = 0
    modeled_only = 0
    observed_only = 0
    by_water_year: dict[int, list[dict[str, Any]]] = {}
    for row in snow_residuals:
        date = row["date"]
        year = int(date[0:4])
        month = int(date[5:7])
        water_year = year + 1 if month >= 10 else year
        by_water_year.setdefault(water_year, []).append(row)

        observed_cover = float(row["observed_snow_depth_m"]) > 0.0
        modeled_cover = float(row["modeled_snow_depth_m"]) > 0.0
        if observed_cover == modeled_cover:
            agreement += 1
        elif modeled_cover:
            modeled_only += 1
        else:
            observed_only += 1

    seasonal_offsets = []
    timing_failures = 0
    timing_checks = 0
    for water_year, rows in sorted(by_water_year.items()):
        observed_dates = [
            row["date"] for row in rows if float(row["observed_snow_depth_m"]) > 0.0
        ]
        modeled_dates = [
            row["date"] for row in rows if float(row["modeled_snow_depth_m"]) > 0.0
        ]
        onset_offset = date_offset_days(
            min(modeled_dates) if modeled_dates else None,
            min(observed_dates) if observed_dates else None,
        )
        thaw_offset = date_offset_days(
            max(modeled_dates) if modeled_dates else None,
            max(observed_dates) if observed_dates else None,
        )
        if onset_offset is not None and abs(onset_offset) > TOLERANCE_DAYS:
            timing_failures += 1
        if onset_offset is not None:
            timing_checks += 1
        if thaw_offset is not None and abs(thaw_offset) > TOLERANCE_DAYS:
            timing_failures += 1
        if thaw_offset is not None:
            timing_checks += 1
        seasonal_offsets.append(
            {
                "water_year": water_year,
                "observed_snow_dates": len(observed_dates),
                "modeled_snow_dates": len(modeled_dates),
                "onset_offset_days": onset_offset,
                "thaw_offset_days": thaw_offset,
            }
        )

    mismatch = modeled_only + observed_only
    paired = len(snow_residuals)
    return {
        "paired_rows": paired,
        "cover_agreement_fraction": agreement / paired,
        "regime_mismatch_fraction": mismatch / paired,
        "modeled_only_cover_rows": modeled_only,
        "observed_only_cover_rows": observed_only,
        "timing_failure_count": timing_failures,
        "timing_check_count": timing_checks,
        "timing_failure_fraction": (
            timing_failures / timing_checks if timing_checks > 0 else None
        ),
        "seasonal_offsets": seasonal_offsets,
    }


def date_offset_days(lhs: str | None, rhs: str | None) -> int | None:
    if lhs is None or rhs is None:
        return None
    lhs_date = date_tuple(lhs)
    rhs_date = date_tuple(rhs)
    return days_from_civil(*lhs_date) - days_from_civil(*rhs_date)


def date_tuple(value: str) -> tuple[int, int, int]:
    return int(value[0:4]), int(value[5:7]), int(value[8:10])


def days_from_civil(year: int, month: int, day: int) -> int:
    if month <= 2:
        year -= 1
    era = year // 400
    yoe = year - era * 400
    doy = (153 * (month + (-3 if month > 2 else 9)) + 2) // 5 + day - 1
    doe = yoe * 365 + yoe // 4 - yoe // 100 + doy
    return era * 146097 + doe


def count_timing_failures(seasonal: list[dict[str, Any]]) -> int:
    failures = 0
    for row in seasonal:
        for key in ("onset_residual_days", "thaw_residual_days"):
            value = row.get(key)
            if value is not None and abs(int(value)) > TOLERANCE_DAYS:
                failures += 1
    return failures


def count_values(values: Any) -> dict[str, int]:
    counts: dict[str, int] = {}
    for value in values:
        counts[str(value)] = counts.get(str(value), 0) + 1
    return dict(sorted(counts.items()))


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# FROST STEP 1 Current-Snow Control Routing",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Schema: `{payload['schema']}`",
        f"- Contract: `{payload['contract']}`",
        f"- Route counts: `{payload['routing_counts']}`",
        f"- Step 2 unblocked sites: `{payload['step2_unblocked_sites']}`",
        "",
        "## Per-Site Routing",
        "",
        "| Site | Route | Snow Gate | Pairs | Failures | Cover Agreement | Snow Timing Fails | Mean signed m | Max abs m | Attributable Frost Signatures |",
        "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
    ]
    for site in payload["sites"]:
        lines.append(
                "| {site_id} | {route} | {gate} | {pairs} | {failures} | {cover} | {timing_fails} | {mean} | {max_abs} | {attrib} |".format(
                site_id=site["site_id"],
                route=site["route"],
                gate=site["snow_gate_status"],
                pairs=site["paired_snow_depth_rows"],
                failures=site["snow_depth_failures"],
                cover=fmt_fraction(site["snow_regime_metrics"]["cover_agreement_fraction"]),
                timing_fails=site["snow_regime_metrics"]["timing_failure_count"],
                mean=fmt(site["mean_signed_snow_depth_residual_m"]),
                max_abs=fmt(site["max_abs_snow_depth_residual_m"]),
                attrib=site["attributable_frost_signatures"],
            )
        )
    lines.extend(
        [
            "",
            "## GAP-SNOWFREEZE-002 Disposition Input",
            "",
            f"- Status: `{payload['disposition']['status']}`",
            f"- Summary: {payload['disposition']['summary']}",
            "",
        ]
    )
    return "\n".join(lines)


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, float):
        if not math.isfinite(value):
            return "n/a"
        return f"{value:.6g}"
    return str(value)


def fmt_fraction(value: Any) -> str:
    if value is None:
        return "n/a"
    return f"{float(value):.3f}"


if __name__ == "__main__":
    raise SystemExit(main())
