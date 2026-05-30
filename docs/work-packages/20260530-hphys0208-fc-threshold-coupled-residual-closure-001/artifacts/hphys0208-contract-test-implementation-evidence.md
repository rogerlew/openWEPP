# HPHYS0208 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Contract-derived test additions
- Static: `tests/integration/hphys0208_fc_threshold_coupled_residual_contract.rs`
  - Added package/contract authority presence test.
  - Added sat-perturbation execution test asserting coupled WB13 publication
    response and `SoilWaterTotal = Total-Soil + frozwt` closure.
- Static: `crates/openwepp-runner/src/hillslope/mod.rs` unit tests
  - Added `hphys0208_wb11_seed_uses_sat_por_cpm_layer_lineage`.
  - Added `hphys0208_wb11_seed_hard_fails_missing_cpm_symbol`.
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Extended normalized overlap mapping expectations to include `por`/`cpm`
    lineage projection and `sat` projection assertions.

## Test registration closure
- Static: `Cargo.toml`
  - Added explicit `[[test]]` target for
    `hphys0208_fc_threshold_coupled_residual_contract` so workspace test gates
    execute the new integration contract test.

## Targeted execution evidence
- Ran: `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract` -> pass
  - `hphys0208_package_and_contract_authority_sections_exist`
  - `hphys0208_sat_perturbation_changes_coupled_wb13_publications`
- Ran: `cargo test -p openwepp-runner hphys0208_` -> pass
  - `hphys0208_wb11_seed_uses_sat_por_cpm_layer_lineage`
  - `hphys0208_wb11_seed_hard_fails_missing_cpm_symbol`
