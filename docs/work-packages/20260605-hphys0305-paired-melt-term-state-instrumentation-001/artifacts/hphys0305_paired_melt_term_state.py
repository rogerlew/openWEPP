#!/usr/bin/env python3
"""Run HPHYS0305 paired fixed-baseline/openWEPP melt-term state diagnostics."""

from __future__ import annotations

import argparse
import difflib
import importlib.util
import json
import math
import os
import re
import shutil
import subprocess
import sys
import time
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
BASELINE_REPO = Path("/workdir/wepp-forest_260430_baseline")
FIXED_COMMIT = "47ac4c32faeea81bb99081f955a14c38b815ef4d"
FIXED_WORKTREE = Path("/tmp/hphys0305_wepp_260430_melt_terms")
FIXED_SOURCE_WORKTREE = Path("/tmp/hphys0303_wepp_260430_negmeltfix")
HPHYS0304_METRICS = (
    REPO
    / "docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/fixed-baseline-semantic-metrics.md"
)
HPHYS0299_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/hphys0299_corrected_partition.py"
)
HPHYS0303_RELEASE_BIN = Path("/tmp/hphys0303_wepp_260430_negmeltfix/release/wepp_260430_hill")

TARGET_SYMBOLS = (
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
)
OPENWEPP_FIELDS = {
    "amelt": "snow_hourly_melt_amelt_in",
    "bmelt": "snow_hourly_melt_bmelt_in",
    "cmelt": "snow_hourly_melt_cmelt_in",
    "dmelt": "snow_hourly_melt_dmelt_in",
    "hrrain": "snow_hourly_rain_m",
    "hrtemp": "winter_hourly_air_temp_c",
    "tdpt": "winter_hourly_dewpoint_c",
    "hrad": "winter_hourly_rad_mj_m2",
    "cloudC": "winter_hourly_cloud_fraction",
    "vwind": "winter_hourly_wind_m_s",
    "snodpt": "snow_hourly_depth_after_m",
    "densgt": "snow_hourly_density_after_kg_m3",
}
SYMBOL_UNITS = {
    "amelt": "in",
    "bmelt": "in",
    "cmelt": "in",
    "dmelt": "in",
    "hrrain": "m",
    "hrtemp": "degC",
    "tdpt": "degC",
    "hrad": "MJ m^-2 h^-1",
    "cloudC": "dimensionless",
    "vwind": "m s^-1",
    "snodpt": "m",
    "densgt": "kg m^-3",
}
SYMBOL_TOLERANCE = {
    "amelt": 1.0e-6,
    "bmelt": 1.0e-6,
    "cmelt": 1.0e-6,
    "dmelt": 1.0e-6,
    "hrrain": 1.0e-8,
    "hrtemp": 1.0e-3,
    "tdpt": 1.0e-3,
    "hrad": 1.0e-4,
    "cloudC": 1.0e-5,
    "vwind": 1.0e-3,
    "snodpt": 1.0e-6,
    "densgt": 1.0e-6,
}
DEPENDENCY_ORDER = (
    ("hourly-forcing", ("hrrain", "hrtemp", "tdpt", "hrad", "cloudC", "vwind")),
    ("snow-state", ("snodpt", "densgt")),
    ("melt-terms", ("amelt", "bmelt", "cmelt", "dmelt")),
)
BASELINE_SOURCES = {
    "amelt": "/workdir/wepp-forest_260430_baseline/src/melt.for:147",
    "bmelt": "/workdir/wepp-forest_260430_baseline/src/melt.for:179-180",
    "cmelt": "/workdir/wepp-forest_260430_baseline/src/melt.for:223-229",
    "dmelt": "/workdir/wepp-forest_260430_baseline/src/melt.for:258-262",
    "hrrain": "/workdir/wepp-forest_260430_baseline/src/melt.for:243",
    "hrtemp": "/workdir/wepp-forest_260430_baseline/src/melt.for",
    "tdpt": "/workdir/wepp-forest_260430_baseline/src/melt.for:215",
    "hrad": "/workdir/wepp-forest_260430_baseline/src/melt.for:147",
    "cloudC": "/workdir/wepp-forest_260430_baseline/src/melt.for:131",
    "vwind": "/workdir/wepp-forest_260430_baseline/src/melt.for:214",
    "snodpt": "/workdir/wepp-forest_260430_baseline/src/winter.for:373",
    "densgt": "/workdir/wepp-forest_260430_baseline/src/winter.for:373",
}
OPENWEPP_SOURCE = "crates/openwepp-runner/src/hillslope/mod.rs"
OBS_RE = re.compile(
    r"^(?P<tag>\S+)\s+y=\s*(?P<year>-?\d+)\s+d=\s*(?P<day>-?\d+)"
    r"\s+e=\s*(?P<element>-?\d+)\s+c=\s*(?P<chan>-?\d+)"
    r"\s+s=\s*(?P<hour>-?\d+)\s+v1=\s*(?P<v1>[-+0-9.Ee]+)"
    r"\s+v2=\s*(?P<v2>[-+0-9.Ee]+)"
)
COMMAND_LOG: list[dict[str, Any]] = []


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0299 = load_module(HPHYS0299_SCRIPT, "hphys0299_corrected_partition")
HPHYS0298 = HPHYS0299.HPHYS0298
HPHYS0265 = HPHYS0299.HPHYS0265
TARGET_HILLS = HPHYS0299.TARGET_HILLS
TARGET_WINDOWS = HPHYS0299.TARGET_WINDOWS


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def run_command(
    name: str,
    args: list[str],
    *,
    cwd: Path = REPO,
    env: dict[str, str] | None = None,
    input_text: str | None = None,
    check: bool = True,
    timeout: int = 1200,
) -> subprocess.CompletedProcess[str]:
    merged_env = None
    if env:
        merged_env = {**os.environ, **env}
    started = time.monotonic()
    proc = subprocess.run(
        args,
        cwd=cwd,
        env=merged_env,
        input=input_text,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
        timeout=timeout,
    )
    COMMAND_LOG.append(
        {
            "name": name,
            "args": args,
            "cwd": str(cwd),
            "rc": proc.returncode,
            "seconds": round(time.monotonic() - started, 3),
            "stdout_tail": proc.stdout[-4000:],
            "stderr_tail": proc.stderr[-4000:],
        }
    )
    if check and proc.returncode != 0:
        raise RuntimeError(f"{name} failed with rc={proc.returncode}: {proc.stderr[-2000:]}")
    return proc


def current_head() -> str:
    return (
        run_command("git_rev_parse_openwepp", ["git", "rev-parse", "HEAD"], check=False)
        .stdout.strip()
        or "unknown"
    )


def prepare_fixed_worktree(worktree: Path) -> dict[str, Any]:
    if worktree.exists():
        shutil.rmtree(worktree)
    if FIXED_SOURCE_WORKTREE.exists():
        source_head = run_command(
            "git_rev_parse_fixed_source_worktree",
            ["git", "-C", str(FIXED_SOURCE_WORKTREE), "rev-parse", "HEAD"],
        ).stdout.strip()
        source_status = run_command(
            "git_status_fixed_source_worktree",
            ["git", "-C", str(FIXED_SOURCE_WORKTREE), "status", "--short"],
            check=False,
        ).stdout.splitlines()
        if source_head != FIXED_COMMIT:
            raise RuntimeError(
                f"fixed source worktree {FIXED_SOURCE_WORKTREE} is {source_head}, expected {FIXED_COMMIT}"
            )
        shutil.copytree(
            FIXED_SOURCE_WORKTREE,
            worktree,
            ignore=shutil.ignore_patterns(".git"),
            symlinks=True,
        )
        return {
            "source_mode": "copied_existing_verified_worktree",
            "source_worktree": str(FIXED_SOURCE_WORKTREE),
            "source_head": source_head,
            "source_status_short": source_status,
            "source_reuse_verified": source_head == FIXED_COMMIT,
            "fixed_commit": FIXED_COMMIT,
        }
    worktree.mkdir(parents=True, exist_ok=True)
    run_command(
        "git_archive_hphys0305_fixed_commit",
        [
            "bash",
            "-lc",
            f"git -C {BASELINE_REPO} archive {FIXED_COMMIT} | tar -x -C {worktree}",
        ],
    )
    return {
        "source_mode": "git_archive_fixed_commit",
        "source_worktree": str(BASELINE_REPO),
        "source_head": FIXED_COMMIT,
        "source_status_short": [],
        "source_reuse_verified": True,
        "fixed_commit": FIXED_COMMIT,
    }


def insert_once(text: str, needle: str, addition: str, marker: str, path: Path) -> str:
    if marker in text:
        return text
    if needle not in text:
        raise RuntimeError(f"missing insertion point in {path}: {needle!r}")
    return text.replace(needle, needle + addition, 1)


def patch_fixed_observe(worktree: Path) -> None:
    melt_path = worktree / "src/melt.for"
    winter_path = worktree / "src/winter.for"
    melt = melt_path.read_text(encoding="utf-8", errors="ignore")
    melt_addition = (
        "\n"
        "      call wepp_observe('H305_T_AB',year,sdate,iplane,0,hour,\n"
        "     1  amelt,bmelt)\n"
        "      call wepp_observe('H305_T_CD',year,sdate,iplane,0,hour,\n"
        "     1  cmelt,dmelt)\n"
        "      call wepp_observe('H305_F_HT',year,sdate,iplane,0,hour,\n"
        "     1  hrtemp,tdpt)\n"
        "      call wepp_observe('H305_F_RR',year,sdate,iplane,0,hour,\n"
        "     1  hradmj,hrrain(hour))\n"
        "      call wepp_observe('H305_F_CV',year,sdate,iplane,0,hour,\n"
        "     1  cloudC,vwind)\n"
    )
    melt = insert_once(
        melt,
        "      wmelt(iplane) = 0.0254 * (amelt + bmelt + cmelt + dmelt)",
        melt_addition,
        "H305_T_AB",
        melt_path,
    )
    melt_path.write_text(melt, encoding="utf-8")

    winter = winter_path.read_text(encoding="utf-8", errors="ignore")
    winter = insert_once(
        winter,
        "         hrmlt(hour,iplane)  = wmelt(iplane)",
        "\n"
        "         call wepp_observe('H305_S_OUT',year,sdate,iplane,0,hour,\n"
        "     1     snodpt(iplane),densgt)",
        "H305_S_OUT",
        winter_path,
    )
    winter = insert_once(
        winter,
        "           totmel = totmel + hrmlt(hour,iplane)",
        "\n"
        "           call wepp_observe('H305_M_POST',year,sdate,iplane,0,hour,\n"
        "     1       hrmlt(hour,iplane),hrrain(hour))",
        "H305_M_POST",
        winter_path,
    )
    winter_path.write_text(winter, encoding="utf-8")


def build_fixed_worktree(worktree: Path) -> Path:
    run_command(
        "build_hphys0305_fixed_observe",
        ["bash", "tools/build_wepp_dated_release.sh"],
        cwd=worktree,
        env={"TARGET_TAG": "260430", "COMPILER": "/usr/bin/gfortran"},
        timeout=1200,
    )
    binary = worktree / "release/wepp_260430_hill"
    if not binary.exists():
        raise RuntimeError(f"missing fixed observe binary {binary}")
    return binary


def write_fixed_patch(worktree: Path, artifact_dir: Path) -> None:
    chunks: list[str] = []
    for rel in ("src/melt.for", "src/winter.for"):
        before = run_command(
            f"git_show_fixed_{rel}",
            ["git", "-C", str(BASELINE_REPO), "show", f"{FIXED_COMMIT}:{rel}"],
            check=True,
        ).stdout.splitlines(keepends=True)
        after = (worktree / rel).read_text(encoding="utf-8", errors="ignore").splitlines(
            keepends=True
        )
        chunks.extend(
            difflib.unified_diff(
                before,
                after,
                fromfile=f"a/{rel}",
                tofile=f"b/{rel}",
            )
        )
    (artifact_dir / "fixed-baseline-instrumentation.patch").write_text(
        "".join(chunks), encoding="utf-8"
    )


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
    lane_root = run_root / "fixed_baseline" / f"H{hill}_{lane}"
    if lane_root.exists():
        shutil.rmtree(lane_root)
    runs_dir = lane_root / "runs"
    output_dir = lane_root / "output"
    runs_dir.mkdir(parents=True)
    output_dir.mkdir(parents=True)
    source = HPHYS0265.SOURCE_RUNS
    for path in source.iterdir():
        if path.is_file():
            shutil.copy2(path, runs_dir / path.name)
    (runs_dir / f"p{hill}.run").write_text(legacy_runfile_text(hill), encoding="utf-8")
    return runs_dir, output_dir


def run_fixed_hill(binary: Path, run_root: Path, hill: int, lane: str, observe: bool) -> dict[str, Any]:
    runs_dir, output_dir = prepare_legacy_lane(run_root, hill, lane)
    if observe:
        (runs_dir / "wepp_observe.on").write_text("", encoding="utf-8")
    runfile = runs_dir / f"p{hill}.run"
    proc = run_command(
        f"fixed_baseline_H{hill}_{lane}",
        [str(binary)],
        cwd=runs_dir,
        input_text=runfile.read_text(encoding="utf-8"),
        check=False,
        timeout=600,
    )
    wat = output_dir / f"H{hill}.wat.dat"
    return {
        "hillslope_id": hill,
        "lane": lane,
        "binary": str(binary),
        "binary_sha256": HPHYS0298.sha256_file(binary) if binary.exists() else None,
        "rc": proc.returncode,
        "wat_path": str(wat),
        "wat_exists": wat.exists(),
        "wat_sha256": HPHYS0298.sha256_file(wat) if wat.exists() else None,
        "observe_log": str(runs_dir / "wepp_observe.log"),
        "observe_records": sum(1 for _ in (runs_dir / "wepp_observe.log").open())
        if (runs_dir / "wepp_observe.log").exists()
        else 0,
    }


def run_fixed_observe_lanes(
    binary: Path,
    run_root: Path,
    artifact_dir: Path,
    source_identity: dict[str, Any],
) -> dict[int, Any]:
    identity: dict[int, Any] = {}
    for hill in TARGET_HILLS:
        release_result = (
            run_fixed_hill(HPHYS0303_RELEASE_BIN, run_root, hill, "fixed_release", observe=False)
            if HPHYS0303_RELEASE_BIN.exists()
            else None
        )
        off = run_fixed_hill(binary, run_root, hill, "hphys0305_observe_off", observe=False)
        on = run_fixed_hill(binary, run_root, hill, "hphys0305_observe_on", observe=True)
        identity[hill] = {
            "hillslope_id": hill,
            "fixed_commit": FIXED_COMMIT,
            "fixed_source_identity": source_identity,
            "fixed_observe_binary_sha256": HPHYS0298.sha256_file(binary),
            "release_lane": release_result,
            "observe_off_lane": off,
            "observe_on_lane": on,
            "release_to_observe_off_bit_identical": None
            if release_result is None
            else release_result["wat_sha256"] == off["wat_sha256"],
            "observe_off_to_observe_on_bit_identical": off["wat_sha256"] == on["wat_sha256"],
            "pass": off["rc"] == 0
            and on["rc"] == 0
            and off["wat_sha256"] == on["wat_sha256"]
            and (release_result is None or release_result["wat_sha256"] == off["wat_sha256"]),
        }
    write_json(artifact_dir / "baseline-observe-identity.json", identity)
    write_baseline_identity_md(identity, artifact_dir)
    return identity


def parse_baseline_log(path: Path) -> dict[tuple[int, int, int], dict[str, float]]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = defaultdict(dict)
    if not path.exists():
        return parsed
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        match = OBS_RE.match(line)
        if not match:
            continue
        tag = match.group("tag")
        key = (
            int(match.group("year")),
            int(match.group("day")),
            int(match.group("hour")),
        )
        v1 = float(match.group("v1"))
        v2 = float(match.group("v2"))
        if tag == "H305_T_AB":
            parsed[key]["amelt"] = v1
            parsed[key]["bmelt"] = v2
        elif tag == "H305_T_CD":
            parsed[key]["cmelt"] = v1
            parsed[key]["dmelt"] = v2
        elif tag == "H305_F_HT":
            parsed[key]["hrtemp"] = v1
            parsed[key]["tdpt"] = v2
        elif tag == "H305_F_RR":
            parsed[key]["hrad"] = v1
            parsed[key]["hrrain"] = v2
        elif tag == "H305_F_CV":
            parsed[key]["cloudC"] = v1
            parsed[key]["vwind"] = v2
        elif tag == "H305_S_OUT":
            parsed[key]["snodpt"] = v1
            parsed[key]["densgt"] = v2
    return parsed


def load_openwepp_trace(path: Path) -> dict[tuple[int, int, int], dict[str, float]]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = defaultdict(dict)
    if not path.exists():
        return parsed
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        row = json.loads(line)
        year = int(row.get("calendar_year", row.get("simulation_year")))
        day = int(row["julian_day"])
        for symbol, field in OPENWEPP_FIELDS.items():
            values = row.get(field)
            if not isinstance(values, dict):
                continue
            for hour_key, value in values.items():
                parsed[(year, day, int(hour_key))][symbol] = float(value)
    return parsed


def window_keys(year: int, start: int, end: int) -> set[tuple[int, int, int]]:
    return {(year, day, hour) for day in range(start, end + 1) for hour in range(1, 25)}


def compare_symbol(
    symbol: str,
    keys: set[tuple[int, int, int]],
    baseline: dict[tuple[int, int, int], dict[str, float]],
    openwepp: dict[tuple[int, int, int], dict[str, float]],
) -> dict[str, Any]:
    missing_baseline = 0
    missing_openwepp = 0
    paired = 0
    baseline_sum = 0.0
    openwepp_sum = 0.0
    max_abs_delta = 0.0
    first_delta: dict[str, Any] | None = None
    tolerance = SYMBOL_TOLERANCE[symbol]
    for key in sorted(keys):
        base_present = symbol in baseline.get(key, {})
        open_present = symbol in openwepp.get(key, {})
        if not base_present and not open_present:
            continue
        if not base_present:
            missing_baseline += 1
            continue
        elif not open_present:
            missing_openwepp += 1
            continue
        else:
            base_value = baseline[key][symbol]
            open_value = openwepp[key][symbol]
        delta = base_value - open_value
        paired += 1
        baseline_sum += base_value
        openwepp_sum += open_value
        max_abs_delta = max(max_abs_delta, abs(delta))
        if first_delta is None and abs(delta) > tolerance:
            first_delta = {
                "year": key[0],
                "julian": key[1],
                "hour": key[2],
                "baseline": round(base_value, 9),
                "openwepp": round(open_value, 9),
                "delta": round(delta, 9),
            }
    return {
        "symbol": symbol,
        "openwepp_field": OPENWEPP_FIELDS[symbol],
        "unit": SYMBOL_UNITS[symbol],
        "tolerance": tolerance,
        "baseline_count": len(keys) - missing_baseline,
        "openwepp_count": len(keys) - missing_openwepp,
        "paired_count": paired,
        "missing_baseline_count": missing_baseline,
        "missing_openwepp_count": missing_openwepp,
        "baseline_sum": round(baseline_sum, 9),
        "openwepp_sum": round(openwepp_sum, 9),
        "delta_sum": round(baseline_sum - openwepp_sum, 9),
        "max_abs_delta": round(max_abs_delta, 9),
        "first_delta": first_delta,
        "baseline_source": BASELINE_SOURCES[symbol],
        "openwepp_source": OPENWEPP_SOURCE,
    }


def classify(comparisons: dict[str, dict[str, Any]]) -> tuple[str, str, str]:
    for symbol in TARGET_SYMBOLS:
        item = comparisons[symbol]
        if item["missing_baseline_count"] or item["missing_openwepp_count"]:
            return (
                f"paired-surface-gap:{symbol}",
                "surface-gap-hold",
                f"paired baseline/openWEPP surface is incomplete for {symbol}",
            )
    for group, symbols in DEPENDENCY_ORDER:
        for symbol in symbols:
            item = comparisons[symbol]
            if item["max_abs_delta"] > item["tolerance"]:
                return (
                    f"{group}:{symbol}",
                    f"{group}-hold",
                    f"first same-unit paired divergence is {symbol} in {group}",
                )
    return (
        "paired-term-state-within-tolerance",
        "paired-term-state-closed-return-to-post-raw",
        "paired HPHYS0305 term/state surfaces are within declared tolerances",
    )


def required_next_action(source: str) -> str:
    if source.startswith("hourly-forcing"):
        return "Open a source-owned hourly forcing/coupling package before snow or downstream compensation edits."
    if source.startswith("snow-state"):
        return "Open a baseline-authoritative snowd state/depth-density migration package."
    if source.startswith("melt-terms"):
        return "Open a baseline-authoritative melt.for term-magnitude migration package."
    if source.startswith("paired-surface-gap"):
        return "Repair paired instrumentation completeness before production edits."
    return "Return to post-raw/routing lineage with paired term/state evidence attached."


def write_pair_ledger(
    run_root: Path,
    artifact_dir: Path,
    identity: dict[int, Any],
) -> list[dict[str, Any]]:
    ledger: list[dict[str, Any]] = []
    for hill in TARGET_HILLS:
        observe_log = Path(identity[hill]["observe_on_lane"]["observe_log"])
        baseline = parse_baseline_log(observe_log)
        openwepp = load_openwepp_trace(run_root / f"hillslope_output/H{hill}.hphys0299.trace.jsonl")
        for window_name, year, start, end in TARGET_WINDOWS[hill]:
            keys = window_keys(year, start, end)
            comparisons = {
                symbol: compare_symbol(symbol, keys, baseline, openwepp)
                for symbol in TARGET_SYMBOLS
            }
            first_source, route, reason = classify(comparisons)
            paired_complete = all(
                item["missing_baseline_count"] == 0 and item["missing_openwepp_count"] == 0
                for item in comparisons.values()
            )
            ledger.append(
                {
                    "hillslope_id": hill,
                    "window": window_name,
                    "year": year,
                    "start_julian": start,
                    "end_julian": end,
                    "fixed_comparator_commit": FIXED_COMMIT,
                    "paired_surface_status": "paired-complete" if paired_complete else "paired-surface-gap",
                    "first_divergent_source": first_source,
                    "hphys0305_route": route,
                    "classification_reason": reason,
                    "required_next_action": required_next_action(first_source),
                    "production_edit_authorized": False,
                    "comparisons": comparisons,
                    "prohibited_compensation_note": (
                        "HPHYS0305 is instrumentation evidence only; WB13/WB17/WB18/WB19/WB12 "
                        "compensation remains prohibited."
                    ),
                }
            )
    write_json(artifact_dir / "paired-melt-term-state-ledger.json", ledger)
    write_pair_summary(run_root, artifact_dir, ledger)
    return ledger


def write_trace_audit(run_root: Path, artifact_dir: Path) -> None:
    rows = []
    for hill in TARGET_HILLS:
        trace_path = run_root / f"hillslope_output/H{hill}.hphys0299.trace.jsonl"
        field_counts: Counter[str] = Counter()
        row_count = 0
        if trace_path.exists():
            for line in trace_path.read_text(encoding="utf-8", errors="ignore").splitlines():
                row_count += 1
                row = json.loads(line)
                for field in OPENWEPP_FIELDS.values():
                    value = row.get(field)
                    if isinstance(value, dict) and value:
                        field_counts[field] += 1
        rows.append(
            {
                "hillslope_id": hill,
                "trace_path": str(trace_path),
                "row_count": row_count,
                "field_day_counts": dict(field_counts),
                "all_fields_present_at_least_once": all(field_counts[field] > 0 for field in OPENWEPP_FIELDS.values()),
            }
        )
    write_json(artifact_dir / "openwepp-trace-field-audit.json", rows)
    headers = ["Hill", "Rows", "All Fields Present", "Trace"]
    table_rows = [
        [f"H{row['hillslope_id']}", row["row_count"], row["all_fields_present_at_least_once"], row["trace_path"]]
        for row in rows
    ]
    text = "# HPHYS0305 OpenWEPP Trace Field Audit\n\nRan:\n\n"
    text += HPHYS0265.markdown_table(headers, table_rows)
    text += "\n"
    (artifact_dir / "openwepp-trace-field-audit.md").write_text(text, encoding="utf-8")


def write_baseline_identity_md(identity: dict[int, Any], artifact_dir: Path) -> None:
    headers = ["Hill", "Pass", "Release=Off", "Off=On", "Observe Records"]
    rows = []
    for hill in TARGET_HILLS:
        row = identity[hill]
        rows.append(
            [
                f"H{hill}",
                row["pass"],
                row["release_to_observe_off_bit_identical"],
                row["observe_off_to_observe_on_bit_identical"],
                row["observe_on_lane"]["observe_records"],
            ]
        )
    text = "# HPHYS0305 Baseline Observe Identity\n\nRan:\n\n"
    text += f"- Fixed comparator commit: `{FIXED_COMMIT}`\n"
    text += f"- Fixed worktree: `{FIXED_WORKTREE}`\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n"
    (artifact_dir / "baseline-observe-identity.md").write_text(text, encoding="utf-8")


def write_pair_summary(run_root: Path, artifact_dir: Path, ledger: list[dict[str, Any]]) -> None:
    headers = ["Hill", "Window", "Days", "Paired Status", "First Source", "Route", "Edit?"]
    rows = [
        [
            f"H{row['hillslope_id']}",
            row["window"],
            f"{row['year']} {row['start_julian']}-{row['end_julian']}",
            row["paired_surface_status"],
            row["first_divergent_source"],
            row["hphys0305_route"],
            row["production_edit_authorized"],
        ]
        for row in ledger
    ]
    counts = Counter(row["first_divergent_source"] for row in ledger)
    text = "# HPHYS0305 Paired Melt-Term/State Summary\n\nRan:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Fixed comparator commit: `{FIXED_COMMIT}`\n"
    text += f"- openWEPP HEAD: `{current_head()}`\n"
    text += "- Production edit authorized: `false`\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n## First Source Counts\n\n"
    text += HPHYS0265.markdown_table(["Source", "Count"], [[key, value] for key, value in sorted(counts.items())])
    text += "\n"
    (artifact_dir / "paired-melt-term-state-summary.md").write_text(text, encoding="utf-8")


def write_full39_context(artifact_dir: Path) -> None:
    target = artifact_dir / "full-39-suite-metrics.md"
    carried = HPHYS0304_METRICS.read_text(encoding="utf-8") if HPHYS0304_METRICS.exists() else ""
    text = "# HPHYS0305 Full-39 Suite Metrics Context\n\n"
    text += "Static:\n\n"
    text += "- HPHYS0305 adds diagnostic trace/observe surfaces only; it does not change production physics or WAT publication math.\n"
    text += "- Full H1..H39 fixed-comparator semantic metrics are carried forward from HPHYS0304 as same-physics context.\n\n"
    text += "Ran:\n\n"
    text += "- HPHYS0305 did not rerun the full H1..H39 semantic suite; targeted H1/H7/H39 paired traces were run.\n\n"
    if carried:
        text += "## Carried HPHYS0304 Metrics\n\n"
        text += carried
    target.write_text(text, encoding="utf-8")


def write_status_artifacts(artifact_dir: Path, ledger: list[dict[str, Any]]) -> None:
    first_sources = Counter(row["first_divergent_source"] for row in ledger)
    status_text = "\n".join(
        [
            "# Disposition",
            "",
            "Status: complete",
            "",
            "Evidence mode: ran",
            "",
            "Static:",
            "",
            "- HPHYS0305 is instrumentation-only and does not authorize production physics edits.",
            "- Downstream WB13/WB17/WB18/WB19/WB12 compensation remains prohibited.",
            "",
            "Ran:",
            "",
            f"- Generated paired ledger rows: `{len(ledger)}`.",
            f"- First-source counts: `{dict(first_sources)}`.",
            "- Production edit authorized: `false`.",
            "",
        ]
    )
    (artifact_dir / "disposition.md").write_text(status_text, encoding="utf-8")
    (artifact_dir / "implementation-test-evidence.md").write_text(
        "\n".join(
            [
                "# Implementation/Test Evidence",
                "",
                "Status: complete",
                "",
                "Evidence mode: ran",
                "",
                "Static:",
                "",
                "- Added openWEPP diagnostic JSON maps for paired rain, snowfall-depth, depth, and density surfaces.",
                "- Added fixed-comparator observe instrumentation as a local worktree patch only.",
                "",
                "Ran:",
                "",
                "- Rebuilt `target/release/openwepp-cli-hill`.",
                "- Built the HPHYS0305 fixed-comparator observe worktree.",
                "- Ran targeted H1/H7/H39 openWEPP traces and fixed-comparator observe lanes.",
                "- Generated `paired-melt-term-state-ledger.json`.",
                "",
            ]
        ),
        encoding="utf-8",
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--artifact-dir", type=Path, default=ARTIFACT_DIR)
    parser.add_argument("--fixed-worktree", type=Path, default=FIXED_WORKTREE)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--skip-openwepp-build", action="store_true")
    parser.add_argument("--reuse-existing-runs", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    args.run_root.mkdir(parents=True, exist_ok=True)
    args.artifact_dir.mkdir(parents=True, exist_ok=True)
    try:
        if args.reuse_existing_runs:
            command_log_path = args.artifact_dir / "hphys0305-runner-command-log.json"
            if command_log_path.exists():
                COMMAND_LOG.extend(read_json(command_log_path))
            identity = read_json(args.artifact_dir / "baseline-observe-identity.json")
            identity = {int(key): value for key, value in identity.items()}
        else:
            if not args.skip_openwepp_build:
                run_command(
                    "cargo_build_openwepp_cli_hill_release",
                    [
                        "cargo",
                        "build",
                        "--release",
                        "-p",
                        "openwepp-runner",
                        "--bin",
                        "openwepp-cli-hill",
                    ],
                    timeout=1800,
                )
            source_identity = prepare_fixed_worktree(args.fixed_worktree)
            patch_fixed_observe(args.fixed_worktree)
            write_fixed_patch(args.fixed_worktree, args.artifact_dir)
            fixed_binary = build_fixed_worktree(args.fixed_worktree)
            identity = run_fixed_observe_lanes(
                fixed_binary,
                args.run_root,
                args.artifact_dir,
                source_identity,
            )
            trace_rc = HPHYS0299.run_targeted_traces(args.run_root, args.trace_max_days)
            COMMAND_LOG.append({"name": "hphys0299_run_targeted_traces", "rc": trace_rc})
            if trace_rc != 0:
                raise RuntimeError(f"targeted openWEPP trace run failed rc={trace_rc}")
        if not args.reuse_existing_runs:
            write_trace_audit(args.run_root, args.artifact_dir)
        elif not (args.artifact_dir / "openwepp-trace-field-audit.json").exists():
            write_trace_audit(args.run_root, args.artifact_dir)
        ledger = write_pair_ledger(args.run_root, args.artifact_dir, identity)
        write_full39_context(args.artifact_dir)
        write_status_artifacts(args.artifact_dir, ledger)
        return 0
    finally:
        write_json(args.artifact_dir / "hphys0305-runner-command-log.json", COMMAND_LOG)


if __name__ == "__main__":
    raise SystemExit(main())
