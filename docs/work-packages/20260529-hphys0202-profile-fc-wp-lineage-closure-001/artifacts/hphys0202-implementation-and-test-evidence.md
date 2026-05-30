# HPHYS0202 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production implementation
- Static: `crates/openwepp-runner/src/hillslope/mod.rs`
  - WB13 publication now treats `wb13_profile_fc_store_mm` and
    `wb13_profile_wp_store_mm` as non-authoritative diagnostic seed symbols.
  - Publication values are always computed from per-layer aggregation:
    `Σ(thetfc_i * dg_i) * 1000` and `Σ(thetdr_i * dg_i) * 1000`.
  - WB13 guard checks for optional FC/WP seed symbols remain typed and
    fail-closed (`CLIHILL-E-011` with `SIMOUT-E-001` details).
- Static: `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - Added behavior-level lineage and upstream guard validation.
- Static: `Cargo.toml`
  - Registered integration target:
    `hphys0202_profile_fc_wp_lineage_contract`.

## Workspace gates
- Ran: `cargo fmt --check` -> pass
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- Ran: `cargo test --workspace` -> pass
- Ran: `cargo deny check` -> pass (warnings only)

## Diagnostic rerun evidence (MEASURE-HP202-004)
- Ran: 39-hillslope execution batch succeeded (`39/39 rc=0`):
  `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_batch_status.tsv`
- Ran: semantic comparator batch executed for all hillslopes (`39/39 rc=0`):
  `/tmp/hphys0202_20260530T003833Z/parity/reports/semantic_status.tsv`
- Ran: summary generated:
  `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_semantic_summary.json`
  - `semantic_pass_count=0`
  - `semantic_fail_count=39`
  - `total_common_rows=56979`
  - Failing columns in all hillslopes:
    `RM`, `Ep`, `Es`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`,
    `Snow-Water`, `Q`, `QOFE`, `ProfileFCStore`, `ProfileWPStore`.
