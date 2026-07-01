# CRAP After

Evidence mode: Ran.

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --lcov --output-path /tmp/openwepp-row9-full-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row9-full-after.lcov --min 0 --format json --output /tmp/openwepp-crap-row9-full-after.json
jq -r '[.entries[] | select(.file | test("(direct_runtime/evapotranspiration|direct_runtime/runoff|direct_runtime/storage|direct_runtime/00_core_frames|direct_runtime/diagnostic_events|support_helpers_mod/typed_boundary)")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row9-full-after.json
```

Result:

- Row #9 owned production functions above CRAP 30: `0`.
- Full workspace functions above CRAP 30, all rows/scopes: `298`.
- The current `cargo crap` JSON shape still emits duplicate rows for these
  source entries; row #9 after-count remains `0` even before de-duplication.

Representative after values for the row #9 before-list:

| Function | Location | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `DirectEvapotranspirationPmetComputeInputs::compute` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs:663` | 27.0 | 100.00 | 27.00 |
| `compute_stage_soil_evaporation` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs:1464` | 14.0 | 77.61 | 16.20 |
| `DirectEvapotranspirationPmetComputeInputs::transpiration_storage_terms` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs:824` | 8.0 | 91.67 | 8.04 |
| `DirectEvapotranspirationPmetComputeInputs::evaporation_storage_terms` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs:781` | 8.0 | 97.44 | 8.00 |
| `validate_direct_snow_layers` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs:2020` | 10.0 | 78.05 | 11.06 |
| `maybe_write_r7h_storage_trace` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs:195` | 3.0 | 3.31 | 11.14 |
| `maybe_write_r7h_et_trace` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs:72` | 3.0 | 3.96 | 10.97 |
| `maybe_write_r7h_runoff_rebalance_trace` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:69` | 3.0 | 6.56 | 10.34 |
| `validate_direct_day_constructor_inputs` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs:1797` | 8.0 | 100.00 | 8.00 |
| `DirectDayFrame::compute_r4n_surface_et` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs:380` | 17.0 | 81.54 | 18.82 |
| `validate_direct_lane_constructor_inputs` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs:1676` | 26.0 | 88.14 | 27.13 |
| `validate_direct_frost_runtime_fine_layers` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs:2269` | 6.0 | 92.11 | 6.02 |
| `DirectLaneFrame::commit_day` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs:968` | 17.0 | 70.89 | 24.13 |
| `DirectDayFrame::rebalance_r4a_frost_projection_to_storage_target` | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1212` | 10.0 | 81.13 | 10.67 |

Disposition: PASS. Row #9 primary CRAP closure is complete without
ADR-0021 warnings.
