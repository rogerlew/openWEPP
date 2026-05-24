# CLI03 wepppy Consumer Boundary Note

Status: completed
Evidence mode: Static + Ran

## Static
- Python compatibility wrapper alignment is implemented in
  `open_wepp_runner/open_wepp_runner.py`:
  - `make_hillslope_run` now emits CLI03 TOML runfiles (`schema =
    "openwepp-hillslope-runfile-v1"`, `unit_system = "metric"`),
  - required output verification now resolves required `[outputs]` entries
    (`pass`, `loss`) from the generated runfile instead of legacy fixed
    `.dat` assertions,
  - legacy ASCII pass-family runfile generation is rejected with explicit
    `OPEN_RUNNER-E-026` because CLI03 runfile authority requires HBP pass
    output and JSON loss output.
- Python tests were updated to assert CLI03 runfile/output behavior and to keep
  flowpath API surfaces omitted.
- Flowpath API surfaces remain intentionally omitted in
  `open_wepp_runner/open_wepp_runner.py` because flowpath execution is
  deprecated in wepppy.

Boundary interpretation:
- CLI03 Rust runner/CLI objective is closed.
- CLI03 Python consumer boundary alignment is also closed in this package.

## Ran
- `cargo build -p openwepp-runner --bins`
  - pass.
- `PYTHONPATH=. pytest -q tests/python/test_open_wepp_runner_api.py`
  - pass (`5 passed`).
