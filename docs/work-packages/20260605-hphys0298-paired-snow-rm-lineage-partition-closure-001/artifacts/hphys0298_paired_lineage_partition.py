#!/usr/bin/env python3
"""Run HPHYS0298 paired baseline/openWEPP snow-RM lineage diagnostics."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import re
import shutil
import subprocess
import sys
import time
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0297_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/artifacts/hphys0297_defect_ledger.py"
)
SEMANTIC_WAT_SCRIPT = REPO / "tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py"
BASELINE_RELEASE_BIN = Path("/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill")
BASELINE_OBSERVE_BIN = Path("/tmp/hphys0298_wepp_forest_obs/src/wepp_hill")
BASELINE_COMMIT = "dac3c950d8b16cc73774bf5ce2e7e11f80baac70"
BASELINE_SOURCE = "/workdir/wepp-forest_260430_baseline"
BASELINE_OBSERVE_WORKTREE = "/tmp/hphys0298_wepp_forest_obs"
WINDOW_TOLERANCE_MM = 2.0
IDENTITY_TOLERANCE_MM = 1.0e-9
PARTITION_CONTEXT_TOLERANCE_MM = 0.011
HRSNOW_DEPTH_TRACE_FIELD = "snow_hourly_snowfall_depth_sum_m"
HRSNOW_WATER_EQUIV_TRACE_FIELD = "snow_hourly_snowfall_water_equiv_sum_m"
HISTORICAL_HPHYS0298_HRSNOW_TRACE_FIELD = HRSNOW_WATER_EQUIV_TRACE_FIELD
REQUIRED_OPENWEPP_TRACE_FIELDS = (
    "snow_hourly_melt_raw_m",
    "snow_hourly_rain_sum_m",
    HISTORICAL_HPHYS0298_HRSNOW_TRACE_FIELD,
    "snow_routed_melt_m",
    "snow_post_winter_rain_m",
    "wb13_rm_mm",
    "wb13_q_mm",
)
BASELINE_SOURCES = {
    "snodpy": "/workdir/wepp-forest_260430_baseline/src/contin.for:846",
    "frdp": "/workdir/wepp-forest_260430_baseline/src/contin.for:846",
    "rain": "/workdir/wepp-forest_260430_baseline/src/contin.for:847",
    "hrmlt": "/workdir/wepp-forest_260430_baseline/src/winter.for:410",
    "hrrain": "/workdir/wepp-forest_260430_baseline/src/winter.for:410",
    "hrsnow": "/workdir/wepp-forest_260430_baseline/src/winter.for:412",
    "pstvML": "/workdir/wepp-forest_260430_baseline/src/winter.for:438",
    "ngtvML": "/workdir/wepp-forest_260430_baseline/src/winter.for:438",
    "wmelt": "/workdir/wepp-forest_260430_baseline/src/winter.for:476",
    "RM": "/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1084",
    "Q": "/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1088",
}
OPENWEPP_SOURCES = {
    "snow_hourly_melt_raw_m": "crates/openwepp-runner/src/hillslope/mod.rs:4540",
    "snow_hourly_rain_sum_m": "crates/openwepp-runner/src/hillslope/mod.rs:4530",
    "snow_hourly_snowfall_depth_sum_m": "crates/openwepp-runner/src/hillslope/mod.rs:4606",
    "snow_hourly_snowfall_water_equiv_sum_m": "crates/openwepp-runner/src/hillslope/mod.rs:4606",
    "snow_routed_melt_m": "crates/openwepp-runner/src/hillslope/mod.rs:4616",
    "snow_post_winter_rain_m": "crates/openwepp-runner/src/hillslope/mod.rs:4618",
    "wb13_rm_mm": "crates/openwepp-runner/src/hillslope/mod.rs:4771",
    "wb13_q_mm": "crates/openwepp-runner/src/hillslope/mod.rs:4772",
}
SELECTED_COLUMNS = {
    "Ep",
    "Total-Soil",
    "SoilWaterTotal",
    "Dp",
    "latqcc",
    "Q",
    "RM",
    "Snow-Water",
}
OBS_RE = re.compile(
    r"^(?P<tag>\S+)\s+y=\s*(?P<year>-?\d+)\s+d=\s*(?P<day>-?\d+)"
    r"\s+e=\s*(?P<element>-?\d+)\s+c=\s*(?P<chan>-?\d+)"
    r"\s+s=\s*(?P<seg>-?\d+)\s+v1=\s*(?P<v1>[-+0-9.Ee]+)"
    r"\s+v2=\s*(?P<v2>[-+0-9.Ee]+)"
)


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0297 = load_module(HPHYS0297_SCRIPT, "hphys0297_defect_ledger")
SEMANTIC_WAT = load_module(SEMANTIC_WAT_SCRIPT, "semantic_hillslope_wat_compare")
HPHYS0295 = HPHYS0297.HPHYS0295
HPHYS0291 = HPHYS0297.HPHYS0291
HPHYS0265 = HPHYS0297.HPHYS0265
TARGET_WINDOWS = HPHYS0297.TARGET_WINDOWS
TARGET_HILLS = sorted(TARGET_WINDOWS)


class UnitPairingEvidenceError(RuntimeError):
    """Raised when a canonical paired-lineage symbol maps to the wrong dimension."""


def validate_unit_pairings() -> None:
    if HISTORICAL_HPHYS0298_HRSNOW_TRACE_FIELD == HRSNOW_WATER_EQUIV_TRACE_FIELD:
        raise UnitPairingEvidenceError(
            "HPHYS0298 unit guard: canonical `hrsnow` is snowfall depth and "
            f"must be paired with `{HRSNOW_DEPTH_TRACE_FIELD}`. The historical "
            f"HPHYS0298 harness pairs it with `{HRSNOW_WATER_EQUIV_TRACE_FIELD}`, "
            "a water-equivalent accounting surface. HPHYS0298 verdicts from this "
            "mapping are non-authoritative; use HPHYS0299 corrected depth-vs-depth "
            "evidence before assigning production migration authority."
        )


@dataclass
class LegacyRunResult:
    hill: int
    lane: str
    run_dir: Path
    output_dir: Path
    wat_path: Path
    observe_log: Path
    rc: int
    seconds: float
    stdout_path: Path
    stderr_path: Path


def rounded(value: Any, digits: int = 6) -> Any:
    if value is None:
        return None
    if isinstance(value, float):
        if math.isnan(value):
            return None
        return round(value, digits)
    return value


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def trace_m_to_mm(row: dict[str, Any] | None, key: str) -> float:
    if row is None:
        return 0.0
    value = row.get(key)
    if value is None:
        return 0.0
    return float(value) * 1_000.0


def trace_mm(row: dict[str, Any] | None, key: str) -> float:
    if row is None:
        return 0.0
    value = row.get(key)
    if value is None:
        return 0.0
    return float(value)


def trace_map_values_mm(row: dict[str, Any] | None, key: str) -> list[float]:
    if row is None:
        return []
    values = row.get(key)
    if not isinstance(values, dict):
        return []
    return [float(value) * 1_000.0 for value in values.values()]


def wat_candidate(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_candidate(row, symbol)
    return 0.0 if value is None else float(value)


def wat_baseline(row: Any | None, symbol: str) -> float:
    if row is None:
        return 0.0
    value = HPHYS0295.wat_baseline(row, symbol)
    return 0.0 if value is None else float(value)


def normalize_full_suite_summary_label(run_root: Path) -> None:
    summary_path = run_root / "reports/hillslope_semantic_summary.md"
    if not summary_path.exists():
        return
    summary = summary_path.read_text(encoding="utf-8")
    for label in (
        "HPHYS0291",
        "HPHYS0295",
        "HPHYS0296",
        "HPHYS0297",
    ):
        summary = summary.replace(
            f"# {label} Full H1..H39 Semantic Summary",
            "# HPHYS0298 Full H1..H39 Semantic Summary",
            1,
        )
    summary_path.write_text(summary, encoding="utf-8")


def write_selected_metrics(run_root: Path) -> dict[str, Any]:
    reports = run_root / "reports"
    summary_json = reports / "hillslope_semantic_summary.json"
    summary = read_json(summary_json)
    selected = {row["column"]: row for row in summary if row["column"] in SELECTED_COLUMNS}
    write_json(reports / "hphys0298_selected_metrics.json", selected)
    return selected


def write_full39_metrics_artifact(run_root: Path, artifact_dir: Path, selected: dict[str, Any]) -> None:
    summary_json = run_root / "reports/hillslope_semantic_summary.json"
    summary_md = run_root / "reports/hillslope_semantic_summary.md"
    headers = [
        "Column",
        "Hillslope Fail Count",
        "Total Fail Count",
        "Mean Abs Diff Mean",
        "Max Abs Diff",
    ]
    rows = []
    for column in sorted(selected):
        row = selected[column]
        rows.append(
            [
                column,
                row.get("hillslope_fail_count"),
                row.get("total_fail_count"),
                rounded(float(row.get("mean_abs_diff_mean", 0.0))),
                rounded(float(row.get("max_abs_diff", 0.0))),
            ]
        )
    text = "# HPHYS0298 Full-39 Suite Metrics\n\n"
    text += "Ran:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Summary JSON: `{summary_json}`\n"
    text += f"- Summary Markdown: `{summary_md}`\n"
    text += f"- Candidate HEAD: `{current_git_head()}`\n"
    text += "- Suite scope: H1..H39 hillslope semantic water-balance comparison.\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n"
    artifact_dir.mkdir(parents=True, exist_ok=True)
    (artifact_dir / "full-39-suite-metrics.md").write_text(text, encoding="utf-8")


def run_targeted_traces(run_root: Path, trace_max_days: int) -> int:
    reports = run_root / "reports"
    logs = run_root / "logs" / "targeted_traces"
    output = run_root / "hillslope_output"
    runs_dir = HPHYS0265.copy_runfiles(run_root)
    status_rows: list[dict[str, Any]] = []
    for hill in TARGET_HILLS:
        trace_path = output / f"H{hill}.hphys0298.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hill}_hphys0298_trace",
            [
                str(HPHYS0291.HILL_BIN),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hill}_openwepp.run",
                "--output-dir",
                str(output),
                "--policy",
                "compat",
            ],
            logs,
            env={
                "OPENWEPP_HPHYS0245_TRACE_PATH": str(trace_path),
                "OPENWEPP_HPHYS0245_TRACE_MAX_DAYS": str(trace_max_days),
            },
        )
        status_rows.append(
            {
                "hillslope_id": hill,
                "rc": result.rc,
                "seconds": f"{result.seconds:.3f}",
                "trace_path": trace_path,
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
        if result.rc != 0:
            HPHYS0265.write_status(reports / "hphys0298_target_trace_status.tsv", status_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "hphys0298_target_trace_status.tsv", status_rows)
    return 0


def legacy_runfile_text(hill: int, years: int = 4) -> str:
    return "\n".join(
        [
            "m",
            "Yes",
            "1",
            "1",
            "Yes",
            f"../output/H{hill}.pass.dat",
            "1",
            "No",
            f"../output/H{hill}.loss.dat",
            "Yes",
            f"../output/H{hill}.wat.dat",
            "No",
            "Yes",
            f"../output/H{hill}.soil.dat",
            "Yes",
            f"../output/H{hill}.plot.dat",
            "No",
            "Yes",
            f"../output/H{hill}.ebe.dat",
            "Yes",
            f"../output/H{hill}.element.dat",
            "No",
            "No",
            "No",
            f"p{hill}.man",
            f"p{hill}.slp",
            f"p{hill}.cli",
            f"p{hill}.sol",
            "0",
            str(years),
            "0",
            "",
        ]
    )


def prepare_legacy_lane(run_root: Path, hill: int, lane: str) -> tuple[Path, Path]:
    lane_root = run_root / "baseline_observe" / f"H{hill}_{lane}"
    if lane_root.exists():
        shutil.rmtree(lane_root)
    runs_dir = lane_root / "runs"
    output_dir = lane_root / "output"
    runs_dir.mkdir(parents=True)
    output_dir.mkdir(parents=True)
    source = HPHYS0265.SOURCE_RUNS
    for suffix in ("cli", "man", "slp", "sol"):
        shutil.copy2(source / f"p{hill}.{suffix}", runs_dir / f"p{hill}.{suffix}")
    for sidecar in ("pmetpara.txt", "snow.txt", "wepp_ui.txt"):
        shutil.copy2(source / sidecar, runs_dir / sidecar)
    (runs_dir / f"p{hill}.run").write_text(legacy_runfile_text(hill), encoding="utf-8")
    return runs_dir, output_dir


def run_legacy_binary(binary: Path, run_root: Path, hill: int, lane: str, observe: bool) -> LegacyRunResult:
    runs_dir, output_dir = prepare_legacy_lane(run_root, hill, lane)
    if observe:
        (runs_dir / "wepp_observe.on").write_text("", encoding="utf-8")
    stdout_path = runs_dir.parent / f"{lane}.stdout"
    stderr_path = runs_dir.parent / f"{lane}.stderr"
    runfile = runs_dir / f"p{hill}.run"
    start = time.monotonic()
    proc = subprocess.run(
        [str(binary)],
        input=runfile.read_text(encoding="utf-8"),
        text=True,
        cwd=runs_dir,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    seconds = time.monotonic() - start
    stdout_path.write_text(proc.stdout, encoding="utf-8", errors="ignore")
    stderr_path.write_text(proc.stderr, encoding="utf-8", errors="ignore")
    return LegacyRunResult(
        hill=hill,
        lane=lane,
        run_dir=runs_dir,
        output_dir=output_dir,
        wat_path=output_dir / f"H{hill}.wat.dat",
        observe_log=runs_dir / "wepp_observe.log",
        rc=proc.returncode,
        seconds=seconds,
        stdout_path=stdout_path,
        stderr_path=stderr_path,
    )


def parse_h298_observe_log(path: Path) -> dict[str, Any]:
    by_key: dict[tuple[int, int], dict[str, list[dict[str, float | int | str]]]] = defaultdict(
        lambda: defaultdict(list)
    )
    counts: Counter[str] = Counter()
    if not path.exists():
        return {"counts": {}, "by_key": by_key, "record_count": 0}
    record_count = 0
    with path.open(encoding="utf-8", errors="ignore") as handle:
        for line in handle:
            if not line.startswith("H298_"):
                continue
            match = OBS_RE.match(line)
            if not match:
                continue
            tag = match.group("tag")
            record = {
                "tag": tag,
                "year": int(match.group("year")),
                "julian": int(match.group("day")),
                "element": int(match.group("element")),
                "chan": int(match.group("chan")),
                "seg": int(match.group("seg")),
                "v1": float(match.group("v1")),
                "v2": float(match.group("v2")),
            }
            by_key[(int(record["year"]), int(record["julian"]))][tag].append(record)
            counts[tag] += 1
            record_count += 1
    return {"counts": dict(sorted(counts.items())), "by_key": by_key, "record_count": record_count}


def obs_records(
    observe: dict[str, Any], year: int, julian: int, tag: str
) -> list[dict[str, float | int | str]]:
    return observe["by_key"].get((year, julian), {}).get(tag, [])


def sum_obs(
    observe: dict[str, Any], year: int, julian: int, tag: str, field: str
) -> float:
    return sum(float(record[field]) for record in obs_records(observe, year, julian, tag))


def load_post_wb13_trace_index(trace_path: Path, target_keys: set[tuple[int, int]]) -> dict[tuple[int, int], dict[str, Any]]:
    index: dict[tuple[int, int], dict[str, Any]] = {}
    with trace_path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            row = json.loads(line)
            if row.get("boundary") != "post_wb13":
                continue
            key = (int(row.get("calendar_year", -1)), int(row.get("julian_day", -1)))
            if key in target_keys:
                index[key] = row
    return index


def target_keys_for_hill(hill: int) -> set[tuple[int, int]]:
    keys: set[tuple[int, int]] = set()
    for _, year, start, end in TARGET_WINDOWS[hill]:
        for julian in range(start, end + 1):
            keys.add((year, julian))
    return keys


def load_baseline_partition_rows(hill: int) -> dict[tuple[int, int], dict[str, float]]:
    import pyarrow.parquet as pq

    path = HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hill}.parquet"
    table = pq.read_table(path)
    arrays = {name: table[name].to_pylist() for name in table.column_names}
    rows: dict[tuple[int, int], dict[str, float]] = {}
    for idx in range(table.num_rows):
        year = int(arrays["year"][idx])
        julian = int(arrays["julian"][idx])
        row: dict[str, float] = {}
        for column in ("RM", "Q", "Snow-Water", "SoilWaterTotal"):
            row[column] = float(arrays[column][idx])
        row["Total-Soil"] = float(arrays["Total-Soil Water"][idx])
        rows[(year, julian)] = row
    return rows


def compare_release_to_partition(wat_path: Path, hill: int) -> dict[str, Any]:
    dat_rows, widths = SEMANTIC_WAT.parse_dat_rows(wat_path, row_year_offset=0)
    partition_rows = load_baseline_partition_rows(hill)
    columns = ("RM", "Q", "Snow-Water", "SoilWaterTotal", "Total-Soil")
    max_abs_by_column = {column: 0.0 for column in columns}
    missing = 0
    compared = 0
    for _, year, start, end in TARGET_WINDOWS[hill]:
        for julian in range(start, end + 1):
            dat = dat_rows.get((1, julian, year))
            part = partition_rows.get((year, julian))
            if dat is None or part is None:
                missing += 1
                continue
            compared += 1
            for column in columns:
                max_abs_by_column[column] = max(
                    max_abs_by_column[column], abs(float(dat[column]) - part[column])
                )
    return {
        "row_widths": widths,
        "target_rows_compared": compared,
        "target_rows_missing": missing,
        "max_abs_by_column_mm": max_abs_by_column,
        "pass": missing == 0
        and all(value <= PARTITION_CONTEXT_TOLERANCE_MM for value in max_abs_by_column.values()),
    }


def compare_wat_identity(release_path: Path, observe_path: Path, hill: int) -> dict[str, Any]:
    release_rows, release_widths = SEMANTIC_WAT.parse_dat_rows(release_path, row_year_offset=0)
    observe_rows, observe_widths = SEMANTIC_WAT.parse_dat_rows(observe_path, row_year_offset=0)
    columns = ("RM", "Q", "Snow-Water", "SoilWaterTotal", "Total-Soil")
    max_abs_by_column = {column: 0.0 for column in columns}
    missing = 0
    compared = 0
    for _, year, start, end in TARGET_WINDOWS[hill]:
        for julian in range(start, end + 1):
            key = (1, julian, year)
            release = release_rows.get(key)
            observe = observe_rows.get(key)
            if release is None or observe is None:
                missing += 1
                continue
            compared += 1
            for column in columns:
                max_abs_by_column[column] = max(
                    max_abs_by_column[column], abs(float(release[column]) - float(observe[column]))
                )
    return {
        "release_row_widths": release_widths,
        "observe_row_widths": observe_widths,
        "target_rows_compared": compared,
        "target_rows_missing": missing,
        "max_abs_by_column_mm": max_abs_by_column,
        "semantic_pass": missing == 0
        and all(value <= IDENTITY_TOLERANCE_MM for value in max_abs_by_column.values()),
    }


def run_baseline_observe_identity(
    run_root: Path, release_bin: Path, observe_bin: Path
) -> tuple[dict[int, dict[str, Any]], dict[int, dict[str, Any]]]:
    reports = run_root / "reports"
    identity: dict[int, dict[str, Any]] = {}
    observe_payloads: dict[int, dict[str, Any]] = {}
    status_rows: list[dict[str, Any]] = []
    for hill in TARGET_HILLS:
        release = run_legacy_binary(release_bin, run_root, hill, "release", observe=False)
        observe_off = run_legacy_binary(observe_bin, run_root, hill, "observe_off", observe=False)
        observe_on = run_legacy_binary(observe_bin, run_root, hill, "observe_on", observe=True)
        release_sha = sha256_file(release.wat_path) if release.wat_path.exists() else None
        observe_off_sha = (
            sha256_file(observe_off.wat_path) if observe_off.wat_path.exists() else None
        )
        observe_on_sha = (
            sha256_file(observe_on.wat_path) if observe_on.wat_path.exists() else None
        )
        release_to_off_identity = (
            compare_wat_identity(release.wat_path, observe_off.wat_path, hill)
            if release.wat_path.exists() and observe_off.wat_path.exists()
            else {"semantic_pass": False}
        )
        off_to_on_identity = (
            compare_wat_identity(observe_off.wat_path, observe_on.wat_path, hill)
            if observe_off.wat_path.exists() and observe_on.wat_path.exists()
            else {"semantic_pass": False}
        )
        partition_identity = (
            compare_release_to_partition(release.wat_path, hill)
            if release.wat_path.exists()
            else {"pass": False}
        )
        observe_payload = parse_h298_observe_log(observe_on.observe_log)
        observe_payloads[hill] = observe_payload
        release_to_off_bit_identical = release_sha is not None and release_sha == observe_off_sha
        off_to_on_bit_identical = observe_off_sha is not None and observe_off_sha == observe_on_sha
        hill_identity = {
            "hillslope_id": hill,
            "release_binary": str(release_bin),
            "observe_binary": str(observe_bin),
            "baseline_source": BASELINE_SOURCE,
            "baseline_commit": BASELINE_COMMIT,
            "observe_worktree": BASELINE_OBSERVE_WORKTREE,
            "release_rc": release.rc,
            "observe_off_rc": observe_off.rc,
            "observe_on_rc": observe_on.rc,
            "release_seconds": release.seconds,
            "observe_off_seconds": observe_off.seconds,
            "observe_on_seconds": observe_on.seconds,
            "release_wat": str(release.wat_path),
            "observe_off_wat": str(observe_off.wat_path),
            "observe_on_wat": str(observe_on.wat_path),
            "observe_log": str(observe_on.observe_log),
            "release_sha256": release_sha,
            "observe_off_sha256": observe_off_sha,
            "observe_on_sha256": observe_on_sha,
            "release_to_observe_off_bit_identical": release_to_off_bit_identical,
            "observe_off_to_observe_on_bit_identical": off_to_on_bit_identical,
            "wat_bit_identical": release_to_off_bit_identical and off_to_on_bit_identical,
            "release_to_observe_off_semantic_identity": release_to_off_identity,
            "observe_off_to_observe_on_semantic_identity": off_to_on_identity,
            "target_window_semantic_identity": off_to_on_identity,
            "release_matches_stored_partition": partition_identity,
            "h298_record_count": observe_payload["record_count"],
            "h298_tag_counts": observe_payload["counts"],
            "pass": release.rc == 0
            and observe_off.rc == 0
            and observe_on.rc == 0
            and release_to_off_bit_identical
            and off_to_on_bit_identical
            and bool(release_to_off_identity.get("semantic_pass"))
            and bool(off_to_on_identity.get("semantic_pass"))
            and observe_payload["record_count"] > 0,
        }
        identity[hill] = hill_identity
        status_rows.append(
            {
                "hillslope_id": hill,
                "release_rc": release.rc,
                "observe_off_rc": observe_off.rc,
                "observe_on_rc": observe_on.rc,
                "release_to_off_bit_identical": release_to_off_bit_identical,
                "off_to_on_bit_identical": off_to_on_bit_identical,
                "release_to_off_semantic_identity": release_to_off_identity.get("semantic_pass"),
                "off_to_on_semantic_identity": off_to_on_identity.get("semantic_pass"),
                "partition_identity": partition_identity.get("pass"),
                "h298_records": observe_payload["record_count"],
                "observe_log": observe_on.observe_log,
            }
        )
    write_json(reports / "hphys0298_baseline_observe_identity.json", identity)
    write_json(
        reports / "hphys0298_baseline_observe_counts.json",
        {hill: payload["counts"] for hill, payload in observe_payloads.items()},
    )
    HPHYS0265.write_status(reports / "hphys0298_baseline_observe_status.tsv", status_rows)
    return identity, observe_payloads


def first_divergence_for(row: dict[str, Any]) -> tuple[str, str, str]:
    if not row["baseline_observe_identity_pass"]:
        return (
            "observe-identity",
            "UNRESOLVED",
            "Instrumented baseline observations are not usable unless release and observe-on WAT outputs are identical.",
        )
    if (
        row["baseline_observe_missing_day_count"] > 0
        or row["openwepp_trace_missing_day_count"] > 0
        or row["openwepp_trace_missing_field_count"] > 0
    ):
        return (
            "trace-gap",
            "UNRESOLVED",
            "At least one target day is missing a required paired trace, so no cut-point verdict is promotable.",
        )
    if abs(row["baseline_wb_rm_observe_minus_wat_mm"]) > WINDOW_TOLERANCE_MM:
        return (
            "baseline-observe-rm-identity",
            "UNRESOLVED",
            "Baseline `H298_WBH_C`/`RM` observe values do not reproduce baseline WAT `RM` within tolerance.",
        )
    if abs(row["openwepp_wb13_rm_identity_abs_sum_mm"]) > WINDOW_TOLERANCE_MM:
        return (
            "WB13-RM-Q-identity",
            "OPENWEPP-DEFECTIVE",
            "openWEPP producer-consumer `RM` identity is open at WB13, so downstream water-balance metrics are not source-authoritative.",
        )
    if (
        abs(row["baseline_raw_rain_minus_openwepp_raw_rain_mm"]) > WINDOW_TOLERANCE_MM
        or abs(row["baseline_raw_snow_minus_openwepp_raw_snow_mm"]) > WINDOW_TOLERANCE_MM
    ):
        return (
            "hourly-forcing",
            "OPENWEPP-DEFECTIVE",
            "Hourly rain/snow forcing reaching the snow producer differs before raw melt, post-winter driver publication, and storage consumers.",
        )
    if abs(row["baseline_raw_melt_minus_openwepp_raw_melt_mm"]) > WINDOW_TOLERANCE_MM:
        return (
            "raw-hourly-melt",
            "OPENWEPP-DEFECTIVE",
            "Baseline observe identity passed; first comparable producer cut-point differs before negative-melt correction or runoff/storage consumers.",
        )
    if abs(row["baseline_post_wmelt_minus_openwepp_routed_melt_mm"]) > WINDOW_TOLERANCE_MM:
        if row["baseline_negative_raw_melt_sum_mm"] < -WINDOW_TOLERANCE_MM:
            return (
                "negative-melt-correction",
                "LEGACY-DEFECTIVE",
                "Baseline has material negative raw melt and the first post-raw divergence is the corrected routed melt; this is the known signed-melt correction authority.",
            )
        return (
            "negative-melt-correction",
            "OPENWEPP-DEFECTIVE",
            "Raw melt is closed, but openWEPP routed melt diverges without a material legacy negative-melt defect signal.",
        )
    if abs(row["baseline_wb_rm_observe_minus_openwepp_wb13_rm_mm"]) > WINDOW_TOLERANCE_MM:
        return (
            "runoff-driver-input",
            "OPENWEPP-DEFECTIVE",
            "Snow/rain producer output is closed, but the runoff driver `RM` differs before storage consumers.",
        )
    return (
        "WB17-WB18-WB19-storage-consumers",
        "UNRESOLVED",
        "Snow/RM lineage is closed through WB13; residuals, if present, must be partitioned in storage consumers.",
    )


def first_divergent_symbols(row: dict[str, Any]) -> list[str]:
    cut_point = row["first_divergent_cut_point"]
    symbols: list[str] = []
    if cut_point == "hourly-forcing":
        if abs(row["baseline_raw_rain_minus_openwepp_raw_rain_mm"]) > WINDOW_TOLERANCE_MM:
            symbols.append("hrrain")
        if abs(row["baseline_raw_snow_minus_openwepp_raw_snow_mm"]) > WINDOW_TOLERANCE_MM:
            symbols.append("hrsnow")
    elif cut_point == "raw-hourly-melt":
        symbols.append("hrmlt")
    elif cut_point == "negative-melt-correction":
        symbols.extend(["pstvML", "ngtvML", "wmelt"])
    elif cut_point == "runoff-driver-input":
        symbols.extend(["rain", "wmelt", "RM"])
    elif cut_point == "WB13-RM-Q-identity":
        symbols.extend(["RM", "Q"])
    elif cut_point == "baseline-observe-rm-identity":
        symbols.append("RM")
    elif cut_point == "trace-gap":
        symbols.extend(sorted({item["field"] for item in row["openwepp_trace_missing_fields"]}))
    return symbols


def provenance_row(
    canonical_symbol: str,
    openwepp_symbol: str,
    unit: str,
    baseline_value_mm: float | None,
    openwepp_value_mm: float | None,
) -> dict[str, Any]:
    return {
        "canonical_symbol": canonical_symbol,
        "openwepp_symbol": openwepp_symbol,
        "unit": unit,
        "baseline_value_mm": rounded(baseline_value_mm),
        "openwepp_value_mm": rounded(openwepp_value_mm),
        "delta_mm": rounded(
            None
            if baseline_value_mm is None or openwepp_value_mm is None
            else baseline_value_mm - openwepp_value_mm
        ),
        "baseline_source_path": BASELINE_SOURCES.get(canonical_symbol, BASELINE_SOURCE),
        "openwepp_source_path": OPENWEPP_SOURCES.get(openwepp_symbol, "crates/openwepp-runner/src/hillslope/mod.rs"),
    }


def source_provenance_for(row: dict[str, Any]) -> list[dict[str, Any]]:
    payload = [
        provenance_row(
            "hrrain",
            "snow_hourly_rain_sum_m",
            "mm",
            row["baseline_raw_rain_sum_mm"],
            row["openwepp_raw_rain_sum_mm"],
        ),
        provenance_row(
            "hrsnow",
            HISTORICAL_HPHYS0298_HRSNOW_TRACE_FIELD,
            "mm",
            row["baseline_raw_snow_sum_mm"],
            row["openwepp_raw_snow_sum_mm"],
        ),
        provenance_row(
            "hrmlt",
            "snow_hourly_melt_raw_m",
            "mm",
            row["baseline_raw_melt_sum_mm"],
            row["openwepp_raw_melt_sum_mm"],
        ),
        provenance_row(
            "wmelt",
            "snow_routed_melt_m",
            "mm",
            row["baseline_post_wmelt_sum_mm"],
            row["openwepp_routed_melt_sum_mm"],
        ),
        provenance_row(
            "rain",
            "snow_post_winter_rain_m",
            "mm",
            row["baseline_raw_rain_sum_mm"],
            row["openwepp_post_winter_rain_sum_mm"],
        ),
        provenance_row(
            "RM",
            "wb13_rm_mm",
            "mm",
            row["baseline_wb_rm_observe_sum_mm"],
            row["openwepp_wb13_rm_sum_mm"],
        ),
        provenance_row(
            "Q",
            "wb13_q_mm",
            "mm",
            row["baseline_wb_q_observe_sum_mm"],
            row["openwepp_wb13_q_sum_mm"],
        ),
    ]
    return payload


def next_action_for(row: dict[str, Any]) -> str:
    cut_point = row["first_divergent_cut_point"]
    if cut_point == "hourly-forcing":
        return "Open follow-on package to migrate baseline-authoritative hourly snow/rain forcing partition into openWEPP winter producer inputs."
    if cut_point == "raw-hourly-melt":
        return "Open follow-on package to migrate baseline-authoritative raw hourly melt lineage after forcing is proven closed."
    if cut_point == "negative-melt-correction":
        return "Retain corrected negative-melt authority; use only after upstream forcing/raw melt closure is proven."
    if cut_point == "trace-gap":
        return "Repair paired trace completeness before any production correction."
    return "Keep residual in HOLD and open a focused follow-on package for the named first divergent cut-point."


def analyze_window(
    hill: int,
    merged: Any,
    trace_index: dict[tuple[int, int], dict[str, Any]],
    observe: dict[str, Any],
    identity: dict[str, Any],
    window: tuple[str, int, int, int],
) -> dict[str, Any]:
    name, year, start, end = window
    rows = merged[
        (merged["_comparison_year"] == year)
        & (merged["julian"] >= start)
        & (merged["julian"] <= end)
    ].sort_values(["_comparison_year", "julian"])
    result: dict[str, Any] = {
        "hillslope_id": hill,
        "window": name,
        "year": year,
        "start_julian": start,
        "end_julian": end,
        "row_count": int(len(rows)),
        "baseline_source": BASELINE_SOURCE,
        "baseline_commit": BASELINE_COMMIT,
        "baseline_observe_identity_pass": bool(identity.get("pass")),
        "baseline_observe_identity": {
            "wat_bit_identical": identity.get("wat_bit_identical"),
            "target_window_semantic_identity": identity.get("target_window_semantic_identity"),
            "release_matches_stored_partition": identity.get("release_matches_stored_partition"),
        },
        "baseline_observe_missing_day_count": 0,
        "openwepp_trace_missing_day_count": 0,
        "openwepp_trace_missing_field_count": 0,
        "openwepp_trace_missing_fields": [],
        "baseline_gate_day_count": 0,
        "baseline_raw_melt_sum_mm": 0.0,
        "baseline_positive_raw_melt_sum_mm": 0.0,
        "baseline_negative_raw_melt_sum_mm": 0.0,
        "baseline_raw_rain_sum_mm": 0.0,
        "baseline_raw_snow_sum_mm": 0.0,
        "baseline_post_wmelt_sum_mm": 0.0,
        "baseline_wb_rm_observe_sum_mm": 0.0,
        "baseline_wb_q_observe_sum_mm": 0.0,
        "openwepp_raw_melt_sum_mm": 0.0,
        "openwepp_positive_raw_melt_sum_mm": 0.0,
        "openwepp_negative_raw_melt_sum_mm": 0.0,
        "openwepp_raw_rain_sum_mm": 0.0,
        "openwepp_raw_snow_sum_mm": 0.0,
        "openwepp_routed_melt_sum_mm": 0.0,
        "openwepp_post_winter_rain_sum_mm": 0.0,
        "openwepp_wb13_rm_sum_mm": 0.0,
        "openwepp_wb13_q_sum_mm": 0.0,
        "openwepp_wb13_rm_identity_abs_sum_mm": 0.0,
        "baseline_wat_rm_sum_mm": 0.0,
        "candidate_wat_rm_sum_mm": 0.0,
        "baseline_wat_q_sum_mm": 0.0,
        "candidate_wat_q_sum_mm": 0.0,
        "baseline_wat_snow_sum_mm": 0.0,
        "candidate_wat_snow_sum_mm": 0.0,
        "candidate_total_soil_sum_mm": 0.0,
        "baseline_total_soil_sum_mm": 0.0,
    }
    for _, row in rows.iterrows():
        julian = int(row["julian"])
        key = (year, julian)
        trace = trace_index.get(key)
        if trace is None:
            result["openwepp_trace_missing_day_count"] += 1
        else:
            for field in REQUIRED_OPENWEPP_TRACE_FIELDS:
                if field not in trace or trace[field] is None:
                    result["openwepp_trace_missing_field_count"] += 1
                    result["openwepp_trace_missing_fields"].append(
                        {"year": year, "julian": julian, "field": field}
                    )
        gate_records = obs_records(observe, year, julian, "H298_GATE_A")
        wb_records = obs_records(observe, year, julian, "H298_WBH_C")
        if gate_records:
            result["baseline_gate_day_count"] += 1
        if not wb_records:
            result["baseline_observe_missing_day_count"] += 1

        raw_values = [float(record["v1"]) * 1_000.0 for record in obs_records(observe, year, julian, "H298_RAW_A")]
        raw_rain = [float(record["v2"]) * 1_000.0 for record in obs_records(observe, year, julian, "H298_RAW_A")]
        raw_snow = [float(record["v1"]) * 1_000.0 for record in obs_records(observe, year, julian, "H298_RAW_B")]
        result["baseline_raw_melt_sum_mm"] += sum(raw_values)
        result["baseline_positive_raw_melt_sum_mm"] += sum(value for value in raw_values if value > 0.0)
        result["baseline_negative_raw_melt_sum_mm"] += sum(value for value in raw_values if value < 0.0)
        result["baseline_raw_rain_sum_mm"] += sum(raw_rain)
        result["baseline_raw_snow_sum_mm"] += sum(raw_snow)
        result["baseline_post_wmelt_sum_mm"] += sum_obs(observe, year, julian, "H298_POST_A", "v1") * 1_000.0
        result["baseline_wb_rm_observe_sum_mm"] += sum_obs(observe, year, julian, "H298_WBH_C", "v1")
        result["baseline_wb_q_observe_sum_mm"] += sum_obs(observe, year, julian, "H298_WBH_C", "v2")

        open_raw = trace_map_values_mm(trace, "snow_hourly_melt_raw_m")
        result["openwepp_raw_melt_sum_mm"] += sum(open_raw)
        result["openwepp_positive_raw_melt_sum_mm"] += sum(value for value in open_raw if value > 0.0)
        result["openwepp_negative_raw_melt_sum_mm"] += sum(value for value in open_raw if value < 0.0)
        result["openwepp_raw_rain_sum_mm"] += trace_m_to_mm(trace, "snow_hourly_rain_sum_m")
        result["openwepp_raw_snow_sum_mm"] += trace_m_to_mm(trace, HISTORICAL_HPHYS0298_HRSNOW_TRACE_FIELD)
        routed_melt = trace_m_to_mm(trace, "snow_routed_melt_m")
        post_rain = trace_m_to_mm(trace, "snow_post_winter_rain_m")
        open_rm = trace_mm(trace, "wb13_rm_mm")
        result["openwepp_routed_melt_sum_mm"] += routed_melt
        result["openwepp_post_winter_rain_sum_mm"] += post_rain
        result["openwepp_wb13_rm_sum_mm"] += open_rm
        result["openwepp_wb13_q_sum_mm"] += trace_mm(trace, "wb13_q_mm")
        result["openwepp_wb13_rm_identity_abs_sum_mm"] += abs(open_rm - (routed_melt + post_rain))

        result["baseline_wat_rm_sum_mm"] += wat_baseline(row, "RM")
        result["candidate_wat_rm_sum_mm"] += wat_candidate(row, "RM")
        result["baseline_wat_q_sum_mm"] += wat_baseline(row, "Q")
        result["candidate_wat_q_sum_mm"] += wat_candidate(row, "Q")
        result["baseline_wat_snow_sum_mm"] += wat_baseline(row, "Snow-Water")
        result["candidate_wat_snow_sum_mm"] += wat_candidate(row, "Snow-Water")
        result["candidate_total_soil_sum_mm"] += wat_candidate(row, "Total-Soil")
        result["baseline_total_soil_sum_mm"] += wat_baseline(row, "Total-Soil")

    result["baseline_wb_rm_observe_minus_wat_mm"] = (
        result["baseline_wb_rm_observe_sum_mm"] - result["baseline_wat_rm_sum_mm"]
    )
    result["baseline_raw_melt_minus_openwepp_raw_melt_mm"] = (
        result["baseline_raw_melt_sum_mm"] - result["openwepp_raw_melt_sum_mm"]
    )
    result["baseline_raw_rain_minus_openwepp_raw_rain_mm"] = (
        result["baseline_raw_rain_sum_mm"] - result["openwepp_raw_rain_sum_mm"]
    )
    result["baseline_raw_snow_minus_openwepp_raw_snow_mm"] = (
        result["baseline_raw_snow_sum_mm"] - result["openwepp_raw_snow_sum_mm"]
    )
    result["baseline_post_wmelt_minus_openwepp_routed_melt_mm"] = (
        result["baseline_post_wmelt_sum_mm"] - result["openwepp_routed_melt_sum_mm"]
    )
    result["baseline_wb_rm_observe_minus_openwepp_wb13_rm_mm"] = (
        result["baseline_wb_rm_observe_sum_mm"] - result["openwepp_wb13_rm_sum_mm"]
    )
    result["observed_baseline_minus_candidate_rm_mm"] = (
        result["baseline_wat_rm_sum_mm"] - result["candidate_wat_rm_sum_mm"]
    )
    result["observed_baseline_minus_candidate_q_mm"] = (
        result["baseline_wat_q_sum_mm"] - result["candidate_wat_q_sum_mm"]
    )
    result["observed_baseline_minus_candidate_snow_mm"] = (
        result["baseline_wat_snow_sum_mm"] - result["candidate_wat_snow_sum_mm"]
    )
    result["observed_baseline_minus_candidate_total_soil_mm"] = (
        result["baseline_total_soil_sum_mm"] - result["candidate_total_soil_sum_mm"]
    )
    cut_point, verdict, reason = first_divergence_for(result)
    result["first_divergent_cut_point"] = cut_point
    result["verdict"] = verdict
    result["verdict_reason"] = reason
    result["first_divergent_symbols"] = first_divergent_symbols(result)
    result["source_provenance"] = source_provenance_for(result)
    result["next_action"] = next_action_for(result)
    result["prohibited_compensation_note"] = (
        "Closed Q/WB13 identity only excludes runoff/storage compensation as first source; "
        "it is not acceptance authority for WB17/WB18/WB19 residuals."
    )
    return result


def write_paired_ledger(
    run_root: Path,
    artifact_dir: Path,
    identity: dict[int, dict[str, Any]],
    observes: dict[int, dict[str, Any]],
) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    ledger: list[dict[str, Any]] = []
    for hill in TARGET_HILLS:
        merged = HPHYS0295.merged_wat_rows(run_root, hill)
        trace_index = load_post_wb13_trace_index(
            run_root / f"hillslope_output/H{hill}.hphys0298.trace.jsonl",
            target_keys_for_hill(hill),
        )
        for window in TARGET_WINDOWS[hill]:
            ledger.append(analyze_window(hill, merged, trace_index, observes[hill], identity[hill], window))
    write_json(reports / "hphys0298_paired_lineage_ledger.json", ledger)
    write_json(artifact_dir / "paired-lineage-ledger.json", ledger)
    write_paired_summary(run_root, artifact_dir, ledger, identity)
    return ledger


def write_paired_summary(
    run_root: Path,
    artifact_dir: Path,
    ledger: list[dict[str, Any]],
    identity: dict[int, dict[str, Any]],
) -> None:
    verdict_counts = Counter(row["verdict"] for row in ledger)
    cut_counts = Counter(row["first_divergent_cut_point"] for row in ledger)
    headers = [
        "Hill",
        "Window",
        "Days",
        "Verdict",
        "First Cut-Point",
        "First Symbols",
        "Baseline RM",
        "Candidate RM",
        "Baseline-Open RM",
        "Raw Snow Δ",
        "Raw Melt Δ",
        "Routed Melt Δ",
        "Q Δ",
        "Total-Soil Δ",
    ]
    rows = [
        [
            f"H{row['hillslope_id']}",
            row["window"],
            f"{row['year']} {row['start_julian']}-{row['end_julian']}",
            row["verdict"],
            row["first_divergent_cut_point"],
            ",".join(row["first_divergent_symbols"]),
            rounded(row["baseline_wat_rm_sum_mm"]),
            rounded(row["candidate_wat_rm_sum_mm"]),
            rounded(row["observed_baseline_minus_candidate_rm_mm"]),
            rounded(row["baseline_raw_snow_minus_openwepp_raw_snow_mm"]),
            rounded(row["baseline_raw_melt_minus_openwepp_raw_melt_mm"]),
            rounded(row["baseline_post_wmelt_minus_openwepp_routed_melt_mm"]),
            rounded(row["observed_baseline_minus_candidate_q_mm"]),
            rounded(row["observed_baseline_minus_candidate_total_soil_mm"]),
        ]
        for row in ledger
    ]
    text = "# HPHYS0298 Paired Snow/RM Lineage Partition\n\n"
    text += "Ran:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Baseline source: `{BASELINE_SOURCE}` at `{BASELINE_COMMIT}`\n"
    text += f"- Baseline observe worktree: `{BASELINE_OBSERVE_WORKTREE}`\n"
    text += f"- Candidate HEAD: `{current_git_head()}`\n"
    text += "- Target windows: H1/H7/H39 spring snow/RM windows from SC-SNOWFREEZE-001#INV-SNOWFREEZE-029.\n\n"
    text += "## Baseline Observe Identity\n\n"
    for hill in TARGET_HILLS:
        row = identity[hill]
        text += (
            f"- H{hill}: pass=`{row['pass']}`, bit-identical=`{row['wat_bit_identical']}`, "
            f"H298 records=`{row['h298_record_count']}`\n"
        )
    text += "\n## Verdict Counts\n\n"
    for verdict, count in sorted(verdict_counts.items()):
        text += f"- `{verdict}`: `{count}` windows\n"
    text += "\n## First Cut-Point Counts\n\n"
    for cut, count in sorted(cut_counts.items()):
        text += f"- `{cut}`: `{count}` windows\n"
    text += "\n## Ledger\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n\n## Source Provenance Payload\n\n"
    text += (
        "- Full per-window source provenance is embedded in "
        "`artifacts/paired-lineage-ledger.json` under `source_provenance`.\n"
    )
    text += (
        "- Each provenance row records canonical symbol, openWEPP symbol, unit, "
        "baseline value, openWEPP value, delta, and source path/line reference.\n"
    )
    text += "\n\n## Interpretation\n\n"
    text += (
        "- `OPENWEPP-DEFECTIVE` means the pinned, observe-identity-passing baseline "
        "and openWEPP first diverge before downstream WB17/WB18/WB19 storage consumers.\n"
    )
    text += (
        "- Supersession: the historical HPHYS0298 all-window `hourly-forcing` "
        "result is non-authoritative for production migration because it paired "
        "canonical depth symbol `hrsnow` with openWEPP water-equivalent field "
        f"`{HRSNOW_WATER_EQUIV_TRACE_FIELD}`. HPHYS0299 supplies the corrected "
        "depth-vs-depth authority.\n"
    )
    text += (
        "- `LEGACY-DEFECTIVE` is reserved for the signed negative-melt correction case; "
        "it must show raw-lineage closure and material negative raw melt before the correction cut-point.\n"
    )
    text += (
        "- `UNRESOLVED` remains a hold and cannot be converted into closure by downstream metric improvement.\n"
    )
    (run_root / "reports/hphys0298_paired_lineage_summary.md").write_text(text, encoding="utf-8")
    (artifact_dir / "paired-lineage-summary.md").write_text(text, encoding="utf-8")


def current_git_head() -> str:
    proc = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    return proc.stdout.strip() if proc.returncode == 0 else "unknown"


def write_baseline_identity_artifact(
    run_root: Path, artifact_dir: Path, identity: dict[int, dict[str, Any]]
) -> None:
    write_json(artifact_dir / "baseline-observe-identity.json", identity)
    headers = [
        "Hill",
        "Pass",
        "Release=Off",
        "Off=On",
        "Partition Identity",
        "Records",
        "Release SHA",
        "Off SHA",
        "On SHA",
    ]
    rows = []
    for hill in TARGET_HILLS:
        row = identity[hill]
        rows.append(
            [
                f"H{hill}",
                row["pass"],
                row["release_to_observe_off_bit_identical"],
                row["observe_off_to_observe_on_bit_identical"],
                row["release_matches_stored_partition"].get("pass"),
                row["h298_record_count"],
                str(row["release_sha256"])[:12],
                str(row["observe_off_sha256"])[:12],
                str(row["observe_on_sha256"])[:12],
            ]
        )
    text = "# HPHYS0298 Baseline Observe Identity\n\n"
    text += "Ran:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Release binary: `{BASELINE_RELEASE_BIN}`\n"
    text += f"- Observe binary: `{BASELINE_OBSERVE_BIN}`\n"
    text += f"- Baseline commit: `{BASELINE_COMMIT}`\n"
    text += "- Lanes: pinned release without observe, instrumented observe-off, instrumented observe-on.\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n"
    (artifact_dir / "baseline-observe-identity.md").write_text(text, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--artifact-dir", type=Path, default=ARTIFACT_DIR)
    parser.add_argument("--baseline-release-bin", type=Path, default=BASELINE_RELEASE_BIN)
    parser.add_argument("--baseline-observe-bin", type=Path, default=BASELINE_OBSERVE_BIN)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--skip-full-suite", action="store_true")
    parser.add_argument("--skip-targeted-traces", action="store_true")
    parser.add_argument("--skip-baseline-observe", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        validate_unit_pairings()
    except UnitPairingEvidenceError as error:
        print(str(error), file=sys.stderr)
        return 2
    args.run_root.mkdir(parents=True, exist_ok=True)
    args.artifact_dir.mkdir(parents=True, exist_ok=True)
    if not args.skip_full_suite:
        full_rc = HPHYS0291.run_full_hillslope_suite(args.run_root)
        if full_rc != 0:
            return int(full_rc)
        normalize_full_suite_summary_label(args.run_root)
        selected = write_selected_metrics(args.run_root)
        write_full39_metrics_artifact(args.run_root, args.artifact_dir, selected)
    elif (args.run_root / "reports/hillslope_semantic_summary.json").exists():
        selected = write_selected_metrics(args.run_root)
        write_full39_metrics_artifact(args.run_root, args.artifact_dir, selected)

    if not args.skip_targeted_traces:
        trace_rc = run_targeted_traces(args.run_root, args.trace_max_days)
        if trace_rc != 0:
            return int(trace_rc)

    if args.skip_baseline_observe:
        identity = read_json(args.run_root / "reports/hphys0298_baseline_observe_identity.json")
        observes = {
            hill: parse_h298_observe_log(
                Path(identity[str(hill)]["observe_log"])
                if str(hill) in identity
                else Path(identity[hill]["observe_log"])
            )
            for hill in TARGET_HILLS
        }
        identity = {int(key): value for key, value in identity.items()}
    else:
        identity, observes = run_baseline_observe_identity(
            args.run_root, args.baseline_release_bin, args.baseline_observe_bin
        )
        write_baseline_identity_artifact(args.run_root, args.artifact_dir, identity)

    write_paired_ledger(args.run_root, args.artifact_dir, identity, observes)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
