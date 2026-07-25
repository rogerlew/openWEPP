#!/usr/bin/env python3
"""Evaluate cargo-crap JSON against openWEPP's adjudicated CRAP ratchet."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


SCHEMA_VERSION = "openwepp-adjudicated-crap-v1"
REPORT_SCHEMA_VERSION = "openwepp-adjudicated-crap-report-v1"
SOURCE_MANIFEST_SCHEMA_VERSION = "openwepp-production-source-manifest-v2"
CARGO_CRAP_SCHEMA = (
    "https://raw.githubusercontent.com/minikin/cargo-crap/main/schemas/report-v1.json"
)
SUPPORTED_CARGO_CRAP_VERSION = "0.2.2"
CANONICAL_REGISTRY_SHA256 = (
    "10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f"
)
THRESHOLD = 30.0
ALLOWED_CLASSIFICATIONS = {
    "R-OBSERVABILITY",
    "R-IRREDUCIBLE-CRAP",
    "X-GENERATED",
    "X-NONDEFAULT-CFG",
    "X-DELEGATING-MAIN",
    "X-IMPOSSIBLE",
}
EXPECTED_FILTER = {
    "path_prefix": "crates/",
    "required_component": "/src/",
    "excluded_component": "/src/tests/",
    "threshold_relation": ">",
}
EXPECTED_DEDUPLICATION_KEY = [
    "file",
    "function",
    "line",
    "cyclomatic",
    "coverage",
    "crap",
]
EXPECTED_AUTHORITY = {
    "decision": "docs/decisions/0021-module-coverage-closure-thresholds.md",
    "campaign": "docs/work-packages/cqr-pre-integration-campaign-evidence/low/campaign-final-assessment.md",
    "ledger": "docs/work-packages/cqr-pre-integration-campaign-evidence/low/raw-to-actionable-ledger.md",
}
WILDCARD_RE = re.compile(r"[*?\[]")
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")


class GateInputError(ValueError):
    """Raised when gate inputs are malformed or unsafe."""


def _read_json(path: Path) -> dict[str, Any]:
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise GateInputError(f"cannot read JSON {path}: {error}") from error
    if not isinstance(payload, dict):
        raise GateInputError(f"JSON root must be an object: {path}")
    return payload


def _finite_number(value: Any, field: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise GateInputError(f"{field} must be a number")
    number = float(value)
    if not math.isfinite(number):
        raise GateInputError(f"{field} must be finite")
    return number


def _repo_relative_path(raw_path: str, repo_root: Path) -> str:
    if not isinstance(raw_path, str) or not raw_path:
        raise GateInputError("row file must be a non-empty string")
    candidate = Path(raw_path)
    if not candidate.is_absolute():
        candidate = repo_root / candidate
    try:
        relative = candidate.resolve(strict=False).relative_to(repo_root)
    except ValueError as error:
        raise GateInputError(f"path escapes repository: {raw_path}") from error
    return relative.as_posix()


def _parse_row(entry: Any, repo_root: Path) -> dict[str, Any]:
    if not isinstance(entry, dict):
        raise GateInputError("every CRAP entry must be an object")
    function = entry.get("function")
    line = entry.get("line")
    if not isinstance(function, str) or not function:
        raise GateInputError("row function must be a non-empty string")
    if isinstance(line, bool) or not isinstance(line, int) or line < 1:
        raise GateInputError(f"row line must be a positive integer for {function}")
    crate_name = entry.get("crate")
    if not isinstance(crate_name, str) or not crate_name:
        raise GateInputError(f"row crate must be a non-empty string for {function}")
    coverage = entry.get("coverage")
    if coverage is not None:
        coverage = _finite_number(coverage, f"coverage for {function}")
    return {
        "file": _repo_relative_path(entry.get("file"), repo_root),
        "function": function,
        "crate": crate_name,
        "line": line,
        "cyclomatic": _finite_number(
            entry.get("cyclomatic"), f"cyclomatic for {function}"
        ),
        "coverage": coverage,
        "crap": _finite_number(entry.get("crap"), f"crap for {function}"),
    }


def _is_production_row(row: dict[str, Any]) -> bool:
    path = row["file"]
    return (
        path.startswith("crates/")
        and "/src/" in path
        and "/src/tests/" not in path
    )


def _row_key(row: dict[str, Any]) -> tuple[Any, ...]:
    return tuple(row[field] for field in EXPECTED_DEDUPLICATION_KEY)


def _row_sort_key(row: dict[str, Any]) -> tuple[Any, ...]:
    return (row["file"], row["line"], row["function"], row["crap"])


def _production_rows(
    crap_payload: dict[str, Any],
    repo_root: Path,
    expected_production_crates: set[str] | None = None,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], set[str]]:
    if crap_payload.get("$schema") != CARGO_CRAP_SCHEMA:
        raise GateInputError("unsupported or missing cargo-crap report schema")
    if crap_payload.get("version") != SUPPORTED_CARGO_CRAP_VERSION:
        raise GateInputError(
            "cargo-crap report version must be " f"{SUPPORTED_CARGO_CRAP_VERSION}"
        )
    entries = crap_payload.get("entries")
    if not isinstance(entries, list) or not entries:
        raise GateInputError("CRAP JSON must contain a non-empty entries array")
    parsed = [_parse_row(entry, repo_root) for entry in entries]
    production_all = [row for row in parsed if _is_production_row(row)]
    if expected_production_crates is not None:
        production_all = [
            row for row in production_all if row["crate"] in expected_production_crates
        ]
    if not production_all:
        raise GateInputError("CRAP JSON contains no production rows after filtering")

    unique_over_threshold: dict[tuple[Any, ...], dict[str, Any]] = {}
    for row in production_all:
        if row["crap"] > THRESHOLD:
            unique_over_threshold[_row_key(row)] = row
    raw_over_threshold = sorted(unique_over_threshold.values(), key=_row_sort_key)
    return production_all, raw_over_threshold, {row["crate"] for row in production_all}


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _sha256_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def production_source_manifest(repo_root: Path) -> dict[str, Any]:
    """Return a deterministic snapshot of the complete production Rust surface."""

    repo_root = repo_root.resolve()
    source_rows: list[dict[str, str]] = []
    for source_path in sorted(repo_root.glob("crates/*/src/**/*.rs")):
        relative = _repo_relative_path(str(source_path), repo_root)
        if not _is_production_source_path(relative):
            continue
        if not source_path.is_file():
            raise GateInputError(f"production source is not a regular file: {relative}")
        source_rows.append({"path": relative, "sha256": _sha256(source_path)})
    if not source_rows:
        raise GateInputError("repository contains no production Rust sources")

    measurement_candidates: set[Path] = set()
    for source_root in (repo_root / "crates", repo_root / "src", repo_root / "tests"):
        if source_root.is_dir():
            measurement_candidates.update(source_root.rglob("*.rs"))
    measurement_candidates.add(repo_root / "Cargo.toml")
    measurement_candidates.update((repo_root / "crates").glob("*/Cargo.toml"))
    for relative_path in (
        ".cargo-crap.toml",
        ".cargo/config",
        ".cargo/config.toml",
        ".config/nextest.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "tools/release/adjudicated_crap_exceptions.json",
        "tools/release/check_adjudicated_crap.py",
        "tools/release/run_adjudicated_crap_gate.sh",
        "tools/local_ci/quality_observatory.py",
        "docs/work-packages/20260724-quality-observatory-merged-coverage-001/artifacts/snowbench-full-only-row-ledger.json",
    ):
        candidate = repo_root / relative_path
        if candidate.is_file():
            measurement_candidates.add(candidate)
    measurement_rows: list[dict[str, str]] = []
    for input_path in sorted(measurement_candidates):
        relative = _repo_relative_path(str(input_path), repo_root)
        if input_path.is_file():
            measurement_rows.append({"path": relative, "sha256": _sha256(input_path)})

    head = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=repo_root,
        check=False,
        capture_output=True,
        text=True,
    )
    if head.returncode != 0:
        raise GateInputError(f"cannot resolve source-manifest HEAD: {head.stderr.strip()}")
    index = subprocess.run(
        ["git", "ls-files", "--stage", "-z"],
        cwd=repo_root,
        check=False,
        capture_output=True,
    )
    if index.returncode != 0:
        raise GateInputError("cannot read Git index for production source manifest")
    return {
        "schema_version": SOURCE_MANIFEST_SCHEMA_VERSION,
        "head_commit": head.stdout.strip(),
        "git_index_sha256": _sha256_bytes(index.stdout),
        "measurement_input_count": len(measurement_rows),
        "measurement_inputs": measurement_rows,
        "source_count": len(source_rows),
        "sources": source_rows,
    }


def _workspace_packages(repo_root: Path) -> dict[str, dict[str, Any]]:
    result = subprocess.run(
        [
            "cargo",
            "metadata",
            "--locked",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
        ],
        cwd=repo_root,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise GateInputError(f"cannot read Cargo workspace metadata: {result.stderr.strip()}")
    try:
        metadata = json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise GateInputError(f"Cargo workspace metadata is invalid JSON: {error}") from error
    packages = metadata.get("packages")
    workspace_members = metadata.get("workspace_members")
    if not isinstance(packages, list) or not isinstance(workspace_members, list):
        raise GateInputError("Cargo workspace metadata has no package list")
    member_ids = {item for item in workspace_members if isinstance(item, str)}
    by_name: dict[str, dict[str, Any]] = {}
    for package in packages:
        if not isinstance(package, dict) or package.get("id") not in member_ids:
            continue
        name = package.get("name")
        manifest_path = package.get("manifest_path")
        if not isinstance(name, str) or not isinstance(manifest_path, str):
            continue
        if name in by_name:
            raise GateInputError(f"Cargo workspace has duplicate package name: {name}")
        package = dict(package)
        package["relative_manifest"] = _repo_relative_path(manifest_path, repo_root)
        by_name[name] = package
    if not by_name:
        raise GateInputError("Cargo workspace has no member packages")
    return by_name


def _production_packages_from_workspace(
    repo_root: Path, workspace: dict[str, dict[str, Any]]
) -> dict[str, str]:
    production: dict[str, str] = {}
    for name, package in workspace.items():
        manifest = str(package["relative_manifest"])
        manifest_parts = Path(manifest).parts
        if (
            len(manifest_parts) != 3
            or manifest_parts[0] != "crates"
            or manifest_parts[2] != "Cargo.toml"
        ):
            continue
        manifest_path = repo_root / manifest
        source_prefix = f"{Path(manifest).parent.as_posix()}/src/"
        source_root = repo_root / source_prefix
        if (
            manifest_path.is_symlink()
            or source_root.is_symlink()
            or not source_root.is_dir()
        ):
            continue
        targets = package.get("targets")
        if not isinstance(targets, list):
            continue
        for target in targets:
            if not isinstance(target, dict) or not isinstance(
                target.get("src_path"), str
            ):
                continue
            kinds = target.get("kind")
            if (
                not isinstance(kinds, list)
                or not kinds
                or not all(isinstance(kind, str) for kind in kinds)
                or not set(kinds).issubset({"lib", "bin", "proc-macro"})
            ):
                continue
            target_path = _repo_relative_path(target["src_path"], repo_root)
            target_file = repo_root / target_path
            if (
                target_path.startswith(source_prefix)
                and _is_production_source_path(target_path)
                and target_file.is_file()
                and not target_file.is_symlink()
            ):
                production[name] = source_prefix
                break
    return production


def _workspace_production_packages(repo_root: Path) -> dict[str, str]:
    return _production_packages_from_workspace(repo_root, _workspace_packages(repo_root))


def _workspace_production_crates(repo_root: Path) -> set[str]:
    names = set(_workspace_production_packages(repo_root))
    if not names:
        raise GateInputError("Cargo workspace has no production crates under crates/")
    return names


def resolve_measurement_packages(
    repo_root: Path, requested_packages: set[str]
) -> dict[str, list[str]]:
    """Validate that affected measurement packages own production sources."""

    if not requested_packages:
        raise GateInputError("affected measurement requires at least one package")
    workspace = _workspace_packages(repo_root)
    unknown = requested_packages - set(workspace)
    if unknown:
        raise GateInputError(
            f"affected measurement names unknown workspace packages: {sorted(unknown)}"
        )
    production = set(_production_packages_from_workspace(repo_root, workspace))
    measurement_only = requested_packages - production
    if measurement_only:
        raise GateInputError(
            "affected measurement package has no production source owner and "
            f"requires global quality: {sorted(measurement_only)}"
        )
    return {
        "measurement_packages": sorted(requested_packages),
        "production_packages": sorted(requested_packages),
    }


def _production_package_source_prefixes(
    repo_root: Path, package_names: set[str]
) -> tuple[str, ...]:
    production = _workspace_production_packages(repo_root)
    if not package_names.issubset(production):
        raise GateInputError("cannot bind every expected package to a source root")
    return tuple(sorted(production[name] for name in package_names))


def _validated_scope_artifact(
    scope_path: Path, repo_root: Path, requested_packages: set[str]
) -> tuple[dict[str, list[str]], str]:
    supplied_bytes = scope_path.read_bytes()
    try:
        supplied = json.loads(supplied_bytes)
    except json.JSONDecodeError as error:
        raise GateInputError(
            f"affected package scope is invalid JSON: {error}"
        ) from error
    if not isinstance(supplied, dict):
        raise GateInputError("affected package scope root must be an object")
    expected = resolve_measurement_packages(repo_root, requested_packages)
    if supplied != expected:
        raise GateInputError(
            "affected package scope changed after preflight or has invalid content"
        )
    canonical_bytes = (json.dumps(expected, sort_keys=True) + "\n").encode("utf-8")
    if supplied_bytes != canonical_bytes:
        raise GateInputError(
            "affected package scope bytes changed after canonical preflight"
        )
    return expected, _sha256_bytes(supplied_bytes)


def _safe_registry_path(raw_path: Any, repo_root: Path, field: str) -> tuple[str, Path]:
    if not isinstance(raw_path, str) or not raw_path:
        raise GateInputError(f"{field} must be a non-empty repository-relative path")
    if Path(raw_path).is_absolute() or WILDCARD_RE.search(raw_path):
        raise GateInputError(f"{field} must be exact and repository-relative: {raw_path}")
    normalized = _repo_relative_path(raw_path, repo_root)
    return normalized, repo_root / normalized


def _git_file_sha256_at_commit(
    repo_root: Path, commit: str, file_name: str
) -> tuple[str | None, str | None]:
    commit_result = subprocess.run(
        ["git", "cat-file", "-e", f"{commit}^{{commit}}"],
        cwd=repo_root,
        check=False,
        capture_output=True,
    )
    if commit_result.returncode != 0:
        return None, "adjudicated commit does not resolve"
    file_result = subprocess.run(
        ["git", "show", f"{commit}:{file_name}"],
        cwd=repo_root,
        check=False,
        capture_output=True,
    )
    if file_result.returncode != 0:
        return None, "source file is absent at adjudicated commit"
    return _sha256_bytes(file_result.stdout), None


def _validated_evidence_record(
    evidence: Any,
    repo_root: Path,
    field: str,
    required_tokens: list[str],
    invalid: list[dict[str, str]],
    adjudication_id: str,
) -> tuple[str, Path]:
    if not isinstance(evidence, dict):
        raise GateInputError(f"{field} must be an evidence object")
    normalized, evidence_path = _safe_registry_path(
        evidence.get("path"), repo_root, f"{field}.path"
    )
    expected_hash = evidence.get("sha256")
    if not isinstance(expected_hash, str) or not SHA256_RE.fullmatch(expected_hash):
        raise GateInputError(f"{field}.sha256 must be a lowercase SHA-256")
    acceptance_token = evidence.get("acceptance_token")
    if not isinstance(acceptance_token, str) or not acceptance_token:
        raise GateInputError(f"{field}.acceptance_token must be non-empty")
    if not evidence_path.is_file():
        invalid.append(
            {
                "id": adjudication_id,
                "reason": f"evidence path is missing: {normalized}",
            }
        )
        return normalized, evidence_path
    content = evidence_path.read_bytes()
    observed_hash = _sha256_bytes(content)
    if observed_hash != expected_hash:
        invalid.append(
            {
                "id": adjudication_id,
                "reason": (
                    f"evidence hash is stale for {normalized}: "
                    f"expected {expected_hash}, observed {observed_hash}"
                ),
            }
        )
    text_content = content.decode("utf-8", errors="replace")
    for token in [*required_tokens, acceptance_token]:
        if token not in text_content:
            invalid.append(
                {
                    "id": adjudication_id,
                    "reason": f"evidence {normalized} is missing binding token: {token}",
                }
            )
    return normalized, evidence_path


def _load_adjudications(
    registry: dict[str, Any], repo_root: Path
) -> tuple[list[dict[str, Any]], list[dict[str, str]]]:
    if registry.get("schema_version") != SCHEMA_VERSION:
        raise GateInputError(f"schema_version must be {SCHEMA_VERSION}")
    if _finite_number(registry.get("threshold"), "registry threshold") != THRESHOLD:
        raise GateInputError(f"registry threshold must be {THRESHOLD:g}")
    if registry.get("production_filter") != EXPECTED_FILTER:
        raise GateInputError("registry production_filter does not match CQR authority")
    if registry.get("deduplication_key") != EXPECTED_DEDUPLICATION_KEY:
        raise GateInputError("registry deduplication_key does not match CQR authority")
    if registry.get("authority") != EXPECTED_AUTHORITY:
        raise GateInputError("registry authority does not match CQR authority")
    for authority_name, authority_path in EXPECTED_AUTHORITY.items():
        _, resolved_authority = _safe_registry_path(
            authority_path, repo_root, f"authority.{authority_name}"
        )
        if not resolved_authority.is_file():
            raise GateInputError(
                f"registry authority is missing: {authority_name}={authority_path}"
            )

    entries = registry.get("adjudications")
    if not isinstance(entries, list):
        raise GateInputError("registry adjudications must be an array")

    seen_ids: set[str] = set()
    seen_symbols: set[tuple[str, str]] = set()
    parsed: list[dict[str, Any]] = []
    invalid: list[dict[str, str]] = []
    for entry in entries:
        if not isinstance(entry, dict):
            raise GateInputError("every adjudication must be an object")
        adjudication_id = entry.get("id")
        function = entry.get("function")
        if (
            not isinstance(adjudication_id, str)
            or not adjudication_id
            or WILDCARD_RE.search(adjudication_id)
        ):
            raise GateInputError("adjudication id must be exact and non-empty")
        if not isinstance(function, str) or not function or WILDCARD_RE.search(function):
            raise GateInputError(
                f"function must be exact and non-empty for {adjudication_id}"
            )
        if adjudication_id in seen_ids:
            raise GateInputError(f"duplicate adjudication id: {adjudication_id}")
        seen_ids.add(adjudication_id)

        file_name, source_path = _safe_registry_path(
            entry.get("file"), repo_root, f"file for {adjudication_id}"
        )
        symbol_key = (file_name, function)
        if symbol_key in seen_symbols:
            raise GateInputError(f"duplicate adjudicated symbol: {file_name}:{function}")
        seen_symbols.add(symbol_key)

        classification = entry.get("classification")
        if classification not in ALLOWED_CLASSIFICATIONS:
            raise GateInputError(
                f"unsupported classification for {adjudication_id}: {classification}"
            )
        if entry.get("status") != "accepted":
            raise GateInputError(f"status must be accepted for {adjudication_id}")
        adjudicated_at_commit = entry.get("adjudicated_at_commit")
        if (
            not isinstance(adjudicated_at_commit, str)
            or not re.fullmatch(r"[0-9a-f]{40}", adjudicated_at_commit)
        ):
            raise GateInputError(
                f"adjudicated_at_commit must be a full commit for {adjudication_id}"
            )
        file_sha256 = entry.get("file_sha256")
        if not isinstance(file_sha256, str) or not SHA256_RE.fullmatch(file_sha256):
            raise GateInputError(f"invalid file_sha256 for {adjudication_id}")
        expected_cyclomatic = _finite_number(
            entry.get("cyclomatic"), f"cyclomatic for {adjudication_id}"
        )

        evidence_key = entry.get("evidence_key")
        if not isinstance(evidence_key, str) or not evidence_key:
            raise GateInputError(f"evidence_key must be non-empty for {adjudication_id}")
        binding_tokens = [evidence_key, function, classification, file_sha256]

        review_evidence = entry.get("review_evidence")
        review_evidence = review_evidence if isinstance(review_evidence, list) else []
        normalized_review_paths: list[str] = []
        for index, evidence in enumerate(review_evidence):
            normalized_evidence, _ = _validated_evidence_record(
                evidence,
                repo_root,
                f"review_evidence[{index}] for {adjudication_id}",
                binding_tokens,
                invalid,
                adjudication_id,
            )
            normalized_review_paths.append(normalized_evidence)
        normalized_adjudication_evidence, _ = _validated_evidence_record(
            entry.get("adjudication_evidence"),
            repo_root,
            f"adjudication_evidence for {adjudication_id}",
            binding_tokens,
            invalid,
            adjudication_id,
        )
        if len(set(normalized_review_paths)) < 2:
            invalid.append(
                {
                    "id": adjudication_id,
                    "reason": "fewer than two distinct review evidence paths",
                }
            )
        if normalized_adjudication_evidence in set(normalized_review_paths):
            invalid.append(
                {
                    "id": adjudication_id,
                    "reason": "adjudication evidence duplicates review evidence",
                }
            )

        if not source_path.is_file():
            invalid.append({"id": adjudication_id, "reason": "source file is missing"})
        else:
            observed_sha256 = _sha256(source_path)
            if observed_sha256 != file_sha256:
                invalid.append(
                    {
                        "id": adjudication_id,
                        "reason": (
                            "source hash is stale: "
                            f"expected {file_sha256}, observed {observed_sha256}"
                        ),
                    }
                )
        historical_sha256, historical_error = _git_file_sha256_at_commit(
            repo_root, adjudicated_at_commit, file_name
        )
        if historical_error is not None:
            invalid.append({"id": adjudication_id, "reason": historical_error})
        elif historical_sha256 != file_sha256:
            invalid.append(
                {
                    "id": adjudication_id,
                    "reason": (
                        "historical source hash mismatch: "
                        f"expected {file_sha256}, observed {historical_sha256}"
                    ),
                }
            )

        parsed.append(
            {
                "id": adjudication_id,
                "file": file_name,
                "function": function,
                "classification": classification,
                "cyclomatic": expected_cyclomatic,
                "file_sha256": file_sha256,
            }
        )
    return parsed, invalid


def _changed_paths(
    repo_root: Path, base_ref: str, head_ref: str | None
) -> list[dict[str, str]]:
    def validate_ref(ref: str, field: str) -> None:
        if not ref or ref.startswith("-") or any(character.isspace() for character in ref):
            raise GateInputError(f"{field} is not a safe Git ref: {ref!r}")
        result = subprocess.run(
            ["git", "rev-parse", "--verify", "--end-of-options", f"{ref}^{{commit}}"],
            cwd=repo_root,
            check=False,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            raise GateInputError(f"{field} does not resolve to a commit: {ref}")

    validate_ref(base_ref, "base_ref")
    if head_ref:
        validate_ref(head_ref, "head_ref")
    revision = f"{base_ref}...{head_ref}" if head_ref else base_ref
    command = [
        "git",
        "diff",
        "-z",
        "--name-status",
        "--find-renames",
        "--diff-filter=ACMRD",
        revision,
        "--",
    ]
    result = subprocess.run(
        command,
        cwd=repo_root,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise GateInputError(
            f"cannot determine touched files from {revision}: {result.stderr.strip()}"
        )
    tokens = [token for token in result.stdout.split("\0") if token]
    changed: list[dict[str, str]] = []
    index = 0
    while index < len(tokens):
        status = tokens[index]
        index += 1
        if status.startswith(("R", "C")):
            if index + 1 >= len(tokens):
                raise GateInputError("malformed Git rename/copy status output")
            old_path = tokens[index]
            new_path = tokens[index + 1]
            index += 2
            changed.extend(
                [
                    {"path": old_path, "status": f"{status}-from"},
                    {"path": new_path, "status": f"{status}-to"},
                ]
            )
        else:
            if index >= len(tokens):
                raise GateInputError("malformed Git changed-path status output")
            changed.append({"path": tokens[index], "status": status})
            index += 1

    # A worktree comparison must include new untracked source files. Commit-to-
    # commit comparisons deliberately use only the two resolved Git trees.
    if head_ref is None:
        untracked_result = subprocess.run(
            ["git", "ls-files", "-z", "--others", "--exclude-standard"],
            cwd=repo_root,
            check=False,
            capture_output=True,
            text=True,
        )
        if untracked_result.returncode != 0:
            raise GateInputError(
                "cannot determine untracked files: "
                f"{untracked_result.stderr.strip()}"
            )
        changed.extend(
            {"path": path, "status": "U"}
            for path in untracked_result.stdout.split("\0")
            if path
        )
    return sorted(changed, key=lambda item: (item["path"], item["status"]))


def _git_output(repo_root: Path, arguments: list[str], description: str) -> str:
    result = subprocess.run(
        ["git", *arguments],
        cwd=repo_root,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise GateInputError(f"cannot read {description}: {result.stderr.strip()}")
    return result.stdout.strip()


def _is_production_source_path(path: str) -> bool:
    return (
        path.startswith("crates/")
        and "/src/" in path
        and "/src/tests/" not in path
        and path.endswith(".rs")
    )


def evaluate(
    crap_payload: dict[str, Any],
    registry: dict[str, Any],
    repo_root: Path,
    touched_paths: list[str | dict[str, str]] | None = None,
    expected_production_crates: set[str] | None = None,
) -> dict[str, Any]:
    """Return a deterministic gate report without writing files."""

    repo_root = repo_root.resolve()
    production_all, raw_rows, reported_production_crates = _production_rows(
        crap_payload, repo_root, expected_production_crates
    )
    if (
        expected_production_crates is not None
        and reported_production_crates != expected_production_crates
    ):
        missing = sorted(expected_production_crates - reported_production_crates)
        unexpected = sorted(reported_production_crates - expected_production_crates)
        raise GateInputError(
            "CRAP report production-crate census mismatch: "
            f"missing={missing}, unexpected={unexpected}"
        )
    adjudications, invalid_adjudications = _load_adjudications(registry, repo_root)
    invalid_ids = {item["id"] for item in invalid_adjudications}
    adjudication_by_symbol = {
        (entry["file"], entry["function"]): entry
        for entry in adjudications
        if entry["id"] not in invalid_ids
    }

    present_symbols = {(row["file"], row["function"]) for row in production_all}
    scoped_prefixes = (
        _production_package_source_prefixes(repo_root, expected_production_crates)
        if expected_production_crates is not None
        else ("crates/",)
    )
    for entry in adjudications:
        if entry["file"].startswith(scoped_prefixes) and (
            entry["file"], entry["function"]
        ) not in present_symbols:
            invalid_adjudications.append(
                {
                    "id": entry["id"],
                    "reason": "adjudicated symbol is absent from the complete CRAP report",
                }
            )
            invalid_ids.add(entry["id"])

    adjudicated_rows: list[dict[str, Any]] = []
    actionable_rows: list[dict[str, Any]] = []
    for row in raw_rows:
        adjudication = adjudication_by_symbol.get((row["file"], row["function"]))
        if (
            adjudication is not None
            and adjudication["id"] not in invalid_ids
            and row["cyclomatic"] == adjudication["cyclomatic"]
        ):
            adjudicated_rows.append({**row, "adjudication_id": adjudication["id"]})
        else:
            actionable_rows.append(row)

    touched_records: list[dict[str, str]] = []
    for item in touched_paths or []:
        if isinstance(item, str):
            raw_path = item
            status_value = "supplied"
        elif isinstance(item, dict):
            raw_path = item.get("path")
            status_value = item.get("status")
            if not isinstance(status_value, str) or not status_value:
                raise GateInputError("touched path status must be non-empty")
        else:
            raise GateInputError("touched path must be a string or status object")
        normalized_path = _repo_relative_path(raw_path, repo_root)
        if _is_production_source_path(normalized_path):
            touched_records.append(
                {"path": normalized_path, "status": status_value}
            )
    touched_records = sorted(
        { (item["path"], item["status"]): item for item in touched_records }.values(),
        key=lambda item: (item["path"], item["status"]),
    )
    touched_production = sorted({item["path"] for item in touched_records})
    touched_set = set(touched_production)
    touched_actionable = [row for row in actionable_rows if row["file"] in touched_set]
    untouched_actionable = [row for row in actionable_rows if row["file"] not in touched_set]
    status = "PASS" if not actionable_rows and not invalid_adjudications else "FAIL"

    return {
        "schema_version": REPORT_SCHEMA_VERSION,
        "status": status,
        "threshold": THRESHOLD,
        "production_filter": EXPECTED_FILTER,
        "deduplication_key": EXPECTED_DEDUPLICATION_KEY,
        "production_entry_count": len(production_all),
        "expected_production_crates": sorted(expected_production_crates or []),
        "reported_production_crates": sorted(reported_production_crates),
        "raw_over_threshold_count": len(raw_rows),
        "adjudicated_count": len(adjudicated_rows),
        "actionable_count": len(actionable_rows),
        "touched_scope_requested": touched_paths is not None,
        "touched_production_files": touched_production,
        "touched_production_records": touched_records,
        "touched_actionable_count": len(touched_actionable),
        "untouched_actionable_count": len(untouched_actionable),
        "raw_over_threshold": raw_rows,
        "adjudicated": adjudicated_rows,
        "actionable": actionable_rows,
        "invalid_adjudications": sorted(
            invalid_adjudications, key=lambda item: (item["id"], item["reason"])
        ),
    }


def render_markdown(report: dict[str, Any]) -> str:
    repository = report.get("repository", {})
    lines = [
        "# Adjudicated CRAP Gate Report",
        "",
        f"Status: `{report['status']}`",
        "",
        f"- Acquisition mode: `{report.get('acquisition_mode', 'not-recorded')}`.",
        f"- Eligible for current-source closure: `{report.get('closure_eligible', False)}`.",
        f"- Debt assessment: `{report.get('debt_status', report['status'])}`.",
        f"- Threshold: CRAP strictly greater than `{report['threshold']:g}` is raw debt.",
        f"- Production entries assessed: `{report['production_entry_count']}`.",
        f"- Raw rows over threshold: `{report['raw_over_threshold_count']}`.",
        f"- Currently adjudicated rows: `{report['adjudicated_count']}`.",
        f"- Actionable rows: `{report['actionable_count']}`.",
        f"- Touched production files: `{len(report['touched_production_files'])}`.",
        f"- Actionable rows in touched files: `{report['touched_actionable_count']}`.",
        f"- Actionable rows outside touched files: `{report['untouched_actionable_count']}`.",
        f"- Source HEAD: `{repository.get('head_commit', 'not-recorded')}`.",
        f"- Touched-file base: `{repository.get('base_commit', 'not-requested')}`.",
        f"- Touched-file head: `{repository.get('comparison_head', 'not-requested')}`.",
        f"- Worktree dirty: `{repository.get('worktree_dirty', 'not-recorded')}`.",
        f"- CRAP JSON SHA-256: `{report.get('crap_json_sha256', 'not-recorded')}`.",
        f"- LCOV SHA-256: `{report.get('lcov_sha256', 'not-applicable')}`.",
        f"- Production source manifest SHA-256: `{report.get('source_manifest_sha256', 'not-applicable')}`.",
        f"- Adjudication registry SHA-256: `{report.get('adjudication_registry_sha256', 'not-recorded')}`.",
        "",
    ]
    if report["touched_production_files"]:
        lines.extend(["## Touched Production Files", ""])
        lines.extend(["| Status | Path |", "| --- | --- |"])
        lines.extend(
            f"| `{item['status']}` | `{item['path']}` |"
            for item in report["touched_production_records"]
        )
        lines.append("")

    def append_rows(title: str, rows: list[dict[str, Any]]) -> None:
        lines.extend([f"## {title}", ""])
        if not rows:
            lines.extend(["None.", ""])
            return
        lines.extend(
            [
                "| File | Function | Line | CC | Coverage | CRAP | Adjudication |",
                "| --- | --- | ---: | ---: | ---: | ---: | --- |",
            ]
        )
        for row in rows:
            coverage = "missing" if row["coverage"] is None else f"{row['coverage']:.6g}"
            lines.append(
                f"| `{row['file']}` | `{row['function']}` | {row['line']} | "
                f"{row['cyclomatic']:.6g} | {coverage} | {row['crap']:.6g} | "
                f"`{row.get('adjudication_id', '')}` |"
            )
        lines.append("")

    append_rows("Adjudicated Rows", report["adjudicated"])
    append_rows("Actionable Rows", report["actionable"])

    lines.extend(["## Invalid Or Stale Adjudications", ""])
    if report["invalid_adjudications"]:
        lines.extend(
            f"- `{item['id']}`: {item['reason']}"
            for item in report["invalid_adjudications"]
        )
        lines.append("")
    else:
        lines.extend(["None.", ""])
    return "\n".join(lines)


def _write_report(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.tmp")
    temporary.write_text(content, encoding="utf-8")
    temporary.replace(path)


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--crap-json", type=Path)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).parents[2])
    parser.add_argument(
        "--adjudications",
        type=Path,
        default=Path(__file__).with_name("adjudicated_crap_exceptions.json"),
    )
    parser.add_argument("--base-ref")
    parser.add_argument("--head-ref")
    parser.add_argument("--acquisition-mode", choices=("fresh", "retained"))
    parser.add_argument("--source-manifest", type=Path)
    parser.add_argument("--snapshot-production-sources", type=Path)
    parser.add_argument("--lcov", type=Path)
    parser.add_argument("--cargo-version-file", type=Path)
    parser.add_argument("--rustc-version-file", type=Path)
    parser.add_argument("--llvm-cov-version-file", type=Path)
    parser.add_argument("--cargo-crap-version-file", type=Path)
    parser.add_argument(
        "--expected-package",
        action="append",
        default=[],
        help="Restrict fresh affected measurement to repeated exact Cargo packages",
    )
    parser.add_argument(
        "--validate-expected-packages",
        action="store_true",
        help="Validate and print affected measurement-to-production package scope",
    )
    parser.add_argument(
        "--expected-package-scope",
        type=Path,
        help="Bind fresh affected assessment to retained preflight scope JSON",
    )
    parser.add_argument("--retained-provenance")
    parser.add_argument("--report-json", type=Path)
    parser.add_argument("--report-markdown", type=Path)
    parser.add_argument(
        "--observational",
        action="store_true",
        help="Publish debt without making actionable rows an execution failure",
    )
    return parser.parse_args()


def main() -> int:
    args = _parse_args()
    repo_root = args.repo_root.resolve()
    try:
        if args.validate_expected_packages:
            scope = resolve_measurement_packages(
                repo_root, set(args.expected_package)
            )
            print(json.dumps(scope, sort_keys=True))
            return 0
        if args.snapshot_production_sources is not None:
            manifest = production_source_manifest(repo_root)
            rendered_manifest = json.dumps(manifest, indent=2, sort_keys=True) + "\n"
            _write_report(args.snapshot_production_sources, rendered_manifest)
            print(
                "production-source-manifest: "
                f"sources={manifest['source_count']} "
                f"sha256={_sha256(args.snapshot_production_sources)}"
            )
            return 0
        if args.crap_json is None:
            raise GateInputError("--crap-json is required for assessment")
        if args.acquisition_mode is None:
            raise GateInputError("--acquisition-mode is required for assessment")
        if args.head_ref and not args.base_ref:
            raise GateInputError("--head-ref requires --base-ref")

        canonical_registry = (
            repo_root / "tools/release/adjudicated_crap_exceptions.json"
        ).resolve()
        if (
            args.acquisition_mode == "fresh"
            and args.adjudications.resolve() != canonical_registry
        ):
            raise GateInputError(
                "fresh closure requires the canonical adjudication registry"
            )
        if (
            args.acquisition_mode == "fresh"
            and _sha256(canonical_registry) != CANONICAL_REGISTRY_SHA256
        ):
            raise GateInputError(
                "canonical adjudication registry hash is not the reviewed gate baseline"
            )
        if args.acquisition_mode == "retained" and (args.base_ref or args.head_ref):
            raise GateInputError(
                "retained assessment cannot claim current touched-file provenance"
            )
        if args.acquisition_mode == "retained" and args.expected_package:
            raise GateInputError(
                "retained assessment cannot claim affected-package measurement"
            )
        if args.acquisition_mode == "retained" and args.expected_package_scope:
            raise GateInputError(
                "retained assessment cannot claim affected scope preflight"
            )

        crap_payload = _read_json(args.crap_json)
        registry = _read_json(args.adjudications)
        touched_paths = (
            _changed_paths(repo_root, args.base_ref, args.head_ref)
            if args.base_ref
            else None
        )
        expected_production_crates: set[str] | None = None
        acquisition_provenance: dict[str, Any]
        if args.acquisition_mode == "fresh":
            required_paths = {
                "--source-manifest": args.source_manifest,
                "--lcov": args.lcov,
                "--cargo-version-file": args.cargo_version_file,
                "--rustc-version-file": args.rustc_version_file,
                "--llvm-cov-version-file": args.llvm_cov_version_file,
                "--cargo-crap-version-file": args.cargo_crap_version_file,
            }
            for option, required_path in required_paths.items():
                if required_path is None or not required_path.is_file():
                    raise GateInputError(f"fresh closure requires {option}")
            supplied_manifest = _read_json(args.source_manifest)
            current_manifest = production_source_manifest(repo_root)
            if supplied_manifest != current_manifest:
                raise GateInputError(
                    "production source manifest is stale or source changed during measurement"
                )
            llvm_cov_version = args.llvm_cov_version_file.read_text(
                encoding="utf-8"
            ).strip()
            cargo_crap_version = args.cargo_crap_version_file.read_text(
                encoding="utf-8"
            ).strip()
            cargo_version = args.cargo_version_file.read_text(
                encoding="utf-8"
            ).strip()
            rustc_version = args.rustc_version_file.read_text(
                encoding="utf-8"
            ).strip()
            if not cargo_version:
                raise GateInputError("fresh closure has an empty cargo version")
            if not rustc_version:
                raise GateInputError("fresh closure has an empty rustc version")
            if llvm_cov_version != "cargo-llvm-cov 0.8.7":
                raise GateInputError("fresh closure has an unexpected llvm-cov version")
            if cargo_crap_version != "cargo-crap 0.2.2":
                raise GateInputError("fresh closure has an unexpected cargo-crap version")
            requested_packages = set(args.expected_package)
            scope_sha256: str | None = None
            if requested_packages:
                if (
                    args.expected_package_scope is None
                    or not args.expected_package_scope.is_file()
                ):
                    raise GateInputError(
                        "fresh affected closure requires --expected-package-scope"
                    )
                scope, scope_sha256 = _validated_scope_artifact(
                    args.expected_package_scope, repo_root, requested_packages
                )
            else:
                if args.expected_package_scope is not None:
                    raise GateInputError(
                        "global closure cannot claim affected package scope"
                    )
                scope = {
                    "measurement_packages": [],
                    "production_packages": sorted(
                        _workspace_production_crates(repo_root)
                    ),
                }
            expected_production_crates = set(scope["production_packages"])
            acquisition_provenance = {
                "source_manifest": str(args.source_manifest),
                "source_count": supplied_manifest.get("source_count"),
                "measurement_input_count": supplied_manifest.get(
                    "measurement_input_count"
                ),
                "cargo_version": cargo_version,
                "rustc_version": rustc_version,
                "llvm_cov_version": llvm_cov_version,
                "cargo_crap_version": cargo_crap_version,
                "measurement_packages": scope["measurement_packages"],
                "production_packages": scope["production_packages"],
                "affected_package_scope_sha256": scope_sha256,
            }
        else:
            if not args.retained_provenance:
                raise GateInputError(
                    "retained assessment requires --retained-provenance"
                )
            provenance_name, provenance_path = _safe_registry_path(
                args.retained_provenance,
                repo_root,
                "retained_provenance",
            )
            if not provenance_path.is_file():
                raise GateInputError(
                    f"retained provenance is missing: {provenance_name}"
                )
            acquisition_provenance = {
                "retained_provenance": provenance_name,
                "retained_provenance_sha256": _sha256(provenance_path),
            }

        report = evaluate(
            crap_payload,
            registry,
            repo_root,
            touched_paths,
            expected_production_crates,
        )
        debt_status = report["status"]
        report["debt_status"] = debt_status
        report["acquisition_mode"] = args.acquisition_mode
        report["closure_eligible"] = args.acquisition_mode == "fresh"
        if args.observational:
            if args.acquisition_mode != "fresh":
                raise GateInputError("--observational requires fresh acquisition")
            report["closure_eligible"] = False
            report["status"] = "OBSERVATION-COMPLETE"
        report["measurement_scope"] = (
            "AFFECTED_PACKAGES" if args.expected_package else "GLOBAL_WORKSPACE"
        )
        report["measured_packages"] = sorted(set(args.expected_package))
        report["acquisition_provenance"] = acquisition_provenance
        if args.acquisition_mode == "retained":
            report["status"] = (
                "ASSESSMENT-PASS" if debt_status == "PASS" else "ASSESSMENT-FAIL"
            )
        head_commit = _git_output(repo_root, ["rev-parse", "HEAD"], "HEAD commit")
        dirty_paths = _git_output(
            repo_root,
            ["status", "--short", "--untracked-files=all"],
            "worktree status",
        ).splitlines()
        base_commit = (
            _git_output(
                repo_root,
                ["rev-parse", "--verify", f"{args.base_ref}^{{commit}}"],
                "base commit",
            )
            if args.base_ref
            else None
        )
        comparison_head = (
            _git_output(
                repo_root,
                ["rev-parse", "--verify", f"{args.head_ref}^{{commit}}"],
                "comparison head",
            )
            if args.head_ref
            else ("WORKTREE" if args.base_ref else None)
        )
        report["repository"] = {
            "head_commit": head_commit,
            "base_ref": args.base_ref,
            "base_commit": base_commit,
            "head_ref": args.head_ref,
            "comparison_head": comparison_head,
            "worktree_dirty": bool(dirty_paths),
            "dirty_paths": dirty_paths,
        }
        report["crap_json_sha256"] = _sha256(args.crap_json)
        report["adjudication_registry_sha256"] = _sha256(args.adjudications)
        if args.acquisition_mode == "fresh":
            report["lcov_sha256"] = _sha256(args.lcov)
            report["source_manifest_sha256"] = _sha256(args.source_manifest)
    except GateInputError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2

    rendered_json = json.dumps(report, indent=2, sort_keys=True) + "\n"
    rendered_markdown = render_markdown(report)
    if args.report_json:
        _write_report(args.report_json, rendered_json)
    if args.report_markdown:
        _write_report(args.report_markdown, rendered_markdown)
    print(
        "adjudicated-crap: "
        f"status={report['status']} raw={report['raw_over_threshold_count']} "
        f"adjudicated={report['adjudicated_count']} "
        f"actionable={report['actionable_count']} "
        f"touched_files={len(report['touched_production_files'])}"
    )
    return 0 if args.observational or report["debt_status"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
