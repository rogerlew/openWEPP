# SIMIMPL22 Contract-Derived Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
| test id | location | contract authority | expected pre-migration posture |
|---|---|---|---|
| `simimpl22_contract_stage_memory_vector_requires_transitioning_s1_s2_tu_tv` | `tests/integration/wb11_hydrology_kernel_contract.rs` | `SC-EVAP-001` (`INV-EVAP-013`), `SC-WATBAL-001` stage-memory lineage obligations | fail (stage-memory transition surfaces not yet migrated) |
| `simimpl22_contract_root_uptake_vector_requires_upi_ui_etp_and_ws_lineage` | `tests/integration/wb11_hydrology_kernel_contract.rs` | `SC-EVAP-001` + `SC-PLANT-001` uptake/stress lineage obligations | fail (missing `UPi`/`Ui` lineage surfaces) |
| `simimpl22_contract_wb11_ordering_vector_requires_purk_before_evap` | `tests/integration/wb11_hydrology_kernel_contract.rs` | `SC-WATBAL-001` ordering authority | fail (baseline ordering not yet closed in runtime) |
| `simimpl22_contract_wb13_publication_vector_requires_watcon_alias_lineage` | `tests/integration/wb11_hydrology_kernel_contract.rs` | `SC-WATBAL-001` + `SC-SYSTEM-001` WB13 alias-lineage obligations | fail (missing `watcon` publication-lineage symbol) |

## Notes
- All four SIMIMPL22 vectors are intentionally marked `#[ignore]` in default
  suite execution and are enabled only by explicit `--ignored` invocation while
  migration remains open.

## Ran
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract --no-run`
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract -- --ignored --nocapture`
- `cargo test -p openwepp --test wb11_hydrology_kernel_contract`
