# HILLSTAB07 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: mixed (`Static` + `Ran`)

## Added Contract-Derived Test

- File: `tests/integration/cli03_runner_contract_derived_tests.rs`
- Test:
  `cli03_fixture_run_publishes_wb16_ealpha_compatibility_seed_provenance`
- Assertions:
  1. `sidecar_warnings` includes `SIMPIPE-W-003`.
  2. Run manifest includes
     `"wb16_ealpha_compatibility_seed_used": true`.
  3. Run manifest includes
     `"wb16_ealpha_seed_policy": "compatibility_seed_1p0"`.

## Execution Evidence

- Ran:
  - `cargo test --test cli03_runner_contract_derived_tests cli03_fixture_run_publishes_wb16_ealpha_compatibility_seed_provenance`
  - Result: `ok` (`1 passed`, `0 failed`).
