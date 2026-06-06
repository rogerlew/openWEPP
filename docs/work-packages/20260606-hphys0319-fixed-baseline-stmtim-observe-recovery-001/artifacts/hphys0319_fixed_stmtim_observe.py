#!/usr/bin/env python3
"""Recover paired HPHYS0319 stmtim observe surfaces.

This script is package-local evidence tooling. It creates a temporary baseline
worktree under /tmp, applies observe-only instrumentation, runs H1/H7/H39, then
regenerates OpenWEPP HPHYS0245 traces for the same key.
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import time
from pathlib import Path
from typing import Any

OPENWEPP_ROOT = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
BASELINE_REPO = Path("/workdir/wepp-forest_260430_baseline")
BASELINE_COMMIT = "dac3c950d8b16cc73774bf5ce2e7e11f80baac70"
SOURCE_RUN_ROOT = Path("/tmp/hphys0305_paired_melt_terms_20260605T000000Z")
WORK_ROOT = Path("/tmp/hphys0319_fixed_stmtim_observe_20260606T000000Z")
BASELINE_TREE = WORK_ROOT / "wepp_260430_stmtim_observe"
TARGET_HILLS = [1, 7, 39]
TARGET_YEAR = 2013
TARGET_DAY = 11
TARGET_HOUR = 11
TRACE_MAX_DAYS = 1800
HOUR_KEY = f"{TARGET_HOUR:04d}"
COMMANDS: list[dict[str, Any]] = []

OBS_RE = re.compile(
    r"^(?P<tag>\S+)\s+y=\s*(?P<year>-?\d+)\s+d=\s*(?P<day>-?\d+)"
    r"\s+e=\s*(?P<element>-?\d+)\s+c=\s*(?P<channel>-?\d+)"
    r"\s+s=\s*(?P<hour>-?\d+)\s+v1=\s*(?P<v1>[-+0-9.Ee]+)"
    r"\s+v2=\s*(?P<v2>[-+0-9.Ee]+)"
)


def tail(text: str, limit: int = 4000) -> str:
    return text if len(text) <= limit else text[-limit:]


def run(
    name: str,
    args: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    input_bytes: bytes | None = None,
) -> subprocess.CompletedProcess[bytes]:
    start = time.monotonic()
    result = subprocess.run(
        args,
        cwd=str(cwd) if cwd else None,
        env=env,
        input=input_bytes,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    seconds = time.monotonic() - start
    stdout = result.stdout.decode("utf-8", errors="replace")
    stderr = result.stderr.decode("utf-8", errors="replace")
    COMMANDS.append(
        {
            "name": name,
            "args": args,
            "cwd": str(cwd) if cwd else None,
            "rc": result.returncode,
            "seconds": round(seconds, 3),
            "stdout_tail": tail(stdout),
            "stderr_tail": tail(stderr),
        }
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"{name} failed with rc={result.returncode}\n"
            f"stdout tail:\n{tail(stdout)}\n\nstderr tail:\n{tail(stderr)}"
        )
    return result


def write_text(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"{label}: expected one match, found {count}")
    return text.replace(old, new)


def prepare_worktree() -> None:
    if WORK_ROOT.exists():
        shutil.rmtree(WORK_ROOT)
    WORK_ROOT.mkdir(parents=True)
    run("baseline_worktree_prune_pre", ["git", "-C", str(BASELINE_REPO), "worktree", "prune"])
    run(
        "baseline_worktree_add",
        [
            "git",
            "-C",
            str(BASELINE_REPO),
            "worktree",
            "add",
            "--detach",
            str(BASELINE_TREE),
            BASELINE_COMMIT,
        ],
    )


def patch_baseline_sources() -> None:
    stmtim = BASELINE_TREE / "src" / "stmtim.for"
    winter = BASELINE_TREE / "src" / "winter.for"

    text = stmtim.read_text(encoding="utf-8")
    text = replace_once(
        text,
        "      subroutine stmtim(rain,stmdur,hour,wnttim,rans,\n"
        "     1                  snodpt,rnhrs,snohrs, daytmin)",
        "      subroutine stmtim(rain,stmdur,hour,wnttim,rans,\n"
        "     1                  snodpt,rnhrs,snohrs, daytmin,iplane,\n"
        "     2                  obs_year,obs_sdate)",
        "stmtim signature",
    )
    text = replace_once(
        text,
        "      integer hour,rnhrs,snohrs\n",
        "      integer hour,rnhrs,snohrs,iplane,obs_year,obs_sdate\n",
        "stmtim integer declaration",
    )
    text = replace_once(
        text,
        "      real denwat,tmpvr3\n"
        "      integer wntdur\n",
        "      real denwat,tmpvr3,active_flag,branch_code\n"
        "      integer wntdur\n",
        "stmtim locals",
    )
    text = replace_once(
        text,
        "      hrrain(hour) = 0.0\n"
        "      hrsnow(hour) = 0.0\n",
        "      hrrain(hour) = 0.0\n"
        "      hrsnow(hour) = 0.0\n"
        "      wntdur = 0\n"
        "      active_flag = 0.0\n"
        "      branch_code = 0.0\n",
        "stmtim flag initialization",
    )
    text = replace_once(
        text,
        "        if ((hour.ge.wnttim) .and. (hour.lt.(wnttim+wntdur))) then\n",
        "        if ((hour.ge.wnttim) .and. (hour.lt.(wnttim+wntdur))) then\n"
        "          active_flag = 1.0\n",
        "stmtim active flag",
    )
    text = replace_once(
        text,
        "            hrrain(hour) = rain / wntdur\n"
        "            hrsnow(hour) = 0.0\n",
        "            hrrain(hour) = rain / wntdur\n"
        "            hrsnow(hour) = 0.0\n"
        "            branch_code = 1.0\n",
        "stmtim rain branch flag",
    )
    text = replace_once(
        text,
        "            hrsnow(hour) = rain / wntdur * 10.0\n"
        "            hrrain(hour) = 0.0\n",
        "            hrsnow(hour) = rain / wntdur * 10.0\n"
        "            hrrain(hour) = 0.0\n"
        "            branch_code = 2.0\n",
        "stmtim snow branch flag",
    )
    text = replace_once(
        text,
        "      return\n"
        "      end\n",
        "      call wepp_observe('STM_RAIN_STMDUR',obs_year,obs_sdate,\n"
        "     1                  iplane,0,hour,rain,stmdur)\n"
        "      call wepp_observe('STM_WNTDUR_WNTTIM',obs_year,obs_sdate,\n"
        "     1                  iplane,0,hour,float(wntdur),wnttim)\n"
        "      call wepp_observe('STM_TEMP_RST',obs_year,obs_sdate,iplane,\n"
        "     1                  0,hour,hrtemp,rst)\n"
        "      call wepp_observe('STM_OUT_RAIN_SNOW',obs_year,obs_sdate,\n"
        "     1                  iplane,0,hour,hrrain(hour),hrsnow(hour))\n"
        "      call wepp_observe('STM_ACTIVE_BRANCH',obs_year,obs_sdate,\n"
        "     1                  iplane,0,hour,active_flag,branch_code)\n"
        "      return\n"
        "      end\n",
        "stmtim observe calls",
    )
    stmtim.write_text(text, encoding="utf-8")

    winter_text = winter.read_text(encoding="utf-8")
    winter_text = replace_once(
        winter_text,
        "        call stmtim(rain(iplane),stmdur,hour,wnttim,rans,\n"
        "c     1              hrrain(hour),hrsnow(hour),hrtemp,\n"
        "     2              snodpt(iplane),rnhrs,snohrs,tmin)",
        "        call stmtim(rain(iplane),stmdur,hour,wnttim,rans,\n"
        "c     1              hrrain(hour),hrsnow(hour),hrtemp,\n"
        "     2              snodpt(iplane),rnhrs,snohrs,tmin,iplane,\n"
        "     3              year,sdate)",
        "winter stmtim call",
    )
    winter.write_text(winter_text, encoding="utf-8")

    diff = run(
        "baseline_instrumentation_diff",
        ["git", "-C", str(BASELINE_TREE), "diff", "--", "src/stmtim.for", "src/winter.for"],
    ).stdout.decode("utf-8", errors="replace")
    write_text(ARTIFACT_DIR / "fixed-baseline-stmtim-observe.patch", diff)


def build_baseline() -> Path:
    env = dict(os.environ)
    env.update({"TARGET_TAG": "hphys0319", "HILL_LABEL": "hill"})
    run(
        "build_fixed_baseline_observe_hill",
        ["bash", "tools/build_wepp_dated_release.sh"],
        cwd=BASELINE_TREE,
        env=env,
    )
    binary = BASELINE_TREE / "release" / "wepp_hphys0319_hill"
    if not binary.exists():
        raise RuntimeError(f"missing expected fixed-baseline hill binary: {binary}")
    return binary


def parse_observe_log(path: Path) -> dict[str, dict[str, float]]:
    rows: dict[str, dict[str, float]] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = OBS_RE.match(line.strip())
        if not match:
            continue
        data = match.groupdict()
        if (
            int(data["year"]) == TARGET_YEAR
            and int(data["day"]) == TARGET_DAY
            and int(data["hour"]) == TARGET_HOUR
        ):
            rows[data["tag"]] = {
                "element": float(data["element"]),
                "channel": float(data["channel"]),
                "v1": float(data["v1"]),
                "v2": float(data["v2"]),
            }
    required = {
        "STM_RAIN_STMDUR",
        "STM_WNTDUR_WNTTIM",
        "STM_TEMP_RST",
        "STM_OUT_RAIN_SNOW",
        "STM_ACTIVE_BRANCH",
    }
    missing = sorted(required.difference(rows))
    if missing:
        raise RuntimeError(f"missing observe tags in {path}: {missing}")
    return rows


def run_fixed_baseline(binary: Path) -> dict[str, Any]:
    fixed_root = WORK_ROOT / "fixed_baseline"
    results: dict[str, Any] = {}
    for hill in TARGET_HILLS:
        label = f"H{hill}"
        source = SOURCE_RUN_ROOT / "fixed_baseline" / f"{label}_hphys0305_observe_on"
        dest = fixed_root / label
        if not source.exists():
            raise RuntimeError(f"missing source fixed-baseline run directory: {source}")
        shutil.copytree(source, dest)
        runs_dir = dest / "runs"
        output_dir = dest / "output"
        if output_dir.exists():
            shutil.rmtree(output_dir)
        output_dir.mkdir(parents=True)
        (runs_dir / "wepp_observe.on").write_text("", encoding="utf-8")
        log_path = runs_dir / "wepp_observe.log"
        if log_path.exists():
            log_path.unlink()
        run_file = runs_dir / f"p{hill}.run"
        run(
            f"{label}_fixed_baseline_stmtim_observe",
            [str(binary)],
            cwd=runs_dir,
            input_bytes=run_file.read_bytes(),
        )
        rows = parse_observe_log(log_path)
        results[label] = {
            "observe_log": str(log_path),
            "rain_m": rows["STM_RAIN_STMDUR"]["v1"],
            "stmdur_s": rows["STM_RAIN_STMDUR"]["v2"],
            "wntdur_h": rows["STM_WNTDUR_WNTTIM"]["v1"],
            "wnttim_h": rows["STM_WNTDUR_WNTTIM"]["v2"],
            "hrtemp_c": rows["STM_TEMP_RST"]["v1"],
            "rst_c": rows["STM_TEMP_RST"]["v2"],
            "hrrain_m": rows["STM_OUT_RAIN_SNOW"]["v1"],
            "hrsnow_m": rows["STM_OUT_RAIN_SNOW"]["v2"],
            "active_interval": rows["STM_ACTIVE_BRANCH"]["v1"],
            "rain_branch": 1.0 if rows["STM_ACTIVE_BRANCH"]["v2"] == 1.0 else 0.0,
            "snow_branch": 1.0 if rows["STM_ACTIVE_BRANCH"]["v2"] == 2.0 else 0.0,
            "branch_code": rows["STM_ACTIVE_BRANCH"]["v2"],
        }
    return results


def build_openwepp() -> Path:
    run(
        "cargo_build_release_openwepp_cli_hill",
        [
            "cargo",
            "build",
            "--release",
            "-p",
            "openwepp-runner",
            "--bin",
            "openwepp-cli-hill",
        ],
        cwd=OPENWEPP_ROOT,
    )
    binary = OPENWEPP_ROOT / "target" / "release" / "openwepp-cli-hill"
    if not binary.exists():
        raise RuntimeError(f"missing expected OpenWEPP hill binary: {binary}")
    return binary


def select_trace_row(path: Path) -> dict[str, Any]:
    selected: dict[str, Any] | None = None
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        row = json.loads(line)
        if row.get("calendar_year") != TARGET_YEAR:
            continue
        if row.get("julian_day") != TARGET_DAY:
            continue
        values = row.get("snow_hourly_stmtim_hrsnow_m")
        if isinstance(values, dict) and HOUR_KEY in values:
            selected = row
    if selected is None:
        raise RuntimeError(f"missing OpenWEPP trace row for {TARGET_YEAR}/{TARGET_DAY} in {path}")
    return selected


def map_value(row: dict[str, Any], field: str) -> float | None:
    values = row.get(field)
    if not isinstance(values, dict):
        return None
    value = values.get(HOUR_KEY)
    if value is None:
        return None
    return float(value)


def run_openwepp_traces(binary: Path) -> dict[str, Any]:
    openwepp_root = WORK_ROOT / "openwepp"
    runs_dir = openwepp_root / "runs"
    output_dir = openwepp_root / "hillslope_output"
    shutil.copytree(SOURCE_RUN_ROOT / "runs", runs_dir)
    output_dir.mkdir(parents=True)
    results: dict[str, Any] = {}
    for hill in TARGET_HILLS:
        label = f"H{hill}"
        trace_path = output_dir / f"{label}.hphys0319.trace.jsonl"
        env = dict(os.environ)
        env.update(
            {
                "OPENWEPP_HPHYS0245_TRACE_PATH": str(trace_path),
                "OPENWEPP_HPHYS0245_TRACE_MAX_DAYS": str(TRACE_MAX_DAYS),
            }
        )
        run(
            f"{label}_openwepp_hphys0319_trace",
            [
                str(binary),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hill}_openwepp.run",
                "--output-dir",
                str(output_dir),
                "--policy",
                "compat",
            ],
            cwd=OPENWEPP_ROOT,
            env=env,
        )
        row = select_trace_row(trace_path)
        results[label] = {
            "trace_path": str(trace_path),
            "schema": row.get("schema"),
            "boundary": row.get("boundary"),
            "phase": row.get("phase"),
            "rain_m": map_value(row, "snow_hourly_stmtim_rain_m"),
            "stmdur_s": map_value(row, "snow_hourly_stmtim_stmdur_s"),
            "wntdur_h": map_value(row, "snow_hourly_stmtim_wntdur_h"),
            "wnttim_h": map_value(row, "snow_hourly_stmtim_wnttim_h"),
            "hrtemp_c": map_value(row, "snow_hourly_stmtim_hrtemp_c"),
            "rst_c": map_value(row, "snow_hourly_stmtim_rst_c"),
            "hrrain_m": map_value(row, "snow_hourly_stmtim_hrrain_m"),
            "hrsnow_m": map_value(row, "snow_hourly_stmtim_hrsnow_m"),
            "active_interval": map_value(row, "snow_hourly_stmtim_active_interval"),
            "rain_branch": map_value(row, "snow_hourly_stmtim_rain_branch"),
            "snow_branch": map_value(row, "snow_hourly_stmtim_snow_branch"),
            "snowfall_m": map_value(row, "snow_hourly_snowfall_depth_m"),
        }
    return results


def compare(fixed: dict[str, Any], openwepp: dict[str, Any]) -> dict[str, Any]:
    paired: dict[str, Any] = {}
    for label in [f"H{hill}" for hill in TARGET_HILLS]:
        baseline = fixed[label]
        candidate = openwepp[label]
        deltas: dict[str, float | None] = {}
        for key in [
            "rain_m",
            "stmdur_s",
            "wntdur_h",
            "wnttim_h",
            "hrtemp_c",
            "rst_c",
            "hrrain_m",
            "hrsnow_m",
            "active_interval",
            "rain_branch",
            "snow_branch",
        ]:
            if baseline[key] is None or candidate[key] is None:
                deltas[key] = None
            else:
                deltas[key] = float(candidate[key]) - float(baseline[key])
        classification = "stmtim-paired-values-present-hold"
        if any(candidate[key] is None for key in deltas):
            classification = "openwepp-stmtim-trace-missing-hold"
        elif abs(deltas["active_interval"] or 0.0) > 1.0e-12:
            classification = "stmtim-active-interval-divergence-hold"
        elif abs(deltas["snow_branch"] or 0.0) > 1.0e-12 or abs(deltas["rain_branch"] or 0.0) > 1.0e-12:
            classification = "stmtim-branch-divergence-hold"
        elif abs(deltas["hrsnow_m"] or 0.0) > 1.0e-8 or abs(deltas["hrrain_m"] or 0.0) > 1.0e-8:
            classification = "stmtim-output-magnitude-divergence-hold"
        else:
            classification = "stmtim-control-surfaces-paired-no-output-delta-hold"
        paired[label] = {
            "fixed_baseline": baseline,
            "openwepp": candidate,
            "candidate_minus_baseline": deltas,
            "classification": classification,
        }
    return paired


def fmt(value: Any) -> str:
    if value is None:
        return "NA"
    if isinstance(value, float):
        return f"{value:.12g}"
    return str(value)


def write_result_artifacts(result: dict[str, Any]) -> None:
    write_text(ARTIFACT_DIR / "hphys0319_fixed_stmtim_observe.json", json.dumps(result, indent=2, sort_keys=True) + "\n")
    write_text(ARTIFACT_DIR / "hphys0319_command_log.json", json.dumps(COMMANDS, indent=2, sort_keys=True) + "\n")

    headers = [
        "Hill",
        "rain m",
        "stmdur s",
        "wntdur h",
        "wnttim h",
        "hrtemp C",
        "rst C",
        "hrrain m",
        "hrsnow m",
        "active",
        "branch",
    ]
    rows = []
    for label, values in result["fixed_baseline"].items():
        rows.append(
            [
                label,
                fmt(values["rain_m"]),
                fmt(values["stmdur_s"]),
                fmt(values["wntdur_h"]),
                fmt(values["wnttim_h"]),
                fmt(values["hrtemp_c"]),
                fmt(values["rst_c"]),
                fmt(values["hrrain_m"]),
                fmt(values["hrsnow_m"]),
                fmt(values["active_interval"]),
                fmt(values["branch_code"]),
            ]
        )
    ledger = "# Fixed-Baseline Stmtim Observe Ledger\n\n"
    ledger += "Status: complete\n\nEvidence mode: Ran\n\n"
    ledger += "Ran:\n"
    ledger += f"- baseline_commit: `{BASELINE_COMMIT}`\n"
    ledger += "- fixed_baseline_stmtim_observe_available: `true`\n"
    ledger += "- carried_rows_total: `57`\n"
    ledger += f"- key: `{TARGET_YEAR}` day `{TARGET_DAY}` hour `{TARGET_HOUR}`\n"
    ledger += f"- temporary_work_root: `{WORK_ROOT}`\n"
    ledger += "- observe tags: `STM_RAIN_STMDUR`, `STM_WNTDUR_WNTTIM`, `STM_TEMP_RST`, `STM_OUT_RAIN_SNOW`, `STM_ACTIVE_BRANCH`\n\n"
    ledger += "| " + " | ".join(headers) + " |\n"
    ledger += "| " + " | ".join(["---"] * len(headers)) + " |\n"
    for row in rows:
        ledger += "| " + " | ".join(row) + " |\n"
    write_text(ARTIFACT_DIR / "fixed-baseline-stmtim-observe-ledger.md", ledger)

    class_headers = [
        "Hill",
        "baseline hrsnow m",
        "openwepp snow.hourly.stmtim.hrsnow_m_0011",
        "baseline active",
        "openwepp snow.hourly.stmtim.active_interval_0011",
        "baseline snow branch",
        "openwepp snow branch",
        "classification",
    ]
    classification = "# Paired Stmtim Observe Classification\n\n"
    classification += "Status: complete\n\nEvidence mode: Ran\n\n"
    classification += "Ran:\n"
    classification += "- paired_fixed_baseline_openwepp_stmtim_values: `true`\n"
    classification += "- production_physics_edit_authorized: `false`\n"
    classification += "- carried_rows_total: `57`\n"
    classification += "- next_owner: `HPHYS0320`\n"
    classification += "- next_route: `paired-stmtim-source-line-classification-hold`\n\n"
    classification += "| " + " | ".join(class_headers) + " |\n"
    classification += "| " + " | ".join(["---"] * len(class_headers)) + " |\n"
    for label, values in result["paired"].items():
        baseline = values["fixed_baseline"]
        candidate = values["openwepp"]
        classification += "| " + " | ".join(
            [
                label,
                fmt(baseline["hrsnow_m"]),
                fmt(candidate["hrsnow_m"]),
                fmt(baseline["active_interval"]),
                fmt(candidate["active_interval"]),
                fmt(baseline["snow_branch"]),
                fmt(candidate["snow_branch"]),
                values["classification"],
            ]
        ) + " |\n"
    classification += "\n"
    classification += "HPHYS0319 recovers the missing fixed-baseline observe lane but does not by itself prove a production defect. HPHYS0320 owns source-line classification for the paired divergence.\n"
    write_text(ARTIFACT_DIR / "paired-stmtim-observe-classification.md", classification)


def main() -> int:
    try:
        prepare_worktree()
        patch_baseline_sources()
        fixed_binary = build_baseline()
        fixed = run_fixed_baseline(fixed_binary)
        openwepp_binary = build_openwepp()
        openwepp = run_openwepp_traces(openwepp_binary)
        paired = compare(fixed, openwepp)
        result = {
            "status": "complete",
            "baseline_commit": BASELINE_COMMIT,
            "baseline_temp_instrumentation": True,
            "production_physics_edit_authorized": False,
            "carried_rows_total": 57,
            "key": {"calendar_year": TARGET_YEAR, "julian_day": TARGET_DAY, "hour": TARGET_HOUR},
            "fixed_baseline": fixed,
            "openwepp": openwepp,
            "paired": paired,
            "next_package": "HPHYS0320",
            "next_route": "paired-stmtim-source-line-classification-hold",
        }
        write_result_artifacts(result)
        return 0
    finally:
        write_text(ARTIFACT_DIR / "hphys0319_command_log.json", json.dumps(COMMANDS, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    raise SystemExit(main())
