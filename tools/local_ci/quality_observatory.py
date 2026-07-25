#!/usr/bin/env python3
"""Collect and verify identity-bound merged openWEPP quality evidence."""

from __future__ import annotations

import argparse
import fnmatch
import hashlib
import importlib.util
import json
import math
import os
import re
import shutil
import stat
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any


SCHEMA = "openwepp-quality-observatory-v2"
ADMISSION_SCHEMA = "openwepp-quality-observatory-admission-v1"
INVENTORY_SCHEMA = "openwepp-quality-observatory-inventory-v1"
COVERAGE_SCHEMA = "openwepp-quality-observatory-coverage-summary-v1"
ENVELOPE_SCHEMA = "openwepp-quality-observatory-envelope-v2"
PROFILES = ("full", "science-manual")
RUNTIME_CARGO_ARTIFACTS = (
    {"package": "openwepp-assurance", "binary": "openwepp-assurance"},
)
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
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
_ACTIVE_ADMISSION_ID: str | None = None


class QualityError(RuntimeError):
    """Raised for a fail-closed quality observation error."""


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        allow_nan=False,
        ensure_ascii=False,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


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
        raise QualityError(f"cannot read JSON {path}: {error}") from error
    if not isinstance(value, dict):
        raise QualityError(f"JSON root is not an object: {path}")
    return value


def run(
    arguments: list[str],
    *,
    cwd: Path,
    env: dict[str, str] | None = None,
    stdout_path: Path | None = None,
) -> str:
    if stdout_path is None:
        result = subprocess.run(
            arguments,
            cwd=cwd,
            env=env,
            check=False,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            raise QualityError(
                f"command failed ({result.returncode}): {' '.join(arguments)}\n"
                f"{result.stderr.strip()}"
            )
        return result.stdout
    stdout_path.parent.mkdir(parents=True, exist_ok=True)
    with stdout_path.open("wb") as stdout:
        result = subprocess.run(
            arguments,
            cwd=cwd,
            env=env,
            check=False,
            stdout=stdout,
            stderr=subprocess.STDOUT,
        )
    if result.returncode != 0:
        raise QualityError(
            f"command failed ({result.returncode}): {' '.join(arguments)}; "
            f"see {stdout_path}"
        )
    return ""


def tool_output(arguments: list[str], repo: Path) -> str:
    return run(arguments, cwd=repo).strip()


def load_crap_module(repo: Path) -> Any:
    path = repo / "tools/release/check_adjudicated_crap.py"
    spec = importlib.util.spec_from_file_location("openwepp_crap", path)
    if spec is None or spec.loader is None:
        raise QualityError(f"cannot load canonical CRAP evaluator: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def source_manifest(repo: Path) -> dict[str, Any]:
    try:
        return load_crap_module(repo).production_source_manifest(repo)
    except (ValueError, OSError) as error:
        raise QualityError(f"cannot construct source manifest: {error}") from error


def manifest_sha256(manifest: dict[str, Any]) -> str:
    return sha256_bytes(canonical_bytes(manifest))


def parse_export_lines(output: str) -> dict[str, str]:
    exports: dict[str, str] = {}
    for line in output.splitlines():
        if not line.startswith("export "):
            continue
        name, separator, raw = line[7:].partition("=")
        if not separator or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", name):
            raise QualityError("cargo llvm-cov emitted an unsafe environment line")
        if len(raw) >= 2 and raw[0] == "'" and raw[-1] == "'":
            exports[name] = raw[1:-1].replace("'\\''", "'")
        elif re.fullmatch(r"[A-Za-z0-9_./,:+=-]+", raw):
            exports[name] = raw
        else:
            raise QualityError("cargo llvm-cov emitted an unsupported environment value")
    required = {
        "LLVM_PROFILE_FILE",
        "RUSTC_WRAPPER",
        "CARGO_LLVM_COV",
        "CARGO_LLVM_COV_TARGET_DIR",
    }
    if not required.issubset(exports):
        raise QualityError("cargo llvm-cov environment is incomplete")
    return exports


def inventory_from_nextest(payload: dict[str, Any], name: str) -> dict[str, Any]:
    suites = payload.get("rust-suites")
    if not isinstance(suites, dict):
        raise QualityError(f"{name} Nextest inventory has no rust-suites object")
    identities: list[str] = []
    ignored = 0
    mismatched = 0
    for suite_name, suite in suites.items():
        if not isinstance(suite, dict):
            raise QualityError(f"{name} suite is malformed: {suite_name}")
        binary_id = suite.get("binary-id")
        testcases = suite.get("testcases")
        if not isinstance(binary_id, str) or not isinstance(testcases, dict):
            raise QualityError(f"{name} suite identity is malformed: {suite_name}")
        for test_name, testcase in testcases.items():
            if not isinstance(test_name, str) or not isinstance(testcase, dict):
                raise QualityError(f"{name} testcase is malformed")
            filter_match = testcase.get("filter-match")
            if not isinstance(filter_match, dict):
                raise QualityError(f"{name} testcase has no filter-match")
            if testcase.get("ignored") is True:
                ignored += 1
            elif filter_match.get("status") == "matches":
                identities.append(f"{binary_id}::{test_name}")
            else:
                mismatched += 1
    if len(identities) != len(set(identities)):
        raise QualityError(f"{name} inventory contains duplicate test identities")
    identities.sort()
    identity_bytes = ("\n".join(identities) + "\n").encode("utf-8")
    return {
        "schema_version": INVENTORY_SCHEMA,
        "inventory": name,
        "count": len(identities),
        "ignored_count": ignored,
        "mismatched_count": mismatched,
        "identities_sha256": sha256_bytes(identity_bytes),
        "identities": identities,
    }


def enumerate_inventory(
    repo: Path,
    profile: str,
    config: Path,
    target: Path,
    env: dict[str, str],
    canonical: bool = False,
) -> dict[str, Any]:
    arguments = [
        "cargo",
        "nextest",
        "list",
        "--workspace",
        "--profile",
        profile,
        "--target-dir",
        str(target),
        "--config-file",
        str(config),
        "--message-format",
        "json",
    ]
    if canonical:
        arguments.append("--ignore-default-filter")
    try:
        payload = json.loads(run(arguments, cwd=repo, env=env))
    except json.JSONDecodeError as error:
        raise QualityError(f"Nextest returned malformed JSON for {profile}") from error
    if not isinstance(payload, dict):
        raise QualityError(f"Nextest returned a non-object inventory for {profile}")
    return inventory_from_nextest(payload, "workspace" if canonical else profile)


def validate_inventory_partition(
    full: dict[str, Any],
    science: dict[str, Any],
    workspace: dict[str, Any],
) -> None:
    full_set = set(full["identities"])
    science_set = set(science["identities"])
    workspace_set = set(workspace["identities"])
    overlap = full_set & science_set
    if overlap:
        raise QualityError(
            f"profile inventories overlap at {len(overlap)} test identities"
        )
    if full_set | science_set != workspace_set:
        missing = sorted(workspace_set - (full_set | science_set))
        unexpected = sorted((full_set | science_set) - workspace_set)
        raise QualityError(
            "profile union does not equal canonical inventory: "
            f"missing={missing[:5]} unexpected={unexpected[:5]}"
        )


def prepare_nextest_config(repo: Path, local: Path) -> Path:
    source = (repo / ".config/nextest.toml").read_text(encoding="utf-8")
    expected = 'dir = "target/nextest"'
    if source.count(expected) != 1:
        raise QualityError("canonical Nextest store declaration is missing or ambiguous")
    destination = local / "nextest.toml"
    destination.write_text(
        source.replace(expected, f"dir = {json.dumps(str(local / 'nextest'))}"),
        encoding="utf-8",
    )
    return destination


def identity_versions(repo: Path) -> dict[str, str]:
    versions = {
        "cargo": tool_output(["cargo", "--version", "--verbose"], repo),
        "rustc": tool_output(["rustc", "--version", "--verbose"], repo),
        "nextest": tool_output(["cargo", "nextest", "--version"], repo),
        "llvm_cov": tool_output(["cargo", "llvm-cov", "--version"], repo),
        "cargo_crap": tool_output(["cargo", "crap", "--version"], repo),
    }
    if versions["llvm_cov"] != "cargo-llvm-cov 0.8.7":
        raise QualityError(f"unexpected cargo-llvm-cov: {versions['llvm_cov']}")
    if versions["cargo_crap"] != "cargo-crap 0.2.2":
        raise QualityError(f"unexpected cargo-crap: {versions['cargo_crap']}")
    return versions


def safe_fresh_attempt(path: Path) -> tuple[Path, Path]:
    ensure_no_symlink_path(path)
    path.mkdir(parents=True, exist_ok=True)
    if any(path.iterdir()):
        raise QualityError("attempt root must be empty")
    local = path / "local"
    published = path / "published"
    local.mkdir()
    published.mkdir()
    return local, published


def ensure_no_symlink_path(path: Path) -> None:
    current = path
    while True:
        if current.exists() and current.is_symlink():
            raise QualityError(f"path contains a symlink component: {current}")
        if current == current.parent:
            break
        current = current.parent


def required_identity(value: str, field: str) -> str:
    if not value.strip() or "\x00" in value or len(value.encode("utf-8")) > 512:
        raise QualityError(f"{field} must be a nonblank bounded identity")
    return value


def required_sha(value: str, field: str, pattern: re.Pattern[str]) -> str:
    if not pattern.fullmatch(value):
        raise QualityError(f"{field} has an invalid digest")
    return value


def source_tree(repo: Path) -> str:
    value = tool_output(["git", "rev-parse", "HEAD^{tree}"], repo)
    return required_sha(value, "source tree", re.compile(r"^[0-9a-f]{40}$"))


def require_priority_clear(path: Path | None, boundary: str) -> None:
    if path is not None and path.exists():
        raise QualityError(f"TESTGATE priority requested {boundary}")


def final_current_main(repo: Path, head: str, workflow_mode: bool) -> bool:
    if not workflow_mode:
        return True
    output = tool_output(
        ["git", "ls-remote", "--exit-code", "origin", "refs/heads/main"], repo
    )
    fields = output.split()
    if len(fields) != 2 or fields[1] != "refs/heads/main":
        raise QualityError("current main lookup is malformed")
    current = required_sha(
        fields[0], "current main", re.compile(r"^[0-9a-f]{40}$")
    )
    return current == head


def changed_paths(repo: Path) -> list[str]:
    tracked = subprocess.run(
        ["git", "diff", "--name-only", "-z", "HEAD"],
        cwd=repo,
        check=False,
        capture_output=True,
    )
    untracked = subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard", "-z"],
        cwd=repo,
        check=False,
        capture_output=True,
    )
    if tracked.returncode != 0 or untracked.returncode != 0:
        raise QualityError("cannot enumerate exact quality package diff")
    try:
        names = {
            token.decode("utf-8")
            for token in (tracked.stdout + untracked.stdout).split(b"\0")
            if token
        }
    except UnicodeDecodeError as error:
        raise QualityError("quality package diff contains a non-UTF-8 path") from error
    return sorted(names)


def validate_write_set(repo: Path, paths: list[str]) -> None:
    allowed = (
        ".config/nextest.toml",
        "Cargo.toml",
        "tools/local_ci/**",
        "tools/release/**",
        "tests/integration/quality_observatory_*",
        "tests/integration/testgate_*",
        "crates/openwepp-runner/tests/**",
        "docs/work-packages/20260724-quality-observatory-merged-coverage-001/**",
        "docs/work-packages/README.md",
    )
    unexpected = [
        path
        for path in paths
        if not any(fnmatch.fnmatchcase(path, pattern) for pattern in allowed)
    ]
    if unexpected:
        raise QualityError(f"diff escapes declared quality write set: {unexpected}")
    package = (
        "docs/work-packages/20260724-quality-observatory-merged-coverage-001/package.md"
    )
    tracked_package = subprocess.run(
        ["git", "cat-file", "-e", f"HEAD:{package}"],
        cwd=repo,
        check=False,
        capture_output=True,
    )
    if tracked_package.returncode != 0:
        raise QualityError("quality package does not exist in authenticated base HEAD")


def run_light_gates(repo: Path, local: Path, paths: list[str]) -> dict[str, Any]:
    logs = local / "light-logs"
    logs.mkdir()
    light_env = dict(os.environ)
    light_env["CARGO_TARGET_DIR"] = str(local / "light-target")
    gates: list[tuple[str, list[str], dict[str, str] | None]] = [
        ("diff-check", ["git", "diff", "--check"], None),
        ("rustfmt", ["cargo", "fmt", "--all", "--", "--check"], light_env),
        (
            "python-compile",
            [
                str(repo / ".venv/bin/python"),
                "-m",
                "py_compile",
                "tools/local_ci/quality_observatory.py",
                "tools/release/check_adjudicated_crap.py",
            ],
            None,
        ),
        (
            "quality-self-test",
            [
                str(repo / ".venv/bin/python"),
                "tools/local_ci/quality_observatory.py",
                "self-test",
            ],
            None,
        ),
        (
            "adjudicated-crap-unit",
            [
                str(repo / ".venv/bin/python"),
                "-m",
                "unittest",
                "tests.python.test_adjudicated_crap_gate",
            ],
            None,
        ),
        (
            "focused-nextest",
            [
                "cargo",
                "nextest",
                "run",
                "--test",
                "quality_observatory_merged_coverage_contract",
            ],
            light_env,
        ),
        (
            "focused-clippy",
            [
                "cargo",
                "clippy",
                "--test",
                "quality_observatory_merged_coverage_contract",
                "--",
                "-D",
                "warnings",
            ],
            light_env,
        ),
    ]
    receipts: dict[str, Any] = {}
    for name, arguments, environment in gates:
        log = logs / f"{name}.log"
        run(arguments, cwd=repo, env=environment, stdout_path=log)
        receipts[name] = {"status": "PASS", "log_sha256": sha256_file(log)}
    markdown_paths = [path for path in paths if path.endswith(".md")]
    if markdown_paths:
        arguments = ["markdown-doc", "lint"]
        for path in markdown_paths:
            arguments.extend(["--path", path])
        log = logs / "markdown-doc.log"
        run(arguments, cwd=repo, stdout_path=log)
        receipts["markdown-doc"] = {
            "status": "PASS",
            "paths": markdown_paths,
            "log_sha256": sha256_file(log),
        }
    changed_rust = [repo / path for path in paths if path.endswith(".rs")]
    line_counts = {
        path.relative_to(repo).as_posix(): sum(
            1 for _line in path.open(encoding="utf-8")
        )
        for path in changed_rust
        if path.is_file()
    }
    if any(count >= 3000 for count in line_counts.values()):
        raise QualityError("changed nonexempt Rust file exceeds 3000 lines")
    receipts["line-count"] = {
        "status": "PASS",
        "rust_files": line_counts,
        "warn_at": 2000,
        "block_at": 3000,
    }
    package_root = (
        repo
        / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
    )
    required = [
        package_root / "package.md",
        package_root / "prompts/active/kickoff.md",
        package_root / "artifacts/required-reading-map.md",
        package_root / "artifacts/intent-plan.md",
        package_root / "artifacts/snowbench-full-only-row-ledger.json",
        package_root / "artifacts/review-a.md",
        package_root / "artifacts/review-b.md",
        package_root / "artifacts/finding-disposition.md",
    ]
    missing = [path.relative_to(repo).as_posix() for path in required if not path.is_file()]
    if missing:
        raise QualityError(f"pre-heavy required artifacts are missing: {missing}")
    disposition = (package_root / "artifacts/finding-disposition.md").read_text(
        encoding="utf-8"
    )
    if "Open closure-blocking findings: `0`" not in disposition:
        raise QualityError("measurement review disposition has open blocking findings")
    receipts["package-artifacts"] = {
        "status": "PASS",
        "files": {
            path.relative_to(repo).as_posix(): sha256_file(path) for path in required
        },
    }
    return receipts


def create_execution_snapshot(
    source_repo: Path,
    local: Path,
    paths: list[str],
    expected_manifest: dict[str, Any],
) -> Path:
    snapshot = local / "execution-root"
    run(
        [
            "git",
            "clone",
            "--local",
            "--no-hardlinks",
            "--quiet",
            str(source_repo),
            str(snapshot),
        ],
        cwd=source_repo,
    )
    for relative in paths:
        source = source_repo / relative
        destination = snapshot / relative
        if source.is_symlink():
            raise QualityError(f"source diff contains a symlink: {relative}")
        if source.is_file():
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, destination)
        elif not source.exists():
            if destination.is_dir():
                shutil.rmtree(destination)
            else:
                destination.unlink(missing_ok=True)
        else:
            raise QualityError(f"unsupported changed path type: {relative}")
    venv = snapshot / ".venv"
    if venv.exists() or venv.is_symlink():
        raise QualityError("execution snapshot unexpectedly contains .venv")
    venv.symlink_to(source_repo / ".venv", target_is_directory=True)
    exclude_bound_venv_from_git(snapshot)
    observed = source_manifest(snapshot)
    if observed != expected_manifest:
        raise QualityError("execution snapshot does not equal admitted source manifest")
    unwritable_directories = [
        path.relative_to(snapshot).as_posix()
        for path in [snapshot, *snapshot.rglob("*")]
        if path.is_dir()
        and not path.is_symlink()
        and not path.stat().st_mode & stat.S_IWUSR
    ]
    if unwritable_directories:
        raise QualityError(
            "execution snapshot has test-incompatible directories: "
            f"{unwritable_directories[:5]}"
        )
    if source_manifest(snapshot) != expected_manifest:
        raise QualityError("writable execution snapshot identity changed")
    return snapshot


def exclude_bound_venv_from_git(repo: Path) -> None:
    git_directory = repo / ".git"
    git_info = git_directory / "info"
    exclude = git_info / "exclude"
    if git_directory.is_symlink() or not git_directory.is_dir():
        raise QualityError("execution snapshot Git metadata is unsafe")
    if git_info.is_symlink() or not git_info.is_dir():
        raise QualityError("execution snapshot Git info metadata is unsafe")
    if exclude.is_symlink() or not exclude.is_file():
        raise QualityError("execution snapshot Git exclude metadata is unsafe")
    exclude.write_bytes(b"/.venv\n")
    observed = run(
        [
            "git",
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            ".venv",
        ],
        cwd=repo,
    )
    if observed:
        raise QualityError("execution snapshot .venv is not Git-clean")


def working_tree_identity(repo: Path) -> str:
    git_directory = repo / ".git"
    git_info = git_directory / "info"
    local_exclude = git_info / "exclude"
    if git_directory.is_symlink() or not git_directory.is_dir():
        raise QualityError("snapshot Git metadata changed type")
    if git_info.is_symlink() or not git_info.is_dir():
        raise QualityError("snapshot Git info metadata changed type")
    if local_exclude.is_symlink() or not local_exclude.is_file():
        raise QualityError("snapshot Git exclude metadata changed type")
    if local_exclude.read_bytes() != b"/.venv\n":
        raise QualityError("snapshot Git exclude policy is not exact")
    index = subprocess.run(
        ["git", "ls-files", "--stage", "-z"],
        cwd=repo,
        check=False,
        capture_output=True,
    )
    untracked = subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard", "-z"],
        cwd=repo,
        check=False,
        capture_output=True,
    )
    if index.returncode != 0 or untracked.returncode != 0:
        raise QualityError("cannot bind snapshot working-tree identity")
    venv = repo / ".venv"
    if not venv.is_symlink():
        raise QualityError("snapshot .venv must be an explicit symlink")
    venv_row = {
        "kind": "symlink",
        "path": ".venv",
        "sha256": sha256_bytes(os.readlink(venv).encode("utf-8")),
    }
    exclude_row = {
        "kind": "git-local-exclude",
        "path": ".git/info/exclude",
        "sha256": sha256_file(local_exclude),
    }
    tracked_rows: list[dict[str, Any]] = []
    for token in index.stdout.split(b"\0"):
        if not token:
            continue
        try:
            record = token.decode("utf-8")
        except UnicodeDecodeError as error:
            raise QualityError("snapshot index has a non-UTF-8 path") from error
        metadata, separator, name = record.partition("\t")
        if not separator or not name:
            raise QualityError("snapshot index record is malformed")
        path = repo / name
        if path.is_symlink():
            digest = sha256_bytes(os.readlink(path).encode("utf-8"))
            kind = "symlink"
        elif path.is_file():
            digest = sha256_file(path)
            kind = "regular"
        elif not path.exists():
            digest = sha256_bytes(b"")
            kind = "missing"
        else:
            raise QualityError(f"snapshot tracked path is unsafe: {name}")
        tracked_rows.append(
            {
                "index": metadata,
                "kind": kind,
                "path": name,
                "sha256": digest,
            }
        )
    untracked_rows: list[dict[str, Any]] = []
    for token in untracked.stdout.split(b"\0"):
        if not token:
            continue
        try:
            name = token.decode("utf-8")
        except UnicodeDecodeError as error:
            raise QualityError("snapshot has a non-UTF-8 untracked path") from error
        path = repo / name
        if name == ".venv":
            continue
        if path.is_symlink() or not path.is_file():
            raise QualityError(f"snapshot untracked path is unsafe: {name}")
        untracked_rows.append({"path": name, "sha256": sha256_file(path)})
    return sha256_bytes(
        canonical_bytes([venv_row, exclude_row, tracked_rows, untracked_rows])
    )


def instrumented_artifact_manifest(target: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in sorted(target.rglob("*")):
        if path.is_symlink() or not path.is_file():
            continue
        mode = path.stat().st_mode
        if not mode & stat.S_IXUSR:
            continue
        rows.append(
            {
                "path": path.relative_to(target).as_posix(),
                "sha256": sha256_file(path),
                "size": path.stat().st_size,
            }
        )
    if not rows:
        raise QualityError("instrumented inventory produced no executable artifacts")
    return rows


def prime_runtime_cargo_artifacts(
    repo: Path, target: Path, env: dict[str, str]
) -> list[dict[str, str]]:
    artifacts = [dict(item) for item in RUNTIME_CARGO_ARTIFACTS]
    for artifact in artifacts:
        run(
            [
                "cargo",
                "build",
                "--locked",
                "--offline",
                "--config",
                "net.offline=true",
                "--package",
                artifact["package"],
                "--bin",
                artifact["binary"],
                "--target-dir",
                str(target),
            ],
            cwd=repo,
            env=env,
        )
        executable = target / "debug" / artifact["binary"]
        if (
            executable.is_symlink()
            or not executable.is_file()
            or not executable.stat().st_mode & stat.S_IXUSR
        ):
            raise QualityError(
                f"runtime Cargo artifact is not a plain executable: {executable}"
            )
    return artifacts


def require_execution_identity(
    repo: Path,
    target: Path,
    expected_artifacts: list[dict[str, Any]],
    expected_working_tree: str,
    context: str,
) -> None:
    if instrumented_artifact_manifest(target) != expected_artifacts:
        raise QualityError(f"instrumented build changed {context}")
    if working_tree_identity(repo) != expected_working_tree:
        raise QualityError(f"execution snapshot changed {context}")


def admit(args: argparse.Namespace) -> int:
    global _ACTIVE_ADMISSION_ID
    ensure_no_symlink_path(args.attempt_root.absolute())
    source_repo = args.repo.resolve()
    attempt_root = args.attempt_root.resolve()
    if attempt_root == source_repo or source_repo in attempt_root.parents:
        raise QualityError("attempt root must be outside the source repository")
    if attempt_root == Path("/tmp") or Path("/tmp") in attempt_root.parents:
        raise QualityError("attempt root must be durable and outside /tmp")
    runner = required_identity(args.runner, "runner")
    workflow = required_identity(args.workflow, "workflow")
    run_id = required_identity(args.run_id, "run ID")
    run_attempt = required_identity(args.run_attempt, "run attempt")
    local, _ = safe_fresh_attempt(attempt_root)
    paths = changed_paths(source_repo)
    admission_mode = args.admission_mode
    if admission_mode == "development":
        validate_write_set(source_repo, paths)
        light_receipts = run_light_gates(source_repo, local, paths)
    elif admission_mode == "workflow":
        if paths:
            raise QualityError("workflow admission requires an exact clean checkout")
        run(
            [
                sys.executable,
                "-m",
                "py_compile",
                "tools/local_ci/quality_observatory.py",
                "tools/local_ci/quality_observatory_workflow.py",
            ],
            cwd=source_repo,
        )
        run(
            [
                sys.executable,
                "tools/local_ci/quality_observatory_workflow.py",
                "self-test",
            ],
            cwd=source_repo,
        )
        light_receipts = {
            "workflow-admission": {
                "status": "PASS",
                "evidence": "clean exact checkout and compiled controller self-test",
            }
        }
    else:
        raise QualityError("quality admission mode is unsupported")
    head = tool_output(["git", "rev-parse", "HEAD"], source_repo)
    tree = source_tree(source_repo)
    if admission_mode == "workflow":
        workflow_revision = required_sha(
            args.workflow_revision,
            "workflow revision",
            re.compile(r"^[0-9a-f]{40}$"),
        )
        workflow_sha256 = required_sha(
            args.workflow_sha256, "workflow SHA-256", SHA256_RE
        )
        if workflow_revision != head:
            raise QualityError("workflow revision differs from source head")
    else:
        workflow_revision = head
        workflow_path = source_repo / ".github/workflows/quality-observatory.yml"
        workflow_sha256 = (
            sha256_file(workflow_path) if workflow_path.is_file() else "0" * 64
        )
    admitted_source = source_manifest(source_repo)
    repo = create_execution_snapshot(source_repo, local, paths, admitted_source)
    if source_manifest(source_repo) != admitted_source:
        raise QualityError("source changed while constructing execution snapshot")
    target = local / "target"
    target.mkdir()
    config = prepare_nextest_config(repo, local)
    versions = identity_versions(repo)
    manifest = source_manifest(repo)
    base_env = dict(os.environ)
    base_env["CARGO_TARGET_DIR"] = str(target)
    llvm_exports = parse_export_lines(
        run(["cargo", "llvm-cov", "show-env", "--sh"], cwd=repo, env=base_env)
    )
    instrumented_env = {**base_env, **llvm_exports}
    instrumented_env["CARGO_TARGET_DIR"] = str(target)
    full = enumerate_inventory(repo, "full", config, target, instrumented_env)
    science = enumerate_inventory(
        repo, "science-manual", config, target, instrumented_env
    )
    workspace = enumerate_inventory(
        repo, "full", config, target, instrumented_env, canonical=True
    )
    validate_inventory_partition(full, science, workspace)
    runtime_artifacts = prime_runtime_cargo_artifacts(
        repo, target, instrumented_env
    )
    for inventory in (full, science, workspace):
        write_json(local / f"admitted-inventory-{inventory['inventory']}.json", inventory)
    registry = repo / "tools/release/adjudicated_crap_exceptions.json"
    ledger = (
        repo
        / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
        / "artifacts/snowbench-full-only-row-ledger.json"
    )
    build_identity = {
        "coverage_mode": "workspace-default-features-instrument-coverage-cfg-coverage",
        "features": [],
        "instrumented_target": str(target),
        "runtime_cargo_artifacts": runtime_artifacts,
        "llvm_environment": {
            key: llvm_exports[key]
            for key in sorted(llvm_exports)
            if key != "LLVM_PROFILE_FILE"
        },
        "nextest_config_sha256": sha256_file(config),
        "source_manifest_sha256": manifest_sha256(manifest),
        "toolchain": versions,
        "artifacts": instrumented_artifact_manifest(target),
        "working_tree_identity": working_tree_identity(repo),
    }
    build_id = sha256_bytes(canonical_bytes(build_identity))
    admission_payload = {
        "schema_version": ADMISSION_SCHEMA,
        "status": "READY",
        "admission_mode": admission_mode,
        "repo": str(repo),
        "source_repo": str(source_repo),
        "execution_root": str(repo),
        "head_commit": manifest["head_commit"],
        "source_tree": tree,
        "workflow_revision": workflow_revision,
        "workflow_sha256": workflow_sha256,
        "source_manifest": manifest,
        "source_manifest_sha256": manifest_sha256(manifest),
        "ordered_profiles": list(PROFILES),
        "inventories": {
            item["inventory"]: {
                "count": item["count"],
                "identities_sha256": item["identities_sha256"],
                "artifact_sha256": sha256_file(
                    local / f"admitted-inventory-{item['inventory']}.json"
                ),
            }
            for item in (full, science, workspace)
        },
        "instrumented_build_id": build_id,
        "build_identity": build_identity,
        "registry_sha256": sha256_file(registry),
        "snowbench_ledger_sha256": sha256_file(ledger),
        "collector_sha256": sha256_file(
            repo / "tools/local_ci/quality_observatory.py"
        ),
        "changed_paths": paths,
        "light_receipts": light_receipts,
        "pre_heavy_checks": [
            {
                "check": 1,
                "status": "PASS",
                "evidence": "base-committed package and declared write-set reconciliation",
            },
            {
                "check": 2,
                "status": "PASS",
                "evidence": "focused/light receipts, prompt, artifacts, diff, docs, and line count",
            },
            {
                "check": 3,
                "status": "PASS",
                "evidence": "three independently enumerated admitted inventories",
            },
            {
                "check": 4,
                "status": "PASS",
                "evidence": "source/toolchain/config/features/coverage/build identities",
            },
            {
                "check": 5,
                "status": "PASS",
                "evidence": "fresh external attempt with separate local and published roots",
            },
            {
                "check": 6,
                "status": "PASS",
                "evidence": "source, local execution, and compact publication roots separated",
            },
            {
                "check": 7,
                "status": "PASS",
                "evidence": "instrumented full is intentional correctness plus coverage realization",
            },
            {
                "check": 8,
                "status": "PASS",
                "evidence": "full then science-manual then merge/CRAP DAG; no retry configured",
            },
            {
                "check": 9,
                "status": "PASS",
                "evidence": "caller-selected external attempt retains logs and evidence",
            },
            {
                "check": 10,
                "status": "PASS",
                "evidence": "no open measurement tooling defect admitted",
            },
        ],
        "runner": runner,
        "workflow": workflow,
        "run_id": run_id,
        "run_attempt": run_attempt,
    }
    admission_id = sha256_bytes(canonical_bytes(admission_payload))
    admission = {
        "admission_id": admission_id,
        "payload": admission_payload,
    }
    write_json(local / "pre-heavy-admission.json", admission)
    _ACTIVE_ADMISSION_ID = admission_id
    print(
        f"quality-admission: READY id={admission_id} "
        f"full={full['count']} science-manual={science['count']} "
        f"workspace={workspace['count']}"
    )
    return 0


def validate_admission(repo: Path, attempt_root: Path) -> dict[str, Any]:
    admission = read_object(attempt_root / "local/pre-heavy-admission.json")
    payload = admission.get("payload")
    if not isinstance(payload, dict):
        raise QualityError("admission payload is missing")
    if admission.get("admission_id") != sha256_bytes(canonical_bytes(payload)):
        raise QualityError("admission ID does not match canonical payload")
    if payload.get("schema_version") != ADMISSION_SCHEMA:
        raise QualityError("admission schema is unsupported")
    if payload.get("status") != "READY":
        raise QualityError("admission is not READY")
    execution_root = Path(str(payload.get("execution_root", "")))
    if execution_root != repo or execution_root.is_symlink():
        raise QualityError("execution root does not match admitted snapshot")
    current_manifest = source_manifest(repo)
    if current_manifest != payload.get("source_manifest"):
        raise QualityError("source or Git index changed after quality admission")
    if identity_versions(repo) != payload["build_identity"]["toolchain"]:
        raise QualityError("toolchain changed after quality admission")
    if payload["build_identity"].get("runtime_cargo_artifacts") != [
        dict(item) for item in RUNTIME_CARGO_ARTIFACTS
    ]:
        raise QualityError("runtime Cargo artifact declaration changed")
    config = attempt_root / "local/nextest.toml"
    if config.is_symlink() or not config.is_file():
        raise QualityError("admitted Nextest config is missing or unsafe")
    if sha256_file(config) != payload["build_identity"]["nextest_config_sha256"]:
        raise QualityError("Nextest config changed after quality admission")
    target = attempt_root / "local/target"
    if instrumented_artifact_manifest(target) != payload["build_identity"]["artifacts"]:
        raise QualityError("instrumented build artifacts changed after admission")
    if working_tree_identity(repo) != payload["build_identity"]["working_tree_identity"]:
        raise QualityError("execution snapshot working tree changed after admission")
    registry = repo / "tools/release/adjudicated_crap_exceptions.json"
    if sha256_file(registry) != payload.get("registry_sha256"):
        raise QualityError("adjudication registry changed after quality admission")
    ledger = (
        repo
        / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
        / "artifacts/snowbench-full-only-row-ledger.json"
    )
    if sha256_file(ledger) != payload.get("snowbench_ledger_sha256"):
        raise QualityError("snowbench ledger changed after quality admission")
    collector = repo / "tools/local_ci/quality_observatory.py"
    if (
        sha256_file(collector) != payload.get("collector_sha256")
        or sha256_file(Path(__file__).resolve()) != payload.get("collector_sha256")
    ):
        raise QualityError("quality collector changed after admission")
    for name in (*PROFILES, "workspace"):
        inventory_path = attempt_root / f"local/admitted-inventory-{name}.json"
        if sha256_file(inventory_path) != payload["inventories"][name][
            "artifact_sha256"
        ]:
            raise QualityError(f"admitted {name} inventory changed")
    return admission


def profraw_index(directory: Path) -> dict[str, Any]:
    files = sorted(directory.glob("*.profraw"))
    if not files:
        raise QualityError(f"no LLVM raw profiles were produced in {directory}")
    rows = [
        {"name": path.name, "sha256": sha256_file(path), "size": path.stat().st_size}
        for path in files
    ]
    return {
        "count": len(rows),
        "files": rows,
        "set_sha256": sha256_bytes(canonical_bytes(rows)),
    }


def compact_junit(source: Path, destination: Path | None) -> dict[str, Any]:
    try:
        root = ET.parse(source).getroot()
    except (OSError, ET.ParseError) as error:
        raise QualityError(f"cannot parse JUnit {source}: {error}") from error
    identities: list[str] = []
    failures = errors = skipped = 0
    compact_root = ET.Element("testsuites")
    for suite in root.iter("testsuite"):
        compact_suite = ET.SubElement(
            compact_root, "testsuite", {"name": suite.attrib.get("name", "")}
        )
        for testcase in suite.findall("testcase"):
            name = testcase.attrib.get("name")
            classname = testcase.attrib.get("classname")
            if not name or not classname:
                raise QualityError("JUnit testcase lacks name or classname")
            identities.append(f"{classname}::{name}")
            compact_case = ET.SubElement(
                compact_suite, "testcase", {"classname": classname, "name": name}
            )
            for tag in ("failure", "error", "skipped"):
                child = testcase.find(tag)
                if child is not None:
                    ET.SubElement(
                        compact_case,
                        tag,
                        {
                            key: child.attrib[key]
                            for key in ("message", "type")
                            if key in child.attrib
                        },
                    )
                    if tag == "failure":
                        failures += 1
                    elif tag == "error":
                        errors += 1
                    else:
                        skipped += 1
    if len(identities) != len(set(identities)):
        raise QualityError("JUnit contains duplicate test identities")
    identities.sort()
    compact_root.set("tests", str(len(identities)))
    compact_root.set("failures", str(failures))
    compact_root.set("errors", str(errors))
    compact_root.set("skipped", str(skipped))
    ET.indent(compact_root, space="  ")
    if destination is not None:
        destination.write_bytes(
            b'<?xml version="1.0" encoding="UTF-8"?>\n'
            + ET.tostring(compact_root, encoding="utf-8")
            + b"\n"
        )
    return {
        "count": len(identities),
        "identities": identities,
        "identities_sha256": sha256_bytes(
            ("\n".join(identities) + "\n").encode("utf-8")
        ),
        "failures": failures,
        "errors": errors,
        "skipped": skipped,
    }


def execute_profile(
    repo: Path,
    profile: str,
    config: Path,
    target: Path,
    raw_dir: Path,
    env: dict[str, str],
    log: Path,
    expected_artifacts: list[dict[str, Any]],
    expected_working_tree: str,
) -> dict[str, Any]:
    raw_dir.mkdir(parents=True)
    profile_env = dict(env)
    profile_env["LLVM_PROFILE_FILE"] = str(raw_dir / "openwepp-%p-%48m.profraw")
    run(
        [
            "cargo",
            "nextest",
            "run",
            "--workspace",
            "--profile",
            profile,
            "--target-dir",
            str(target),
            "--config-file",
            str(config),
        ],
        cwd=repo,
        env=profile_env,
        stdout_path=log,
    )
    require_execution_identity(
        repo,
        target,
        expected_artifacts,
        expected_working_tree,
        f"while executing {profile}",
    )
    return profraw_index(raw_dir)


def workspace_package_arguments(repo: Path) -> list[str]:
    payload = json.loads(
        run(
            [
                "cargo",
                "metadata",
                "--locked",
                "--offline",
                "--no-deps",
                "--format-version",
                "1",
            ],
            cwd=repo,
        )
    )
    members = set(payload["workspace_members"])
    names = sorted(
        package["name"]
        for package in payload["packages"]
        if package["id"] in members
    )
    if not names:
        raise QualityError("Cargo metadata contains no workspace packages")
    arguments: list[str] = []
    for name in names:
        arguments.extend(["--package", name])
    return arguments


def lcov_report(
    repo: Path,
    target: Path,
    raw_sets: list[Path],
    output: Path,
    env: dict[str, str],
    package_arguments: list[str],
    label: str,
    expected_indexes: list[dict[str, Any]],
    expected_working_tree: str,
) -> None:
    staged: list[Path] = []
    try:
        if working_tree_identity(repo) != expected_working_tree:
            raise QualityError(f"execution snapshot changed before {label} merge")
        if len(raw_sets) != len(expected_indexes):
            raise QualityError("coverage raw-set/index cardinality mismatch")
        for set_index, (raw_dir, expected_index) in enumerate(
            zip(raw_sets, expected_indexes, strict=True)
        ):
            if profraw_index(raw_dir) != expected_index:
                raise QualityError(f"coverage raw input changed before {label} merge")
            for file_index, source in enumerate(sorted(raw_dir.glob("*.profraw"))):
                destination = (
                    target / f"quality-{label}-{set_index}-{file_index}.profraw"
                )
                if destination.exists():
                    raise QualityError(f"coverage staging collision: {destination}")
                os.link(source, destination)
                staged.append(destination)
        report_env = dict(env)
        report_env["LLVM_PROFILE_FILE"] = str(target / "openwepp-%p-%48m.profraw")
        run(
            [
                "cargo",
                "llvm-cov",
                "report",
                *package_arguments,
                "--lcov",
                "--output-path",
                str(output),
            ],
            cwd=repo,
            env=report_env,
        )
        if not output.is_file() or output.stat().st_size == 0:
            raise QualityError(f"coverage report is empty: {output}")
    finally:
        for path in staged:
            path.unlink(missing_ok=True)
    for raw_dir, expected_index in zip(raw_sets, expected_indexes, strict=True):
        if profraw_index(raw_dir) != expected_index:
            raise QualityError(f"coverage raw input changed during {label} merge")
    if working_tree_identity(repo) != expected_working_tree:
        raise QualityError(f"execution snapshot changed during {label} merge")


def cargo_crap(
    repo: Path,
    lcov: Path,
    output: Path,
    log: Path,
    env: dict[str, str],
) -> None:
    run(
        [
            "cargo",
            "crap",
            "--workspace",
            "--lcov",
            str(lcov),
            "--min",
            "0",
            "--format",
            "json",
            "--output",
            str(output),
        ],
        cwd=repo,
        env=env,
        stdout_path=log,
    )


def lcov_summary(path: Path, repo: Path) -> list[dict[str, Any]]:
    summaries: list[dict[str, Any]] = []
    current: dict[str, Any] | None = None
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("SF:"):
            raw = Path(line[3:])
            try:
                name = raw.resolve().relative_to(repo).as_posix()
            except ValueError:
                name = raw.as_posix()
            current = {"file": name}
        elif line == "end_of_record":
            if current is not None:
                summaries.append(current)
            current = None
        elif current is not None:
            key, separator, value = line.partition(":")
            if separator and key in {"LF", "LH", "FNF", "FNH", "BRF", "BRH"}:
                current[key.lower()] = int(value)
    return sorted(summaries, key=lambda item: item["file"])


def crap_entries(
    path: Path, repo: Path
) -> dict[tuple[str, str, str, int, float], dict[str, Any]]:
    payload = read_object(path)
    entries = payload.get("entries")
    if not isinstance(entries, list):
        raise QualityError(f"CRAP report has no entries: {path}")
    result: dict[tuple[str, str, str, int, float], dict[str, Any]] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            raise QualityError(f"CRAP report contains a malformed row: {path}")
        raw_file = Path(str(entry.get("file", "")))
        try:
            file_name = raw_file.resolve().relative_to(repo).as_posix()
        except ValueError:
            file_name = raw_file.as_posix()
        crate_name = entry.get("crate")
        function = entry.get("function")
        line = entry.get("line")
        cyclomatic = entry.get("cyclomatic")
        if (
            isinstance(crate_name, str)
            and isinstance(function, str)
            and isinstance(line, int)
            and isinstance(cyclomatic, (int, float))
        ):
            key = (crate_name, file_name, function, line, float(cyclomatic))
            if key in result:
                existing = result[key]
                if (
                    existing.get("coverage") != entry.get("coverage")
                    or existing.get("crap") != entry.get("crap")
                ):
                    raise QualityError(
                        f"CRAP report contains conflicting exact rows: {key}"
                    )
                continue
            result[key] = {**entry, "file": file_name}
    return result


def snowbench_proof(
    ledger_path: Path,
    science_crap: Path,
    merged_crap: Path,
    repo: Path,
) -> list[dict[str, Any]]:
    ledger = read_object(ledger_path)
    rows = ledger.get("rows")
    if ledger.get("row_count") != 18 or not isinstance(rows, list) or len(rows) != 18:
        raise QualityError("historical snowbench ledger is not the exact 18-row set")
    science = crap_entries(science_crap, repo)
    merged = crap_entries(merged_crap, repo)
    proof: list[dict[str, Any]] = []
    for historical in rows:
        key = (
            historical["crate"],
            historical["file"],
            historical["function"],
            historical["line"],
            float(historical["cyclomatic"]),
        )
        science_row = science.get(key)
        merged_row = merged.get(key)
        if science_row is None:
            raise QualityError(f"science-manual CRAP lacks snowbench symbol: {key}")
        if merged_row is None:
            raise QualityError(f"merged CRAP lacks historical snowbench symbol: {key}")
        science_coverage = science_row.get("coverage")
        contribution = isinstance(science_coverage, (int, float)) and science_coverage > 0
        merged_coverage = merged_row.get("coverage")
        contribution = (
            contribution
            and isinstance(merged_coverage, (int, float))
            and merged_coverage > 0
        )
        disposition = (
            "SCIENCE_MANUAL_CONTRIBUTION"
            if contribution
            else "RETAINED_OBSERVATIONAL_DEBT_REQUIRES_REVIEW"
        )
        proof.append(
            {
                "historical": historical,
                "science_manual": science_row,
                "merged": merged_row,
                "science_manual_contributed": contribution,
                "disposition": disposition,
                "retained_as_debt": not contribution,
                "merged_coverage": merged_coverage,
            }
        )
    return proof


def invoke_observational_crap(
    repo: Path,
    merged_crap: Path,
    merged_lcov: Path,
    source_manifest_path: Path,
    versions: dict[str, str],
    published: Path,
    local: Path,
) -> None:
    version_paths: dict[str, Path] = {}
    for key in ("cargo", "rustc", "llvm_cov", "cargo_crap"):
        path = local / f"{key.replace('_', '-')}-version.txt"
        path.write_text(versions[key] + "\n", encoding="utf-8")
        version_paths[key] = path
    run(
        [
            str(repo / ".venv/bin/python"),
            str(repo / "tools/release/check_adjudicated_crap.py"),
            "--repo-root",
            str(repo),
            "--crap-json",
            str(merged_crap),
            "--adjudications",
            str(repo / "tools/release/adjudicated_crap_exceptions.json"),
            "--acquisition-mode",
            "fresh",
            "--observational",
            "--source-manifest",
            str(source_manifest_path),
            "--lcov",
            str(merged_lcov),
            "--cargo-version-file",
            str(version_paths["cargo"]),
            "--rustc-version-file",
            str(version_paths["rustc"]),
            "--llvm-cov-version-file",
            str(version_paths["llvm_cov"]),
            "--cargo-crap-version-file",
            str(version_paths["cargo_crap"]),
            "--report-json",
            str(published / "adjudicated-crap-report.json"),
            "--report-markdown",
            str(published / "adjudicated-crap-report.md"),
        ],
        cwd=repo,
    )


def inventory_payload(path: Path) -> dict[str, Any]:
    payload = read_object(path)
    if payload.get("schema_version") != INVENTORY_SCHEMA:
        raise QualityError(f"unsupported inventory schema: {path}")
    identities = payload.get("identities")
    if not isinstance(identities, list) or not all(
        isinstance(item, str) for item in identities
    ):
        raise QualityError(f"inventory identities are malformed: {path}")
    if payload.get("count") != len(identities) or identities != sorted(set(identities)):
        raise QualityError(f"inventory count/order is malformed: {path}")
    digest = sha256_bytes(("\n".join(identities) + "\n").encode("utf-8"))
    if payload.get("identities_sha256") != digest:
        raise QualityError(f"inventory identity digest is invalid: {path}")
    return payload


def artifact_digest_map(directory: Path, names: set[str]) -> dict[str, dict[str, Any]]:
    return {
        name: {
            "sha256": sha256_file(directory / name),
            "size": (directory / name).stat().st_size,
        }
        for name in sorted(names)
    }


def collect(args: argparse.Namespace) -> int:
    global _ACTIVE_ADMISSION_ID
    source_repo = args.repo.resolve()
    attempt_root = args.attempt_root.resolve()
    local = attempt_root / "local"
    published = attempt_root / "published"
    unvalidated = read_object(local / "pre-heavy-admission.json")
    unvalidated_payload = unvalidated.get("payload")
    if not isinstance(unvalidated_payload, dict):
        raise QualityError("admission payload is missing")
    repo = Path(str(unvalidated_payload.get("execution_root", "")))
    admission = validate_admission(repo, attempt_root)
    if _ACTIVE_ADMISSION_ID != admission.get("admission_id"):
        raise QualityError(
            "standalone HEAVY is forbidden; use the one-process transition command"
        )
    payload = admission["payload"]
    admitted_source_repo = Path(str(payload.get("source_repo", "")))
    if admitted_source_repo != source_repo:
        raise QualityError("source checkout path differs from admission")
    if source_manifest(source_repo) != payload["source_manifest"]:
        raise QualityError("source checkout changed before heavy execution")
    config = local / "nextest.toml"
    target = local / "target"
    environment = dict(os.environ)
    environment["CARGO_TARGET_DIR"] = str(target)
    llvm_exports = parse_export_lines(
        run(["cargo", "llvm-cov", "show-env", "--sh"], cwd=repo, env=environment)
    )
    environment.update(llvm_exports)
    if (
        sha256_bytes(
            canonical_bytes(
                {
                    **payload["build_identity"],
                    "llvm_environment": {
                        key: llvm_exports[key]
                        for key in sorted(llvm_exports)
                        if key != "LLVM_PROFILE_FILE"
                    },
                }
            )
        )
        != payload["instrumented_build_id"]
    ):
        raise QualityError("instrumented build identity changed after admission")
    raw_root = local / "profraw"
    full_raw = raw_root / "full"
    science_raw = raw_root / "science-manual"
    full_index = execute_profile(
        repo,
        "full",
        config,
        target,
        full_raw,
        environment,
        local / "nextest-full.log",
        payload["build_identity"]["artifacts"],
        payload["build_identity"]["working_tree_identity"],
    )
    require_priority_clear(args.priority_sentinel, "before science-manual")
    science_index = execute_profile(
        repo,
        "science-manual",
        config,
        target,
        science_raw,
        environment,
        local / "nextest-science-manual.log",
        payload["build_identity"]["artifacts"],
        payload["build_identity"]["working_tree_identity"],
    )
    require_priority_clear(args.priority_sentinel, "after science-manual")
    junit_results: dict[str, dict[str, Any]] = {}
    for profile in PROFILES:
        source = local / "nextest" / profile / "junit.xml"
        destination = published / f"junit-{profile}.xml"
        result = compact_junit(source, destination)
        inventory = inventory_payload(local / f"admitted-inventory-{profile}.json")
        if result["identities"] != inventory["identities"]:
            raise QualityError(f"{profile} JUnit does not equal admitted inventory")
        if result["failures"] or result["errors"] or result["skipped"]:
            raise QualityError(f"{profile} JUnit contains non-passing results")
        result.pop("identities")
        junit_results[profile] = result
        shutil.copyfile(
            local / f"admitted-inventory-{profile}.json",
            published / f"inventory-{profile}.json",
        )
    shutil.copyfile(
        local / "admitted-inventory-workspace.json",
        published / "inventory-workspace.json",
    )
    package_arguments = workspace_package_arguments(repo)
    require_priority_clear(args.priority_sentinel, "before CRAP/report work")
    full_lcov = local / "full-only.lcov"
    science_lcov = local / "science-manual-only.lcov"
    merged_lcov = local / "merged.lcov"
    lcov_report(
        repo,
        target,
        [full_raw],
        full_lcov,
        environment,
        package_arguments,
        "full",
        [full_index],
        payload["build_identity"]["working_tree_identity"],
    )
    lcov_report(
        repo,
        target,
        [science_raw],
        science_lcov,
        environment,
        package_arguments,
        "science",
        [science_index],
        payload["build_identity"]["working_tree_identity"],
    )
    lcov_report(
        repo,
        target,
        [full_raw, science_raw],
        merged_lcov,
        environment,
        package_arguments,
        "merged",
        [full_index, science_index],
        payload["build_identity"]["working_tree_identity"],
    )
    require_execution_identity(
        repo,
        target,
        payload["build_identity"]["artifacts"],
        payload["build_identity"]["working_tree_identity"],
        "during coverage derivation",
    )
    science_crap = local / "science-manual-crap.json"
    merged_crap = local / "merged-workspace-crap.json"
    cargo_crap(
        repo,
        science_lcov,
        science_crap,
        local / "cargo-crap-science.log",
        environment,
    )
    cargo_crap(
        repo,
        merged_lcov,
        merged_crap,
        local / "cargo-crap-merged.log",
        environment,
    )
    manifest_path = local / "source-manifest.json"
    write_json(manifest_path, payload["source_manifest"])
    invoke_observational_crap(
        repo,
        merged_crap,
        merged_lcov,
        manifest_path,
        payload["build_identity"]["toolchain"],
        published,
        local,
    )
    ledger_path = (
        repo
        / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
        / "artifacts/snowbench-full-only-row-ledger.json"
    )
    snowbench_rows = snowbench_proof(ledger_path, science_crap, merged_crap, repo)
    coverage_summary = {
        "schema_version": COVERAGE_SCHEMA,
        "coverage_inputs": {
            "full": full_index,
            "science-manual": science_index,
        },
        "lcov": {
            "full_only_sha256": sha256_file(full_lcov),
            "science_manual_only_sha256": sha256_file(science_lcov),
            "merged_sha256": sha256_file(merged_lcov),
        },
        "merged_files": lcov_summary(merged_lcov, repo),
        "snowbench_ledger_sha256": sha256_file(ledger_path),
        "snowbench_gate_status": (
            "PASS"
            if all(row["science_manual_contributed"] for row in snowbench_rows)
            else "REVIEW_REQUIRED"
        ),
        "snowbench_rows": snowbench_rows,
    }
    write_json(published / "coverage-summary.json", coverage_summary)
    require_execution_identity(
        repo,
        target,
        payload["build_identity"]["artifacts"],
        payload["build_identity"]["working_tree_identity"],
        "during quality finalization",
    )
    require_priority_clear(args.priority_sentinel, "before complete publication")
    if source_manifest(repo) != payload["source_manifest"]:
        raise QualityError("execution snapshot changed during quality collection")
    if source_manifest(source_repo) != payload["source_manifest"]:
        raise QualityError("source checkout changed during quality collection")
    report = read_object(published / "adjudicated-crap-report.json")
    if report.get("closure_eligible") is not False:
        raise QualityError("observational CRAP report is incorrectly closure eligible")
    run_status = {
        "schema_version": SCHEMA,
        "execution_integrity": "PASS",
        "debt_status": report.get("debt_status"),
        "closure_eligible": False,
        "admission_id": admission["admission_id"],
    }
    write_json(published / "run-status.json", run_status)
    bound_names = PUBLISHED_FILES - {
        "quality-envelope.json",
        "quality-payload.json",
        "run-status.json",
    }
    artifact_digests = artifact_digest_map(published, bound_names)
    inventories = {
        name: inventory_payload(published / f"inventory-{name}.json")
        for name in (*PROFILES, "workspace")
    }
    validate_inventory_partition(
        inventories["full"], inventories["science-manual"], inventories["workspace"]
    )
    quality_payload = {
        "schema_version": SCHEMA,
        "closure_eligible": False,
        "admission_id": admission["admission_id"],
        "head_commit": payload["head_commit"],
        "subject": {
            "source_commit": payload["head_commit"],
            "source_tree": payload["source_tree"],
            "workflow_revision": payload["workflow_revision"],
            "workflow_sha256": payload["workflow_sha256"],
            "current_main": final_current_main(
                source_repo,
                payload["head_commit"],
                payload.get("admission_mode") == "workflow",
            ),
        },
        "source_manifest_sha256": payload["source_manifest_sha256"],
        "instrumented_build_id": payload["instrumented_build_id"],
        "coverage_mode": payload["build_identity"]["coverage_mode"],
        "features": payload["build_identity"]["features"],
        "runtime_cargo_artifacts": payload["build_identity"][
            "runtime_cargo_artifacts"
        ],
        "toolchain": payload["build_identity"]["toolchain"],
        "control_inputs": {
            "registry_sha256": payload["registry_sha256"],
            "snowbench_ledger_sha256": payload["snowbench_ledger_sha256"],
            "collector_sha256": payload["collector_sha256"],
            "nextest_config_sha256": payload["build_identity"][
                "nextest_config_sha256"
            ],
        },
        "ordered_profiles": list(PROFILES),
        "inventories": {
            name: {
                "count": inventory["count"],
                "identities_sha256": inventory["identities_sha256"],
                "artifact_sha256": sha256_file(
                    published / f"inventory-{name}.json"
                ),
            }
            for name, inventory in inventories.items()
        },
        "junit": {
            profile: {
                **junit_results[profile],
                "sha256": sha256_file(published / f"junit-{profile}.xml"),
            }
            for profile in PROFILES
        },
        "coverage": {
            "merged_lcov_sha256": sha256_file(merged_lcov),
            "summary_sha256": sha256_file(published / "coverage-summary.json"),
        },
        "crap": {
            "registry_sha256": payload["registry_sha256"],
            "workspace_crap_sha256": sha256_file(merged_crap),
            "source_manifest_artifact_sha256": sha256_file(manifest_path),
            "report_json_sha256": sha256_file(
                published / "adjudicated-crap-report.json"
            ),
            "report_markdown_sha256": sha256_file(
                published / "adjudicated-crap-report.md"
            ),
            "raw_count": report.get("raw_over_threshold_count"),
            "adjudicated_count": report.get("adjudicated_count"),
            "actionable_count": report.get("actionable_count"),
            "debt_status": report.get("debt_status"),
        },
        "execution": {
            "runner": payload["runner"],
            "workflow": payload["workflow"],
            "run_id": payload["run_id"],
            "run_attempt": payload["run_attempt"],
        },
        "artifacts": artifact_digests,
    }
    quality_id = sha256_bytes(canonical_bytes(quality_payload))
    write_json(published / "quality-payload.json", quality_payload)
    run_status["quality_evidence_id"] = quality_id
    write_json(published / "run-status.json", run_status)
    envelope_digests = artifact_digest_map(
        published, PUBLISHED_FILES - {"quality-envelope.json"}
    )
    envelope = {
        "schema_version": ENVELOPE_SCHEMA,
        "quality_evidence_id": quality_id,
        "payload": quality_payload,
        "publication": {
            "files": envelope_digests,
            "allowed_files": sorted(PUBLISHED_FILES),
            "max_total_bytes": MAX_PUBLISHED_BYTES,
        },
    }
    write_json(published / "quality-envelope.json", envelope)
    verify_published(
        source_repo,
        published,
        local / "pre-heavy-admission.json",
        independent_inventory=False,
    )
    print(
        f"quality-observation: PASS id={quality_id} "
        f"debt={report.get('debt_status')} actionable={report.get('actionable_count')}"
    )
    return 0


def transition(args: argparse.Namespace) -> int:
    admit(args)
    return collect(args)


def parse_compact_junit(path: Path) -> dict[str, Any]:
    return compact_junit(path, None)


def independent_inventory_partition(repo: Path) -> dict[str, dict[str, Any]]:
    before = source_manifest(repo)
    with tempfile.TemporaryDirectory(prefix="openwepp-quality-verify-") as temporary:
        root = Path(temporary)
        target = root / "target"
        target.mkdir()
        config = prepare_nextest_config(repo, root)
        environment = dict(os.environ)
        environment["CARGO_TARGET_DIR"] = str(target)
        environment.update(
            parse_export_lines(
                run(
                    ["cargo", "llvm-cov", "show-env", "--sh"],
                    cwd=repo,
                    env=environment,
                )
            )
        )
        inventories = {
            "full": enumerate_inventory(
                repo, "full", config, target, environment
            ),
            "science-manual": enumerate_inventory(
                repo, "science-manual", config, target, environment
            ),
            "workspace": enumerate_inventory(
                repo, "full", config, target, environment, canonical=True
            ),
        }
        validate_inventory_partition(
            inventories["full"],
            inventories["science-manual"],
            inventories["workspace"],
        )
    if source_manifest(repo) != before:
        raise QualityError("source changed during independent inventory verification")
    return inventories


def exact_row_key(row: dict[str, Any]) -> tuple[Any, ...]:
    fields = ("crate", "file", "function", "line", "cyclomatic", "coverage", "crap")
    try:
        values = tuple(row[field] for field in fields)
    except KeyError as error:
        raise QualityError(f"compact CRAP row lacks {error.args[0]}") from error
    crate_name, file_name, function, line, cyclomatic, coverage, crap = values
    if (
        not isinstance(crate_name, str)
        or not crate_name
        or not isinstance(file_name, str)
        or not file_name
        or not isinstance(function, str)
        or not function
        or isinstance(line, bool)
        or not isinstance(line, int)
        or line < 1
        or isinstance(cyclomatic, bool)
        or not isinstance(cyclomatic, (int, float))
        or not math.isfinite(float(cyclomatic))
        or coverage is not None
        and (
            isinstance(coverage, bool)
            or not isinstance(coverage, (int, float))
            or not math.isfinite(float(coverage))
        )
        or isinstance(crap, bool)
        or not isinstance(crap, (int, float))
        or not math.isfinite(float(crap))
        or float(crap) <= 30.0
    ):
        raise QualityError("compact CRAP row has invalid field types or metrics")
    return values


def verify_compact_crap(
    repo: Path,
    report: dict[str, Any],
    payload: dict[str, Any],
    *,
    check_current_registry: bool = True,
) -> None:
    raw = report.get("raw_over_threshold")
    adjudicated = report.get("adjudicated")
    actionable = report.get("actionable")
    if not all(isinstance(rows, list) for rows in (raw, adjudicated, actionable)):
        raise QualityError("compact CRAP report lacks exact row arrays")
    raw_keys = [exact_row_key(row) for row in raw]
    adjudicated_keys = [exact_row_key(row) for row in adjudicated]
    actionable_keys = [exact_row_key(row) for row in actionable]
    if len(raw_keys) != len(set(raw_keys)):
        raise QualityError("compact CRAP raw rows contain duplicates")
    sort_key = lambda row: (row["file"], row["line"], row["function"], row["crap"])
    if raw != sorted(raw, key=sort_key):
        raise QualityError("compact CRAP raw rows are not canonically ordered")
    if adjudicated != sorted(adjudicated, key=sort_key):
        raise QualityError("compact CRAP adjudicated rows are not canonically ordered")
    if actionable != sorted(actionable, key=sort_key):
        raise QualityError("compact CRAP actionable rows are not canonically ordered")
    if set(adjudicated_keys) & set(actionable_keys):
        raise QualityError("compact CRAP adjudicated/actionable partitions overlap")
    if set(adjudicated_keys) | set(actionable_keys) != set(raw_keys):
        raise QualityError("compact CRAP partitions do not reconstruct raw rows")
    if (
        report.get("raw_over_threshold_count") != len(raw_keys)
        or report.get("adjudicated_count") != len(adjudicated_keys)
        or report.get("actionable_count") != len(actionable_keys)
    ):
        raise QualityError("compact CRAP counts do not match exact rows")
    if report.get("invalid_adjudications"):
        raise QualityError("compact CRAP report contains invalid adjudications")
    crap_binding = payload.get("crap")
    if not isinstance(crap_binding, dict):
        raise QualityError("payload lacks CRAP identity")
    if (
        report.get("adjudication_registry_sha256")
        != crap_binding.get("registry_sha256")
    ):
        raise QualityError("CRAP registry bindings disagree")
    if report.get("crap_json_sha256") != crap_binding.get("workspace_crap_sha256"):
        raise QualityError("workspace CRAP source digest is invalid")
    if report.get("lcov_sha256") != payload.get("coverage", {}).get(
        "merged_lcov_sha256"
    ):
        raise QualityError("merged LCOV digest is invalid")
    if report.get("source_manifest_sha256") != crap_binding.get(
        "source_manifest_artifact_sha256"
    ):
        raise QualityError("CRAP source-manifest artifact digest is invalid")
    if report.get("closure_eligible") is not False:
        raise QualityError("CRAP report is incorrectly closure eligible")
    if report.get("status") != "OBSERVATION-COMPLETE":
        raise QualityError("CRAP report is not a completed observation")
    if report.get("schema_version") != "openwepp-adjudicated-crap-report-v1":
        raise QualityError("compact CRAP report schema is unsupported")
    if not check_current_registry:
        return
    registry = repo / "tools/release/adjudicated_crap_exceptions.json"
    if crap_binding.get("registry_sha256") != sha256_file(registry):
        raise QualityError("CRAP registry identity is stale")
    registry_payload = read_object(registry)
    try:
        adjudications, invalid = load_crap_module(repo)._load_adjudications(
            registry_payload, repo
        )
    except (ValueError, OSError) as error:
        raise QualityError(f"cannot independently validate CRAP registry: {error}") from error
    if invalid:
        raise QualityError("canonical CRAP registry has invalid adjudications")
    accepted = {
        (entry["file"], entry["function"], float(entry["cyclomatic"])): entry["id"]
        for entry in adjudications
    }
    for row in adjudicated:
        key = (row["file"], row["function"], float(row["cyclomatic"]))
        if accepted.get(key) != row.get("adjudication_id"):
            raise QualityError("compact CRAP adjudicated row lacks exact registry match")
    for row in actionable:
        key = (row["file"], row["function"], float(row["cyclomatic"]))
        if key in accepted:
            raise QualityError("compact CRAP actionable row has an accepted adjudication")
    expected_debt = "PASS" if not actionable and not report["invalid_adjudications"] else "FAIL"
    if report.get("debt_status") != expected_debt:
        raise QualityError("CRAP debt status does not reconstruct from compact rows")


def validate_admission_binding(
    admission_path: Path, payload: dict[str, Any]
) -> dict[str, Any]:
    ensure_no_symlink_path(admission_path.absolute())
    if admission_path.is_symlink() or not admission_path.is_file():
        raise QualityError("terminal verification admission is missing or unsafe")
    admission = read_object(admission_path)
    admitted = admission.get("payload")
    if not isinstance(admitted, dict):
        raise QualityError("terminal verification admission payload is missing")
    if admission.get("admission_id") != sha256_bytes(canonical_bytes(admitted)):
        raise QualityError("terminal verification admission ID is invalid")
    if (
        admitted.get("schema_version") != ADMISSION_SCHEMA
        or admitted.get("status") != "READY"
    ):
        raise QualityError("terminal verification admission is not READY")
    if admission.get("admission_id") != payload.get("admission_id"):
        raise QualityError("published payload does not bind the admitted attempt")
    expected = {
        "head_commit": admitted.get("head_commit"),
        "source_manifest_sha256": admitted.get("source_manifest_sha256"),
        "instrumented_build_id": admitted.get("instrumented_build_id"),
        "ordered_profiles": admitted.get("ordered_profiles"),
        "coverage_mode": admitted.get("build_identity", {}).get("coverage_mode"),
        "features": admitted.get("build_identity", {}).get("features"),
        "runtime_cargo_artifacts": admitted.get("build_identity", {}).get(
            "runtime_cargo_artifacts"
        ),
        "toolchain": admitted.get("build_identity", {}).get("toolchain"),
        "inventories": admitted.get("inventories"),
        "registry_sha256": admitted.get("registry_sha256"),
        "control_inputs": {
            "registry_sha256": admitted.get("registry_sha256"),
            "snowbench_ledger_sha256": admitted.get("snowbench_ledger_sha256"),
            "collector_sha256": admitted.get("collector_sha256"),
            "nextest_config_sha256": admitted.get("build_identity", {}).get(
                "nextest_config_sha256"
            ),
        },
        "execution": {
            "runner": admitted.get("runner"),
            "workflow": admitted.get("workflow"),
            "run_id": admitted.get("run_id"),
            "run_attempt": admitted.get("run_attempt"),
        },
        "subject": {
            "source_commit": admitted.get("head_commit"),
            "source_tree": admitted.get("source_tree"),
            "workflow_revision": admitted.get("workflow_revision"),
            "workflow_sha256": admitted.get("workflow_sha256"),
        },
    }
    observed = {
        "head_commit": payload.get("head_commit"),
        "source_manifest_sha256": payload.get("source_manifest_sha256"),
        "instrumented_build_id": payload.get("instrumented_build_id"),
        "ordered_profiles": payload.get("ordered_profiles"),
        "coverage_mode": payload.get("coverage_mode"),
        "features": payload.get("features"),
        "runtime_cargo_artifacts": payload.get("runtime_cargo_artifacts"),
        "toolchain": payload.get("toolchain"),
        "inventories": payload.get("inventories"),
        "registry_sha256": payload.get("crap", {}).get("registry_sha256"),
        "control_inputs": payload.get("control_inputs"),
        "execution": payload.get("execution"),
        "subject": {
            key: payload.get("subject", {}).get(key)
            for key in (
                "source_commit",
                "source_tree",
                "workflow_revision",
                "workflow_sha256",
            )
        },
    }
    if observed != expected:
        raise QualityError("published identity differs from admitted identity")
    return admitted


def verify_published(
    repo: Path,
    published: Path,
    admission_path: Path,
    *,
    independent_inventory: bool = True,
    check_source: bool = True,
    check_current_controls: bool = True,
) -> str:
    ensure_no_symlink_path(published.absolute())
    if not published.is_dir() or published.is_symlink():
        raise QualityError("published path is not a real directory")
    entries = list(published.iterdir())
    observed = {
        path.name
        for path in entries
        if stat.S_ISREG(path.lstat().st_mode) and not path.is_symlink()
    }
    non_files = [path.name for path in entries if path.name not in observed]
    if non_files or observed != PUBLISHED_FILES:
        raise QualityError(
            f"published file set mismatch: missing={sorted(PUBLISHED_FILES - observed)} "
            f"unexpected={sorted(observed - PUBLISHED_FILES | set(non_files))}"
        )
    total = sum((published / name).stat().st_size for name in observed)
    if total > MAX_PUBLISHED_BYTES:
        raise QualityError(f"published evidence exceeds 100 MiB: {total}")
    payload_path = published / "quality-payload.json"
    payload = read_object(payload_path)
    if payload_path.read_bytes() != canonical_bytes(payload) + b"\n":
        raise QualityError("quality payload is not canonical JSON")
    quality_id = sha256_bytes(canonical_bytes(payload))
    if payload.get("schema_version") != SCHEMA:
        raise QualityError("quality payload schema is unsupported")
    if payload.get("ordered_profiles") != list(PROFILES):
        raise QualityError("quality payload profile order is invalid")
    subject = payload.get("subject")
    if (
        not isinstance(subject, dict)
        or set(subject)
        != {
            "source_commit",
            "source_tree",
            "workflow_revision",
            "workflow_sha256",
            "current_main",
        }
        or subject.get("source_commit") != payload.get("head_commit")
        or not re.fullmatch(r"[0-9a-f]{40}", str(subject.get("source_tree", "")))
        or not re.fullmatch(
            r"[0-9a-f]{40}", str(subject.get("workflow_revision", ""))
        )
        or not SHA256_RE.fullmatch(str(subject.get("workflow_sha256", "")))
        or not isinstance(subject.get("current_main"), bool)
    ):
        raise QualityError("quality payload subject identity is invalid")
    if (
        payload.get("coverage_mode")
        != "workspace-default-features-instrument-coverage-cfg-coverage"
        or payload.get("features") != []
        or payload.get("runtime_cargo_artifacts")
        != [dict(item) for item in RUNTIME_CARGO_ARTIFACTS]
    ):
        raise QualityError(
            "quality payload coverage mode/features/runtime artifacts are invalid"
        )
    toolchain = payload.get("toolchain")
    execution = payload.get("execution")
    if (
        not isinstance(toolchain, dict)
        or set(toolchain) != {"cargo", "rustc", "nextest", "llvm_cov", "cargo_crap"}
        or not isinstance(execution, dict)
        or set(execution) != {"runner", "workflow", "run_id", "run_attempt"}
        or not all(isinstance(value, str) and value for value in execution.values())
    ):
        raise QualityError("quality payload toolchain/execution identity is invalid")
    admitted = validate_admission_binding(admission_path, payload)
    if payload["subject"]["workflow_revision"] != payload["head_commit"]:
        raise QualityError("published workflow revision differs from subject HEAD")
    if check_current_controls:
        if identity_versions(repo) != payload["toolchain"]:
            raise QualityError("terminal toolchain differs from admitted toolchain")
        if sha256_file(Path(__file__).resolve()) != payload["control_inputs"].get(
            "collector_sha256"
        ):
            raise QualityError("terminal collector differs from admitted collector")
        workflow_path = repo / ".github/workflows/quality-observatory.yml"
        if (
            not workflow_path.is_file()
            or sha256_file(workflow_path) != payload["subject"]["workflow_sha256"]
        ):
            raise QualityError("terminal workflow identity differs from admission")
    envelope = read_object(published / "quality-envelope.json")
    if (published / "quality-envelope.json").read_bytes() != canonical_bytes(
        envelope
    ) + b"\n":
        raise QualityError("quality envelope is not canonical JSON")
    if envelope.get("schema_version") != ENVELOPE_SCHEMA:
        raise QualityError("quality envelope schema is unsupported")
    if set(envelope) != {
        "schema_version",
        "quality_evidence_id",
        "payload",
        "publication",
    }:
        raise QualityError("quality envelope contains unexpected fields")
    if envelope.get("quality_evidence_id") != quality_id:
        raise QualityError("quality evidence ID does not match canonical payload")
    if envelope.get("payload") != payload:
        raise QualityError("envelope payload differs from quality-payload.json")
    if "quality_evidence_id" in payload:
        raise QualityError("quality payload contains its derived ID")
    if payload.get("closure_eligible") is not False:
        raise QualityError("quality payload is incorrectly closure eligible")
    publication = envelope.get("publication")
    if not isinstance(publication, dict):
        raise QualityError("envelope publication metadata is missing")
    if publication.get("allowed_files") != sorted(PUBLISHED_FILES):
        raise QualityError("envelope publication allowlist is invalid")
    if publication.get("max_total_bytes") != MAX_PUBLISHED_BYTES:
        raise QualityError("envelope publication size policy is invalid")
    expected_envelope_files = PUBLISHED_FILES - {"quality-envelope.json"}
    if publication.get("files") != artifact_digest_map(
        published, expected_envelope_files
    ):
        raise QualityError("envelope publication digests are invalid")
    if payload.get("artifacts") != artifact_digest_map(
        published,
        PUBLISHED_FILES
        - {"quality-envelope.json", "quality-payload.json", "run-status.json"},
    ):
        raise QualityError("payload artifact digests are invalid")
    inventories = {
        name: inventory_payload(published / f"inventory-{name}.json")
        for name in (*PROFILES, "workspace")
    }
    validate_inventory_partition(
        inventories["full"], inventories["science-manual"], inventories["workspace"]
    )
    if independent_inventory:
        independently_observed = independent_inventory_partition(repo)
        for name in (*PROFILES, "workspace"):
            if independently_observed[name]["identities"] != inventories[name][
                "identities"
            ]:
                raise QualityError(
                    f"independent {name} inventory differs from published inventory"
                )
    for name, inventory in inventories.items():
        bound = payload["inventories"].get(name)
        if not isinstance(bound, dict):
            raise QualityError(f"payload lacks {name} inventory binding")
        if bound != {
            "count": inventory["count"],
            "identities_sha256": inventory["identities_sha256"],
            "artifact_sha256": sha256_file(published / f"inventory-{name}.json"),
        }:
            raise QualityError(f"payload {name} inventory binding is invalid")
    for profile in PROFILES:
        result = parse_compact_junit(published / f"junit-{profile}.xml")
        if result["identities"] != inventories[profile]["identities"]:
            raise QualityError(f"{profile} compact JUnit differs from inventory")
        result.pop("identities")
        expected = {
            **result,
            "sha256": sha256_file(published / f"junit-{profile}.xml"),
        }
        if payload["junit"].get(profile) != expected:
            raise QualityError(f"payload {profile} JUnit binding is invalid")
    report = read_object(published / "adjudicated-crap-report.json")
    verify_compact_crap(
        repo,
        report,
        payload,
        check_current_registry=check_current_controls,
    )
    if payload.get("crap", {}).get("raw_count") != report.get(
        "raw_over_threshold_count"
    ):
        raise QualityError("CRAP raw-row count binding is invalid")
    if payload.get("crap", {}).get("adjudicated_count") != report.get(
        "adjudicated_count"
    ):
        raise QualityError("CRAP adjudicated-row count binding is invalid")
    if payload.get("crap", {}).get("actionable_count") != report.get(
        "actionable_count"
    ):
        raise QualityError("CRAP actionable-row count binding is invalid")
    coverage = read_object(published / "coverage-summary.json")
    snowbench = coverage.get("snowbench_rows")
    if not isinstance(snowbench, list) or len(snowbench) != 18:
        raise QualityError("coverage summary lacks the exact 18-row snowbench proof")
    if coverage.get("snowbench_ledger_sha256") != admitted.get(
        "snowbench_ledger_sha256"
    ):
        raise QualityError("snowbench proof does not bind the admitted ledger")
    if check_current_controls:
        ledger = read_object(
            repo
            / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
            / "artifacts/snowbench-full-only-row-ledger.json"
        )
        if [row.get("historical") for row in snowbench] != ledger.get("rows"):
            raise QualityError("snowbench proof does not reproduce the historical ledger")
    if coverage.get("snowbench_gate_status") != "PASS":
        raise QualityError("snowbench coverage contribution requires review")
    for row in snowbench:
        if (
            row.get("disposition") != "SCIENCE_MANUAL_CONTRIBUTION"
            or row.get("science_manual_contributed") is not True
            or row.get("retained_as_debt") is not False
        ):
            raise QualityError("snowbench row lacks science-manual contribution proof")
        historical = row["historical"]
        exact = (
            historical["crate"],
            historical["file"],
            historical["function"],
            historical["line"],
            float(historical["cyclomatic"]),
        )
        for label in ("science_manual", "merged"):
            current = row.get(label)
            if not isinstance(current, dict):
                raise QualityError(f"snowbench {label} exact row is missing")
            current_key = (
                current.get("crate"),
                Path(str(current.get("file"))).as_posix(),
                current.get("function"),
                current.get("line"),
                float(current.get("cyclomatic")),
            )
            if current_key != exact or not isinstance(
                current.get("coverage"), (int, float)
            ) or current["coverage"] <= 0:
                raise QualityError(f"snowbench {label} contribution is invalid")
    status = read_object(published / "run-status.json")
    if (
        set(status)
        != {
            "schema_version",
            "execution_integrity",
            "debt_status",
            "closure_eligible",
            "admission_id",
            "quality_evidence_id",
        }
        or status.get("schema_version") != SCHEMA
        or status.get("quality_evidence_id") != quality_id
        or status.get("execution_integrity") != "PASS"
        or status.get("closure_eligible") is not False
        or status.get("admission_id") != payload.get("admission_id")
        or status.get("debt_status") != report.get("debt_status")
    ):
        raise QualityError("run status does not bind a valid observation")
    if check_source:
        current = source_manifest(repo)
        if manifest_sha256(current) != payload.get("source_manifest_sha256"):
            raise QualityError("quality evidence is stale for current source")
        if current.get("head_commit") != payload.get("head_commit"):
            raise QualityError("quality evidence HEAD does not match current source")
        if source_tree(repo) != subject.get("source_tree"):
            raise QualityError("quality evidence source tree does not match current source")
    return quality_id


def verify_command(args: argparse.Namespace) -> int:
    quality_id = verify_published(
        args.repo.resolve(),
        args.published_dir,
        args.admission,
    )
    print(f"quality-verification: PASS id={quality_id}")
    return 0


def self_test() -> int:
    first = {"z": 1, "a": ["x", False]}
    second = {"a": ["x", False], "z": 1}
    if canonical_bytes(first) != canonical_bytes(second):
        raise QualityError("canonical JSON is order-dependent")
    inventory_full = {
        "identities": ["a::one"],
    }
    inventory_science = {
        "identities": ["b::two"],
    }
    inventory_workspace = {
        "identities": ["a::one", "b::two"],
    }
    validate_inventory_partition(inventory_full, inventory_science, inventory_workspace)
    try:
        validate_inventory_partition(
            inventory_full,
            {"identities": ["a::one"]},
            inventory_workspace,
        )
    except QualityError:
        pass
    else:
        raise QualityError("overlapping profile inventories were accepted")
    try:
        parse_export_lines("export LLVM_PROFILE_FILE='/tmp/a'\n")
    except QualityError:
        pass
    else:
        raise QualityError("incomplete LLVM environment was accepted")
    if len(PUBLISHED_FILES) != 11 or MAX_PUBLISHED_BYTES != 104857600:
        raise QualityError("publication contract drifted")
    with tempfile.TemporaryDirectory(prefix="openwepp-quality-self-test-") as temporary:
        root = Path(temporary)
        snapshot = root / "snapshot"
        snapshot.mkdir()
        run(["git", "init", "--quiet"], cwd=snapshot)
        first_venv = root / "first-venv"
        second_venv = root / "second-venv"
        first_venv.mkdir()
        second_venv.mkdir()
        (snapshot / ".venv").symlink_to(first_venv, target_is_directory=True)
        (snapshot / ".git/info/exclude").write_text("*\n", encoding="utf-8")
        exclude_bound_venv_from_git(snapshot)
        if (snapshot / ".git/info/exclude").read_bytes() != b"/.venv\n":
            raise QualityError("broad pre-existing Git exclude policy survived")
        if run(
            ["git", "status", "--porcelain=v1", "--untracked-files=all"],
            cwd=snapshot,
        ):
            raise QualityError("identity-bound .venv left the snapshot Git-dirty")
        first_identity = working_tree_identity(snapshot)
        visible_drift = snapshot / "must-remain-visible.rs"
        visible_drift.write_text("drift\n", encoding="utf-8")
        if "must-remain-visible.rs" not in run(
            ["git", "status", "--porcelain=v1", "--untracked-files=all"],
            cwd=snapshot,
        ):
            raise QualityError("exact .venv exclude hid other untracked drift")
        if working_tree_identity(snapshot) == first_identity:
            raise QualityError("other untracked drift did not change working-tree identity")
        visible_drift.unlink()
        (snapshot / ".venv").unlink()
        (snapshot / ".venv").symlink_to(second_venv, target_is_directory=True)
        if working_tree_identity(snapshot) == first_identity:
            raise QualityError(
                "excluded .venv symlink-target drift did not change working-tree identity"
            )
        second_identity = working_tree_identity(snapshot)
        with (snapshot / ".git/info/exclude").open("a", encoding="utf-8") as exclude:
            exclude.write("/hidden-drift\n")
        try:
            working_tree_identity(snapshot)
        except QualityError:
            pass
        else:
            raise QualityError("Git exclude-policy drift was accepted")
        (snapshot / ".git/info/exclude").write_bytes(b"/.venv\n")
        git_info = snapshot / ".git/info"
        saved_info = snapshot / ".git/info-real"
        git_info.rename(saved_info)
        git_info.symlink_to(saved_info, target_is_directory=True)
        try:
            working_tree_identity(snapshot)
        except QualityError:
            pass
        else:
            raise QualityError("Git info-directory symlink was accepted")
        git_info.unlink()
        saved_info.rename(git_info)
        if working_tree_identity(snapshot) != second_identity:
            raise QualityError("restored Git metadata did not restore identity")
        target = root / "instrumented-target"
        target.mkdir()
        executable = target / "admitted-tool"
        executable.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        executable.chmod(0o755)
        expected_artifacts = instrumented_artifact_manifest(target)
        expected_working_tree = working_tree_identity(snapshot)
        require_execution_identity(
            snapshot,
            target,
            expected_artifacts,
            expected_working_tree,
            "during self-test",
        )
        added = target / "runtime-growth"
        added.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        added.chmod(0o755)
        try:
            require_execution_identity(
                snapshot,
                target,
                expected_artifacts,
                expected_working_tree,
                "during self-test",
            )
        except QualityError:
            pass
        else:
            raise QualityError("post-admission executable growth was accepted")
        row = {
            "crate": "example",
            "file": str(root / "crates/example/src/lib.rs"),
            "function": "example",
            "line": 7,
            "cyclomatic": 4.0,
            "coverage": 50.0,
            "crap": 12.0,
        }
        report = root / "crap.json"
        write_json(report, {"entries": [row, dict(row)]})
        if len(crap_entries(report, root)) != 1:
            raise QualityError("identical CRAP rows did not deduplicate")
        write_json(
            report,
            {"entries": [row, {**row, "coverage": 25.0, "crap": 20.0}]},
        )
        try:
            crap_entries(report, root)
        except QualityError:
            pass
        else:
            raise QualityError("conflicting CRAP rows were accepted")
    print("quality-observatory-self-test: PASS")
    return 0


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    subcommands = root.add_subparsers(dest="command", required=True)
    admission = subcommands.add_parser("admit")
    admission.add_argument("--repo", type=Path, default=Path(__file__).parents[2])
    admission.add_argument("--attempt-root", type=Path, required=True)
    admission.add_argument("--runner", required=True)
    admission.add_argument("--workflow", required=True)
    admission.add_argument("--run-id", required=True)
    admission.add_argument("--run-attempt", required=True)
    admission.add_argument(
        "--admission-mode",
        choices=("development", "workflow"),
        default="development",
    )
    admission.add_argument("--workflow-revision", default="")
    admission.add_argument("--workflow-sha256", default="")
    admission.add_argument("--priority-sentinel", type=Path)
    admission.set_defaults(function=admit)
    collection = subcommands.add_parser("collect")
    collection.add_argument("--repo", type=Path, default=Path(__file__).parents[2])
    collection.add_argument("--attempt-root", type=Path, required=True)
    collection.set_defaults(function=collect)
    combined = subcommands.add_parser("transition")
    combined.add_argument("--repo", type=Path, default=Path(__file__).parents[2])
    combined.add_argument("--attempt-root", type=Path, required=True)
    combined.add_argument("--runner", required=True)
    combined.add_argument("--workflow", required=True)
    combined.add_argument("--run-id", required=True)
    combined.add_argument("--run-attempt", required=True)
    combined.add_argument(
        "--admission-mode",
        choices=("development", "workflow"),
        default="development",
    )
    combined.add_argument("--workflow-revision", default="")
    combined.add_argument("--workflow-sha256", default="")
    combined.add_argument("--priority-sentinel", type=Path)
    combined.set_defaults(function=transition)
    verification = subcommands.add_parser("verify")
    verification.add_argument("--repo", type=Path, default=Path(__file__).parents[2])
    verification.add_argument("--published-dir", type=Path, required=True)
    verification.add_argument("--admission", type=Path, required=True)
    verification.set_defaults(function=verify_command)
    check = subcommands.add_parser("self-test")
    check.set_defaults(function=lambda _args: self_test())
    return root


def main() -> int:
    args = parser().parse_args()
    try:
        return int(args.function(args))
    except (QualityError, OSError, KeyError, TypeError, ValueError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
