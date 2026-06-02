# Pre-Implementation Contract Gate

Status: complete

Evidence mode: ran

Ran:

- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
- Log: `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/gate-logs/pre_impl_wb17_contract_test.log`
- Result: failed as expected before production edits.
- Passing before the new tests: `5`.
- Failing HPHYS0249 vectors: `2`.

Failure signal:

- `hphys0249_wb17_soil_evaporation_mutates_layer_storage_before_aggregate_writeback`
  failed because WB17 did not publish `wb18_perc_theta_0001`.
- `hphys0249_wb17_root_uptake_mutates_layer_storage_and_stress_from_swu_lineage`
  failed because WB17 did not publish `wb18_perc_theta_0001`.
