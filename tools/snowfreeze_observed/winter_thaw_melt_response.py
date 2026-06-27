#!/usr/bin/env python3
"""Diagnose winter-thaw melt response over observed snow-depth loss windows.

This is SNOWDENSITY-10.3.6 evidence tooling. It reuses the diagnostic-only
``openwepp-snowbench coe-melt --model legacy_coe`` replay from SNOWDENSITY-10.3.4
and compares paired observed snow-depth ablation intervals with modeled snow
depth loss, CoE melt operands, positive-temperature snowpack hours, and warm-rain
heat context. It does not change production physics or tune coefficients.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import maritime_overaccumulation_diagnosis as maritime  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowdensity10-3-6-winter-thaw-melt-response-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 INV-SNOWFREEZE-050"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_6_winter_thaw_melt_response"
PACKAGE_ARTIFACTS = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/artifacts"
)
DEFAULT_SNOWBENCH_BINARY = REPO_ROOT / "target/debug/openwepp-snowbench"

MAX_INTERVAL_DAYS = 45
OBSERVED_ABLATION_THRESHOLD_M = 0.05
DEFICIT_FRACTION_THRESHOLD = 0.30


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument("--snowbench-binary", type=Path, default=DEFAULT_SNOWBENCH_BINARY)
    parser.add_argument("--skip-runs", action="store_true")
    args = parser.parse_args(argv)

    report = diagnose(
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        snowbench_binary=args.snowbench_binary.resolve(),
        run_snowbench=not args.skip_runs,
    )
    print(
        json.dumps(
            {
                "schema": report["schema"],
                "disposition": report["summary"]["disposition"],
                "paired_surface_count": report["summary"]["paired_surface_count"],
                "under_ablation_interval_count": report["summary"][
                    "under_ablation_interval_count"
                ],
                "next_route": report["summary"]["next_route"],
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


def diagnose(
    output_dir: Path,
    package_artifacts_dir: Path,
    snowbench_binary: Path,
    run_snowbench: bool,
) -> dict[str, Any]:
    if run_snowbench and not snowbench_binary.is_file():
        raise FileNotFoundError(f"openwepp-snowbench binary not found: {snowbench_binary}")
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    surfaces = []
    for surface in maritime.SURFACES:
        run_dir = output_dir / "runs" / surface.surface_id
        if run_snowbench:
            maritime.run_coe_melt(surface.fixture_dir, run_dir, snowbench_binary)
        surfaces.append(analyze_surface(surface, run_dir))

    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "runtime_coupling": "diagnostic snowbench replay only; legacy_coe; no production activation",
        "rank_source": "SNOWDENSITY-10.3.4 rank-2 winter_thaw_melt_response",
        "evidence_mode": "Static/Ran",
        "no_physics_change": True,
        "no_tuning": True,
        "default_activation_changed": False,
        "parser_runfile_user_cli_selector_added": False,
        "fixture_inputs_changed": False,
        "public_output_schema_changed": False,
        "snowbench_binary": str(snowbench_binary),
        "output_dir": str(output_dir),
        "summary": summarize(surfaces),
        "surfaces": surfaces,
        "static_scope_scan": static_scope_scan_record(),
    }
    rubric.write_json(output_dir / "winter-thaw-melt-response.json", report)
    (output_dir / "winter-thaw-melt-response.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    rubric.write_json(package_artifacts_dir / "winter-thaw-melt-response.json", report)
    (package_artifacts_dir / "winter-thaw-melt-response.md").write_text(
        render_markdown(report),
        encoding="utf-8",
    )
    return report


def analyze_surface(surface: maritime.Surface, run_dir: Path) -> dict[str, Any]:
    coe_rows = maritime.read_coe_rows(run_dir / "coe_melt_snow.csv")
    forcing_rows = maritime.read_forcing_rows(run_dir / maritime.FORCING_RELATIVE_PATH)
    observations = maritime.load_observations(surface)
    pairs = maritime.pair_observations(observations, coe_rows, surface.observation_kind)
    intervals = build_intervals(pairs, forcing_rows, coe_rows)
    thaw_summary = surface_thaw_summary(forcing_rows, coe_rows)
    interval_summary = summarize_intervals(intervals)
    verdict_scope = (
        "OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY"
        if surface.verdict_scope == "observation_blocked"
        else "PAIRED-OBSERVATION-EVENT-WINDOW"
    )
    return {
        "surface_id": surface.surface_id,
        "site_group": surface.site_group,
        "cover": surface.cover,
        "fixture_dir": str(surface.fixture_dir.relative_to(REPO_ROOT)),
        "verdict_scope": verdict_scope,
        "observation_source": surface.observation_source,
        "observation_kind": surface.observation_kind,
        "observation_row_count": len(observations),
        "paired_row_count": len(pairs),
        "event_window_count": len(intervals),
        "note": surface.note,
        "snowbench_run_dir": str(run_dir),
        "thaw_summary": thaw_summary,
        "event_summary": interval_summary,
        "sample_under_ablation_intervals": top_under_ablation_intervals(intervals),
    }


def build_intervals(
    pairs: list[dict[str, Any]],
    forcing_rows: list[dict[str, Any]],
    coe_rows: dict[dt.date, dict[str, float | None]],
) -> list[dict[str, Any]]:
    sorted_pairs = sorted(pairs, key=lambda item: item["date_obj"])
    intervals = []
    for previous, current in zip(sorted_pairs, sorted_pairs[1:]):
        start_date = previous["date_obj"]
        end_date = current["date_obj"]
        duration_days = (end_date - start_date).days
        if duration_days <= 0 or duration_days > MAX_INTERVAL_DAYS:
            continue
        observed_loss_m = previous["observed_snow_depth_m"] - current["observed_snow_depth_m"]
        modeled_loss_m = previous["modeled_snow_depth_m"] - current["modeled_snow_depth_m"]
        interval_dates = date_range(start_date + dt.timedelta(days=1), end_date)
        forcing = forcing_window_summary(interval_dates, forcing_rows, coe_rows)
        modeled = modeled_window_summary(interval_dates, coe_rows)
        observed_ablation = observed_loss_m >= OBSERVED_ABLATION_THRESHOLD_M
        thaw_observed_ablation = observed_ablation and forcing["positive_temp_snowpack_hours"] > 0
        depth_loss_deficit_m = observed_loss_m - modeled_loss_m
        deficit_threshold_m = max(
            OBSERVED_ABLATION_THRESHOLD_M,
            DEFICIT_FRACTION_THRESHOLD * observed_loss_m if observed_loss_m > 0.0 else 0.0,
        )
        under_ablation = thaw_observed_ablation and depth_loss_deficit_m > deficit_threshold_m
        intervals.append(
            {
                "start_date": start_date.isoformat(),
                "end_date": end_date.isoformat(),
                "duration_days": duration_days,
                "observed_start_depth_m": previous["observed_snow_depth_m"],
                "observed_end_depth_m": current["observed_snow_depth_m"],
                "modeled_start_depth_m": previous["modeled_snow_depth_m"],
                "modeled_end_depth_m": current["modeled_snow_depth_m"],
                "observed_depth_loss_m": observed_loss_m,
                "modeled_depth_loss_m": modeled_loss_m,
                "depth_loss_deficit_m": depth_loss_deficit_m,
                "deficit_threshold_m": deficit_threshold_m,
                "observed_ablation_interval": observed_ablation,
                "thaw_observed_ablation_interval": thaw_observed_ablation,
                "under_ablation_interval": under_ablation,
                "modeled_loss_to_observed_loss_ratio": (
                    modeled_loss_m / observed_loss_m if observed_loss_m > 0.0 else None
                ),
                "forcing_window": forcing,
                "modeled_window": modeled,
            }
        )
    return intervals


def forcing_window_summary(
    dates: list[dt.date],
    forcing_rows: list[dict[str, Any]],
    coe_rows: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    date_set = set(dates)
    positive_temp_values = []
    snowpack_hours = 0
    positive_temp_snowpack_hours = 0
    rain_on_snow_m = 0.0
    warm_rain_on_snow_m = 0.0
    warm_rain_heat_melt_equiv_m = 0.0
    for row in forcing_rows:
        if row["date"] not in date_set:
            continue
        snowpack_present = maritime.snowpack_present_for_hour(row["date"], coe_rows)
        if snowpack_present:
            snowpack_hours += 1
            if row["temp_air_degC"] > 0.0:
                positive_temp_snowpack_hours += 1
                positive_temp_values.append(row["temp_air_degC"])
        precip_m = row["precip_mass_mm"] / 1000.0
        rain_m = precip_m * (1.0 - row["snow_precip_fraction"])
        if snowpack_present and rain_m > 0.0:
            rain_on_snow_m += rain_m
            if row["temp_air_degC"] > 0.0:
                warm_rain_on_snow_m += rain_m
                warm_rain_heat_melt_equiv_m += (
                    rain_m
                    * row["temp_air_degC"]
                    * maritime.WATER_SPECIFIC_HEAT_J_KG_C
                    / maritime.WATER_LATENT_HEAT_FUSION_J_KG
                )
    return {
        "snowpack_hours": snowpack_hours,
        "positive_temp_snowpack_hours": positive_temp_snowpack_hours,
        "mean_positive_temp_degC": maritime.mean(positive_temp_values),
        "rain_on_snow_m": rain_on_snow_m,
        "warm_rain_on_snow_m": warm_rain_on_snow_m,
        "warm_rain_heat_melt_equiv_m": warm_rain_heat_melt_equiv_m,
    }


def modeled_window_summary(
    dates: list[dt.date],
    coe_rows: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    raw_melt_m = 0.0
    redistributed_melt_m = 0.0
    routed_melt_m = 0.0
    snowpack_swe_loss_m = 0.0
    snowpack_day_count = 0
    for date in dates:
        row = coe_rows.get(date)
        if row is None:
            continue
        if maritime.daily_snowpack_present(date, coe_rows):
            snowpack_day_count += 1
        raw_melt_m += row.get("raw_melt_m") or 0.0
        redistributed_melt_m += row.get("redistributed_melt_m") or 0.0
        routed_melt_m += row.get("routed_melt_m") or 0.0
        snowpack_swe_loss_m += row.get("snowpack_swe_loss_m") or 0.0
    return {
        "snowpack_day_count": snowpack_day_count,
        "raw_melt_m": raw_melt_m,
        "redistributed_melt_m": redistributed_melt_m,
        "routed_melt_m": routed_melt_m,
        "snowpack_swe_loss_m": snowpack_swe_loss_m,
    }


def surface_thaw_summary(
    forcing_rows: list[dict[str, Any]],
    coe_rows: dict[dt.date, dict[str, float | None]],
) -> dict[str, Any]:
    all_dates = sorted(coe_rows)
    forcing = forcing_window_summary(all_dates, forcing_rows, coe_rows)
    modeled = modeled_window_summary(all_dates, coe_rows)
    peak_depth = max((row.get("snow_depth_m") or 0.0 for row in coe_rows.values()), default=0.0)
    return {
        "modeled_snowpack_day_count": modeled["snowpack_day_count"],
        "modeled_peak_snow_depth_m": peak_depth,
        "positive_temp_snowpack_hours": forcing["positive_temp_snowpack_hours"],
        "snowpack_hours": forcing["snowpack_hours"],
        "total_raw_melt_m": modeled["raw_melt_m"],
        "total_routed_melt_m": modeled["routed_melt_m"],
        "total_snowpack_swe_loss_m": modeled["snowpack_swe_loss_m"],
        "warm_rain_heat_melt_equiv_m": forcing["warm_rain_heat_melt_equiv_m"],
    }


def summarize_intervals(intervals: list[dict[str, Any]]) -> dict[str, Any]:
    observed_ablation = [item for item in intervals if item["observed_ablation_interval"]]
    thaw_ablation = [item for item in intervals if item["thaw_observed_ablation_interval"]]
    under_ablation = [item for item in intervals if item["under_ablation_interval"]]
    return {
        "interval_count": len(intervals),
        "observed_ablation_interval_count": len(observed_ablation),
        "thaw_observed_ablation_interval_count": len(thaw_ablation),
        "under_ablation_interval_count": len(under_ablation),
        "under_ablation_fraction": (
            len(under_ablation) / len(thaw_ablation) if thaw_ablation else None
        ),
        "total_observed_depth_loss_m": sum_positive(
            item["observed_depth_loss_m"] for item in thaw_ablation
        ),
        "total_modeled_depth_loss_m": sum(item["modeled_depth_loss_m"] for item in thaw_ablation),
        "total_depth_loss_deficit_m": sum(
            item["depth_loss_deficit_m"] for item in under_ablation
        ),
        "total_positive_temp_snowpack_hours": sum(
            item["forcing_window"]["positive_temp_snowpack_hours"] for item in thaw_ablation
        ),
        "total_raw_melt_m": sum(item["modeled_window"]["raw_melt_m"] for item in thaw_ablation),
        "total_routed_melt_m": sum(
            item["modeled_window"]["routed_melt_m"] for item in thaw_ablation
        ),
        "total_snowpack_swe_loss_m": sum(
            item["modeled_window"]["snowpack_swe_loss_m"] for item in thaw_ablation
        ),
        "warm_rain_heat_melt_equiv_m": sum(
            item["forcing_window"]["warm_rain_heat_melt_equiv_m"] for item in thaw_ablation
        ),
    }


def top_under_ablation_intervals(intervals: list[dict[str, Any]]) -> list[dict[str, Any]]:
    selected = sorted(
        [item for item in intervals if item["under_ablation_interval"]],
        key=lambda item: item["depth_loss_deficit_m"],
        reverse=True,
    )[:8]
    return [
        {
            "start_date": item["start_date"],
            "end_date": item["end_date"],
            "duration_days": item["duration_days"],
            "observed_depth_loss_m": item["observed_depth_loss_m"],
            "modeled_depth_loss_m": item["modeled_depth_loss_m"],
            "depth_loss_deficit_m": item["depth_loss_deficit_m"],
            "positive_temp_snowpack_hours": item["forcing_window"][
                "positive_temp_snowpack_hours"
            ],
            "raw_melt_m": item["modeled_window"]["raw_melt_m"],
            "routed_melt_m": item["modeled_window"]["routed_melt_m"],
            "snowpack_swe_loss_m": item["modeled_window"]["snowpack_swe_loss_m"],
            "warm_rain_heat_melt_equiv_m": item["forcing_window"][
                "warm_rain_heat_melt_equiv_m"
            ],
        }
        for item in selected
    ]


def summarize(surfaces: list[dict[str, Any]]) -> dict[str, Any]:
    paired = [item for item in surfaces if item["paired_row_count"] > 0]
    blocked = [item for item in surfaces if item["paired_row_count"] == 0]
    event_summaries = [item["event_summary"] for item in paired]
    thaw_observed = sum(item["thaw_observed_ablation_interval_count"] for item in event_summaries)
    under = sum(item["under_ablation_interval_count"] for item in event_summaries)
    under_fraction = under / thaw_observed if thaw_observed else None
    disposition = disposition_for(thaw_observed, under, under_fraction)
    next_route = next_route_for(disposition)
    return {
        "disposition": disposition,
        "surface_count": len(surfaces),
        "paired_surface_count": len(paired),
        "observation_blocked_surface_count": len(blocked),
        "observation_blocked_surface_ids": [item["surface_id"] for item in blocked],
        "event_window_count": sum(item["event_window_count"] for item in paired),
        "observed_ablation_interval_count": sum(
            item["observed_ablation_interval_count"] for item in event_summaries
        ),
        "thaw_observed_ablation_interval_count": thaw_observed,
        "under_ablation_interval_count": under,
        "under_ablation_fraction": under_fraction,
        "total_observed_depth_loss_m": sum(
            item["total_observed_depth_loss_m"] for item in event_summaries
        ),
        "total_modeled_depth_loss_m": sum(
            item["total_modeled_depth_loss_m"] for item in event_summaries
        ),
        "total_depth_loss_deficit_m": sum(
            item["total_depth_loss_deficit_m"] for item in event_summaries
        ),
        "total_positive_temp_snowpack_hours": sum(
            item["total_positive_temp_snowpack_hours"] for item in event_summaries
        ),
        "total_raw_melt_m": sum(item["total_raw_melt_m"] for item in event_summaries),
        "total_routed_melt_m": sum(item["total_routed_melt_m"] for item in event_summaries),
        "total_snowpack_swe_loss_m": sum(
            item["total_snowpack_swe_loss_m"] for item in event_summaries
        ),
        "warm_rain_heat_melt_equiv_m": sum(
            item["warm_rain_heat_melt_equiv_m"] for item in event_summaries
        ),
        "next_route": next_route,
    }


def disposition_for(
    thaw_observed_interval_count: int,
    under_ablation_count: int,
    under_ablation_fraction: float | None,
) -> str:
    if thaw_observed_interval_count == 0:
        return "WINTER-THAW-MELT-RESPONSE-HOLD"
    if (
        under_ablation_fraction is not None
        and under_ablation_fraction >= 0.50
        and under_ablation_count >= 5
    ):
        return "WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE"
    if under_ablation_count > 0:
        return "WINTER-THAW-MELT-RESPONSE-PARTIAL"
    return "WINTER-THAW-MELT-RESPONSE-NOT-PRIMARY"


def next_route_for(disposition: str) -> str:
    if disposition == "WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE":
        return (
            "scaffold contract-first opt-in winter-thaw melt-response correction; "
            "preserve rain-heat and longwave as separate later levers"
        )
    if disposition == "WINTER-THAW-MELT-RESPONSE-PARTIAL":
        return (
            "carry event-window deficits into the next candidate package, but check "
            "sub-canopy longwave before coefficient tuning"
        )
    if disposition == "WINTER-THAW-MELT-RESPONSE-NOT-PRIMARY":
        return "proceed to 10.3.4 rank-3 sub-canopy longwave / forest energy"
    return "hold for more paired observed thaw-ablation windows"


def date_range(start_date: dt.date, end_date: dt.date) -> list[dt.date]:
    if end_date < start_date:
        return []
    days = (end_date - start_date).days
    return [start_date + dt.timedelta(days=offset) for offset in range(days + 1)]


def sum_positive(values: Any) -> float:
    return sum(value for value in values if value > 0.0)


def static_scope_scan_record() -> dict[str, Any]:
    return {
        "evidence_class": "Static",
        "production_physics": "No production kernel/runtime code is edited by this diagnostic package.",
        "diagnostic_path": "openwepp-snowbench coe-melt --model legacy_coe",
        "rain_heat": "Warm-rain heat is reported only as context; no dmelt correction is made.",
        "longwave": "Sub-canopy longwave remains a separate later candidate; no correction is made.",
    }


def render_markdown(report: dict[str, Any]) -> str:
    lines = [
        "# SNOWDENSITY-10.3.6 Winter-Thaw Melt Response",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Schema: `{report['schema']}`",
        f"- Contract: `{report['contract']}`",
        f"- Runtime coupling: `{report['runtime_coupling']}`",
        f"- Rank source: {report['rank_source']}",
        f"- No physics change: `{report['no_physics_change']}`",
        f"- No tuning: `{report['no_tuning']}`",
        f"- Default activation changed: `{report['default_activation_changed']}`",
        f"- Public output schema changed: `{report['public_output_schema_changed']}`",
        f"- Disposition: `{report['summary']['disposition']}`",
        f"- Next route: {report['summary']['next_route']}",
        "",
        "## Cohort Summary",
        "",
        "| Metric | Value |",
        "|---|---:|",
    ]
    summary = report["summary"]
    for key in [
        "paired_surface_count",
        "observation_blocked_surface_count",
        "event_window_count",
        "observed_ablation_interval_count",
        "thaw_observed_ablation_interval_count",
        "under_ablation_interval_count",
        "under_ablation_fraction",
        "total_observed_depth_loss_m",
        "total_modeled_depth_loss_m",
        "total_depth_loss_deficit_m",
        "total_positive_temp_snowpack_hours",
        "total_raw_melt_m",
        "total_routed_melt_m",
        "total_snowpack_swe_loss_m",
        "warm_rain_heat_melt_equiv_m",
    ]:
        lines.append(f"| `{key}` | {fmt(summary[key])} |")

    lines.extend(
        [
            "",
            "## Surface Event Windows",
            "",
            "| Surface | Scope | Pairs | Windows | Thaw ablation windows | Under-ablation windows | Under-ablation fraction | Positive-temp snowpack h | Depth-loss deficit m | Warm-rain heat equiv m |",
            "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for item in report["surfaces"]:
        event = item["event_summary"]
        thaw = item["thaw_summary"]
        lines.append(
            "| `{surface}` | `{scope}` | {pairs} | {windows} | {thaw_windows} | {under} | {under_frac} | {hours} | {deficit} | {rain_heat} |".format(
                surface=item["surface_id"],
                scope=item["verdict_scope"],
                pairs=item["paired_row_count"],
                windows=item["event_window_count"],
                thaw_windows=event["thaw_observed_ablation_interval_count"],
                under=event["under_ablation_interval_count"],
                under_frac=fmt(event["under_ablation_fraction"]),
                hours=event["total_positive_temp_snowpack_hours"]
                or thaw["positive_temp_snowpack_hours"],
                deficit=fmt(event["total_depth_loss_deficit_m"]),
                rain_heat=fmt(event["warm_rain_heat_melt_equiv_m"]),
            )
        )

    lines.extend(
        [
            "",
            "## Largest Under-Ablation Intervals",
            "",
            "| Surface | Start | End | Days | Observed loss m | Modeled loss m | Deficit m | Positive-temp h | Raw melt m | SWE loss m |",
            "|---|---|---|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for item in report["surfaces"]:
        for interval in item["sample_under_ablation_intervals"]:
            lines.append(
                "| `{surface}` | {start} | {end} | {days} | {obs} | {model} | {deficit} | {hours} | {raw} | {swe} |".format(
                    surface=item["surface_id"],
                    start=interval["start_date"],
                    end=interval["end_date"],
                    days=interval["duration_days"],
                    obs=fmt(interval["observed_depth_loss_m"]),
                    model=fmt(interval["modeled_depth_loss_m"]),
                    deficit=fmt(interval["depth_loss_deficit_m"]),
                    hours=interval["positive_temp_snowpack_hours"],
                    raw=fmt(interval["raw_melt_m"]),
                    swe=fmt(interval["snowpack_swe_loss_m"]),
                )
            )

    lines.extend(
        [
            "",
            "## Observation-Blocked Surfaces",
            "",
            "| Surface | Positive-temp snowpack h | Reason |",
            "|---|---:|---|",
        ]
    )
    for item in report["surfaces"]:
        if item["paired_row_count"] == 0:
            lines.append(
                f"| `{item['surface_id']}` | {item['thaw_summary']['positive_temp_snowpack_hours']} | {item['note']} |"
            )
    lines.extend(
        [
            "",
            "Conclusion: this package adjudicates winter-thaw melt response using observed "
            "snow-depth loss windows. Warm-rain heat and sub-canopy longwave are reported "
            "as context only and remain separate candidate levers.",
            "",
        ]
    )
    return "\n".join(lines)


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


if __name__ == "__main__":
    raise SystemExit(main())
