# CQR15 CRAP After

Status: complete.

Ran:

```bash
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/artifacts/crap_after.json
```

Ran: final after LCOV completed and wrote `artifacts/lcov_after.info`.

Ran: final after CRAP completed and wrote `artifacts/crap_after.json`. It
emitted the recurring workspace warning that 125 source files had no matching
LCOV entry; CRAP JSON was still emitted.

Ran: target-file coverage after:

- Lines: `1424/2011`, `70.81%`
- Functions: `79/108`, `73.15%`

Ran: target and new helper closure rows:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `seed_wb11_runtime_surface_inputs` | 6 | 15.0 | 100.0 | 15.0 |
| `validate_wb11_layer_transport_scalars` | 396 | 7.0 | 31.11111111111111 | 23.01930315500686 |
| `derive_wb11_layer_seed_stores` | 471 | 9.0 | 48.333333333333336 | 20.171624999999995 |
| `validate_wb19_lateral_and_drainage_inputs` | 623 | 9.0 | 48.57142857142857 | 20.017889212827985 |
| `apply_wb11_layer_saturation_floor` | 442 | 6.0 | 53.57142857142857 | 9.602951895043732 |
| `accumulate_wb11_hyetograph_rainfall` | 170 | 7.0 | 62.5 | 9.583984375 |
| `seed_wb11_hyetograph_inputs` | 104 | 9.0 | 93.33333333333333 | 9.024000000000001 |
| `validate_wb11_layer_storage_geometry` | 343 | 4.0 | 33.33333333333333 | 8.740740740740742 |
| `require_wb11_layer_seed_inputs` | 300 | 7.0 | 100.0 | 7.0 |
| `seed_initial_wb11_storage_if_needed` | 242 | 7.0 | 100.0 | 7.0 |
| `initial_wb11_saturation` | 279 | 6.0 | 73.68421052631578 | 6.656072313748361 |
| `seed_wb11_lane_substep_controls` | 64 | 6.0 | 100.0 | 6.0 |
| `resolve_wb11_seed_nsl` | 43 | 5.0 | 66.66666666666666 | 5.925925925925927 |
| `seed_wb16_ealpha_compatibility` | 783 | 5.0 | 100.0 | 5.0 |
| `require_positive_wb19_drain_geometry_scalar` | 685 | 3.0 | 40.0 | 4.944 |
| `validate_wb19_drain_geometry` | 676 | 4.0 | 62.5 | 4.84375 |
| `seed_wb11_efflen_if_missing` | 762 | 4.0 | 100.0 | 4.0 |
| `validate_wb11_layer_seed_inputs` | 333 | 4.0 | 100.0 | 4.0 |
| `require_nonnegative_wb11_soil_water_for_reconciliation` | 594 | 3.0 | 57.14285714285714 | 3.7084548104956268 |
| `require_nonnegative_wb11_prcp` | 91 | 3.0 | 66.66666666666666 | 3.333333333333334 |
| `resolve_wb19_drain_enabled_flag` | 659 | 3.0 | 100.0 | 3.0 |
| `seed_wb11_optional_default_symbols` | 609 | 3.0 | 100.0 | 3.0 |
| `validate_wb11_layer_storage_order` | 380 | 2.0 | 53.333333333333336 | 2.4065185185185185 |
| `seed_wb12_reconciliation_runtime_inputs` | 701 | 2.0 | 100.0 | 2.0 |
| `synthesize_zero_point_wb11_hyetograph` | 141 | 2.0 | 100.0 | 2.0 |
| `wb11_runtime_state_is_seeded` | 269 | 2.0 | 100.0 | 2.0 |
| `publish_wb11_initial_seed_totals` | 560 | 1.0 | 100.0 | 1.0 |
| `publish_wb11_layer_seed_stores` | 536 | 1.0 | 100.0 | 1.0 |

Static: scoped closure is satisfied. The target and every newly extracted CQR15
helper are CRAP `<= 30`.

Static: out-of-scope rows still above 30 in the target file are not closed by
CQR15:

- `produce_wb16_ealpha_from_runtime_surface`: CRAP `317.2103869084884`
- `execute_scheduler_kernel_lifecycle`: CRAP `54.66251538901941`
- `pl_runtime_has_active_crop_for_scheduler_day`: CRAP `46.39958249299923`
- `refresh_wb18_frozen_depth_from_fine_frost_state`: CRAP `35.38643604915795`
- `pl_crop_slot_is_active_for_day`: CRAP `33.11738193219675`
