# Shared Climate Runtime Adapter Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Extracted climate parser-to-runtime adaptation logic into a single owner crate: `openwepp-climate-runtime-adapter`.
- Rewired both orchestrators to consume shared APIs while preserving CLIM11 ownership boundaries.
- Removed duplicated adaptation/disaggregation implementations from both `runtime_inputs.rs` files.

Ran:
- Executed required CLIM12 gates (`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`) after rewiring.

## Ownership Preservation (CLIM11)
1. Shared implementation owner is now `crates/openwepp-climate-runtime-adapter/src/lib.rs`.
2. Hillslope seam authority remains in hillslope orchestrator boundary projection (`HS-CLIM-SEAM-001`).
3. Watershed seam authority remains in watershed orchestrator assignment projection (`WS-CLIM-SEAM-001`) with per-hillslope context mapping.
4. This extraction centralizes implementation only; it does not move climate-routing authority away from CLIM11 contracts.

## Shared Public Surface
- `build_climate_runtime_request(&ClimateFile) -> Result<SharedClimateRuntimeRequest, SharedClimateRuntimeInputError>`
- `select_day_forcing(&SharedClimateRuntimeRequest, usize) -> Result<&SharedClimateDailyForcing, SharedClimateRuntimeInputError>`
- `SharedClimateRuntimeRequest`
- `SharedClimateDailyForcing`
- `SharedNoBreakpointForcing`
- `SharedBreakpointForcing`
- `SharedClimateRuntimeInputError`

## Integration Contract
1. Hillslope runtime seam now delegates climate adaptation and day selection to shared APIs, then writes canonical hillslope boundary symbols.
2. Watershed runtime seam now delegates per-hillslope adaptation and day selection to shared APIs, then writes `hs{ID}_*` namespaced symbols.
3. Watershed preserves contextual error ownership by mapping shared errors into `WatershedClimateRuntimeInputError` variants with `hillslope_id` where applicable.

## Error Taxonomy Contract
- Shared crate emits climate seam errors `CLIM-RUNTIME-E-001..015`.
- Watershed retains existing local `EmptyClimateAssignments` handling (`CLIM-RUNTIME-E-012`) and contextual variants for assignment-level reporting.
- Taxonomy normalization beyond extraction boundary remains follow-on scope (`CLIM15`).

## Acceptance Check
1. Duplicated climate adaptation logic removed from both orchestrators: `met`.
2. Single shared owner crate introduced and wired to both orchestrators: `met`.
3. CLIM11 boundary/authority split preserved: `met`.

## Evidence
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `docs/work-packages/20260522-clim11-climate-ownership-boundary-reconciliation-001/artifacts/climate-ownership-boundary-contract.md`
