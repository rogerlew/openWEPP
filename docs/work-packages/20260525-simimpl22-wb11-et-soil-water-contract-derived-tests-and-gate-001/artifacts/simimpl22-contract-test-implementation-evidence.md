# SIMIMPL22 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Implemented four SIMIMPL22 contract-derived vectors in
  `tests/integration/wb11_hydrology_kernel_contract.rs`:
  - `simimpl22_contract_stage_memory_vector_requires_transitioning_s1_s2_tu_tv`
  - `simimpl22_contract_root_uptake_vector_requires_upi_ui_etp_and_ws_lineage`
  - `simimpl22_contract_wb11_ordering_vector_requires_purk_before_evap`
  - `simimpl22_contract_wb13_publication_vector_requires_watcon_alias_lineage`
- Vectors are intentionally `#[ignore]` with explicit migration-block reason
  strings tied to SIMIMPL23 closure.
- Added helper accessors to keep symbol lookup failures typed and explicit:
  - `require_state_scalar(...)`
  - `require_flux_scalar(...)`

## Ran
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract --no-run`
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract -- --ignored --nocapture`
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract`
