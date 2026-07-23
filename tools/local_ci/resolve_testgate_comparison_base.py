#!/usr/bin/env python3
"""Resolve one fail-closed TESTGATE comparison base."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

TRAILER_KEY = "TESTGATE-Comparison-Base"
COMMIT_RE = re.compile(r"[0-9a-f]{40}")


class ComparisonBaseError(RuntimeError):
    """Raised when a trusted event does not identify a safe comparison base."""


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
        raise ComparisonBaseError(result.stderr.strip() or "git command failed")
    return result.stdout


def _resolve_commit(repo: Path, revision: str) -> str:
    return _git(
        repo, ["rev-parse", "--verify", "--end-of-options", f"{revision}^{{commit}}"]
    ).strip()


def _head_overrides(repo: Path, head: str) -> list[str]:
    message = _git(repo, ["show", "-s", "--format=%B", head])
    parsed = _git(repo, ["interpret-trailers", "--parse"], input_text=message)
    values = []
    for line in parsed.splitlines():
        key, separator, value = line.partition(":")
        if separator and key == TRAILER_KEY:
            values.append(value.strip())
    return values


def _is_ancestor(repo: Path, ancestor: str, descendant: str) -> bool:
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", ancestor, descendant],
        cwd=repo,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode not in {0, 1}:
        raise ComparisonBaseError(result.stderr.strip() or "git merge-base failed")
    return result.returncode == 0


def resolve(
    repo: Path,
    event_name: str,
    head: str,
    event_before: str,
    input_base: str,
) -> str:
    exact_head = _resolve_commit(repo, head)
    supplied = input_base.strip()
    overrides = _head_overrides(repo, exact_head)
    if event_name == "push":
        if supplied:
            raise ComparisonBaseError(
                "push comparison base must come from the event or exact head"
            )
        if len(overrides) > 1:
            raise ComparisonBaseError(
                f"push head may contain at most one {TRAILER_KEY} trailer"
            )
        default_revision = (
            f"{exact_head}^"
            if not event_before or set(event_before) == {"0"}
            else event_before
        )
        default_base = _resolve_commit(repo, default_revision)
        if overrides:
            override = overrides[0]
            if COMMIT_RE.fullmatch(override) is None:
                raise ComparisonBaseError(
                    f"{TRAILER_KEY} must be one lowercase 40-character commit ID"
                )
            base = _resolve_commit(repo, override)
            if base != override or not _is_ancestor(repo, base, default_base):
                raise ComparisonBaseError(
                    f"{TRAILER_KEY} may only expand to an ancestor of event before"
                )
        else:
            base = default_base
    elif event_name == "workflow_dispatch":
        if overrides:
            raise ComparisonBaseError(
                f"workflow_dispatch rejects {TRAILER_KEY} trailers"
            )
        revision = supplied or f"{exact_head}^"
        base = _resolve_commit(repo, revision)
    else:
        raise ComparisonBaseError(f"unsupported trusted event: {event_name}")
    if base == exact_head or not _is_ancestor(repo, base, exact_head):
        raise ComparisonBaseError("comparison base must be a strict head ancestor")
    return base


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--event-name", required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--event-before", default="")
    parser.add_argument("--input-base", default="")
    return parser.parse_args()


def main() -> int:
    args = _parse_args()
    try:
        base = resolve(
            args.repo.resolve(),
            args.event_name,
            args.head,
            args.event_before,
            args.input_base,
        )
    except ComparisonBaseError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2
    print(base)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
