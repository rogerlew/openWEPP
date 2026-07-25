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
DEFUNCT_OMARCHY_RUNS = {
    29673299308: "850f7f6f10044c078299718d8e9c46b77d278a86",
    29672334757: "d4420b2431558dab0619c08a7bdcd7ac497ae229",
    29672149962: "4ee31784044694f856a2eef855b9864beac9f3cf",
}
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
FINALIZATION_SECONDS = 54.0
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
        required = {
            "id",
            "repository",
            "workflow",
            "event",
            "head_sha",
            "status",
            "conclusion",
            "jobs",
            "artifacts",
        }
        if set(item) != required:
            raise WorkflowError("occupancy run fields are not exact")
        run_id = item["id"]
        workflow = item["workflow"]
        status = item["status"]
        if (
            not isinstance(run_id, int)
            or not isinstance(item["repository"], str)
            or not isinstance(workflow, str)
            or item["repository"] != repository
            or item["event"] != "workflow_dispatch"
            or not isinstance(item["head_sha"], str)
            or not SHA40.fullmatch(item["head_sha"])
            or status
            not in {"requested", "waiting", "pending", "queued", "in_progress", "completed"}
            or (
                item["conclusion"] is not None
                and not isinstance(item["conclusion"], str)
            )
            or not isinstance(item["jobs"], list)
            or not isinstance(item["artifacts"], int)
            or item["artifacts"] < 0
        ):
            raise WorkflowError("occupancy run field type is invalid")
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
        if run_id in DEFUNCT_OMARCHY_RUNS:
            retired_omarchy = (
                workflow == CURRENT_WORKFLOW
                and item["head_sha"] == DEFUNCT_OMARCHY_RUNS[run_id]
                and status == "completed"
                and item["conclusion"] == "cancelled"
                and item["event"] == "workflow_dispatch"
                and item["repository"] == repository
                and not jobs
                and item["artifacts"] == 0
            )
            if not retired_omarchy:
                raise WorkflowError("defunct Omarchy record metadata drifted")
            ignored_omarchy.append(run_id)
            continue
        if status == "completed":
            if item["conclusion"] is None:
                raise WorkflowError("completed run lacks a conclusion")
            continue
        if item["conclusion"] is not None:
            raise WorkflowError("nonterminal run has a conclusion")
        if workflow != CURRENT_WORKFLOW:
            raise WorkflowError("nonterminal run workflow path is unexpected")
        if forest1_live or status in {
            "requested",
            "waiting",
            "pending",
            "queued",
            "in_progress",
        }:
            live.append({"id": run_id, "status": status})
    return {
        "status": "LIVE_TESTGATE" if live else "CLEAR",
        "live_runs": live,
        "ignored_omarchy_runs": ignored_omarchy,
    }


def live_snapshot(repository: str) -> dict[str, Any]:
    runs: list[dict[str, Any]] = []
    seen: set[int] = set()
    for status in ("requested", "waiting", "pending", "queued", "in_progress"):
        pages = json.loads(
            run_text(
                [
                    "gh",
                    "api",
                    "--method",
                    "GET",
                    "--paginate",
                    "--slurp",
                    f"repos/{repository}/actions/workflows/testgate-shadow.yml/runs",
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
        workflow_runs = flatten_run_pages(pages)
        for run in workflow_runs:
            if not isinstance(run, dict):
                raise WorkflowError("GitHub run is malformed")
            run_id = run.get("id")
            path = run.get("path")
            repo_name = (run.get("repository") or {}).get("full_name")
            run_status = run.get("status")
            if not isinstance(run_id, int):
                raise WorkflowError("GitHub run ID is malformed")
            if run_id in seen:
                raise WorkflowError("GitHub run ID appeared in multiple pages/states")
            seen.add(run_id)
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
            job_count = jobs_payload.get("total_count")
            if (
                not isinstance(raw_jobs, list)
                or not isinstance(job_count, int)
                or job_count != len(raw_jobs)
            ):
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
                    "event": run.get("event"),
                    "head_sha": run.get("head_sha"),
                    "status": run_status,
                    "conclusion": run.get("conclusion"),
                    "jobs": jobs,
                    "artifacts": artifact_count,
                }
            )
    return {"schema_version": OCCUPANCY_SCHEMA, "runs": runs}


def flatten_run_pages(pages: Any) -> list[Any]:
    if not isinstance(pages, list):
        raise WorkflowError("GitHub paginated runs response is malformed")
    workflow_runs: list[Any] = []
    expected_total: int | None = None
    for payload in pages:
        if not isinstance(payload, dict) or not isinstance(
            payload.get("workflow_runs"), list
        ):
            raise WorkflowError("GitHub runs response is malformed")
        page_total = payload.get("total_count")
        if not isinstance(page_total, int) or page_total < 0:
            raise WorkflowError("GitHub run total_count is malformed")
        if expected_total is None:
            expected_total = page_total
        elif expected_total != page_total:
            raise WorkflowError("GitHub paginated run totals disagree")
        workflow_runs.extend(payload["workflow_runs"])
    if expected_total is None or len(workflow_runs) != expected_total:
        raise WorkflowError("GitHub run pagination is incomplete")
    return workflow_runs


class OccupancySource:
    def __init__(
        self, repository: str, fixture: Path | None, watch_root: Path | None = None
    ) -> None:
        self.repository = repository
        self.fixture = fixture
        self.watch_root = watch_root
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
            if isinstance(snapshot, dict) and set(snapshot) == {
                "after_path",
                "snapshot",
            }:
                if self.watch_root is None:
                    raise WorkflowError("watched fixture requires a watch root")
                after_path = snapshot["after_path"]
                if (
                    not isinstance(after_path, str)
                    or after_path.startswith("/")
                    or ".." in Path(after_path).parts
                ):
                    raise WorkflowError("watched fixture path is unsafe")
                if not (self.watch_root / after_path).exists():
                    return {
                        "status": "CLEAR",
                        "live_runs": [],
                        "ignored_omarchy_runs": [],
                    }
                snapshot = snapshot["snapshot"]
            self.index += 1
            if not isinstance(snapshot, dict):
                raise WorkflowError("fixture snapshot must be an object")
        else:
            snapshot = self.payload
        return classify_occupancy(snapshot, self.repository)

    def classify_fail_closed(self) -> dict[str, Any]:
        try:
            return self.classify()
        except (
            WorkflowError,
            OSError,
            ValueError,
            TypeError,
            KeyError,
            AttributeError,
        ) as error:
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
    admission: dict[str, Any] | None = None,
    quality_evidence_id: str | None = None,
) -> None:
    existing = list(control.iterdir())
    if any(
        path.name != "quality-partial-index.json"
        or path.is_symlink()
        or not path.is_file()
        for path in existing
    ):
        raise WorkflowError("control evidence contains an unexpected entry")
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
        "admission": admission,
        "quality_evidence_id": quality_evidence_id,
    }
    encoded = canonical_bytes(receipt) + b"\n"
    existing_bytes = sum(path.stat().st_size for path in existing)
    if existing_bytes + len(encoded) > MAX_CONTROL_BYTES:
        raise WorkflowError("control evidence exceeds 1 MiB")
    write_json(control / "quality-control-receipt.json", receipt)
    allowed = {"quality-control-receipt.json"}
    if (control / "quality-partial-index.json").exists():
        allowed.add("quality-partial-index.json")
    entries = list(control.iterdir())
    if {
        path.name
        for path in entries
        if path.is_file() and not path.is_symlink()
    } != allowed or any(
        path.is_symlink() or not path.is_file() for path in entries
    ):
        raise WorkflowError("control evidence file set is not exact")
    total = sum(path.stat().st_size for path in entries)
    if total > MAX_CONTROL_BYTES:
        raise WorkflowError("control evidence exceeds 1 MiB")


def validate_control(control: Path) -> dict[str, Any]:
    if not control.is_dir() or control.is_symlink():
        raise WorkflowError("control evidence directory is missing or unsafe")
    entries = list(control.iterdir())
    names = {path.name for path in entries}
    if (
        "quality-control-receipt.json" not in names
        or not names.issubset(
            {"quality-control-receipt.json", "quality-partial-index.json"}
        )
        or any(
            path.is_symlink()
            or not path.is_file()
            or path.stat().st_nlink != 1
            for path in entries
        )
        or sum(path.stat().st_size for path in entries) > MAX_CONTROL_BYTES
    ):
        raise WorkflowError("control evidence file set or size is invalid")
    receipt = read_object(control / "quality-control-receipt.json")
    receipt_path = control / "quality-control-receipt.json"
    if receipt_path.read_bytes() != canonical_bytes(receipt) + b"\n":
        raise WorkflowError("control receipt is not canonical JSON")
    expected_fields = {
        "schema_version",
        "disposition",
        "source_sha",
        "source_tree",
        "workflow_revision",
        "workflow_sha256",
        "occupancy",
        "child_exit",
        "publication",
        "admission",
        "quality_evidence_id",
    }
    if set(receipt) != expected_fields or receipt.get("schema_version") != SCHEMA:
        raise WorkflowError("control receipt schema or fields are invalid")
    disposition = receipt.get("disposition")
    allowed_dispositions = {
        "COMPLETE",
        "DEFERRED_TESTGATE_PRIORITY",
        "DEFERRED_OCCUPANCY_UNKNOWN",
        "DEFERRED_FOREST1_LEASE",
        "EXECUTION_FAILED",
    }
    if (
        disposition not in allowed_dispositions
        or not isinstance(receipt.get("source_sha"), str)
        or not SHA40.fullmatch(receipt["source_sha"])
        or not isinstance(receipt.get("source_tree"), str)
        or not SHA40.fullmatch(receipt["source_tree"])
        or not isinstance(receipt.get("workflow_revision"), str)
        or not SHA40.fullmatch(receipt["workflow_revision"])
        or not isinstance(receipt.get("workflow_sha256"), str)
        or not SHA256.fullmatch(receipt["workflow_sha256"])
        or not isinstance(receipt.get("occupancy"), dict)
        or (
            receipt.get("child_exit") is not None
            and not isinstance(receipt.get("child_exit"), int)
        )
    ):
        raise WorkflowError("control receipt identity or disposition is invalid")
    if disposition.startswith("DEFERRED_") and (
        receipt.get("quality_evidence_id") is not None
        or receipt.get("admission") is not None
        or receipt.get("publication") is not None
    ):
        raise WorkflowError("deferred control receipt carries complete evidence")
    if disposition == "COMPLETE" and (
        receipt.get("child_exit") != 0
        or not isinstance(receipt.get("publication"), dict)
        or not isinstance(receipt.get("admission"), dict)
        or not isinstance(receipt.get("quality_evidence_id"), str)
        or not SHA256.fullmatch(receipt["quality_evidence_id"])
        or "quality-partial-index.json" in names
    ):
        raise WorkflowError("complete control receipt is incomplete or partial")
    if disposition == "EXECUTION_FAILED" and any(
        receipt.get(field) is not None
        for field in ("publication", "admission", "quality_evidence_id")
    ):
        raise WorkflowError("failed control receipt carries complete evidence")
    partial_path = control / "quality-partial-index.json"
    if partial_path.exists():
        partial = read_object(partial_path)
        if (
            partial_path.read_bytes() != canonical_bytes(partial) + b"\n"
            or partial.get("schema_version") != SCHEMA
            or not isinstance(partial.get("files"), list)
            or disposition == "COMPLETE"
        ):
            raise WorkflowError("partial index is malformed or attached to COMPLETE")
    return receipt


def partial_index(attempt: Path, control: Path) -> None:
    files: list[dict[str, Any]] = []
    allowed = (
        "local/nextest-full.log",
        "local/nextest-science-manual.log",
        "local/cargo-crap-science.log",
        "local/cargo-crap-merged.log",
        "published/run-status.json",
    )
    for relative in allowed:
        path = attempt / relative
        if not path.is_file() or path.is_symlink():
            continue
        size = path.stat().st_size
        if size > 256 * 1024:
            files.append({"path": relative, "size": size, "sha256": None})
        else:
            files.append(
                {"path": relative, "size": size, "sha256": sha256_file(path)}
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
        or status.get("debt_status") not in {"PASS", "FAIL"}
    ):
        raise WorkflowError("canonical run status is not observational PASS")
    evidence_id = status.get("quality_evidence_id")
    if not isinstance(evidence_id, str) or not SHA256.fullmatch(evidence_id):
        raise WorkflowError("canonical run status lacks an evidence ID")
    return manifest, evidence_id


def verify_upload(args: argparse.Namespace) -> int:
    receipt = validate_control(args.control)
    if receipt.get("disposition") != "COMPLETE":
        raise WorkflowError("control receipt is not complete")
    source_manifest_value = publication_manifest(args.published)
    if receipt.get("publication") != source_manifest_value:
        raise WorkflowError("publication changed after supervision")
    occupancy = OccupancySource(
        args.repository, args.occupancy_fixture
    ).classify_fail_closed()
    disposition = deferral_for(occupancy)
    if disposition:
        receipt["disposition"] = disposition
        receipt["occupancy"] = occupancy
        receipt["publication"] = None
        receipt["quality_evidence_id"] = None
        receipt["admission"] = None
        write_json(args.control / "quality-control-receipt.json", receipt)
        remove_raw(args.published.parent, keep_publication=False)
        if args.staging.exists():
            shutil.rmtree(args.staging)
        raise WorkflowError(f"upload deferred: {disposition}")
    args.staging.mkdir(parents=True, exist_ok=False)
    for name in sorted(PUBLISHED_FILES):
        shutil.copyfile(args.published / name, args.staging / name)
    if publication_manifest(args.staging) != source_manifest_value:
        raise WorkflowError("private upload staging differs from verified publication")
    print("quality-workflow-upload-verification: PASS")
    return 0


def verify_control(args: argparse.Namespace) -> int:
    validate_control(args.control)
    print("quality-workflow-control-verification: PASS")
    return 0


def remove_raw(
    attempt: Path, keep_publication: bool, deadline: float | None = None
) -> None:
    local = attempt / "local"
    if local.exists():
        bounded_rmtree(local, deadline)
    if not keep_publication:
        published = attempt / "published"
        if published.exists():
            bounded_rmtree(published, deadline)


def bounded_rmtree(path: Path, deadline: float | None) -> None:
    if deadline is None:
        shutil.rmtree(path)
        return
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise WorkflowError("priority cleanup deadline expired")
    cleanup = subprocess.Popen(
        ["rm", "-rf", "--", str(path)],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        start_new_session=True,
    )
    try:
        cleanup.wait(timeout=remaining)
    except subprocess.TimeoutExpired:
        terminate_group(cleanup, 0)
        raise WorkflowError("priority cleanup exceeded finalization deadline")
    if cleanup.returncode:
        raise WorkflowError("priority cleanup failed")


def group_alive(process_group: int) -> bool:
    try:
        os.killpg(process_group, 0)
    except ProcessLookupError:
        return False
    except PermissionError as error:
        raise WorkflowError("cannot inspect supervised process group") from error
    return True


def terminate_group(child: subprocess.Popen[Any], grace_seconds: float) -> None:
    deadline = time.monotonic() + grace_seconds
    if group_alive(child.pid):
        os.killpg(child.pid, signal.SIGTERM)
    while group_alive(child.pid) and time.monotonic() < deadline:
        child.poll()
        time.sleep(0.05)
    if group_alive(child.pid):
        os.killpg(child.pid, signal.SIGKILL)
    try:
        child.wait(timeout=1)
    except subprocess.TimeoutExpired as error:
        raise WorkflowError("supervised process group did not quiesce") from error
    if group_alive(child.pid):
        raise WorkflowError("supervised descendants survived SIGKILL")


def monitor_child(
    child: subprocess.Popen[Any],
    occupancy_source: OccupancySource,
    control: Path,
    poll_seconds: float,
    grace_seconds: float,
    deadline_state: dict[str, float | None],
) -> tuple[int, dict[str, Any], str | None, float | None]:
    occupancy: dict[str, Any] = {"status": "CLEAR"}
    while child.poll() is None:
        time.sleep(poll_seconds)
        occupancy = occupancy_source.classify_fail_closed()
        disposition = deferral_for(occupancy)
        if disposition:
            deadline = time.monotonic() + FINALIZATION_SECONDS
            deadline_state["deadline"] = deadline
            write_json(
                control / "priority-stop.json",
                {
                    "schema_version": SCHEMA,
                    "disposition": disposition,
                    "occupancy": occupancy,
                },
            )
            terminate_group(
                child,
                min(grace_seconds, max(0.0, deadline - time.monotonic())),
            )
            return (
                child.returncode or 0,
                occupancy,
                disposition,
                deadline,
            )
    if group_alive(child.pid):
        terminate_group(child, grace_seconds)
        return child.returncode or 2, occupancy, "EXECUTION_FAILED", None
    occupancy = occupancy_source.classify_fail_closed()
    disposition = deferral_for(occupancy)
    deadline = None
    if disposition:
        deadline = time.monotonic() + FINALIZATION_SECONDS
        deadline_state["deadline"] = deadline
    return child.returncode or 0, occupancy, disposition, (
        deadline
    )


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
    occupancy_source = OccupancySource(
        args.repository, args.occupancy_fixture, attempt
    )
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
        active = child
        finalization_deadline: float | None = None
        deadline_state: dict[str, float | None] = {"deadline": None}
        try:
            child_exit, occupancy, disposition, finalization_deadline = monitor_child(
                child,
                occupancy_source,
                control,
                args.poll_seconds,
                args.grace_seconds,
                deadline_state,
            )
            if disposition:
                partial_index(attempt, control)
                remove_raw(
                    attempt,
                    keep_publication=False,
                    deadline=finalization_deadline,
                )
                (control / "priority-stop.json").unlink(missing_ok=True)
                control_receipt(
                    control,
                    disposition=disposition,
                    source_sha=source_sha,
                    source_tree=source_tree,
                    workflow_revision=workflow_revision,
                    workflow_sha256=workflow_sha256,
                    occupancy=occupancy,
                    child_exit=child_exit,
                )
                print(f"quality-workflow: {disposition}")
                return 0 if disposition.startswith("DEFERRED_") else 2
            if child_exit:
                raise WorkflowError(f"quality child failed with exit {child_exit}")
            verifier_python = repo / ".venv/bin/python"
            if not verifier_python.is_file():
                verifier_python = Path(sys.executable)
            verifier_env = {
                **os.environ,
                "TMPDIR": str(attempt.parent / f"{attempt.name}-verify-tmp"),
            }
            Path(verifier_env["TMPDIR"]).mkdir(parents=True, exist_ok=False)
            verifier = subprocess.Popen(
                [
                    str(verifier_python),
                    "tools/local_ci/quality_observatory.py",
                    "verify",
                    "--repo",
                    ".",
                    "--published-dir",
                    str(attempt / "published"),
                    "--admission",
                    str(attempt / "local/pre-heavy-admission.json"),
                ],
                cwd=repo,
                env=verifier_env,
                start_new_session=True,
            )
            active = verifier
            verify_exit, occupancy, disposition, finalization_deadline = monitor_child(
                verifier,
                occupancy_source,
                control,
                args.poll_seconds,
                args.grace_seconds,
                deadline_state,
            )
            verifier_tmp = Path(verifier_env["TMPDIR"])
            if verifier_tmp.exists():
                bounded_rmtree(verifier_tmp, finalization_deadline)
            if disposition:
                partial_index(attempt, control)
                remove_raw(
                    attempt,
                    keep_publication=False,
                    deadline=finalization_deadline,
                )
                (control / "priority-stop.json").unlink(missing_ok=True)
                control_receipt(
                    control,
                    disposition=disposition,
                    source_sha=source_sha,
                    source_tree=source_tree,
                    workflow_revision=workflow_revision,
                    workflow_sha256=workflow_sha256,
                    occupancy=occupancy,
                    child_exit=verify_exit,
                )
                print(f"quality-workflow: {disposition}")
                return 0 if disposition.startswith("DEFERRED_") else 2
            if verify_exit:
                raise WorkflowError(
                    f"independent quality verification failed with exit {verify_exit}"
                )
            publication, evidence_id = verify_publication(attempt / "published")
            admission = read_object(attempt / "local/pre-heavy-admission.json")
            remove_raw(attempt, keep_publication=True)
            control_receipt(
                control,
                disposition="COMPLETE",
                source_sha=source_sha,
                source_tree=source_tree,
                workflow_revision=workflow_revision,
                workflow_sha256=workflow_sha256,
                occupancy=occupancy,
                child_exit=child_exit,
                publication=publication,
                admission=admission,
                quality_evidence_id=evidence_id,
            )
            print(
                f"quality-workflow: PASS id={evidence_id} "
                f"files={len(PUBLISHED_FILES)} bytes={publication['total_bytes']}"
            )
            return 0
        except BaseException as error:
            if finalization_deadline is None:
                finalization_deadline = deadline_state["deadline"]
            termination_error: WorkflowError | None = None
            if group_alive(active.pid):
                termination_budget = args.grace_seconds
                if finalization_deadline is not None:
                    termination_budget = max(
                        0.0, finalization_deadline - time.monotonic()
                    )
                try:
                    terminate_group(active, termination_budget)
                except WorkflowError as failure:
                    termination_error = failure
                    try:
                        os.killpg(active.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
            try:
                partial_index(attempt, control)
            finally:
                if finalization_deadline is None:
                    remove_raw(attempt, keep_publication=False)
                (control / "priority-stop.json").unlink(missing_ok=True)
            if not (control / "quality-control-receipt.json").exists():
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
            if isinstance(error, (KeyboardInterrupt, SystemExit)):
                raise
            if termination_error is not None:
                raise WorkflowError(
                    f"{error}; final termination failed: {termination_error}"
                ) from error
            raise WorkflowError(str(error)) from error


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
                "event": "workflow_dispatch",
                "head_sha": "1" * 40,
                "status": "queued",
                "conclusion": None,
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
                "workflow": CURRENT_WORKFLOW,
                "event": "workflow_dispatch",
                "head_sha": DEFUNCT_OMARCHY_RUNS[29673299308],
                "status": "completed",
                "conclusion": "cancelled",
                "jobs": [],
                "artifacts": 0,
            }
        ],
    }
    result = classify_occupancy(omarchy, repository)
    assert result["status"] == "CLEAR" and result["ignored_omarchy_runs"] == [
        29673299308
    ]
    assert flatten_run_pages(
        [
            {"total_count": 2, "workflow_runs": [{"id": 1}]},
            {"total_count": 2, "workflow_runs": [{"id": 2}]},
        ]
    ) == [{"id": 1}, {"id": 2}]
    try:
        flatten_run_pages([{"total_count": 2, "workflow_runs": [{"id": 1}]}])
    except WorkflowError:
        pass
    else:
        raise WorkflowError("truncated run pagination was accepted")
    with tempfile.TemporaryDirectory(prefix="quality-workflow-self-test-") as raw:
        test_root = Path(raw)
        published = test_root / "published"
        published.mkdir()
        for name in PUBLISHED_FILES:
            write_json(published / name, {})
        write_json(
            published / "run-status.json",
            {
                "execution_integrity": "PASS",
                "closure_eligible": False,
                "debt_status": "FAIL",
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
        victim.unlink()
        write_json(victim, {})
        control = test_root / "control"
        control.mkdir()
        complete_manifest, evidence_id = verify_publication(published)
        control_receipt(
            control,
            disposition="COMPLETE",
            source_sha="1" * 40,
            source_tree="2" * 40,
            workflow_revision="1" * 40,
            workflow_sha256="3" * 64,
            occupancy={"status": "CLEAR"},
            child_exit=0,
            publication=complete_manifest,
            admission={"admission_id": "fixture"},
            quality_evidence_id=evidence_id,
        )
        fixture = test_root / "live.json"
        write_json(
            fixture,
            {
                "schema_version": OCCUPANCY_SCHEMA,
                "runs": [
                    {
                        "id": 77,
                        "repository": repository,
                        "workflow": CURRENT_WORKFLOW,
                        "event": "workflow_dispatch",
                        "head_sha": "1" * 40,
                        "status": "queued",
                        "conclusion": None,
                        "jobs": [],
                        "artifacts": 0,
                    }
                ],
            },
        )
        try:
            verify_upload(
                argparse.Namespace(
                    control=control,
                    published=published,
                    staging=test_root / "staging",
                    repository=repository,
                    occupancy_fixture=fixture,
                )
            )
        except WorkflowError:
            pass
        else:
            raise WorkflowError("late TESTGATE priority allowed upload")
        deferred_receipt = validate_control(control)
        if (
            deferred_receipt.get("quality_evidence_id") is not None
            or deferred_receipt.get("admission") is not None
            or published.exists()
        ):
            raise WorkflowError("late deferral retained complete evidence")
    with tempfile.TemporaryDirectory(prefix="quality-control-self-test-") as raw:
        control = Path(raw)
        control_receipt(
            control,
            disposition="DEFERRED_TESTGATE_PRIORITY",
            source_sha="1" * 40,
            source_tree="2" * 40,
            workflow_revision="1" * 40,
            workflow_sha256="3" * 64,
            occupancy={"status": "LIVE_TESTGATE"},
        )
        validate_control(control)
        (control / "unexpected").mkdir()
        try:
            validate_control(control)
        except WorkflowError:
            pass
        else:
            raise WorkflowError("unexpected control directory was accepted")
    with tempfile.TemporaryDirectory(prefix="quality-forged-control-") as raw:
        control = Path(raw)
        write_json(
            control / "quality-control-receipt.json",
            {"disposition": "COMPLETE"},
        )
        try:
            validate_control(control)
        except WorkflowError:
            pass
        else:
            raise WorkflowError("forged complete control receipt was accepted")
    with tempfile.TemporaryDirectory(prefix="quality-acquire-self-test-") as raw:
        root = Path(raw)
        fixture = root / "race.json"
        clear_snapshot = {"schema_version": OCCUPANCY_SCHEMA, "runs": []}
        live_snapshot_value = {
            "schema_version": OCCUPANCY_SCHEMA,
            "runs": [
                {
                    "id": 88,
                    "repository": repository,
                    "workflow": CURRENT_WORKFLOW,
                    "event": "workflow_dispatch",
                    "head_sha": "1" * 40,
                    "status": "queued",
                    "conclusion": None,
                    "jobs": [],
                    "artifacts": 0,
                }
            ],
        }
        write_json(
            fixture,
            {"snapshots": [clear_snapshot, live_snapshot_value]},
        )
        repo = Path.cwd()
        head = run_text(["git", "rev-parse", "HEAD"], repo)
        marker = root / "child-started"
        result = supervise(
            argparse.Namespace(
                source_sha=head,
                workflow_revision=head,
                workflow_sha256="0" * 64,
                repo=repo,
                attempt_root=root / "attempt",
                control_root=root / "control",
                repository=repository,
                occupancy_fixture=fixture,
                lease=root / "lease",
                poll_seconds=0.01,
                grace_seconds=0.1,
                child=["touch", str(marker)],
            )
        )
        if result != 0 or marker.exists():
            raise WorkflowError("post-lock priority guard started the child")
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
    run.add_argument("--grace-seconds", type=float, default=30.0)
    run.add_argument("child", nargs=argparse.REMAINDER)
    run.set_defaults(function=supervise)
    upload = commands.add_parser("verify-upload")
    upload.add_argument("--published", type=Path, required=True)
    upload.add_argument("--control", type=Path, required=True)
    upload.add_argument("--staging", type=Path, required=True)
    upload.add_argument("--repository", required=True)
    upload.add_argument("--occupancy-fixture", type=Path)
    upload.set_defaults(function=verify_upload)
    control = commands.add_parser("verify-control")
    control.add_argument("--control", type=Path, required=True)
    control.set_defaults(function=verify_control)
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
