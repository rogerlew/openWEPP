# HPHYS0234 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static

## Static

Updated contract-derived stale-state-vs-flux conflict vector in:
- `crates/openwepp-runner/src/hillslope/mod.rs`

Added vector:
1. `hphys0234_wb13_subhyd_publication_prefers_flux_surface_over_stale_state_surface`
   - injects conflicting state/flux values for `q`, `Qdd`, `Qd`,
   - requires WB13 row assembly to succeed with flux-authoritative values,
   - asserts `latqcc` and `Tile` outputs match flux-surface magnitudes, which
     also guards `Qd` coupling closure against stale-state shadowing.

Validation command (Ran in package gate stack):
- `cargo test --workspace`
