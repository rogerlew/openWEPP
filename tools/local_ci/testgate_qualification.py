#!/usr/bin/env python3
"""Black-box controller for the versioned TESTGATE workflow qualification matrix."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


class QualificationError(RuntimeError):
    """Raised when qualification evidence cannot be represented exactly."""


def _read(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise QualificationError(f"expected JSON object: {path}")
    return value


def _write(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.tmp")
    temporary.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temporary.replace(path)


def _digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _derived(value: dict[str, Any], field: str) -> str:
    payload = {key: item for key, item in value.items() if key != field}
    canonical = json.dumps(payload, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def _git(repo: Path, arguments: list[str]) -> str:
    result = subprocess.run(
        ["git", *arguments], cwd=repo, check=False, capture_output=True, text=True
    )
    if result.returncode != 0:
        raise QualificationError(result.stderr.strip() or "git command failed")
    return result.stdout.strip()


def _verify_freeze(repo: Path, freeze: dict[str, Any]) -> None:
    if freeze.get("schema_version") != "openwepp-testgate-subject-freeze-v1":
        raise QualificationError("unsupported subject-freeze schema")
    if freeze.get("subject_freeze_id") != _derived(freeze, "subject_freeze_id"):
        raise QualificationError("subject-freeze identity mismatch")
    for item in freeze.get("paths", []):
        path = repo / item["path"]
        if not path.is_file() or _digest(path) != item["sha256"]:
            raise QualificationError(f"frozen subject changed: {item['path']}")


def validate(args: argparse.Namespace) -> dict[str, Any]:
    repo = args.repo.resolve()
    commit = _git(repo, ["rev-parse", "--verify", f"{args.implementation_commit}^{{commit}}"])
    schema = _read(args.matrix_schema.resolve())
    helper = (repo / "tools/local_ci/testgate.py").read_text(encoding="utf-8")
    controller = Path(__file__).read_text(encoding="utf-8")
    cli = (repo / "crates/openwepp-gate-planner/src/main.rs").read_text(encoding="utf-8")
    required = [
        "validate-package",
        "pre-heavy-audit",
        '"--stage", "light"',
        '"--stage", "heavy"',
        "verify_receipt",
    ]
    surface = helper + controller + cli
    missing = [token for token in required if token not in surface]
    result = "PASS" if not missing else "FAIL"
    report = {
        "schema_version": "openwepp-testgate-interface-validation-v1",
        "result": result,
        "implementation_commit": commit,
        "matrix_schema_id": schema.get("$id"),
        "helper_sha256": _digest(repo / "tools/local_ci/testgate.py"),
        "controller_sha256": _digest(Path(__file__)),
        "missing_real_path_tokens": missing,
    }
    _write(args.output.resolve(), report)
    return report


def run_matrix(args: argparse.Namespace) -> dict[str, Any]:
    repo = args.repo.resolve()
    freeze = _read(args.subject_freeze.resolve())
    controller = _read(args.controller_input.resolve())
    _verify_freeze(repo, freeze)
    root = args.artifact_root.resolve()
    root.mkdir(parents=True, exist_ok=False)
    reports = []
    stopped_after = None
    for case in controller.get("cases", []):
        _verify_freeze(repo, freeze)
        case_root = root / case["case_id"]
        forbidden = {"--repo", "--binary", "--artifact-root"}
        if any(argument in forbidden for argument in case["helper_arguments"]):
            raise QualificationError("case may not replace frozen helper entry-point options")
        command = [
            sys.executable,
            str(repo / "tools/local_ci/testgate.py"),
            "--repo",
            str(repo),
            "--binary",
            str(args.binary.resolve()),
            "--artifact-root",
            str(case_root),
            *case["helper_arguments"],
        ]
        completed = subprocess.run(
            command, cwd=repo, check=False, capture_output=True, text=True
        )
        observed = None
        for line in reversed(completed.stdout.splitlines()):
            try:
                candidate = json.loads(line)
            except json.JSONDecodeError:
                continue
            if isinstance(candidate, dict):
                observed = candidate.get("execution_result", candidate).get("result")
                break
        spawn_file = case_root / "heavy-spawn-count.json"
        heavy_spawns = _read(spawn_file).get("count", 0) if spawn_file.is_file() else 0
        expected = case["expected_status"]
        case_result = (
            "PASS"
            if observed == expected
            and heavy_spawns == case["expected_heavy_spawn_count"]
            else "FAIL"
        )
        artifacts = [
            {"path": path.relative_to(root).as_posix(), "sha256": _digest(path)}
            for path in sorted(case_root.rglob("*"))
            if path.is_file()
        ]
        report = {
            "schema_version": "openwepp-testgate-qualification-case-v1",
            "case_id": case["case_id"],
            "command": command,
            "exit_code": completed.returncode,
            "expected_status": expected,
            "observed_status": observed,
            "expected_heavy_spawn_count": case["expected_heavy_spawn_count"],
            "observed_heavy_spawn_count": heavy_spawns,
            "artifacts": artifacts,
            "result": case_result,
        }
        _write(case_root / "case-report.json", report)
        reports.append(report)
        if case_result != "PASS":
            stopped_after = case["case_id"]
            break
    aggregate = {
        "schema_version": "openwepp-testgate-workflow-qualification-v1",
        "qualification_report_id": "0" * 64,
        "subject_freeze_id": freeze["subject_freeze_id"],
        "matrix_id": controller["matrix_id"],
        "cases": reports,
        "stopped_after": stopped_after,
        "result": "PASS" if stopped_after is None and len(reports) == len(controller["cases"]) else "FAIL",
    }
    aggregate["qualification_report_id"] = _derived(
        aggregate, "qualification_report_id"
    )
    _write(args.output.resolve(), aggregate)
    return aggregate


def verify(args: argparse.Namespace) -> dict[str, Any]:
    repo = args.repo.resolve()
    freeze = _read(args.subject_freeze.resolve())
    report = _read(args.report.resolve())
    _verify_freeze(repo, freeze)
    if report.get("qualification_report_id") != _derived(
        report, "qualification_report_id"
    ):
        raise QualificationError("qualification report identity mismatch")
    if report.get("subject_freeze_id") != freeze.get("subject_freeze_id"):
        raise QualificationError("qualification report used another subject freeze")
    root = args.artifact_root.resolve()
    for case in report.get("cases", []):
        if case.get("result") != "PASS":
            raise QualificationError(f"case did not pass: {case.get('case_id')}")
        for artifact in case.get("artifacts", []):
            path = root / artifact["path"]
            if not path.is_file() or _digest(path) != artifact["sha256"]:
                raise QualificationError(f"artifact digest mismatch: {artifact['path']}")
    if report.get("result") != "PASS":
        raise QualificationError("aggregate qualification did not pass")
    return {"schema_version": "openwepp-testgate-qualification-verdict-v1", "result": "PASS"}


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    validate_parser = commands.add_parser("validate")
    validate_parser.add_argument("--repo", type=Path, required=True)
    validate_parser.add_argument("--implementation-commit", required=True)
    validate_parser.add_argument("--matrix-schema", type=Path, required=True)
    validate_parser.add_argument("--output", type=Path, required=True)
    run_parser = commands.add_parser("run")
    run_parser.add_argument("--repo", type=Path, required=True)
    run_parser.add_argument("--binary", type=Path, required=True)
    run_parser.add_argument("--subject-freeze", type=Path, required=True)
    run_parser.add_argument("--controller-input", type=Path, required=True)
    run_parser.add_argument("--artifact-root", type=Path, required=True)
    run_parser.add_argument("--output", type=Path, required=True)
    verify_parser = commands.add_parser("verify")
    verify_parser.add_argument("--repo", type=Path, required=True)
    verify_parser.add_argument("--subject-freeze", type=Path, required=True)
    verify_parser.add_argument("--report", type=Path, required=True)
    verify_parser.add_argument("--artifact-root", type=Path, required=True)
    return parser


def main() -> int:
    args = _parser().parse_args()
    try:
        if args.command == "validate":
            result = validate(args)
        elif args.command == "run":
            result = run_matrix(args)
        else:
            result = verify(args)
    except (OSError, KeyError, ValueError, QualificationError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2
    print(json.dumps(result, sort_keys=True))
    return 0 if result.get("result") == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
