# Consumer Path Evidence

Status: **PASS**.

## Consumer Path

Static:

| Stage | Evidence |
| --- | --- |
| Producer | `DirectDayFrame.wb14_hourly_rainfall_m` is produced at R4K; `DirectDayFrame.evapotranspiration_compute_inputs.leaf_area_index` is the post-growth ET/growth state surface; typed management projection retains `canhgt` on `DirectProductionEvapotranspirationAuthority`. |
| Frame handoff | `DirectFrameExecutor::run_publication_stream_with_interleaved_day_inputs_and_day_frames` passes the executed `DirectDayFrame` to the stream consumer after day spans run and before commit. |
| Runner builder | `DirectProductionDayInputBuilder::laned_shadow_lane_day_operands(day_frame)` validates hourly rainfall, post-growth LAI, and `canhgt` policy, then returns `LanedShadowLaneDayOperands`. |
| Shadow collector | `LanedShadowCollector::observe_row(row, operands)` buffers the dynamic operands per lane-day with the routed source-depth series. |
| Solver consumer | `commit_day()` sets `CellParameters.leaf_area_index` and `canopy_height_m`, converts `hourly_rainfall_m` to a rate series with `seam_source_rates_from_hourly_depths`, and passes that sampler as `CascadeForcing.rainfall_intensity_m_s`. |
| Negative proof | Source guard search found no `let intensity = |_ofe: usize, _t: f64| 0.0` placeholder in the active shadow collector; existing static routing-coefficient guard still rejects missing native extension data. |

Ran:

- `rg -n "let intensity = \\|_ofe|k_o = 500|I=0|LAI=0|h_c=0|laned_shadow_lane_day_operands|hourly_rainfall_m" ...`
  found the new dynamic operand path and no old shadow `I=0` closure.
- `cargo test -q -p openwepp-runner laned_shadow` passed with `6` tests,
  including vegetation-active fail-closed tests and a routed nonzero-intensity
  cascade differential test; see `test-implementation-evidence.md`.
