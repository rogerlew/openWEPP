# CLI03 Owned File Manifest

Status: completed
Evidence mode: Static

## Files Changed For CLI03 Scope
- `Cargo.toml`
- `Cargo.lock`
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-hillslope-output/**` (new crate)
- `tests/integration/cli03_runner_contract_derived_tests.rs` (new)
- `tests/integration/cli01_runner_hillslope_integration.rs`
- `tests/fixtures/cli01/hillslope_run_dir/case.run`
- `tests/fixtures/cli01/hillslope_run_dir_unknown/case.run`
- `open_wepp_runner/open_wepp_runner.py`
- `open_wepp_runner/__init__.py`
- `open_wepp_runner/README.md`
- `tests/python/test_open_wepp_runner_api.py`
- `docs/decisions/0007-openwepp-runner-and-release-governance.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/package.md`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/artifacts/*.md`

## Excluded Generated Noise
- Python `__pycache__` files are runtime-generated artifacts and are not part of
  CLI03 functional scope.
