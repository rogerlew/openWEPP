#!/usr/bin/env python3
"""Run the D16 selected-cohort active plain-vs-hybrid suite."""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
MATERIALIZATION = ARTIFACTS / "selected-cohort-materialization.json"
LOG_DIR = ARTIFACTS / "active-suite-run-logs"
COMMAND_LOG = ARTIFACTS / "active-suite-command-log.json"
SUMMARY_MD = ARTIFACTS / "active-suite-run-summary.md"
BIN = Path("target/release/openwepp-cli-hill")


def parse_time_log(text: str) -> dict[str, str | None]:
    def find(pattern: str) -> str | None:
        match = re.search(pattern, text)
        return match.group(1) if match else None

    return {
        "wall_seconds": find(r"Elapsed \(wall clock\) time.*: ([0-9:.]+)"),
        "user_seconds": find(r"User time \(seconds\): ([0-9.]+)"),
        "sys_seconds": find(r"System time \(seconds\): ([0-9.]+)"),
    }


def env_for_mode(mode: str) -> dict[str, str]:
    env = os.environ.copy()
    env.pop("OPENWEPP_LANED_SHADOW", None)
    env.pop("OPENWEPP_LANED_ACTIVE_IMPLICIT", None)
    env["OPENWEPP_LANED_ACTIVE"] = "1"
    env["OPENWEPP_LANED_SHADOW_PROFILE"] = "1"
    if mode == "hybrid":
        env["OPENWEPP_LANED_ACTIVE_IMPLICIT"] = "1"
    return env


def expected_output_subdir(mode: str) -> str:
    return "output-hybrid" if mode == "hybrid" else "output-plain"


def read_manifest(path: Path) -> dict[str, Any] | None:
    if not path.is_file():
        return None
    return json.loads(path.read_text())


def checksums_under_output_dir(manifest: dict[str, Any] | None, output_dir: Path) -> bool | None:
    if manifest is None:
        return None
    checksums = manifest.get("output_checksums", {})
    return all(Path(raw).is_relative_to(output_dir) for raw in checksums)


def implicit_flag_ok(manifest: dict[str, Any] | None, mode: str) -> bool | None:
    if manifest is None:
        return None
    actual = (
        manifest.get("execution_provenance", {})
        .get("laned_active", {})
        .get("hybrid_implicit_stepping")
    )
    return actual is (mode == "hybrid")


def command_record(
    scope: str,
    command: list[str],
    log_path: Path,
    exit_code: int,
    member_id: str | None = None,
    mode: str | None = None,
    run_dir: Path | None = None,
    run_file: str | None = None,
    output_dir: Path | None = None,
) -> dict[str, Any]:
    text = log_path.read_text(errors="replace") if log_path.is_file() else ""
    timing = parse_time_log(text)
    output_manifest_path = (
        output_dir / "openwepp_hillslope_run_manifest.json" if output_dir else None
    )
    manifest = read_manifest(output_manifest_path) if output_manifest_path else None
    status = "PASS" if exit_code == 0 else "FAIL"
    return {
        "scope": scope,
        "member_id": member_id,
        "mode": mode,
        "status": status,
        "exit_code": exit_code,
        "command": " ".join(command),
        "log_path": str(log_path),
        "run_dir": str(run_dir) if run_dir else None,
        "run_file": run_file,
        "output_dir": str(output_dir) if output_dir else None,
        "output_manifest_path": str(output_manifest_path) if output_manifest_path else None,
        "output_manifest_exists": manifest is not None,
        "output_checksums_under_output_dir": checksums_under_output_dir(manifest, output_dir)
        if output_dir
        else None,
        "hybrid_implicit_flag_ok": implicit_flag_ok(manifest, mode) if mode else None,
        **timing,
        "failure_detail": text.splitlines()[0] if exit_code != 0 and text.splitlines() else None,
    }


def run_logged(command: list[str], log_path: Path, env: dict[str, str] | None = None) -> int:
    LOG_DIR.mkdir(parents=True, exist_ok=True)
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
    return result.returncode


def write_summary(records: list[dict[str, Any]], halted: dict[str, Any] | None) -> None:
    lines = [
        "# Active Suite Run Summary",
        "",
        "Status: EXECUTED-HOLD-ACTIVE-RUN." if halted else "Status: EXECUTED.",
        "Evidence mode: Ran.",
        "",
        "| Scope | Member | Mode | Status | Exit | Wall | User | Sys | Manifest | Output dir check | Hybrid flag check |",
        "|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for record in records:
        lines.append(
            "| {scope} | {member} | {mode} | {status} | {exit_code} | {wall} | {user} | {sys_time} | {manifest} | {outdir} | {hybrid} |".format(
                scope=record["scope"],
                member=record.get("member_id") or "",
                mode=record.get("mode") or "",
                status=record["status"],
                exit_code=record["exit_code"],
                wall=record.get("wall_seconds") or "",
                user=record.get("user_seconds") or "",
                sys_time=record.get("sys_seconds") or "",
                manifest=record.get("output_manifest_exists"),
                outdir=record.get("output_checksums_under_output_dir"),
                hybrid=record.get("hybrid_implicit_flag_ok"),
            )
        )
    if halted:
        lines.extend(
            [
                "",
                "Hold condition:",
                "",
                f"- Member: `{halted['member_id']}`",
                f"- Mode: `{halted['mode']}`",
                f"- Log: `{halted['log_path']}`",
                f"- First failure line: `{halted['failure_detail']}`",
                "",
            ]
        )
    SUMMARY_MD.write_text("\n".join(lines))


def main() -> int:
    if LOG_DIR.exists():
        shutil.rmtree(LOG_DIR)
    LOG_DIR.mkdir(parents=True)

    records: list[dict[str, Any]] = []
    build_log = LOG_DIR / "build.log"
    build_command = ["cargo", "build", "--release", "-p", "openwepp-runner", "--bins"]
    build_exit = run_logged(build_command, build_log)
    records.append(command_record("build", build_command, build_log, build_exit))
    if build_exit != 0:
        write_outputs(records, records[-1])
        return build_exit

    members = json.loads(MATERIALIZATION.read_text())
    halted: dict[str, Any] | None = None
    for member in members:
        run_dir = Path(member["run_dir"])
        for mode, run_file_key in (("plain", "plain_run_file"), ("hybrid", "hybrid_run_file")):
            output_dir = run_dir / expected_output_subdir(mode)
            if output_dir.exists():
                shutil.rmtree(output_dir)
            command = [
                "/usr/bin/time",
                "-v",
                str(BIN),
                "--run-dir",
                str(run_dir),
                "--run-file",
                member[run_file_key],
                "--output-dir",
                str(output_dir),
            ]
            log_path = LOG_DIR / f"{member['member_id']}-{mode}.time.log"
            exit_code = run_logged(command, log_path, env_for_mode(mode))
            record = command_record(
                "run",
                command,
                log_path,
                exit_code,
                member_id=member["member_id"],
                mode=mode,
                run_dir=run_dir,
                run_file=member[run_file_key],
                output_dir=output_dir,
            )
            records.append(record)
            if exit_code != 0:
                halted = record
                write_outputs(records, halted)
                return exit_code

    write_outputs(records, halted)
    return 0


def write_outputs(records: list[dict[str, Any]], halted: dict[str, Any] | None) -> None:
    COMMAND_LOG.write_text(
        json.dumps(
            {
                "schema": "d16-selected-cohort-active-suite-command-log-v1",
                "generated_at": datetime.now(timezone.utc).isoformat(),
                "evidence": "Ran",
                "expected_runs": 8,
                "completed_runs": sum(1 for record in records if record["scope"] == "run"),
                "status": "EXECUTED-HOLD-ACTIVE-RUN" if halted else "EXECUTED",
                "records": records,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n"
    )
    write_summary(records, halted)


if __name__ == "__main__":
    sys.exit(main())
