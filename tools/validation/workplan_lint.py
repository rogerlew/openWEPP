#!/usr/bin/env python3
"""Read-only, non-authoritative work-package advisory analysis."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import selectors
import stat
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import Any, Callable, Sequence

SCHEMA_VERSION = "1.0.0"
MAX_FILE_BYTES = 2 * 1024 * 1024
MAX_GIT_BYTES = 4 * 1024 * 1024
GIT_TIMEOUT_SECONDS = 5
GIT = Path("/usr/bin/git")
OWNED_PATH = "/usr/bin:/bin"
MODES = ("pre-edit", "working-tree", "terminal")
FORMATS = ("human", "json")
REVISION = re.compile(r"[A-Za-z0-9][A-Za-z0-9._/@{}^~:+-]{0,199}\Z")
HEX40 = re.compile(r"[0-9a-f]{40}\Z")
PACKAGE_ID = re.compile(r"`([^`\n]+)`")

GIT_PREFIX = (
    str(GIT),
    "--no-pager",
    "-c",
    "core.fsmonitor=false",
    "-c",
    "maintenance.auto=false",
    "-c",
    "core.hooksPath=/dev/null",
)
GIT_ENV = {
    "PATH": OWNED_PATH,
    "LC_ALL": "C",
    "LANG": "C",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_COUNT": "0",
    "GIT_TERMINAL_PROMPT": "0",
    "GIT_OPTIONAL_LOCKS": "0",
    "GIT_PAGER": "cat",
    "PAGER": "cat",
}
POLICY_PATHS = (
    "AGENTS.md",
    "docs/work-packages/AGENTS.md",
    "docs/standards/testing-and-gate-strategy.md",
    "docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md",
)
PRUNED_ATTRIBUTE_DIRECTORIES = {
    ".git",
    ".venv",
    ".mypy_cache",
    ".pytest_cache",
    "__pycache__",
    "node_modules",
    "target",
    "build",
    "dist",
}


class InvocationError(ValueError):
    """The user-facing command shape is invalid."""


class AnalysisUnavailable(RuntimeError):
    """A bounded read-only analysis could not be performed."""

    def __init__(self, analysis_id: str, reason_code: str, message: str) -> None:
        super().__init__(message)
        self.analysis_id = analysis_id
        self.reason_code = reason_code
        self.message = message

    def value(self) -> dict[str, str]:
        return {
            "analysis_id": self.analysis_id,
            "reason_code": self.reason_code,
            "message": self.message,
        }


@dataclass(frozen=True)
class PackageDeclaration:
    identifier: str
    path: str
    base: str
    write_set: tuple[str, ...]
    has_intent: bool
    declaration_issues: tuple[str, ...]
    text: str


@dataclass(frozen=True)
class GitObservation:
    root: Path
    base_sha: str
    head_sha: str
    changed_paths: tuple[str, ...]
    scope: dict[str, bool]


Runner = Callable[[Path, Sequence[str]], bytes]


def canonical_json(value: object) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def normalize_relative(value: str, *, label: str) -> str:
    if not value or value.startswith("-") or "\x00" in value or "\\" in value:
        raise InvocationError(f"{label} must be a normalized repository-relative path")
    path = PurePosixPath(value)
    if path.is_absolute() or any(part in ("", ".", "..") for part in path.parts):
        raise InvocationError(f"{label} must be a normalized repository-relative path")
    return path.as_posix()


def read_regular(root: Path, relative: str, *, required: bool = True) -> bytes | None:
    normalized = normalize_relative(relative, label="read path")
    candidate = root.joinpath(*PurePosixPath(normalized).parts)
    try:
        resolved_parent = candidate.parent.resolve(strict=True)
    except OSError as error:
        if not required and isinstance(error, FileNotFoundError):
            return None
        raise AnalysisUnavailable("filesystem", "PATH_UNAVAILABLE", str(error)) from error
    try:
        resolved_parent.relative_to(root)
    except ValueError:
        raise AnalysisUnavailable(
            "filesystem", "PATH_ESCAPE", f"path escapes repository: {relative}"
        ) from None
    flags = os.O_RDONLY
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    try:
        descriptor = os.open(candidate, flags)
    except FileNotFoundError:
        if not required:
            return None
        raise AnalysisUnavailable(
            "filesystem", "MISSING_FILE", f"required file is missing: {relative}"
        ) from None
    except OSError as error:
        raise AnalysisUnavailable("filesystem", "OPEN_FAILED", str(error)) from error
    try:
        metadata = os.fstat(descriptor)
        if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1:
            raise AnalysisUnavailable(
                "filesystem",
                "UNSAFE_FILE_TYPE",
                f"path is not a unique regular file: {relative}",
            )
        if metadata.st_size > MAX_FILE_BYTES:
            raise AnalysisUnavailable(
                "filesystem", "FILE_TOO_LARGE", f"bounded read exceeded: {relative}"
            )
        value = bytearray()
        while len(value) <= MAX_FILE_BYTES:
            block = os.read(descriptor, min(65536, MAX_FILE_BYTES + 1 - len(value)))
            if not block:
                break
            value.extend(block)
        if len(value) > MAX_FILE_BYTES:
            raise AnalysisUnavailable(
                "filesystem", "FILE_TOO_LARGE", f"bounded read exceeded: {relative}"
            )
        return bytes(value)
    finally:
        os.close(descriptor)


def discover_root(start: Path) -> Path:
    current = start.resolve(strict=True)
    for candidate in (current, *current.parents):
        marker = candidate / ".git"
        if marker.is_dir() and not marker.is_symlink():
            return candidate
        if marker.exists():
            raise AnalysisUnavailable(
                "repository", "UNSUPPORTED_GIT_MARKER", ".git must be a directory"
            )
    raise AnalysisUnavailable(
        "repository", "NOT_A_REPOSITORY", "no bounded repository root was found"
    )


def prohibited_config(root: Path) -> list[str]:
    findings: list[str] = []
    for relative in (".git/config", ".git/config.worktree"):
        raw = read_regular(root, relative, required=False)
        if raw is None:
            continue
        section = ""
        for line_number, raw_line in enumerate(
            raw.decode("utf-8", errors="replace").splitlines(), 1
        ):
            line = raw_line.strip()
            if not line or line.startswith(("#", ";")):
                continue
            if line.startswith("[") and line.endswith("]"):
                section = line[1:-1].strip().lower()
                if (
                    section.startswith("include")
                    or section.startswith("alias")
                    or section.startswith("pager")
                    or section.startswith("credential")
                    or section.startswith("url ")
                    or section.startswith("filter ")
                ):
                    findings.append(f"{relative}:{line_number}:{section}")
                continue
            key = line.split("=", 1)[0].strip().lower()
            qualified = f"{section}.{key}"
            if (
                qualified
                in {
                    "core.fsmonitor",
                    "core.hookspath",
                    "core.pager",
                    "core.attributesfile",
                    "diff.external",
                }
                or section.startswith("maintenance ")
                or (section == "maintenance" and key.startswith("repo"))
                or key in {"textconv", "command", "helper", "insteadOf".lower()}
                or key in {"clean", "smudge", "process"}
            ):
                findings.append(f"{relative}:{line_number}:{qualified}")
    attribute_paths = [".git/info/attributes"]
    for directory, names, _files in os.walk(root, followlinks=False):
        directory_path = Path(directory)
        if directory_path == root / ".git":
            names[:] = []
            continue
        names[:] = [
            name
            for name in names
            if not (directory_path / name).is_symlink()
            and name not in PRUNED_ATTRIBUTE_DIRECTORIES
        ]
        attributes = directory_path / ".gitattributes"
        if not attributes.exists():
            continue
        attribute_paths.append(attributes.relative_to(root).as_posix())
    for relative in sorted(set(attribute_paths)):
        raw = read_regular(root, relative, required=False)
        if raw is None:
            continue
        assert raw is not None
        for line_number, raw_line in enumerate(
            raw.decode("utf-8", errors="replace").splitlines(), 1
        ):
            line = raw_line.strip()
            if not line or line.startswith("#"):
                continue
            tokens = line.split()[1:]
            if any(
                token == "filter"
                or token.startswith(("filter=", "diff=", "textconv=", "clean=", "smudge=", "process="))
                for token in tokens
            ):
                findings.append(f"{relative}:{line_number}:attribute-driver")
    return sorted(findings)


def validate_git_binary() -> None:
    try:
        metadata = GIT.lstat()
    except OSError as error:
        raise AnalysisUnavailable("git", "GIT_UNAVAILABLE", str(error)) from error
    if GIT.is_symlink() or not stat.S_ISREG(metadata.st_mode):
        raise AnalysisUnavailable("git", "GIT_UNAVAILABLE", "Git binary is not regular")


def validate_git_suffix(suffix: Sequence[str]) -> tuple[str, ...]:
    value = tuple(suffix)
    if value == ("rev-parse", "--show-toplevel"):
        return value
    if (
        len(value) == 4
        and value[:3] == ("rev-parse", "--verify", "--end-of-options")
        and value[3].endswith("^{commit}")
    ):
        validated_revision(value[3][: -len("^{commit}")])
        return value
    if (
        len(value) == 4
        and value[:2] == ("merge-base", "--is-ancestor")
        and HEX40.fullmatch(value[2])
        and HEX40.fullmatch(value[3])
    ):
        return value
    if value == ("status", "--porcelain=v2", "-z", "--untracked-files=all"):
        return value
    prefix = ("diff", "--no-ext-diff", "--no-textconv", "--name-status", "-z")
    if (
        len(value) >= 9
        and value[:5] == prefix
        and HEX40.fullmatch(value[5])
        and HEX40.fullmatch(value[6])
        and value[7] == "--"
    ):
        for path in value[8:]:
            if path != ".":
                normalize_relative(path, label="Git path")
        return value
    raise AnalysisUnavailable(
        "git", "GIT_ARGV_REFUSED", "Git argv is outside the frozen read allowlist"
    )


def stop_process(process: subprocess.Popen[bytes]) -> None:
    process.kill()
    for stream in (process.stdout, process.stderr):
        if stream is not None:
            stream.close()
    try:
        process.wait(timeout=1)
    except subprocess.TimeoutExpired:
        pass


def bounded_communicate(process: subprocess.Popen[bytes]) -> tuple[bytes, bytes]:
    """Capture both pipes without retaining more than the declared bounds."""
    if process.stdout is None or process.stderr is None:
        raise AnalysisUnavailable("git", "GIT_CAPTURE_FAILED", "Git pipes are unavailable")
    selector = selectors.DefaultSelector()
    buffers = {"stdout": bytearray(), "stderr": bytearray()}
    for name, stream in (("stdout", process.stdout), ("stderr", process.stderr)):
        os.set_blocking(stream.fileno(), False)
        selector.register(stream, selectors.EVENT_READ, name)
    deadline = time.monotonic() + GIT_TIMEOUT_SECONDS
    try:
        while selector.get_map():
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                stop_process(process)
                raise AnalysisUnavailable("git", "GIT_TIMEOUT", "bounded Git read timed out")
            events = selector.select(remaining)
            if not events:
                stop_process(process)
                raise AnalysisUnavailable("git", "GIT_TIMEOUT", "bounded Git read timed out")
            for key, _mask in events:
                buffer = buffers[key.data]
                block = os.read(
                    key.fileobj.fileno(),
                    min(65536, MAX_GIT_BYTES - len(buffer) + 1),
                )
                if not block:
                    selector.unregister(key.fileobj)
                    key.fileobj.close()
                    continue
                buffer.extend(block)
                if len(buffer) > MAX_GIT_BYTES:
                    stop_process(process)
                    raise AnalysisUnavailable(
                        "git", "GIT_OUTPUT_LIMIT", "bounded Git output exceeded"
                    )
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            stop_process(process)
            raise AnalysisUnavailable("git", "GIT_TIMEOUT", "bounded Git read timed out")
        process.wait(timeout=remaining)
    except subprocess.TimeoutExpired as error:
        stop_process(process)
        raise AnalysisUnavailable("git", "GIT_TIMEOUT", "bounded Git read timed out") from error
    finally:
        selector.close()
    return bytes(buffers["stdout"]), bytes(buffers["stderr"])


def default_runner(root: Path, suffix: Sequence[str]) -> bytes:
    allowed_suffix = validate_git_suffix(suffix)
    validate_git_binary()
    blocked = prohibited_config(root)
    if blocked:
        raise AnalysisUnavailable(
            "git",
            "PROHIBITED_GIT_CONFIGURATION",
            "Git inspection refused before launch: " + ", ".join(blocked),
        )
    argv = [*GIT_PREFIX, *allowed_suffix]
    process = subprocess.Popen(
        argv,
        cwd=root,
        env=GIT_ENV,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    stdout, stderr = bounded_communicate(process)
    if process.returncode != 0:
        message = stderr.decode("utf-8", errors="replace").strip()
        raise AnalysisUnavailable(
            "git", "GIT_READ_FAILED", message or f"Git exited {process.returncode}"
        )
    return stdout


def validated_revision(value: str) -> str:
    if not REVISION.fullmatch(value) or value.startswith("-"):
        raise InvocationError("base revision has invalid syntax")
    return value


def resolve_commit(root: Path, revision: str, runner: Runner) -> str:
    output = runner(
        root, ("rev-parse", "--verify", "--end-of-options", f"{validated_revision(revision)}^{{commit}}")
    )
    value = output.decode("ascii", errors="strict").strip()
    if not HEX40.fullmatch(value):
        raise AnalysisUnavailable("git", "INVALID_GIT_IDENTITY", "Git returned a non-commit identity")
    return value


def verify_root(root: Path, runner: Runner) -> None:
    output = runner(root, ("rev-parse", "--show-toplevel"))
    try:
        observed = Path(output.decode("utf-8", errors="strict").strip()).resolve(strict=True)
    except (OSError, UnicodeError) as error:
        raise AnalysisUnavailable("repository", "ROOT_IDENTITY_FAILED", str(error)) from error
    if observed != root:
        raise AnalysisUnavailable(
            "repository", "ROOT_IDENTITY_MISMATCH", "Git and filesystem roots differ"
        )


def parse_status(payload: bytes) -> tuple[str, ...]:
    values: list[str] = []
    records = payload.split(b"\0")
    index = 0
    while index < len(records):
        raw = records[index]
        index += 1
        if not raw:
            continue
        text = raw.decode("utf-8", errors="strict")
        if text.startswith("? "):
            values.append(normalize_relative(text[2:], label="Git path"))
        elif text.startswith(("1 ", "u ")):
            split_at = 10 if text.startswith("u ") else 8
            fields = text.split(" ", split_at)
            if len(fields) != split_at + 1:
                raise AnalysisUnavailable(
                    "git", "STATUS_PARSE_FAILED", "malformed porcelain record"
                )
            values.append(normalize_relative(fields[-1], label="Git path"))
        elif text.startswith("2 "):
            fields = text.split(" ", 9)
            if len(fields) != 10 or index >= len(records) or not records[index]:
                raise AnalysisUnavailable(
                    "git", "STATUS_PARSE_FAILED", "truncated rename record"
                )
            values.append(normalize_relative(fields[-1], label="Git path"))
            values.append(
                normalize_relative(
                    records[index].decode("utf-8", errors="strict"), label="Git path"
                )
            )
            index += 1
        elif not text.startswith("# "):
            raise AnalysisUnavailable("git", "STATUS_PARSE_FAILED", "unknown porcelain record")
    return tuple(sorted(set(values)))


def parse_name_status(payload: bytes) -> tuple[str, ...]:
    records = [item for item in payload.split(b"\0") if item]
    values: list[str] = []
    index = 0
    while index < len(records):
        status_code = records[index].decode("ascii", errors="strict")
        index += 1
        path_count = 2 if status_code.startswith(("R", "C")) else 1
        if index + path_count > len(records):
            raise AnalysisUnavailable("git", "DIFF_PARSE_FAILED", "truncated name-status output")
        for _ in range(path_count):
            values.append(
                normalize_relative(
                    records[index].decode("utf-8", errors="strict"), label="Git path"
                )
            )
            index += 1
    return tuple(sorted(set(values)))


def inspect_git(
    root: Path, declaration: PackageDeclaration, mode: str, runner: Runner
) -> GitObservation:
    verify_root(root, runner)
    if not declaration.base:
        raise AnalysisUnavailable(
            "git", "BASE_UNAVAILABLE", "Git inspection requires one valid declared base"
        )
    base = resolve_commit(root, declaration.base, runner)
    head = resolve_commit(root, "HEAD", runner)
    runner(root, ("merge-base", "--is-ancestor", base, head))
    scope = {
        "index": mode in {"working-tree", "terminal"},
        "worktree": mode in {"working-tree", "terminal"},
        "untracked": mode in {"working-tree", "terminal"},
    }
    changed: set[str] = set()
    if mode == "terminal":
        changed.update(
            parse_name_status(
                runner(
                    root,
                    (
                        "diff",
                        "--no-ext-diff",
                        "--no-textconv",
                        "--name-status",
                        "-z",
                        base,
                        head,
                        "--",
                        ".",
                    ),
                )
            )
        )
    if mode in {"working-tree", "terminal"}:
        changed.update(
            parse_status(
                runner(
                    root,
                    ("status", "--porcelain=v2", "-z", "--untracked-files=all"),
                )
            )
        )
    return GitObservation(root, base, head, tuple(sorted(changed)), scope)


def parse_package(root: Path, package_path: str) -> PackageDeclaration:
    relative = normalize_relative(package_path, label="package")
    raw = read_regular(root, relative)
    assert raw is not None
    try:
        text = raw.decode("utf-8", errors="strict")
    except UnicodeError as error:
        raise AnalysisUnavailable("package", "PACKAGE_ENCODING", str(error)) from error
    identifiers: list[str] = []
    bases: list[str] = []
    write_set: list[str] = []
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if line.strip() == "Package ID:" and index + 1 < len(lines):
            match = PACKAGE_ID.search(lines[index + 1])
            if match:
                identifiers.append(match.group(1))
        if line.startswith("Base commit:"):
            match = PACKAGE_ID.search(line)
            candidate = match.group(1) if match else line.partition(":")[2].strip()
            if candidate:
                bases.append(candidate)
        if line.strip() == "## Declared Write Set":
            for item in lines[index + 1 :]:
                if item.startswith("## "):
                    break
                stripped = item.strip()
                if stripped.startswith("- `") and stripped.endswith("`"):
                    candidate = stripped[3:-1]
                    if candidate == "this package subtree":
                        continue
                    write_set.append(candidate)
    issues: list[str] = []
    identifier = identifiers[0] if len(identifiers) == 1 else ""
    if len(identifiers) != 1:
        issues.append("PACKAGE_ID_MISSING" if not identifiers else "PACKAGE_ID_AMBIGUOUS")
    if identifier and identifier != PurePosixPath(relative).parent.name:
        issues.append("PACKAGE_ID_PATH_MISMATCH")
    base = bases[0] if len(bases) == 1 else ""
    if len(bases) != 1:
        issues.append("BASE_MISSING" if not bases else "BASE_AMBIGUOUS")
    elif not REVISION.fullmatch(base) or base.startswith("-"):
        issues.append("BASE_INVALID")
        base = ""
    return PackageDeclaration(
        identifier,
        relative,
        base,
        tuple(write_set),
        "## Implementation Intent" in text or "Implementation intent:" in text,
        tuple(issues),
        text,
    )


def policy_inputs(root: Path) -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    values: list[dict[str, str]] = []
    unavailable: list[dict[str, str]] = []
    for relative in POLICY_PATHS:
        try:
            raw = read_regular(root, relative)
            assert raw is not None
            values.append({"path": relative, "sha256": hashlib.sha256(raw).hexdigest()})
        except AnalysisUnavailable as error:
            unavailable.append(
                {
                    "analysis_id": f"policy:{relative}",
                    "reason_code": error.reason_code,
                    "message": error.message,
                }
            )
    return values, unavailable


def path_matches(path: str, declaration: str, package_path: str) -> bool:
    if declaration == "this package subtree":
        return path.startswith(str(PurePosixPath(package_path).parent) + "/")
    if declaration.endswith("/**"):
        return path.startswith(declaration[:-3].rstrip("/") + "/")
    return path == declaration or path.startswith(declaration.rstrip("/") + "/")


def location(path: str, line: int = 1) -> dict[str, object]:
    return {"path": path, "line": line}


def detached_head(root: Path) -> bool:
    raw = read_regular(root, ".git/HEAD")
    assert raw is not None
    try:
        value = raw.decode("ascii", errors="strict").strip()
    except UnicodeError as error:
        raise AnalysisUnavailable("repository", "HEAD_ENCODING", str(error)) from error
    if value.startswith("ref: refs/"):
        return False
    if HEX40.fullmatch(value):
        return True
    raise AnalysisUnavailable(
        "repository", "HEAD_IDENTITY_FAILED", "HEAD has an unsupported representation"
    )


def finding(
    rule_id: str,
    category: str,
    confidence: str,
    impact: str,
    action: str,
    message: str,
    governing_source: str,
    applicability: str,
    reasoning: str,
    observed_location: dict[str, object] | None = None,
    suggested_command: dict[str, object] | None = None,
) -> dict[str, object]:
    return {
        "rule_id": rule_id,
        "category": category,
        "confidence": confidence,
        "impact": impact,
        "action": action,
        "message": message,
        "observed_location": observed_location,
        "governing_source": governing_source,
        "applicability": applicability,
        "reasoning": reasoning,
        "suggested_command": suggested_command,
    }


def analyze_findings(
    declaration: PackageDeclaration,
    observation: GitObservation | None,
    is_detached: bool | None,
) -> list[dict[str, object]]:
    values: list[dict[str, object]] = []
    issue_messages = {
        "PACKAGE_ID_MISSING": "The package does not declare one package identity.",
        "PACKAGE_ID_AMBIGUOUS": "The package declares multiple package identities.",
        "PACKAGE_ID_PATH_MISMATCH": "The package identity does not match its directory.",
        "BASE_MISSING": "The package does not declare one base revision.",
        "BASE_AMBIGUOUS": "The package declares multiple base revisions.",
        "BASE_INVALID": "The declared base revision has invalid syntax.",
    }
    for issue in declaration.declaration_issues:
        values.append(
            finding(
                f"WP-IDENTITY-{issue}",
                "declaration-conflict",
                "deterministic",
                "high",
                "amend-declaration",
                issue_messages[issue],
                "docs/work-packages/AGENTS.md",
                "work-package identity and terminal boundary",
                "The linter will not infer or choose package or base identity.",
                location(declaration.path),
            )
        )
    if is_detached:
        values.append(
            finding(
                "WP-IDENTITY-DETACHED-HEAD",
                "declaration-conflict",
                "deterministic",
                "medium",
                "inspect",
                "Repository HEAD is detached.",
                "docs/work-packages/gate-planner-advisory-linter-roadmap.md",
                "repository identity",
                "Detached HEAD is reported explicitly and is never interpreted as a lifecycle state.",
                location(".git/HEAD"),
            )
        )
    if not declaration.has_intent:
        values.append(
            finding(
                "WP-INTENT-001",
                "declaration-conflict",
                "deterministic",
                "medium",
                "amend-declaration",
                "The package does not declare implementation intent.",
                "docs/standards/testing-and-gate-strategy.md",
                "all implementation work packages",
                "Gate selection and terminal reconciliation depend on declared intent.",
                location(declaration.path),
            )
        )
    changed = observation.changed_paths if observation else ()
    for index, path in enumerate(changed, 1):
        if not any(
            path_matches(path, item, declaration.path) for item in declaration.write_set
        ) and not path.startswith(str(PurePosixPath(declaration.path).parent) + "/"):
            values.append(
                finding(
                    f"WP-WRITESET-{index:03d}",
                    "scope-mismatch",
                    "deterministic",
                    "high",
                    "amend-declaration",
                    f"Changed path is outside the declared write set: {path}",
                    "docs/work-packages/AGENTS.md",
                    "current observed package diff",
                    "The changed path matches neither a declared entry nor the package subtree.",
                    location(path),
                )
            )
    extensions = {PurePosixPath(path).suffix for path in changed}
    if ".rs" in extensions:
        values.append(
            finding(
                "OBL-RUST-001",
                "relevant-obligation",
                "heuristic",
                "high",
                "consider-command",
                "Rust changes may require focused nextest validation.",
                "docs/standards/testing-and-gate-strategy.md",
                "changed Rust source",
                "The exact test target depends on the owning crate and package intent.",
            )
        )
    if ".md" in extensions:
        values.append(
            finding(
                "OBL-DOC-001",
                "suggested-command",
                "deterministic",
                "low",
                "consider-command",
                "Markdown changes have a canonical documentation lint.",
                "docs/work-packages/AGENTS.md",
                "changed Markdown documentation",
                "Documentation lint is a cheap deterministic check.",
                suggested_command={
                    "argv": ["markdown-doc", "lint", "--path", "."],
                    "working_directory": ".",
                    "affected_surface": "repository Markdown documentation",
                    "governing_citation": "docs/work-packages/AGENTS.md",
                    "cost_class": "quick",
                },
            )
        )
    if ".py" in extensions:
        values.append(
            finding(
                "OBL-PYTHON-001",
                "relevant-obligation",
                "heuristic",
                "medium",
                "inspect",
                "Python changes require their owning focused tests.",
                "AGENTS.md",
                "changed Python tooling",
                "The linter cannot safely infer or execute a project-specific test target.",
            )
        )
    return sorted(
        values,
        key=lambda item: (
            str(item["rule_id"]),
            canonical_json(item["observed_location"]),
        ),
    )


def empty_envelope() -> dict[str, Any]:
    return {
        "schema_version": SCHEMA_VERSION,
        "analysis_status": None,
        "mode": None,
        "repository_root": None,
        "package": None,
        "base_sha": None,
        "head_sha": None,
        "observed_scope": None,
        "policy_inputs": [],
        "unavailable_analyses": [],
        "findings": [],
        "error": None,
    }


def analyze(package_path: str, mode: str, runner: Runner = default_runner) -> dict[str, Any]:
    result = empty_envelope()
    result["mode"] = mode
    root = discover_root(Path.cwd())
    result["repository_root"] = str(root)
    declaration: PackageDeclaration | None = None
    unavailable: list[dict[str, str]] = []
    try:
        declaration = parse_package(root, package_path)
        if declaration.identifier:
            result["package"] = {"id": declaration.identifier, "path": declaration.path}
    except AnalysisUnavailable as error:
        unavailable.append(error.value())
    policies, policy_unavailable = policy_inputs(root)
    result["policy_inputs"] = policies
    unavailable.extend(policy_unavailable)
    observation: GitObservation | None = None
    is_detached: bool | None = None
    try:
        is_detached = detached_head(root)
    except AnalysisUnavailable as error:
        unavailable.append(error.value())
    if declaration is not None:
        try:
            observation = inspect_git(root, declaration, mode, runner)
            result["base_sha"] = observation.base_sha
            result["head_sha"] = observation.head_sha
            result["observed_scope"] = observation.scope
        except AnalysisUnavailable as error:
            unavailable.append(error.value())
    result["findings"] = (
        analyze_findings(declaration, observation, is_detached) if declaration else []
    )
    result["unavailable_analyses"] = sorted(
        unavailable, key=lambda item: (item["analysis_id"], item["reason_code"])
    )
    completed = declaration is not None or bool(policies)
    if unavailable:
        result["analysis_status"] = "partial" if completed else "unavailable"
    else:
        result["analysis_status"] = "complete"
    return result


def human_output(result: dict[str, Any]) -> str:
    lines = [
        "workplan-lint advisory analysis",
        f"analysis_status: {result['analysis_status']}",
        f"mode: {result['mode']}",
        f"repository_root: {result['repository_root']}",
    ]
    package = result["package"]
    lines.append(
        f"package: {package['id']} ({package['path']})" if package else "package: unavailable"
    )
    lines.extend(
        [
            f"base_sha: {result['base_sha']}",
            f"head_sha: {result['head_sha']}",
            f"observed_scope: {canonical_json(result['observed_scope'])}",
            f"policy_inputs: {len(result['policy_inputs'])}",
            f"unavailable_analyses: {len(result['unavailable_analyses'])}",
            f"findings: {len(result['findings'])}",
        ]
    )
    for unavailable in result["unavailable_analyses"]:
        lines.append(
            f"UNAVAILABLE {unavailable['analysis_id']} "
            f"{unavailable['reason_code']}: {unavailable['message']}"
        )
    for item in result["findings"]:
        lines.append(
            f"{item['rule_id']} [{item['impact']}/{item['confidence']}] "
            f"{item['message']}"
        )
        lines.append(f"  source: {item['governing_source']}")
        lines.append(f"  action: {item['action']}")
        command = item["suggested_command"]
        if command:
            lines.append(f"  suggested argv: {canonical_json(command['argv'])}")
    lines.append("This output is advisory; it grants no permission or lifecycle status.")
    return "\n".join(lines) + "\n"


def misuse(message: str, json_requested: bool, mode: str | None = None) -> int:
    if json_requested:
        result = empty_envelope()
        result["mode"] = mode
        result["error"] = {"code": "INVOCATION_MISUSE", "message": message}
        print(canonical_json(result))
    else:
        print(f"workplan-lint: {message}", file=sys.stderr)
    return 2


def parse_arguments(argv: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False, exit_on_error=False)
    parser.add_argument("--package")
    parser.add_argument("--mode", choices=MODES)
    parser.add_argument("--format", choices=FORMATS, default="human")
    parser.add_argument("--help", action="store_true")
    try:
        options, unknown = parser.parse_known_args(argv)
    except argparse.ArgumentError as error:
        raise InvocationError(str(error)) from error
    if options.help:
        raise InvocationError(
            "usage: workplan-lint --package <path> --mode "
            "<pre-edit|working-tree|terminal> [--format human|json]"
        )
    if unknown or options.package is None or options.mode is None:
        raise InvocationError("package and mode are required; unknown arguments are rejected")
    normalize_relative(options.package, label="package")
    return options


def main(argv: Sequence[str] | None = None) -> int:
    arguments = list(sys.argv[1:] if argv is None else argv)
    json_requested = any(
        arguments[index : index + 2] == ["--format", "json"]
        for index in range(len(arguments))
    ) or "--format=json" in arguments
    recognized_mode = next(
        (
            arguments[index + 1]
            for index, value in enumerate(arguments[:-1])
            if value == "--mode" and arguments[index + 1] in MODES
        ),
        None,
    )
    try:
        options = parse_arguments(arguments)
    except InvocationError as error:
        return misuse(str(error), json_requested, recognized_mode)
    try:
        result = analyze(options.package, options.mode)
    except (InvocationError, AnalysisUnavailable) as error:
        if isinstance(error, InvocationError):
            return misuse(str(error), options.format == "json", options.mode)
        result = empty_envelope()
        result["mode"] = options.mode
        result["analysis_status"] = "unavailable"
        result["unavailable_analyses"] = [error.value()]
        result["error"] = {"code": error.reason_code, "message": error.message}
    except Exception:
        result = empty_envelope()
        result["mode"] = options.mode
        result["analysis_status"] = "unavailable"
        unavailable = {
            "analysis_id": "internal",
            "reason_code": "INTERNAL_ERROR",
            "message": "an internal read-only analysis failed; use the manual route",
        }
        result["unavailable_analyses"] = [unavailable]
        result["error"] = {
            "code": unavailable["reason_code"],
            "message": unavailable["message"],
        }
    if options.format == "json":
        print(canonical_json(result))
    else:
        sys.stdout.write(human_output(result))
    return 0 if result["analysis_status"] == "complete" else 3


if __name__ == "__main__":
    sys.exit(main())
