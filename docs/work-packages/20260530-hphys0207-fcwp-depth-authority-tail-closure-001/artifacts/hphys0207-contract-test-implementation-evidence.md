# HPHYS0207 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Contract-derived test additions
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Added normalized-profile storage expectation helper from corrected-layer
    lineage.
  - Added `hphys0207_profile_fc_wp_projection_preserves_normalized_depth_authority`.
  - Updated profile-storage assertions to enforce divergence from parser-layer
    depth aggregation when normalized-tail exists.
- Static: `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - Added HPHYS0207 package/contract authority assertions.
  - Added
    `hphys0207_profile_storage_projection_differs_from_parser_layer_depth_aggregation`.
  - Updated publication check so WB13 row must follow
    `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm`.
- Static: `tests/integration/parser_runtime_seam_integration.rs`
  - Added seam assertions that projected profile FC/WP storage is finite,
    ordered, and not silently truncated to parser-layer aggregate depth.
- Static: `crates/openwepp-runner/src/hillslope/mod.rs` unit tests
  - Added HPHYS0207 WB13 guard probes for FC/WP storage symbol domain failures.
  - Added HPHYS0207 WB13 publication authority test consuming storage symbols.

## Targeted execution evidence
- Ran: `cargo test --workspace` -> pass, including:
  - `hphys0207_profile_fc_wp_projection_preserves_normalized_depth_authority`
  - `hphys0207_profile_fc_wp_publication_uses_projected_storage_symbols`
  - `hphys0207_profile_storage_projection_differs_from_parser_layer_depth_aggregation`
  - `hphys0207_wb13_fc_storage_guard_is_exercised_by_direct_row_builder_probe`
  - `hphys0207_wb13_wp_storage_guard_is_exercised_by_direct_row_builder_probe`
