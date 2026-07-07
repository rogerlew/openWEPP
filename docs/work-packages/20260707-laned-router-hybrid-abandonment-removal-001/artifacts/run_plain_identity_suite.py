#!/usr/bin/env python3
"""Run the ADR-0037 active-plain pre/post identity suite."""

from __future__ import annotations

import argparse
import hashlib
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
MATERIALIZATION = ARTIFACTS / "plain-identity-materialization.json"
BIN = Path("target/release/openwepp-cli-hill")


def sha256(path: Path) -> str | None:
    if not path.is_file():
        return None
    digest = hashlib.sha256()
    with path.open("rb") as fp:
        for chunk in iter(lambda: fp.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def parse_time_log(text: str) -> dict[str, str | None]:
    def find(pattern: str) -> str | None:
        match = re.search(pattern, text)
        return match.group(1) if match else None

    return {
        "wall_seconds": find(r"Elapsed \(wall clock\) time.*: ([0-9:.]+)"),
        "user_seconds": find(r"User time \(seconds\): ([0-9.]+)"),
        "sys_seconds": find(r"System time \(seconds\): ([0-9.]+)"),
    }


def run_logged(command: list[str], log_path: Path, env: dict[str, str] | None = None) -> int:
    log_path.parent.mkdir(parents=True, exist_ok=True)
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


def output_file_from_manifest(manifest: dict[str, Any], suffix: str) -> Path | None:
    checksums = manifest.get("output_checksums", {})
    for raw_path in checksums:
        if raw_path.endswith(suffix):
            return Path(raw_path)
    return None


def read_json(path: Path) -> dict[str, Any] | None:
    if not path.is_file():
        return None
    return json.loads(path.read_text())


def env_for_active_plain() -> dict[str, str]:
    env = os.environ.copy()
    env.pop("OPENWEPP_LANED_SHADOW", None)
    env.pop("OPENWEPP_LANED_ACTIVE_IMPLICIT", None)
    env["OPENWEPP_LANED_ACTIVE"] = "1"
    env["OPENWEPP_LANED_SHADOW_PROFILE"] = "1"
    return env


def git_text(args: list[str]) -> str:
    return subprocess.check_output(["git", *args], text=True).strip()


def binary_provenance() -> dict[str, Any]:
    stat = BIN.stat()
    return {
        "path": str(BIN),
        "sha256": sha256(BIN),
        "size_bytes": stat.st_size,
        "mtime_utc": datetime.fromtimestamp(stat.st_mtime, timezone.utc).isoformat(),
        "git_head": git_text(["rev-parse", "HEAD"]),
        "git_branch": git_text(["branch", "--show-current"]),
        "git_status_short": git_text(["status", "--short"]),
    }


def phase_label(phase: str) -> str:
    return "baseline" if phase == "pre" else "after"


def artifact_stem(phase: str) -> str:
    return f"plain-identity-{phase_label(phase)}"


def summarize_member(member: dict[str, Any], phase: str, log_dir: Path) -> dict[str, Any]:
    run_dir = Path(member["run_dir"])
    output_dir = run_dir / f"output-{phase}"
    if output_dir.exists():
        shutil.rmtree(output_dir)
    run_file = member[f"{phase}_run_file"]
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
    log_path = log_dir / f"{member['member_id']}-{phase}.time.log"
    exit_code = run_logged(command, log_path, env_for_active_plain())
    log_text = log_path.read_text(errors="replace") if log_path.is_file() else ""
    manifest_path = output_dir / "openwepp_hillslope_run_manifest.json"
    manifest = read_json(manifest_path)
    hbp = output_file_from_manifest(manifest, ".hbp") if manifest else None
    pass_parquet = output_file_from_manifest(manifest, ".pass.parquet") if manifest else None
    laned = (
        manifest.get("execution_provenance", {}).get("laned_active", {})
        if manifest
        else {}
    )
    return {
        "member_id": member["member_id"],
        "phase": phase,
        "status": "PASS" if exit_code == 0 else "FAIL",
        "exit_code": exit_code,
        "command": " ".join(command),
        "log_path": str(log_path),
        "run_dir": str(run_dir),
        "run_file": run_file,
        "output_dir": str(output_dir),
        "manifest_path": str(manifest_path),
        "manifest_exists": manifest is not None,
        "hbp_path": str(hbp) if hbp else None,
        "pass_parquet_path": str(pass_parquet) if pass_parquet else None,
        "hbp_sha256": sha256(hbp) if hbp else None,
        "pass_parquet_sha256": sha256(pass_parquet) if pass_parquet else None,
        "laned_active": laned,
        "timing": parse_time_log(log_text),
        "failure_detail": log_text.splitlines()[0]
        if exit_code != 0 and log_text.splitlines()
        else None,
    }


def compare_after(current: dict[str, Any], baseline_path: Path) -> dict[str, Any]:
    baseline = json.loads(baseline_path.read_text())
    by_member = {record["member_id"]: record for record in baseline["members"]}
    comparisons = []
    for record in current["members"]:
        base = by_member.get(record["member_id"])
        comparisons.append(
            {
                "member_id": record["member_id"],
                "hbp_identical": bool(
                    base
                    and base.get("hbp_sha256") is not None
                    and base.get("hbp_sha256") == record.get("hbp_sha256")
                ),
                "pass_parquet_identical": bool(
                    base
                    and base.get("pass_parquet_sha256") is not None
                    and base.get("pass_parquet_sha256")
                    == record.get("pass_parquet_sha256")
                ),
                "baseline_hbp_sha256": base.get("hbp_sha256") if base else None,
                "after_hbp_sha256": record.get("hbp_sha256"),
                "baseline_pass_parquet_sha256": base.get("pass_parquet_sha256")
                if base
                else None,
                "after_pass_parquet_sha256": record.get("pass_parquet_sha256"),
            }
        )
    return {
        "baseline_path": str(baseline_path),
        "comparisons": comparisons,
        "all_identical": all(
            item["hbp_identical"] and item["pass_parquet_identical"]
            for item in comparisons
        ),
    }


def write_markdown(summary: dict[str, Any], path: Path) -> None:
    comparison = summary.get("comparison")
    git_status = summary["release_binary"]["git_status_short"] or "(clean)"
    lines = [
        f"# Plain Identity {phase_label(summary['phase']).title()}",
        "",
        f"Status: {summary['status']}. Evidence mode: Ran.",
        "",
        "Release binary:",
        "",
        f"- Path: `{summary['release_binary']['path']}`",
        f"- SHA256: `{summary['release_binary']['sha256']}`",
        f"- Git HEAD: `{summary['release_binary']['git_head']}`",
        "- Git status short:",
        "",
        "```text",
        git_status,
        "```",
        "",
        "| Member | Status | Wall | User | Sys | HBP SHA256 | Pass parquet SHA256 |",
        "|---|---:|---:|---:|---:|---|---|",
    ]
    for record in summary["members"]:
        timing = record["timing"]
        lines.append(
            "| {member} | {status} | {wall} | {user} | {sys_time} | `{hbp}` | `{pass_hash}` |".format(
                member=record["member_id"],
                status=record["status"],
                wall=timing.get("wall_seconds") or "",
                user=timing.get("user_seconds") or "",
                sys_time=timing.get("sys_seconds") or "",
                hbp=record.get("hbp_sha256"),
                pass_hash=record.get("pass_parquet_sha256"),
            )
        )
    if comparison:
        lines.extend(
            [
                "",
                "Pre/post comparison:",
                "",
                "| Member | HBP identical | Pass parquet identical |",
                "|---|---:|---:|",
            ]
        )
        for item in comparison["comparisons"]:
            lines.append(
                f"| {item['member_id']} | {item['hbp_identical']} | {item['pass_parquet_identical']} |"
            )
    lines.extend(["", f"Detailed JSON: `{path.with_suffix('.json')}`", ""])
    path.write_text("\n".join(lines))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("phase", choices=("pre", "post"))
    args = parser.parse_args()

    log_dir = ARTIFACTS / f"{artifact_stem(args.phase)}-logs"
    if log_dir.exists():
        shutil.rmtree(log_dir)
    log_dir.mkdir(parents=True)

    build_command = ["cargo", "build", "--release", "-p", "openwepp-runner", "--bins"]
    build_log = log_dir / "build.log"
    build_exit = run_logged(build_command, build_log)
    if build_exit != 0:
        print(f"release build failed; see {build_log}", file=sys.stderr)
        return build_exit

    members = json.loads(MATERIALIZATION.read_text())
    summary: dict[str, Any] = {
        "schema": "adr0037-active-plain-identity-v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "phase": args.phase,
        "status": "PASS",
        "release_binary": binary_provenance(),
        "build": {
            "command": " ".join(build_command),
            "log_path": str(build_log),
            "exit_code": build_exit,
        },
        "members": [],
    }
    for member in members:
        record = summarize_member(member, args.phase, log_dir)
        summary["members"].append(record)
        if record["exit_code"] != 0:
            summary["status"] = "FAIL"
            break

    if args.phase == "post" and summary["status"] == "PASS":
        baseline_path = ARTIFACTS / "plain-identity-baseline.json"
        comparison = compare_after(summary, baseline_path)
        summary["comparison"] = comparison
        if not comparison["all_identical"]:
            summary["status"] = "FAIL"

    json_path = ARTIFACTS / f"{artifact_stem(args.phase)}.json"
    md_path = ARTIFACTS / f"{artifact_stem(args.phase)}.md"
    json_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    write_markdown(summary, md_path)
    print(json.dumps({"summary": str(md_path), "status": summary["status"]}))
    return 0 if summary["status"] == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
