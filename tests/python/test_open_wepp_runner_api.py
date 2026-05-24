from __future__ import annotations

import os
import shutil
import tempfile
from pathlib import Path

import pytest

from open_wepp_runner import open_wepp_runner as owr


def _repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def _fixture_dir(name: str) -> Path:
    return _repo_root() / "tests" / "fixtures" / "cli01" / name


def _binary_path(binary_name: str) -> Path:
    return _repo_root() / "target" / "debug" / binary_name


@pytest.fixture
def runner_env(monkeypatch: pytest.MonkeyPatch) -> None:
    runner = _binary_path("open_wepp_runner")
    hill = _binary_path("openwepp-cli-hill")
    if not runner.is_file() or not hill.is_file():
        pytest.skip("openWEPP runner binaries are not built")
    monkeypatch.setenv("OPENWEPP_RUNNER_BIN", str(runner))
    monkeypatch.setenv("OPENWEPP_HILLSLOPE_BIN", str(hill))
    monkeypatch.setenv("OPENWEPP_SIDECAR_POLICY", "strict")


def _prepare_runs_dir() -> tuple[Path, Path]:
    tmp = Path(tempfile.mkdtemp(prefix="open_wepp_runner_pytest_"))
    runs = tmp / "runs"
    output = tmp / "output"
    runs.mkdir(parents=True, exist_ok=True)
    output.mkdir(parents=True, exist_ok=True)

    source = _fixture_dir("hillslope_run_dir")
    shutil.copy(source / "case.man", runs / "p1.man")
    shutil.copy(source / "case.slp", runs / "p1.slp")
    shutil.copy(source / "case.cli", runs / "p1.cli")
    shutil.copy(source / "case.sol", runs / "p1.sol")
    shutil.copy(source / "frost.txt", runs / "frost.txt")
    shutil.copy(source / "snow.txt", runs / "snow.txt")
    shutil.copy(source / "wepp_ui.txt", runs / "wepp_ui.txt")
    shutil.copy(source / "pmetpara.txt", runs / "pmetpara.txt")
    return tmp, runs


def test_make_hillslope_run_writes_compatible_runfile() -> None:
    tmp, runs = _prepare_runs_dir()
    try:
        owr.make_hillslope_run(1, 1, str(runs))
        run_file = runs / "p1.run"
        assert run_file.is_file()
        contents = run_file.read_text(encoding="utf-8")
        assert "../output/H1.pass.dat" in contents
        assert "p1.man" in contents
        assert "p1.cli" in contents
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


def test_run_hillslope_executes_openwepp_runner_boundary(runner_env: None) -> None:
    tmp, runs = _prepare_runs_dir()
    try:
        owr.make_hillslope_run(1, 1, str(runs))
        ok, wepp_id, elapsed = owr.run_hillslope(1, str(runs), timeout=30, timeout_retries=0)
        assert ok is True
        assert wepp_id == 1
        assert elapsed >= 0
        output_dir = tmp / "output"
        assert (output_dir / "H5.wat.dat").is_file()
        assert (output_dir / "H5.plot.dat").is_file()
        assert (output_dir / "openwepp_hillslope_run_manifest.json").is_file()
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


def test_watershed_surfaces_fail_explicitly() -> None:
    with pytest.raises(NotImplementedError):
        owr.run_watershed("/tmp/nowhere")


def test_ss_surfaces_are_not_exposed() -> None:
    forbidden_names = [
        "make_ss_flowpath_run",
        "make_ss_hillslope_run",
        "make_ss_batch_hillslope_run",
        "run_ss_batch_hillslope",
        "make_ss_watershed_run",
        "make_ss_batch_watershed_run",
        "run_ss_batch_watershed",
    ]
    for name in forbidden_names:
        assert not hasattr(owr, name), f"unexpected SS symbol exported: {name}"
