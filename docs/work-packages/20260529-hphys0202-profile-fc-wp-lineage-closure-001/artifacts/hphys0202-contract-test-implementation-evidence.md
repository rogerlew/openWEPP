# HPHYS0202 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Contract-derived test surfaces
- Static: Added `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`.
  - Asserts HPHYS0202 package and SC contract authority sections exist.
  - Executes an end-to-end runner fixture lane where seed symbols diverge from
    per-layer aggregation and asserts WB13 publication follows layer
    aggregation (`thetfc/thetdr * dg`) rather than seed overrides.
  - Asserts invalid layer-storage state hard-fails at upstream `wb11_seed`
    guards with typed failure metadata.
- Static: Added WB13 direct guard probes in
  `crates/openwepp-runner/src/hillslope/mod.rs` unit tests:
  - `hphys0202_wb13_fc_seed_guard_is_exercised_by_direct_row_builder_probe`
  - `hphys0202_wb13_wp_seed_guard_is_exercised_by_direct_row_builder_probe`
  - `hphys0202_wb13_profile_fc_wp_publication_ignores_seed_values_when_valid`
  These cover the WB13 publication guard branch directly and protect against
  future refactors that might bypass FC/WP guard/type-state checks.

## Execution evidence
- Ran: `cargo test --workspace` -> pass
  - Includes `hphys0202_profile_fc_wp_lineage_contract` (`3` tests passed).
  - Includes runner WB13 probe unit tests (`3` tests passed).
