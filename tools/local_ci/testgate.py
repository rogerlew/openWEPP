#!/usr/bin/env python3
"""Plan and execute one authoritative TESTGATE increment without a shell."""

from __future__ import annotations

import argparse
import fnmatch
import hashlib
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


PACKAGE_PATH_RE = re.compile(r"^docs/work-packages/[^/]+/package\.md$")


class TestgateError(RuntimeError):
    """Raised when a TESTGATE execution cannot be represented exactly."""


def _atomic_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.tmp")
    temporary.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temporary.replace(path)


def _git(repo: Path, arguments: list[str], *, binary: bool = False) -> bytes | str:
    result = subprocess.run(
        ["git", *arguments],
        cwd=repo,
        check=False,
        capture_output=True,
        text=not binary,
    )
    if result.returncode != 0:
        stderr = result.stderr if isinstance(result.stderr, str) else result.stderr.decode()
        raise TestgateError(stderr.strip() or f"git {' '.join(arguments)} failed")
    return result.stdout


def _resolve_commit(repo: Path, revision: str) -> str:
    value = _git(repo, ["rev-parse", "--verify", "--end-of-options", f"{revision}^{{commit}}"])
    if not isinstance(value, str):
        raise TestgateError("Git commit output is not text")
    return value.strip()


def _changed_paths(repo: Path, base: str, head: str) -> list[str]:
    output = _git(
        repo,
        ["diff", "--name-only", "-z", "--no-renames", base, head, "--"],
        binary=True,
    )
    if not isinstance(output, bytes):
        raise TestgateError("Git path output is not bytes")
    try:
        paths = [item.decode("utf-8") for item in output.split(b"\0") if item]
    except UnicodeDecodeError as error:
        raise TestgateError("changed path is not UTF-8") from error
    return sorted(set(paths), key=lambda path: path.encode("utf-8"))


def _dirty_changed_paths(repo: Path, base: str) -> list[str]:
    tracked = _git(repo, ["diff", "--name-only", "-z", base, "--"], binary=True)
    untracked = _git(
        repo,
        ["ls-files", "--others", "--exclude-standard", "-z"],
        binary=True,
    )
    if not isinstance(tracked, bytes) or not isinstance(untracked, bytes):
        raise TestgateError("Git dirty path output is not bytes")
    try:
        paths = [
            item.decode("utf-8")
            for item in (tracked + untracked).split(b"\0")
            if item
        ]
    except UnicodeDecodeError as error:
        raise TestgateError("changed path is not UTF-8") from error
    return sorted(set(paths), key=lambda path: path.encode("utf-8"))


def _base_text(repo: Path, base: str, path: str) -> str:
    output = _git(repo, ["show", f"{base}:{path}"])
    if not isinstance(output, str):
        raise TestgateError(f"base package is not text: {path}")
    return output


def _declared_write_set(package_text: str) -> list[str]:
    in_write_set = False
    patterns: list[str] = []
    for line in package_text.splitlines():
        if line == "## Declared Write Set":
            in_write_set = True
            continue
        if in_write_set and line.startswith("## "):
            break
        if in_write_set:
            match = re.fullmatch(r"- `([^`]+)`", line)
            if match:
                patterns.append(match.group(1))
    if not patterns:
        raise TestgateError("intent package has no declared write set")
    return patterns


def _path_is_authorized(path: str, patterns: list[str]) -> bool:
    return any(path == pattern or fnmatch.fnmatchcase(path, pattern) for pattern in patterns)


def _intent_authorization(
    repo: Path,
    base: str,
    changed_paths: list[str],
    requested_package: str | None,
) -> dict[str, Any]:
    if not changed_paths:
        raise TestgateError("zero-work increment cannot be admitted")
    changed_packages = sorted(path for path in changed_paths if PACKAGE_PATH_RE.fullmatch(path))
    candidates = [requested_package] if requested_package else changed_packages
    if not candidates:
        raise TestgateError(
            "increment must change its pre-existing work-package package.md or name it explicitly"
        )
    admitted: list[dict[str, Any]] = []
    for package_path in candidates:
        if package_path is None or not PACKAGE_PATH_RE.fullmatch(package_path):
            raise TestgateError(f"invalid intent package path: {package_path}")
        if package_path not in changed_paths:
            raise TestgateError(f"intent package must be updated by the increment: {package_path}")
        try:
            package_text = _base_text(repo, base, package_path)
            patterns = _declared_write_set(package_text)
        except TestgateError:
            if requested_package:
                raise
            continue
        status = next(
            (line.removeprefix("Status:").strip(" `") for line in package_text.splitlines() if line.startswith("Status:")),
            "",
        )
        if "READY" not in status and "ACTIVE" not in status:
            continue
        unauthorized = [
            path for path in changed_paths if not _path_is_authorized(path, patterns)
        ]
        if not unauthorized:
            admitted.append(
                {
                    "package_path": package_path,
                    "package_sha256": hashlib.sha256(package_text.encode("utf-8")).hexdigest(),
                    "declared_write_set": patterns,
                }
            )
    if len(admitted) != 1:
        raise TestgateError(
            f"expected exactly one base-commit work package to authorize the diff; found {len(admitted)}"
        )
    authorization = admitted[0]
    authorization["base_commit"] = base
    authorization["authorized_changed_paths"] = changed_paths
    return authorization


def _invoke(
    arguments: list[str], repo: Path, *, allow_nonpass: bool = False
) -> dict[str, Any]:
    result = subprocess.run(arguments, cwd=repo, check=False, capture_output=True, text=True)
    try:
        value = json.loads(result.stdout)
    except json.JSONDecodeError as error:
        if result.returncode != 0:
            raise TestgateError(result.stderr.strip() or result.stdout.strip()) from error
        raise TestgateError("gate CLI emitted invalid JSON") from error
    if not isinstance(value, dict):
        raise TestgateError("gate CLI result must be an object")
    if result.returncode != 0 and not (
        allow_nonpass and value.get("result") in {"FAIL", "BLOCKED", "INVALID"}
    ):
        raise TestgateError(result.stderr.strip() or result.stdout.strip())
    return value


def observe(args: argparse.Namespace) -> dict[str, Any]:
    repo = args.repo.resolve()
    artifact_root = args.artifact_root.resolve()
    if artifact_root == repo or repo in artifact_root.parents:
        raise TestgateError("artifact root must be outside the repository")
    artifact_root.mkdir(parents=True, exist_ok=True)
    execution_root = artifact_root / "execution"
    execution_root.mkdir(exist_ok=False)
    base = _resolve_commit(repo, args.base)
    head = None if args.dirty else _resolve_commit(repo, args.head)
    authorized_paths = (
        _dirty_changed_paths(repo, base)
        if args.dirty
        else _changed_paths(repo, base, str(head))
    )
    authorization = _intent_authorization(
        repo, base, authorized_paths, args.intent_package
    )
    authorized_path = artifact_root / "authorized-paths.json"
    intent_path = artifact_root / "intent-plan.json"
    terminal_path = artifact_root / "terminal-plan.json"
    receipt_path = artifact_root / "receipt.json"
    _atomic_json(authorized_path, authorized_paths)
    _atomic_json(artifact_root / "intent-authorization.json", authorization)

    common = [
        str(args.binary.resolve()),
        "plan",
        "--repo",
        str(repo),
        "--base",
        base,
        "--boundary",
        args.boundary,
        "--campaign",
        args.campaign,
        "--authorized-paths",
        str(authorized_path),
    ]
    if head is not None:
        common.extend(["--head", head])
    started = time.monotonic_ns()
    intent_result = _invoke(
        [*common, "--stage", "intent", "--output", str(intent_path)], repo
    )
    terminal_result = _invoke(
        [
            *common,
            "--stage",
            "terminal",
            "--predecessor",
            str(intent_result["plan_id"]),
            "--output",
            str(terminal_path),
        ],
        repo,
    )
    planner_ms = (time.monotonic_ns() - started) // 1_000_000
    terminal_plan = json.loads(terminal_path.read_text(encoding="utf-8"))
    execution_result: dict[str, Any] | None = None
    execution_error: str | None = None
    execution_ms: int | None = None
    if args.execute:
        execution_started = time.monotonic_ns()
        try:
            execution_result = _invoke(
                [
                    str(args.binary.resolve()),
                    "run",
                    "--repo",
                    str(repo),
                    "--plan",
                    str(terminal_path),
                    "--artifact-root",
                    str(execution_root),
                    "--output",
                    str(receipt_path),
                    "--principal",
                    args.principal,
                    "--repository",
                    args.repository,
                    "--source-event",
                    args.source_event,
                    "--source-ref",
                    args.source_ref,
                    "--workflow",
                    args.workflow,
                    "--job",
                    args.job,
                    "--runner",
                    args.runner,
                    "--attempt",
                    str(args.attempt),
                ],
                repo,
                allow_nonpass=True,
            )
        except TestgateError as error:
            execution_error = str(error)
        execution_ms = (time.monotonic_ns() - execution_started) // 1_000_000

    observation = {
        "schema_version": "openwepp-testgate-execution-v1",
        "enforcement_status": "PENDING_GITHUB_ATTESTATION",
        "base_commit": base,
        "head_commit": head,
        "comparison_head": "WORKTREE" if args.dirty else head,
        "boundary": args.boundary,
        "campaign_id": args.campaign,
        "changed_paths": authorized_paths,
        "risk_class": terminal_plan["risk"]["class"],
        "reason_codes": terminal_plan["risk"]["reason_codes"],
        "planned_node_count": len(terminal_plan["nodes"]),
        "planned_inventory_count": len(
            {
                item
                for node in terminal_plan["nodes"]
                for item in node["expected_inventory"]["ids"]
            }
        ),
        "planner_wall_time_ms": planner_ms,
        "intent_plan_id": intent_result["plan_id"],
        "terminal_plan_id": terminal_result["plan_id"],
        "execution_requested": args.execute,
        "execution_result": execution_result,
        "execution_error": execution_error,
        "execution_wall_time_ms": execution_ms,
        "authority_status": "LOCAL_RECEIPT_PENDING_GITHUB_ATTESTATION",
        "intent_authorization": authorization,
    }
    _atomic_json(artifact_root / "observation.json", observation)
    if execution_result is not None and receipt_path.is_file():
        receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
        predicate = {
            "schema_version": "openwepp-testgate-attestation-v1",
            "base_commit": base,
            "head_commit": head,
            "intent_authorization": authorization,
            "receipt_sha256": hashlib.sha256(receipt_path.read_bytes()).hexdigest(),
            "receipt_plan_id": receipt.get("plan_id"),
            "receipt_execution_key": receipt.get("execution_key"),
            "receipt_result": execution_result.get("result"),
            "receipt_trust_class": receipt.get("claims", {}).get("trust_class"),
            "repository": args.repository,
            "source_ref": args.source_ref,
            "workflow": args.workflow,
            "job": args.job,
            "runner": args.runner,
            "runner_image": os.environ.get("OPENWEPP_RUNNER_IMAGE_ID"),
        }
        _atomic_json(artifact_root / "attestation-predicate.json", predicate)
    return observation


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path(__file__).parents[2])
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--base", required=True)
    parser.add_argument("--head", default="HEAD")
    parser.add_argument(
        "--dirty",
        action="store_true",
        help="Observe the current index/worktree/untracked state instead of a head commit",
    )
    parser.add_argument("--artifact-root", type=Path, required=True)
    parser.add_argument(
        "--intent-package",
        help="Base-commit work package that prospectively authorizes the changed paths",
    )
    parser.add_argument("--boundary", choices=("INCREMENT", "CHECKPOINT", "CAMPAIGN", "RELEASE"), default="INCREMENT")
    parser.add_argument("--campaign", default="TESTGATE-CI-01")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--principal", default=os.environ.get("GITHUB_ACTOR", "developer"))
    parser.add_argument("--repository", default=os.environ.get("GITHUB_REPOSITORY", "rogerlew/openWEPP"))
    parser.add_argument("--source-event", default=os.environ.get("GITHUB_EVENT_NAME", "local"))
    parser.add_argument("--source-ref", default=os.environ.get("GITHUB_REF", "refs/heads/main"))
    parser.add_argument("--workflow", default=os.environ.get("GITHUB_WORKFLOW", "testgate"))
    parser.add_argument("--job", default="openwepp/increment-gates")
    parser.add_argument("--runner", default=os.environ.get("RUNNER_NAME", "local"))
    parser.add_argument("--attempt", type=int, default=int(os.environ.get("GITHUB_RUN_ATTEMPT", "1")))
    return parser.parse_args()


def main() -> int:
    try:
        observation = observe(_parse_args())
    except (OSError, KeyError, ValueError, TestgateError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2
    print(json.dumps(observation, sort_keys=True))
    execution = observation["execution_result"]
    accepted = not observation["execution_requested"] or (
        isinstance(execution, dict)
        and execution.get("result") in {"PASS", "PASS_WITH_RETRY"}
    )
    return 0 if observation["execution_error"] is None and accepted else 1


if __name__ == "__main__":
    raise SystemExit(main())
