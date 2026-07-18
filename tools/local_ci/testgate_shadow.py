#!/usr/bin/env python3
"""Plan and optionally execute one TESTGATE shadow observation without a shell."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


class ShadowError(RuntimeError):
    """Raised when a shadow observation cannot be represented exactly."""


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
        raise ShadowError(stderr.strip() or f"git {' '.join(arguments)} failed")
    return result.stdout


def _resolve_commit(repo: Path, revision: str) -> str:
    value = _git(repo, ["rev-parse", "--verify", "--end-of-options", f"{revision}^{{commit}}"])
    if not isinstance(value, str):
        raise ShadowError("Git commit output is not text")
    return value.strip()


def _changed_paths(repo: Path, base: str, head: str) -> list[str]:
    output = _git(
        repo,
        ["diff", "--name-only", "-z", "--no-renames", base, head, "--"],
        binary=True,
    )
    if not isinstance(output, bytes):
        raise ShadowError("Git path output is not bytes")
    try:
        paths = [item.decode("utf-8") for item in output.split(b"\0") if item]
    except UnicodeDecodeError as error:
        raise ShadowError("changed path is not UTF-8") from error
    return sorted(set(paths), key=lambda path: path.encode("utf-8"))


def _dirty_changed_paths(repo: Path, base: str) -> list[str]:
    tracked = _git(repo, ["diff", "--name-only", "-z", base, "--"], binary=True)
    untracked = _git(
        repo,
        ["ls-files", "--others", "--exclude-standard", "-z"],
        binary=True,
    )
    if not isinstance(tracked, bytes) or not isinstance(untracked, bytes):
        raise ShadowError("Git dirty path output is not bytes")
    try:
        paths = [
            item.decode("utf-8")
            for item in (tracked + untracked).split(b"\0")
            if item
        ]
    except UnicodeDecodeError as error:
        raise ShadowError("changed path is not UTF-8") from error
    return sorted(set(paths), key=lambda path: path.encode("utf-8"))


def _invoke(
    arguments: list[str], repo: Path, *, allow_nonpass: bool = False
) -> dict[str, Any]:
    result = subprocess.run(arguments, cwd=repo, check=False, capture_output=True, text=True)
    try:
        value = json.loads(result.stdout)
    except json.JSONDecodeError as error:
        if result.returncode != 0:
            raise ShadowError(result.stderr.strip() or result.stdout.strip()) from error
        raise ShadowError("gate CLI emitted invalid JSON") from error
    if not isinstance(value, dict):
        raise ShadowError("gate CLI result must be an object")
    if result.returncode != 0 and not (
        allow_nonpass and value.get("result") in {"FAIL", "BLOCKED", "INVALID"}
    ):
        raise ShadowError(result.stderr.strip() or result.stdout.strip())
    return value


def observe(args: argparse.Namespace) -> dict[str, Any]:
    repo = args.repo.resolve()
    artifact_root = args.artifact_root.resolve()
    if artifact_root == repo or repo in artifact_root.parents:
        raise ShadowError("artifact root must be outside the repository")
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
    authorized_path = artifact_root / "authorized-paths.json"
    intent_path = artifact_root / "intent-plan.json"
    terminal_path = artifact_root / "terminal-plan.json"
    receipt_path = artifact_root / "receipt.json"
    _atomic_json(authorized_path, authorized_paths)

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
        except ShadowError as error:
            execution_error = str(error)
        execution_ms = (time.monotonic_ns() - execution_started) // 1_000_000

    observation = {
        "schema_version": "openwepp-testgate-shadow-observation-v1",
        "enforcement_status": "SHADOW_NONBLOCKING",
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
        "cutover_eligible": False,
        "cutover_blockers": [
            "14_CONSECUTIVE_DAYS_NOT_PROVEN",
            "20_REPRESENTATIVE_INCREMENTS_NOT_PROVEN",
            "RETAINED_CAMPAIGN_REPLAY_INCOMPLETE",
            "PROTECTED_CONTEXT_MIGRATION_NOT_PROVEN",
        ],
    }
    _atomic_json(artifact_root / "observation.json", observation)
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
    parser.add_argument("--boundary", choices=("INCREMENT", "CHECKPOINT", "CAMPAIGN", "RELEASE"), default="INCREMENT")
    parser.add_argument("--campaign", default="TESTGATE-CI-01")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--principal", default=os.environ.get("GITHUB_ACTOR", "developer"))
    parser.add_argument("--repository", default=os.environ.get("GITHUB_REPOSITORY", "rogerlew/openWEPP"))
    parser.add_argument("--source-event", default=os.environ.get("GITHUB_EVENT_NAME", "local"))
    parser.add_argument("--source-ref", default=os.environ.get("GITHUB_REF", "refs/heads/main"))
    parser.add_argument("--workflow", default=os.environ.get("GITHUB_WORKFLOW", "testgate-shadow"))
    parser.add_argument("--job", default="testgate-shadow")
    parser.add_argument("--runner", default=os.environ.get("RUNNER_NAME", "local"))
    parser.add_argument("--attempt", type=int, default=int(os.environ.get("GITHUB_RUN_ATTEMPT", "1")))
    return parser.parse_args()


def main() -> int:
    try:
        observation = observe(_parse_args())
    except (OSError, KeyError, ValueError, ShadowError) as error:
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
