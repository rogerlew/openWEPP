#!/usr/bin/env python3
"""Record and summarize local cargo-nextest timing evidence.

The history written by this tool is intentionally machine-local. It lives under
target/ so developers and agents can compare their own local CI runs without
committing volatile timing data.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import platform
import re
import socket
import subprocess
import sys
import tempfile
import time
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any


DEFAULT_HISTORY_DIR = Path("target/local-ci-history")
DEFAULT_CONFIG = Path(".config/nextest.toml")


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()


def run_text(command: list[str]) -> str | None:
    try:
        return subprocess.check_output(command, text=True, stderr=subprocess.DEVNULL).strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def git_state() -> dict[str, Any]:
    status = run_text(["git", "status", "--short"])
    return {
        "sha": run_text(["git", "rev-parse", "HEAD"]),
        "branch": run_text(["git", "branch", "--show-current"]) or None,
        "dirty": bool(status),
        "status_short": status or "",
    }


def machine_state() -> dict[str, Any]:
    return {
        "hostname": socket.gethostname(),
        "platform": platform.platform(),
        "python": platform.python_version(),
        "cpu_count": os.cpu_count(),
    }


def resolve_junit_path(profile: str | None, junit: Path | None) -> Path:
    if junit is not None:
        return junit
    return Path("target/nextest") / (profile or "default") / "junit.xml"


def testcase_status(testcase: ET.Element) -> str:
    if testcase.find("failure") is not None:
        return "failed"
    if testcase.find("error") is not None:
        return "error"
    if testcase.find("skipped") is not None:
        return "skipped"
    return "passed"


def parse_junit(path: Path, top: int) -> dict[str, Any]:
    if not path.exists():
        raise FileNotFoundError(f"nextest JUnit file not found: {path}")

    root = ET.parse(path).getroot()
    tests: list[dict[str, Any]] = []
    counts = {"passed": 0, "failed": 0, "error": 0, "skipped": 0}

    for testcase in root.iter("testcase"):
        status = testcase_status(testcase)
        counts[status] += 1
        duration = float(testcase.get("time") or 0.0)
        tests.append(
            {
                "name": testcase.get("name") or "",
                "classname": testcase.get("classname") or "",
                "time_seconds": duration,
                "status": status,
            }
        )

    tests.sort(key=lambda item: item["time_seconds"], reverse=True)
    by_class: dict[str, dict[str, Any]] = {}
    for item in tests:
        classname = item["classname"] or "<unknown>"
        bucket = by_class.setdefault(
            classname,
            {
                "classname": classname,
                "test_count": 0,
                "test_time_seconds": 0.0,
                "slowest_test_seconds": 0.0,
            },
        )
        bucket["test_count"] += 1
        bucket["test_time_seconds"] += item["time_seconds"]
        bucket["slowest_test_seconds"] = max(bucket["slowest_test_seconds"], item["time_seconds"])

    slow_classes = sorted(
        by_class.values(),
        key=lambda item: item["test_time_seconds"],
        reverse=True,
    )

    return {
        "junit_path": str(path),
        "test_count": len(tests),
        "status_counts": counts,
        "test_time_seconds": round(sum(item["time_seconds"] for item in tests), 6),
        "top_tests": tests[:top],
        "top_classes": [
            {
                **item,
                "test_time_seconds": round(item["test_time_seconds"], 6),
                "slowest_test_seconds": round(item["slowest_test_seconds"], 6),
            }
            for item in slow_classes[:top]
        ],
    }


def remove_existing_junit(path: Path) -> None:
    path.unlink(missing_ok=True)


def require_fresh_junit(path: Path, started_wall_ns: int) -> None:
    if not path.exists():
        raise FileNotFoundError(f"nextest JUnit file not found after command: {path}")
    mtime_ns = path.stat().st_mtime_ns
    if mtime_ns < started_wall_ns:
        raise RuntimeError(
            f"nextest JUnit file is stale: {path} mtime_ns={mtime_ns} "
            f"started_ns={started_wall_ns}"
        )


def write_history(record: dict[str, Any], history_dir: Path, write_latest: bool) -> Path:
    history_dir.mkdir(parents=True, exist_ok=True)
    jsonl_path = history_dir / "nextest-runs.jsonl"
    with jsonl_path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(record, sort_keys=True) + "\n")
    if write_latest:
        (history_dir / "latest.json").write_text(
            json.dumps(record, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        (history_dir / "latest.md").write_text(render_markdown(record), encoding="utf-8")
    return jsonl_path


def render_markdown(record: dict[str, Any]) -> str:
    timing = record["nextest"]
    lines = [
        "# Latest local CI nextest timing",
        "",
        f"- Label: `{record['label']}`",
        f"- Timestamp: `{record['timestamp_utc']}`",
        f"- Git: `{record['git'].get('branch') or '<detached>'}` `{record['git'].get('sha')}` dirty={record['git'].get('dirty')}",
        f"- Exit code: `{record.get('exit_code')}`",
        f"- Wall time: `{record.get('wall_time_seconds')}` seconds",
        f"- JUnit: `{timing['junit_path']}`",
        f"- Tests: `{timing['test_count']}`; status={timing['status_counts']}; test-time-sum=`{timing['test_time_seconds']}` seconds",
        "",
        "## Slowest Tests",
        "",
        "| seconds | class | test | status |",
        "|---:|---|---|---|",
    ]
    for item in timing["top_tests"]:
        lines.append(
            f"| {item['time_seconds']:.3f} | `{item['classname']}` | `{item['name']}` | `{item['status']}` |"
        )
    lines.extend(["", "## Slowest Classes", "", "| seconds | class | tests | slowest |", "|---:|---|---:|---:|"])
    for item in timing["top_classes"]:
        lines.append(
            f"| {item['test_time_seconds']:.3f} | `{item['classname']}` | {item['test_count']} | {item['slowest_test_seconds']:.3f} |"
        )
    lines.append("")
    return "\n".join(lines)


def build_record(
    *,
    label: str,
    profile: str | None,
    command: list[str] | None,
    junit: Path,
    top: int,
    wall_time_seconds: float | None,
    exit_code: int | None,
    extra: dict[str, Any] | None = None,
) -> dict[str, Any]:
    record = {
        "schema": "openwepp-local-ci-nextest-timing-v1",
        "timestamp_utc": utc_now(),
        "label": label,
        "profile": profile,
        "command": command,
        "wall_time_seconds": None if wall_time_seconds is None else round(wall_time_seconds, 3),
        "exit_code": exit_code,
        "git": git_state(),
        "machine": machine_state(),
        "nextest": parse_junit(junit, top=top),
    }
    if extra:
        record["extra"] = extra
    return record


def command_summarize(args: argparse.Namespace) -> int:
    junit = resolve_junit_path(args.profile, args.junit)
    record = build_record(
        label=args.label,
        profile=args.profile,
        command=None,
        junit=junit,
        top=args.top,
        wall_time_seconds=None,
        exit_code=None,
    )
    path = write_history(record, args.history_dir, not args.no_latest)
    print(f"recorded timing summary: {path}")
    print(render_markdown(record))
    return 0


def command_run(args: argparse.Namespace) -> int:
    if not args.command:
        raise SystemExit("run requires a command after --")
    junit = resolve_junit_path(args.profile, args.junit)
    remove_existing_junit(junit)
    started_wall_ns = time.time_ns()
    started = time.monotonic()
    completed = subprocess.run(args.command, check=False)
    wall = time.monotonic() - started
    try:
        require_fresh_junit(junit, started_wall_ns)
    except (FileNotFoundError, RuntimeError) as error:
        print(f"error: {error}", file=sys.stderr)
        return completed.returncode if completed.returncode != 0 else 1
    record = build_record(
        label=args.label,
        profile=args.profile,
        command=args.command,
        junit=junit,
        top=args.top,
        wall_time_seconds=wall,
        exit_code=completed.returncode,
    )
    path = write_history(record, args.history_dir, not args.no_latest)
    print(f"recorded timing run: {path}")
    return completed.returncode


def replace_group_cap(config_text: str, group: str, cap: int) -> str:
    header = re.compile(rf"^(\[test-groups\.{re.escape(group)}\]\s*)$", re.MULTILINE)
    match = header.search(config_text)
    if not match:
        raise ValueError(f"test group not found in nextest config: {group}")
    start = match.end()
    next_header = re.search(r"^\[", config_text[start:], flags=re.MULTILINE)
    end = start + next_header.start() if next_header else len(config_text)
    block = config_text[start:end]
    if not re.search(r"^max-threads\s*=", block, flags=re.MULTILINE):
        raise ValueError(f"max-threads not found for test group: {group}")
    block = re.sub(r"^max-threads\s*=.*$", f"max-threads = {cap}", block, count=1, flags=re.MULTILINE)
    return config_text[:start] + block + config_text[end:]


def command_sweep(args: argparse.Namespace) -> int:
    junit = resolve_junit_path(args.profile, args.junit)
    caps = [int(value) for value in args.caps.split(",") if value.strip()]
    if not caps:
        raise SystemExit("--caps must name at least one integer")
    config_text = args.base_config.read_text(encoding="utf-8")
    failures = 0
    for cap in caps:
        patched = replace_group_cap(config_text, args.group, cap)
        for repeat in range(1, args.repeats + 1):
            with tempfile.NamedTemporaryFile("w", suffix=".toml", delete=False, encoding="utf-8") as tmp:
                tmp.write(patched)
                tmp_path = Path(tmp.name)
            try:
                command = [
                    "cargo",
                    "nextest",
                    "run",
                    "--workspace",
                    "--profile",
                    args.profile,
                    "--config-file",
                    str(tmp_path),
                    "--ignore-default-filter",
                    "-E",
                    args.filterset,
                    "--no-fail-fast",
                    "--no-tests",
                    "pass",
                ]
                label = f"sweep:{args.group}:cap{cap}:rep{repeat}"
                print(f"running {label}: {' '.join(command)}", flush=True)
                remove_existing_junit(junit)
                started_wall_ns = time.time_ns()
                started = time.monotonic()
                completed = subprocess.run(command, check=False)
                wall = time.monotonic() - started
                if completed.returncode != 0:
                    failures += 1
                try:
                    require_fresh_junit(junit, started_wall_ns)
                except (FileNotFoundError, RuntimeError) as error:
                    print(f"error: {error}", file=sys.stderr)
                    failures += 1
                    continue
                record = build_record(
                    label=label,
                    profile=args.profile,
                    command=command,
                    junit=junit,
                    top=args.top,
                    wall_time_seconds=wall,
                    exit_code=completed.returncode,
                    extra={
                        "sweep_group": args.group,
                        "sweep_cap": cap,
                        "sweep_repeat": repeat,
                        "filterset": args.filterset,
                    },
                )
                write_history(record, args.history_dir, not args.no_latest)
            finally:
                tmp_path.unlink(missing_ok=True)
    return 1 if failures else 0


def add_common_arguments(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--label", required=True, help="human-readable run label")
    parser.add_argument("--profile", help="nextest profile name")
    parser.add_argument("--junit", type=Path, help="nextest JUnit path; defaults to target/nextest/<profile>/junit.xml")
    parser.add_argument("--history-dir", type=Path, default=DEFAULT_HISTORY_DIR)
    parser.add_argument("--top", type=int, default=20)
    parser.add_argument("--no-latest", action="store_true", help="do not rewrite latest.json/latest.md")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command_name", required=True)

    summarize = subparsers.add_parser("summarize", help="record timing from an existing nextest JUnit file")
    add_common_arguments(summarize)
    summarize.set_defaults(func=command_summarize)

    run = subparsers.add_parser("run", help="run a command, then record its nextest JUnit timing")
    add_common_arguments(run)
    run.add_argument("command", nargs=argparse.REMAINDER, help="command to run; prefix with --")
    run.set_defaults(func=command_run)

    sweep = subparsers.add_parser("sweep", help="benchmark nextest test-group caps with temporary configs")
    sweep.add_argument("--group", required=True, help="nextest test group to patch")
    sweep.add_argument("--caps", required=True, help="comma-separated cap values, e.g. 2,3,4")
    sweep.add_argument("--filterset", required=True, help="nextest filterset expression to run for each cap")
    sweep.add_argument("--profile", default="full")
    sweep.add_argument("--base-config", type=Path, default=DEFAULT_CONFIG)
    sweep.add_argument("--junit", type=Path, help="nextest JUnit path; defaults to target/nextest/<profile>/junit.xml")
    sweep.add_argument("--history-dir", type=Path, default=DEFAULT_HISTORY_DIR)
    sweep.add_argument("--top", type=int, default=20)
    sweep.add_argument("--repeats", type=int, default=1)
    sweep.add_argument("--no-latest", action="store_true")
    sweep.set_defaults(func=command_sweep)

    args = parser.parse_args(argv)
    if getattr(args, "command", None) and args.command[0] == "--":
        args.command = args.command[1:]
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
