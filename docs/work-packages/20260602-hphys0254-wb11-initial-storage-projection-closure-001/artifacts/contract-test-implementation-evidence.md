# Contract-Test Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `tests/integration/wb11_storage_projection_kernel_contract.rs`.
- Registered the new integration test in `Cargo.toml`.
- Extended `tests/integration/wb18_percolation_physics_kernel_contract.rs` with `hphys0254_wb18_lower_layer_over_ul_uses_legacy_stu_cap`.
- Updated parser/runtime and runner tests to assert hydrology alias separation.

Ran:

- Pre-implementation `cargo test --test wb11_storage_projection_kernel_contract -- --nocapture` failed on normalized seed depth, proving the pre-patch defect.
- Pre-implementation WB18 targeted contract test failed with the existing hard-fail lower-layer over-UL path.
- Post-implementation focused tests passed:
  - `cargo test --test parser_runtime_seam_integration -- --nocapture`
  - `cargo test -p openwepp --test auth07_fc_authority_cohort_contract -- --nocapture`
  - `cargo test -p openwepp --test auth05_level4_constitutive_authority_hardening_contract -- --nocapture`
  - `cargo test --test wb11_storage_projection_kernel_contract -- --nocapture`
  - `cargo test --test hphys0202_profile_fc_wp_lineage_contract -- --nocapture`
  - `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`
