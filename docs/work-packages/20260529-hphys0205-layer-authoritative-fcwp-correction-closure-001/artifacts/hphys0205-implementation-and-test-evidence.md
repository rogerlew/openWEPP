# HPHYS0205 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production implementation
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - Introduced corrected-layer theta projection for authoritative layer symbols.
  - Added helper paths:
    `compute_corrected_layer_theta_symbols`,
    `legacy_correct_layer_moisture`.
  - Kept WB13 publication layer-authoritative while reconciling
    `wb13_profile_fc_store_mm` / `wb13_profile_wp_store_mm` to authoritative
    layer aggregates (`Σ(thetfc_i*dg_i)*1000`, `Σ(thetdr_i*dg_i)*1000`).
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Added corrected-lineage and reconciliation contract tests.
- Static: `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - Added HPHYS0205 corrected-layer lineage assertion.
- Static: `tests/integration/parser_runtime_seam_integration.rs`
  - Updated seam assertions to corrected-lineage semantics.

## Workspace gates
- Ran: `cargo fmt --check` -> pass
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- Ran: `cargo test --workspace` -> pass
- Ran: `cargo deny check` -> pass (warnings only)

## Diagnostic rerun evidence (MEASURE-HP205-004)
- Ran: 39-hillslope execution batch (`openwepp-cli-hill`) succeeded (`39/39 rc=0`):
  `/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_batch_status.tsv`
- Ran: semantic comparator batch succeeded for all `39/39` hillslopes:
  `/tmp/hphys0205_20260530T022235Z/parity/reports/semantic_status.tsv`
- Ran: summary generated:
  `/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`
  - `semantic_pass_count=0`
  - `semantic_fail_count=39`
  - `ProfileFCStore`: `39/39` fail hillslopes
  - `ProfileWPStore`: `39/39` fail hillslopes
