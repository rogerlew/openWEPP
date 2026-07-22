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
    try:
        start = lines.index(marker) + 1
    except ValueError as error:
        raise AdmissionError(f"missing section: {marker}") from error
    end = next(
        (index for index in range(start, len(lines)) if lines[index].startswith("## ")),
        len(lines),
    )
    return lines[start:end]


def _write_set(text: str) -> list[str]:
    patterns = [
        match.group("path")
        for line in _section(text, "Declared Write Set")
        if (match := WRITE_SET_ITEM_RE.fullmatch(line))
    ]
    if not patterns:
        raise AdmissionError("declared write set is empty or not canonical")
    return patterns


def _field(text: str, label: str) -> str:
    for line in text.splitlines():
        match = FIELD_RE.fullmatch(line)
        if match and match.group("label") == label:
            value = match.group("value")
            if "{{" in value or "}}" in value:
                raise AdmissionError(f"unresolved placeholder in {label}")
            return value
    raise AdmissionError(f"missing field: {label}")


def _status(text: str) -> str:
    for line in text.splitlines():
        if line.startswith("Status:"):
            return line.removeprefix("Status:").strip(" `*")
    raise AdmissionError("aggregate package has no status")


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
    base_write_set = _write_set(base_aggregate)
    if base_write_set != _write_set(current_aggregate):
        raise AdmissionError("aggregate declared write set changed after scaffold")

    module_text = _git_text(repo, head, module_package)
    if _field(module_text, "Aggregate admission package") != aggregate_package:
        raise AdmissionError("module aggregate package binding does not match")
    if _field(module_text, "Aggregate scaffold commit") != aggregate_scaffold:
        raise AdmissionError("module aggregate scaffold binding does not match")

    additions = _git(
        repo,
        "log",
        "--reverse",
        "--diff-filter=A",
        "--format=%H",
        "--",
        module_package,
    ).splitlines()
    if not additions:
        raise AdmissionError("module package has no committed scaffold addition")
    module_scaffold = additions[0]
    ancestor = subprocess.run(
        ["git", "merge-base", "--is-ancestor", aggregate_scaffold, module_scaffold],
        cwd=repo,
        check=False,
    ).returncode == 0
    if not ancestor or aggregate_scaffold == module_scaffold:
        raise AdmissionError("aggregate scaffold must predate module scaffold")

    planned_paths = _write_set(module_text)
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
