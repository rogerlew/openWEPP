# HPHYS0233 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static

## Static

Updated contract-derived vectors in:
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`

Added vectors:
1. `wb18_contract_conformance_daily_restrictive_layer_harmonic_conductivity_reduces_bottom_flux`
   - verifies daily bottom-layer restrictive branch (`slflag=1`,
     `kslast > 0`) reduces `pei` relative to unrestricted conductivity.
2. `wb18_contract_conformance_rejects_non_positive_kslast_when_slflag_enabled`
   - verifies typed hard-fail posture for invalid restrictive conductivity.

Added anti-shadow regression vector in runner publication path:
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - `hphys0233_wb13_dp_publication_prefers_flux_surface_over_stale_state_surface`.
