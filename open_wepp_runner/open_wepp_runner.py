"""
open_wepp_runner: compatibility-facing Python launcher API for openWEPP.

This module mirrors the call surface of ``wepppy.wepp_runner`` where practical,
but executes through the canonical openWEPP launcher boundary:

    open_wepp_runner run-hillslope ...

Only hillslope surfaces are implemented in the current CLI scope. Flowpath and
watershed surfaces are not implemented; flowpath surfaces are intentionally
omitted because flowpath execution is deprecated in wepppy.
"""

from __future__ import annotations

import json
import os
import random
import shutil
import subprocess
import tomllib
from os.path import abspath as _abspath
from os.path import dirname as _dirname
from os.path import exists as _exists
from os.path import isfile as _isfile
from os.path import join as _join
from pathlib import Path
from time import sleep, time

from .status_messenger import StatusMessenger

__all__ = [
    "wepp_bin_dir",
    "linux_wepp_bin_opts",
    "get_linux_wepp_bin_opts",
    "infer_pass_family_for_wepp_bin",
    "PASS_FAMILY_HBP",
    "PASS_FAMILY_CHOICES",
    "make_hillslope_run",
    "run_hillslope",
    "make_watershed_omni_contrasts_run",
    "make_watershed_run",
    "run_watershed",
]

PASS_FAMILY_HBP = "hbp"
PASS_FAMILY_CHOICES = {PASS_FAMILY_HBP}

_thisdir = _dirname(__file__)
wepp_bin_dir = _abspath(_join(_thisdir, "bin"))

_DEFAULT_RUNNER_BINARY = "open_wepp_runner"
_DEFAULT_HILLSLOPE_BINARY = "openwepp-cli-hill"
_DEFAULT_HILLSLOPE_OUTPUT_DIR_NAME = "output"
_DEFAULT_HILLSLOPE_MANIFEST_NAME = "openwepp_hillslope_run_manifest.json"
_SUPPORTED_SIDECAR_POLICIES = {"strict", "compat"}


def _compute_linux_wepp_bin_opts() -> list[str]:
    if not os.path.isdir(wepp_bin_dir):
        return ["latest"]
    opts = []
    for entry in os.listdir(wepp_bin_dir):
        path = _join(wepp_bin_dir, entry)
        if not _isfile(path):
            continue
        if not entry.startswith("openwepp_"):
            continue
        if "." in entry:
            continue
        if entry.endswith("_hill") or entry.endswith("_replay"):
            continue
        opts.append(entry)
    opts.append("latest")
    opts.sort()
    return opts


linux_wepp_bin_opts = _compute_linux_wepp_bin_opts()


def get_linux_wepp_bin_opts() -> list[str]:
    """Return the current openWEPP binary tags available on disk."""
    return _compute_linux_wepp_bin_opts()


def infer_pass_family_for_wepp_bin(wepp_bin: str | None = None) -> str:
    """
    Infer pass-family support from the selected hillslope binary sidecar.

    Sidecar absence defaults to HBP under binary-only policy.
    """
    binary_path = _resolve_hillslope_binary(wepp_bin)
    metadata = _load_binary_release_metadata(binary_path)
    if metadata is None:
        return PASS_FAMILY_HBP

    features = metadata.get("features")
    if not isinstance(features, dict):
        raise RuntimeError(
            f"OPEN_RUNNER-E-020 invalid release sidecar features for {binary_path}"
        )
    supported = features.get("hbp_supported")
    if not isinstance(supported, bool):
        raise RuntimeError(
            f"OPEN_RUNNER-E-021 invalid features.hbp_supported for {binary_path}"
        )
    if not supported:
        raise RuntimeError(
            "OPEN_RUNNER-E-027 release sidecar declares hbp_supported=false; "
            "legacy ASCII pass family is unsupported."
        )
    return PASS_FAMILY_HBP


def _normalize_pass_family(pass_family: str | None) -> str:
    if pass_family is None:
        return PASS_FAMILY_HBP
    normalized = str(pass_family).strip().lower()
    if normalized not in PASS_FAMILY_CHOICES:
        raise ValueError(
            f"OPEN_RUNNER-E-010 unsupported pass_family '{pass_family}'. "
            "Expected: hbp"
        )
    return normalized


def _pass_suffix(pass_family: str | None) -> str:
    _ = _normalize_pass_family(pass_family)
    return ".hbp"


def _binary_sidecar_path(binary_path: str) -> str:
    return f"{binary_path}.json"


def _load_binary_release_metadata(binary_path: str) -> dict | None:
    sidecar_path = _binary_sidecar_path(binary_path)
    if not _exists(sidecar_path):
        return None
    with open(sidecar_path, encoding="utf-8") as fp:
        payload = json.load(fp)
    if not isinstance(payload, dict):
        raise RuntimeError(f"OPEN_RUNNER-E-022 invalid JSON object in {sidecar_path}")
    return payload


def _require_relpath_suffix(name: str, value: str) -> None:
    if value and not value.endswith("/"):
        raise ValueError(
            f"OPEN_RUNNER-E-011 {name} must end with '/' when non-empty: {value}"
        )


def _resolve_env_or_path(
    *,
    env_var: str,
    default_executable: str,
) -> str:
    configured = os.environ.get(env_var, "").strip()
    candidate = configured or default_executable
    return _resolve_executable_token(candidate)


def _resolve_executable_token(token: str) -> str:
    token = token.strip()
    if not token:
        raise FileNotFoundError("OPEN_RUNNER-E-001 empty executable token")

    if os.path.sep in token or token.startswith("."):
        path = Path(token).expanduser()
        if not path.is_file():
            raise FileNotFoundError(f"OPEN_RUNNER-E-001 executable not found: {path}")
        return str(path.resolve())

    local_candidate = Path(wepp_bin_dir) / token
    if local_candidate.is_file():
        return str(local_candidate.resolve())

    resolved = shutil.which(token)
    if resolved:
        return resolved

    raise FileNotFoundError(f"OPEN_RUNNER-E-001 executable not found: {token}")


def _resolve_runner_binary() -> str:
    return _resolve_env_or_path(
        env_var="OPENWEPP_RUNNER_BIN",
        default_executable=_DEFAULT_RUNNER_BINARY,
    )


def _resolve_hillslope_binary(wepp_bin: str | None) -> str:
    if wepp_bin is None:
        return _resolve_env_or_path(
            env_var="OPENWEPP_HILLSLOPE_BIN",
            default_executable=_DEFAULT_HILLSLOPE_BINARY,
        )

    token = str(wepp_bin).strip()
    if not token:
        raise ValueError("OPEN_RUNNER-E-002 empty wepp_bin selector")

    if token == "latest":
        latest_env = os.environ.get("OPENWEPP_HILLSLOPE_LATEST_BIN", "").strip()
        if latest_env:
            return _resolve_executable_token(latest_env)
        # Fall back to canonical default hillslope binary name.
        return _resolve_env_or_path(
            env_var="OPENWEPP_HILLSLOPE_BIN",
            default_executable=_DEFAULT_HILLSLOPE_BINARY,
        )

    if os.path.sep in token or token.startswith("."):
        return _resolve_executable_token(token)

    # Compatibility: if caller passes base tag without role suffix, prefer _hill.
    candidates = [token]
    if not token.endswith("_hill"):
        candidates.insert(0, f"{token}_hill")
    for candidate in candidates:
        try:
            return _resolve_executable_token(candidate)
        except FileNotFoundError:
            continue

    raise FileNotFoundError(
        f"OPEN_RUNNER-E-002 unable to resolve hillslope binary for selector '{token}'"
    )


def _validate_hillslope_inputs(
    *,
    runs_dir: str,
    run_file_name: str,
    required_inputs: list[str],
    no_file_checks: bool,
) -> None:
    if not _exists(runs_dir):
        raise FileNotFoundError(f"OPEN_RUNNER-E-003 runs_dir does not exist: {runs_dir}")
    if not os.path.isdir(runs_dir):
        raise NotADirectoryError(f"OPEN_RUNNER-E-004 runs_dir is not a directory: {runs_dir}")

    run_file_path = _join(runs_dir, run_file_name)
    if not _isfile(run_file_path):
        raise FileNotFoundError(f"OPEN_RUNNER-E-005 run file not found: {run_file_path}")

    if no_file_checks:
        return

    missing = [path for path in required_inputs if not _isfile(path)]
    if missing:
        details = ", ".join(missing)
        raise FileNotFoundError(f"OPEN_RUNNER-E-006 missing required input(s): {details}")


def _resolve_sidecar_policy() -> str:
    policy = os.environ.get("OPENWEPP_SIDECAR_POLICY", "strict").strip().lower()
    if policy not in _SUPPORTED_SIDECAR_POLICIES:
        supported = ", ".join(sorted(_SUPPORTED_SIDECAR_POLICIES))
        raise ValueError(
            f"OPEN_RUNNER-E-012 unsupported sidecar policy '{policy}'. "
            f"Expected one of: {supported}"
        )
    return policy


def _default_output_dir(runs_dir: str) -> str:
    return _abspath(_join(runs_dir, "..", _DEFAULT_HILLSLOPE_OUTPUT_DIR_NAME))


def _publish_status(status_channel: str | None, message: str) -> None:
    if not status_channel:
        return
    StatusMessenger.publish(status_channel, message)


def _run_hillslope_process(
    *,
    wepp_id: int | str,
    runs_dir: str,
    run_file_name: str,
    wepp_bin: str | None,
    status_channel: str | None,
    timeout: float,
    timeout_retries: int,
) -> tuple[str, str]:
    if timeout_retries < 0:
        raise ValueError(
            f"OPEN_RUNNER-E-013 timeout_retries must be >= 0 (received {timeout_retries})"
        )
    if timeout <= 0:
        raise ValueError(f"OPEN_RUNNER-E-014 timeout must be > 0 (received {timeout})")

    runner_binary = _resolve_runner_binary()
    hillslope_binary = _resolve_hillslope_binary(wepp_bin)
    sidecar_policy = _resolve_sidecar_policy()
    output_dir = _default_output_dir(runs_dir)
    os.makedirs(output_dir, exist_ok=True)
    manifest_path = _join(output_dir, _DEFAULT_HILLSLOPE_MANIFEST_NAME)

    stderr_path = _join(runs_dir, f"p{wepp_id}.err")
    command = [
        runner_binary,
        "run-hillslope",
        "--hillslope-binary",
        hillslope_binary,
        "--run-dir",
        runs_dir,
        "--run-file",
        run_file_name,
        "--output-dir",
        output_dir,
        "--policy",
        sidecar_policy,
        "--manifest-path",
        manifest_path,
    ]

    total_attempts = timeout_retries + 1
    timeout_summary: list[str] = []
    backoff_base_seconds = 0.5
    backoff_cap_seconds = 5.0

    log_lines = [
        f"[open_wepp_runner] wepp_id={wepp_id} runs_dir={runs_dir} "
        f'cmd="{" ".join(command)}" timeout={timeout}s timeout_retries={timeout_retries}'
    ]

    for attempt in range(1, total_attempts + 1):
        log_lines.append(f"[open_wepp_runner] attempt {attempt}/{total_attempts} start")
        _publish_status(
            status_channel,
            f"open_wepp_runner attempt {attempt}/{total_attempts} wepp_id={wepp_id}",
        )

        try:
            completed = subprocess.run(
                command,
                cwd=runs_dir,
                check=False,
                capture_output=True,
                text=True,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            timeout_summary.append(f"a{attempt}:{timeout:.2f}s")
            log_lines.append(
                f"[open_wepp_runner] timeout attempt={attempt}/{total_attempts} "
                f"timeout={timeout}s"
            )
            if attempt < total_attempts:
                backoff_seconds = min(
                    backoff_cap_seconds,
                    backoff_base_seconds * (2 ** (attempt - 1)),
                )
                jitter_seconds = random.uniform(0.0, backoff_base_seconds)
                sleep(backoff_seconds + jitter_seconds)
                continue
            _write_run_log(stderr_path, log_lines)
            summary = "; ".join(timeout_summary)
            raise TimeoutError(
                f"OPEN_RUNNER-E-015 hillslope command timed out "
                f"(attempts={total_attempts}; summary=[{summary}])"
            )

        if completed.stdout:
            for line in completed.stdout.splitlines():
                line = line.strip()
                if not line:
                    continue
                log_lines.append(line)
                _publish_status(status_channel, line)

        if completed.stderr:
            for line in completed.stderr.splitlines():
                line = line.strip()
                if not line:
                    continue
                log_lines.append(line)
                _publish_status(status_channel, line)

        log_lines.append(
            f"[open_wepp_runner] attempt {attempt}/{total_attempts} "
            f"returncode={completed.returncode}"
        )

        if completed.returncode == 0:
            _write_run_log(stderr_path, log_lines)
            return output_dir, manifest_path

        if attempt < total_attempts:
            backoff_seconds = min(
                backoff_cap_seconds,
                backoff_base_seconds * (2 ** (attempt - 1)),
            )
            jitter_seconds = random.uniform(0.0, backoff_base_seconds)
            sleep(backoff_seconds + jitter_seconds)
            continue

        _write_run_log(stderr_path, log_lines)
        raise RuntimeError(
            f"OPEN_RUNNER-E-016 open_wepp_runner failed with exit "
            f"{completed.returncode}; see {stderr_path}"
        )

    raise RuntimeError("OPEN_RUNNER-E-017 internal execution state failure")


def _write_run_log(path: str, lines: list[str]) -> None:
    with open(path, "w", encoding="utf-8") as log:
        for line in lines:
            log.write(line + "\n")


def _resolve_runfile_required_outputs(runs_dir: str, run_file_name: str) -> list[str]:
    run_file_path = Path(runs_dir) / run_file_name
    payload = run_file_path.read_text(encoding="utf-8")
    try:
        runfile = tomllib.loads(payload)
    except tomllib.TOMLDecodeError as exc:
        raise RuntimeError(
            f"OPEN_RUNNER-E-023 invalid TOML in run file {run_file_path}: {exc}"
        ) from exc

    outputs = runfile.get("outputs")
    if not isinstance(outputs, dict):
        raise RuntimeError(
            f"OPEN_RUNNER-E-024 missing required [outputs] table in run file {run_file_path}"
        )

    required_fields = ("pass", "loss")
    resolved_paths: list[str] = []
    for field in required_fields:
        value = outputs.get(field)
        if not isinstance(value, str) or not value.strip():
            raise RuntimeError(
                f"OPEN_RUNNER-E-025 missing required outputs.{field} in run file {run_file_path}"
            )
        output_path = Path(value.strip())
        if not output_path.is_absolute():
            output_path = run_file_path.parent / output_path
        resolved_paths.append(str(output_path))

    return resolved_paths


def _verify_required_outputs(
    *,
    runs_dir: str,
    run_file_name: str,
) -> None:
    required = _resolve_runfile_required_outputs(runs_dir, run_file_name)
    missing = [path for path in required if not _isfile(path)]
    if missing:
        raise RuntimeError(
            "OPEN_RUNNER-E-018 missing required output(s): " + ", ".join(missing)
        )


def make_hillslope_run(
    wepp_id,
    sim_years,
    runs_dir,
    reveg=True,  # noqa: FBT002
    man_relpath="",
    cli_relpath="",
    slp_relpath="",
    sol_relpath="",
    pass_family=PASS_FAMILY_HBP,
    wepp_bin=None,
):  # noqa: ANN001, ARG001
    _require_relpath_suffix("man_relpath", man_relpath)
    _require_relpath_suffix("cli_relpath", cli_relpath)
    _require_relpath_suffix("slp_relpath", slp_relpath)
    _require_relpath_suffix("sol_relpath", sol_relpath)
    _ = _normalize_pass_family(pass_family)

    _ = reveg  # retained for API compatibility
    _ = wepp_bin  # retained for API compatibility
    run_text = f"""schema = "openwepp-hillslope-runfile-v1"
run_name = "p{wepp_id}_y{sim_years}"
unit_system = "metric"

[inputs]
soil = "{sol_relpath}p{wepp_id}.sol"
management = "{man_relpath}p{wepp_id}.man"
slope = "{slp_relpath}p{wepp_id}.slp"
climate = "{cli_relpath}p{wepp_id}.cli"
wepp_ui = true
pmetpara = "pmetpara.txt"

[outputs]
pass = "../output/H{wepp_id}.hbp"
loss = "../output/H{wepp_id}.loss.json"
wat = "../output/H{wepp_id}.wat.parquet"
plot = "../output/H{wepp_id}.plot.parquet"
"""
    os.makedirs(runs_dir, exist_ok=True)
    with open(_join(runs_dir, f"p{wepp_id}.run"), "w", encoding="utf-8") as fp:
        fp.write(run_text)


def run_hillslope(
    wepp_id,
    runs_dir,
    wepp_bin=None,
    status_channel=None,
    man_relpath="",
    cli_relpath="",
    slp_relpath="",
    sol_relpath="",
    no_file_checks=False,
    timeout=60,
    timeout_retries=3,
):  # noqa: ANN001
    _require_relpath_suffix("man_relpath", man_relpath)
    _require_relpath_suffix("cli_relpath", cli_relpath)
    _require_relpath_suffix("slp_relpath", slp_relpath)
    _require_relpath_suffix("sol_relpath", sol_relpath)

    t0 = time()
    runs_dir = _abspath(runs_dir)
    run_file_name = f"p{wepp_id}.run"
    required_inputs = [
        _join(runs_dir, man_relpath, f"p{wepp_id}.man"),
        _join(runs_dir, slp_relpath, f"p{wepp_id}.slp"),
        _join(runs_dir, cli_relpath, f"p{wepp_id}.cli"),
        _join(runs_dir, sol_relpath, f"p{wepp_id}.sol"),
    ]
    _validate_hillslope_inputs(
        runs_dir=runs_dir,
        run_file_name=run_file_name,
        required_inputs=required_inputs,
        no_file_checks=bool(no_file_checks),
    )

    _output_dir, _manifest_path = _run_hillslope_process(
        wepp_id=wepp_id,
        runs_dir=runs_dir,
        run_file_name=run_file_name,
        wepp_bin=wepp_bin,
        status_channel=status_channel,
        timeout=float(timeout),
        timeout_retries=int(timeout_retries),
    )
    _verify_required_outputs(
        runs_dir=runs_dir,
        run_file_name=run_file_name,
    )
    return True, wepp_id, time() - t0


def make_watershed_omni_contrasts_run(
    sim_years,
    wepp_path_ids,
    runs_dir,
    *,
    output_options=None,
    pass_family=PASS_FAMILY_HBP,
    wepp_bin=None,
):  # noqa: ANN001, ARG001
    raise NotImplementedError(
        "OPEN_RUNNER-E-101 watershed runfile generation is not implemented."
    )


def make_watershed_run(
    sim_years,
    wepp_ids,
    runs_dir,
    *,
    pass_family=PASS_FAMILY_HBP,
    wepp_bin=None,
):  # noqa: ANN001, ARG001
    raise NotImplementedError(
        "OPEN_RUNNER-E-101 watershed runfile generation is not implemented."
    )


def run_watershed(runs_dir, wepp_bin=None, status_channel=None):  # noqa: ANN001, ARG001
    raise NotImplementedError(
        "OPEN_RUNNER-E-102 watershed execution is not implemented."
    )
