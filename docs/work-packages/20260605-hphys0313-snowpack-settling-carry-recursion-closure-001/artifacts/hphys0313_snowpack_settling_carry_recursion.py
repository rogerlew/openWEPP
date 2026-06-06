#!/usr/bin/env python3
"""Run HPHYS0313 split-route snowpack settling/carry recursion diagnostics."""

from __future__ import annotations

import argparse
import difflib
import hashlib
import json
import math
import os
import re
import shutil
import subprocess
import time
from collections import defaultdict
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
BASELINE_REPO = Path("/workdir/wepp-forest_260430_baseline")
FIXED_COMMIT = "47ac4c32faeea81bb99081f955a14c38b815ef4d"
FIXED_WORKTREE = Path("/tmp/hphys0313_wepp_260430_snowpack_settling")
RUN_ROOT = Path("/tmp/hphys0313_snowpack_settling_carry_recursion")
SOURCE_RUNS = Path("/tmp/unpalatable_parity_20260529T192707Z/runs")
HPHYS0312_LEDGER = (
    REPO
    / "docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/prior-year-terminal-snowpack-lineage-ledger.json"
)
HPHYS0305_ARTIFACT_DIR = (
    REPO
    / "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts"
)
TARGET_HILLS = (1, 7, 39)
MATERIAL_DEPTH_TOL_M = 0.0005
MATERIAL_DENSITY_TOL_KG_M3 = 0.5
SETTLING_RECON_TOL = 1.0e-7
OBS_RE = re.compile(
    r"^(?P<tag>\S+)\s+y=\s*(?P<year>-?\d+)\s+d=\s*(?P<day>-?\d+)"
    r"\s+e=\s*(?P<element>-?\d+)\s+c=\s*(?P<chan>-?\d+)"
    r"\s+s=\s*(?P<hour>-?\d+)\s+v1=\s*(?P<v1>[-+0-9.Ee]+)"
    r"\s+v2=\s*(?P<v2>[-+0-9.Ee]+)"
)
SOURCE_LINE_REQUIREMENTS = [
    (BASELINE_REPO / "src/snowd.for", 61, "if (hour .eq. 1)", "snowd.for:61-65"),
    (BASELINE_REPO / "src/snowd.for", 65, "if (hrsnow(hour) .gt. 0.0)", "snowd.for:61-65"),
    (BASELINE_REPO / "src/snowd.for", 122, "if (snodpt(iplane) .gt. 0.0)", "snowd.for:122-139"),
    (BASELINE_REPO / "src/snowd.for", 125, "setf = ((exp(-(float(wdayct(iplane)) * 2.0)))", "snowd.for:125-126"),
    (BASELINE_REPO / "src/snowd.for", 129, "if(densgy.gt.ssd) setf = 1", "snowd.for:129"),
    (BASELINE_REPO / "src/snowd.for", 131, "densgt = densgy * setf", "snowd.for:131"),
    (BASELINE_REPO / "src/snowd.for", 135, "if (densgt .gt. 522) densgt = 522", "snowd.for:135"),
    (BASELINE_REPO / "src/snowd.for", 139, "snodpt(iplane) = snodpt(iplane) * densgy/ densgt", "snowd.for:139"),
    (BASELINE_REPO / "src/snowd.for", 145, "if (hrsnow(hour) .le. 0.0) then", "snowd.for:145-146"),
    (BASELINE_REPO / "src/snowd.for", 146, "snodep = snodpt(iplane) + driftg", "snowd.for:145-146"),
    (BASELINE_REPO / "src/snowd.for", 167, "snodep = snodpt(iplane) + hrsnow(hour)+driftf+driftg", "snowd.for:166-172"),
    (BASELINE_REPO / "src/winter.for", 366, "call snowd(iresd(1,iplane)", "winter.for:366-367"),
    (BASELINE_REPO / "src/snowd.for", 311, "snodpy(iplane) = snodep", "snowd.for:310-312"),
    (REPO / "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs", 3872, "if hour == 1", "03_kernel_support_00_support_helpers.rs:3872-3877"),
    (REPO / "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs", 3901, "let mut setf", "03_kernel_support_00_support_helpers.rs:3901-3905"),
    (REPO / "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs", 3906, "densgt = dens * setf", "03_kernel_support_00_support_helpers.rs:3906-3912"),
    (REPO / "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs", 3911, "snodpt = snodpt * dens / densgt", "03_kernel_support_00_support_helpers.rs:3906-3912"),
    (REPO / "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs", 3914, "if hrsnow <= WB11_ZERO_THRESHOLD", "03_kernel_support_00_support_helpers.rs:3914-3924"),
    (REPO / "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs", 3918, "snodep = snodpt + hrsnow", "03_kernel_support_00_support_helpers.rs:3914-3924"),
]
COMMAND_LOG: list[dict[str, Any]] = []


class SourceLineEvidenceError(RuntimeError):
    """Raised when required canonical source-line evidence is absent."""


class PairedEvidenceError(RuntimeError):
    """Raised when required paired baseline/openWEPP state evidence is absent."""


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


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
    merged_env = {**os.environ, **env} if env else None
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


def verify_source_lines(requirements: list[tuple[Path, int, str, str]] | None = None) -> dict[str, list[str]]:
    citations: dict[str, list[str]] = defaultdict(list)
    for path, line_number, needle, citation in requirements or SOURCE_LINE_REQUIREMENTS:
        if not path.exists():
            raise SourceLineEvidenceError(f"missing source file for {citation}: {path}")
        lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        if len(lines) < line_number:
            raise SourceLineEvidenceError(f"missing source line for {citation}: {path}:{line_number}")
        observed = lines[line_number - 1]
        if needle not in observed:
            raise SourceLineEvidenceError(
                f"source line mismatch for {citation}: expected {needle!r} at {path}:{line_number}"
            )
        citations[citation].append(f"{path}:{line_number}")
    return dict(citations)


def insert_once(text: str, needle: str, addition: str, marker: str, path: Path) -> str:
    if marker in text:
        return text
    if needle not in text:
        raise RuntimeError(f"missing insertion point in {path}: {needle!r}")
    return text.replace(needle, needle + addition, 1)


def prepare_fixed_worktree() -> None:
    if FIXED_WORKTREE.exists():
        shutil.rmtree(FIXED_WORKTREE)
    FIXED_WORKTREE.mkdir(parents=True, exist_ok=True)
    run_command(
        "git_archive_hphys0313_fixed_commit",
        ["bash", "-lc", f"git -C {BASELINE_REPO} archive {FIXED_COMMIT} | tar -x -C {FIXED_WORKTREE}"],
    )


def patch_fixed_observe() -> None:
    snowd_path = FIXED_WORKTREE / "src/snowd.for"
    winter_path = FIXED_WORKTREE / "src/winter.for"
    observe_path = FIXED_WORKTREE / "src/wepp_observe.for"
    snowd = snowd_path.read_text(encoding="utf-8", errors="ignore")
    snowd = snowd.replace(
        "      subroutine snowd(irtype,denh2o,iplane,driftf,driftg,snodep,\n"
        "     1                 densgy,densgt,smelt,hour)",
        "      subroutine snowd(irtype,denh2o,iplane,driftf,driftg,snodep,\n"
        "     1                 densgy,densgt,smelt,hour,obs_year,\n"
        "     2                 obs_sdate)",
        1,
    )
    snowd = snowd.replace(
        "      integer irtype,iplane,hour",
        "      integer irtype,iplane,hour,obs_year,obs_sdate",
        1,
    )
    snowd = insert_once(
        snowd,
        "            setf = ((exp(-(float(wdayct(iplane)) * 2.0)))*0.0416667)\n"
        "     1             +1.0",
        "\n"
        "            call wepp_observe('H313_WDAY',obs_year,obs_sdate,\n"
        "     1        iplane,0,hour,\n"
        "     1        float(wdayct(iplane)),hrsnow(hour))\n"
        "            call wepp_observe('H313_PRE',obs_year,obs_sdate,\n"
        "     1        iplane,0,hour,\n"
        "     1        snodpt(iplane),densgy)",
        "H313_WDAY",
        snowd_path,
    )
    snowd = insert_once(
        snowd,
        "            densgt = densgy * setf",
        "\n"
        "            call wepp_observe('H313_SETF',obs_year,obs_sdate,\n"
        "     1        iplane,0,hour,\n"
        "     1        setf,float(wdayct(iplane)))",
        "H313_SETF",
        snowd_path,
    )
    snowd = insert_once(
        snowd,
        "            if (densgt .gt. 522) densgt = 522",
        "\n"
        "            call wepp_observe('H313_RAW',obs_year,obs_sdate,\n"
        "     1        iplane,0,hour,\n"
        "     1        densgy,densgt)",
        "H313_RAW",
        snowd_path,
    )
    snowd = insert_once(
        snowd,
        "              snodpt(iplane) = snodpt(iplane) * densgy/ densgt",
        "\n"
        "              call wepp_observe('H313_POST',obs_year,obs_sdate,\n"
        "     1          iplane,0,hour,\n"
        "     1          snodpt(iplane),densgt)",
        "H313_POST",
        snowd_path,
    )
    snowd = insert_once(
        snowd,
        "          endif\nc     End Loop M3\nc    ",
        "\n"
        "          call wepp_observe('H313_FINAL',obs_year,obs_sdate,\n"
        "     1      iplane,0,hour,\n"
        "     1      snodep,densgt)",
        "H313_FINAL",
        snowd_path,
    )
    snowd_path.write_text(snowd, encoding="utf-8")

    winter = winter_path.read_text(encoding="utf-8", errors="ignore")
    winter = winter.replace(
        "        call snowd(iresd(1,iplane),denh2o,iplane,driftf,driftg,\n"
        "     1             snodep,densgy,densgt,smelt,hour)",
        "        call snowd(iresd(1,iplane),denh2o,iplane,driftf,driftg,\n"
        "     1             snodep,densgy,densgt,smelt,hour,year,sdate)",
        1,
    )
    winter_path.write_text(winter, encoding="utf-8")

    observe = observe_path.read_text(encoding="utf-8", errors="ignore")
    observe = observe.replace("' v1=',1pe12.4,' v2=',1pe12.4", "' v1=',1pe24.16,' v2=',1pe24.16")
    observe_path.write_text(observe, encoding="utf-8")


def write_fixed_patch() -> None:
    chunks: list[str] = []
    for rel in ("src/snowd.for", "src/winter.for", "src/wepp_observe.for"):
        before = run_command(
            f"git_show_fixed_{rel}",
            ["git", "-C", str(BASELINE_REPO), "show", f"{FIXED_COMMIT}:{rel}"],
        ).stdout.splitlines(keepends=True)
        after = (FIXED_WORKTREE / rel).read_text(encoding="utf-8", errors="ignore").splitlines(keepends=True)
        chunks.extend(difflib.unified_diff(before, after, fromfile=f"a/{rel}", tofile=f"b/{rel}"))
    (ARTIFACT_DIR / "fixed-baseline-settling-instrumentation.patch").write_text(
        "".join(chunks), encoding="utf-8"
    )


def build_fixed_worktree() -> Path:
    run_command(
        "build_hphys0313_fixed_observe",
        ["bash", "tools/build_wepp_dated_release.sh"],
        cwd=FIXED_WORKTREE,
        env={"TARGET_TAG": "260430", "COMPILER": "/usr/bin/gfortran"},
        timeout=1200,
    )
    binary = FIXED_WORKTREE / "release/wepp_260430_hill"
    if not binary.exists():
        raise RuntimeError(f"missing fixed observe binary {binary}")
    return binary


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


def prepare_legacy_lane(hill: int, lane: str) -> tuple[Path, Path]:
    lane_root = RUN_ROOT / "fixed_baseline" / f"H{hill}_{lane}"
    if lane_root.exists():
        shutil.rmtree(lane_root)
    runs_dir = lane_root / "runs"
    output_dir = lane_root / "output"
    runs_dir.mkdir(parents=True, exist_ok=True)
    output_dir.mkdir(parents=True, exist_ok=True)
    if not SOURCE_RUNS.exists():
        raise FileNotFoundError(f"missing source runs directory: {SOURCE_RUNS}")
    for path in SOURCE_RUNS.iterdir():
        if path.is_file():
            shutil.copy2(path, runs_dir / path.name)
    (runs_dir / f"p{hill}.run").write_text(legacy_runfile_text(hill), encoding="utf-8")
    return runs_dir, output_dir


def run_fixed_hill(binary: Path, hill: int, lane: str, observe: bool) -> dict[str, Any]:
    runs_dir, output_dir = prepare_legacy_lane(hill, lane)
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
    observe_log = runs_dir / "wepp_observe.log"
    return {
        "hillslope_id": hill,
        "lane": lane,
        "binary": str(binary),
        "binary_sha256": sha256_file(binary) if binary.exists() else None,
        "rc": proc.returncode,
        "wat_path": str(wat),
        "wat_exists": wat.exists(),
        "wat_sha256": sha256_file(wat) if wat.exists() else None,
        "observe_log": str(observe_log),
        "observe_records": sum(1 for _ in observe_log.open()) if observe_log.exists() else 0,
    }


def run_fixed_observe_lanes(binary: Path) -> dict[str, Any]:
    if RUN_ROOT.exists():
        shutil.rmtree(RUN_ROOT)
    RUN_ROOT.mkdir(parents=True, exist_ok=True)
    identity: dict[str, Any] = {}
    for hill in TARGET_HILLS:
        off = run_fixed_hill(binary, hill, "hphys0313_observe_off", observe=False)
        on = run_fixed_hill(binary, hill, "hphys0313_observe_on", observe=True)
        identity[str(hill)] = {
            "hillslope_id": hill,
            "fixed_commit": FIXED_COMMIT,
            "fixed_observe_binary_sha256": sha256_file(binary),
            "observe_off_lane": off,
            "observe_on_lane": on,
            "observe_off_to_observe_on_bit_identical": off["wat_sha256"] == on["wat_sha256"],
            "pass": off["rc"] == 0 and on["rc"] == 0 and off["wat_sha256"] == on["wat_sha256"],
        }
    write_json(ARTIFACT_DIR / "fixed-baseline-settling-observe-identity.json", identity)
    return identity


def parse_h305_log(path: Path) -> dict[tuple[int, int, int], dict[str, float]]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = defaultdict(dict)
    if not path.exists():
        raise FileNotFoundError(f"missing baseline observe log: {path}")
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        match = OBS_RE.match(line)
        if not match:
            continue
        tag = match.group("tag")
        key = (int(match.group("year")), int(match.group("day")), int(match.group("hour")))
        v1 = float(match.group("v1"))
        v2 = float(match.group("v2"))
        if tag == "H305_S_OUT":
            parsed[key]["depth_after_m"] = v1
            parsed[key]["density_after_kg_m3"] = v2
    return parsed


def parse_h313_log(path: Path) -> dict[tuple[int, int, int], dict[str, float]]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = defaultdict(dict)
    if not path.exists():
        raise FileNotFoundError(f"missing HPHYS0313 observe log: {path}")
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        match = OBS_RE.match(line)
        if not match:
            continue
        tag = match.group("tag")
        key = (int(match.group("year")), int(match.group("day")), int(match.group("hour")))
        v1 = float(match.group("v1"))
        v2 = float(match.group("v2"))
        if tag == "H313_WDAY":
            parsed[key]["wdayct"] = v1
            parsed[key]["hrsnow_m"] = v2
        elif tag == "H313_PRE":
            parsed[key]["depth_before_settling_m"] = v1
            parsed[key]["densgy_before_kg_m3"] = v2
        elif tag == "H313_SETF":
            parsed[key]["setf"] = v1
            parsed[key]["wdayct_at_setf"] = v2
        elif tag == "H313_RAW":
            parsed[key]["densgy_raw_kg_m3"] = v1
            parsed[key]["densgt_after_setf_kg_m3"] = v2
        elif tag == "H313_POST":
            parsed[key]["depth_after_settling_m"] = v1
            parsed[key]["densgt_after_kg_m3"] = v2
        elif tag == "H313_FINAL":
            parsed[key]["depth_after_cold_branch_m"] = v1
            parsed[key]["density_after_cold_branch_kg_m3"] = v2
    return parsed


def load_trace_rows(path: Path) -> dict[tuple[int, int], dict[str, Any]]:
    parsed: dict[tuple[int, int], dict[str, Any]] = {}
    if not path.exists():
        raise FileNotFoundError(f"missing openWEPP trace: {path}")
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        row = json.loads(line)
        year = int(row.get("calendar_year", row.get("simulation_year")))
        day = int(row["julian_day"])
        parsed[(year, day)] = row
    return parsed


def baseline_log_paths_by_hillslope() -> dict[int, Path]:
    identity = read_json(HPHYS0305_ARTIFACT_DIR / "baseline-observe-identity.json")
    return {int(hill): Path(row["observe_on_lane"]["observe_log"]) for hill, row in identity.items()}


def trace_paths_by_hillslope() -> dict[int, Path]:
    audit = read_json(HPHYS0305_ARTIFACT_DIR / "openwepp-trace-field-audit.json")
    return {int(row["hillslope_id"]): Path(row["trace_path"]) for row in audit}


def h313_log_paths_by_hillslope(identity: dict[str, Any]) -> dict[int, Path]:
    return {int(hill): Path(row["observe_on_lane"]["observe_log"]) for hill, row in identity.items()}


def hour_value(row: dict[str, Any], field: str, hour: int) -> float:
    values = row.get(field)
    if not isinstance(values, dict):
        raise PairedEvidenceError(f"missing openWEPP hourly field {field}")
    key = f"{hour:04d}"
    if key not in values:
        raise PairedEvidenceError(f"missing openWEPP hourly field {field}[{key}]")
    return float(values[key])


def paired_state(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
    year: int,
    day: int,
    hour: int,
) -> dict[str, Any]:
    base = baseline.get((year, day, hour))
    if base is None or "depth_after_m" not in base or "density_after_kg_m3" not in base:
        raise PairedEvidenceError(f"missing baseline snow state y={year} d={day} h={hour}")
    row = traces.get((year, day))
    if row is None:
        raise PairedEvidenceError(f"missing openWEPP trace row y={year} d={day}")
    open_depth = hour_value(row, "snow_hourly_depth_after_m", hour)
    open_density = hour_value(row, "snow_hourly_density_after_kg_m3", hour)
    depth_delta = open_depth - base["depth_after_m"]
    density_delta = open_density - base["density_after_kg_m3"]
    return {
        "year": year,
        "julian": day,
        "hour": hour,
        "baseline_depth_after_m": base["depth_after_m"],
        "baseline_density_after_kg_m3": base["density_after_kg_m3"],
        "openwepp_depth_before_m": hour_value(row, "snow_hourly_depth_before_m", hour),
        "openwepp_depth_after_m": open_depth,
        "openwepp_density_before_kg_m3": hour_value(row, "snow_hourly_density_before_kg_m3", hour),
        "openwepp_density_after_kg_m3": open_density,
        "openwepp_snowfall_depth_m": hour_value(row, "snow_hourly_snowfall_depth_m", hour),
        "openwepp_raw_melt_m": hour_value(row, "snow_hourly_melt_raw_m", hour),
        "openwepp_routed_melt_m": hour_value(row, "snow_hourly_melt_m", hour),
        "openwepp_rain_m": hour_value(row, "snow_hourly_rain_m", hour),
        "openwepp_air_temp_c": hour_value(row, "winter_hourly_air_temp_c", hour),
        "openwepp_melt_branch_active": hour_value(row, "snow_hourly_melt_branch_active", hour),
        "depth_delta_openwepp_minus_baseline_m": depth_delta,
        "density_delta_openwepp_minus_baseline_kg_m3": density_delta,
        "material_depth_divergent": abs(depth_delta) > MATERIAL_DEPTH_TOL_M,
        "material_density_divergent": abs(density_delta) > MATERIAL_DENSITY_TOL_KG_M3,
    }


def max_day_for_year(year: int) -> int:
    return 366 if year % 4 == 0 else 365


def scan_year(
    baseline: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
    year: int,
) -> tuple[dict[str, Any], dict[str, Any] | None, dict[str, Any]]:
    previous_key: tuple[int, int, int] | None = None
    for day in range(1, max_day_for_year(year) + 1):
        for hour in range(1, 25):
            state = paired_state(baseline, traces, year, day, hour)
            if state["material_depth_divergent"] or state["material_density_divergent"]:
                previous = paired_state(baseline, traces, *previous_key) if previous_key else None
                terminal = paired_state(baseline, traces, year, max_day_for_year(year), 24)
                return state, previous, terminal
            previous_key = (year, day, hour)
    terminal = paired_state(baseline, traces, year, max_day_for_year(year), 24)
    return terminal, terminal, terminal


def reconstruct_settling_key(
    key: tuple[int, int, int],
    h313: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
) -> dict[str, Any]:
    record = h313.get(key)
    required = {
        "wdayct",
        "hrsnow_m",
        "depth_before_settling_m",
        "densgy_before_kg_m3",
        "setf",
        "wdayct_at_setf",
        "densgt_after_setf_kg_m3",
        "depth_after_settling_m",
        "densgt_after_kg_m3",
    }
    if record is None or not required.issubset(record):
        missing = sorted(required - set(record or {}))
        raise PairedEvidenceError(f"missing HPHYS0313 settling fields for {key}: {missing}")
    row = traces.get((key[0], key[1]))
    if row is None:
        raise PairedEvidenceError(f"missing openWEPP trace row for settling key {key}")
    baseline_expected_setf = math.exp(-(record["wdayct"] * 2.0)) * 0.0416667 + 1.0
    if record["densgy_before_kg_m3"] > 250.0:
        baseline_expected_setf = 1.0
    baseline_expected_density = min(record["densgy_before_kg_m3"] * baseline_expected_setf, 522.0)
    baseline_expected_depth = (
        record["depth_before_settling_m"] * record["densgy_before_kg_m3"] / record["densgt_after_kg_m3"]
        if record["densgt_after_kg_m3"] != 0.0
        else 0.0
    )
    open_settle_day_count = float(row.get("snow_runtime_settle_day_count", record["wdayct"]))
    open_expected_setf = math.exp(-(open_settle_day_count * 2.0)) * 0.0416667 + 1.0
    open_density_before = hour_value(row, "snow_hourly_density_before_kg_m3", key[2])
    open_depth_before = hour_value(row, "snow_hourly_depth_before_m", key[2])
    open_density_after = hour_value(row, "snow_hourly_density_after_kg_m3", key[2])
    open_depth_after = hour_value(row, "snow_hourly_depth_after_m", key[2])
    open_expected_density = min(open_density_before * open_expected_setf, 522.0)
    open_expected_depth = open_depth_before * open_density_before / open_expected_density if open_expected_density != 0.0 else 0.0
    baseline_residual = baseline_expected_depth - record["depth_after_settling_m"]
    open_residual = open_expected_depth - open_depth_after
    settling_equation_parity = (
        abs(baseline_residual) <= SETTLING_RECON_TOL
        and abs(open_residual) <= SETTLING_RECON_TOL
    )
    baseline_final_depth = record.get("depth_after_cold_branch_m", record["depth_after_settling_m"])
    baseline_final_density = record.get("density_after_cold_branch_kg_m3", record["densgt_after_kg_m3"])
    baseline_final_increment = baseline_final_depth - record["depth_after_settling_m"]
    baseline_m3_branch = "snowing" if record["hrsnow_m"] > 0.0 else "no_snow"
    inferred_driftf_plus_driftg = (
        baseline_final_increment - record["hrsnow_m"]
        if baseline_m3_branch == "snowing"
        else baseline_final_increment
    )
    open_snowfall_depth = hour_value(row, "snow_hourly_snowfall_depth_m", key[2])
    snowfall_delta = open_snowfall_depth - record["hrsnow_m"]
    final_depth_delta = open_depth_after - baseline_final_depth
    final_density_delta = open_density_after - baseline_final_density
    snowfall_input_lineage = (
        settling_equation_parity
        and baseline_m3_branch == "snowing"
        and abs(snowfall_delta) > MATERIAL_DEPTH_TOL_M
        and abs(open_residual) <= SETTLING_RECON_TOL
    )
    no_snow_drift_lineage = (
        settling_equation_parity
        and baseline_m3_branch == "no_snow"
        and abs(inferred_driftf_plus_driftg) > MATERIAL_DEPTH_TOL_M
        and abs(open_residual) <= SETTLING_RECON_TOL
    )
    return {
        "source_owned_openwepp_defect_proven": False,
        "production_edit_authorized": False,
        "key": {"year": key[0], "julian": key[1], "hour": key[2]},
        "baseline": {
            **record,
            "expected_setf_from_wdayct": baseline_expected_setf,
            "expected_density_after_kg_m3": baseline_expected_density,
            "expected_depth_after_m": baseline_expected_depth,
            "settling_depth_reconstruction_residual_m": baseline_residual,
            "depth_after_cold_branch_m": baseline_final_depth,
            "density_after_cold_branch_kg_m3": baseline_final_density,
            "m3_branch": baseline_m3_branch,
            "final_depth_increment_m": baseline_final_increment,
            "inferred_driftf_plus_driftg_m": inferred_driftf_plus_driftg,
        },
        "openwepp": {
            "settle_day_count_used": open_settle_day_count,
            "density_before_kg_m3": open_density_before,
            "depth_before_m": open_depth_before,
            "expected_setf_from_settle_day_count": open_expected_setf,
            "expected_density_after_kg_m3": open_expected_density,
            "density_after_kg_m3": open_density_after,
            "expected_depth_after_m": open_expected_depth,
            "depth_after_m": open_depth_after,
            "settling_depth_reconstruction_residual_m": open_residual,
            "snowfall_depth_m": open_snowfall_depth,
            "raw_melt_m": hour_value(row, "snow_hourly_melt_raw_m", key[2]),
        },
        "comparison": {
            "settling_depth_delta_openwepp_minus_baseline_m": open_depth_after
            - record["depth_after_settling_m"],
            "settling_density_delta_openwepp_minus_baseline_kg_m3": open_density_after
            - record["densgt_after_kg_m3"],
            "final_depth_delta_openwepp_minus_baseline_m": final_depth_delta,
            "final_density_delta_openwepp_minus_baseline_kg_m3": final_density_delta,
            "settling_equation_reconstruction_parity": settling_equation_parity,
            "baseline_m3_branch": baseline_m3_branch,
            "baseline_final_depth_increment_m": baseline_final_increment,
            "baseline_hrsnow_m": record["hrsnow_m"],
            "openwepp_hourly_snowfall_depth_m": open_snowfall_depth,
            "snowfall_depth_delta_openwepp_minus_baseline_m": snowfall_delta,
            "inferred_driftf_plus_driftg_m": inferred_driftf_plus_driftg,
            "no_snow_drift_lineage_candidate": no_snow_drift_lineage,
            "snowfall_input_lineage_candidate": snowfall_input_lineage,
            "openwepp_matches_post_settling_before_final_addition": abs(open_residual) <= SETTLING_RECON_TOL,
            "classification_reason": (
                "baseline and openWEPP settling equations reconstruct internally through post-settling depth; baseline then executes the snowing M3 branch at snowd.for:167 with positive hrsnow, while openWEPP records zero hourly snowfall at the homologous hour"
                if snowfall_input_lineage
                else (
                    "baseline and openWEPP settling equations reconstruct internally through post-settling depth; baseline then executes the no-snow M3 drift branch at snowd.for:145-146"
                    if no_snow_drift_lineage
                    else "branch-aware settling/snowfall reconstruction remains unresolved"
                )
            ),
        },
    }


def first_high_precision_settling_divergence(
    h313: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
    year: int,
) -> tuple[dict[str, Any], dict[str, Any] | None]:
    previous: dict[str, Any] | None = None
    required = {"depth_after_cold_branch_m", "density_after_cold_branch_kg_m3"}
    for key in sorted(h313):
        if key[0] != year:
            continue
        record = h313[key]
        if not required.issubset(record):
            continue
        row = traces.get((key[0], key[1]))
        if row is None:
            raise PairedEvidenceError(f"missing openWEPP trace row for high-precision settling key {key}")
        open_depth = hour_value(row, "snow_hourly_depth_after_m", key[2])
        open_density = hour_value(row, "snow_hourly_density_after_kg_m3", key[2])
        state = {
            "year": key[0],
            "julian": key[1],
            "hour": key[2],
            "baseline_depth_after_m": record["depth_after_cold_branch_m"],
            "baseline_density_after_kg_m3": record["density_after_cold_branch_kg_m3"],
            "openwepp_depth_after_m": open_depth,
            "openwepp_density_after_kg_m3": open_density,
            "depth_delta_openwepp_minus_baseline_m": open_depth - record["depth_after_cold_branch_m"],
            "density_delta_openwepp_minus_baseline_kg_m3": open_density - record["density_after_cold_branch_kg_m3"],
            "material_depth_divergent": abs(open_depth - record["depth_after_cold_branch_m"]) > MATERIAL_DEPTH_TOL_M,
            "material_density_divergent": abs(open_density - record["density_after_cold_branch_kg_m3"]) > MATERIAL_DENSITY_TOL_KG_M3,
        }
        if state["material_depth_divergent"] or state["material_density_divergent"]:
            return state, previous
        previous = state
    raise PairedEvidenceError(f"no high-precision H313 settling divergence found for {year}")


def settling_reconstruction(
    source_row: dict[str, Any],
    h313: dict[tuple[int, int, int], dict[str, float]],
    traces: dict[tuple[int, int], dict[str, Any]],
) -> dict[str, Any]:
    hphys0312_first = source_row["first_material_divergence"]
    hphys0312_key = (
        int(hphys0312_first["year"]),
        int(hphys0312_first["julian"]),
        int(hphys0312_first["hour"]),
    )
    hphys0312_candidate = reconstruct_settling_key(hphys0312_key, h313, traces)
    first_high_precision, previous_high_precision = first_high_precision_settling_divergence(
        h313, traces, int(source_row["scan_year"])
    )
    first_key = (
        int(first_high_precision["year"]),
        int(first_high_precision["julian"]),
        int(first_high_precision["hour"]),
    )
    first_reconstruction = reconstruct_settling_key(first_key, h313, traces)
    route = (
        "hourly-snowfall-input-lineage-hold"
        if first_reconstruction["comparison"]["snowfall_input_lineage_candidate"]
        else (
            "cold-drift-addition-lineage-hold"
            if first_reconstruction["comparison"]["no_snow_drift_lineage_candidate"]
            else "settling-high-precision-reconstruction-residual-hold"
        )
    )
    hphys0312_candidate_material = (
        abs(hphys0312_candidate["comparison"]["settling_depth_delta_openwepp_minus_baseline_m"])
        > MATERIAL_DEPTH_TOL_M
        or abs(hphys0312_candidate["comparison"]["settling_density_delta_openwepp_minus_baseline_kg_m3"])
        > MATERIAL_DENSITY_TOL_KG_M3
    )
    return {
        "route": route,
        "source_owned_openwepp_defect_proven": False,
        "production_edit_authorized": False,
        "hphys0312_candidate_key": {
            "year": hphys0312_key[0],
            "julian": hphys0312_key[1],
            "hour": hphys0312_key[2],
        },
        "hphys0312_candidate_material_after_high_precision": hphys0312_candidate_material,
        "hphys0312_candidate_reconstruction": hphys0312_candidate,
        "first_high_precision_material_divergence": first_high_precision,
        "last_high_precision_within_tolerance_state_before_first_divergence": previous_high_precision,
        "first_high_precision_reconstruction": first_reconstruction,
        "classification_reason": (
            "high-precision H313 evidence shows the post-settling equation itself reconstructs in both implementations; the material final-state delta follows positive baseline hrsnow in the snowing M3 branch while openWEPP records zero hourly snowfall"
            if first_reconstruction["comparison"]["snowfall_input_lineage_candidate"]
            else (
                "high-precision H313 evidence shows the post-settling equation itself reconstructs in both implementations; the material final-state delta follows the baseline no-snow drift branch"
                if first_reconstruction["comparison"]["no_snow_drift_lineage_candidate"]
                else "high-precision H313 evidence keeps the first material settling/snowfall branch divergence unresolved"
            )
        ),
    }


def classify_recursion(first_state: dict[str, Any]) -> tuple[str, str]:
    if first_state["julian"] == 1 and first_state["hour"] == 1:
        return (
            "recursive-year-start-inherited-state-hold",
            "first 2014 material divergence is already present at year-start; recurse earlier again before edits",
        )
    cold_settling = (
        first_state["openwepp_depth_before_m"] > MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_snowfall_depth_m"]) <= MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_raw_melt_m"]) <= MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_routed_melt_m"]) <= MATERIAL_DEPTH_TOL_M
        and abs(first_state["openwepp_rain_m"]) <= MATERIAL_DEPTH_TOL_M
        and first_state["openwepp_melt_branch_active"] == 0.0
        and first_state["openwepp_air_temp_c"] < 0.0
    )
    if cold_settling:
        return (
            "recursive-settling-depth-update-hold",
            "first 2014 material divergence occurs during cold existing-snow no-snowfall/no-melt settling",
        )
    return (
        "recursive-prior-year-lineage-hold",
        "first 2014 material divergence is within prior-year snowpack lineage and requires source-line subclassification",
    )


def build_ledger(identity: dict[str, Any], source_citations: dict[str, list[str]]) -> list[dict[str, Any]]:
    hphys0312 = read_json(HPHYS0312_LEDGER)
    h305_paths = baseline_log_paths_by_hillslope()
    h313_paths = h313_log_paths_by_hillslope(identity)
    trace_paths = trace_paths_by_hillslope()
    ledger: list[dict[str, Any]] = []
    for row in hphys0312:
        hill = int(row["hillslope_id"])
        h305 = parse_h305_log(h305_paths[hill])
        h313 = parse_h313_log(h313_paths[hill])
        traces = load_trace_rows(trace_paths[hill])
        if row["route"] == "settling-depth-update-hold":
            reconstruction = settling_reconstruction(row, h313, traces)
            ledger.append(
                {
                    "hillslope_id": hill,
                    "window": row["window"],
                    "target_year": row["target_year"],
                    "scan_year": row["scan_year"],
                    "source_hphys0312_route": row["route"],
                    "affected_hphys0309_rows": row["affected_hphys0309_rows"],
                    "hphys0313_route": reconstruction["route"],
                    "settling_reconstruction": reconstruction,
                    "recursive_scan": None,
                    "production_edit_authorized": False,
                    "source_line_findings": source_citations,
                    "prohibited_compensation_note": "No downstream compensation is authorized; branch-predicate, melt-term, WB13, WB17, WB18, WB19, and WB12 edits remain invalid.",
                }
            )
        elif row["route"] == "year-start-inherited-state-hold":
            first, previous, terminal = scan_year(h305, traces, 2014)
            route, reason = classify_recursion(first)
            ledger.append(
                {
                    "hillslope_id": hill,
                    "window": row["window"],
                    "target_year": row["target_year"],
                    "scan_year": 2014,
                    "source_hphys0312_route": row["route"],
                    "affected_hphys0309_rows": row["affected_hphys0309_rows"],
                    "hphys0313_route": route,
                    "classification_reason": reason,
                    "settling_reconstruction": None,
                    "recursive_scan": {
                        "first_material_divergence": first,
                        "last_within_tolerance_state_before_first_divergence": previous,
                        "terminal_state": terminal,
                        "material_thresholds": {
                            "depth_tolerance_m": MATERIAL_DEPTH_TOL_M,
                            "density_tolerance_kg_m3": MATERIAL_DENSITY_TOL_KG_M3,
                        },
                    },
                    "production_edit_authorized": False,
                    "source_line_findings": source_citations,
                    "prohibited_compensation_note": "No downstream compensation is authorized; branch-predicate, melt-term, WB13, WB17, WB18, WB19, and WB12 edits remain invalid.",
                }
            )
        else:
            raise PairedEvidenceError(f"unexpected HPHYS0312 route: {row['route']}")
    return ledger


def write_markdown_artifacts(ledger: list[dict[str, Any]], identity: dict[str, Any]) -> None:
    route_counts: dict[str, int] = defaultdict(int)
    represented = 0
    for row in ledger:
        route_counts[row["hphys0313_route"]] += 1
        represented += int(row["affected_hphys0309_rows"])
    lines = [
        "# HPHYS0313 Snowpack Settling/Carry Recursion Summary",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "Ran:",
        "",
        f"- HPHYS0312 groups represented: `{len(ledger)}`.",
        f"- HPHYS0309 rows represented: `{represented}`.",
        "- Route counts:",
    ]
    for route, count in sorted(route_counts.items()):
        lines.append(f"  - `{route}`: `{count}`")
    lines.extend(
        [
            "- Production edits authorized: `0`.",
            "",
            "Static:",
            "",
            "- HPHYS0313 is diagnostic/source-line lineage evidence only.",
        ]
    )
    (ARTIFACT_DIR / "snowpack-settling-carry-recursion-summary.md").write_text("\n".join(lines) + "\n", encoding="utf-8")

    (ARTIFACT_DIR / "snowpack-settling-carry-recursion-method.md").write_text(
        "# HPHYS0313 Method\n\n"
        "Status: complete\n\n"
        "Evidence mode: ran\n\n"
        "Static:\n\n"
        "- Input ledger: HPHYS0312 prior-year terminal snowpack lineage ledger.\n"
        "- Settling route: temporary fixed-comparator instrumentation added high-precision `H313_*` observe tags in `snowd.for`, including post-settling, branch input, and final cold-branch depth.\n"
        "- Carry recursion route: existing HPHYS0305 fixed observe and openWEPP traces were scanned across calendar year 2014.\n"
        "- Material thresholds remained `0.0005 m` depth and `0.5 kg m^-3` density.\n"
        "- Instrumented observe tags are diagnostic evidence only; canonical source authority remains `/workdir/wepp-forest_260430_baseline`.\n\n"
        "Ran:\n\n"
        "- Built and ran a temporary fixed comparator with observe-off and observe-on lanes for H1, H7, and H39.\n"
        "- Verified observe-on/off WAT output identity for the temporary instrumentation lanes.\n"
        "- Wrote split-route ledger, summary, method, source-lineage, instrumentation patch, identity, and command log artifacts.\n",
        encoding="utf-8",
    )

    source_lines = [
        "# HPHYS0313 Source Lineage\n",
        "Status: complete\n",
        "Evidence mode: static\n",
        "Static:\n",
        "- Baseline settle-day count: `/workdir/wepp-forest_260430_baseline/src/snowd.for:61-65`.",
        "- Baseline cold settling equation: `/workdir/wepp-forest_260430_baseline/src/snowd.for:122-139`.",
        "- Baseline cold no-snow `driftg` final-depth addition: `/workdir/wepp-forest_260430_baseline/src/snowd.for:145-146`.",
        "- Baseline cold snowing branch fresh-snow/depth update: `/workdir/wepp-forest_260430_baseline/src/snowd.for:166-172`.",
        "- Baseline carry writeback: `/workdir/wepp-forest_260430_baseline/src/snowd.for:310-312`.",
        "- openWEPP settle-day count and settling equations: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3872-3924`.",
        "- Temporary diagnostic instrumentation patch: `fixed-baseline-settling-instrumentation.patch`.",
    ]
    (ARTIFACT_DIR / "snowpack-settling-carry-recursion-source-lineage.md").write_text("\n".join(source_lines) + "\n", encoding="utf-8")

    identity_md = [
        "# HPHYS0313 Fixed Baseline Settling Observe Identity",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "Ran:",
        "",
    ]
    for hill, row in sorted(identity.items(), key=lambda item: int(item[0])):
        identity_md.append(
            f"- H{hill}: observe-off/on WAT identity `{row['observe_off_to_observe_on_bit_identical']}`; observe records `{row['observe_on_lane']['observe_records']}`."
        )
    (ARTIFACT_DIR / "fixed-baseline-settling-observe-identity.md").write_text("\n".join(identity_md) + "\n", encoding="utf-8")


def run() -> int:
    source_citations = verify_source_lines()
    prepare_fixed_worktree()
    patch_fixed_observe()
    write_fixed_patch()
    binary = build_fixed_worktree()
    identity = run_fixed_observe_lanes(binary)
    if not all(row["pass"] for row in identity.values()):
        raise PairedEvidenceError("fixed comparator observe-off/on identity failed")
    ledger = build_ledger(identity, source_citations)
    write_json(ARTIFACT_DIR / "snowpack-settling-carry-recursion-ledger.json", ledger)
    write_json(ARTIFACT_DIR / "fixed-baseline-settling-command-log.json", COMMAND_LOG)
    write_markdown_artifacts(ledger, identity)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test-missing-source-line", action="store_true")
    args = parser.parse_args()
    try:
        if args.self_test_missing_source_line:
            bad = [(BASELINE_REPO / "src/snowd.for", 125, "INTENTIONALLY_MISSING_HPHYS0313_SOURCE_TOKEN", "snowd.for:125")]
            verify_source_lines(bad)
            raise AssertionError("missing-source-line self-test did not fail")
        return run()
    except (SourceLineEvidenceError, PairedEvidenceError, RuntimeError) as exc:
        print(f"HPHYS0313 failed closed: {exc}", file=os.sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
