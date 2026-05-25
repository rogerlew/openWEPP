# SIMIMPL23 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL22 migration vectors are now enabled in default suite execution by
  removing `#[ignore]` from:
  - `simimpl22_contract_stage_memory_vector_requires_transitioning_s1_s2_tu_tv`
  - `simimpl22_contract_root_uptake_vector_requires_upi_ui_etp_and_ws_lineage`
  - `simimpl22_contract_wb11_ordering_vector_requires_purk_before_evap`
  - `simimpl22_contract_wb13_publication_vector_requires_watcon_alias_lineage`
- Contract-vector fixture updates made to align with migrated runtime order and
  guard semantics:
  - stage-memory vector `tu` seed adjusted to valid in-domain value,
  - uptake vector explicitly seeds `lai` so `Etp` lineage assertions are
    meaningful,
  - WB17 ET physics fixture per-layer WB18 values adjusted for
    percolation-before-ET ordering.

## Ran
- `git diff -- tests/integration/wb11_hydrology_kernel_contract.rs`
- `git diff -- tests/integration/wb17_et_physics_kernel_contract.rs`
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract`
- `cargo test -p openwepp --test wb17_et_physics_kernel_contract`
