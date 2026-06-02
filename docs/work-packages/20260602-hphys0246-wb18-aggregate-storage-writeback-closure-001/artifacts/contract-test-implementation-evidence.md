# HPHYS0246 Contract Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Added Tests
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
  - `hphys0246_wb18_percolation_preserves_residual_storage_in_aggregate_soil_water`
    proves WB18 publishes `wb11_soil_water = Σ(theta_i + thetdr_i*dg_i)` for an
    unfrozen two-layer vector.
  - `hphys0246_wb18_percolation_requires_residual_storage_symbols_for_aggregate_writeback`
    proves WB18 hard-fails with `HKERNEL-WB11-PERC-E-001` when required
    residual-storage input is missing.

## Updated Existing Contract Fixtures
- `tests/integration/auth03_level4_constitutive_gate_contract.rs`
- `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`

These pre-existing one-layer WB18 vectors were updated with explicit
`thetdr_0001 = 0.0` and `dg_0001 = 1.0` (and equivalent layer-2 values where
needed). This preserves their flux-focused assertions while satisfying the new
aggregate-storage input contract.

## Ran
- `cargo test -p openwepp-hillslope-orchestrator hphys0246_wb18 -- --nocapture`
  - Pre-implementation: failed on the target invariant.
  - Post-implementation: passed, `2 passed`.
- `cargo test -p openwepp --test auth03_level4_constitutive_gate_contract auth03_relax_to_fc_kernel_vectors_cover_cutoff_and_positive_branch -- --nocapture`
  - Passed.
- `cargo test -p openwepp --test auth05_level4_constitutive_authority_hardening_contract auth05_relax_to_fc_requires_explicit_positive_branch_assertions -- --nocapture`
  - Passed.
- `cargo test -p openwepp --test wb18_percolation_physics_kernel_contract`
  - Passed, `11 passed`.
