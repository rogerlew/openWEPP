#!/usr/bin/env python3
"""Launch one authenticated CAL-04B external-DAG transaction."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
PLAN = PACKAGE / "artifacts/external-dag-transaction-plan.json"
TRANSACTIONS = ("calibration-v1", "holdout-v1")


def planner_binary(execution_root: Path) -> Path:
    target = execution_root.with_name(f"{execution_root.name}.planner-target")
    environment = {
        **os.environ,
        "CARGO_TARGET_DIR": str(target),
    }
    subprocess.run(
        [
            "cargo",
            "build",
            "-p",
            "openwepp-gate-planner",
            "--bin",
            "openwepp-gate-plan",
        ],
        cwd=ROOT,
        env=environment,
        check=True,
    )
    return target / "debug/openwepp-gate-plan"


def command(options: argparse.Namespace, binary: Path) -> list[str]:
    execution_root = options.execution_root
    control_root = options.control_root
    transaction_id = options.transaction_id
    argv = [
        str(binary),
        "run-external-transition",
        "--repo",
        str(ROOT),
        "--external-plan",
        str(options.external_plan),
        "--transaction-id",
        transaction_id,
        "--attempt-root",
        str(execution_root),
        "--ledger",
        str(control_root / "ledger.jsonl"),
        "--output",
        str(control_root / f"{transaction_id}.receipt.json"),
        "--principal",
        options.principal,
        "--repository",
        options.repository,
        "--source-event",
        options.source_event,
        "--source-ref",
        options.source_ref,
        "--workflow",
        options.workflow,
        "--job",
        options.job,
        "--runner",
        options.runner,
        "--attempt",
        str(options.attempt),
    ]
    if transaction_id == "holdout-v1":
        custody_root = options.custody_root.resolve(strict=True)
        argv.extend(
            [
                "--custody-root",
                str(custody_root),
                "--opening-token",
                str(custody_root / "holdout-opened-once.lock"),
            ]
        )
    return argv


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def read_canonical_json(path: Path) -> tuple[bytes, dict[str, object]]:
    raw = path.read_bytes()
    value = json.loads(raw)
    if not isinstance(value, dict):
        raise ValueError(f"JSON custody object is not an object: {path}")
    canonical = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    if raw.strip() != canonical:
        raise ValueError(f"JSON custody object is not canonical: {path}")
    return raw, value


def git_output(repository: Path, *arguments: str, binary: bool = False) -> bytes | str:
    result = subprocess.run(
        ["git", *arguments],
        cwd=repository,
        check=True,
        capture_output=True,
    )
    return result.stdout if binary else result.stdout.decode("utf-8").strip()


def authenticated_source_identity(base_plan: Path) -> dict[str, str]:
    repository = Path(
        str(git_output(base_plan.parent, "rev-parse", "--show-toplevel"))
    ).resolve(strict=True)
    relative = base_plan.resolve(strict=True).relative_to(repository).as_posix()
    committed = git_output(repository, "show", f"HEAD:{relative}", binary=True)
    if committed != base_plan.read_bytes():
        raise ValueError("Generation-A plan bytes are not committed at HEAD")
    status = git_output(
        repository,
        "status",
        "--porcelain=v1",
        "-z",
        "--untracked-files=all",
        binary=True,
    )
    if status:
        raise ValueError("Generation-B requires a clean authenticated source checkout")
    return {
        "head": str(git_output(repository, "rev-parse", "HEAD")),
        "tree": str(git_output(repository, "rev-parse", "HEAD^{tree}")),
        "diff_sha256": sha256_bytes(status),
    }


def derived_id(value: dict[str, object], field: str) -> str:
    payload = dict(value)
    payload.pop(field, None)
    return sha256_bytes(
        json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
    )


def confined_custody_path(root: Path, path: Path) -> str:
    resolved = path.resolve(strict=True)
    try:
        relative = resolved.relative_to(root)
    except ValueError:
        raise ValueError(f"custody input escapes custody root: {path}") from None
    if path.is_symlink() or not path.is_file():
        raise ValueError(f"custody input is not a regular file: {path}")
    return relative.as_posix()


def verified_attestation(
    custody_root: Path, path: Path, freeze_digest: str
) -> tuple[str, dict[str, object]]:
    relative = confined_custody_path(custody_root, path)
    raw, value = read_canonical_json(path)
    if (
        value.get("schema") != "openwepp-external-verifier-attestation-v1"
        or value.get("attestation_id") != derived_id(value, "attestation_id")
        or value.get("freeze_digest") != freeze_digest
    ):
        raise ValueError(f"verifier attestation identity differs: {path}")
    command_id = Path(relative).stem
    if command_id not in {"freeze_verify_a", "freeze_verify_b"}:
        raise ValueError("verifier attestation filename does not bind command identity")
    capability_hash = str(value.get("capability_hash", ""))
    capability = custody_root / "capabilities" / f"{capability_hash}.cap"
    capability_metadata = capability.lstat()
    if (
        capability.is_symlink()
        or not capability.is_file()
        or capability_metadata.st_nlink != 1
        or sha256_bytes(capability.read_bytes()) != capability_hash
    ):
        raise ValueError("verifier capability preimage differs")
    verifier_id = command_id.removeprefix("freeze_verify_")
    receipt = custody_root / "freeze-receipts" / f"verifier_{verifier_id}.csv"
    if sha256_bytes(receipt.read_bytes()) != value.get("receipt_sha256"):
        raise ValueError("verifier attestation receipt bytes differ")
    script = PACKAGE / "tools/freeze-verify.py"
    if sha256_bytes(script.read_bytes()) != value.get("script_sha256"):
        raise ValueError("verifier attestation script bytes differ")
    argv = value.get("argv")
    if not isinstance(argv, list) or "--verifier-id" not in argv:
        raise ValueError("verifier attestation argv is incomplete")
    return relative, value


def build_generation_b(
    base_plan: Path,
    calibration_receipt: Path,
    freeze_receipt: Path,
    attestation_paths: list[Path],
    custody_root: Path,
) -> dict[str, object]:
    base_raw, plan = read_canonical_json(base_plan)
    source_identity = authenticated_source_identity(base_plan)
    if (
        plan.get("schema") != "openwepp-external-dag-plan-v1"
        or plan.get("generation") != "A"
        or plan.get("parent_plan") is not None
        or plan.get("source_identity") is not None
    ):
        raise ValueError("base external plan schema differs")
    transactions = plan.get("transactions")
    if not isinstance(transactions, list):
        raise ValueError("base external plan transaction inventory differs")
    holdout = next(
        (
            transaction
            for transaction in transactions
            if isinstance(transaction, dict)
            and transaction.get("transaction_id") == "holdout-v1"
        ),
        None,
    )
    if holdout is None or holdout.get("custody_prerequisites") or holdout.get(
        "custody_receipts"
    ):
        raise ValueError("base plan is not the sealed Generation-A plan")

    calibration_raw, calibration = read_canonical_json(calibration_receipt)
    if (
        calibration.get("transaction_id") != "calibration-v1"
        or calibration.get("result") != "PASS"
        or not isinstance(calibration.get("receipt_id"), str)
        or len(calibration["receipt_id"]) != 64
    ):
        raise ValueError("calibration transaction receipt is not passing")
    freeze_relative = confined_custody_path(custody_root, freeze_receipt)
    freeze_raw, freeze = read_canonical_json(freeze_receipt)
    freeze_digest = freeze.get("freeze_digest")
    if (
        freeze.get("result") != "PASS"
        or not isinstance(freeze.get("freeze_receipt_id"), str)
        or freeze["freeze_receipt_id"]
        != derived_id(freeze, "freeze_receipt_id")
        or not isinstance(freeze_digest, str)
        or len(freeze_digest) != 64
    ):
        raise ValueError("freeze receipt is not passing or digest-bound")

    if len(attestation_paths) != 2:
        raise ValueError("exactly two verifier attestations are required")
    attestations = [
        verified_attestation(custody_root, path, freeze_digest)
        for path in attestation_paths
    ]
    values = [value for _relative, value in attestations]
    distinct_fields = ("attestation_id", "capability_hash", "agent_task_id", "principal")
    if any(len({value[field] for value in values}) != 2 for field in distinct_fields):
        raise ValueError("verifier attestations are duplicate or replayed")
    execution_claims = {
        (
            value["workflow"],
            value["job"],
            value["runner"],
            value["attempt"],
        )
        for value in values
    }
    if len(execution_claims) != 2:
        raise ValueError("verifier execution claims are duplicate or replayed")
    if len({value["parent_dispatch_id"] for value in values}) != 1:
        raise ValueError("verifier attestations do not share one parent dispatch")

    calibration_destination = custody_root / "calibration-v1.receipt.json"
    if calibration_destination.exists():
        if calibration_destination.read_bytes() != calibration_raw:
            raise ValueError("calibration custody receipt drifted")
    else:
        with calibration_destination.open("xb") as stream:
            stream.write(calibration_raw)
            stream.flush()
            os.fsync(stream.fileno())
    holdout["custody_prerequisites"] = sorted(relative for relative, _value in attestations)
    holdout["custody_receipts"] = [
        {
            "command_id": "summarize_pre_freeze",
            "path": calibration_destination.relative_to(custody_root).as_posix(),
            "sha256": sha256_bytes(calibration_raw),
            "kind": "TRANSACTION",
        },
        {
            "command_id": "freeze",
            "path": freeze_relative,
            "sha256": sha256_bytes(freeze_raw),
            "kind": "FREEZE",
        },
    ]
    plan["generation"] = "B"
    plan["parent_plan"] = {
        "path": str(base_plan.resolve(strict=True)),
        "plan_id": plan["plan_id"],
        "sha256": sha256_bytes(base_raw),
    }
    plan["source_identity"] = source_identity
    plan.pop("plan_id", None)
    plan["plan_id"] = derived_id(plan, "plan_id")
    return plan


def generate_holdout_plan_main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-plan", type=Path, required=True)
    parser.add_argument("--calibration-receipt", type=Path, required=True)
    parser.add_argument("--freeze-receipt", type=Path, required=True)
    parser.add_argument("--attestation", type=Path, action="append", required=True)
    parser.add_argument("--custody-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    options = parser.parse_args(argv)
    custody_root = options.custody_root.resolve(strict=True)
    output = options.output.resolve(strict=False)
    try:
        output.relative_to(ROOT)
    except ValueError:
        pass
    else:
        raise ValueError("Generation-B plan must be written outside the repository")
    plan = build_generation_b(
        options.base_plan.resolve(strict=True),
        options.calibration_receipt.resolve(strict=True),
        options.freeze_receipt.resolve(strict=True),
        [path.resolve(strict=True) for path in options.attestation],
        custody_root,
    )
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("x", encoding="utf-8") as stream:
        json.dump(plan, stream, sort_keys=True, separators=(",", ":"))
        stream.flush()
        os.fsync(stream.fileno())
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--transaction-id", choices=TRANSACTIONS, required=True)
    parser.add_argument("--external-plan", type=Path)
    parser.add_argument("--principal", required=True)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--source-event", required=True)
    parser.add_argument("--source-ref", required=True)
    parser.add_argument("--workflow", required=True)
    parser.add_argument("--job", required=True)
    parser.add_argument("--runner", required=True)
    parser.add_argument("--attempt", type=int, required=True)
    parser.add_argument("--custody-root", type=Path)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=False)
    if execution_root.exists() or not execution_root.parent.is_dir():
        raise ValueError("execution root must be a fresh path below an existing directory")
    options.execution_root = execution_root
    if options.transaction_id == "holdout-v1":
        if options.external_plan is None:
            raise ValueError("holdout transaction requires a Generation-B external plan")
        options.external_plan = options.external_plan.resolve(strict=True)
        try:
            options.external_plan.relative_to(ROOT)
        except ValueError:
            pass
        else:
            raise ValueError("holdout Generation-B plan must remain outside the repository")
    else:
        if options.external_plan not in (None, PLAN):
            raise ValueError("calibration transaction uses the committed Generation-A plan")
        options.external_plan = PLAN
    control_root = execution_root.with_name(f"{execution_root.name}.control")
    control_root.mkdir()
    options.control_root = control_root
    if options.transaction_id == "holdout-v1" and options.custody_root is None:
        raise ValueError("holdout transaction requires an external custody root")
    subprocess.run(command(options, planner_binary(execution_root)), cwd=ROOT, check=True)
    return 0


if __name__ == "__main__":
    try:
        arguments = sys.argv[1:]
        if arguments[:1] == ["generate-holdout-plan"]:
            sys.exit(generate_holdout_plan_main(arguments[1:]))
        sys.exit(main(arguments))
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
