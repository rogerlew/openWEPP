#!/usr/bin/env python3
"""Shared fail-closed custody checks for the CAL-04B freeze boundary."""

from __future__ import annotations

import csv
import hashlib
import json
import os
from datetime import datetime, timezone
from pathlib import Path


FREEZE_FIELDS = ["identity_id", "path_or_command", "role", "sha256", "state"]
BUNDLE_FIELDS = ["identity", "path", "sha256"]
RECEIPT_FIELDS = [
    "verifier_id",
    "freeze_digest",
    "verifier_script_sha256",
    "command",
    "command_sha256",
    "timestamp",
    "state",
]
ATTESTATION_SCHEMA = "openwepp-external-verifier-attestation-v1"


def sha256_file(path: Path) -> str:
    if not path.is_file():
        raise ValueError(f"custody object is not a regular file: {path}")
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def derived_id(value: dict[str, object], field: str) -> str:
    candidate = dict(value)
    candidate.pop(field, None)
    encoded = json.dumps(
        candidate, sort_keys=True, separators=(",", ":"), ensure_ascii=False
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def consume_capability(path: Path, consumed_path: Path) -> str:
    metadata = path.lstat()
    if path.is_symlink() or not path.is_file() or metadata.st_nlink != 1:
        raise ValueError("verifier capability is not a unique regular file")
    capability = path.read_bytes()
    if len(capability) < 32:
        raise ValueError("verifier capability is too short")
    consumed_path.parent.mkdir(parents=True, exist_ok=True)
    if consumed_path.exists():
        raise ValueError("verifier capability was already consumed")
    os.rename(path, consumed_path)
    return hashlib.sha256(capability).hexdigest()


def write_attestation(
    path: Path,
    *,
    capability_hash: str,
    parent_dispatch_id: str,
    agent_task_id: str,
    principal: str,
    workflow: str,
    job: str,
    runner: str,
    attempt: int,
    script: Path,
    argv: list[str],
    receipt: Path,
    freeze_digest: str,
) -> None:
    value: dict[str, object] = {
        "schema": ATTESTATION_SCHEMA,
        "attestation_id": "",
        "capability_hash": capability_hash,
        "parent_dispatch_id": parent_dispatch_id,
        "agent_task_id": agent_task_id,
        "principal": principal,
        "workflow": workflow,
        "job": job,
        "runner": runner,
        "attempt": attempt,
        "script_sha256": sha256_file(script),
        "argv": argv,
        "receipt_sha256": sha256_file(receipt),
        "freeze_digest": freeze_digest,
        "created_at": datetime.now(timezone.utc).isoformat(),
    }
    value["attestation_id"] = derived_id(value, "attestation_id")
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, sort_keys=True, separators=(",", ":"))
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())


def read_csv_exact(path: Path, fields: list[str]) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as stream:
        reader = csv.DictReader(stream)
        if reader.fieldnames != fields:
            raise ValueError(
                f"unexpected custody schema in {path}: {reader.fieldnames!r}"
            )
        rows = list(reader)
    if not rows:
        raise ValueError(f"empty custody manifest: {path}")
    return rows


def validate_bundle(
    path: Path,
    bundle_directory: Path | None = None,
    visited: set[Path] | None = None,
) -> int:
    """Recursively validate every member named by an immutable bundle."""

    bundle_root = (
        bundle_directory.resolve()
        if bundle_directory is not None
        else path.resolve().parent
    )
    seen = visited if visited is not None else set()
    resolved_manifest = path.resolve()
    if resolved_manifest in seen:
        raise ValueError(f"recursive custody bundle cycle: {path}")
    seen.add(resolved_manifest)
    rows = read_csv_exact(path, BUNDLE_FIELDS)
    identities = [row["identity"] for row in rows]
    if identities != sorted(identities) or len(identities) != len(set(identities)):
        raise ValueError(f"bundle identities are unsorted or duplicated: {path}")
    checked = 0
    for row in rows:
        leaf = Path(row["path"])
        expected = row["sha256"]
        if len(expected) != 64 or sha256_file(leaf) != expected:
            raise ValueError(f"bundle member identity mismatch: {leaf}")
        checked += 1
        resolved_leaf = leaf.resolve()
        if resolved_leaf.parent == bundle_root and resolved_leaf.suffix == ".csv":
            checked += validate_bundle(leaf, bundle_root, seen)
    seen.remove(resolved_manifest)
    return checked


def validate_freeze(
    manifest: Path,
    digest_path: Path,
    bundle_directory: Path,
    expected_rows: int = 16,
) -> tuple[str, int]:
    """Validate the root digest, all root objects, and every bundle member."""

    digest = digest_path.read_text(encoding="ascii").strip()
    if len(digest) != 64 or sha256_file(manifest) != digest:
        raise ValueError("freeze digest mismatch")
    rows = read_csv_exact(manifest, FREEZE_FIELDS)
    identities = [row["identity_id"] for row in rows]
    if (
        len(rows) != expected_rows
        or identities != sorted(identities)
        or len(identities) != len(set(identities))
    ):
        raise ValueError("freeze manifest is incomplete, unsorted, or duplicated")

    bundle_root = bundle_directory.resolve()
    leaf_count = 0
    for row in rows:
        frozen = Path(row["path_or_command"])
        if row["state"] != "FROZEN":
            raise ValueError(f"invalid frozen state for {row['identity_id']}")
        if len(row["sha256"]) != 64 or sha256_file(frozen) != row["sha256"]:
            raise ValueError(f"invalid frozen identity {row['identity_id']}")
        resolved = frozen.resolve()
        if resolved.parent == bundle_root and resolved.suffix == ".csv":
            leaf_count += validate_bundle(frozen, bundle_root)
    if leaf_count == 0:
        raise ValueError("freeze manifest did not contain any component bundles")
    return digest, leaf_count


def validate_receipt_barrier(
    receipt_paths: list[Path],
    freeze_digest: str,
    verifier_script: Path,
    expected_commands: dict[str, str],
) -> list[dict[str, str]]:
    """Validate two distinct immutable-digest receipts against code and argv."""

    if len(receipt_paths) != 2:
        raise ValueError("exactly two verifier receipts are required")
    script_digest = sha256_file(verifier_script)
    rows: list[dict[str, str]] = []
    for path in receipt_paths:
        receipt_rows = read_csv_exact(path, RECEIPT_FIELDS)
        if len(receipt_rows) != 1:
            raise ValueError(f"receipt must have exactly one row: {path}")
        rows.append(receipt_rows[0])
    if {row["verifier_id"] for row in rows} != set(expected_commands):
        raise ValueError("verifier identities are not the required distinct pair")
    for row in rows:
        command = expected_commands[row["verifier_id"]]
        if (
            row["freeze_digest"] != freeze_digest
            or row["verifier_script_sha256"] != script_digest
            or row["command"] != command
            or row["command_sha256"]
            != hashlib.sha256(command.encode("utf-8")).hexdigest()
            or row["state"] != "PASS"
        ):
            raise ValueError(
                f"verifier receipt is not bound to the frozen digest/code/argv: "
                f"{row['verifier_id']}"
            )
    return rows
