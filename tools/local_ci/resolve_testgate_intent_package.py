#!/usr/bin/env python3
"""Resolve one explicit TESTGATE intent package for a trusted event."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

TRAILER_KEY = "TESTGATE-Intent-Package"


class IntentPackageError(RuntimeError):
    """Raised when an event does not declare exactly one valid package."""


def _git(
    repo: Path, arguments: list[str], *, input_text: str | None = None
) -> str:
    result = subprocess.run(
        ["git", *arguments],
        cwd=repo,
        input=input_text,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise IntentPackageError(result.stderr.strip() or "git command failed")
    return result.stdout


def _push_package(repo: Path, head: str) -> str:
    commit = _git(repo, ["rev-parse", "--verify", f"{head}^{{commit}}"]).strip()
    message = _git(repo, ["show", "-s", "--format=%B", commit])
    parsed = _git(repo, ["interpret-trailers", "--parse"], input_text=message)
    values = []
    for line in parsed.splitlines():
        key, separator, value = line.partition(":")
        if separator and key == TRAILER_KEY:
            values.append(value.strip())
    if len(values) != 1:
        raise IntentPackageError(
            f"push head must contain exactly one {TRAILER_KEY} trailer"
        )
    return values[0]


def _valid_package_path(package: str) -> bool:
    parts = package.split("/")
    return (
        len(parts) == 4
        and parts[:2] == ["docs", "work-packages"]
        and parts[2] not in {"", ".", ".."}
        and "\r" not in parts[2]
        and "\n" not in parts[2]
        and parts[3] == "package.md"
    )


def resolve(
    repo: Path, event_name: str, head: str, input_package: str
) -> str:
    supplied = input_package.strip()
    if event_name == "push":
        if supplied:
            raise IntentPackageError(
                "push intent package must come from the exact head commit"
            )
        package = _push_package(repo, head)
    elif event_name == "workflow_dispatch":
        if not supplied:
            raise IntentPackageError(
                "workflow_dispatch requires an explicit intent package input"
            )
        package = supplied
    else:
        raise IntentPackageError(f"unsupported trusted event: {event_name}")
    if not _valid_package_path(package):
        raise IntentPackageError(f"invalid intent package path: {package}")
    return package


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--event-name", required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--input-package", default="")
    return parser.parse_args()


def main() -> int:
    args = _parse_args()
    try:
        package = resolve(
            args.repo.resolve(),
            args.event_name,
            args.head,
            args.input_package,
        )
    except IntentPackageError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2
    print(package)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
