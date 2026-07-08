#!/usr/bin/env python3
"""Run production-default dx5 evidence for the selected real cohort."""

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

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
MATERIALIZATION = Path(
    "docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/"
    "artifacts/selected-cohort-materialization.json"
)
BIN = Path("target/release/openwepp-cli-hill")
RUN_ROOT = ARTIFACTS / "default-dx5-runs"
SUMMARY_JSON = ARTIFACTS / "default-dx5-evidence.json"
SUMMARY_MD = ARTIFACTS / "default-dx5-evidence.md"

REAL_MEMBERS = {"mn_corn_h4", "n_idaho_forest_h1", "wa_cascades_forest_h1"}
OUTPUT_LINE = re.compile(r'^(pass|loss|pass_parquet|wat)\s*=\s*"([^"]+)"', re.MULTILINE)
TEXT_INPUT_SUFFIXES = {".cli", ".man", ".slp", ".sol", ".toml", ".txt"}
OUTPUT_SUFFIXES = [".hbp", ".loss.json", ".pass.parquet", ".wat.parquet", "laned_active_trace.jsonl"]


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


def read_json(path: Path) -> dict[str, Any] | None:
    if not path.is_file():
        return None
    return json.loads(path.read_text())


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


def output_file_from_manifest(manifest: dict[str, Any] | None, suffix: str) -> Path | None:
    if manifest is None:
        return None
    for raw_path in manifest.get("output_checksums", {}):
        if raw_path.endswith(suffix):
            return Path(raw_path)
    return None


def trace_summary(path: Path | None) -> dict[str, Any] | None:
    if path is None or not path.is_file():
        return None
    rows = [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    terminal = [
        row for row in rows
        if row.get("is_terminal_lane") and row.get("terminal_day_outlet_m3") is not None
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
        "erosion_source_shape_degenerate_rows": sum(
            1 for row in rows if row.get("erosion_source_shape_degenerate")
        ),
    }


def clean_laned_env() -> dict[str, str]:
    env = os.environ.copy()
    for key in [
        "OPENWEPP_LANED_ACTIVE",
        "OPENWEPP_LANED_ACTIVE_TRACE",
        "OPENWEPP_LANED_ACTIVE_TRACE_DETAIL",
        "OPENWEPP_LANED_ACTIVE_STEP_TRACE",
        "OPENWEPP_LANED_ACTIVE_MAX_DT_S",
        "OPENWEPP_LANED_ACTIVE_IMPLICIT",
        "OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M",
        "OPENWEPP_LANED_SHADOW",
        "OPENWEPP_LANED_SHADOW_PROFILE",
    ]:
        env.pop(key, None)
    return env


def env_for_mode(mode: str) -> dict[str, str]:
    env = clean_laned_env()
    if mode == "active_default_dx5":
        env["OPENWEPP_LANED_ACTIVE"] = "1"
        env["OPENWEPP_LANED_ACTIVE_TRACE"] = "1"
        env["OPENWEPP_LANED_SHADOW_PROFILE"] = "1"
    elif mode == "active_explicit_dx5":
        env["OPENWEPP_LANED_ACTIVE"] = "1"
        env["OPENWEPP_LANED_ACTIVE_TRACE"] = "1"
        env["OPENWEPP_LANED_SHADOW_PROFILE"] = "1"
        env["OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M"] = "5.0"
    elif mode == "off_default":
        pass
    elif mode == "off_mesh_env_control":
        env["OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M"] = "5.0"
    else:  # pragma: no cover - parser limits modes.
        raise ValueError(mode)
    return env


def copy_run_dir(member: dict[str, Any], mode: str) -> tuple[Path, str, Path]:
    source_run_dir = Path(member["run_dir"])
    mode_root = RUN_ROOT / member["member_id"] / mode
    isolated_run_dir = mode_root / "run_dir"
    if mode_root.exists():
        shutil.rmtree(mode_root)
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

    run_file_path.write_text(OUTPUT_LINE.sub(redirect, run_file_path.read_text()))
    return isolated_run_dir, run_file, output_dir


def output_hashes(manifest: dict[str, Any] | None) -> dict[str, str | None]:
    hashes: dict[str, str | None] = {}
    for suffix in OUTPUT_SUFFIXES:
        path = output_file_from_manifest(manifest, suffix)
        hashes[suffix] = sha256(path)
    return hashes


def run_member_mode(member: dict[str, Any], mode: str) -> dict[str, Any]:
    run_dir, run_file, output_dir = copy_run_dir(member, mode)
    log_path = RUN_ROOT / member["member_id"] / mode / "time.log"
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
            env=env_for_mode(mode),
            stdout=fp,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )
    log_text = log_path.read_text(errors="replace")
    manifest_path = output_dir / "openwepp_hillslope_run_manifest.json"
    manifest = read_json(manifest_path)
    laned = manifest.get("execution_provenance", {}).get("laned_active") if manifest else None
    trace_path = output_file_from_manifest(manifest, "laned_active_trace.jsonl")
    return {
        "member_id": member["member_id"],
        "mode": mode,
        "status": "PASS" if result.returncode == 0 else "FAIL",
        "exit_code": result.returncode,
        "command": " ".join(command),
        "run_dir": str(run_dir),
        "run_file": run_file,
        "output_dir": str(output_dir),
        "log_path": str(log_path),
        "manifest_path": str(manifest_path),
        "manifest_exists": manifest is not None,
        "output_hashes": output_hashes(manifest),
        "laned_active": laned,
        "trace_summary": trace_summary(trace_path),
        "timing": parse_time_log(log_text),
        "solver_profile": parse_profile(log_text),
        "failure_tail": "\n".join(log_text.splitlines()[-40:])
        if result.returncode != 0
        else None,
    }


def compare_modes(runs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_member: dict[str, dict[str, dict[str, Any]]] = {}
    for run in runs:
        by_member.setdefault(run["member_id"], {})[run["mode"]] = run
    comparisons = []
    for member_id, modes in by_member.items():
        for name, left_mode, right_mode, suffixes in [
            (
                "active_default_vs_explicit_dx5",
                "active_default_dx5",
                "active_explicit_dx5",
                OUTPUT_SUFFIXES,
            ),
            (
                "off_default_vs_mesh_env_control",
                "off_default",
                "off_mesh_env_control",
                [".hbp", ".loss.json", ".pass.parquet", ".wat.parquet"],
            ),
        ]:
            left = modes.get(left_mode)
            right = modes.get(right_mode)
            if left is None or right is None:
                comparisons.append({
                    "member_id": member_id,
                    "comparison": name,
                    "status": "FAIL",
                    "reason": "missing mode run",
                })
                continue
            mismatches = []
            for suffix in suffixes:
                left_hash = left["output_hashes"].get(suffix)
                right_hash = right["output_hashes"].get(suffix)
                if left_hash != right_hash:
                    mismatches.append({
                        "suffix": suffix,
                        "left_hash": left_hash,
                        "right_hash": right_hash,
                    })
            comparisons.append({
                "member_id": member_id,
                "comparison": name,
                "left_mode": left_mode,
                "right_mode": right_mode,
                "status": "PASS" if not mismatches else "FAIL",
                "mismatches": mismatches,
            })
    return comparisons


def mesh_policy_assertions(runs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    assertions = []
    for run in runs:
        mode = run["mode"]
        laned = run.get("laned_active")
        if mode.startswith("active_"):
            policy = (laned or {}).get("mesh_policy") or {}
            ok = (
                run["status"] == "PASS"
                and policy.get("mode") == "target_dx"
                and policy.get("target_dx_m") == 5.0
                and policy.get("min_cells") == 10
                and policy.get("max_cells") == 4096
                and (laned or {}).get("max_dt_s") == 300.0
            )
            assertions.append({
                "member_id": run["member_id"],
                "mode": mode,
                "status": "PASS" if ok else "FAIL",
                "observed": laned,
            })
        else:
            ok = run["status"] == "PASS" and laned is None
            assertions.append({
                "member_id": run["member_id"],
                "mode": mode,
                "status": "PASS" if ok else "FAIL",
                "observed": laned,
            })
    return assertions


def closure_assertions(runs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    assertions = []
    for run in runs:
        if not run["mode"].startswith("active_"):
            continue
        laned = run.get("laned_active") or {}
        source_m3 = float(laned.get("total_source_m3") or 0.0)
        clamp_m3 = float(laned.get("total_clamp_m3") or 0.0)
        clamp_rel_source = clamp_m3 / max(source_m3, 1.0)
        ok = (
            run["status"] == "PASS"
            and clamp_rel_source <= 1.0e-12
            and float(laned.get("max_supply_reconstruction_rel") or 1.0) <= 1.0e-12
            and float(laned.get("max_day_cascade_residual_rel") or 1.0) <= 1.0e-10
            and float(laned.get("max_day_seam_residual_rel") or 1.0) <= 1.0e-10
            and float(laned.get("max_day_identity_residual_rel") or 1.0) <= 1.0e-10
        )
        assertions.append({
            "member_id": run["member_id"],
            "mode": run["mode"],
            "status": "PASS" if ok else "FAIL",
            "observed": {
                key: laned.get(key)
                for key in [
                    "total_clamp_m3",
                    "max_supply_reconstruction_rel",
                    "max_day_cascade_residual_rel",
                    "max_day_seam_residual_rel",
                    "max_day_identity_residual_rel",
                    "days_seen",
                    "days_routed",
                    "total_source_m3",
                    "total_routed_outlet_m3",
                    "total_end_window_storage_m3",
                    "total_tail_fold_m3",
                    "total_latqcc_outlet_m3",
                    "lane_days_erosion_source_shape_degenerate",
                ]
            }
            | {"total_clamp_rel_source": clamp_rel_source},
        })
    return assertions


def write_markdown(summary: dict[str, Any]) -> None:
    git_status = summary["release_binary"]["git_status_short"] or "(clean)"
    lines = [
        "# Default DX5 Runtime Evidence",
        "",
        f"Status: {summary['status']}. Evidence mode: Ran.",
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
        "| Member | Mode | Status | Wall | User | Mesh mode | Target dx | Max dt | Steps | Trace rows | Clamp m3 | Max seam residual | HBP | Pass parquet |",
        "|---|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---|---|",
    ]
    for rec in summary["runs"]:
        laned = rec.get("laned_active") or {}
        policy = laned.get("mesh_policy") or {}
        timing = rec.get("timing") or {}
        profile = rec.get("solver_profile") or {}
        trace = rec.get("trace_summary") or {}
        hashes = rec.get("output_hashes") or {}
        lines.append(
            "| {member} | {mode} | {status} | {wall} | {user} | {mesh_mode} | {dx} | {dt} | {steps} | {trace_rows} | {clamp} | {seam} | `{hbp}` | `{parquet}` |".format(
                member=rec["member_id"],
                mode=rec["mode"],
                status=rec["status"],
                wall=timing.get("wall_seconds_raw") or "n/a",
                user=timing.get("user_seconds") or "n/a",
                mesh_mode=policy.get("mode", "n/a"),
                dx=policy.get("target_dx_m", "n/a"),
                dt=laned.get("max_dt_s", "n/a"),
                steps=profile.get("solver_steps", "n/a"),
                trace_rows=trace.get("rows", "n/a"),
                clamp=laned.get("total_clamp_m3", "n/a"),
                seam=laned.get("max_day_seam_residual_rel", "n/a"),
                hbp=(hashes.get(".hbp") or "n/a")[:12],
                parquet=(hashes.get(".pass.parquet") or "n/a")[:12],
            )
        )
    lines.extend(["", "Identity comparisons:", ""])
    lines.extend([
        "| Member | Comparison | Status | Mismatches |",
        "|---|---|---:|---:|",
    ])
    for comparison in summary["identity_comparisons"]:
        lines.append(
            f"| {comparison['member_id']} | {comparison['comparison']} | "
            f"{comparison['status']} | {len(comparison.get('mismatches', []))} |"
        )
    lines.extend(["", "Mesh policy assertions:", ""])
    lines.extend(["| Member | Mode | Status |", "|---|---|---:|"])
    for assertion in summary["mesh_policy_assertions"]:
        lines.append(
            f"| {assertion['member_id']} | {assertion['mode']} | {assertion['status']} |"
        )
    lines.extend(["", "Closure assertions:", ""])
    lines.extend(["| Member | Mode | Status |", "|---|---|---:|"])
    for assertion in summary["closure_assertions"]:
        lines.append(
            f"| {assertion['member_id']} | {assertion['mode']} | {assertion['status']} |"
        )
    lines.extend(["", "Detailed JSON:", "", f"- `{SUMMARY_JSON}`"])
    SUMMARY_MD.write_text("\n".join(lines) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--members", nargs="*", help="member ids to run")
    parser.add_argument(
        "--modes",
        nargs="*",
        choices=[
            "active_default_dx5",
            "active_explicit_dx5",
            "off_default",
            "off_mesh_env_control",
        ],
        help="modes to run",
    )
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()

    if not args.skip_build:
        build_command = ["cargo", "build", "--release", "-p", "openwepp-runner", "--bins"]
        subprocess.run(build_command, check=True)
    else:
        build_command = ["(skipped)"]

    members = [m for m in json.loads(MATERIALIZATION.read_text()) if m["member_id"] in REAL_MEMBERS]
    if args.members:
        wanted = set(args.members)
        members = [m for m in members if m["member_id"] in wanted]
    modes = args.modes or [
        "active_default_dx5",
        "active_explicit_dx5",
        "off_default",
        "off_mesh_env_control",
    ]

    runs = [run_member_mode(member, mode) for member in members for mode in modes]
    comparisons = compare_modes(runs)
    mesh_assertions = mesh_policy_assertions(runs)
    closure = closure_assertions(runs)
    status = (
        "PASS"
        if all(run["status"] == "PASS" for run in runs)
        and all(comparison["status"] == "PASS" for comparison in comparisons)
        and all(assertion["status"] == "PASS" for assertion in mesh_assertions)
        and all(assertion["status"] == "PASS" for assertion in closure)
        else "FAIL"
    )
    summary = {
        "schema": "openwepp-dx5-production-default-evidence-v1",
        "status": status,
        "created_utc": datetime.now(timezone.utc).isoformat(),
        "release_binary": binary_provenance(build_command),
        "materialization": str(MATERIALIZATION),
        "members": [member["member_id"] for member in members],
        "modes": modes,
        "runs": runs,
        "identity_comparisons": comparisons,
        "mesh_policy_assertions": mesh_assertions,
        "closure_assertions": closure,
    }
    SUMMARY_JSON.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    write_markdown(summary)
    raise SystemExit(0 if status == "PASS" else 1)


if __name__ == "__main__":
    main()
