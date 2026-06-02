# HPHYS0239 Implementation and Test Evidence

Status: completed  
Evidence mode: mixed (`Static` + `Ran`)

## Production Implementation

1. `crates/openwepp-runner/src/hillslope/mod.rs`
   - Updated WB13 hydrology publication reads to flux-authoritative resolution:
     - `Q`: `require_runtime_surface_scalar_prefer_flux("Q")`
     - `Ep`: `require_runtime_surface_scalar_prefer_flux("Ep")`
     - `Es`: `require_runtime_surface_scalar_prefer_flux("Es")`
     - `Er`: `require_runtime_surface_scalar_prefer_flux("Er")`
   - Existing WB13 anti-shadow coverage for `D`/`q`/`Qdd`/`Qd` remains intact.

## Validation Execution

Ran:
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract hphys0239_contract_wb11_hydrology_tail_order_requires_wb19_then_wb12_reconciliation`
- `cargo test -p openwepp-runner hphys0239_wb13_hydrology_publication_prefers_flux_surface_over_stale_state_surface`

Result:
- New ordering vector test passed.
- New WB13 anti-shadow vector for `Q`/`Ep`/`Es`/`Er` passed after production
  update.
