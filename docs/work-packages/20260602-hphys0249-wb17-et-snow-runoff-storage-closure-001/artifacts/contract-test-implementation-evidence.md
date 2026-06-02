# Contract-Test Implementation Evidence

Status: complete

Evidence mode: static

Static:

- Added `hphys0249_wb17_soil_evaporation_mutates_layer_storage_before_aggregate_writeback`
  in `tests/integration/wb17_et_physics_kernel_contract.rs`.
- Added `hphys0249_wb17_root_uptake_mutates_layer_storage_and_stress_from_swu_lineage`
  in `tests/integration/wb17_et_physics_kernel_contract.rs`.
- Added `hphys0249_wb17_residue_remainder_adds_back_to_top_layer_and_clears_interception`
  in `tests/integration/wb17_et_physics_kernel_contract.rs`.
- Added `hphys0249_wb17_soil_evaporation_aggregate_includes_residual_and_frozen_depth_terms`
  in `tests/integration/wb17_et_physics_kernel_contract.rs`.
- Added `hphys0249_wb17_soil_evaporation_depth_rationing_cap_limits_partial_layer_withdrawal`
  in `tests/integration/wb17_et_physics_kernel_contract.rs` after Claude Code
  review.
- The tests assert the HPHYS0249 contract obligation directly: WB17 must
  publish mutated `wb18_perc_theta_####` layer storage before aggregate
  `wb11_soil_water` writeback.

Ran:

- Final post-review `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
  passed `10/10`.
