#!/usr/bin/env python3
"""Supervise the optional forest1 quality-observatory workflow."""

from __future__ import annotations

import argparse
import fcntl
import hashlib
import json
import os
import re
import shutil
import signal
import stat
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


SCHEMA = "openwepp-quality-workflow-control-v1"
OCCUPANCY_SCHEMA = "openwepp-quality-occupancy-v1"
CURRENT_WORKFLOW = ".github/workflows/testgate-shadow.yml"
RETIRED_OMARCHY_WORKFLOW = ".github/workflows/testgate-conservative.yml"
DEFUNCT_OMARCHY_RUN_IDS = {29673299308, 29672334757, 29672149962}
PUBLISHED_FILES = {
    "quality-envelope.json",
    "quality-payload.json",
    "run-status.json",
    "inventory-full.json",
    "inventory-science-manual.json",
    "inventory-workspace.json",
    "junit-full.xml",
    "junit-science-manual.xml",
    "adjudicated-crap-report.json",
    "adjudicated-crap-report.md",
    "coverage-summary.json",
}
MAX_PUBLISHED_BYTES = 100 * 1024 * 1024
MAX_CONTROL_BYTES = 1024 * 1024
SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")


class WorkflowError(RuntimeError):
    """A fail-closed workflow-control error."""


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        allow_nan=False,
        ensure_ascii=False,
        separators=(",", ":"),
        sort_keys=True,
    ).encode()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.tmp")
    temporary.write_bytes(canonical_bytes(value) + b"\n")
    temporary.replace(path)


def read_object(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise WorkflowError(f"cannot read occupancy JSON {path}: {error}") from error
    if not isinstance(value, dict):
        raise WorkflowError("occupancy JSON root must be an object")
    return value


def run_text(arguments: list[str], cwd: Path) -> str:
    result = subprocess.run(
        arguments, cwd=cwd, check=False, capture_output=True, text=True
    )
    if result.returncode:
        raise WorkflowError(
            f"command failed ({result.returncode}): {' '.join(arguments)}: "
            f"{result.stderr.strip()}"
        )
    return result.stdout.strip()


def exact_sha(value: str, field: str) -> str:
    if not SHA40.fullmatch(value):
        raise WorkflowError(f"{field} must be one lowercase 40-character SHA")
    return value


def exact_sha256(value: str, field: str) -> str:
    if not SHA256.fullmatch(value):
        raise WorkflowError(f"{field} must be one lowercase SHA-256")
    return value


def normalize_labels(value: Any) -> set[str]:
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        raise WorkflowError("job labels must be an array of strings")
    return set(value)


def classify_occupancy(snapshot: dict[str, Any], repository: str) -> dict[str, Any]:
    if snapshot.get("schema_version") != OCCUPANCY_SCHEMA:
        raise WorkflowError("occupancy snapshot schema is unsupported")
    runs = snapshot.get("runs")
    if not isinstance(runs, list):
        raise WorkflowError("occupancy snapshot runs must be an array")
    live: list[dict[str, Any]] = []
    ignored_omarchy: list[int] = []
    for item in runs:
        if not isinstance(item, dict):
            raise WorkflowError("occupancy run must be an object")
        required = {"id", "repository", "workflow", "status", "jobs", "artifacts"}
        if set(item) != required:
            raise WorkflowError("occupancy run fields are not exact")
        run_id = item["id"]
        workflow = item["workflow"]
        status = item["status"]
        if (
            not isinstance(run_id, int)
            or not isinstance(item["repository"], str)
            or not isinstance(workflow, str)
            or status
            not in {
                "requested",
                "waiting",
                "pending",
                "queued",
                "in_progress",
                "completed",
                "cancelled",
                "failure",
                "success",
            }
            or not isinstance(item["jobs"], list)
            or not isinstance(item["artifacts"], int)
            or item["artifacts"] < 0
        ):
            raise WorkflowError("occupancy run field type is invalid")
        if item["repository"] != repository or status in {
            "completed",
            "cancelled",
            "failure",
            "success",
        }:
            continue
        jobs: list[dict[str, Any]] = []
        for job in item["jobs"]:
            if not isinstance(job, dict) or set(job) != {"name", "status", "labels"}:
                raise WorkflowError("occupancy job fields are not exact")
            labels = normalize_labels(job["labels"])
            if not isinstance(job["name"], str) or job["status"] not in {
                "queued",
                "in_progress",
                "completed",
            }:
                raise WorkflowError("occupancy job field type is invalid")
            jobs.append({**job, "labels": labels})
        forest1_live = any(
            "forest1" in job["labels"]
            and job["status"] in {"queued", "in_progress"}
            for job in jobs
        )
        retired_omarchy = (
            run_id in DEFUNCT_OMARCHY_RUN_IDS
            and workflow == RETIRED_OMARCHY_WORKFLOW
            and status == "queued"
            and not jobs
            and item["artifacts"] == 0
        )
        if retired_omarchy:
            ignored_omarchy.append(run_id)
            continue
        if workflow == CURRENT_WORKFLOW and (
            forest1_live
            or status in {"requested", "waiting", "pending", "queued", "in_progress"}
        ):
            live.append({"id": run_id, "status": status})
    return {
        "status": "LIVE_TESTGATE" if live else "CLEAR",
        "live_runs": live,
        "ignored_omarchy_runs": ignored_omarchy,
    }


def live_snapshot(repository: str) -> dict[str, Any]:
    runs: list[dict[str, Any]] = []
    for status in ("queued", "in_progress"):
        pages = json.loads(
            run_text(
                [
                    "gh",
                    "api",
                    "--method",
                    "GET",
                    "--paginate",
                    "--slurp",
                    f"repos/{repository}/actions/runs",
                    "-f",
                    "event=workflow_dispatch",
                    "-f",
                    f"status={status}",
                    "-f",
                    "per_page=100",
                ],
                Path.cwd(),
            )
        )
        if not isinstance(pages, list):
            raise WorkflowError("GitHub paginated runs response is malformed")
        workflow_runs: list[Any] = []
        for payload in pages:
            if not isinstance(payload, dict) or not isinstance(
                payload.get("workflow_runs"), list
            ):
                raise WorkflowError("GitHub runs response is malformed")
            workflow_runs.extend(payload["workflow_runs"])
        for run in workflow_runs:
            if not isinstance(run, dict):
                raise WorkflowError("GitHub run is malformed")
            run_id = run.get("id")
            path = run.get("path")
            repo_name = (run.get("repository") or {}).get("full_name")
            run_status = run.get("status")
            if not isinstance(run_id, int):
                raise WorkflowError("GitHub run ID is malformed")
            jobs_payload = json.loads(
                run_text(
                    [
                        "gh",
                        "api",
                        "--method",
                        "GET",
                        f"repos/{repository}/actions/runs/{run_id}/jobs",
                        "-f",
                        "per_page=100",
                    ],
                    Path.cwd(),
                )
            )
            raw_jobs = jobs_payload.get("jobs")
            if not isinstance(raw_jobs, list):
                raise WorkflowError("GitHub jobs response is malformed")
            jobs = [
                {
                    "name": job.get("name"),
                    "status": job.get("status"),
                    "labels": job.get("labels"),
                }
                for job in raw_jobs
            ]
            artifacts_payload = json.loads(
                run_text(
                    [
                        "gh",
                        "api",
                        "--method",
                        "GET",
                        f"repos/{repository}/actions/runs/{run_id}/artifacts",
                        "-f",
                        "per_page=100",
                    ],
                    Path.cwd(),
                )
            )
            artifact_count = artifacts_payload.get("total_count")
            if not isinstance(artifact_count, int):
                raise WorkflowError("GitHub artifacts response is malformed")
            runs.append(
                {
                    "id": run_id,
                    "repository": repo_name,
                    "workflow": path,
                    "status": run_status,
                    "jobs": jobs,
                    "artifacts": artifact_count,
                }
            )
    return {"schema_version": OCCUPANCY_SCHEMA, "runs": runs}


class OccupancySource:
    def __init__(self, repository: str, fixture: Path | None) -> None:
        self.repository = repository
        self.fixture = fixture
        self.index = 0
        self.payload = read_object(fixture) if fixture else None

    def classify(self) -> dict[str, Any]:
        if self.payload is None:
            snapshot = live_snapshot(self.repository)
        elif "snapshots" in self.payload:
            snapshots = self.payload["snapshots"]
            if not isinstance(snapshots, list) or not snapshots:
                raise WorkflowError("fixture snapshots must be a nonempty array")
            snapshot = snapshots[min(self.index, len(snapshots) - 1)]
            self.index += 1
            if not isinstance(snapshot, dict):
                raise WorkflowError("fixture snapshot must be an object")
        else:
            snapshot = self.payload
        return classify_occupancy(snapshot, self.repository)

    def classify_fail_closed(self) -> dict[str, Any]:
        try:
            return self.classify()
        except WorkflowError as error:
            return {"status": "UNKNOWN", "reason": str(error)}


def deferral_for(occupancy: dict[str, Any]) -> str | None:
    if occupancy.get("status") == "LIVE_TESTGATE":
        return "DEFERRED_TESTGATE_PRIORITY"
    if occupancy.get("status") == "UNKNOWN":
        return "DEFERRED_OCCUPANCY_UNKNOWN"
    if occupancy.get("status") != "CLEAR":
        return "DEFERRED_OCCUPANCY_UNKNOWN"
    return None


def control_receipt(
    control: Path,
    *,
    disposition: str,
    source_sha: str,
    source_tree: str,
    workflow_revision: str,
    workflow_sha256: str,
    occupancy: dict[str, Any],
    child_exit: int | None = None,
    publication: dict[str, Any] | None = None,
) -> None:
    receipt = {
        "schema_version": SCHEMA,
        "disposition": disposition,
        "source_sha": source_sha,
        "source_tree": source_tree,
        "workflow_revision": workflow_revision,
        "workflow_sha256": workflow_sha256,
        "occupancy": occupancy,
        "child_exit": child_exit,
        "publication": publication,
    }
    write_json(control / "quality-control-receipt.json", receipt)
    total = sum(path.stat().st_size for path in control.iterdir() if path.is_file())
    if total > MAX_CONTROL_BYTES:
        raise WorkflowError("control evidence exceeds 1 MiB")


def partial_index(attempt: Path, control: Path) -> None:
    files: list[dict[str, Any]] = []
    for root_name in ("local", "published"):
        root = attempt / root_name
        if not root.exists():
            continue
        for path in sorted(root.rglob("*")):
            if path.is_file() and not path.is_symlink():
                files.append(
                    {
                        "path": path.relative_to(attempt).as_posix(),
                        "size": path.stat().st_size,
                        "sha256": sha256_file(path),
                    }
                )
    compact = {"schema_version": SCHEMA, "files": files}
    encoded = canonical_bytes(compact) + b"\n"
    if len(encoded) > MAX_CONTROL_BYTES:
        compact = {
            "schema_version": SCHEMA,
            "files": [],
            "omitted": "partial index exceeded 1 MiB",
        }
    write_json(control / "quality-partial-index.json", compact)


def publication_manifest(published: Path) -> dict[str, Any]:
    if not published.is_dir() or published.is_symlink():
        raise WorkflowError("canonical publication directory is absent")
    entries = list(published.iterdir())
    observed: set[str] = set()
    files: dict[str, dict[str, Any]] = {}
    for path in entries:
        metadata = path.lstat()
        if (
            path.is_symlink()
            or not stat.S_ISREG(metadata.st_mode)
            or metadata.st_nlink != 1
        ):
            raise WorkflowError("canonical publication entry is not a private file")
        observed.add(path.name)
        files[path.name] = {
            "sha256": sha256_file(path),
            "size": metadata.st_size,
        }
    if observed != PUBLISHED_FILES:
        raise WorkflowError("canonical publication file set is not exact")
    total = sum(item["size"] for item in files.values())
    if total > MAX_PUBLISHED_BYTES:
        raise WorkflowError("canonical publication exceeds 100 MiB")
    return {"files": files, "total_bytes": total}


def verify_publication(published: Path) -> tuple[dict[str, Any], str]:
    manifest = publication_manifest(published)
    status = read_object(published / "run-status.json")
    if (
        status.get("execution_integrity") != "PASS"
        or status.get("closure_eligible") is not False
    ):
        raise WorkflowError("canonical run status is not observational PASS")
    evidence_id = status.get("quality_evidence_id")
    if not isinstance(evidence_id, str) or not SHA256.fullmatch(evidence_id):
        raise WorkflowError("canonical run status lacks an evidence ID")
    return manifest, evidence_id


def verify_upload(args: argparse.Namespace) -> int:
    receipt = read_object(args.control / "quality-control-receipt.json")
    if receipt.get("disposition") != "COMPLETE":
        raise WorkflowError("control receipt is not complete")
    if receipt.get("publication") != publication_manifest(args.published):
        raise WorkflowError("publication changed after supervision")
    print("quality-workflow-upload-verification: PASS")
    return 0


def remove_raw(attempt: Path, keep_publication: bool) -> None:
    local = attempt / "local"
    if local.exists():
        shutil.rmtree(local)
    if not keep_publication:
        published = attempt / "published"
        if published.exists():
            shutil.rmtree(published)


def preflight(args: argparse.Namespace) -> int:
    source = OccupancySource(args.repository, args.occupancy_fixture)
    occupancy = source.classify_fail_closed()
    disposition = deferral_for(occupancy) or "READY"
    write_json(
        args.output,
        {
            "schema_version": SCHEMA,
            "disposition": disposition,
            "occupancy": occupancy,
        },
    )
    print(f"quality-workflow-preflight: {disposition}")
    return 0


def supervise(args: argparse.Namespace) -> int:
    source_sha = exact_sha(args.source_sha, "source SHA")
    workflow_revision = exact_sha(args.workflow_revision, "workflow revision")
    workflow_sha256 = exact_sha256(args.workflow_sha256, "workflow SHA-256")
    repo = args.repo.resolve()
    actual = run_text(["git", "rev-parse", "HEAD"], repo)
    if actual != source_sha:
        raise WorkflowError("checkout does not equal dispatched source SHA")
    source_tree = run_text(["git", "rev-parse", "HEAD^{tree}"], repo)
    attempt = args.attempt_root.resolve()
    control = args.control_root.resolve()
    if attempt == control or attempt in control.parents or control in attempt.parents:
        raise WorkflowError("attempt and control roots must be separate")
    control.mkdir(parents=True, exist_ok=False)
    occupancy_source = OccupancySource(args.repository, args.occupancy_fixture)
    occupancy = occupancy_source.classify_fail_closed()
    disposition = deferral_for(occupancy)
    if disposition:
        control_receipt(
            control,
            disposition=disposition,
            source_sha=source_sha,
            source_tree=source_tree,
            workflow_revision=workflow_revision,
            workflow_sha256=workflow_sha256,
            occupancy=occupancy,
        )
        print(f"quality-workflow: {disposition}")
        return 0
    args.lease.parent.mkdir(parents=True, exist_ok=True)
    with args.lease.open("a+b") as lease:
        try:
            fcntl.flock(lease, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            control_receipt(
                control,
                disposition="DEFERRED_FOREST1_LEASE",
                source_sha=source_sha,
                source_tree=source_tree,
                workflow_revision=workflow_revision,
                workflow_sha256=workflow_sha256,
                occupancy=occupancy,
            )
            print("quality-workflow: DEFERRED_FOREST1_LEASE")
            return 0
        occupancy = occupancy_source.classify_fail_closed()
        disposition = deferral_for(occupancy)
        if disposition:
            control_receipt(
                control,
                disposition=disposition,
                source_sha=source_sha,
                source_tree=source_tree,
                workflow_revision=workflow_revision,
                workflow_sha256=workflow_sha256,
                occupancy=occupancy,
            )
            print(f"quality-workflow: {disposition}")
            return 0
        child_env = {
            **os.environ,
            "QUALITY_SOURCE_TREE": source_tree,
            "QUALITY_WORKFLOW_REVISION": workflow_revision,
            "QUALITY_WORKFLOW_SHA256": workflow_sha256,
            "QUALITY_PRIORITY_SENTINEL": str(control / "priority-stop.json"),
        }
        child = subprocess.Popen(
            args.child, cwd=repo, env=child_env, start_new_session=True
        )
        deferred = False
        while child.poll() is None:
            time.sleep(args.poll_seconds)
            occupancy = occupancy_source.classify_fail_closed()
            disposition = deferral_for(occupancy)
            if disposition:
                deferred = True
                write_json(
                    control / "priority-stop.json",
                    {
                        "schema_version": SCHEMA,
                        "disposition": disposition,
                        "occupancy": occupancy,
                    },
                )
                os.killpg(child.pid, signal.SIGTERM)
                try:
                    child.wait(timeout=args.grace_seconds)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
                break
        if deferred:
            partial_index(attempt, control)
            remove_raw(attempt, keep_publication=False)
            control_receipt(
                control,
                disposition=disposition or "DEFERRED_OCCUPANCY_UNKNOWN",
                source_sha=source_sha,
                source_tree=source_tree,
                workflow_revision=workflow_revision,
                workflow_sha256=workflow_sha256,
                occupancy=occupancy,
                child_exit=child.returncode,
            )
            print(
                "quality-workflow: "
                f"{disposition or 'DEFERRED_OCCUPANCY_UNKNOWN'}"
            )
            return 0
        if child.returncode:
            partial_index(attempt, control)
            remove_raw(attempt, keep_publication=False)
            control_receipt(
                control,
                disposition="EXECUTION_FAILED",
                source_sha=source_sha,
                source_tree=source_tree,
                workflow_revision=workflow_revision,
                workflow_sha256=workflow_sha256,
                occupancy=occupancy,
                child_exit=child.returncode,
            )
            return child.returncode
        publication, evidence_id = verify_publication(attempt / "published")
        remove_raw(attempt, keep_publication=True)
        control_receipt(
            control,
            disposition="COMPLETE",
            source_sha=source_sha,
            source_tree=source_tree,
            workflow_revision=workflow_revision,
            workflow_sha256=workflow_sha256,
            occupancy=occupancy,
            child_exit=child.returncode,
            publication=publication,
        )
        print(
            f"quality-workflow: PASS id={evidence_id} "
            f"files={len(PUBLISHED_FILES)} bytes={publication['total_bytes']}"
        )
        return 0


def self_test() -> int:
    repository = "openwepp/openwepp"
    clear = {
        "schema_version": OCCUPANCY_SCHEMA,
        "runs": [],
    }
    assert classify_occupancy(clear, repository)["status"] == "CLEAR"
    live = {
        "schema_version": OCCUPANCY_SCHEMA,
        "runs": [
            {
                "id": 1,
                "repository": repository,
                "workflow": CURRENT_WORKFLOW,
                "status": "queued",
                "jobs": [],
                "artifacts": 0,
            }
        ],
    }
    assert classify_occupancy(live, repository)["status"] == "LIVE_TESTGATE"
    omarchy = {
        "schema_version": OCCUPANCY_SCHEMA,
        "runs": [
            {
                "id": 29673299308,
                "repository": repository,
                "workflow": RETIRED_OMARCHY_WORKFLOW,
                "status": "queued",
                "jobs": [],
                "artifacts": 0,
            }
        ],
    }
    result = classify_occupancy(omarchy, repository)
    assert result["status"] == "CLEAR" and result["ignored_omarchy_runs"] == [
        29673299308
    ]
    with tempfile.TemporaryDirectory(prefix="quality-workflow-self-test-") as raw:
        published = Path(raw)
        for name in PUBLISHED_FILES:
            write_json(published / name, {})
        write_json(
            published / "run-status.json",
            {
                "execution_integrity": "PASS",
                "closure_eligible": False,
                "quality_evidence_id": "0" * 64,
            },
        )
        manifest, evidence_id = verify_publication(published)
        assert len(manifest["files"]) == 11 and evidence_id == "0" * 64
        victim = published / "coverage-summary.json"
        victim.unlink()
        os.link(published / "quality-payload.json", victim)
        try:
            verify_publication(published)
        except WorkflowError:
            pass
        else:
            raise WorkflowError("hard-linked publication entry was accepted")
        victim.unlink()
        write_json(victim, {})
        with victim.open("wb") as stream:
            stream.truncate(MAX_PUBLISHED_BYTES + 1)
        try:
            verify_publication(published)
        except WorkflowError:
            pass
        else:
            raise WorkflowError("oversize publication was accepted")
    print("quality-workflow-self-test: PASS")
    return 0


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    commands = root.add_subparsers(dest="command", required=True)
    check = commands.add_parser("self-test")
    check.set_defaults(function=lambda _args: self_test())
    pre = commands.add_parser("preflight")
    pre.add_argument("--repository", required=True)
    pre.add_argument("--occupancy-fixture", type=Path)
    pre.add_argument("--output", type=Path, required=True)
    pre.set_defaults(function=preflight)
    run = commands.add_parser("supervise")
    run.add_argument("--repo", type=Path, default=Path.cwd())
    run.add_argument("--repository", required=True)
    run.add_argument("--source-sha", required=True)
    run.add_argument("--workflow-revision", required=True)
    run.add_argument("--workflow-sha256", required=True)
    run.add_argument("--attempt-root", type=Path, required=True)
    run.add_argument("--control-root", type=Path, required=True)
    run.add_argument("--lease", type=Path, required=True)
    run.add_argument("--occupancy-fixture", type=Path)
    run.add_argument("--poll-seconds", type=float, default=30.0)
    run.add_argument("--grace-seconds", type=float, default=60.0)
    run.add_argument("child", nargs=argparse.REMAINDER)
    run.set_defaults(function=supervise)
    upload = commands.add_parser("verify-upload")
    upload.add_argument("--published", type=Path, required=True)
    upload.add_argument("--control", type=Path, required=True)
    upload.set_defaults(function=verify_upload)
    return root


def main() -> int:
    args = parser().parse_args()
    if args.command == "supervise":
        if args.poll_seconds <= 0 or args.poll_seconds > 30:
            print("ERROR: poll interval must be in (0, 30] seconds", file=sys.stderr)
            return 2
        if args.grace_seconds < 0 or args.grace_seconds > 60:
            print("ERROR: grace interval must be in [0, 60] seconds", file=sys.stderr)
            return 2
        if not args.child:
            print("ERROR: supervised child command is required", file=sys.stderr)
            return 2
        if args.child[0] == "--":
            args.child = args.child[1:]
        if not args.child:
            print("ERROR: supervised child command is required", file=sys.stderr)
            return 2
    try:
        return int(args.function(args))
    except (WorkflowError, OSError, ValueError, json.JSONDecodeError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
