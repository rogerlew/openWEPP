# CQR24 CRAP After

Status: complete.

Ran: `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr24-scheduler-seed-runtime-complexity-001/artifacts/lcov_after.info`.

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr24-scheduler-seed-runtime-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr24-scheduler-seed-runtime-complexity-001/artifacts/crap_after.json`.

Static: final target result from `crap_after.json`:

- Function: `produce_wb16_ealpha_from_runtime_surface`
- Line: `957`
- Cyclomatic complexity: `6.0`
- Coverage: `93.33333333333333`
- CRAP: `6.010666666666666`

Static: newly extracted WB16 helpers are all below CRAP `30`:

- `wb16_resolve_canhgt`: `15.401920438957477`
- `wb16_validate_surface_nonnegative`: `14.98582533150434`
- `wb16_compute_frcteq`: `9.55241288817612`
- `wb16_validate_canopy_nonnegative`: `9.466145833333334`
- `wb16_produce_ofe_alpha`: `9.041472`
- `wb16_equivalent_plane_alpha`: `8.974857142857145`
- `wb16_ofe_geometry`: `8.125`
- `wb16_ofe_surface_controls`: `8.0`
- `wb16_normalized_width`: `6.166666666666666`
- `wb16_normalize_ofe_controls`: `5.00063606757582`
- `wb16_ofe_canopy_controls`: `5.0`
- `wb16_ealpha_powers`: `4.84375`
- `wb16_validate_ealpha`: `4.460555972952667`
- `wb16_compute_ofe_alpha`: `3.474609375`
- `wb16_ealpha_ofe_count`: `3.4099854227405246`
- `wb16_compute_frlive`: `3.333333333333334`
- `wb16_resolve_rrc`: `3.333333333333334`
- `wb16_validate_finite_ofe_values`: `3.0987654320987654`
- `wb16_publish_ofe_alpha`: `2.0`
- One-way publication helpers are at CRAP `1.0`.

Static: same-file non-target CRAP rows above closure threshold after refactor:

- `execute_scheduler_kernel_lifecycle`: CRAP `54.66251538901941`
- `pl_runtime_has_active_crop_for_scheduler_day`: CRAP `46.39958249299923`
- `refresh_wb18_frozen_depth_from_fine_frost_state`: CRAP `35.38643604915795`
- `pl_crop_slot_is_active_for_day`: CRAP `33.11738193219675`

Static: those rows are pre-existing and out of CQR24 scope. They remain WARNs
for subsequent CQR rows, not CQR24 closure blockers.
