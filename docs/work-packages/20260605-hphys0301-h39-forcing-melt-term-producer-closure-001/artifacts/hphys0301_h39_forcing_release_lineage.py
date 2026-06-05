#!/usr/bin/env python3
"""Run HPHYS0301 H39 first-2013 forcing/release lineage reconciliation."""

from __future__ import annotations

import argparse
import json
import re
import shutil
from collections import defaultdict
from pathlib import Path
from typing import Any

PACKAGE = "20260605-hphys0301-h39-forcing-melt-term-producer-closure-001"
SOURCE_PACKAGE_0300 = (
    "docs/work-packages/"
    "20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001"
)
H39_YEAR = 2013
H39_START_JULIAN = 97
H39_END_JULIAN = 112
TRACE_FILE = "H39.hphys0299.trace.jsonl"
OBSERVE_TAG_PATTERN = re.compile(
    r"^(?P<tag>\S+) y=\s*(?P<year>\d+) d=\s*(?P<day>\d+) e=.*?"
    r" s=\s*(?P<hour>\d+) v1=\s*(?P<v1>[+-]?\d\.\d+E[+-]\d+)"
    r" v2=\s*(?P<v2>[+-]?\d\.\d+E[+-]\d+)"
)


def round6(value: float) -> float:
    return round(value, 6)


def mm(value_m: float | None) -> float:
    return 0.0 if value_m is None else value_m * 1000.0


def load_hphys0300_h39_row(repo_root: Path) -> dict[str, Any]:
    ledger_path = repo_root / SOURCE_PACKAGE_0300 / "artifacts/raw-post-raw-lineage-ledger.json"
    rows = json.loads(ledger_path.read_text(encoding="utf-8"))
    for row in rows:
        if (
            row["hillslope_id"] == 39
            and row["window"] == "first-abs-storage-ge-10mm"
            and row["year"] == H39_YEAR
        ):
            return row
    raise SystemExit(f"missing H39 first-2013 row in {ledger_path}")


def parse_baseline_observe(log_path: Path) -> dict[int, dict[str, float]]:
    if not log_path.is_file():
        raise SystemExit(f"missing baseline observe log: {log_path}")
    rows: dict[int, dict[str, float]] = defaultdict(
        lambda: {
            "residual_rain_m": 0.0,
            "snowfall_depth_m": 0.0,
            "raw_melt_m": 0.0,
            "post_wmelt_m": 0.0,
            "driver_rain_m": 0.0,
        }
    )
    for line in log_path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = OBSERVE_TAG_PATTERN.match(line)
        if not match:
            continue
        year = int(match.group("year"))
        day = int(match.group("day"))
        if year != H39_YEAR or not (H39_START_JULIAN <= day <= H39_END_JULIAN):
            continue
        tag = match.group("tag")
        v1 = float(match.group("v1"))
        v2 = float(match.group("v2"))
        if tag == "H298_RAW_A":
            rows[day]["raw_melt_m"] += v1
            rows[day]["residual_rain_m"] += v2
        elif tag == "H298_RAW_B":
            rows[day]["snowfall_depth_m"] += v1
        elif tag == "H298_POST_A":
            rows[day]["post_wmelt_m"] += v1
        elif tag == "H298_DRV_A":
            rows[day]["driver_rain_m"] += v2
    return dict(rows)


def parse_openwepp_trace(trace_path: Path) -> dict[int, dict[str, float]]:
    if not trace_path.is_file():
        raise SystemExit(f"missing openWEPP trace: {trace_path}")
    rows: dict[int, dict[str, float]] = {}
    with trace_path.open(encoding="utf-8") as handle:
        for line in handle:
            row = json.loads(line)
            day = int(row.get("julian_day", -1))
            if (
                row.get("calendar_year") == H39_YEAR
                and H39_START_JULIAN <= day <= H39_END_JULIAN
                and row.get("wb13_p_mm") is not None
            ):
                rows[day] = {
                    "raw_rain_m": row.get("snow_hourly_rain_sum_m") or 0.0,
                    "rain_retained_m": row.get("snow_hourly_rain_retained_sum_m") or 0.0,
                    "rain_released_m": row.get("snow_hourly_rain_released_sum_m") or 0.0,
                    "post_winter_rain_m": row.get("snow_post_winter_rain_m") or 0.0,
                    "snowfall_depth_m": row.get("snow_hourly_snowfall_depth_sum_m") or 0.0,
                    "raw_melt_m": row.get("snow_hourly_melt_raw_sum_m") or 0.0,
                    "routed_melt_m": row.get("snow_routed_melt_m") or 0.0,
                    "p_mm": row.get("wb13_p_mm") or 0.0,
                }
    return rows


def build_daily_rows(
    baseline: dict[int, dict[str, float]], openwepp: dict[int, dict[str, float]]
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for day in range(H39_START_JULIAN, H39_END_JULIAN + 1):
        base = baseline.get(day, {})
        candidate = openwepp.get(day, {})
        open_residual_rain_m = (
            candidate.get("rain_released_m", 0.0) + candidate.get("post_winter_rain_m", 0.0)
        )
        rows.append(
            {
                "julian_day": day,
                "baseline_residual_rain_mm": round6(mm(base.get("residual_rain_m"))),
                "openwepp_raw_rain_mm": round6(mm(candidate.get("raw_rain_m"))),
                "openwepp_rain_retained_mm": round6(mm(candidate.get("rain_retained_m"))),
                "openwepp_rain_released_mm": round6(mm(candidate.get("rain_released_m"))),
                "openwepp_post_winter_rain_mm": round6(
                    mm(candidate.get("post_winter_rain_m"))
                ),
                "openwepp_released_plus_post_rain_mm": round6(mm(open_residual_rain_m)),
                "baseline_minus_open_raw_rain_mm": round6(
                    mm(base.get("residual_rain_m")) - mm(candidate.get("raw_rain_m"))
                ),
                "baseline_minus_open_released_plus_post_rain_mm": round6(
                    mm(base.get("residual_rain_m")) - mm(open_residual_rain_m)
                ),
                "baseline_snowfall_depth_mm": round6(mm(base.get("snowfall_depth_m"))),
                "openwepp_snowfall_depth_mm": round6(
                    mm(candidate.get("snowfall_depth_m"))
                ),
                "baseline_minus_open_snowfall_depth_mm": round6(
                    mm(base.get("snowfall_depth_m")) - mm(candidate.get("snowfall_depth_m"))
                ),
                "baseline_raw_melt_mm": round6(mm(base.get("raw_melt_m"))),
                "openwepp_raw_melt_mm": round6(mm(candidate.get("raw_melt_m"))),
                "baseline_minus_open_raw_melt_mm": round6(
                    mm(base.get("raw_melt_m")) - mm(candidate.get("raw_melt_m"))
                ),
                "baseline_post_wmelt_mm": round6(mm(base.get("post_wmelt_m"))),
                "openwepp_routed_melt_mm": round6(mm(candidate.get("routed_melt_m"))),
                "baseline_minus_open_routed_melt_mm": round6(
                    mm(base.get("post_wmelt_m")) - mm(candidate.get("routed_melt_m"))
                ),
            }
        )
    return rows


def sum_field(rows: list[dict[str, Any]], key: str) -> float:
    return round6(sum(float(row[key]) for row in rows))


def build_ledger(source_row: dict[str, Any], daily_rows: list[dict[str, Any]]) -> dict[str, Any]:
    baseline_residual_rain = sum_field(daily_rows, "baseline_residual_rain_mm")
    open_raw_rain = sum_field(daily_rows, "openwepp_raw_rain_mm")
    open_retained_rain = sum_field(daily_rows, "openwepp_rain_retained_mm")
    open_released_plus_post = sum_field(daily_rows, "openwepp_released_plus_post_rain_mm")
    baseline_snow = sum_field(daily_rows, "baseline_snowfall_depth_mm")
    open_snow = sum_field(daily_rows, "openwepp_snowfall_depth_mm")
    baseline_raw_melt = sum_field(daily_rows, "baseline_raw_melt_mm")
    open_raw_melt = sum_field(daily_rows, "openwepp_raw_melt_mm")
    baseline_post_wmelt = sum_field(daily_rows, "baseline_post_wmelt_mm")
    open_routed_melt = sum_field(daily_rows, "openwepp_routed_melt_mm")

    return {
        "hillslope_id": 39,
        "window": "first-abs-storage-ge-10mm",
        "year": H39_YEAR,
        "start_julian": H39_START_JULIAN,
        "end_julian": H39_END_JULIAN,
        "source_hphys0300_route": source_row["hphys0300_route"],
        "hphys0301_route": "h39-rain-release-lineage-reclassified-hold",
        "production_edit_authorized": False,
        "production_forcing_edit_authorized": False,
        "production_snow_melt_edit_authorized": False,
        "implementation_decision": "no_production_edit_without_paired_term_state_source",
        "baseline_residual_rain_mm": baseline_residual_rain,
        "openwepp_raw_rain_mm": open_raw_rain,
        "openwepp_retained_rain_mm": open_retained_rain,
        "openwepp_released_plus_post_rain_mm": open_released_plus_post,
        "baseline_minus_open_raw_rain_mm": round6(baseline_residual_rain - open_raw_rain),
        "baseline_minus_open_released_plus_post_rain_mm": round6(
            baseline_residual_rain - open_released_plus_post
        ),
        "baseline_snowfall_depth_mm": baseline_snow,
        "openwepp_snowfall_depth_mm": open_snow,
        "baseline_minus_open_snowfall_depth_mm": round6(baseline_snow - open_snow),
        "baseline_raw_melt_mm": baseline_raw_melt,
        "openwepp_raw_melt_mm": open_raw_melt,
        "baseline_minus_open_raw_melt_mm": round6(baseline_raw_melt - open_raw_melt),
        "baseline_post_wmelt_mm": baseline_post_wmelt,
        "openwepp_routed_melt_mm": open_routed_melt,
        "baseline_minus_open_routed_melt_mm": round6(baseline_post_wmelt - open_routed_melt),
        "raw_rain_delta_explained_by_openwepp_retention_mm": open_retained_rain,
        "forcing_root_cause_confirmed": False,
        "blocking_invariant": (
            "The HPHYS0300 H39 raw-rain aggregate compared baseline residual rain-on-snow "
            "`hrrain` evidence against openWEPP raw `snow_hourly_rain_sum_m`. Comparing "
            "baseline residual rain to openWEPP released plus post-winter rain collapses "
            "the 16.476985 mm raw-rain delta to a sub-millimeter residual. The pinned "
            "source tree does not contain the H298 observe tag sites, so the package "
            "cannot use those tags as source-line authority for a forcing edit. Remaining "
            "`hrmlt`/`wmelt` deltas require paired `melt.for`/`snowd.for` term/state "
            "instrumentation."
        ),
        "source_provenance": [
            {
                "canonical_symbol": "raw stmtim hrrain/hrsnow",
                "baseline_source_path": "/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95",
                "openwepp_source_path": "crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs:629-689",
                "finding": "static partition formulas match visible source; no source-line forcing defect proven",
            },
            {
                "canonical_symbol": "breakpoint stmstr/stmdur/prcp",
                "baseline_source_path": "/workdir/wepp-forest_260430_baseline/src/brkpt.for:61-117",
                "openwepp_source_path": "crates/openwepp-climate-runtime-adapter/src/lib.rs:451-570",
                "finding": "H39 target-day breakpoint start/duration projection follows first-to-last breakpoint span",
            },
            {
                "canonical_symbol": "residual rain-on-snow hrrain",
                "baseline_source_path": "/workdir/wepp-forest_260430_baseline/src/snowd.for:240-279",
                "openwepp_source_path": "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4005-4075",
                "finding": "baseline residual-rain aggregate aligns with openWEPP released plus post-winter rain, not open raw rain",
            },
            {
                "canonical_symbol": "daily wmelt/routed melt",
                "baseline_source_path": "/workdir/wepp-forest_260430_baseline/src/winter.for:420-476",
                "openwepp_source_path": "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3945-4095",
                "finding": "remaining melt/routed-melt delta is post-retention producer lineage, not raw forcing authority",
            },
        ],
        "required_follow_on_symbols": [
            "amelt",
            "bmelt",
            "cmelt",
            "dmelt",
            "hrrain",
            "hrtemp",
            "tdpt",
            "hrad",
            "cloudC",
            "vwind",
            "snodpt",
            "densgt",
            "rain_retained",
            "rain_released",
        ],
        "daily_rows": daily_rows,
    }


def write_summary(artifact_dir: Path, ledger: dict[str, Any]) -> None:
    text = "# HPHYS0301 H39 Forcing/Release Lineage Summary\n\n"
    text += "Evidence mode: static + ran\n\n"
    text += "Static:\n\n"
    text += "- Baseline partition authority: `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-95`.\n"
    text += "- Baseline rain-retention authority: `/workdir/wepp-forest_260430_baseline/src/snowd.for:240-279`.\n"
    text += "- Baseline daily routed-melt authority: `/workdir/wepp-forest_260430_baseline/src/winter.for:420-476`.\n"
    text += "- OpenWEPP forcing authority: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`.\n\n"
    text += "Ran:\n\n"
    text += "- Parsed HPHYS0300 H39 trace and pinned-baseline observe artifacts from `/tmp/hphys0300_full_20260605T155527Z`.\n\n"
    text += "## Totals\n\n"
    text += "| Metric | Value (mm) |\n"
    text += "|---|---:|\n"
    for key in [
        "baseline_residual_rain_mm",
        "openwepp_raw_rain_mm",
        "openwepp_retained_rain_mm",
        "openwepp_released_plus_post_rain_mm",
        "baseline_minus_open_raw_rain_mm",
        "baseline_minus_open_released_plus_post_rain_mm",
        "baseline_snowfall_depth_mm",
        "openwepp_snowfall_depth_mm",
        "baseline_minus_open_snowfall_depth_mm",
        "baseline_raw_melt_mm",
        "openwepp_raw_melt_mm",
        "baseline_minus_open_raw_melt_mm",
        "baseline_post_wmelt_mm",
        "openwepp_routed_melt_mm",
        "baseline_minus_open_routed_melt_mm",
    ]:
        text += f"| `{key}` | {ledger[key]:.6f} |\n"
    text += "\n## Decision\n\n"
    text += f"- Route: `{ledger['hphys0301_route']}`.\n"
    text += "- Production edit authorized: `false`.\n"
    text += "- Forcing root cause confirmed: `false`.\n"
    text += "- HPHYS0300's `baseline_minus_open_raw_rain_mm = -16.476985` raw-rain comparison is not production forcing authority because it compares baseline residual rain-on-snow evidence against openWEPP raw rain.\n"
    text += "- Comparing baseline residual rain to openWEPP released plus post-winter rain leaves a sub-millimeter aggregate residual, so H39 first-2013 must move to rain-retention/post-raw melt lineage closure.\n"
    text += "- Remaining `hrmlt`/`wmelt` residuals require paired `melt.for`/`snowd.for` term/state evidence before a production snow producer edit.\n\n"
    text += "## Daily Rows\n\n"
    text += "| Day | Base Resid Rain | Open Raw Rain | Open Retained | Open Released+Post | Δ Raw | Δ Released+Post | Base Snow Depth | Open Snow Depth | Δ Snow Depth |\n"
    text += "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n"
    for row in ledger["daily_rows"]:
        text += (
            f"| {row['julian_day']} | {row['baseline_residual_rain_mm']:.6f} | "
            f"{row['openwepp_raw_rain_mm']:.6f} | {row['openwepp_rain_retained_mm']:.6f} | "
            f"{row['openwepp_released_plus_post_rain_mm']:.6f} | "
            f"{row['baseline_minus_open_raw_rain_mm']:.6f} | "
            f"{row['baseline_minus_open_released_plus_post_rain_mm']:.6f} | "
            f"{row['baseline_snowfall_depth_mm']:.6f} | "
            f"{row['openwepp_snowfall_depth_mm']:.6f} | "
            f"{row['baseline_minus_open_snowfall_depth_mm']:.6f} |\n"
        )
    artifact_dir.joinpath("h39-forcing-release-lineage-summary.md").write_text(
        text, encoding="utf-8"
    )


def write_decision(artifact_dir: Path, ledger: dict[str, Any]) -> None:
    text = "# HPHYS0301 Implementation Decision\n\n"
    text += "Status: executed-hold\n\n"
    text += "Evidence mode: static + ran\n\n"
    text += "Static:\n\n"
    text += "- No production forcing or snow-kernel edit is authorized by HPHYS0301.\n"
    text += "- Static `stmtim.for` and openWEPP hourly partition formulas are aligned for the visible raw partition equation.\n"
    text += "- Baseline observe tags used by HPHYS0298/0299 are not present as source-line tag sites in `/workdir/wepp-forest_260430_baseline/src`, so they are evidence artifacts, not equation authority.\n\n"
    text += "Ran:\n\n"
    text += "- HPHYS0301 lineage runner parsed H39 first-2013 daily rows and produced `h39-forcing-release-lineage-ledger.json`.\n\n"
    text += "Decision:\n\n"
    text += "- `production_forcing_edit_authorized = false`.\n"
    text += "- `production_snow_melt_edit_authorized = false`.\n"
    text += f"- `hphys0301_route = {ledger['hphys0301_route']}`.\n"
    text += f"- Blocking invariant: {ledger['blocking_invariant']}\n"
    artifact_dir.joinpath("correction-decision.md").write_text(text, encoding="utf-8")


def copy_full_suite_artifacts(repo_root: Path, artifact_dir: Path) -> None:
    source_dir = repo_root / SOURCE_PACKAGE_0300 / "artifacts"
    for filename in ["full-39-suite-metrics.md", "full-39-suite-summary.json"]:
        source = source_dir / filename
        if source.is_file():
            shutil.copyfile(source, artifact_dir / filename)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--run-root",
        type=Path,
        default=Path("/tmp/hphys0300_full_20260605T155527Z"),
    )
    parser.add_argument(
        "--artifact-dir",
        type=Path,
        default=Path("docs/work-packages") / PACKAGE / "artifacts",
    )
    args = parser.parse_args()

    repo_root = Path.cwd()
    artifact_dir = args.artifact_dir
    artifact_dir.mkdir(parents=True, exist_ok=True)

    source_row = load_hphys0300_h39_row(repo_root)
    baseline = parse_baseline_observe(
        args.run_root
        / "baseline_observe/H39_observe_on/runs/wepp_observe.log"
    )
    openwepp = parse_openwepp_trace(args.run_root / "hillslope_output" / TRACE_FILE)
    daily_rows = build_daily_rows(baseline, openwepp)
    ledger = build_ledger(source_row, daily_rows)

    (artifact_dir / "h39-forcing-release-lineage-ledger.json").write_text(
        json.dumps(ledger, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    write_summary(artifact_dir, ledger)
    write_decision(artifact_dir, ledger)
    copy_full_suite_artifacts(repo_root, artifact_dir)

    print(
        json.dumps(
            {
                "route": ledger["hphys0301_route"],
                "production_edit_authorized": ledger["production_edit_authorized"],
                "baseline_minus_open_raw_rain_mm": ledger[
                    "baseline_minus_open_raw_rain_mm"
                ],
                "baseline_minus_open_released_plus_post_rain_mm": ledger[
                    "baseline_minus_open_released_plus_post_rain_mm"
                ],
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
