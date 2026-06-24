# Snow Lane Authority Proof

Status: COMPLETE.

## Static

Authority direction after this package:

| Surface | Source of truth | Evidence |
| --- | --- | --- |
| Day-zero lane seed | `DirectLaneConstructorInputs.winter_column.snow` | `seed_direct_production_lane_constructor_inputs` writes `lane_inputs.winter_column.snow = ...initial_snow_lane_state()`. |
| Legacy constructor bridge | `DirectWinterColumnState.snow` with `DirectSnowRuntimeCarry` fallback only when winter snow is empty | `DirectLaneFrame::from_constructor_inputs` prefers `inputs.winter_column.snow` and only migrates `snow_runtime_carry` when no winter snow state exists. |
| Day frame seed | `lane.winter_column.snow` | `DirectRunFrame::seed_day_frame` clones `lane.winter_column` into the day frame and regenerates the legacy mirror from winter snow before falling back to the mirror. |
| R4G state mutation | `DirectDayFrame.winter_column.snow` | `run_r4g_snow_coupling_span` writes `DirectSnowLaneState::from_runtime_values(...)` before updating the legacy mirror. |
| Lane commit | `day_frame.winter_column.snow` | `DirectLaneFrame::commit_day` clones the day winter column to the lane and regenerates the legacy mirror from that state. |
| Direct publication snow/frost read | `lane.winter_column.snow` | `current_snow_lane_state` reads `let lane_state = lane.winter_column.snow;`. |

Residual compatibility mirror:

- `DirectSnowRuntimeCarry` remains in `direct_runtime/00_core_frames.rs` frame
  surfaces and validation for unmigrated internal APIs.
- The runner production direct helper no longer imports or reads
  `DirectSnowRuntimeCarry`.

## Source Scans

Ran:

```bash
rg -n "lane\\.snow_runtime_carry|current_snow_runtime_carry|initial_snow_runtime_carry|snow_runtime_carry\\.map_or" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
```

Result: exit code `1`, no matches. The direct publication helper does not read
the stale lane snow carry or helper names.

Ran:

```bash
rg -n "lane\\.winter_column\\.snow|current_snow_lane_state|initial_snow_lane_state|snow_state_projected\\(snow_lane_state\\)|snow_lane_state\\.runtime_depth_m|snow_lane_state\\.runtime_density_kg_m3|snow_lane_state\\.runtime_swe_m" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
```

Result: matches show direct publication reading and propagating
`snow_lane_state`:

- `current_snow_lane_state(lane)` at line 573.
- `snow_lane_state.runtime_swe_m` for active snow forcing and winter hourly
  context.
- `lane.winter_column.snow` inside `current_snow_lane_state`.
- `snow_lane_state.runtime_depth_m` and
  `snow_lane_state.runtime_density_kg_m3` for frost prior-snow forcing.
- `snow_state_projected(snow_lane_state)` for projection status.

## Ran

- `cargo test -p openwepp-hillslope-orchestrator --lib r7g_ -- --nocapture`
  passed. Covered constructor precedence, legacy carry migration, R4G
  mutation into winter snow state, and executor commit back to lane state.
- `cargo test -p openwepp-runner --lib r7g_direct_production -- --nocapture`
  passed. Covered direct publication no-carry source scan and prior snowpack
  ordering.

## Non-Claims

This proof does not claim frost lane-state migration, output parity closure,
performance closure, default activation, or deletion of residual carry
surfaces.
