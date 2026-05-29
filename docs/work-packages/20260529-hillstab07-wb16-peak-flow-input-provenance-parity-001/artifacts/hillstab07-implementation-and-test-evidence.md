# HILLSTAB07 Implementation and Test Evidence

Status: complete  
Evidence mode: mixed (`Static` + `Ran`)

## Production Implementation

- File: `crates/openwepp-runner/src/hillslope/mod.rs`
- Implemented:
  1. Sticky runtime flag publication:
     `wb16_ealpha_compatibility_seed_used` (state surface symbol).
  2. Manifest execution provenance fields:
     - `wb16_ealpha_compatibility_seed_used` (`bool`)
     - `wb16_ealpha_seed_policy` (`"runtime_provided"` or
       `"compatibility_seed_1p0"`)
  3. Explicit warning publication when compatibility seed path is used:
     `SIMPIPE-W-003`.
  4. Retained canonical compatibility seed value `ealpha=1.0` while making the
     behavior explicit/non-silent.

## Test/Gate Execution

- Ran: `cargo fmt --check` (pass; final run).
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` (pass).
- Ran: `cargo test --workspace` (pass).
- Ran: `cargo deny check` (pass with existing duplicate/license-not-encountered
  warnings; no deny failures).
- Ran (targeted): `cargo test --test cli03_runner_contract_derived_tests cli03_fixture_run_publishes_wb16_ealpha_compatibility_seed_provenance` (pass).

## Intermediate Fixups

- Ran: initial `cargo fmt --check` reported formatting drift; fixed via
  `cargo fmt`.
- Ran: initial targeted test compile failed due `serde_json` usage in the
  integration test target; test updated to manifest-string assertions and
  rerun to pass.
