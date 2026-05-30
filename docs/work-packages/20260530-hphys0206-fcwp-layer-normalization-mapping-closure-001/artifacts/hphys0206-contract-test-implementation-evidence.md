# HPHYS0206 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Contract-derived test additions
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Added `hphys0206_authoritative_theta_uses_normalized_overlap_mapping`.
  - Added `hphys0206_soil_runtime_surface_fail_closed_when_normalized_correction_input_missing`.
  - Added normalized-overlap oracle helper using legacy layer normalization +
    corrected-layer lineage.
- Static: `tests/integration/parser_runtime_seam_integration.rs`
  - Added
    `soil_runtime_surface_rejects_missing_normalized_corrected_lineage_input`
    to enforce typed fail-closed runtime seam behavior.
- Static: `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - Extended contract-authority presence assertions to include HPHYS0206
    canonical addenda.
  - Updated invalid-layer-state guard coverage to assert
    `HS-RUNTIME-E-060` fail-closed behavior.

## Targeted test execution
- Ran: `cargo test -p openwepp-hillslope-orchestrator hphys0206_` -> pass.
- Ran: `cargo test --test parser_runtime_seam_integration soil_runtime_surface_rejects_missing_normalized_corrected_lineage_input` -> pass.
- Ran: `cargo test --test hphys0202_profile_fc_wp_lineage_contract` -> pass.
