# Contract Test Implementation Evidence

Status: complete

Evidence mode: Static + Ran.

## Added Tests

Static: `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
adds:

- `wb14_contract_conformance_storage_limit_routes_excess_to_runoff`
- `wb14_contract_conformance_uses_percolation_published_infiltration`

The first test verifies that saturated top-two-layer storage routes event liquid
to runoff instead of same-pass infiltration. The second verifies that WB14
runoff consumes the WB18/percolation producer's same-pass infiltration when that
producer has already run.

## Targeted Test Runs

Ran:

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract wb14_contract_conformance_storage_limit_routes_excess_to_runoff -- --nocapture`
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract wb14_contract_conformance_uses_percolation_published_infiltration -- --nocapture`
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`

Result: all passed after tightening the producer-consumer trigger.
