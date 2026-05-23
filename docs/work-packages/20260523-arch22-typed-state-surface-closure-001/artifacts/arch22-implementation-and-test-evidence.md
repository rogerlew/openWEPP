# ARCH22 Implementation and Test Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Implementation Summary
- Added ARCH22 typed symbol families in
  `crates/openwepp-kernel-contract/src/lib.rs`:
  - `HillslopeIrrigationDepletionPeriodField`
  - `HillslopeIrrigationFixedDateEventField`
  - `HillslopeProductionStateSymbol`
  - `HillslopeProductionFluxSymbol`
  - `WatershedChannelStateField`
  - `WatershedChannelFluxField`
  - `WatershedImpoundmentStateField`
  - `WatershedImpoundmentFluxField`
  - `WatershedProductionStateSymbol`
  - `WatershedProductionFluxSymbol`
  - plus `From<...> for BoundarySymbol` mappings.
- Migrated covered production hillslope guard/accessor surfaces in
  `crates/openwepp-hillslope-orchestrator/src/lib.rs` to typed symbols:
  - `require_state_scalar`, `require_flux_scalar`,
    `optional_state_scalar`, `optional_flux_scalar`,
    `require_state_range`, `require_flux_range`,
    `optional_state_non_negative_integral`.
  - irrigation indexed builders now consume typed fields
    (`HillslopeIrrigationFixedDateEventField::*`,
    `HillslopeIrrigationDepletionPeriodField::*`).
- Migrated covered production watershed guard/accessor/writeback surfaces in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` to typed symbols:
  - `require_state_scalar`, `require_flux_scalar`,
    `require_state_range`, `require_flux_range`.
  - replaced raw string formatting for channel/impoundment/hillslope payload
    symbols with typed `WatershedProduction*Symbol` variants.
- Added contract-derived integration test target:
  - `tests/integration/arch22_typed_state_surface_contract.rs`
  - wired in root `Cargo.toml`.

## Targeted Contract/Test Runs
```bash
cargo test --test arch22_typed_state_surface_contract
cargo test --test wb11_hydrology_kernel_contract
cargo test --test ws10_watershed_kernel_contract
cargo test --test parser_runtime_seam_integration
```

Results:
- `arch22_typed_state_surface_contract`: pass (`6 passed`).
- `wb11_hydrology_kernel_contract`: pass (`3 passed`).
- `ws10_watershed_kernel_contract`: pass (`4 passed`).
- `parser_runtime_seam_integration`: pass (`45 passed`).

Logs:
- `artifacts/test-logs/01-arch22-typed-state-surface-contract.log`
- `artifacts/test-logs/02-wb11-hydrology-kernel-contract.log`
- `artifacts/test-logs/03-ws10-watershed-kernel-contract.log`
- `artifacts/test-logs/04-parser-runtime-seam-integration.log`

## Required Repository Gates
```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Results:
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`).
  - note: allowlist `license-not-encountered` warnings are non-fatal.

Gate logs:
- `artifacts/gate-logs/01-cargo-fmt-check.log`
- `artifacts/gate-logs/02-cargo-clippy-workspace.log`
- `artifacts/gate-logs/03-cargo-test-workspace.log`
- `artifacts/gate-logs/04-cargo-deny-check.log`
