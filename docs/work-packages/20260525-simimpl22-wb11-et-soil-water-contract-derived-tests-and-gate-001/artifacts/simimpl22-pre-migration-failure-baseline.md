# SIMIMPL22 Pre-Migration Failure Baseline

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Objective: confirm SIMIMPL22 contract-derived vectors fail on pre-migration
  runtime behavior before SIMIMPL23 production migration begins.

## Ran
- Command: `cargo test -p openwepp --test wb11_hydrology_kernel_contract -- --ignored --nocapture`
- Result: failed as expected (`0 passed; 4 failed`).
- Observed blocking failures:
  - `simimpl22_contract_stage_memory_vector_requires_transitioning_s1_s2_tu_tv`
    - panic: `baseline-authoritative ET stage-memory transition was not observed`
  - `simimpl22_contract_root_uptake_vector_requires_upi_ui_etp_and_ws_lineage`
    - panic: `missing expected flux symbol UPi`
  - `simimpl22_contract_wb11_ordering_vector_requires_purk_before_evap`
    - panic: `baseline WB11 ordering requires purk/percolation before evap/evappm`
  - `simimpl22_contract_wb13_publication_vector_requires_watcon_alias_lineage`
    - panic: `missing expected state symbol watcon`
- Control run:
  - `cargo test -p openwepp --test wb11_hydrology_kernel_contract`
  - Result: pass (`3 passed; 0 failed; 4 ignored`).
