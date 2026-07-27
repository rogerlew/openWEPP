#!/usr/bin/env python3
"""Plan and execute one authoritative TESTGATE increment without a shell."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import stat
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


PACKAGE_PATH_RE = re.compile(r"^docs/work-packages/[^/]+/package\.md$")
DISPOSABLE_ATTEMPT_DIRECTORIES = (
    Path("execution/.work/cargo-target"),
    Path("execution/.work/reconstruction"),
    Path("execution/.work/audit-reconstruction"),
    Path("execution/.work/tmp"),
)


class TestgateError(RuntimeError):
    """Raised when a TESTGATE execution cannot be represented exactly."""


class AttemptFinalizationError(TestgateError):
    """Raised after the single authoritative attempt-finalization pass fails."""


class _PathIdentity:
    __slots__ = ("path", "device", "inode", "mode")

    def __init__(self, path: Path, device: int, inode: int, mode: int) -> None:
        self.path = path
        self.device = device
        self.inode = inode
        self.mode = mode


class _LedgerGuard:
    """Retain no-follow identities and descriptors for one selected ledger."""

    def __init__(
        self,
        path: Path,
        directories: list[tuple[int, _PathIdentity]],
        file_descriptor: int,
        file_identity: _PathIdentity,
    ) -> None:
        self.path = path
        self._directories = directories
        self._file_descriptor = file_descriptor
        self._file_identity = file_identity

    def close(self) -> None:
        for descriptor, _identity in reversed(self._directories):
            os.close(descriptor)
        self._directories.clear()
        if self._file_descriptor >= 0:
            os.close(self._file_descriptor)
            self._file_descriptor = -1

    def validate(self) -> None:
        """Reject substitution of the file or any selected path ancestor."""
        if self._file_descriptor < 0:
            raise TestgateError("history ledger authority is closed")
        for descriptor, identity in self._directories:
            current = os.fstat(descriptor)
            _require_identity(current, identity, "history ledger ancestor")
            try:
                visible = identity.path.lstat()
            except FileNotFoundError as error:
                raise TestgateError(
                    f"history ledger ancestor was replaced: {identity.path}"
                ) from error
            _require_identity(visible, identity, "history ledger ancestor")
        current_file = os.fstat(self._file_descriptor)
        _require_identity(current_file, self._file_identity, "history ledger")
        try:
            visible_file = self.path.lstat()
        except FileNotFoundError as error:
            raise TestgateError(f"history ledger was replaced: {self.path}") from error
        _require_identity(visible_file, self._file_identity, "history ledger")

    def read_bytes(self) -> bytes:
        self.validate()
        os.lseek(self._file_descriptor, 0, os.SEEK_SET)
        chunks: list[bytes] = []
        while True:
            chunk = os.read(self._file_descriptor, 1024 * 1024)
            if not chunk:
                break
            chunks.append(chunk)
        return b"".join(chunks)

    def append(self, payload: bytes) -> None:
        self.validate()
        os.lseek(self._file_descriptor, 0, os.SEEK_END)
        view = memoryview(payload)
        while view:
            written = os.write(self._file_descriptor, view)
            if written <= 0:
                raise TestgateError("history ledger append made no progress")
            view = view[written:]
        os.fsync(self._file_descriptor)


def _identity(path: Path, value: os.stat_result) -> _PathIdentity:
    return _PathIdentity(path, value.st_dev, value.st_ino, value.st_mode)


def _require_identity(
    value: os.stat_result, expected: _PathIdentity, description: str
) -> None:
    if (
        value.st_dev != expected.device
        or value.st_ino != expected.inode
        or stat.S_IFMT(value.st_mode) != stat.S_IFMT(expected.mode)
    ):
        raise TestgateError(f"{description} identity changed: {expected.path}")


def _lexical_absolute_path(raw: Path) -> Path:
    """Make a selected path absolute without resolving any filesystem entry."""
    source = os.fspath(raw)
    if not source:
        raise TestgateError("history ledger path must not be empty")
    selected = raw if raw.is_absolute() else Path.cwd() / raw
    if any(component in {"", ".", ".."} for component in selected.parts[1:]):
        raise TestgateError("history ledger path contains an unsafe component")
    if selected.name in {"", ".", ".."}:
        raise TestgateError("history ledger path must name a file")
    return selected


def _open_ledger_guard(raw: Path, *, create: bool) -> _LedgerGuard:
    """Open a ledger and its full path chain without following symlinks."""
    path = _lexical_absolute_path(raw)
    directory_flags = os.O_RDONLY | os.O_DIRECTORY
    nofollow = getattr(os, "O_NOFOLLOW", 0)
    directory_flags |= nofollow
    descriptors: list[tuple[int, _PathIdentity]] = []
    file_descriptor = -1
    descriptor = os.open(path.anchor, directory_flags)
    root_stat = os.fstat(descriptor)
    descriptors.append((descriptor, _identity(Path(path.anchor), root_stat)))
    current = Path(path.anchor)
    try:
        for component in path.parts[1:-1]:
            current /= component
            try:
                child = os.open(component, directory_flags, dir_fd=descriptor)
            except FileNotFoundError:
                if not create:
                    raise
                try:
                    os.mkdir(component, mode=0o700, dir_fd=descriptor)
                except FileExistsError:
                    pass
                os.fsync(descriptor)
                child = os.open(component, directory_flags, dir_fd=descriptor)
            child_stat = os.fstat(child)
            if not stat.S_ISDIR(child_stat.st_mode):
                os.close(child)
                raise TestgateError(
                    f"history ledger ancestor is not a directory: {current}"
                )
            descriptors.append((child, _identity(current, child_stat)))
            descriptor = child

        flags = os.O_RDWR | nofollow
        created = False
        try:
            selected_status = os.stat(
                path.name, dir_fd=descriptor, follow_symlinks=False
            )
        except FileNotFoundError:
            selected_status = None
        if selected_status is not None and not stat.S_ISREG(selected_status.st_mode):
            raise TestgateError(f"history ledger is not a regular file: {path}")
        if selected_status is None:
            if not create:
                raise FileNotFoundError(path)
            try:
                file_descriptor = os.open(
                    path.name,
                    flags | os.O_CREAT | os.O_EXCL,
                    0o600,
                    dir_fd=descriptor,
                )
                created = True
            except FileExistsError:
                # A competing creator owns the name; never adopt or overwrite it.
                raise TestgateError(
                    f"history ledger exclusive creation collided: {path}"
                ) from None
        else:
            file_descriptor = os.open(path.name, flags, dir_fd=descriptor)
        file_stat = os.fstat(file_descriptor)
        if not stat.S_ISREG(file_stat.st_mode):
            os.close(file_descriptor)
            file_descriptor = -1
            raise TestgateError(f"history ledger is not a regular file: {path}")
        if selected_status is not None:
            try:
                _require_identity(
                    file_stat,
                    _identity(path, selected_status),
                    "history ledger",
                )
            except TestgateError:
                os.close(file_descriptor)
                file_descriptor = -1
                raise
        guard = _LedgerGuard(
            path,
            descriptors,
            file_descriptor,
            _identity(path, file_stat),
        )
        if created:
            os.fsync(file_descriptor)
            os.fsync(descriptor)
        guard.validate()
        _verify_history_bytes(guard.read_bytes())
        file_descriptor = -1
        return guard
    except BaseException as error:
        if file_descriptor >= 0:
            os.close(file_descriptor)
        for opened, _identity_value in reversed(descriptors):
            os.close(opened)
        if isinstance(error, FileNotFoundError):
            raise
        if isinstance(error, OSError):
            raise TestgateError(
                f"history ledger path is unsafe or unavailable: {path}"
            ) from error
        raise


def _canonical_json(value: Any) -> str:
    """Serialize the integer-only RFC 8785 subset shared with the Rust gate."""
    if value is None or isinstance(value, (bool, str)):
        return json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    if isinstance(value, int) and not isinstance(value, bool):
        if abs(value) > 9_007_199_254_740_991:
            raise TestgateError("integer is outside the I-JSON safe range")
        return str(value)
    if isinstance(value, list):
        return "[" + ",".join(_canonical_json(item) for item in value) + "]"
    if isinstance(value, dict):
        if not all(isinstance(key, str) for key in value):
            raise TestgateError("JSON object keys must be strings")
        keys = sorted(value, key=lambda key: key.encode("utf-16-be"))
        return "{" + ",".join(
            f"{_canonical_json(key)}:{_canonical_json(value[key])}" for key in keys
        ) + "}"
    raise TestgateError(f"unsupported canonical JSON value: {type(value).__name__}")


def _strict_json(text: str) -> Any:
    def object_from_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in pairs:
            if key in value:
                raise TestgateError(f"duplicate JSON object key: {key}")
            value[key] = item
        return value

    return json.loads(text, object_pairs_hook=object_from_pairs)


def _atomic_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.tmp")
    temporary.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temporary.replace(path)


def _append_history(
    path: Path, value: dict[str, Any], guard: _LedgerGuard | None = None
) -> str:
    """Append one canonical, predecessor-bound record and return its digest."""
    owned = guard is None
    authority = guard or _open_ledger_guard(path, create=True)
    try:
        authority.validate()
        raw = authority.read_bytes()
        _verify_history_bytes(raw)
        lines = [line for line in raw.decode("utf-8").splitlines() if line]
        previous = _strict_json(lines[-1]).get("entry_sha256") if lines else None
        record = {**value, "previous_entry_sha256": previous}
        canonical = _canonical_json(record)
        entry_sha256 = hashlib.sha256(canonical.encode("utf-8")).hexdigest()
        record["entry_sha256"] = entry_sha256
        authority.append((_canonical_json(record) + "\n").encode("utf-8"))
        return entry_sha256
    finally:
        if owned:
            authority.close()


def _write_attempt_index(root: Path, provenance: dict[str, Any] | None = None) -> None:
    entries = []
    for path in sorted(root.rglob("*")):
        if path.is_symlink():
            raise TestgateError(f"attempt evidence contains symlink: {path}")
        if path.is_file() and path.name != "attempt-index.json":
            entries.append(
                {
                    "path": path.relative_to(root).as_posix(),
                    "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                }
            )
    _atomic_json(
        root / "attempt-index.json",
        {
            "schema_version": "openwepp-testgate-attempt-index-v1",
            "provenance": provenance or _workflow_provenance(),
            "files": entries,
        },
    )


def _prune_disposable_execution_state(artifact_root: Path) -> None:
    """Remove only explicit cache/temp trees that are not evidence artifacts."""
    for relative in DISPOSABLE_ATTEMPT_DIRECTORIES:
        target = artifact_root / relative
        cursor = artifact_root
        missing = False
        for part in relative.parts:
            cursor /= part
            if cursor.is_symlink():
                raise TestgateError(f"disposable execution path is a symlink: {cursor}")
            if not cursor.exists():
                missing = True
                break
            if not cursor.is_dir():
                raise TestgateError(f"disposable execution path is not a directory: {cursor}")
        if not missing:
            shutil.rmtree(target)


def _finalize_attempt_archive(
    ledger: Path,
    artifact_root: Path,
    ledger_guard: _LedgerGuard | None = None,
) -> None:
    """Snapshot, prune, and index once; expose finalizer failures distinctly."""
    try:
        if ledger_guard is not None:
            ledger_guard.validate()
        _snapshot_history(ledger, artifact_root, ledger_guard)
        _prune_disposable_execution_state(artifact_root)
        _write_attempt_index(artifact_root)
    except (OSError, KeyError, ValueError, TestgateError) as error:
        raise AttemptFinalizationError(str(error)) from error


def _snapshot_history(
    ledger: Path,
    artifact_root: Path,
    ledger_guard: _LedgerGuard | None = None,
) -> None:
    """Copy the durable ledger into the indexed upload without changing authority."""
    owned = ledger_guard is None
    try:
        authority = ledger_guard or _open_ledger_guard(ledger, create=False)
    except FileNotFoundError:
        return
    try:
        authority.validate()
        ledger_bytes = authority.read_bytes()
        _verify_history_bytes(ledger_bytes)
    finally:
        if owned:
            authority.close()
    destination = artifact_root / "attempts.jsonl"
    if destination == authority.path:
        return
    temporary = destination.with_name(f".{destination.name}.tmp")
    temporary.write_bytes(ledger_bytes)
    temporary.replace(destination)
    recovery_roots: set[Path] = set()
    configured = os.environ.get("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT")
    current_recovery = Path(configured) if configured else None
    if configured:
        recovery_roots.add(current_recovery)
    for raw in ledger_bytes.decode("utf-8").splitlines():
        if not raw:
            continue
        record = _strict_json(raw)
        named = record.get("recovery_root")
        if isinstance(named, str):
            recovery_roots.add(Path(named))
    allowed = ledger.parent / "recovery"
    verified_indexes: set[str] = set()
    for recovery in sorted(recovery_roots):
        _require_confined_recovery_root(recovery, allowed)
        if recovery != current_recovery and not _has_retained_provenance(
            ledger, recovery, verified_indexes
        ):
            continue
        if recovery.exists() or recovery.is_symlink():
            _validate_directory_nofollow(recovery)
            _copy_regular_tree(recovery, artifact_root / "recovery" / recovery.name)


def _has_retained_provenance(
    ledger: Path, recovery: Path, verified_indexes: set[str]
) -> bool:
    provenance = ledger.parent / "provenance" / recovery.name
    if not provenance.exists() and not provenance.is_symlink():
        return False
    _validate_directory_nofollow(provenance)
    for filename in (
        "attempt-index.json",
        "recovery-predicate.json",
        "recovery-attestation.jsonl",
    ):
        path = provenance / filename
        if path.is_symlink() or not path.is_file():
            raise TestgateError(f"retained provenance is incomplete: {path}")
    index_path = provenance / "attempt-index.json"
    index_bytes = index_path.read_bytes()
    index = _strict_json(index_bytes.decode("utf-8"))
    predicate = _strict_json(
        (provenance / "recovery-predicate.json").read_text(encoding="utf-8")
    )
    index_sha = hashlib.sha256(index_bytes).hexdigest()
    repository = os.environ.get("GITHUB_REPOSITORY")
    workflow = os.environ.get("GITHUB_WORKFLOW")
    source_ref = os.environ.get("GITHUB_REF")
    expected = index.get("provenance", {})
    if (
        predicate.get("schema_version")
        != "openwepp-testgate-recovery-provenance-v1"
        or predicate.get("index_sha256") != index_sha
        or predicate.get("repository") != repository
        or predicate.get("workflow") != workflow
        or predicate.get("source_ref") != source_ref
        or any(predicate.get(key) != expected.get(key) for key in ("repository", "workflow", "run_id", "run_attempt", "head_sha"))
    ):
        raise TestgateError("retained recovery provenance identity mismatch")
    prefix = f"recovery/{recovery.name}/"
    indexed = {
        item["path"][len(prefix) :]: item.get("sha256")
        for item in index.get("files", [])
        if isinstance(item.get("path"), str) and item["path"].startswith(prefix)
    }
    actual: dict[str, str] = {}
    for path in recovery.rglob("*"):
        if path.is_symlink():
            raise TestgateError(f"retained recovery contains symlink: {path}")
        if path.is_file():
            actual[path.relative_to(recovery).as_posix()] = hashlib.sha256(
                path.read_bytes()
            ).hexdigest()
        elif not path.is_dir():
            raise TestgateError(f"retained recovery contains unsafe entry: {path}")
    if not indexed or actual != indexed:
        raise TestgateError("retained recovery differs from its authenticated index")
    if index_sha not in verified_indexes:
        command = subprocess.run(
            [
                "gh", "attestation", "verify", str(index_path),
                "--repo", str(repository),
                "--signer-workflow", f"{repository}/.github/workflows/testgate-shadow.yml",
                "--source-ref", str(source_ref),
                "--source-digest", str(predicate.get("head_sha")),
                "--predicate-type", "https://openwepp.org/attestations/testgate-recovery/v1",
                "--deny-self-hosted-runners",
                "--bundle", str(provenance / "recovery-attestation.jsonl"),
                "--format", "json",
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        try:
            verified = _strict_json(command.stdout)
        except (json.JSONDecodeError, TestgateError) as error:
            raise TestgateError("retained recovery attestation is invalid") from error
        if command.returncode != 0 or not isinstance(verified, list) or not verified:
            raise TestgateError("retained recovery attestation did not verify")
        verified_indexes.add(index_sha)
    return True


def _finalize_recovery(artifact_root: Path) -> str | None:
    """Make an accepted aggregate receipt available beside durable checkpoints."""
    configured = os.environ.get("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT")
    receipt = artifact_root / "receipt.json"
    if not configured or not receipt.is_file():
        return None
    recovery = Path(configured)
    if not recovery.is_absolute():
        raise TestgateError("checkpoint mirror root must be absolute")
    _create_absolute_directories_nofollow(recovery)
    _copy_atomic_nofollow(receipt, recovery / "receipt.json")
    plan = artifact_root / "terminal-plan.json"
    if not plan.is_file() or plan.is_symlink():
        raise TestgateError("accepted recovery is missing a safe terminal plan")
    _copy_atomic_nofollow(plan, recovery / "plan.json")
    return str(recovery)


def _initialize_recovery_plan(plan: Path) -> str | None:
    """Persist the signed plan beside checkpoints before any HEAVY process starts."""
    configured = os.environ.get("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT")
    if not configured:
        return None
    recovery = Path(configured)
    if not recovery.is_absolute():
        raise TestgateError("checkpoint mirror root must be absolute")
    _create_absolute_directories_nofollow(recovery)
    _copy_atomic_nofollow(plan, recovery / "plan.json")
    return str(recovery)


def _create_absolute_directories_nofollow(path: Path) -> None:
    current = Path(path.anchor)
    for component in path.parts[1:]:
        if component in {"", ".", ".."}:
            raise TestgateError(f"unsafe recovery directory component: {component}")
        current /= component
        try:
            mode = current.lstat().st_mode
        except FileNotFoundError:
            current.mkdir()
            mode = current.lstat().st_mode
        if stat.S_ISLNK(mode) or not stat.S_ISDIR(mode):
            raise TestgateError(f"recovery directory is not a real directory: {current}")


def _validate_directory_nofollow(path: Path) -> None:
    if not path.is_absolute():
        raise TestgateError("recovery directory must be absolute")
    current = Path(path.anchor)
    for component in path.parts[1:]:
        if component in {"", ".", ".."}:
            raise TestgateError(f"unsafe recovery directory component: {component}")
        current /= component
        mode = current.lstat().st_mode
        if stat.S_ISLNK(mode) or not stat.S_ISDIR(mode):
            raise TestgateError(f"recovery directory is not a real directory: {current}")


def _require_confined_recovery_root(path: Path, allowed: Path) -> None:
    if not path.is_absolute() or ".." in path.parts:
        raise TestgateError(f"unsafe recovery root: {path}")
    try:
        relative = path.relative_to(allowed)
    except ValueError as error:
        raise TestgateError(f"recovery root is outside durable history: {path}") from error
    if len(relative.parts) != 1 or relative.parts[0] in {"", ".", ".."}:
        raise TestgateError(f"recovery root is not one attempt directory: {path}")


def _copy_atomic_nofollow(source: Path, destination: Path) -> None:
    if source.is_symlink() or not source.is_file():
        raise TestgateError(f"recovery source is missing or unsafe: {source}")
    if destination.is_symlink():
        raise TestgateError(f"recovery destination is a symlink: {destination}")
    temporary = destination.with_name(f".{destination.name}.{os.getpid()}.tmp")
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    descriptor = os.open(temporary, flags, 0o600)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(source.read_bytes())
            stream.flush()
            os.fsync(stream.fileno())
    except BaseException:
        temporary.unlink(missing_ok=True)
        raise
    temporary.replace(destination)


def _copy_regular_tree(source: Path, destination: Path) -> None:
    """Copy an exact regular-file tree without ever following symlinks."""
    for path in sorted(source.rglob("*")):
        if path.is_symlink():
            raise TestgateError(f"history tree contains symlink: {path}")
        relative = path.relative_to(source)
        target = destination / relative
        if path.is_dir():
            target.mkdir(parents=True, exist_ok=True)
        elif path.is_file():
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, target)
        else:
            raise TestgateError(f"history tree contains non-regular entry: {path}")


def _workflow_provenance() -> dict[str, Any]:
    return {
        "repository": os.environ.get("GITHUB_REPOSITORY"),
        "workflow": os.environ.get("GITHUB_WORKFLOW"),
        "run_id": os.environ.get("GITHUB_RUN_ID"),
        "run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
        "head_sha": os.environ.get("GITHUB_SHA"),
    }


def _verify_history_bytes(content: bytes) -> None:
    try:
        text = content.decode("utf-8")
    except UnicodeDecodeError as error:
        raise TestgateError("attempt ledger is not UTF-8") from error
    previous = None
    for raw in text.splitlines():
        if not raw:
            continue
        record = _strict_json(raw)
        if not isinstance(record, dict) or record.get("previous_entry_sha256") != previous:
            raise TestgateError("attempt ledger predecessor mismatch")
        claimed = record.pop("entry_sha256", None)
        actual = hashlib.sha256(_canonical_json(record).encode("utf-8")).hexdigest()
        if claimed != actual:
            raise TestgateError("attempt ledger entry digest mismatch")
        previous = claimed


def _verify_history_chain(path: Path) -> None:
    authority = _open_ledger_guard(path, create=False)
    try:
        _verify_history_bytes(authority.read_bytes())
    finally:
        authority.close()


def _verify_attempt_archive(
    root: Path, *, repository: str, workflow: str, run_id: str, run_attempt: str, head_sha: str
) -> None:
    index_path = root / "attempt-index.json"
    if index_path.is_symlink() or not index_path.is_file():
        raise TestgateError("attempt index is missing or unsafe")
    index = _strict_json(index_path.read_text(encoding="utf-8"))
    if index.get("schema_version") != "openwepp-testgate-attempt-index-v1":
        raise TestgateError("attempt index schema mismatch")
    provenance = index.get("provenance", {})
    expected = {
        "repository": repository,
        "workflow": workflow,
        "run_id": run_id,
        "run_attempt": run_attempt,
        "head_sha": head_sha,
    }
    if any(str(provenance.get(key)) != value for key, value in expected.items()):
        raise TestgateError("attempt archive provenance mismatch")
    entries = index.get("files")
    if not isinstance(entries, list):
        raise TestgateError("attempt index files must be an array")
    indexed: dict[str, str] = {}
    for item in entries:
        relative = Path(item.get("path", ""))
        name = relative.as_posix()
        if relative.is_absolute() or not name or ".." in relative.parts or name in indexed:
            raise TestgateError("invalid or duplicate attempt-index path")
        indexed[name] = item.get("sha256", "")
    actual: set[str] = set()
    for path in root.rglob("*"):
        if path.is_symlink():
            raise TestgateError("attempt archive contains symlink")
        if path == index_path:
            continue
        if path.is_file() and path != index_path:
            actual.add(path.relative_to(root).as_posix())
        elif not path.is_dir():
            raise TestgateError("attempt archive contains non-regular entry")
    if actual != set(indexed) or "attempts.jsonl" not in actual:
        raise TestgateError("attempt archive file set differs from its index")
    for name, expected_digest in indexed.items():
        if hashlib.sha256((root / name).read_bytes()).hexdigest() != expected_digest:
            raise TestgateError("attempt-index digest mismatch")
    _verify_history_chain(root / "attempts.jsonl")


def _restore_attempt_archive(root: Path, history_root: Path) -> None:
    """Install only independently verified regular files into a fresh history root."""
    _validate_directory_nofollow(history_root)
    entries = list(history_root.iterdir())
    if entries:
        placeholder = history_root / "attempts.jsonl"
        if len(entries) != 1 or entries[0] != placeholder:
            raise TestgateError("history restore destination is not empty")
        status = placeholder.lstat()
        if (
            stat.S_ISLNK(status.st_mode)
            or not stat.S_ISREG(status.st_mode)
            or status.st_size != 0
        ):
            raise TestgateError("history restore destination is not empty")
    _copy_atomic_nofollow(root / "attempts.jsonl", history_root / "attempts.jsonl")
    recovery = root / "recovery"
    if recovery.is_dir():
        _copy_regular_tree(recovery, history_root / "recovery")


def _install_recovery_provenance(root: Path, auth_root: Path, history_root: Path) -> None:
    """Retain the hosted-runner attestation needed by the Rust reuse verifier."""
    index = _strict_json((root / "attempt-index.json").read_text(encoding="utf-8"))
    names = {
        item["path"].split("/", 2)[1]
        for item in index["files"]
        if isinstance(item.get("path"), str)
        and item["path"].startswith("recovery/")
        and len(item["path"].split("/", 2)) == 3
    }
    for name in names:
        _install_provenance_for_root(root, auth_root, history_root, name)


def _install_provenance_for_root(
    root: Path, auth_root: Path, history_root: Path, name: str
) -> None:
    if not name or name in {".", ".."} or "/" in name:
        raise TestgateError("attempt index contains an unsafe recovery root")
    destination = history_root / "provenance" / name
    _create_absolute_directories_nofollow(destination)
    _copy_atomic_nofollow(root / "attempt-index.json", destination / "attempt-index.json")
    for filename in ("recovery-predicate.json", "recovery-attestation.jsonl"):
        source = auth_root / filename
        if source.is_symlink() or not source.is_file():
            raise TestgateError(f"authenticated recovery file is missing: {filename}")
        _copy_atomic_nofollow(source, destination / filename)


def _git(repo: Path, arguments: list[str], *, binary: bool = False) -> bytes | str:
    result = subprocess.run(
        ["git", *arguments],
        cwd=repo,
        check=False,
        capture_output=True,
        text=not binary,
    )
    if result.returncode != 0:
        stderr = result.stderr if isinstance(result.stderr, str) else result.stderr.decode()
        raise TestgateError(stderr.strip() or f"git {' '.join(arguments)} failed")
    return result.stdout


def _resolve_commit(repo: Path, revision: str) -> str:
    value = _git(repo, ["rev-parse", "--verify", "--end-of-options", f"{revision}^{{commit}}"])
    if not isinstance(value, str):
        raise TestgateError("Git commit output is not text")
    return value.strip()


def _changed_paths(repo: Path, base: str, head: str) -> list[str]:
    output = _git(
        repo,
        ["diff", "--name-only", "-z", "--no-renames", base, head, "--"],
        binary=True,
    )
    if not isinstance(output, bytes):
        raise TestgateError("Git path output is not bytes")
    try:
        paths = [item.decode("utf-8") for item in output.split(b"\0") if item]
    except UnicodeDecodeError as error:
        raise TestgateError("changed path is not UTF-8") from error
    return sorted(set(paths), key=lambda path: path.encode("utf-8"))


def _dirty_changed_paths(repo: Path, base: str) -> list[str]:
    tracked = _git(repo, ["diff", "--name-only", "-z", base, "--"], binary=True)
    untracked = _git(
        repo,
        ["ls-files", "--others", "--exclude-standard", "-z"],
        binary=True,
    )
    if not isinstance(tracked, bytes) or not isinstance(untracked, bytes):
        raise TestgateError("Git dirty path output is not bytes")
    try:
        paths = [
            item.decode("utf-8")
            for item in (tracked + untracked).split(b"\0")
            if item
        ]
    except UnicodeDecodeError as error:
        raise TestgateError("changed path is not UTF-8") from error
    return sorted(set(paths), key=lambda path: path.encode("utf-8"))


def _intent_authorization(
    repo: Path,
    binary: Path,
    base: str,
    head: str,
    changed_paths: list[str],
    requested_package: str,
    output: Path,
) -> dict[str, Any]:
    if not changed_paths:
        raise TestgateError("zero-work increment cannot be admitted")
    if not PACKAGE_PATH_RE.fullmatch(requested_package):
        raise TestgateError(f"invalid intent package path: {requested_package}")
    result = _invoke(
        [
            str(binary.resolve()),
            "validate-package-chain",
            "--repo",
            str(repo),
            "--base",
            base,
            "--head",
            head,
            "--package",
            requested_package,
            "--output",
            str(output),
        ],
        repo,
        allow_nonpass=True,
    )
    if result.get("result") != "READY":
        raise TestgateError(
            f"package authority chain did not authorize execution: {result.get('reason_codes')}"
        )
    authorization = _strict_json(output.read_text(encoding="utf-8"))
    if not isinstance(authorization, dict):
        raise TestgateError("package authority chain must be an object")
    if (
        authorization.get("status") != "READY"
        or authorization.get("base_commit") != base
        or authorization.get("head_commit") != head
        or authorization.get("intent_package_path") != requested_package
        or authorization.get("changed_paths") != changed_paths
        or authorization.get("package_authority_chain_id")
        != result.get("package_authority_chain_id")
    ):
        raise TestgateError("package authority chain does not bind the observed intent")
    return authorization


def _invoke(
    arguments: list[str], repo: Path, *, allow_nonpass: bool = False
) -> dict[str, Any]:
    result = subprocess.run(arguments, cwd=repo, check=False, capture_output=True, text=True)
    try:
        value = json.loads(result.stdout)
    except json.JSONDecodeError as error:
        if result.returncode != 0:
            raise TestgateError(result.stderr.strip() or result.stdout.strip()) from error
        raise TestgateError("gate CLI emitted invalid JSON") from error
    if not isinstance(value, dict):
        raise TestgateError("gate CLI result must be an object")
    if result.returncode != 0 and not (
        allow_nonpass and value.get("result") in {"FAIL", "BLOCKED", "INVALID"}
    ):
        raise TestgateError(result.stderr.strip() or result.stdout.strip())
    return value


def _final_observation(
    fields: dict[str, Any], authorization: dict[str, Any]
) -> dict[str, Any]:
    """Bind both observation authority views to the retained Rust artifact."""
    return {
        **fields,
        "intent_authorization": authorization,
        "package_audit": authorization,
    }


def observe(args: argparse.Namespace) -> dict[str, Any]:
    repo = args.repo.resolve()
    artifact_root = args.artifact_root.resolve()
    if artifact_root == repo or repo in artifact_root.parents:
        raise TestgateError("artifact root must be outside the repository")
    artifact_root.mkdir(parents=True, exist_ok=True)
    execution_root = artifact_root / "execution"
    execution_root.mkdir(exist_ok=False)
    ledger_guard = getattr(args, "_history_ledger_guard", None)
    ledger = (
        ledger_guard.path
        if isinstance(ledger_guard, _LedgerGuard)
        else _lexical_absolute_path(args.history_ledger)
    )
    if Path("/tmp") in ledger.parents or ledger == Path("/tmp"):
        raise TestgateError("history ledger must not be ephemeral-only")
    base = _resolve_commit(repo, args.base)
    if args.dirty:
        raise TestgateError("sequential package authority requires a committed head")
    head = _resolve_commit(repo, args.head)
    authorized_paths = _changed_paths(repo, base, head)
    authorized_path = artifact_root / "authorized-paths.json"
    intent_path = artifact_root / "intent-plan.json"
    terminal_path = artifact_root / "terminal-plan.json"
    receipt_path = artifact_root / "receipt.json"
    light_receipt_path = artifact_root / "light-receipts.json"
    audit_path = artifact_root / "pre-heavy-audit.json"
    package_audit_path = artifact_root / "package-authority-chain.json"
    authorization = _intent_authorization(
        repo,
        args.binary,
        base,
        head,
        authorized_paths,
        args.intent_package,
        package_audit_path,
    )
    _atomic_json(authorized_path, authorized_paths)
    shutil.copyfile(package_audit_path, artifact_root / "intent-authorization.json")
    _atomic_json(
        artifact_root / "observation.json",
        {
            "schema_version": "openwepp-testgate-execution-v1",
            "enforcement_status": "STARTED",
            "base_commit": base,
            "head_commit": head,
            "execution_requested": args.execute,
            "execution_result": None,
            "execution_error": None,
        },
    )

    common = [
        str(args.binary.resolve()),
        "plan",
        "--repo",
        str(repo),
        "--base",
        base,
        "--boundary",
        args.boundary,
        "--campaign",
        args.campaign,
        "--authorized-paths",
        str(authorized_path),
        "--package-authority-chain",
        str(package_audit_path),
    ]
    if head is not None:
        common.extend(["--head", head])
    started = time.monotonic_ns()
    intent_result = _invoke(
        [*common, "--stage", "intent", "--output", str(intent_path)], repo
    )
    terminal_result = _invoke(
        [
            *common,
            "--stage",
            "terminal",
            "--predecessor",
            str(intent_result["plan_id"]),
            "--output",
            str(terminal_path),
        ],
        repo,
    )
    planner_ms = (time.monotonic_ns() - started) // 1_000_000
    terminal_plan = json.loads(terminal_path.read_text(encoding="utf-8"))
    execution_result: dict[str, Any] | None = None
    execution_error: str | None = None
    execution_ms: int | None = None
    if args.execute:
        if not isinstance(ledger_guard, _LedgerGuard):
            ledger_guard = _open_ledger_guard(ledger, create=True)
            args._history_ledger_guard = ledger_guard
        _initialize_recovery_plan(terminal_path)
        execution_started = time.monotonic_ns()
        try:
            execution_arguments = [
                "--repo", str(repo),
                "--plan", str(terminal_path),
                "--artifact-root", str(execution_root),
                "--principal", args.principal,
                "--repository", args.repository,
                "--source-event", args.source_event,
                "--source-ref", args.source_ref,
                "--workflow", args.workflow,
                "--job", args.job,
                "--runner", args.runner,
                "--attempt", str(args.attempt),
            ]
            has_heavy = any(
                node.get("execution_cost_class") == "HEAVY"
                for node in terminal_plan["nodes"]
            )
            if has_heavy:
                ledger_guard.validate()
                execution_result = _invoke(
                    [
                        str(args.binary.resolve()), "run", *execution_arguments,
                        "--stage", "transition",
                        "--resume", str(ledger),
                        "--light-output", str(light_receipt_path),
                        "--audit-output", str(audit_path),
                        "--output", str(receipt_path),
                    ],
                    repo,
                    allow_nonpass=True,
                )
            else:
                ledger_guard.validate()
                execution_result = _invoke(
                    [
                        str(args.binary.resolve()), "run", *execution_arguments,
                        "--output", str(receipt_path),
                    ],
                    repo,
                    allow_nonpass=True,
                )
        except TestgateError as error:
            execution_error = str(error)
        execution_ms = (time.monotonic_ns() - execution_started) // 1_000_000
        recovery_root = _finalize_recovery(artifact_root)
        _append_history(
            ledger,
            {
                "record_type": "ATTEMPT",
                "status": "CLOSED",
                "plan_id": terminal_result["plan_id"],
                "result": None if execution_result is None else execution_result.get("result"),
                "error": execution_error,
                "artifact_root": str(artifact_root),
                "recovery_root": recovery_root,
                "wall_time_ms": execution_ms,
            },
            ledger_guard,
        )

    observation = _final_observation({
        "schema_version": "openwepp-testgate-execution-v1",
        "enforcement_status": "LOCAL_RECEIPT_ACCEPTED",
        "base_commit": base,
        "head_commit": head,
        "comparison_head": "WORKTREE" if args.dirty else head,
        "boundary": args.boundary,
        "campaign_id": args.campaign,
        "changed_paths": authorized_paths,
        "risk_class": terminal_plan["risk"]["class"],
        "reason_codes": terminal_plan["risk"]["reason_codes"],
        "planned_node_count": len(terminal_plan["nodes"]),
        "planned_inventory_count": len(
            {
                item
                for node in terminal_plan["nodes"]
                for item in node["expected_inventory"]["ids"]
            }
        ),
        "planner_wall_time_ms": planner_ms,
        "intent_plan_id": intent_result["plan_id"],
        "terminal_plan_id": terminal_result["plan_id"],
        "execution_requested": args.execute,
        "execution_result": execution_result,
        "execution_error": execution_error,
        "execution_wall_time_ms": execution_ms,
        "authority_status": "LOCAL_RECEIPT_ACCEPTED",
        "pre_heavy_audit_path": str(audit_path) if audit_path.is_file() else None,
        "history_ledger": str(ledger),
    }, authorization)
    _atomic_json(artifact_root / "observation.json", observation)
    if execution_result is not None and receipt_path.is_file():
        receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
        predicate = {
            "schema_version": "openwepp-testgate-attestation-v1",
            "base_commit": base,
            "head_commit": head,
            "intent_authorization": authorization,
            "receipt_sha256": hashlib.sha256(receipt_path.read_bytes()).hexdigest(),
            "receipt_plan_id": receipt.get("plan_id"),
            "receipt_execution_key": receipt.get("execution_key"),
            "receipt_result": execution_result.get("result"),
            "receipt_trust_class": receipt.get("claims", {}).get("trust_class"),
            "repository": args.repository,
            "source_ref": args.source_ref,
            "workflow": args.workflow,
            "job": args.job,
            "runner": args.runner,
            "runner_image": os.environ.get("OPENWEPP_RUNNER_IMAGE_ID"),
        }
        _atomic_json(artifact_root / "attestation-predicate.json", predicate)
    _finalize_attempt_archive(ledger, artifact_root, ledger_guard)
    return observation


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path(__file__).parents[2])
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--base", required=True)
    parser.add_argument("--head", default="HEAD")
    parser.add_argument(
        "--dirty",
        action="store_true",
        help="Observe the current index/worktree/untracked state instead of a head commit",
    )
    parser.add_argument("--artifact-root", type=Path, required=True)
    parser.add_argument(
        "--history-ledger",
        type=Path,
        default=Path(__file__).parents[2] / "target/local-ci-history/testgate-attempts.jsonl",
    )
    parser.add_argument(
        "--intent-package",
        required=True,
        help="Base-commit work package that prospectively authorizes the changed paths",
    )
    parser.add_argument("--boundary", choices=("INCREMENT", "CHECKPOINT", "CAMPAIGN", "RELEASE"), default="INCREMENT")
    parser.add_argument("--campaign", default="TESTGATE-CI-01")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--principal", default=os.environ.get("GITHUB_ACTOR", "developer"))
    parser.add_argument("--repository", default=os.environ.get("GITHUB_REPOSITORY", "rogerlew/openWEPP"))
    parser.add_argument("--source-event", default=os.environ.get("GITHUB_EVENT_NAME", "local"))
    parser.add_argument("--source-ref", default=os.environ.get("GITHUB_REF", "refs/heads/main"))
    parser.add_argument("--workflow", default=os.environ.get("GITHUB_WORKFLOW", "testgate"))
    parser.add_argument("--job", default="openwepp/increment-gates")
    parser.add_argument("--runner", default=os.environ.get("RUNNER_NAME", "local"))
    parser.add_argument("--attempt", type=int, default=int(os.environ.get("GITHUB_RUN_ATTEMPT", "1")))
    return parser.parse_args()


def main() -> int:
    args = _parse_args()
    ledger_guard: _LedgerGuard | None = None
    try:
        if getattr(args, "execute", False):
            ledger_guard = _open_ledger_guard(args.history_ledger, create=True)
            args._history_ledger_guard = ledger_guard
        observation = observe(args)
    except (OSError, KeyError, ValueError, TestgateError) as error:
        if args.artifact_root.is_dir():
            try:
                _atomic_json(
                    args.artifact_root / "pre-receipt-failure.json",
                    {
                        "schema_version": "openwepp-testgate-pre-receipt-failure-v1",
                        "error": str(error),
                    },
                )
                if not isinstance(error, AttemptFinalizationError):
                    _finalize_attempt_archive(
                        _lexical_absolute_path(args.history_ledger),
                        args.artifact_root,
                        ledger_guard,
                    )
            except (OSError, KeyError, ValueError, TestgateError) as finalization_error:
                print(f"ERROR: {error}", file=sys.stderr)
                print(
                    f"ERROR: attempt finalization failed: {finalization_error}",
                    file=sys.stderr,
                )
                if ledger_guard is not None:
                    ledger_guard.close()
                return 2
        print(f"ERROR: {error}", file=sys.stderr)
        if ledger_guard is not None:
            ledger_guard.close()
        return 2
    if ledger_guard is not None:
        ledger_guard.close()
    print(json.dumps(observation, sort_keys=True))
    execution = observation["execution_result"]
    accepted = not observation["execution_requested"] or (
        isinstance(execution, dict)
        and execution.get("result") in {"PASS", "PASS_WITH_RETRY"}
    )
    return 0 if observation["execution_error"] is None and accepted else 1


if __name__ == "__main__":
    raise SystemExit(main())
