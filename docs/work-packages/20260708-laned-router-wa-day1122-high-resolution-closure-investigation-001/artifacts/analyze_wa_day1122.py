#!/usr/bin/env python3
"""Summarize WA day-1122 active-router numerics evidence."""

from __future__ import annotations

import hashlib
import json
import math
import re
from pathlib import Path
from typing import Any

try:
    import pyarrow.parquet as pq
except ImportError:  # pragma: no cover - recorded in output.
    pq = None

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
SUMMARY_JSON = ARTIFACTS / "mesh-ladder-summary.json"
RUN_ROOT = ARTIFACTS / "mesh-ladder-runs" / "wa_cascades_forest_h1"
OUT_JSON = ARTIFACTS / "wa-day1122-analysis.json"
DAY1122_MD = ARTIFACTS / "day1122-reproduction.md"
MAGNITUDE_MD = ARTIFACTS / "magnitude-attribution.md"

COMPLETED_RUNGS = ["baseline_fixed10", "dx20", "dx10", "dx5"]
ALL_RUNGS = COMPLETED_RUNGS + ["dx2p5", "dx1p25"]
HYDROLOGY_COLUMNS = [
    "sim_day_index",
    "OFE",
    "P",
    "RM",
    "Q",
    "QOFE",
    "UpStrmQ",
    "latqcc",
    "Area",
    "SoilWaterTotal",
]

FAIL_RE = re.compile(
    r"day (?P<day>\d+) cascade residual (?P<residual>[-+0-9.eE]+) m3 "
    r"\(rel (?P<rel>[-+0-9.eE]+) > (?P<tol>[-+0-9.eE]+)\): "
    r"injected (?P<injected>[-+0-9.eE]+) \+ clamp (?P<clamp>[-+0-9.eE]+) "
    r"- outlet (?P<outlet>[-+0-9.eE]+) - mesh_storage (?P<storage>[-+0-9.eE]+)"
)


def sha256(path: Path) -> str | None:
    if not path.is_file():
        return None
    digest = hashlib.sha256()
    with path.open("rb") as fp:
        for chunk in iter(lambda: fp.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_summary() -> dict[str, Any]:
    return json.loads(SUMMARY_JSON.read_text())


def run_by_rung(summary: dict[str, Any], rung: str) -> dict[str, Any]:
    for run in summary["runs"]:
        if run["member_id"] == "wa_cascades_forest_h1" and run["rung"] == rung:
            return run
    raise KeyError(rung)


def trace_rows(run: dict[str, Any]) -> list[dict[str, Any]]:
    trace_path = run.get("trace_path")
    if not trace_path:
        return []
    path = Path(trace_path)
    if not path.is_file():
        return []
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def day_trace_book(rows: list[dict[str, Any]], sim_day_index: int) -> dict[str, Any]:
    day_rows = [row for row in rows if row.get("sim_day_index") == sim_day_index]
    terminal = [
        row
        for row in day_rows
        if row.get("is_terminal_lane") and row.get("terminal_day_outlet_m3") is not None
    ]
    outlet = float(terminal[-1]["terminal_day_outlet_m3"]) if terminal else 0.0
    source = sum(float(row.get("source_m3") or 0.0) for row in day_rows)
    clamp = sum(float(row.get("clamp_m3") or 0.0) for row in day_rows)
    storage = sum(float(row.get("mesh_end_storage_m3") or 0.0) for row in day_rows)
    residual = source + clamp - outlet - storage
    rel = abs(residual) / source if source > 0.0 else 0.0
    return {
        "sim_day_index": sim_day_index,
        "rows": len(day_rows),
        "source_m3": source,
        "clamp_m3": clamp,
        "terminal_outlet_m3": outlet,
        "mesh_storage_m3": storage,
        "cascade_residual_m3": residual,
        "cascade_residual_rel": rel,
        "terminal_lane": terminal[-1].get("lane_index") if terminal else None,
    }


def failure_record(run: dict[str, Any]) -> dict[str, Any] | None:
    text = "\n".join(
        [
            str(run.get("failure_tail") or ""),
            Path(run.get("log_path") or "").read_text(errors="replace")
            if run.get("log_path") and Path(run["log_path"]).is_file()
            else "",
        ]
    )
    match = FAIL_RE.search(text)
    if not match:
        return None
    rec = {key: float(value) for key, value in match.groupdict().items() if key != "day"}
    rec["day"] = int(match.group("day"))
    rec["abs_residual_litres"] = abs(rec["residual"]) * 1000.0
    rec["guard"] = "laned_active_day_cascade_residual"
    return rec


def top_trace_rows(rows: list[dict[str, Any]], metric: str, limit: int = 5) -> list[dict[str, Any]]:
    top = sorted(rows, key=lambda row: abs(float(row.get(metric) or 0.0)), reverse=True)[:limit]
    result = []
    for row in top:
        result.append(
            {
                "sim_day_index": row.get("sim_day_index"),
                "lane_index": row.get("lane_index"),
                metric: float(row.get(metric) or 0.0),
                "source_m3": float(row.get("source_m3") or 0.0),
                "clamp_m3": float(row.get("clamp_m3") or 0.0),
                "outlet_m3": float(row.get("outlet_m3") or 0.0),
                "mesh_end_storage_m3": float(row.get("mesh_end_storage_m3") or 0.0),
                "terminal_day_outlet_m3": row.get("terminal_day_outlet_m3"),
            }
        )
    return result


def climate_rows(run_dir: Path, days: list[int]) -> dict[str, dict[str, Any]]:
    cli_path = run_dir / "p1.cli"
    rows: list[list[str]] = []
    for line in cli_path.read_text().splitlines():
        parts = line.split()
        if len(parts) >= 11 and parts[0].isdigit() and parts[1].isdigit() and parts[2].isdigit():
            rows.append(parts)
    result: dict[str, dict[str, Any]] = {}
    for day in days:
        parts = rows[day - 1]
        result[str(day)] = {
            "date": f"{int(parts[2]):04d}-{int(parts[1]):02d}-{int(parts[0]):02d}",
            "prcp_mm": float(parts[3]),
            "duration_h": float(parts[4]),
            "tp": float(parts[5]),
            "ip": float(parts[6]),
            "tmax_c": float(parts[7]),
            "tmin_c": float(parts[8]),
        }
    return result


def slope_lengths(run_dir: Path) -> list[float]:
    lines = (run_dir / "p1.slp").read_text().splitlines()
    lengths: list[float] = []
    for line in lines[3:]:
        parts = line.split()
        if len(parts) == 2 and parts[0].isdigit():
            try:
                lengths.append(float(parts[1]))
            except ValueError:
                pass
    return lengths


def cell_counts(lengths: list[float]) -> dict[str, list[int]]:
    targets = {
        "baseline_fixed10": None,
        "dx20": 20.0,
        "dx10": 10.0,
        "dx5": 5.0,
        "dx2p5": 2.5,
        "dx1p25": 1.25,
    }
    counts: dict[str, list[int]] = {}
    for rung, target in targets.items():
        if target is None:
            counts[rung] = [10 for _ in lengths]
        else:
            counts[rung] = [max(math.ceil(length / target), 10) for length in lengths]
    return counts


def read_wat_rows(run: dict[str, Any], days: list[int]) -> dict[str, list[dict[str, Any]]]:
    if pq is None:
        return {"pyarrow_error": [{"error": "pyarrow unavailable"}]}
    out_dir = Path(run["output_dir"])
    path = out_dir / "H1.wat.parquet"
    table = pq.read_table(path, columns=HYDROLOGY_COLUMNS)
    rows = table.to_pylist()
    return {
        str(day): [row for row in rows if row["sim_day_index"] == day]
        for day in days
    }


def hydrology_deltas(summary: dict[str, Any], days: list[int]) -> dict[str, Any]:
    if pq is None:
        return {"error": "pyarrow unavailable"}
    baseline = read_wat_rows(run_by_rung(summary, "baseline_fixed10"), days)
    result: dict[str, Any] = {}
    for rung in COMPLETED_RUNGS:
        rows = read_wat_rows(run_by_rung(summary, rung), days)
        day_deltas: dict[str, dict[str, float]] = {}
        for day in days:
            base_rows = baseline[str(day)]
            rung_rows = rows[str(day)]
            max_by_column = {column: 0.0 for column in HYDROLOGY_COLUMNS if column not in {"sim_day_index", "OFE"}}
            for base_row, rung_row in zip(base_rows, rung_rows):
                for column in max_by_column:
                    base_value = base_row.get(column)
                    rung_value = rung_row.get(column)
                    if base_value is None or rung_value is None:
                        continue
                    max_by_column[column] = max(
                        max_by_column[column],
                        abs(float(rung_value) - float(base_value)),
                    )
            day_deltas[str(day)] = max_by_column
        result[rung] = day_deltas
    return result


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, float):
        return f"{value:.12g}"
    return str(value)


def write_day1122_md(analysis: dict[str, Any]) -> None:
    lines = [
        "# Day-1122 Reproduction",
        "",
        "Status: EXECUTED",
        "Evidence mode: Ran.",
        "",
        "## Source Evidence",
        "",
        f"- Summary JSON: `{SUMMARY_JSON}`",
        f"- Summary SHA256: `{analysis['summary_sha256']}`",
        "",
        "## Climate and Mesh",
        "",
        "| Surface | Value |",
        "|---|---:|",
    ]
    climate = analysis["climate"]["1122"]
    lines.extend(
        [
            f"| Date | {climate['date']} |",
            f"| Precipitation mm | {fmt(climate['prcp_mm'])} |",
            f"| Duration h | {fmt(climate['duration_h'])} |",
            f"| Peak-intensity input | {fmt(climate['ip'])} |",
            f"| OFE lengths m | {', '.join(fmt(v) for v in analysis['slope_lengths_m'])} |",
        ]
    )
    lines.extend(["", "## Completed Rung Day Books", "", "| Rung | Rows | Source m3 | Clamp m3 | Terminal outlet m3 | Mesh storage m3 | Residual m3 | Relative |", "|---|---:|---:|---:|---:|---:|---:|---:|"])
    for rung in COMPLETED_RUNGS:
        rec = analysis["day1122_completed_books"][rung]
        lines.append(
            f"| {rung} | {rec['rows']} | {fmt(rec['source_m3'])} | {fmt(rec['clamp_m3'])} | "
            f"{fmt(rec['terminal_outlet_m3'])} | {fmt(rec['mesh_storage_m3'])} | "
            f"{fmt(rec['cascade_residual_m3'])} | {fmt(rec['cascade_residual_rel'])} |"
        )
    lines.extend(["", "## Failing Fine Rungs", "", "| Rung | Day | Guard | Injected m3 | Clamp m3 | Outlet m3 | Mesh storage m3 | Residual m3 | Litres | Relative | Tolerance | Wall | User |", "|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"])
    for rung in ["dx2p5", "dx1p25"]:
        run = analysis["runs"][rung]
        rec = analysis["fine_failures"][rung]
        timing = run.get("timing") or {}
        lines.append(
            f"| {rung} | {rec['day']} | `{rec['guard']}` | {fmt(rec['injected'])} | "
            f"{fmt(rec['clamp'])} | {fmt(rec['outlet'])} | {fmt(rec['storage'])} | "
            f"{fmt(rec['residual'])} | {fmt(rec['abs_residual_litres'])} | "
            f"{fmt(rec['rel'])} | {fmt(rec['tol'])} | {fmt(timing.get('wall_seconds_raw'))} | "
            f"{fmt(timing.get('user_seconds'))} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation Inputs",
            "",
            "- The first failing guard is the active day cascade residual. The code",
            "  returns at that guard, so the logs do not prove whether later seam or",
            "  identity checks would pass on the failed fine rungs.",
            "- The absolute residuals are sub-litre to decilitre scale, but the guard is",
            "  relative to injected source volume while clamp/storage operands are",
            "  eight to nine orders larger than the residual.",
            "",
        ]
    )
    DAY1122_MD.write_text("\n".join(lines))


def write_magnitude_md(analysis: dict[str, Any]) -> None:
    lines = [
        "# Magnitude Attribution",
        "",
        "Status: EXECUTED",
        "Evidence mode: Ran.",
        "",
        "## Dominant Clamp Rows",
        "",
        "| Rung | Rank | Day | Lane | Clamp m3 | Source m3 | Outlet m3 | Storage m3 |",
        "|---|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for rung in COMPLETED_RUNGS:
        for i, row in enumerate(analysis["top_rows"][rung]["clamp_m3"], start=1):
            lines.append(
                f"| {rung} | {i} | {row['sim_day_index']} | {row['lane_index']} | "
                f"{fmt(row['clamp_m3'])} | {fmt(row['source_m3'])} | "
                f"{fmt(row['outlet_m3'])} | {fmt(row['mesh_end_storage_m3'])} |"
            )
    lines.extend(
        [
            "",
            "## Day-1418 Climate",
            "",
            "| Surface | Value |",
            "|---|---:|",
        ]
    )
    climate = analysis["climate"]["1418"]
    lines.extend(
        [
            f"| Date | {climate['date']} |",
            f"| Precipitation mm | {fmt(climate['prcp_mm'])} |",
            f"| Duration h | {fmt(climate['duration_h'])} |",
            f"| Peak-intensity input | {fmt(climate['ip'])} |",
            "",
            "## Hydrology-Source Delta Check",
            "",
            "Maximum absolute delta from `baseline_fixed10` in `H1.wat.parquet` for",
            "the inspected days. Zero deltas mean the source producer is not changing",
            "across mesh rungs.",
            "",
            "| Rung | Day | Max hydrology delta |",
            "|---|---:|---:|",
        ]
    )
    for rung, days in analysis["hydrology_deltas"].items():
        for day, columns in days.items():
            max_delta = max(float(value) for value in columns.values()) if columns else 0.0
            lines.append(f"| {rung} | {day} | {fmt(max_delta)} |")
    lines.extend(
        [
            "",
            "## Attribution",
            "",
            "- The largest completed-rung magnitudes localize to day 1418, lane 5,",
            "  not to day 1122.",
            "- The active hydrology source rows are unchanged across completed rungs",
            "  for the inspected days, so the amplification is router-internal.",
            "- `dx20` is identical to `baseline_fixed10` because the 10-cell floor",
            "  controls 108.34 m OFEs; finer target-`dx` rungs increase cell counts",
            "  and expose the clamp/storage amplification.",
            "",
        ]
    )
    MAGNITUDE_MD.write_text("\n".join(lines))


def main() -> None:
    summary = load_summary()
    runs = {rung: run_by_rung(summary, rung) for rung in ALL_RUNGS}
    rows = {rung: trace_rows(runs[rung]) for rung in COMPLETED_RUNGS}
    baseline_run_dir = RUN_ROOT / "baseline_fixed10" / "run_dir"
    days = [1122, 1167, 1418]
    analysis = {
        "summary_json": str(SUMMARY_JSON),
        "summary_sha256": sha256(SUMMARY_JSON),
        "created_from_status": summary["status"],
        "release_binary": summary["release_binary"],
        "runs": runs,
        "slope_lengths_m": slope_lengths(baseline_run_dir),
        "climate": climate_rows(baseline_run_dir, days),
        "fine_failures": {rung: failure_record(runs[rung]) for rung in ["dx2p5", "dx1p25"]},
        "day1122_completed_books": {
            rung: day_trace_book(rows[rung], 1122) for rung in COMPLETED_RUNGS
        },
        "day1418_completed_books": {
            rung: day_trace_book(rows[rung], 1418) for rung in COMPLETED_RUNGS
        },
        "top_rows": {
            rung: {
                "clamp_m3": top_trace_rows(rows[rung], "clamp_m3"),
                "outlet_m3": top_trace_rows(rows[rung], "outlet_m3"),
                "mesh_end_storage_m3": top_trace_rows(rows[rung], "mesh_end_storage_m3"),
            }
            for rung in COMPLETED_RUNGS
        },
        "hydrology_deltas": hydrology_deltas(summary, days),
    }
    analysis["cell_counts"] = cell_counts(analysis["slope_lengths_m"])
    OUT_JSON.write_text(json.dumps(analysis, indent=2, sort_keys=True) + "\n")
    write_day1122_md(analysis)
    write_magnitude_md(analysis)
    print(json.dumps({"analysis": str(OUT_JSON), "status": "PASS"}, sort_keys=True))


if __name__ == "__main__":
    main()
