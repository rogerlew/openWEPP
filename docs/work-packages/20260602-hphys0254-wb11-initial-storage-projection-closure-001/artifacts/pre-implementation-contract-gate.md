# Pre-Implementation Contract Gate

Status: complete

Evidence mode: ran

Ran:

- `cargo test --test wb11_storage_projection_kernel_contract -- --nocapture`
- `cargo test --test wb18_percolation_physics_kernel_contract hphys0254_wb18_lower_layer_over_ul_uses_legacy_stu_cap -- --nocapture`

Expected failures:

- WB11 seed projection test failed before production correction because hydrology seed depth was not represented by the normalized corrected layer grid.
- WB18 lower-layer cap test failed before production correction because finite lower-layer over-UL ratios hard-failed instead of following the baseline `stu >= 0.95` cap.

Interpretation:

- The red tests were contract-derived and targeted the defects corrected by this package.
