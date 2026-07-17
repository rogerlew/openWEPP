#!/usr/bin/env python3
"""Validate one assurance amendment receipt and run its proportional gate."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
GATE_BY_IMPACT = {
    "metadata-fast": "assurance-amendment-metadata-v1",
    "editorial-fast": "assurance-amendment-editorial-v1",
    "governance-focused": "assurance-amendment-governance-v1",
}
FOCUSED_ARGV = [
    "cargo",
    "nextest",
    "run",
    "--workspace",
    "--profile",
    "assurance-amendment",
]
FORBIDDEN_TOKENS = {
    "agent",
    "crap",
    "comparator",
    "full",
    "publish",
    "release",
    "reproduce",
    "work-package",
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def receipt_id(data: bytes) -> str:
    return sha256(b"openwepp-assurance-amendment-receipt-v1\0" + data)


def load_receipt(path: Path) -> tuple[dict[str, object], bytes, str]:
    data = path.read_bytes()
    value = json.loads(data)
    canonical = (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()
    if data != canonical:
        raise ValueError("receipt is not canonical pretty JSON")
    observed_id = receipt_id(data)
    if path.name != f"{observed_id}.json":
        raise ValueError("receipt filename does not match its content identity")
    required = {
        "schema_version",
        "operation",
        "impact_class",
        "changed",
        "old_generation_id",
        "new_generation_id",
        "affected_reports",
        "affected_paths",
        "invalidated_authority",
        "gate_ids",
        "gate_argv",
    }
    if set(value) != required or value["schema_version"] != 1:
        raise ValueError("receipt field set or schema version is invalid")
    if not value["changed"]:
        raise ValueError("no-op receipts do not require a focused gate")
    impact = value["impact_class"]
    if impact not in GATE_BY_IMPACT:
        raise ValueError(f"receipt impact class is not focused: {impact}")
    gates = value["gate_ids"]
    if gates != [GATE_BY_IMPACT[impact]]:
        raise ValueError("receipt gate ID does not match its focused impact class")
    argv_text = " ".join(part for argv in value["gate_argv"] for part in argv).lower()
    if any(token in argv_text for token in FORBIDDEN_TOKENS):
        raise ValueError("receipt argv contains a forbidden escalation token")
    if value["gate_argv"] != [FOCUSED_ARGV]:
        raise ValueError("receipt argv does not match the pinned focused gate")
    paths = value["affected_paths"]
    if not paths or any(not path.startswith("assurance/v2/") for path in paths):
        raise ValueError("focused receipt affects a path outside assurance/v2")
    if any("/schemas/" in path for path in paths):
        raise ValueError("schema changes require implementation-package gates")
    return value, data, observed_id


def current_generation(source_root: Path) -> str:
    value = json.loads((source_root / "assurance/v2/identity.lock.json").read_text())
    generation = value.get("generation_id")
    if not isinstance(generation, str) or len(generation) != 64:
        raise ValueError("current identity lock has no valid generation ID")
    return generation


def require_unique_transition_receipt(
    archive: Path, selected_path: Path, generation: str
) -> None:
    matches: list[Path] = []
    for candidate in sorted(archive.glob("*.json")):
        if candidate.is_symlink() or not candidate.is_file():
            raise ValueError("transaction archive contains a non-regular receipt member")
        data = candidate.read_bytes()
        value = json.loads(data)
        canonical = (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()
        if data != canonical or candidate.name != f"{receipt_id(data)}.json":
            raise ValueError("transaction archive contains a noncanonical receipt")
        if value["new_generation_id"] == generation:
            matches.append(candidate.resolve())
    if matches != [selected_path]:
        raise ValueError(
            "selected receipt is not the unique archived transition to the current generation"
        )


def tree_digest(path: Path) -> str:
    digest = hashlib.sha256(b"openwepp-assurance-protected-tree-v1\0")
    if not path.exists():
        return digest.hexdigest()
    for member in sorted(item for item in path.rglob("*") if item.is_file()):
        relative = member.relative_to(path).as_posix().encode()
        data = member.read_bytes()
        digest.update(len(relative).to_bytes(8, "big"))
        digest.update(relative)
        digest.update(len(data).to_bytes(8, "big"))
        digest.update(data)
    return digest.hexdigest()


def run(
    argv: list[str], records: list[dict[str, object]], *, cwd: Path = ROOT
) -> None:
    started = time.monotonic()
    completed = subprocess.run(argv, cwd=cwd, text=True, capture_output=True, check=False)
    elapsed = time.monotonic() - started
    record = {
        "argv": argv,
        "elapsed_seconds": round(elapsed, 6),
        "returncode": completed.returncode,
        "stdout_sha256": sha256(completed.stdout.encode()),
        "stderr_sha256": sha256(completed.stderr.encode()),
    }
    records.append(record)
    if completed.returncode:
        sys.stderr.write(completed.stdout)
        sys.stderr.write(completed.stderr)
        raise RuntimeError(f"focused gate failed: {argv}")


def seed_stage(source_root: Path, stage: Path) -> None:
    if stage.exists():
        shutil.rmtree(stage)
    usersum = source_root / "usersum"
    if usersum.exists():
        shutil.copytree(usersum, stage / "usersum")
    else:
        stage.mkdir(parents=True)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--receipt", required=True, type=Path)
    parser.add_argument("--evidence", type=Path)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--allow-test-fixture", action="store_true")
    args = parser.parse_args()
    source_root = args.root.resolve()
    receipt_path = args.receipt.resolve()
    archive = (source_root / "assurance/v2/transactions").resolve()
    if receipt_path.parent != archive or receipt_path.is_symlink():
        raise ValueError("receipt must be a regular direct member of the active transaction archive")
    if source_root != ROOT and not args.allow_test_fixture:
        raise ValueError("repository-free roots require explicit --allow-test-fixture")
    if source_root == ROOT and args.allow_test_fixture:
        raise ValueError("--allow-test-fixture cannot weaken the production repository")
    receipt, receipt_bytes, observed_id = load_receipt(receipt_path)
    generation = current_generation(source_root)
    if receipt["new_generation_id"] != generation:
        raise ValueError("receipt does not terminate at the current generation")
    require_unique_transition_receipt(archive, receipt_path, generation)

    binary = ROOT / "target/release/openwepp-assurance"
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise ValueError(
            "build target/release/openwepp-assurance before running the focused gate"
        )
    stage = ROOT / "target/assurance-amendment-stage" / observed_id
    seed_stage(source_root, stage)
    protected_before = tree_digest(source_root / "usersum")
    records: list[dict[str, object]] = []
    started = time.monotonic()
    try:
        if not args.allow_test_fixture:
            identity = json.loads(
                (source_root / "assurance/v2/identity.lock.json").read_text()
            )
            base_ref = identity.get("genesis", {}).get("base_ref")
            if not isinstance(base_ref, str) or not base_ref:
                raise ValueError("active identity lock has no trusted genesis base ref")
            run(
                [str(binary), "verify-generation", "--base-ref", base_ref],
                records,
                cwd=source_root,
            )
        for report in receipt["affected_reports"]:
            run(
                [str(binary), "validate", "--report", report],
                records,
                cwd=source_root,
            )
            run(
                [
                    str(binary),
                    "build",
                    "--report",
                    report,
                    "--staging-root",
                    str(stage),
                ],
                records,
                cwd=source_root,
            )
            run(
                [
                    str(binary),
                    "check",
                    "--report",
                    report,
                    "--staging-root",
                    str(stage),
                ],
                records,
                cwd=source_root,
            )
        run(FOCUSED_ARGV, records)
    finally:
        shutil.rmtree(stage, ignore_errors=True)
    elapsed = time.monotonic() - started
    protected_after = tree_digest(source_root / "usersum")
    if protected_after != protected_before:
        raise RuntimeError("focused gate mutated the protected usersum tree")

    evidence_path = args.evidence or (
        ROOT / "target/local-ci-history/assurance-amendment" / f"{observed_id}.json"
    )
    evidence_path.parent.mkdir(parents=True, exist_ok=True)
    evidence = {
        "format": "openwepp-assurance-amendment-gate-evidence:1",
        "receipt_id": observed_id,
        "receipt_sha256": sha256(receipt_bytes),
        "generation_id": generation,
        "impact_class": receipt["impact_class"],
        "gate_ids": receipt["gate_ids"],
        "affected_reports": receipt["affected_reports"],
        "affected_paths": receipt["affected_paths"],
        "binary_sha256": sha256(binary.read_bytes()),
        "protected_usersum_sha256_before": protected_before,
        "protected_usersum_sha256_after": protected_after,
        "elapsed_seconds": round(elapsed, 6),
        "commands": records,
        "status": "PASS",
        "test_fixture": args.allow_test_fixture,
    }
    evidence_path.write_text(json.dumps(evidence, indent=2) + "\n")
    try:
        display_path = evidence_path.relative_to(ROOT)
    except ValueError:
        display_path = evidence_path
    print(display_path)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"assurance amendment gate: FAIL: {error}", file=sys.stderr)
        raise SystemExit(1) from error
