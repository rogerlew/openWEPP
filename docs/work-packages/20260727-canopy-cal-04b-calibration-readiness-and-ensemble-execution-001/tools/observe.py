#!/usr/bin/env python3
"""Execute the frozen CAL-04B DAG and retain append-only observed receipts."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
import shlex
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

from custody import sha256_file

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
OBJECTS = Path("/nonexistent/cal04b-execution-root-required")
LEDGER = OBJECTS / "execution-ledger"
PLAN = ARTIFACTS / "executor-command-plan.csv"
CONTRACT = ARTIFACTS / "observed-command-contract.csv"
RECEIPT_FIELDS = [
    "schema",
    "order",
    "command_id",
    "plan_sha256",
    "plan_row_sha256",
    "contract_sha256",
    "source_path",
    "source_sha256",
    "planned_argv",
    "observed_argv_json",
    "working_directory",
    "environment_json",
    "started_at",
    "finished_at",
    "elapsed_ns",
    "exit_code",
    "stdout_path",
    "stdout_sha256",
    "stderr_path",
    "stderr_sha256",
    "output_manifest_path",
    "output_manifest_sha256",
    "output_count",
    "state",
]


def csv_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    if not rows:
        raise ValueError(f"empty observed-execution control: {path}")
    return rows


def controls() -> tuple[list[dict[str, str]], dict[str, dict[str, str]]]:
    plan = csv_rows(PLAN)
    contract_rows = csv_rows(CONTRACT)
    contract = {row["command_id"]: row for row in contract_rows}
    ids = [row["command_id"] for row in plan]
    if len(contract) != len(contract_rows) or set(contract) != set(ids):
        raise ValueError("observed command contract does not exactly cover frozen plan")
    return plan, contract


def row_sha256(row: dict[str, str]) -> str:
    encoded = json.dumps(row, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def stem(row: dict[str, str]) -> str:
    return f"{row['order'].replace('/', '_')}-{row['command_id']}"


def receipt_path(row: dict[str, str]) -> Path:
    return LEDGER / f"{stem(row)}.receipt.csv"


def output_paths(value: str) -> list[Path]:
    if value == "-":
        return []
    paths = [Path(item) for item in value.split(";") if item]
    mapped = []
    legacy_objects = Path("/home/workdir") / "cal04b-objects"
    for path in paths:
        if path.is_absolute():
            try:
                mapped.append(OBJECTS / path.relative_to(legacy_objects))
                continue
            except ValueError:
                pass
        repository_path = path if path.is_absolute() else ROOT / path
        try:
            relative = repository_path.relative_to(ROOT)
        except ValueError:
            mapped.append(repository_path)
        else:
            mapped.append(OBJECTS.parent / "publication" / relative)
    return mapped


def read_receipt(path: Path) -> dict[str, str]:
    rows = csv_rows(path)
    if len(rows) != 1 or list(rows[0]) != RECEIPT_FIELDS:
        raise ValueError(f"invalid observed receipt schema/count: {path}")
    return rows[0]


def validate_receipt(
    plan_row: dict[str, str],
    contract_row: dict[str, str],
) -> dict[str, str]:
    receipt = read_receipt(receipt_path(plan_row))
    if (
        receipt["schema"] != "CAL04B-OBSERVED-EXECUTION-01"
        or receipt["order"] != plan_row["order"]
        or receipt["command_id"] != plan_row["command_id"]
        or receipt["plan_sha256"] != sha256_file(PLAN)
        or receipt["plan_row_sha256"] != row_sha256(plan_row)
        or receipt["contract_sha256"] != sha256_file(CONTRACT)
        or receipt["planned_argv"] != plan_row["argv"]
        or json.loads(receipt["observed_argv_json"]) != shlex.split(plan_row["argv"])
        or receipt["working_directory"] != plan_row["working_directory"]
        or json.loads(receipt["environment_json"])
        != split_environment(shlex.split(plan_row["argv"]))[0]
        or receipt["exit_code"] != "0"
        or receipt["state"] != "PASS"
    ):
        raise ValueError(f"observed receipt differs from plan: {plan_row['command_id']}")
    source = Path(receipt["source_path"])
    if (
        source != Path(plan_row["source_path"])
        or sha256_file(source) != receipt["source_sha256"]
    ):
        raise ValueError(f"observed source identity differs: {plan_row['command_id']}")
    for key in ("stdout", "stderr"):
        path = Path(receipt[f"{key}_path"])
        if sha256_file(path) != receipt[f"{key}_sha256"]:
            raise ValueError(f"observed {key} identity differs: {plan_row['command_id']}")
    output_manifest = Path(receipt["output_manifest_path"])
    if sha256_file(output_manifest) != receipt["output_manifest_sha256"]:
        raise ValueError(f"output manifest identity differs: {plan_row['command_id']}")
    declared = output_paths(contract_row["receipt_outputs"])
    output_rows = csv_rows(output_manifest) if declared else []
    if declared:
        if list(output_rows[0]) != ["path", "sha256", "bytes", "state"]:
            raise ValueError(f"output manifest schema differs: {plan_row['command_id']}")
        if [row["path"] for row in output_rows] != [str(path) for path in declared]:
            raise ValueError(f"declared output inventory differs: {plan_row['command_id']}")
        if any(row["state"] != "OBSERVED" or len(row["sha256"]) != 64 for row in output_rows):
            raise ValueError(f"observed output identity is incomplete: {plan_row['command_id']}")
        for output, output_row in zip(declared, output_rows):
            if (
                not output.is_file()
                or output.stat().st_size != int(output_row["bytes"])
                or sha256_file(output) != output_row["sha256"]
            ):
                raise ValueError(
                    f"observed output changed after receipt: "
                    f"{plan_row['command_id']}:{output}"
                )
    if int(receipt["output_count"]) != len(declared):
        raise ValueError(f"output count differs: {plan_row['command_id']}")
    started = datetime.fromisoformat(receipt["started_at"])
    finished = datetime.fromisoformat(receipt["finished_at"])
    if (
        started.tzinfo is None
        or finished.tzinfo is None
        or finished < started
        or int(receipt["elapsed_ns"]) < 0
    ):
        raise ValueError(f"observed timestamps differ: {plan_row['command_id']}")
    return receipt


def validate_prefix(last_command_id: str) -> list[dict[str, str]]:
    plan, contract = controls()
    ids = [row["command_id"] for row in plan]
    if last_command_id not in ids:
        raise ValueError(f"unknown observed prefix {last_command_id}")
    receipts = []
    for row in plan[: ids.index(last_command_id) + 1]:
        receipts.append(validate_receipt(row, contract[row["command_id"]]))
    return receipts


def validate_snapshot(name: str, last_command_id: str) -> list[dict[str, str]]:
    receipts = validate_prefix(last_command_id)
    plan, _contract = controls()
    snapshot = LEDGER / f"{name}-snapshot.csv"
    snapshot_rows = csv_rows(snapshot)
    if len(snapshot_rows) != len(receipts):
        raise ValueError(f"observed snapshot count differs: {snapshot}")
    for plan_row, receipt, row in zip(plan, receipts, snapshot_rows):
        path = receipt_path(plan_row)
        if (
            row["command_id"] != receipt["command_id"]
            or row["state"] != "PASS"
            or row["receipt_path"] != str(path)
            or row["receipt_sha256"] != sha256_file(path)
        ):
            raise ValueError(f"observed snapshot differs: {receipt['command_id']}")
    return snapshot_rows


def split_environment(tokens: list[str]) -> tuple[dict[str, str], list[str]]:
    environment: dict[str, str] = {}
    index = 0
    while index < len(tokens):
        token = tokens[index]
        if "=" not in token or token.startswith("-"):
            break
        key, value = token.split("=", 1)
        if not key.replace("_", "").isalnum() or key[0].isdigit():
            break
        environment[key] = value
        index += 1
    if index == len(tokens):
        raise ValueError("observed command has no executable")
    return environment, tokens[index:]


def write_output_manifest(
    path: Path, declared: list[Path], success: bool
) -> tuple[int, bool]:
    complete = True
    with path.open("x", newline="", encoding="utf-8") as stream:
        writer = csv.writer(stream, lineterminator="\n")
        writer.writerow(["path", "sha256", "bytes", "state"])
        for output in declared:
            if not output.is_file():
                complete = False
                state = "MISSING_AFTER_SUCCESS" if success else "MISSING_AFTER_FAILURE"
                writer.writerow([str(output), "", "", state])
                continue
            writer.writerow(
                [str(output), sha256_file(output), output.stat().st_size, "OBSERVED"]
            )
        stream.flush()
        os.fsync(stream.fileno())
    return len(declared), complete


def execute(command_id: str, observed_tokens: list[str]) -> int:
    plan, contract = controls()
    by_id = {row["command_id"]: row for row in plan}
    if command_id not in by_id:
        raise ValueError(f"unknown command id {command_id}")
    row = by_id[command_id]
    expected_tokens = shlex.split(row["argv"])
    if observed_tokens != expected_tokens:
        raise ValueError(f"observed argv differs from frozen plan for {command_id}")
    if Path.cwd().resolve() != Path(row["working_directory"]).resolve():
        raise ValueError(f"working directory differs for {command_id}")
    LEDGER.mkdir(parents=True, exist_ok=True)
    receipt = receipt_path(row)
    stdout_path = LEDGER / f"{stem(row)}.stdout.log"
    stderr_path = LEDGER / f"{stem(row)}.stderr.log"
    outputs_path = LEDGER / f"{stem(row)}.outputs.csv"
    for path in (receipt, stdout_path, stderr_path, outputs_path):
        if path.exists():
            raise ValueError(f"append-only execution object already exists: {path}")
    prerequisites = contract[command_id]["prerequisites"]
    if prerequisites != "-":
        for prerequisite in prerequisites.split(";"):
            validate_receipt(by_id[prerequisite], contract[prerequisite])

    plan_digest = sha256_file(PLAN)
    contract_digest = sha256_file(CONTRACT)
    source = Path(row["source_path"])
    source_digest = sha256_file(source)
    environment_delta, executable = split_environment(observed_tokens)
    environment = os.environ.copy()
    environment.update(environment_delta)
    started = datetime.now(timezone.utc)
    start_ns = time.monotonic_ns()
    with stdout_path.open("xb") as stdout, stderr_path.open("xb") as stderr:
        result = subprocess.run(
            executable,
            cwd=row["working_directory"],
            env=environment,
            stdout=stdout,
            stderr=stderr,
            check=False,
        )
        stdout.flush()
        stderr.flush()
        os.fsync(stdout.fileno())
        os.fsync(stderr.fileno())
    finished = datetime.now(timezone.utc)
    elapsed = time.monotonic_ns() - start_ns
    declared = output_paths(contract[command_id]["receipt_outputs"])
    output_count, outputs_complete = write_output_manifest(
        outputs_path, declared, result.returncode == 0
    )
    effective_exit = result.returncode if result.returncode != 0 or outputs_complete else 86
    controls_unchanged = (
        sha256_file(PLAN) == plan_digest
        and sha256_file(CONTRACT) == contract_digest
        and sha256_file(source) == source_digest
    )
    if effective_exit == 0 and not controls_unchanged:
        effective_exit = 87
    receipt_row = {
        "schema": "CAL04B-OBSERVED-EXECUTION-01",
        "order": row["order"],
        "command_id": command_id,
        "plan_sha256": plan_digest,
        "plan_row_sha256": row_sha256(row),
        "contract_sha256": contract_digest,
        "source_path": row["source_path"],
        "source_sha256": source_digest,
        "planned_argv": row["argv"],
        "observed_argv_json": json.dumps(observed_tokens, separators=(",", ":")),
        "working_directory": str(Path.cwd().resolve()),
        "environment_json": json.dumps(environment_delta, sort_keys=True, separators=(",", ":")),
        "started_at": started.isoformat(),
        "finished_at": finished.isoformat(),
        "elapsed_ns": str(elapsed),
        "exit_code": str(effective_exit),
        "stdout_path": str(stdout_path),
        "stdout_sha256": sha256_file(stdout_path),
        "stderr_path": str(stderr_path),
        "stderr_sha256": sha256_file(stderr_path),
        "output_manifest_path": str(outputs_path),
        "output_manifest_sha256": sha256_file(outputs_path),
        "output_count": str(output_count),
        "state": "PASS" if effective_exit == 0 else "FAIL",
    }
    with receipt.open("x", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=RECEIPT_FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerow(receipt_row)
        stream.flush()
        os.fsync(stream.fileno())
    directory = os.open(LEDGER, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)
    sys.stdout.buffer.write(stdout_path.read_bytes())
    sys.stderr.buffer.write(stderr_path.read_bytes())
    return effective_exit


def render(last_command_id: str, snapshot: str | None) -> None:
    receipts = validate_prefix(last_command_id)
    plan, _contract = controls()
    rows = []
    for plan_row, receipt in zip(plan, receipts):
        receipt_file = receipt_path(plan_row)
        rows.append({
            "command_id": receipt["command_id"],
            "state": receipt["state"],
            "working_directory": receipt["working_directory"],
            "planned_command": receipt["planned_argv"],
            "observed_argv_json": receipt["observed_argv_json"],
            "started_at": receipt["started_at"],
            "finished_at": receipt["finished_at"],
            "exit_code": receipt["exit_code"],
            "stdout_sha256": receipt["stdout_sha256"],
            "stderr_sha256": receipt["stderr_sha256"],
            "output_manifest_sha256": receipt["output_manifest_sha256"],
            "receipt_path": str(receipt_file),
            "receipt_sha256": sha256_file(receipt_file),
        })
    fields = list(rows[0])
    if snapshot:
        destination = LEDGER / f"{snapshot}-snapshot.csv"
        mode = "x"
    else:
        destination = ARTIFACTS / "command-log.csv"
        mode = "w"
    with destination.open(mode, newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
        stream.flush()
        os.fsync(stream.fileno())
    if snapshot:
        directory = os.open(LEDGER, os.O_RDONLY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    if not snapshot:
        inventories = ARTIFACTS / "execution-inventory.csv"
        with inventories.open("w", newline="", encoding="utf-8") as stream:
            writer = csv.DictWriter(
                stream,
                fieldnames=["inventory_id", "stage", "state", "command_ids", "evidence"],
                lineterminator="\n",
            )
            writer.writeheader()
            for index, row in enumerate(rows, 1):
                writer.writerow({
                    "inventory_id": f"CAL04B-{index:02d}",
                    "stage": row["command_id"],
                    "state": row["state"],
                    "command_ids": row["command_id"],
                    "evidence": row["receipt_path"],
                })
    print(f"PASS observed ledger rendered commands={len(rows)} destination={destination}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    subparsers = parser.add_subparsers(dest="action", required=True)
    run = subparsers.add_parser("run")
    run.add_argument("--command-id", required=True)
    run.add_argument("command", nargs=argparse.REMAINDER)
    render_parser = subparsers.add_parser("render")
    render_parser.add_argument("--through", required=True)
    render_parser.add_argument("--snapshot")
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    global ARTIFACTS, OBJECTS, LEDGER
    ARTIFACTS = execution_root.parent / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    OBJECTS = execution_root
    LEDGER = execution_root.parent / "legacy-observed-ledger"
    if options.action == "run":
        command = options.command
        if command and command[0] == "--":
            command = command[1:]
        return execute(options.command_id, command)
    render(options.through, options.snapshot)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError, json.JSONDecodeError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
