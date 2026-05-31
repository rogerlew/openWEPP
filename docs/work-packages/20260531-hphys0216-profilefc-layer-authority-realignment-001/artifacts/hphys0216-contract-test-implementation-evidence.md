# HPHYS0216 Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Contract-derived tests updated
1. `crates/openwepp-runner/src/hillslope/mod.rs`
   - `hphys0216_wb13_fc_storage_guard_rejects_missing_layer_authority_symbol`
   - `hphys0216_wb13_profile_fc_publication_uses_layer_aggregation_authority`
   - updated perturbation test to drive `thetfc_0001` authority path.
2. `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
   - updated FC/WP publication assertions to HPHYS0216 split authority:
     FC from layer aggregation; WP from projected storage symbol.
   - added HPHYS0216 contract-section presence checks.

## Ran commands
1. `cargo test -p openwepp-runner hphys0216_ -- --nocapture` (pass)
2. `cargo test -p openwepp --test hphys0202_profile_fc_wp_lineage_contract` (pass)
3. `cargo test --workspace` (pass)
