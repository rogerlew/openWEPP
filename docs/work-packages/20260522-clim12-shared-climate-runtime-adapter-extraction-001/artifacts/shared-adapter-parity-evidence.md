# Shared Adapter Parity Evidence

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Added integration parity assertion that compares hillslope runtime climate symbols to watershed `hs{ID}_*` symbols generated from the same climate/day input.
- Reused canonical strict climate fixture to avoid fixture drift in parity evidence.

Ran:
- `cargo test --workspace climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path -- --exact`
  - observed: `test climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path ... ok`
- `cargo test --workspace`
  - observed: integration suite includes and passes `climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path`.

## Parity Method
1. Parse one strict climate payload.
2. Build hillslope climate runtime surface for day 0.
3. Build watershed climate runtime surface for a single assignment (`hillslope_id=7`) for day 0.
4. Assert `nclimhs=1` and exact numeric equality (`abs diff < 1e-12`) for each hillslope runtime symbol and its watershed-prefixed counterpart.

## Outcome
- Symbol-level parity for shared adapter projection is demonstrated for the strict daily climate path.
- No projection drift detected between hillslope and watershed adapter consumers in executed parity test.

## Residual Hold
- Broader taxonomy/governance normalization (including cross-surface error-code harmonization) remains follow-on scope (`CLIM15`, `CLIM16`).

## Evidence
- `tests/integration/parser_runtime_seam_integration.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
