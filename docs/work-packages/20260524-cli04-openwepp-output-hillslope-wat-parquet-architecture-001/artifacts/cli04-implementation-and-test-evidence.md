# CLI04 Implementation And Test Evidence

Status: completed
Evidence mode: Static + Ran

## Static
Production implementation changes completed:
- Added dedicated hillslope WAT parquet writer module:
  - `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- Exposed WAT writer module from output crate:
  - `crates/openwepp-hillslope-output/src/lib.rs`
- Added output crate dependencies for CLI04 parquet stack:
  - `arrow-array`, `arrow-schema`, `parquet`.
- Wired runner `outputs.wat` emission to real parquet serialization:
  - `crates/openwepp-runner/src/lib.rs`
  - `execute_hillslope_run` now writes `outputs.wat` via
    `write_hillslope_wat_parquet(...)` when configured.
- Added typed WAT writer errors:
  - `OHOUT-WAT-E-001` (IO)
  - `OHOUT-WAT-E-002` (parquet)
- Added contract-derived CLI04 integration test:
  - `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
- Registered CLI04 integration test target in root `Cargo.toml`.
- Added root dev-dependencies required for schema inspection in integration
  tests:
  - `arrow-schema`
  - `parquet`
- Updated CLI03 heading assertion for combined spec heading compatibility:
  - `tests/integration/cli03_runner_contract_derived_tests.rs`
- Recorded temporary RustSec risk acceptance in `deny.toml` for
  transitive `paste` advisory used through parquet/arrow-rs dependency graph:
  - `RUSTSEC-2024-0436`.

Boundary decision status:
- Shared boundary target `crates/openwepp-output/` is ratified in contracts.
- CLI04 implementation remains on transition predecessor crate
  `crates/openwepp-hillslope-output/` for current code surfaces.

## Ran
Pre-implementation sequencing evidence (Phase B gate):
- `cargo test --test cli03_runner_contract_derived_tests`
  - pass (`9 passed; 0 failed`).
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests`
  - fail pre-implementation (`Parquet error: Invalid Parquet file. Corrupt footer`).

Post-implementation targeted verification:
- `cargo test -p openwepp-hillslope-output`
  - pass (`14 passed; 0 failed`).
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests`
  - pass (`2 passed; 0 failed`).
- `cargo test --test cli03_runner_contract_derived_tests`
  - pass (`9 passed; 0 failed`).

Repository closeout gates:
- `cargo fmt --check`
  - pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - pass.
- `cargo test --workspace`
  - pass.
- `cargo deny check`
  - pass (`advisories ok, bans ok, licenses ok, sources ok`) with
    non-fatal duplicate/allowlist warnings.

Consumer-boundary verification:
- `PYTHONPATH=. pytest -q tests/python/test_open_wepp_runner_api.py`
  - pass (`5 passed`).
