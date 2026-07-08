#!/usr/bin/env python3
"""Run the narrow mn_corn_h4 day-792 raw-hydrograph numerics ladder."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

try:
    import pyarrow.parquet as pq
except ImportError:  # pragma: no cover - recorded as evidence if missing.
    pq = None

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
MATERIALIZATION = Path(
    "docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/"
    "artifacts/selected-cohort-materialization.json"
)
BIN = Path("target/release/openwepp-cli-hill")
RUN_ROOT = ARTIFACTS / "raw-hydrograph-numerics-runs"
SUMMARY_JSON = ARTIFACTS / "raw-hydrograph-numerics-summary.json"
SUMMARY_MD = ARTIFACTS / "raw-hydrograph-numerics-summary.md"

RUNGS: dict[str, float | None] = {
    "baseline_fixed10": None,
    "dx20": 20.0,
    "dx10": 10.0,
    "dx5": 5.0,
    "dx2p5": 2.5,
    "dx1p25": 1.25,
    "dx0p625": 0.625,
}

OUTPUT_LINE = re.compile(r'^(pass|loss|pass_parquet|wat)\s*=\s*"([^"]+)"', re.MULTILINE)
CLAMP_FAILURE_RE = re.compile(
    r"direct runtime kernel guard failed in (?P<phase>[^:]+): day (?P<day>\d+) "
    r"positivity clamp (?P<clamp>[0-9.eE+-]+) m3 exceeds active routed source cap "
    r"(?P<cap>[0-9.eE+-]+) m3 \(rel (?P<rel>[0-9.eE+-]+) > (?P<threshold>[0-9.eE+-]+)\)"
)
TEXT_INPUT_SUFFIXES = {".cli", ".man", ".slp", ".sol", ".toml", ".txt"}


def sha256(path: Path | None) -> str | None:
    if path is None or not path.is_file():
        return None
    digest = hashlib.sha256()
    with path.open("rb") as fp:
        for chunk in iter(lambda: fp.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run_text(command: list[str]) -> str:
    return subprocess.check_output(command, text=True).strip()


def binary_provenance(build_command: list[str]) -> dict[str, Any]:
    stat = BIN.stat()
    return {
        "build_command": " ".join(build_command),
        "path": str(BIN),
        "sha256": sha256(BIN),
        "size_bytes": stat.st_size,
        "mtime_utc": datetime.fromtimestamp(stat.st_mtime, timezone.utc).isoformat(),
        "git_head": run_text(["git", "rev-parse", "HEAD"]),
        "git_branch": run_text(["git", "branch", "--show-current"]),
        "git_status_short": run_text(["git", "status", "--short"]),
    }


def parse_time_log(text: str) -> dict[str, str | None]:
    def find(pattern: str) -> str | None:
        match = re.search(pattern, text)
        return match.group(1) if match else None

    return {
        "wall_seconds_raw": find(r"Elapsed \(wall clock\) time.*: ([0-9:.]+)"),
        "user_seconds": find(r"User time \(seconds\): ([0-9.]+)"),
        "sys_seconds": find(r"System time \(seconds\): ([0-9.]+)"),
    }


def parse_profile(text: str) -> dict[str, int] | None:
    match = re.search(r"laned_active_profile (\{.*\})", text)
    if not match:
        return None
    return json.loads(match.group(1))


def parse_clamp_failure(text: str) -> dict[str, Any] | None:
    match = CLAMP_FAILURE_RE.search(text)
    if not match:
        return None
    return {
        "phase": match.group("phase"),
        "day": int(match.group("day")),
        "clamp_m3": float(match.group("clamp")),
        "source_cap_m3": float(match.group("cap")),
        "clamp_source_ratio": float(match.group("rel")),
        "threshold": float(match.group("threshold")),
    }


def read_json(path: Path) -> dict[str, Any] | None:
    if not path.is_file():
        return None
    return json.loads(path.read_text())


def normalize_copied_text_inputs(run_dir: Path) -> None:
    """Remove whitespace-only diff noise from package-local input copies."""
    for path in run_dir.rglob("*"):
        if not path.is_file() or path.suffix not in TEXT_INPUT_SUFFIXES:
            continue
        try:
            original = path.read_text()
        except UnicodeDecodeError:
            continue
        lines = [line.rstrip(" \t") for line in original.splitlines()]
        while lines and lines[-1] == "":
            lines.pop()
        normalized = "\n".join(lines) + "\n"
        if normalized != original:
            path.write_text(normalized)


def output_file_from_manifest(manifest: dict[str, Any], suffix: str) -> Path | None:
    checksums = manifest.get("output_checksums", {})
    for raw_path in checksums:
        if raw_path.endswith(suffix):
            return Path(raw_path)
    return None


def pass_summary(path: Path | None) -> dict[str, Any] | None:
    if path is None or not path.is_file():
        return None
    if pq is None:
        return {"error": "pyarrow unavailable"}
    table = pq.read_table(path)
    names = set(table.column_names)
    summary: dict[str, Any] = {"rows": table.num_rows, "columns": table.column_names}
    for column in [
        "tdet",
        "tdep",
        "sedcon_1",
        "sedcon_2",
        "sedcon_3",
        "sedcon_4",
        "sedcon_5",
    ]:
        if column not in names:
            continue
        arr = table[column].combine_chunks()
        total = 0.0
        nonzero = 0
        for i in range(len(arr)):
            value = arr[i].as_py()
            if value is None:
                continue
            numeric = float(value)
            total += numeric
            if numeric != 0.0:
                nonzero += 1
        summary[f"{column}_sum"] = total
        summary[f"{column}_nonzero"] = nonzero
    if "year" in names:
        years = table["year"].combine_chunks()
        for column in ["tdet", "tdep", "sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5"]:
            if column not in names:
                continue
            values = table[column].combine_chunks()
            annual: dict[str, float] = {}
            for i in range(len(values)):
                year = years[i].as_py()
                value = values[i].as_py()
                if year is None or value is None:
                    continue
                annual[str(year)] = annual.get(str(year), 0.0) + float(value)
            summary[f"{column}_annual_sum"] = annual
    return summary


def trace_summary(path: Path | None) -> dict[str, Any] | None:
    if path is None or not path.is_file():
        return None
    rows = [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    terminal = [
        row for row in rows
        if row.get("is_terminal_lane") and row.get("terminal_day_outlet_m3") is not None
    ]
    shape_positive = [
        row for row in rows
        if float(row.get("source_m3") or 0.0) > 1.0e-12
    ]
    return {
        "path": str(path),
        "sha256": sha256(path),
        "rows": len(rows),
        "terminal_days": len(terminal),
        "terminal_outlet_total_m3": sum(float(row["terminal_day_outlet_m3"]) for row in terminal),
        "source_total_m3": sum(float(row.get("source_m3") or 0.0) for row in rows),
        "end_storage_total_m3": sum(float(row.get("mesh_end_storage_m3") or 0.0) for row in rows),
        "tail_fold_total_m3": sum(float(row.get("tail_fold_m3") or 0.0) for row in rows),
        "uniform_shape_rows": sum(1 for row in rows if row.get("uniform_shape")),
        "erosion_source_shape_degenerate_rows": sum(
            1 for row in rows if row.get("erosion_source_shape_degenerate")
        ),
        "positive_shape_rows": len(shape_positive),
    }


def read_trace_rows(path: str | None) -> list[dict[str, Any]]:
    if path is None:
        return []
    trace_path = Path(path)
    if not trace_path.is_file():
        return []
    return [json.loads(line) for line in trace_path.read_text().splitlines() if line.strip()]


def l1(values_a: list[float], values_b: list[float]) -> float:
    return sum(abs(a - b) for a, b in zip(values_a, values_b))


def annual_pass_delta(candidate: dict[str, Any], reference: dict[str, Any]) -> dict[str, Any]:
    cand_pass = candidate.get("pass_summary") or {}
    ref_pass = reference.get("pass_summary") or {}
    max_rel = 0.0
    max_surface = None
    for column in ["tdet", "tdep", "sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5"]:
        cand_annual = cand_pass.get(f"{column}_annual_sum") or {}
        ref_annual = ref_pass.get(f"{column}_annual_sum") or {}
        for year in sorted(set(cand_annual) | set(ref_annual)):
            cand_value = float(cand_annual.get(year, 0.0))
            ref_value = float(ref_annual.get(year, 0.0))
            if abs(ref_value) <= 1.0e-12 and abs(cand_value) <= 1.0e-12:
                rel = 0.0
            else:
                rel = abs(cand_value - ref_value) / max(abs(ref_value), 1.0e-12)
            if rel > max_rel:
                max_rel = rel
                max_surface = f"{column}:{year}"
    return {"max_rel": max_rel, "max_surface": max_surface, "passes_2pct": max_rel <= 0.02}


def compare_run_to_reference(candidate: dict[str, Any], reference: dict[str, Any]) -> dict[str, Any]:
    cand_rows = read_trace_rows(candidate.get("trace_path"))
    ref_rows = read_trace_rows(reference.get("trace_path"))
    cand_terminal = {
        row["sim_day_index"]: float(row["terminal_day_outlet_m3"])
        for row in cand_rows
        if row.get("is_terminal_lane") and row.get("terminal_day_outlet_m3") is not None
    }
    ref_terminal = {
        row["sim_day_index"]: float(row["terminal_day_outlet_m3"])
        for row in ref_rows
        if row.get("is_terminal_lane") and row.get("terminal_day_outlet_m3") is not None
    }
    terminal_days = sorted(set(cand_terminal) & set(ref_terminal))
    terminal_l1 = sum(abs(cand_terminal[day] - ref_terminal[day]) for day in terminal_days)
    terminal_ref_total = sum(abs(ref_terminal[day]) for day in terminal_days)
    terminal_rel = terminal_l1 / max(terminal_ref_total, 1.0e-12)

    cand_shapes = {
        (row["sim_day_index"], row["lane_index"]): row
        for row in cand_rows
        if float(row.get("source_m3") or 0.0) > 1.0e-12
    }
    ref_shapes = {
        (row["sim_day_index"], row["lane_index"]): row
        for row in ref_rows
        if float(row.get("source_m3") or 0.0) > 1.0e-12
    }
    shape_keys = sorted(set(cand_shapes) | set(ref_shapes))
    max_shape_l1 = 0.0
    shape_exceedances = 0
    for key in shape_keys:
        cand_weights = [float(value) for value in cand_shapes.get(key, {}).get("routed_hourly_weights", [0.0] * 24)]
        ref_weights = [float(value) for value in ref_shapes.get(key, {}).get("routed_hourly_weights", [0.0] * 24)]
        delta = l1(cand_weights, ref_weights)
        max_shape_l1 = max(max_shape_l1, delta)
        if delta > 0.05:
            shape_exceedances += 1

    cand_trace = candidate.get("trace_summary") or {}
    ref_trace = reference.get("trace_summary") or {}
    source_scale = max(float(ref_trace.get("source_total_m3") or 0.0), 1.0e-12)
    end_storage_rel = abs(
        float(cand_trace.get("end_storage_total_m3") or 0.0)
        - float(ref_trace.get("end_storage_total_m3") or 0.0)
    ) / source_scale
    tail_fold_rel = abs(
        float(cand_trace.get("tail_fold_total_m3") or 0.0)
        - float(ref_trace.get("tail_fold_total_m3") or 0.0)
    ) / source_scale
    uniform_increase = int(cand_trace.get("uniform_shape_rows") or 0) - int(
        ref_trace.get("uniform_shape_rows") or 0
    )
    degenerate_increase = int(cand_trace.get("erosion_source_shape_degenerate_rows") or 0) - int(
        ref_trace.get("erosion_source_shape_degenerate_rows") or 0
    )
    pass_delta = annual_pass_delta(candidate, reference)
    return {
        "candidate_rung": candidate["rung"],
        "reference_rung": reference["rung"],
        "terminal_days_compared": len(terminal_days),
        "terminal_outlet_l1_m3": terminal_l1,
        "terminal_outlet_l1_rel": terminal_rel,
        "terminal_outlet_passes_1pct": terminal_rel <= 0.01,
        "positive_shape_lane_days_compared": len(shape_keys),
        "max_shape_l1": max_shape_l1,
        "shape_exceedances_gt_0p05": shape_exceedances,
        "shape_passes_0p05": shape_exceedances == 0,
        "end_storage_delta_rel_source": end_storage_rel,
        "end_storage_passes_1pct_source": end_storage_rel <= 0.01,
        "tail_fold_delta_rel_source": tail_fold_rel,
        "tail_fold_passes_1pct_source": tail_fold_rel <= 0.01,
        "uniform_shape_row_increase": uniform_increase,
        "uniform_shape_passes_no_increase": uniform_increase <= 0,
        "degenerate_shape_row_increase": degenerate_increase,
        "degenerate_shape_passes_no_increase": degenerate_increase <= 0,
        "annual_pass_sediment": pass_delta,
    }


def build_comparisons(runs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_member: dict[str, dict[str, dict[str, Any]]] = {}
    for run in runs:
        by_member.setdefault(run["member_id"], {})[run["rung"]] = run
    comparisons = []
    skips = []
    def has_trace(run: dict[str, Any] | None) -> bool:
        return bool(
            run
            and run.get("status") == "PASS"
            and run.get("trace_path")
            and Path(run["trace_path"]).is_file()
        )

    for member_id, rungs in by_member.items():
        if has_trace(rungs.get("dx2p5")) and has_trace(rungs.get("dx1p25")):
            adequacy = compare_run_to_reference(rungs["dx2p5"], rungs["dx1p25"])
            adequacy["member_id"] = member_id
            adequacy["comparison_role"] = "fine_reference_adequacy"
            comparisons.append(adequacy)
        else:
            skips.append({
                "member_id": member_id,
                "comparison_role": "fine_reference_adequacy",
                "reason": "dx2p5 or dx1p25 reference rung failed or lacks trace output",
            })
        if has_trace(rungs.get("dx1p25")) and has_trace(rungs.get("dx0p625")):
            adequacy = compare_run_to_reference(rungs["dx1p25"], rungs["dx0p625"])
            adequacy["member_id"] = member_id
            adequacy["comparison_role"] = "fine_reference_adequacy_dx1p25_vs_dx0p625"
            comparisons.append(adequacy)
        elif "dx0p625" in rungs or "dx1p25" in rungs:
            skips.append({
                "member_id": member_id,
                "comparison_role": "fine_reference_adequacy_dx1p25_vs_dx0p625",
                "reason": "dx1p25 or dx0p625 reference rung failed or lacks trace output",
            })
        if not has_trace(rungs.get("dx2p5")):
            reference = None
        else:
            reference = rungs["dx2p5"]
        if reference is not None:
            for rung in ["baseline_fixed10", "dx20", "dx10", "dx5"]:
                if not has_trace(rungs.get(rung)):
                    continue
                comparison = compare_run_to_reference(rungs[rung], reference)
                comparison["member_id"] = member_id
                comparison["comparison_role"] = "candidate_vs_dx2p5_reference"
                comparisons.append(comparison)
        if has_trace(rungs.get("dx1p25")):
            reference = rungs["dx1p25"]
            for rung in ["baseline_fixed10", "dx20", "dx10", "dx5", "dx2p5"]:
                if not has_trace(rungs.get(rung)):
                    continue
                comparison = compare_run_to_reference(rungs[rung], reference)
                comparison["member_id"] = member_id
                comparison["comparison_role"] = "candidate_vs_dx1p25_reference"
                comparisons.append(comparison)
    comparisons.extend({"comparison_skip": skip} for skip in skips)
    return comparisons


def active_env(target_dx_m: float | None) -> dict[str, str]:
    env = os.environ.copy()
    env.pop("OPENWEPP_LANED_SHADOW", None)
    env.pop("OPENWEPP_LANED_ACTIVE_IMPLICIT", None)
    env.pop("OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M", None)
    env["OPENWEPP_LANED_ACTIVE"] = "1"
    env["OPENWEPP_LANED_ACTIVE_TRACE"] = "1"
    env["OPENWEPP_LANED_ACTIVE_STEP_TRACE"] = "1"
    env["OPENWEPP_LANED_SHADOW_PROFILE"] = "1"
    if target_dx_m is not None:
        env["OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M"] = str(target_dx_m)
    return env


def copy_run_dir_for_rung(member: dict[str, Any], rung: str) -> tuple[Path, str, Path]:
    """Create a package-local run_dir copy with outputs redirected locally."""
    source_run_dir = Path(member["run_dir"])
    rung_root = RUN_ROOT / member["member_id"] / rung
    isolated_run_dir = rung_root / "run_dir"
    if rung_root.exists():
        shutil.rmtree(rung_root)
    shutil.copytree(
        source_run_dir,
        isolated_run_dir,
        ignore=shutil.ignore_patterns("output", "output-*"),
    )
    normalize_copied_text_inputs(isolated_run_dir)
    run_file = Path(member["plain_run_file"]).name
    run_file_path = isolated_run_dir / run_file
    output_dir = isolated_run_dir / "output"
    output_dir.mkdir(parents=True, exist_ok=True)

    def redirect(match: re.Match[str]) -> str:
        key = match.group(1)
        basename = Path(match.group(2)).name
        return f'{key} = "output/{basename}"'

    updated = OUTPUT_LINE.sub(redirect, run_file_path.read_text())
    run_file_path.write_text(updated)
    return isolated_run_dir, run_file, output_dir


def run_member_rung(member: dict[str, Any], rung: str, target_dx_m: float | None) -> dict[str, Any]:
    member_id = member["member_id"]
    run_dir, run_file, output_dir = copy_run_dir_for_rung(member, rung)
    log_path = RUN_ROOT / member_id / rung / "time.log"
    log_path.parent.mkdir(parents=True, exist_ok=True)
    env = active_env(target_dx_m)
    material_environment = {
        key: env[key]
        for key in [
            "OPENWEPP_LANED_ACTIVE",
            "OPENWEPP_LANED_ACTIVE_TRACE",
            "OPENWEPP_LANED_ACTIVE_TRACE_DETAIL",
            "OPENWEPP_LANED_ACTIVE_STEP_TRACE",
            "OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M",
            "OPENWEPP_LANED_SHADOW_PROFILE",
        ]
        if key in env
    }
    command = [
        "/usr/bin/time",
        "-v",
        str(BIN),
        "--run-dir",
        str(run_dir),
        "--run-file",
        run_file,
        "--output-dir",
        str(output_dir),
    ]
    with log_path.open("w") as fp:
        result = subprocess.run(
            command,
            cwd=Path.cwd(),
            env=env,
            stdout=fp,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )
    log_text = log_path.read_text(errors="replace") if log_path.is_file() else ""
    manifest_path = output_dir / "openwepp_hillslope_run_manifest.json"
    manifest = read_json(manifest_path)
    hbp = output_file_from_manifest(manifest, ".hbp") if manifest else None
    pass_parquet = output_file_from_manifest(manifest, ".pass.parquet") if manifest else None
    trace = output_file_from_manifest(manifest, "laned_active_trace.jsonl") if manifest else None
    laned = manifest.get("execution_provenance", {}).get("laned_active", {}) if manifest else {}
    failure = parse_clamp_failure(log_text) if result.returncode != 0 else None
    return {
        "member_id": member_id,
        "role": "synthetic_stress" if member_id == "h2637" else "real_cohort",
        "rung": rung,
        "target_dx_m": target_dx_m,
        "status": "PASS" if result.returncode == 0 else "FAIL",
        "exit_code": result.returncode,
        "command": " ".join(command),
        "material_environment": material_environment,
        "run_dir": str(run_dir),
        "run_file": run_file,
        "output_dir": str(output_dir),
        "log_path": str(log_path),
        "manifest_path": str(manifest_path),
        "manifest_exists": manifest is not None,
        "hbp_path": str(hbp) if hbp else None,
        "hbp_sha256": sha256(hbp),
        "pass_parquet_path": str(pass_parquet) if pass_parquet else None,
        "pass_parquet_sha256": sha256(pass_parquet),
        "trace_path": str(trace) if trace else None,
        "trace_sha256": sha256(trace),
        "laned_active": laned,
        "pass_summary": pass_summary(pass_parquet),
        "trace_summary": trace_summary(trace),
        "timing": parse_time_log(log_text),
        "solver_profile": parse_profile(log_text),
        "failure_phase": failure.get("phase") if failure else None,
        "failure_day": failure.get("day") if failure else None,
        "failure_clamp_m3": failure.get("clamp_m3") if failure else None,
        "failure_source_cap_m3": failure.get("source_cap_m3") if failure else None,
        "failure_clamp_source_ratio": failure.get("clamp_source_ratio") if failure else None,
        "failure_threshold": failure.get("threshold") if failure else None,
        "failure_tail": "\n".join(log_text.splitlines()[-40:])
        if result.returncode != 0
        else None,
    }


def write_markdown(summary: dict[str, Any]) -> None:
    git_status = summary["release_binary"]["git_status_short"] or "(clean)"
    lines = [
        "# Raw-Hydrograph Numerics Run Summary",
        "",
        f"Status: RUN-COMPLETION-{summary['status']}. Evidence mode: Ran.",
        "",
        "The status above means every requested process run completed. The day-792",
        "mechanism verdict is recorded separately in",
        "`mechanism-attribution.md`.",
        "",
        "Release binary:",
        "",
        f"- Build: `{summary['release_binary']['build_command']}`",
        f"- Path: `{summary['release_binary']['path']}`",
        f"- SHA256: `{summary['release_binary']['sha256']}`",
        f"- Git HEAD: `{summary['release_binary']['git_head']}`",
        "- Git status short:",
        "",
        "```text",
        git_status,
        "```",
        "",
        "Material run environment is recorded per rung in the JSON summary under",
        "`material_environment`. The runner forces active routing, active trace",
        "output, opt-in selected day/lane step trace, shadow profiling, and rung-specific",
        "`OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M`; this package also supplied",
        "`OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1` and",
        "`OPENWEPP_LANED_ACTIVE_STEP_TRACE=1`.",
        "",
        "| Member | Rung | Status | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |",
        "|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for rec in summary["runs"]:
        laned = rec.get("laned_active", {})
        profile = rec.get("solver_profile") or {}
        pass_sum = rec.get("pass_summary") or {}
        trace_sum = rec.get("trace_summary") or {}
        timing = rec.get("timing", {})
        lines.append(
            "| {member} | {rung} | {status} | {failure_phase} | {failure_day} | {failure_ratio} | {wall} | {user} | {steps} | {trace_rows} | {outlet} | {storage} | {tail} | {tdet} |".format(
                member=rec["member_id"],
                rung=rec["rung"],
                status=rec["status"],
                failure_phase=rec.get("failure_phase") or "n/a",
                failure_day=rec.get("failure_day") or "n/a",
                failure_ratio=rec.get("failure_clamp_source_ratio") or "n/a",
                wall=timing.get("wall_seconds_raw") or "n/a",
                user=timing.get("user_seconds") or "n/a",
                steps=profile.get("solver_steps", "n/a"),
                trace_rows=trace_sum.get("rows", "n/a"),
                outlet=laned.get("total_routed_outlet_m3", "n/a"),
                storage=laned.get("total_end_window_storage_m3", "n/a"),
                tail=laned.get("total_tail_fold_m3", "n/a"),
                tdet=pass_sum.get("tdet_sum", "n/a"),
            )
        )
    lines.extend(
        [
            "",
            "Comparisons:",
            "",
            "| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |",
            "|---|---|---|---|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for comparison in summary.get("comparisons", []):
        if "comparison_skip" in comparison:
            skip = comparison["comparison_skip"]
            lines.append(
                "| {member} | {role} | SKIPPED | SKIPPED | n/a | n/a | n/a | n/a | n/a | n/a |".format(
                    member=skip["member_id"],
                    role=f"{skip['comparison_role']}: {skip['reason']}",
                )
            )
            continue
        annual = comparison.get("annual_pass_sediment") or {}
        lines.append(
            "| {member} | {role} | {candidate} | {reference} | {outlet:.6g} | {shape:.6g} | {shape_exceed} | {storage:.6g} | {tail:.6g} | {sed:.6g} |".format(
                member=comparison["member_id"],
                role=comparison["comparison_role"],
                candidate=comparison["candidate_rung"],
                reference=comparison["reference_rung"],
                outlet=comparison["terminal_outlet_l1_rel"],
                shape=comparison["max_shape_l1"],
                shape_exceed=comparison["shape_exceedances_gt_0p05"],
                storage=comparison["end_storage_delta_rel_source"],
                tail=comparison["tail_fold_delta_rel_source"],
                sed=annual.get("max_rel", 0.0),
            )
        )
    lines.extend(
        [
            "",
            "Detailed JSON:",
            "",
            f"- `{SUMMARY_JSON}`",
        ]
    )
    SUMMARY_MD.write_text("\n".join(lines) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--members", nargs="*", help="member ids to run")
    parser.add_argument("--rungs", nargs="*", choices=RUNGS.keys(), help="rungs to run")
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument(
        "--expect-fail-guard",
        help="Treat runs as passing only when every rung fails with this guard phase",
    )
    args = parser.parse_args()

    if not args.skip_build:
        build_command = ["cargo", "build", "--release", "-p", "openwepp-runner", "--bins"]
        subprocess.run(build_command, check=True)
    else:
        build_command = ["(skipped)"]
    members = json.loads(MATERIALIZATION.read_text())
    if args.members:
        wanted = set(args.members)
        members = [member for member in members if member["member_id"] in wanted]
    rung_items = [(rung, RUNGS[rung]) for rung in (args.rungs or RUNGS.keys())]
    runs = []
    for member in members:
        for rung, target_dx_m in rung_items:
            runs.append(run_member_rung(member, rung, target_dx_m))
    comparisons = build_comparisons(runs)
    if args.expect_fail_guard:
        expected_fail_guard = all(
            run["status"] == "FAIL"
            and run.get("failure_phase") == args.expect_fail_guard
            and (run.get("failure_clamp_source_ratio") or 0.0) > 1.0
            for run in runs
        )
        status = "PASS_EXPECTED_FAIL" if expected_fail_guard else "FAIL"
    else:
        status = "PASS" if all(run["status"] == "PASS" for run in runs) else "FAIL"
    summary = {
        "status": status,
        "status_scope": "run_completion_only",
        "expected_fail_guard": args.expect_fail_guard,
        "created_utc": datetime.now(timezone.utc).isoformat(),
        "release_binary": binary_provenance(build_command),
        "rungs": RUNGS,
        "runs": runs,
        "comparisons": comparisons,
    }
    SUMMARY_JSON.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    write_markdown(summary)
    print(json.dumps({"status": summary["status"], "runs": len(runs), "summary": str(SUMMARY_JSON)}))
    raise SystemExit(0 if summary["status"] in {"PASS", "PASS_EXPECTED_FAIL"} else 1)


if __name__ == "__main__":
    main()
