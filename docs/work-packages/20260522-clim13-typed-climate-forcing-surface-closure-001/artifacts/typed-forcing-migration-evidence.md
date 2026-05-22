# Typed Forcing Migration Evidence

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- `openwepp-kernel-contract` now defines typed climate series symbol surfaces and explicit watershed/hillslope constructors.
- `openwepp-hillslope-orchestrator::runtime_inputs` now builds `HillslopeClimateRuntimeRequest` with precomputed per-day typed symbol surfaces.
- `openwepp-watershed-orchestrator::runtime_inputs` now builds `WatershedHillslopeClimateAssignment` with precomputed per-day typed symbol surfaces.
- Runtime seeding writes series values through typed symbol vectors (`insert_series_values`) rather than string-format loops.

Ran:
1. `cargo test --workspace`
- observed: full workspace pass, including:
  - `openwepp_kernel_contract` tests validating typed surface alias continuity.
  - hillslope and watershed runtime input test suites.
  - integration parity test `climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path`.

2. `cargo clippy --workspace --all-targets -- -D warnings`
- observed: pass.

3. `cargo fmt --check`
- observed: pass.

4. `cargo deny check`
- observed: pass with existing non-failing allowlist warnings.

## Residual Hold
- Typed climate forcing surfaces are closed for series key synthesis in current seam paths; broader taxonomy/governance normalization remains follow-on (`CLIM15`, `CLIM16`).
