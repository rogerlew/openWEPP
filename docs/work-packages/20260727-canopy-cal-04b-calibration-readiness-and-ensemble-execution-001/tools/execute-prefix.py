#!/usr/bin/env python3
"""Run CAL-04B package commands directly with durable failure evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
PLAN = PACKAGE / "artifacts/direct-execution-plan.json"
SCHEMA = "cal04b-direct-execution-plan-v1"
EXPECTED_PHASES = {
    "calibration": [
        "prepare",
        "build_executor",
        "build_production_runner",
        "native_proof",
        "synthetic_gsi",
        "hubbard_producer",
        "hubbard_primary_reconstruct",
        "hubbard_verify_reconstruct",
        "retain_trace",
        "readiness",
        "summarize_pre_freeze",
    ],
    "custody": ["freeze", "freeze_verify_a", "freeze_verify_b"],
    "holdout": [
        "freeze_barrier",
        "holdout",
        "summarize_post_holdout",
        "terminal_validate",
    ],
}
ALLOWED_EXECUTABLES = {
    "cargo",
    "${REPO}/.venv/bin/python",
    "${CARGO_TARGET_DIR}/release/native-producer",
    "${CARGO_TARGET_DIR}/release/reconstruct",
    "${CARGO_TARGET_DIR}/release/verify-reconstruct",
    "${CARGO_TARGET_DIR}/release/readiness",
}


@dataclass(frozen=True)
class Context:
    execution_root: Path
    attempt_root: Path
    publication_root: Path
    cargo_target_dir: Path
    evidence_root: Path
    custody_root: Path | None = None


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def canonical_bytes(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def fsync_directory(path: Path) -> None:
    descriptor = os.open(path, os.O_RDONLY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def write_exclusive_json(path: Path, value: object) -> None:
    encoded = canonical_bytes(value) + b"\n"
    with path.open("xb") as stream:
        stream.write(encoded)
        stream.flush()
        os.fsync(stream.fileno())
    fsync_directory(path.parent)


def append_jsonl(path: Path, value: object) -> None:
    encoded = canonical_bytes(value) + b"\n"
    with path.open("ab") as stream:
        stream.write(encoded)
        stream.flush()
        os.fsync(stream.fileno())
    fsync_directory(path.parent)


def load_plan(path: Path = PLAN) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if (
        not isinstance(value, dict)
        or value.get("schema") != SCHEMA
        or value.get("planner_state") is not False
        or value.get("ci") is not False
    ):
        raise ValueError("direct execution plan authority fields differ")
    phases = value.get("phases")
    if not isinstance(phases, dict) or list(phases) != list(EXPECTED_PHASES):
        raise ValueError("direct execution phases are missing")
    identifiers: list[str] = []
    orders: list[int] = []
    for phase, nodes in phases.items():
        if not isinstance(nodes, list) or [
            node.get("command_id") for node in nodes if isinstance(node, dict)
        ] != EXPECTED_PHASES[phase]:
            raise ValueError("direct execution phase inventory differs")
        for node in nodes:
            if not isinstance(node, dict):
                raise ValueError("direct execution command is malformed")
            required = {
                "command_id": str,
                "order": int,
                "argv": list,
                "cwd": str,
                "env": dict,
                "source_path": str,
                "declared_outputs": list,
                "prerequisites": list,
                "harvard_access": str,
                "timeout_seconds": int,
            }
            if any(not isinstance(node.get(key), kind) for key, kind in required.items()):
                raise ValueError(f"direct execution command fields differ: {node}")
            identifiers.append(node["command_id"])
            orders.append(node["order"])
            argv = node.get("argv")
            if (
                not isinstance(argv, list)
                or not argv
                or any(not isinstance(item, str) or not item for item in argv)
            ):
                raise ValueError(f"invalid argv for {node['command_id']}")
            if argv[0] not in ALLOWED_EXECUTABLES:
                raise ValueError(f"executable is outside the literal allowlist: {argv[0]}")
            if (
                node["cwd"] != "${REPO}"
                or node["timeout_seconds"] <= 0
                or any(not isinstance(item, str) for item in node["declared_outputs"])
                or any(not isinstance(item, str) for item in node["prerequisites"])
                or node["prerequisites"]
                != ([] if not identifiers[:-1] else [identifiers[-2]])
            ):
                raise ValueError(f"direct command boundary differs: {node['command_id']}")
            expected_harvard = (
                "OPENS_HARVARD"
                if node["command_id"] == "holdout"
                else "NONE"
            )
            if node["harvard_access"] != expected_harvard:
                raise ValueError(f"Harvard policy differs: {node['command_id']}")
    if len(identifiers) != len(set(identifiers)):
        raise ValueError("direct execution command IDs are duplicated")
    if orders != list(range(1, 19)):
        raise ValueError("direct execution order is not exactly 1 through 18")
    return value


def prepare_context(execution_root: Path) -> Context:
    root = execution_root.resolve(strict=False)
    if root.exists() or not root.parent.is_dir():
        raise ValueError("execution root must be a fresh path below an existing directory")
    attempt = root.parent
    publication = attempt / "publication"
    cargo_target = attempt / "cargo-target"
    evidence = attempt / "direct-evidence"
    for path in (root, publication, cargo_target, evidence):
        path.mkdir(parents=True, exist_ok=False)
    return Context(root, attempt, publication, cargo_target, evidence)


def replacements(context: Context) -> dict[str, str]:
    values = {
        "${REPO}": str(ROOT),
        "${OBJECTS_ROOT}": str(context.execution_root),
        "${PUBLICATION_ROOT}": str(context.publication_root),
        "${CARGO_TARGET_DIR}": str(context.cargo_target_dir),
    }
    if context.custody_root is not None:
        values["${CUSTODY_ROOT}"] = str(context.custody_root)
    return values


def expand(value: str, context: Context) -> str:
    expanded = value
    for token, replacement in replacements(context).items():
        expanded = expanded.replace(token, replacement)
    if "${" in expanded:
        raise ValueError(f"unresolved direct execution operand: {expanded}")
    return expanded


def command_record(
    node: dict[str, Any],
    context: Context,
    *,
    started: str,
    finished: str,
    exit_code: int | None,
    state: str,
    stdout_path: Path,
    stderr_path: Path,
    error: str | None = None,
) -> dict[str, object]:
    source = Path(expand(str(node["source_path"]), context))
    if not source.is_absolute():
        source = ROOT / source
    argv = [expand(item, context) for item in node["argv"]]
    cwd = Path(expand(str(node["cwd"]), context)).resolve()
    environment = {
        key: expand(str(value), context)
        for key, value in dict(node.get("env", {})).items()
    }
    record: dict[str, object] = {
        "schema": "cal04b-direct-command-evidence-v1",
        "command_id": node["command_id"],
        "order": node["order"],
        "state": state,
        "argv": argv,
        "cwd": str(cwd),
        "environment": environment,
        "source_path": str(source),
        "source_sha256": sha256_file(source),
        "started_at": started,
        "finished_at": finished,
        "exit_code": exit_code,
        "stdout_path": str(stdout_path),
        "stdout_sha256": sha256_file(stdout_path),
        "stderr_path": str(stderr_path),
        "stderr_sha256": sha256_file(stderr_path),
        "declared_outputs": [
            expand(str(path), context) for path in node.get("declared_outputs", [])
        ],
        "harvard_access": node["harvard_access"],
    }
    if error is not None:
        record["error"] = error
    return record


def execute_node(node: dict[str, Any], context: Context) -> dict[str, object]:
    command_id = str(node["command_id"])
    stdout_path = context.evidence_root / f"{node['order']}-{command_id}.stdout.log"
    stderr_path = context.evidence_root / f"{node['order']}-{command_id}.stderr.log"
    evidence_path = context.evidence_root / f"{node['order']}-{command_id}.json"
    for path in (stdout_path, stderr_path, evidence_path):
        if path.exists():
            raise ValueError(f"direct execution evidence already exists: {path}")
    argv = [expand(item, context) for item in node["argv"]]
    cwd = Path(expand(str(node["cwd"]), context)).resolve()
    environment = os.environ.copy()
    environment.update(
        {
            key: expand(str(value), context)
            for key, value in dict(node.get("env", {})).items()
        }
    )
    started = datetime.now(timezone.utc).isoformat()
    state = "FAIL"
    exit_code: int | None = None
    error: str | None = None
    with stdout_path.open("xb") as stdout, stderr_path.open("xb") as stderr:
        try:
            result = subprocess.run(
                argv,
                cwd=cwd,
                env=environment,
                stdout=stdout,
                stderr=stderr,
                timeout=int(node["timeout_seconds"]),
                check=False,
            )
            exit_code = result.returncode
            state = "PASS" if result.returncode == 0 else "FAIL"
        except subprocess.TimeoutExpired:
            state = "TIMEOUT"
            error = "command timed out"
        except OSError as command_error:
            state = "ERROR"
            error = f"{type(command_error).__name__}: {command_error}"
            stderr.write((error + "\n").encode())
        finally:
            stdout.flush()
            stderr.flush()
            os.fsync(stdout.fileno())
            os.fsync(stderr.fileno())
    finished = datetime.now(timezone.utc).isoformat()
    record = command_record(
        node,
        context,
        started=started,
        finished=finished,
        exit_code=exit_code,
        state=state,
        stdout_path=stdout_path,
        stderr_path=stderr_path,
        error=error,
    )
    write_exclusive_json(evidence_path, record)
    append_jsonl(context.evidence_root / "command-log.jsonl", record)
    if state != "PASS":
        primary = context.evidence_root / "primary-failure.json"
        if not primary.exists():
            write_exclusive_json(primary, record)
    return record


def run_calibration(context: Context, plan: dict[str, Any]) -> list[dict[str, object]]:
    records: list[dict[str, object]] = []
    for node in plan["phases"]["calibration"]:
        record = execute_node(node, context)
        records.append(record)
        if record["state"] != "PASS":
            raise RuntimeError(
                f"CAL-04B direct command failed: {record['command_id']} "
                f"evidence={context.evidence_root / 'primary-failure.json'}"
            )
    completion = {
        "schema": "cal04b-direct-calibration-completion-v1",
        "state": "PASS",
        "plan_sha256": sha256_file(PLAN),
        "command_log_sha256": sha256_file(context.evidence_root / "command-log.jsonl"),
        "command_ids": [record["command_id"] for record in records],
        "completed_at": datetime.now(timezone.utc).isoformat(),
    }
    write_exclusive_json(context.evidence_root / "calibration-complete.json", completion)
    return records


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    options = parser.parse_args(argv)
    context = prepare_context(options.execution_root)
    run_calibration(context, load_plan())
    print(
        "PASS direct CAL-04B calibration commands "
        f"evidence={context.evidence_root}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
