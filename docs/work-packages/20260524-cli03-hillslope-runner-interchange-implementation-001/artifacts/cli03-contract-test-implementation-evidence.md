# CLI03 Contract Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Static
- Added dedicated CLI03 output crate:
  - `crates/openwepp-hillslope-output/Cargo.toml`
  - `crates/openwepp-hillslope-output/src/lib.rs`
  - `crates/openwepp-hillslope-output/src/contracts.rs`
  - `crates/openwepp-hillslope-output/src/manifest.rs`
  - `crates/openwepp-hillslope-output/src/writers.rs`
- Added CLI03 contract-derived integration target and registration:
  - `tests/integration/cli03_runner_contract_derived_tests.rs`
  - root `Cargo.toml` `[[test]]` entry: `cli03_runner_contract_derived_tests`
- Registered new workspace member in root `Cargo.toml`:
  - `crates/openwepp-hillslope-output`
- Contract-derived test coverage implemented for CLI03 minimums:
  - contract/spec surface assertions for metric-only runfile + required outputs,
  - output crate layout assertions,
  - runner hillslope binary metadata assertion,
  - runner-to-output-crate wiring assertion,
  - `.run` schema mismatch hard-fail assertion,
  - `.run` non-metric unit-system rejection assertion,
  - `.run` unresolved required input path rejection assertion.

## Ran
- Command:
  - `cargo test -p openwepp-hillslope-output`
- Observed result:
  - pass (`11 passed; 0 failed`)

- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed result:
  - fail (`4 passed; 3 failed`)
- Failing tests (pre-implementation gaps):
  - `cli03_runner_crate_wires_output_surface_dependency`
  - `cli03_runfile_validation_rejects_non_metric_unit_system`
  - `cli03_runfile_validation_rejects_unresolved_required_input_paths`
- Failure signatures show current runner behavior is not yet CLI03-conformant:
  - runner crate does not yet declare/wire `openwepp-hillslope-output` dependency,
  - non-metric and unresolved-required-input `.run` test cases currently
    execute successfully instead of hard-failing on contract-required
    validation checks.
