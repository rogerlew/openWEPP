# ARCH22 Migration Proof Tests Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Added Contract-Derived Test Target
- `tests/integration/arch22_typed_state_surface_contract.rs`
- Registered in `Cargo.toml` as:
  - `name = "arch22_typed_state_surface_contract"`

## Contract Vector Coverage
1. `arch22_hillslope_static_symbol_projection_matches_authority`
- validates canonical projection for covered static hillslope symbols.

2. `arch22_hillslope_dynamic_irrigation_symbol_projection_matches_authority`
- validates indexed typed irrigation projection for depletion and fixed-date
  schedules.

3. `arch22_watershed_node_scoped_symbol_projection_matches_authority`
- validates node-scoped watershed channel/impoundment symbol projection.

4. `arch22_watershed_hillslope_payload_symbol_projection_matches_authority`
- validates hillslope contributor symbol projection (`hs{id}_peakro`,
  `hs{id}_watdur`).

5. `arch22_hillslope_guard_accessor_signature_is_typed`
- asserts covered hillslope production accessors no longer use raw
  `&'static str` symbol signatures.

6. `arch22_watershed_guard_accessor_signature_is_typed`
- asserts covered watershed production accessors no longer use raw `&str`
  symbol signatures.

## Pre-Implementation Gate (Phase 2)
Command run before production ARCH22 migration code edits:
```bash
cargo test --test arch22_typed_state_surface_contract
```

Observed pre-implementation result: **failed at compile stage** with `E0432`
unresolved imports for not-yet-implemented ARCH22 typed symbols:
- `HillslopeIrrigationDepletionPeriodField`
- `HillslopeIrrigationFixedDateEventField`
- `HillslopeProductionFluxSymbol`
- `HillslopeProductionStateSymbol`
- `WatershedChannelStateField`
- `WatershedImpoundmentStateField`
- `WatershedProductionFluxSymbol`
- `WatershedProductionStateSymbol`

Interpretation:
- Contract tests were authored and executed before production migration symbols
  existed, satisfying the contract-first pre-implementation gate.

## Post-Implementation Test Result
Command:
```bash
cargo test --test arch22_typed_state_surface_contract
```

Result: pass (`6 passed; 0 failed`).

Log:
- `artifacts/test-logs/01-arch22-typed-state-surface-contract.log`
