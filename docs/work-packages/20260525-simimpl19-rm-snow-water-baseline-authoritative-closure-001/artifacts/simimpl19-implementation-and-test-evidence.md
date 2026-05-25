# simimpl19-implementation-and-test-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Updated `crates/openwepp-runner/src/lib.rs` to:
  - carry runtime SWE publication state across days,
  - derive `Snow-Water` from runtime SWE progression,
  - derive `RM` from `prcp + SWE_before - SWE_after + Irr`,
  - retain `Total-Soil` as full-profile semantics (legacy `watcon` lineage)
    and annotate that this is not top-layer `TSW`.

## Ran
- `cargo fmt --check` passed.
- `cargo test simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage -- --nocapture` passed.
- `cargo test simimpl18_contract_requires_multi_day_storage_state_mutation -- --nocapture` passed.
