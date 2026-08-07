#!/usr/bin/env python3
"""Build and execute the frozen Stage 3 predecessor endpoint matrix.

This module owns execution custody only. It intentionally performs no energy
reduction or scientific classification; the independent consumer is a separate
module that does not import this file.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
FREEZE_PATH = PACKAGE / "artifacts/protocol-freeze.json"
OUTPUT = REPO / "target/snow_stage3_legacy_predecessor_bridge_reconciliation"
SOURCE_FIXTURE = REPO / (
    "target/snow_stage3_operator_reconciliation_v3/fixtures/snotel_snowbird_ut"
)
CANONICAL_CLIMATE = REPO / "tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli"
DEVELOPMENT_CLIMATE = REPO / (
    "tests/fixtures/snotel_observed/snotel_snowbird_ut/development/"
    "precip_x1p2155576/p8.cli"
)
TRACE_NAME = "snowbird-predecessor-bridge.snow.jsonl"
RUN_STEM = "snowbird-predecessor-bridge"


class CustodyError(RuntimeError):
    """Raised when immutable execution custody does not close."""


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def json_bytes(value: Any) -> bytes:
    return (json.dumps(value, allow_nan=False, indent=2, sort_keys=True) + "\n").encode()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(json_bytes(value))


def run(
    argv: list[str],
    *,
    cwd: Path,
    env: dict[str, str] | None = None,
    check: bool = True,
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        argv,
        cwd=cwd,
        env=env,
        check=False,
        text=True,
        capture_output=True,
    )
    if check and completed.returncode:
        raise CustodyError(
            f"command failed ({completed.returncode}): {' '.join(argv)}\n"
            f"{completed.stderr[-4000:]}"
        )
    return completed


def git_output(argv: list[str], *, cwd: Path = REPO) -> str:
    environment, _ = sanitized_environment()
    environment["GIT_LFS_SKIP_SMUDGE"] = "1"
    return run(["git", *argv], cwd=cwd, env=environment).stdout.strip()


def require_clean_head(expected_head: str) -> None:
    if not re.fullmatch(r"[0-9a-f]{40}", expected_head):
        raise CustodyError("expected HEAD must be a full lowercase Git SHA")
    actual = git_output(["rev-parse", "HEAD"])
    if actual != expected_head:
        raise CustodyError(f"HEAD {actual} differs from admitted {expected_head}")
    if git_output(["status", "--porcelain"]):
        raise CustodyError("execution requires a clean worktree")


def sanitized_environment(
    *, cargo_home: Path | None = None, cargo_target: Path | None = None
) -> tuple[dict[str, str], list[str]]:
    ambient = dict(os.environ)
    allowed_exact = {
        "HOME",
        "LANG",
        "LD_LIBRARY_PATH",
        "LOGNAME",
        "PATH",
        "RUSTUP_HOME",
        "SHELL",
        "TEMP",
        "TERM",
        "TMP",
        "TMPDIR",
        "TZ",
        "USER",
    }
    environment = {
        key: value
        for key, value in ambient.items()
        if key in allowed_exact or key.startswith("LC_")
    }
    removed = sorted(set(ambient) - set(environment))
    environment["LC_ALL"] = "C"
    if cargo_home is not None:
        environment["CARGO_HOME"] = str(cargo_home.resolve())
        environment["CARGO_NET_OFFLINE"] = "true"
    if cargo_target is not None:
        environment["CARGO_TARGET_DIR"] = str(cargo_target.resolve())
    return environment, removed


def cargo_seed_source() -> Path:
    configured = os.environ.get("CARGO_HOME")
    return Path(configured).resolve() if configured else Path.home() / ".cargo"


def seed_cargo_home(target: Path) -> dict[str, Any]:
    if target.exists():
        raise CustodyError(f"refusing to overwrite Cargo home {target}")
    source = cargo_seed_source()
    target.mkdir(parents=True)
    copied_roots: list[str] = []
    for relative in (
        Path("registry/cache"),
        Path("registry/index"),
        Path("registry/src"),
        Path("git/checkouts"),
        Path("git/db"),
    ):
        origin = source / relative
        if not origin.exists():
            continue
        destination = target / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(origin, destination, copy_function=shutil.copy2)
        copied_roots.append(relative.as_posix())
    if not copied_roots:
        raise CustodyError(f"no non-credential Cargo cache found under {source}")
    forbidden = {
        "credentials",
        "credentials.toml",
        "config",
        "config.toml",
        "env",
    }
    if any((target / name).exists() for name in forbidden):
        raise CustodyError("Cargo seed copied forbidden credential/config material")
    manifest = file_manifest(target)
    manifest["source"] = str(source)
    manifest["copied_roots"] = copied_roots
    return manifest


def file_manifest(root: Path, *, exclude: frozenset[str] = frozenset()) -> dict[str, Any]:
    files = []
    for path in sorted(candidate for candidate in root.rglob("*") if candidate.is_file()):
        relative = path.relative_to(root).as_posix()
        if relative in exclude:
            continue
        files.append(
            {
                "path": relative,
                "sha256": sha256(path),
                "size_bytes": path.stat().st_size,
            }
        )
    payload = json.dumps(files, sort_keys=True, separators=(",", ":")).encode()
    return {
        "file_count": len(files),
        "files": files,
        "manifest_sha256": hashlib.sha256(payload).hexdigest(),
    }


def clone_source(source_sha: str, destination: Path) -> dict[str, Any]:
    if destination.exists():
        raise CustodyError(f"refusing to overwrite clone {destination}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    environment, _ = sanitized_environment()
    environment["GIT_LFS_SKIP_SMUDGE"] = "1"
    run(
        ["git", "clone", "--shared", "--no-checkout", str(REPO), str(destination)],
        cwd=REPO,
        env=environment,
    )
    run(["git", "checkout", "--detach", source_sha], cwd=destination, env=environment)
    actual = git_output(["rev-parse", "HEAD"], cwd=destination)
    status = git_output(["status", "--porcelain"], cwd=destination)
    if actual != source_sha or status:
        raise CustodyError(f"detached clone identity failed for {source_sha}")
    return {"source_sha": actual, "clean": True, "path": str(destination)}


def require_clone_identity(clone: Path, source_sha: str, phase: str) -> None:
    actual = git_output(["rev-parse", "HEAD"], cwd=clone)
    status = git_output(["status", "--porcelain"], cwd=clone)
    if actual != source_sha or status:
        raise CustodyError(f"clone identity changed during {phase}: {clone}")


def build_input_digest(source_sha: str) -> str:
    environment, _ = sanitized_environment()
    environment["GIT_LFS_SKIP_SMUDGE"] = "1"
    tree = run(
        [
            "git",
            "ls-tree",
            "-r",
            source_sha,
            "--",
            "Cargo.toml",
            "Cargo.lock",
            "rust-toolchain.toml",
            ".cargo",
            "crates",
        ],
        cwd=REPO,
        env=environment,
    ).stdout.encode()
    return hashlib.sha256(tree).hexdigest()


def toolchain_receipt(environment: dict[str, str]) -> dict[str, str]:
    return {
        "cargo": run(["cargo", "-V"], cwd=REPO, env=environment).stdout.strip(),
        "rustc_vv": run(["rustc", "-Vv"], cwd=REPO, env=environment).stdout.strip(),
        "os": run(["uname", "-a"], cwd=REPO, env=environment).stdout.strip(),
        "linker": run(["cc", "--version"], cwd=REPO, env=environment).stdout.splitlines()[0],
    }


def build_source(
    source_sha: str,
    clone: Path,
    cargo_home: Path,
    cargo_target: Path,
    *,
    artifact_group: str = "builds",
    binary_group: str = "binaries",
) -> dict[str, Any]:
    environment, removed = sanitized_environment(
        cargo_home=cargo_home, cargo_target=cargo_target
    )
    rustc_vv = run(["rustc", "-Vv"], cwd=clone, env=environment).stdout
    host_match = re.search(r"^host: ([A-Za-z0-9_.-]+)$", rustc_vv, re.MULTILINE)
    if host_match is None:
        raise CustodyError("rustc did not report a host target")
    target_triple = host_match.group(1)
    linker_raw = shutil.which("cc", path=environment.get("PATH"))
    if linker_raw is None:
        raise CustodyError("required explicit C linker 'cc' is unavailable")
    linker = Path(linker_raw).resolve()
    linker_key = f"CARGO_TARGET_{target_triple.upper().replace('-', '_')}_LINKER"
    environment[linker_key] = str(linker)
    argv = [
        "cargo",
        "build",
        "--locked",
        "--offline",
        "--release",
        "--target",
        target_triple,
        "-p",
        "openwepp-runner",
        "--bin",
        "openwepp-cli-hill",
    ]
    started = time.perf_counter()
    completed = run(argv, cwd=clone, env=environment, check=False)
    elapsed = time.perf_counter() - started
    log_root = OUTPUT / artifact_group / source_sha
    log_root.mkdir(parents=True, exist_ok=True)
    (log_root / "stdout.txt").write_text(completed.stdout, encoding="utf-8")
    (log_root / "stderr.txt").write_text(completed.stderr, encoding="utf-8")
    binary = cargo_target / target_triple / "release/openwepp-cli-hill"
    receipt = {
        "source_sha": source_sha,
        "argv": argv,
        "cwd": str(clone),
        "returncode": completed.returncode,
        "elapsed_seconds": elapsed,
        "stdout_sha256": sha256(log_root / "stdout.txt"),
        "stderr_sha256": sha256(log_root / "stderr.txt"),
        "removed_environment_keys": removed,
        "effective_cargo_home": str(cargo_home.resolve()),
        "effective_cargo_target_dir": str(cargo_target.resolve()),
        "effective_target_triple": target_triple,
        "effective_linker": {
            "path": str(linker),
            "sha256": sha256(linker),
            "version": run([str(linker), "--version"], cwd=clone, env=environment)
            .stdout.splitlines()[0],
        },
        "cargo_lock_sha256": sha256(clone / "Cargo.lock"),
        "build_input_digest": build_input_digest(source_sha),
        "toolchain": toolchain_receipt(environment),
    }
    require_clone_identity(clone, source_sha, "build")
    if completed.returncode or not binary.is_file():
        write_json(log_root / "build-receipt.json", receipt)
        raise CustodyError(f"release build failed for {source_sha}")
    retained = OUTPUT / binary_group / source_sha / "openwepp-cli-hill"
    retained.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(binary, retained)
    receipt["binary"] = {
        "path": str(retained.relative_to(OUTPUT)),
        "sha256": sha256(retained),
        "size_bytes": retained.stat().st_size,
    }
    receipt["cargo_target_manifest"] = file_manifest(cargo_target)
    write_json(log_root / "build-receipt.json", receipt)
    return receipt


def prepare_fixture(forcing: str, frozen: dict[str, Any]) -> tuple[Path, dict[str, Any]]:
    if not SOURCE_FIXTURE.is_dir():
        raise CustodyError(f"missing retained source fixture {SOURCE_FIXTURE}")
    target = OUTPUT / "fixtures" / forcing
    if target.exists():
        raise CustodyError(f"refusing to overwrite fixture {target}")
    target.mkdir(parents=True)
    common = frozen["common_fixture_sha256"]
    for name, expected in common.items():
        source = SOURCE_FIXTURE / name
        if sha256(source) != expected:
            raise CustodyError(f"source fixture hash mismatch: {name}")
        shutil.copy2(source, target / name)
    climate = CANONICAL_CLIMATE if forcing == "canonical" else DEVELOPMENT_CLIMATE
    expected_climate = frozen["forcings"][forcing]["sha256"]
    if sha256(climate) != expected_climate:
        raise CustodyError(f"{forcing} climate hash mismatch")
    shutil.copy2(climate, target / "p8.cli")
    return target, file_manifest(target)


def render_runfile(fixture: Path, run_dir: Path) -> str:
    fields = {
        "soil": fixture / "p8.sol",
        "management": fixture / "p8.man",
        "slope": fixture / "p8.slp",
        "climate": fixture / "p8.cli",
        "pass": run_dir / f"{RUN_STEM}.hbp",
        "loss": run_dir / f"{RUN_STEM}.loss.json",
        "wat": run_dir / f"{RUN_STEM}.wat.parquet",
    }
    return "\n".join(
        [
            'schema = "openwepp-hillslope-runfile-v1"',
            f'run_name = "{RUN_STEM}"',
            'unit_system = "metric"',
            "",
            "[inputs]",
            *(f'{name} = "{path.resolve()}"' for name, path in list(fields.items())[:4]),
            "wepp_ui = false",
            "",
            "[outputs]",
            *(f'{name} = "{path.resolve()}"' for name, path in list(fields.items())[4:]),
            "",
        ]
    )


def science_selectors(frozen: dict[str, Any]) -> dict[str, str]:
    selectors = frozen["selectors"]
    return {
        key: value
        for key, value in selectors.items()
        if key.startswith("OPENWEPP_") and key not in {
            "OPENWEPP_SNOW_STAGE3_COMPLETE_CARRIER_SHADOW",
            "OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR",
        }
    }


def normalized_semantic_input_manifest(
    *,
    source_sha: str,
    forcing: str,
    fixture: Path,
    runfile: Path,
    effective: dict[str, str],
    frozen: dict[str, Any],
) -> dict[str, Any]:
    """Describe the complete semantic input surface independently of outputs."""
    inputs = {
        path.name: {"sha256": sha256(path), "size_bytes": path.stat().st_size}
        for path in sorted(candidate for candidate in fixture.iterdir() if candidate.is_file())
    }
    normalized_selectors = dict(sorted(effective.items()))
    if "OPENWEPP_R7H_SNOW_TRACE_PATH" in normalized_selectors:
        normalized_selectors["OPENWEPP_R7H_SNOW_TRACE_PATH"] = f"<{TRACE_NAME}>"
    return {
        "source_sha": source_sha,
        "forcing": forcing,
        "forcing_sha256": frozen["forcings"][forcing]["sha256"],
        "protocol_sha256": sha256(FREEZE_PATH),
        "fixture_inputs": inputs,
        "runfile_semantics": {
            "schema": "openwepp-hillslope-runfile-v1",
            "run_name": RUN_STEM,
            "unit_system": "metric",
            "wepp_ui": False,
            "input_bindings": ["soil", "management", "slope", "climate"],
            "output_bindings": ["pass", "loss", "wat"],
            "raw_sha256": sha256(runfile),
        },
        "science_selectors": normalized_selectors,
        "scheduler": "daily",
        "snow_evaluation_forcing_cadence": "hourly",
        "executor": "direct-production-executor",
        "date_count": frozen["forcings"]["date_count"],
        "first_date": frozen["forcings"]["first_date"],
        "last_date": frozen["forcings"]["last_date"],
        "windows_sha256": hashlib.sha256(json_bytes(frozen["windows"])).hexdigest(),
    }


def forcing_matched_semantic_checks(cells: dict[str, dict[str, Any]]) -> dict[str, Any]:
    """Prove old/current cells differ only by source and output-path runfile bytes."""
    checks = {}
    for forcing, old_cell, current_cell in (
        ("canonical", "E00", "E10"),
        ("development", "E01", "E11"),
    ):
        checks[forcing] = {}
        for mode in ("control", "legacy"):
            old = json.loads(json.dumps(cells[old_cell][mode]["normalized_semantic_inputs"]))
            current = json.loads(
                json.dumps(cells[current_cell][mode]["normalized_semantic_inputs"])
            )
            old.pop("source_sha")
            current.pop("source_sha")
            old["runfile_semantics"].pop("raw_sha256")
            current["runfile_semantics"].pop("raw_sha256")
            passed = old == current
            checks[forcing][mode] = passed
            if not passed:
                raise CustodyError(f"forcing-matched semantic inputs differ for {forcing}/{mode}")
    return checks


def normalized_operator_semantics(manifest: dict[str, Any]) -> dict[str, Any]:
    normalized = json.loads(json.dumps(manifest))
    normalized.pop("source_sha")
    normalized["runfile_semantics"].pop("raw_sha256")
    selectors = normalized["science_selectors"]
    legacy = selectors.pop("OPENWEPP_SNOW_STAGE3_COMPLETE_CARRIER_SHADOW", None)
    explicit = selectors.pop("OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR", None)
    if (legacy, explicit) not in {
        ("enabled", None),
        (None, "sequential_resolved_shadow_v1"),
    }:
        raise CustodyError("operator activation does not select the frozen sequential operator")
    normalized["effective_evaluation_operator"] = "sequential_resolved_shadow_v1"
    return normalized


def current_selector_semantic_checks(cells: dict[str, dict[str, Any]]) -> dict[str, bool]:
    checks = {}
    for cell in ("E10", "E11"):
        legacy = normalized_operator_semantics(
            cells[cell]["legacy"]["normalized_semantic_inputs"]
        )
        explicit = normalized_operator_semantics(
            cells[cell]["explicit"]["normalized_semantic_inputs"]
        )
        passed = legacy == explicit
        checks[cell] = passed
        if not passed:
            raise CustodyError(f"legacy/explicit semantic inputs differ for {cell}")
    return checks


def require_path_checksum(
    checksums: dict[str, str], expected_path: Path, expected_sha: str, label: str
) -> None:
    resolved = str(expected_path.resolve())
    matches = [value for key, value in checksums.items() if str(Path(key).resolve()) == resolved]
    if matches != [expected_sha]:
        raise CustodyError(f"{label} checksum binding differs for {resolved}")


def validate_run_manifest(
    *,
    run_dir: Path,
    fixture: Path,
    runfile: Path,
    arm_binary: Path,
    binary_sha: str,
    source_sha: str,
    argv: list[str],
    frozen: dict[str, Any],
) -> dict[str, Any]:
    """Fail closed on runtime publication and executable identity custody."""
    manifest_path = run_dir / "openwepp_hillslope_run_manifest.json"
    if not manifest_path.is_file():
        raise CustodyError(f"missing runtime manifest under {run_dir}")
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    exact = {
        "schema": "openwepp-hillslope-run-manifest-v1",
        "engine": "openwepp",
        "source_commit": source_sha,
        "binary_path": str(arm_binary.resolve()),
        "binary_sha256": binary_sha,
        "argv": argv,
        "run_dir": str(fixture.resolve()),
        "run_file": str(runfile.resolve()),
    }
    for key, expected in exact.items():
        if manifest.get(key) != expected:
            raise CustodyError(f"runtime manifest {key} differs under {run_dir}")
    provenance = manifest.get("execution_provenance", {})
    expected_days = frozen["forcings"]["date_count"]
    for key in ("climate_day_count", "executed_day_count"):
        if provenance.get(key) != expected_days:
            raise CustodyError(f"runtime manifest {key} differs under {run_dir}")
    if manifest.get("runtime_selection", {}).get("selected") != "direct-production-executor":
        raise CustodyError(f"runtime executor differs under {run_dir}")
    timestep = manifest.get("timestep_policy", {})
    if timestep.get("scheduler_mode") != "daily" or timestep.get("timestep_seconds") != 86_400:
        raise CustodyError(f"runtime timestep policy differs under {run_dir}")

    input_checksums = manifest.get("input_checksums")
    if not isinstance(input_checksums, dict):
        raise CustodyError(f"runtime input checksums absent under {run_dir}")
    expected_inputs = [runfile, *(path for path in fixture.iterdir() if path.is_file())]
    if len(input_checksums) != len(expected_inputs):
        raise CustodyError(f"runtime input checksum cardinality differs under {run_dir}")
    for path in expected_inputs:
        require_path_checksum(input_checksums, path, sha256(path), "runtime input")

    output_checksums = manifest.get("output_checksums")
    if not isinstance(output_checksums, dict) or not output_checksums:
        raise CustodyError(f"runtime output checksums absent under {run_dir}")
    for raw_path, expected_sha in output_checksums.items():
        path = Path(raw_path).resolve()
        if path.parent != run_dir.resolve() or not path.is_file() or sha256(path) != expected_sha:
            raise CustodyError(f"runtime output checksum differs or escapes run dir: {raw_path}")
    for suffix in (".hbp", ".loss.json", ".wat.parquet"):
        path = run_dir / f"{RUN_STEM}{suffix}"
        require_path_checksum(output_checksums, path, sha256(path), "runtime output")

    sidecar_path = Path(manifest.get("binary_sidecar_path", ""))
    if not sidecar_path.is_absolute():
        sidecar_path = (run_dir / sidecar_path).resolve()
    expected_sidecar = arm_binary.with_suffix(arm_binary.suffix + ".json").resolve()
    if sidecar_path != expected_sidecar:
        raise CustodyError(f"binary sidecar path differs under {run_dir}")
    if not sidecar_path.is_file() or sha256(sidecar_path) != manifest.get("binary_sidecar_sha256"):
        raise CustodyError(f"binary sidecar checksum differs under {run_dir}")
    sidecar = json.loads(sidecar_path.read_text(encoding="utf-8"))
    sidecar_exact = {
        "schema": "openwepp-binary-release-metadata-v1",
        "binary_name": "openwepp-cli-hill",
        "binary_role": "hillslope",
        "source_commit": source_sha,
        "sha256": binary_sha,
    }
    for key, expected in sidecar_exact.items():
        if sidecar.get(key) != expected:
            raise CustodyError(f"binary sidecar {key} differs under {run_dir}")
    return {
        "path": str(manifest_path.relative_to(OUTPUT)),
        "sha256": sha256(manifest_path),
        "binary_sidecar_path": str(sidecar_path.relative_to(OUTPUT)),
        "binary_sidecar_sha256": sha256(sidecar_path),
        "input_checksums": input_checksums,
        "output_checksums": output_checksums,
    }


def run_arm(
    *,
    cell: str,
    mode: str,
    forcing: str,
    source_sha: str,
    clone: Path,
    binary: Path,
    fixture: Path,
    frozen: dict[str, Any],
    destination: Path | None = None,
) -> dict[str, Any]:
    run_dir = destination if destination is not None else OUTPUT / "runs" / cell / mode
    if run_dir.exists():
        raise CustodyError(f"refusing to overwrite run {cell}/{mode}")
    run_dir.mkdir(parents=True)
    arm_binary = run_dir / "bin/openwepp-cli-hill"
    arm_binary.parent.mkdir()
    shutil.copy2(binary, arm_binary)
    binary_before = sha256(arm_binary)
    runfile = run_dir / f"{RUN_STEM}.run"
    runfile.write_text(render_runfile(fixture, run_dir), encoding="utf-8")
    trace = run_dir / TRACE_NAME
    environment, removed = sanitized_environment()
    effective = science_selectors(frozen)
    if mode == "legacy":
        effective["OPENWEPP_SNOW_STAGE3_COMPLETE_CARRIER_SHADOW"] = "enabled"
    elif mode == "explicit":
        effective["OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR"] = (
            "sequential_resolved_shadow_v1"
        )
    elif mode != "control":
        raise CustodyError(f"unknown arm mode {mode}")
    if mode != "control":
        effective["OPENWEPP_R7H_SNOW_TRACE_PATH"] = str(trace.resolve())
    environment.update(effective)
    argv = [
        str(arm_binary.resolve()),
        "--run-dir",
        str(fixture.resolve()),
        "--run-file",
        str(runfile.resolve()),
        "--output-dir",
        str(run_dir.resolve()),
        "--legacy-sidecar-discovery",
        "--direct-production-executor",
    ]
    started = time.perf_counter()
    completed = run(argv, cwd=clone, env=environment, check=False)
    elapsed = time.perf_counter() - started
    stdout_path = run_dir / "stdout.txt"
    stderr_path = run_dir / "stderr.txt"
    stdout_path.write_text(completed.stdout, encoding="utf-8")
    stderr_path.write_text(completed.stderr, encoding="utf-8")
    binary_after = sha256(arm_binary)
    require_clone_identity(clone, source_sha, f"run {cell}/{mode}")
    if binary_after != binary_before or binary_after != sha256(binary):
        raise CustodyError(f"runtime binary changed for {cell}/{mode}")
    runtime_manifest = None
    if completed.returncode == 0:
        runtime_manifest = validate_run_manifest(
            run_dir=run_dir,
            fixture=fixture,
            runfile=runfile,
            arm_binary=arm_binary,
            binary_sha=binary_after,
            source_sha=source_sha,
            argv=argv,
            frozen=frozen,
        )
    semantic_inputs = normalized_semantic_input_manifest(
        source_sha=source_sha,
        forcing=forcing,
        fixture=fixture,
        runfile=runfile,
        effective=effective,
        frozen=frozen,
    )
    files = file_manifest(run_dir)
    receipt = {
        "cell": cell,
        "mode": mode,
        "source_sha": source_sha,
        "forcing": forcing,
        "argv": argv,
        "cwd": str(clone.resolve()),
        "returncode": completed.returncode,
        "elapsed_seconds": elapsed,
        "removed_environment_keys": removed,
        "effective_openwepp_environment": dict(sorted(effective.items())),
        "runfile_sha256": sha256(runfile),
        "binary_sha256_before": binary_before,
        "binary_sha256_after": binary_after,
        "runtime_manifest": runtime_manifest,
        "normalized_semantic_inputs": semantic_inputs,
        "outputs": files,
    }
    write_json(run_dir / "arm-receipt.json", receipt)
    if completed.returncode:
        raise CustodyError(f"run failed for {cell}/{mode}")
    if mode == "control" and trace.exists():
        raise CustodyError(f"disabled control emitted a trace: {cell}")
    if mode != "control" and (not trace.is_file() or trace.stat().st_size == 0):
        raise CustodyError(f"enabled arm did not emit a trace: {cell}/{mode}")
    return receipt


def output_hash(run_dir: Path, suffix: str) -> str:
    matches = [path for path in run_dir.iterdir() if path.name.endswith(suffix)]
    if len(matches) != 1:
        raise CustodyError(f"expected one {suffix} under {run_dir}")
    return sha256(matches[0])


def require_arm_identity(
    arm: dict[str, Any],
    *,
    cell: str,
    mode: str,
    source_sha: str,
    forcing: str,
    context: str,
) -> None:
    if (
        arm.get("cell") != cell
        or arm.get("mode") != mode
        or arm.get("source_sha") != source_sha
        or arm.get("forcing") != forcing
        or arm.get("returncode") != 0
    ):
        raise CustodyError(f"{context} arm identity differs: {cell}/{mode}")


def require_protected_output_receipt(value: Any, *, context: str) -> None:
    if (
        not isinstance(value, dict)
        or set(value) != {".hbp", ".wat.parquet", ".loss.json"}
        or set(value.values()) != {True}
    ):
        raise CustodyError(f"{context} protected output receipt differs")


def require_binary_match(
    path: Path,
    expected_sha256: str,
    *,
    context: str,
    expected_size_bytes: int | None = None,
) -> None:
    if (
        not path.is_file()
        or sha256(path) != expected_sha256
        or (
            expected_size_bytes is not None
            and path.stat().st_size != expected_size_bytes
        )
    ):
        raise CustodyError(f"{context} binary differs")


def require_semantic_input_receipt(
    actual: Any, expected: dict[str, Any], *, context: str
) -> None:
    if actual != expected:
        raise CustodyError(f"{context} semantic input receipt differs")


def protected_output_checks(cells: dict[str, dict[str, Any]]) -> dict[str, Any]:
    checks: dict[str, Any] = {}
    for cell, modes in cells.items():
        control = OUTPUT / "runs" / cell / "control"
        checks[cell] = {}
        for mode in sorted(name for name in modes if name != "control"):
            enabled = OUTPUT / "runs" / cell / mode
            comparisons = {
                suffix: output_hash(control, suffix) == output_hash(enabled, suffix)
                for suffix in (".hbp", ".wat.parquet", ".loss.json")
            }
            if not all(comparisons.values()):
                raise CustodyError(f"protected output differs for {cell}/{mode}")
            checks[cell][mode] = comparisons
    return checks


def retained_anchor_checks(frozen: dict[str, Any]) -> dict[str, Any]:
    checks = {}
    for name in ("historical_trace", "current_trace"):
        item = frozen["retained_custody"][name]
        path = REPO / item["path"]
        checks[name] = {
            "path": item["path"],
            "sha256": sha256(path),
            "size_bytes": path.stat().st_size,
        }
        if checks[name]["sha256"] != item["sha256"] or checks[name]["size_bytes"] != item["size_bytes"]:
            raise CustodyError(f"retained {name} differs")
    return checks


def endpoint_cells(frozen: dict[str, Any]) -> dict[str, list[str]]:
    values = frozen.get("endpoint_matrix")
    expected = {
        "E00": ["old", "canonical"],
        "E01": ["old", "development"],
        "E10": ["current", "canonical"],
        "E11": ["current", "development"],
    }
    if not isinstance(values, dict):
        raise CustodyError("endpoint matrix is absent")
    actual = {key: values.get(key) for key in expected}
    if actual != expected:
        raise CustodyError("endpoint matrix differs from the frozen 2x2 design")
    return expected


def checkpoint_trigger_lanes(result: dict[str, Any]) -> list[str]:
    gates = result.get("source_gates")
    if not isinstance(gates, dict) or set(gates) != {"canonical", "development"}:
        raise CustodyError("result source-gate inventory differs")
    derived = []
    for forcing in ("canonical", "development"):
        gate = gates[forcing]
        if not isinstance(gate, dict) or not isinstance(gate.get("checkpoint_trigger"), bool):
            raise CustodyError(f"result checkpoint predicate differs for {forcing}")
        if gate["checkpoint_trigger"]:
            derived.append(forcing)
    if result.get("checkpoint_lanes_triggered") != derived:
        raise CustodyError("result checkpoint lane list differs from source gates")
    return derived


def frozen_checkpoints(frozen: dict[str, Any]) -> list[tuple[str, str]]:
    values = frozen.get("checkpoint_grouping", {}).get("checkpoints")
    if not isinstance(values, list) or len(values) != 14:
        raise CustodyError("frozen checkpoint inventory differs")
    checkpoints = []
    for value in values:
        if (
            not isinstance(value, list)
            or len(value) != 2
            or not re.fullmatch(r"[0-9a-f]{40}", value[0])
            or not re.fullmatch(r"[0-9a-f]{64}", value[1])
        ):
            raise CustodyError("malformed frozen checkpoint")
        if build_input_digest(value[0]) != value[1]:
            raise CustodyError(f"frozen checkpoint digest differs at {value[0]}")
        checkpoints.append((value[0], value[1]))
    return checkpoints


def execute_checkpoints(expected_head: str) -> None:
    require_clean_head(expected_head)
    receipt_path = OUTPUT / "execution-receipt.json"
    result_path = OUTPUT / "results/predecessor-bridge-results.json"
    consumer_trigger_path = OUTPUT / "results/checkpoint-trigger-receipt.json"
    destination = OUTPUT / "checkpoint-search"
    if destination.exists():
        raise CustodyError(f"refusing to overwrite {destination}")
    if not receipt_path.is_file() or not result_path.is_file() or not consumer_trigger_path.is_file():
        raise CustodyError("endpoint execution and reconstruction are required first")
    execution = json.loads(receipt_path.read_text(encoding="utf-8"))
    result = json.loads(result_path.read_text(encoding="utf-8"))
    consumer_trigger = json.loads(consumer_trigger_path.read_text(encoding="utf-8"))
    if execution.get("execution_head") != expected_head or result.get("execution_head") != expected_head:
        raise CustodyError("checkpoint execution HEAD differs from endpoint custody")
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    if execution.get("protocol_sha256") != sha256(FREEZE_PATH):
        raise CustodyError("checkpoint protocol differs from endpoint execution")
    checkpoints = frozen_checkpoints(frozen)
    lanes = checkpoint_trigger_lanes(result)
    expected_trigger = {
        "schema_version": 1,
        "execution_head": expected_head,
        "protocol_sha256": sha256(FREEZE_PATH),
        "endpoint_execution_receipt_sha256": sha256(receipt_path),
        "endpoint_result_sha256": sha256(result_path),
        "triggered_lanes": lanes,
    }
    if consumer_trigger != expected_trigger:
        raise CustodyError("independent consumer checkpoint trigger receipt differs")
    destination.mkdir(parents=True)
    trigger_receipt = {
        "schema_version": 1,
        "execution_head": expected_head,
        "protocol_sha256": sha256(FREEZE_PATH),
        "endpoint_execution_receipt_sha256": sha256(receipt_path),
        "endpoint_result_sha256": sha256(result_path),
        "triggered_lanes": lanes,
        "checkpoint_count": len(checkpoints),
    }
    write_json(destination / "trigger-receipt.json", trigger_receipt)
    if not lanes:
        checkpoint_cargo_manifest = file_manifest(OUTPUT / "cargo-home")
        write_json(destination / "cargo-home-final-manifest.json", checkpoint_cargo_manifest)
        write_json(
            destination / "execution-receipt.json",
            {
                **trigger_receipt,
                "status": "not_triggered",
                "cargo_home_final_manifest_sha256": checkpoint_cargo_manifest["manifest_sha256"],
                "builds": {},
                "runs": {},
            },
        )
        require_clean_head(expected_head)
        return

    cargo_home = OUTPUT / "cargo-home"
    if not cargo_home.is_dir():
        raise CustodyError("endpoint Cargo home is absent")
    builds: dict[str, Any] = {}
    clones: dict[str, Path] = {}
    for index, (source_sha, expected_digest) in enumerate(checkpoints):
        checkpoint_id = f"{index:02d}-{source_sha}"
        clone = destination / "sources" / checkpoint_id
        clone_source(source_sha, clone)
        clones[source_sha] = clone
        receipt = build_source(
            source_sha,
            clone,
            cargo_home,
            destination / "cargo-targets" / checkpoint_id,
            artifact_group="checkpoint-search/builds",
            binary_group="checkpoint-search/binaries",
        )
        if receipt["build_input_digest"] != expected_digest:
            raise CustodyError(f"checkpoint build digest differs at {source_sha}")
        builds[source_sha] = receipt

    runs: dict[str, Any] = {}
    for forcing in lanes:
        fixture = OUTPUT / "fixtures" / forcing
        runs[forcing] = {}
        for index, (source_sha, _) in enumerate(checkpoints):
            checkpoint_id = f"{index:02d}-{source_sha}"
            binary = OUTPUT / builds[source_sha]["binary"]["path"]
            modes = {}
            for mode in ("control", "legacy"):
                modes[mode] = run_arm(
                    cell=checkpoint_id,
                    mode=mode,
                    forcing=forcing,
                    source_sha=source_sha,
                    clone=clones[source_sha],
                    binary=binary,
                    fixture=fixture,
                    frozen=frozen,
                    destination=destination / "runs" / forcing / checkpoint_id / mode,
                )
            control = destination / "runs" / forcing / checkpoint_id / "control"
            enabled = destination / "runs" / forcing / checkpoint_id / "legacy"
            protected = {
                suffix: output_hash(control, suffix) == output_hash(enabled, suffix)
                for suffix in (".hbp", ".wat.parquet", ".loss.json")
            }
            if not all(protected.values()):
                raise CustodyError(f"checkpoint protected output differs at {forcing}/{source_sha}")
            runs[forcing][checkpoint_id] = {
                "source_sha": source_sha,
                "modes": modes,
                "protected_outputs": protected,
            }
    checkpoint_cargo_manifest = file_manifest(cargo_home)
    write_json(destination / "cargo-home-final-manifest.json", checkpoint_cargo_manifest)
    write_json(
        destination / "execution-receipt.json",
        {
            **trigger_receipt,
            "status": "executed",
            "cargo_home_final_manifest_sha256": checkpoint_cargo_manifest["manifest_sha256"],
            "builds": builds,
            "runs": runs,
        },
    )
    require_clean_head(expected_head)


def execute(expected_head: str) -> None:
    require_clean_head(expected_head)
    execute_endpoint_body(expected_head)


def verify_checkpoint_search(frozen: dict[str, Any], current_head: str) -> None:
    destination = OUTPUT / "checkpoint-search"
    receipt_path = destination / "execution-receipt.json"
    endpoint_result = OUTPUT / "results/predecessor-bridge-results.json"
    if not receipt_path.exists():
        if endpoint_result.is_file():
            raise CustodyError(
                "endpoint result lacks mandatory checkpoint execution receipt"
            )
        return
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    endpoint_receipt = OUTPUT / "execution-receipt.json"
    consumer_trigger = OUTPUT / "results/checkpoint-trigger-receipt.json"
    if not endpoint_result.is_file() or not consumer_trigger.is_file():
        raise CustodyError("checkpoint search lacks endpoint result/trigger custody")
    result = json.loads(endpoint_result.read_text(encoding="utf-8"))
    lanes = checkpoint_trigger_lanes(result)
    exact = {
        "execution_head": current_head,
        "protocol_sha256": sha256(FREEZE_PATH),
        "endpoint_execution_receipt_sha256": sha256(endpoint_receipt),
        "endpoint_result_sha256": sha256(endpoint_result),
        "triggered_lanes": lanes,
        "checkpoint_count": 14,
    }
    for key, expected in exact.items():
        if receipt.get(key) != expected:
            raise CustodyError(f"checkpoint receipt {key} differs")
    if receipt.get("schema_version") != 1:
        raise CustodyError("checkpoint receipt schema differs")
    local_trigger = destination / "trigger-receipt.json"
    if not local_trigger.is_file():
        raise CustodyError("checkpoint local trigger receipt is absent")
    expected_local_trigger = {
        "schema_version": 1,
        **exact,
    }
    if json.loads(local_trigger.read_text(encoding="utf-8")) != expected_local_trigger:
        raise CustodyError("checkpoint local trigger receipt differs")
    trigger_expected = {
        "schema_version": 1,
        "execution_head": current_head,
        "protocol_sha256": sha256(FREEZE_PATH),
        "endpoint_execution_receipt_sha256": sha256(endpoint_receipt),
        "endpoint_result_sha256": sha256(endpoint_result),
        "triggered_lanes": lanes,
    }
    if json.loads(consumer_trigger.read_text(encoding="utf-8")) != trigger_expected:
        raise CustodyError("consumer checkpoint trigger differs")
    if not lanes:
        if receipt.get("status") != "not_triggered" or receipt.get("builds") != {} or receipt.get("runs") != {}:
            raise CustodyError("untriggered checkpoint receipt differs")
        return
    if receipt.get("status") != "executed":
        raise CustodyError("triggered checkpoint status differs")
    checkpoints = frozen_checkpoints(frozen)
    expected_ids = [f"{index:02d}-{source_sha}" for index, (source_sha, _) in enumerate(checkpoints)]
    builds = receipt.get("builds")
    if not isinstance(builds, dict) or set(builds) != {source_sha for source_sha, _ in checkpoints}:
        raise CustodyError("checkpoint build inventory differs")
    for index, (source_sha, expected_digest) in enumerate(checkpoints):
        checkpoint_id = expected_ids[index]
        build = builds[source_sha]
        if (
            build.get("source_sha") != source_sha
            or build.get("build_input_digest") != expected_digest
            or build.get("returncode") != 0
        ):
            raise CustodyError(f"checkpoint build differs at {source_sha}")
        binary = OUTPUT / build["binary"]["path"]
        if build["binary"]["path"] != (
            f"checkpoint-search/binaries/{source_sha}/openwepp-cli-hill"
        ):
            raise CustodyError(f"checkpoint binary path differs at {source_sha}")
        require_binary_match(
            binary,
            build["binary"]["sha256"],
            expected_size_bytes=build["binary"]["size_bytes"],
            context=f"checkpoint retained {source_sha}",
        )
        clone = destination / "sources" / checkpoint_id
        require_clone_identity(clone, source_sha, "checkpoint verification")
        target = destination / "cargo-targets" / checkpoint_id
        if file_manifest(target) != build.get("cargo_target_manifest"):
            raise CustodyError(f"checkpoint Cargo target differs at {source_sha}")
        build_receipt = destination / "builds" / source_sha / "build-receipt.json"
        if json.loads(build_receipt.read_text(encoding="utf-8")) != build:
            raise CustodyError(f"checkpoint build receipt differs at {source_sha}")
    runs = receipt.get("runs")
    if not isinstance(runs, dict) or set(runs) != set(lanes):
        raise CustodyError("checkpoint forcing-lane inventory differs")
    for forcing in lanes:
        if list(runs[forcing]) != expected_ids:
            raise CustodyError(f"checkpoint ordering differs for {forcing}")
        for checkpoint_id in expected_ids:
            item = runs[forcing][checkpoint_id]
            source_sha = item.get("source_sha")
            if source_sha != checkpoint_id.split("-", 1)[1]:
                raise CustodyError(
                    f"checkpoint source identity differs at {forcing}/{checkpoint_id}"
                )
            modes = item.get("modes")
            if not isinstance(modes, dict) or set(modes) != {"control", "legacy"}:
                raise CustodyError(f"checkpoint modes differ at {forcing}/{checkpoint_id}")
            for mode in ("control", "legacy"):
                arm = modes[mode]
                run_dir = destination / "runs" / forcing / checkpoint_id / mode
                require_arm_identity(
                    arm,
                    cell=checkpoint_id,
                    mode=mode,
                    source_sha=source_sha,
                    forcing=forcing,
                    context=f"checkpoint {forcing}",
                )
                if json.loads((run_dir / "arm-receipt.json").read_text(encoding="utf-8")) != arm:
                    raise CustodyError(f"checkpoint arm receipt differs at {forcing}/{checkpoint_id}/{mode}")
                if file_manifest(run_dir, exclude=frozenset({"arm-receipt.json"})) != arm.get("outputs"):
                    raise CustodyError(f"checkpoint run manifest differs at {forcing}/{checkpoint_id}/{mode}")
                trace = run_dir / TRACE_NAME
                if mode == "control" and trace.exists():
                    raise CustodyError(f"checkpoint control trace exists at {forcing}/{checkpoint_id}")
                if mode == "legacy" and (not trace.is_file() or trace.stat().st_size == 0):
                    raise CustodyError(f"checkpoint legacy trace absent at {forcing}/{checkpoint_id}")
                binary = run_dir / "bin/openwepp-cli-hill"
                require_binary_match(
                    binary,
                    builds[source_sha]["binary"]["sha256"],
                    context=f"checkpoint arm {forcing}/{checkpoint_id}/{mode}",
                )
                runtime = validate_run_manifest(
                    run_dir=run_dir,
                    fixture=OUTPUT / "fixtures" / forcing,
                    runfile=run_dir / f"{RUN_STEM}.run",
                    arm_binary=binary,
                    binary_sha=sha256(binary),
                    source_sha=source_sha,
                    argv=arm["argv"],
                    frozen=frozen,
                )
                if runtime != arm.get("runtime_manifest"):
                    raise CustodyError(f"checkpoint runtime manifest differs at {forcing}/{checkpoint_id}/{mode}")
                semantic = normalized_semantic_input_manifest(
                    source_sha=source_sha,
                    forcing=forcing,
                    fixture=OUTPUT / "fixtures" / forcing,
                    runfile=run_dir / f"{RUN_STEM}.run",
                    effective=arm["effective_openwepp_environment"],
                    frozen=frozen,
                )
                require_semantic_input_receipt(
                    arm.get("normalized_semantic_inputs"),
                    semantic,
                    context=f"checkpoint {forcing}/{checkpoint_id}/{mode}",
                )
            protected = {
                suffix: output_hash(
                    destination / "runs" / forcing / checkpoint_id / "control", suffix
                )
                == output_hash(
                    destination / "runs" / forcing / checkpoint_id / "legacy", suffix
                )
                for suffix in (".hbp", ".wat.parquet", ".loss.json")
            }
            require_protected_output_receipt(
                item.get("protected_outputs"),
                context=f"checkpoint {forcing}/{checkpoint_id}",
            )
            if protected != item.get("protected_outputs"):
                raise CustodyError(f"checkpoint protected outputs differ at {forcing}/{checkpoint_id}")


def execute_endpoint_body(expected_head: str) -> None:
    if OUTPUT.exists():
        raise CustodyError(f"refusing to overwrite {OUTPUT}")
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    if frozen.get("status") != "prospectively_frozen_before_result_execution":
        raise CustodyError("protocol freeze is not admitted")
    OUTPUT.mkdir(parents=True)
    inputs = OUTPUT / "inputs"
    inputs.mkdir()
    shutil.copy2(FREEZE_PATH, inputs / "protocol-freeze.json")
    retained_before = retained_anchor_checks(frozen)
    cargo_home = OUTPUT / "cargo-home"
    cargo_manifest = seed_cargo_home(cargo_home)
    write_json(OUTPUT / "cargo-home-manifest.json", cargo_manifest)
    sources = frozen["sources"]
    clones: dict[str, Path] = {}
    builds: dict[str, Any] = {}
    for source_name in ("old", "current"):
        source_sha = sources[source_name]
        clone = OUTPUT / "checkpoints" / source_sha / "source"
        clone_source(source_sha, clone)
        clones[source_name] = clone
        cargo_target = OUTPUT / "checkpoints" / source_sha / "cargo-target"
        builds[source_name] = build_source(
            source_sha, clone, cargo_home, cargo_target
        )
    fixtures = {}
    fixture_receipts = {}
    for forcing in ("canonical", "development"):
        fixtures[forcing], fixture_receipts[forcing] = prepare_fixture(forcing, frozen)
    cells: dict[str, dict[str, Any]] = {}
    for cell, value in endpoint_cells(frozen).items():
        source_name, forcing = value
        source_sha = sources[source_name]
        binary = OUTPUT / builds[source_name]["binary"]["path"]
        cells[cell] = {}
        for mode in ("control", "legacy"):
            cells[cell][mode] = run_arm(
                cell=cell,
                mode=mode,
                forcing=forcing,
                source_sha=source_sha,
                clone=clones[source_name],
                binary=binary,
                fixture=fixtures[forcing],
                frozen=frozen,
            )
        if source_name == "current":
            cells[cell]["explicit"] = run_arm(
                cell=cell,
                mode="explicit",
                forcing=forcing,
                source_sha=source_sha,
                clone=clones[source_name],
                binary=binary,
                fixture=fixtures[forcing],
                frozen=frozen,
            )
    semantic_checks = forcing_matched_semantic_checks(cells)
    selector_semantic_checks = current_selector_semantic_checks(cells)
    protected = protected_output_checks(cells)
    retained_after = retained_anchor_checks(frozen)
    if retained_before != retained_after:
        raise CustodyError("retained anchor changed during execution")
    cargo_final_manifest = file_manifest(cargo_home)
    write_json(OUTPUT / "cargo-home-final-manifest.json", cargo_final_manifest)
    receipt = {
        "schema_version": 1,
        "status": "endpoint_matrix_executed",
        "execution_head": expected_head,
        "protocol_sha256": sha256(FREEZE_PATH),
        "cargo_home_manifest_sha256": cargo_manifest["manifest_sha256"],
        "cargo_home_final_manifest_sha256": cargo_final_manifest["manifest_sha256"],
        "sources": sources,
        "builds": builds,
        "fixtures": fixture_receipts,
        "cells": cells,
        "forcing_matched_semantic_checks": semantic_checks,
        "current_selector_semantic_checks": selector_semantic_checks,
        "protected_outputs": protected,
        "retained_before": retained_before,
        "retained_after": retained_after,
    }
    write_json(OUTPUT / "execution-receipt.json", receipt)
    require_clean_head(expected_head)


def verify_existing() -> None:
    if git_output(["status", "--porcelain"]):
        raise CustodyError("verification requires a clean worktree")
    receipt_path = OUTPUT / "execution-receipt.json"
    if not receipt_path.is_file():
        raise CustodyError("missing execution receipt")
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    frozen = json.loads(FREEZE_PATH.read_text(encoding="utf-8"))
    current_head = git_output(["rev-parse", "HEAD"])
    if receipt.get("status") != "endpoint_matrix_executed":
        raise CustodyError("execution receipt status differs")
    if receipt.get("execution_head") != current_head:
        raise CustodyError("execution receipt HEAD differs from current HEAD")
    if receipt.get("protocol_sha256") != sha256(FREEZE_PATH):
        raise CustodyError("protocol freeze differs from execution")
    copied_protocol = OUTPUT / "inputs/protocol-freeze.json"
    if not copied_protocol.is_file() or sha256(copied_protocol) != sha256(FREEZE_PATH):
        raise CustodyError("copied protocol freeze differs")
    if receipt.get("sources") != frozen["sources"]:
        raise CustodyError("execution source inventory differs")

    cargo_initial_path = OUTPUT / "cargo-home-manifest.json"
    checkpoint_cargo_final = OUTPUT / "checkpoint-search/cargo-home-final-manifest.json"
    cargo_final_path = (
        checkpoint_cargo_final
        if checkpoint_cargo_final.is_file()
        else OUTPUT / "cargo-home-final-manifest.json"
    )
    if not cargo_initial_path.is_file() or not cargo_final_path.is_file():
        raise CustodyError("Cargo-home manifests are absent")
    cargo_initial = json.loads(cargo_initial_path.read_text(encoding="utf-8"))
    cargo_final = json.loads(cargo_final_path.read_text(encoding="utf-8"))
    if receipt.get("cargo_home_manifest_sha256") != cargo_initial.get("manifest_sha256"):
        raise CustodyError("initial Cargo-home manifest differs")
    expected_cargo_hash = receipt.get("cargo_home_final_manifest_sha256")
    checkpoint_execution_path = OUTPUT / "checkpoint-search/execution-receipt.json"
    if checkpoint_execution_path.is_file():
        checkpoint_execution = json.loads(
            checkpoint_execution_path.read_text(encoding="utf-8")
        )
        expected_cargo_hash = checkpoint_execution.get("cargo_home_final_manifest_sha256")
    if expected_cargo_hash != cargo_final.get("manifest_sha256"):
        raise CustodyError("final Cargo-home manifest binding differs")
    if file_manifest(OUTPUT / "cargo-home") != cargo_final:
        raise CustodyError("retained Cargo home differs from final manifest")

    for forcing in ("canonical", "development"):
        fixture = OUTPUT / "fixtures" / forcing
        if file_manifest(fixture) != receipt.get("fixtures", {}).get(forcing):
            raise CustodyError(f"fixture manifest differs for {forcing}")
        if sha256(fixture / "p8.cli") != frozen["forcings"][forcing]["sha256"]:
            raise CustodyError(f"fixture forcing differs for {forcing}")

    builds = receipt.get("builds")
    if not isinstance(builds, dict) or set(builds) != {"old", "current"}:
        raise CustodyError("endpoint build inventory differs")
    for source_name in ("old", "current"):
        source_sha = frozen["sources"][source_name]
        build = builds[source_name]
        if (
            build.get("source_sha") != source_sha
            or build.get("returncode") != 0
            or build.get("build_input_digest") != build_input_digest(source_sha)
        ):
            raise CustodyError(f"build receipt differs for {source_name}")
        binary = OUTPUT / build["binary"]["path"]
        require_binary_match(
            binary,
            build["binary"]["sha256"],
            expected_size_bytes=build["binary"]["size_bytes"],
            context=f"endpoint retained {source_name}",
        )
        clone = OUTPUT / "checkpoints" / source_sha / "source"
        require_clone_identity(clone, source_sha, "verification")
        target = OUTPUT / "checkpoints" / source_sha / "cargo-target"
        if file_manifest(target) != build.get("cargo_target_manifest"):
            raise CustodyError(f"Cargo target manifest differs for {source_name}")
        build_receipt = OUTPUT / "builds" / source_sha / "build-receipt.json"
        if json.loads(build_receipt.read_text(encoding="utf-8")) != build:
            raise CustodyError(f"retained build receipt differs for {source_name}")

    expected_cells = endpoint_cells(frozen)
    cells = receipt.get("cells")
    if not isinstance(cells, dict) or set(cells) != set(expected_cells):
        raise CustodyError("endpoint cell inventory differs")
    for cell, (source_name, forcing) in expected_cells.items():
        modes = cells[cell]
        expected_modes = {"control", "legacy", "explicit"} if source_name == "current" else {"control", "legacy"}
        if not isinstance(modes, dict) or set(modes) != expected_modes:
            raise CustodyError(f"endpoint mode inventory differs for {cell}")
        source_sha = frozen["sources"][source_name]
        for mode in sorted(expected_modes):
            arm = modes[mode]
            run_dir = OUTPUT / "runs" / cell / mode
            arm_receipt = run_dir / "arm-receipt.json"
            if not arm_receipt.is_file() or json.loads(arm_receipt.read_text(encoding="utf-8")) != arm:
                raise CustodyError(f"arm receipt differs: {cell}/{mode}")
            require_arm_identity(
                arm,
                cell=cell,
                mode=mode,
                source_sha=source_sha,
                forcing=forcing,
                context="endpoint",
            )
            actual_outputs = file_manifest(run_dir, exclude=frozenset({"arm-receipt.json"}))
            if actual_outputs != arm.get("outputs"):
                raise CustodyError(f"complete run manifest differs: {cell}/{mode}")
            for item in arm["outputs"]["files"]:
                path = run_dir / item["path"]
                if not path.is_file() or sha256(path) != item["sha256"] or path.stat().st_size != item["size_bytes"]:
                    raise CustodyError(f"retained artifact differs: {cell}/{mode}/{item['path']}")
            trace = run_dir / TRACE_NAME
            if mode == "control" and trace.exists():
                raise CustodyError(f"control trace exists: {cell}")
            if mode != "control" and (not trace.is_file() or trace.stat().st_size == 0):
                raise CustodyError(f"enabled trace absent: {cell}/{mode}")
            binary = run_dir / "bin/openwepp-cli-hill"
            require_binary_match(
                binary,
                builds[source_name]["binary"]["sha256"],
                context=f"endpoint arm {cell}/{mode}",
            )
            runtime = validate_run_manifest(
                run_dir=run_dir,
                fixture=OUTPUT / "fixtures" / forcing,
                runfile=run_dir / f"{RUN_STEM}.run",
                arm_binary=binary,
                binary_sha=sha256(binary),
                source_sha=source_sha,
                argv=arm["argv"],
                frozen=frozen,
            )
            if runtime != arm.get("runtime_manifest"):
                raise CustodyError(f"runtime manifest receipt differs: {cell}/{mode}")
            semantic = normalized_semantic_input_manifest(
                source_sha=source_sha,
                forcing=forcing,
                fixture=OUTPUT / "fixtures" / forcing,
                runfile=run_dir / f"{RUN_STEM}.run",
                effective=arm["effective_openwepp_environment"],
                frozen=frozen,
            )
            require_semantic_input_receipt(
                arm.get("normalized_semantic_inputs"),
                semantic,
                context=f"endpoint {cell}/{mode}",
            )
    if forcing_matched_semantic_checks(cells) != receipt.get("forcing_matched_semantic_checks"):
        raise CustodyError("forcing-matched semantic checks differ")
    if current_selector_semantic_checks(cells) != receipt.get("current_selector_semantic_checks"):
        raise CustodyError("current selector semantic checks differ")
    if protected_output_checks(cells) != receipt.get("protected_outputs"):
        raise CustodyError("protected output receipt differs")
    if retained_anchor_checks(frozen) != receipt["retained_after"]:
        raise CustodyError("retained anchor custody differs")
    verify_checkpoint_search(frozen, current_head)
    allowed_top = {
        "binaries", "builds", "cargo-home", "cargo-home-final-manifest.json",
        "cargo-home-manifest.json", "checkpoints", "execution-receipt.json",
        "fixtures", "inputs", "runs", "results", "retained-artifact-manifest.json",
        "checkpoint-search",
    }
    unexpected = {path.name for path in OUTPUT.iterdir()} - allowed_top
    if unexpected:
        raise CustodyError(f"unexpected retained top-level artifacts: {sorted(unexpected)}")
    print("PASS verified predecessor endpoint matrix custody")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--execute", action="store_true")
    group.add_argument("--execute-checkpoints", action="store_true")
    group.add_argument("--verify-existing", action="store_true")
    parser.add_argument("--expected-head")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if args.execute or args.execute_checkpoints:
            if args.expected_head is None:
                raise CustodyError("--expected-head is required with execution")
            if args.execute:
                execute(args.expected_head)
            else:
                execute_checkpoints(args.expected_head)
        else:
            verify_existing()
    except CustodyError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
