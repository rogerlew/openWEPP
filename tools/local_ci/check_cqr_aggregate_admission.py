#!/usr/bin/env python3
"""Validate pre-implementation aggregate authority for one CQR module package."""

from __future__ import annotations

import argparse
import fnmatch
import json
import re
import subprocess
import sys
from pathlib import Path


PACKAGE_RE = re.compile(r"^docs/work-packages/[^/]+/package\.md$")
FIELD_RE = re.compile(r"^(?P<label>[^:]+):\s*`(?P<value>[^`]+)`\s*$")
WRITE_SET_ITEM_RE = re.compile(r"^- `(?P<path>[^`]+)`$")
MANIFEST_SCHEMA = "openwepp-cqr-aggregate-batch-v1"


class AdmissionError(RuntimeError):
    """Raised when aggregate authority cannot be proven exactly."""


def _git(repo: Path, *arguments: str, allow_failure: bool = False) -> str:
    result = subprocess.run(
        ["git", *arguments],
        cwd=repo,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0 and not allow_failure:
        raise AdmissionError(result.stderr.strip() or "Git command failed")
    return result.stdout.strip()


def _git_text(repo: Path, revision: str, path: str) -> str:
    if not PACKAGE_RE.fullmatch(path):
        raise AdmissionError(f"invalid work-package path: {path}")
    return _git(repo, "show", f"{revision}:{path}")


def _section(text: str, heading: str) -> list[str]:
    lines = text.splitlines()
    marker = f"## {heading}"
    positions = [index for index, line in enumerate(lines) if line == marker]
    if len(positions) != 1:
        raise AdmissionError(f"expected exactly one section: {marker}")
    start = positions[0] + 1
    end = next(
        (index for index in range(start, len(lines)) if lines[index].startswith("## ")),
        len(lines),
    )
    return lines[start:end]


def _canonical_path(path: str, label: str) -> str:
    if (
        path.startswith("/")
        or "\\" in path
        or "{{" in path
        or "}}" in path
        or any(part in {"", ".", ".."} for part in path.split("/"))
    ):
        raise AdmissionError(f"{label} is not repository-relative: {path}")
    return path


def _write_set(text: str, heading: str) -> list[str]:
    body = [line for line in _section(text, heading) if line]
    if any(WRITE_SET_ITEM_RE.fullmatch(line) is None for line in body):
        raise AdmissionError(f"malformed bullet in {heading}")
    patterns = [
        _canonical_path(WRITE_SET_ITEM_RE.fullmatch(line).group("path"), heading)
        for line in body
    ]
    if not patterns:
        raise AdmissionError("declared write set is empty or not canonical")
    if len(patterns) != len(set(patterns)):
        raise AdmissionError(f"duplicate path in {heading}")
    return patterns


def _field(text: str, label: str) -> str:
    values = []
    for line in text.splitlines():
        match = FIELD_RE.fullmatch(line)
        if match and match.group("label") == label:
            values.append(match.group("value"))
    if len(values) != 1:
        raise AdmissionError(f"expected exactly one field: {label}")
    value = values[0]
    if "{{" in value or "}}" in value:
        raise AdmissionError(f"unresolved placeholder in {label}")
    return value


def _status(text: str) -> str:
    values = [
        line.removeprefix("Status:").strip(" `*")
        for line in text.splitlines()
        if line.startswith("Status:")
    ]
    if len(values) != 1:
        raise AdmissionError("aggregate package must have exactly one status")
    return values[0]


def _batch_manifest(
    repo: Path, revision: str, path: str
) -> tuple[dict[str, object], str]:
    path = _canonical_path(path, "aggregate batch manifest")
    raw = _git(repo, "show", f"{revision}:{path}")
    try:
        value = json.loads(raw)
    except json.JSONDecodeError as error:
        raise AdmissionError("aggregate batch manifest is not valid JSON") from error
    if not isinstance(value, dict) or value.get("schema_version") != MANIFEST_SCHEMA:
        raise AdmissionError("aggregate batch manifest schema is invalid")
    return value, raw


def _string_paths(value: object, label: str) -> list[str]:
    if not isinstance(value, list) or any(not isinstance(item, str) for item in value):
        raise AdmissionError(f"{label} must be a list of strings")
    paths = [_canonical_path(item, label) for item in value]
    if len(paths) != len(set(paths)):
        raise AdmissionError(f"{label} contains duplicates")
    return paths


def _covers(authority: str, planned: str) -> bool:
    if authority == planned:
        return True
    if authority.endswith("/**"):
        prefix = authority.removesuffix("**")
        return planned.startswith(prefix)
    if not any(character in planned for character in "*?["):
        return fnmatch.fnmatchcase(planned, authority)
    return False


def validate(
    repo: Path,
    aggregate_package: str,
    aggregate_scaffold: str,
    module_package: str,
) -> dict[str, object]:
    repo = repo.resolve()
    aggregate_scaffold = _git(repo, "rev-parse", f"{aggregate_scaffold}^{{commit}}")
    head = _git(repo, "rev-parse", "HEAD^{commit}")
    base_aggregate = _git_text(repo, aggregate_scaffold, aggregate_package)
    current_aggregate = _git_text(repo, head, aggregate_package)
    status = _status(base_aggregate)
    if status not in {"ACTIVE", "READY"}:
        raise AdmissionError("aggregate scaffold status must be ACTIVE or READY")
    base_write_set = _write_set(base_aggregate, "Declared Write Set")
    if base_write_set != _write_set(current_aggregate, "Declared Write Set"):
        raise AdmissionError("aggregate declared write set changed after scaffold")

    additions = _git(
        repo,
        "log",
        "--reverse",
        "--diff-filter=A",
        "--format=%H",
        "--",
        module_package,
    ).splitlines()
    if len(additions) != 1:
        raise AdmissionError("module package must have one unique scaffold addition")
    module_scaffold = additions[0]
    ancestor = subprocess.run(
        ["git", "merge-base", "--is-ancestor", aggregate_scaffold, module_scaffold],
        cwd=repo,
        check=False,
    ).returncode == 0
    if not ancestor or aggregate_scaffold == module_scaffold:
        raise AdmissionError("aggregate scaffold must predate module scaffold")

    scaffold_module = _git_text(repo, module_scaffold, module_package)
    current_module = _git_text(repo, head, module_package)
    binding_labels = [
        "Aggregate admission package",
        "Aggregate scaffold commit",
        "Aggregate batch manifest",
        "Master ExecPlan",
    ]
    scaffold_bindings = {label: _field(scaffold_module, label) for label in binding_labels}
    current_bindings = {label: _field(current_module, label) for label in binding_labels}
    if scaffold_bindings != current_bindings:
        raise AdmissionError("module aggregate binding changed after scaffold")
    if scaffold_bindings["Aggregate admission package"] != aggregate_package:
        raise AdmissionError("module aggregate package binding does not match")
    if scaffold_bindings["Aggregate scaffold commit"] != aggregate_scaffold:
        raise AdmissionError("module aggregate scaffold binding does not match")
    scaffold_paths = _write_set(scaffold_module, "Intended Write Set")
    if scaffold_paths != _write_set(current_module, "Intended Write Set"):
        raise AdmissionError("module intended write set changed after scaffold")

    manifest_path = scaffold_bindings["Aggregate batch manifest"]
    aggregate_root = aggregate_package.removesuffix("package.md")
    if not manifest_path.startswith(aggregate_root):
        raise AdmissionError("aggregate batch manifest must be package-local")
    manifest, scaffold_manifest_text = _batch_manifest(
        repo, aggregate_scaffold, manifest_path
    )
    _, current_manifest_text = _batch_manifest(repo, head, manifest_path)
    if scaffold_manifest_text != current_manifest_text:
        raise AdmissionError("aggregate batch manifest changed after scaffold")
    if manifest.get("aggregate_package") != aggregate_package:
        raise AdmissionError("aggregate batch manifest package binding does not match")
    master_execplan = _canonical_path(
        scaffold_bindings["Master ExecPlan"], "master ExecPlan"
    )
    if manifest.get("master_execplan") != master_execplan:
        raise AdmissionError("module and batch manifest master ExecPlan differ")
    module_packages = _string_paths(manifest.get("module_packages"), "module_packages")
    required_paths = _string_paths(manifest.get("required_paths"), "required_paths")
    if module_package not in module_packages:
        raise AdmissionError("module package is absent from aggregate batch manifest")
    mandatory = [
        manifest_path,
        master_execplan,
        "docs/work-packages/README.md",
        *module_packages,
    ]
    if any(path not in required_paths for path in mandatory):
        raise AdmissionError("aggregate batch manifest omits mandatory paths")
    uncovered_manifest = [
        path
        for path in required_paths
        if not any(_covers(authority, path) for authority in base_write_set)
    ]
    if uncovered_manifest:
        raise AdmissionError(
            f"aggregate write set does not cover batch manifest: {uncovered_manifest}"
        )
    planned_paths = scaffold_paths
    absent_from_manifest = [
        path
        for path in planned_paths
        if not any(_covers(required, path) for required in required_paths)
    ]
    if absent_from_manifest:
        raise AdmissionError(
            f"aggregate batch manifest does not cover module paths: {absent_from_manifest}"
        )
    uncovered = [
        planned
        for planned in planned_paths
        if not any(_covers(authority, planned) for authority in base_write_set)
    ]
    if uncovered:
        raise AdmissionError(f"aggregate write set does not cover: {uncovered}")
    return {
        "schema_version": "openwepp-cqr-aggregate-admission-v1",
        "status": "PASS",
        "aggregate_package": aggregate_package,
        "aggregate_scaffold_commit": aggregate_scaffold,
        "module_package": module_package,
        "module_scaffold_commit": module_scaffold,
        "aggregate_batch_manifest": manifest_path,
        "master_execplan": master_execplan,
        "planned_paths": planned_paths,
    }


def _arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.cwd())
    parser.add_argument("--aggregate-package", required=True)
    parser.add_argument("--aggregate-scaffold", required=True)
    parser.add_argument("--module-package", required=True)
    return parser.parse_args()


def main() -> int:
    args = _arguments()
    try:
        result = validate(
            args.repo,
            args.aggregate_package,
            args.aggregate_scaffold,
            args.module_package,
        )
    except (AdmissionError, OSError) as error:
        print(json.dumps({"status": "FAIL", "error": str(error)}, sort_keys=True))
        return 2
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
