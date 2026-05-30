# HPHYS0205 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Contract-derived test surfaces
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Added corrected-layer authority assertions for 9002/7778 projection vectors.
  - Added `hphys0205_corrected_layer_fc_wp_aggregate_matches_projected_profile_seeds`.
- Static: `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - Added
    `hphys0205_layer_authority_projects_corrected_fc_wp_lineage_not_raw_parser_theta`.
  - Updated FC/WP publication contract assertions to enforce authoritative layer
    aggregation/reconciliation posture without seed-authority fallback.
- Static: `tests/integration/parser_runtime_seam_integration.rs`
  - Updated seam probes to assert corrected-lineage theta publication and reject
    raw parser-theta assumptions.

## Execution evidence
- Ran: `cargo test --workspace` -> pass
  - Includes the HPHYS0205 integration test and updated seam/runtime-input
    suites.
