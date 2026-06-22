# Anti-Alias Fixture

Status: executed.

## Fixture

- Unit fixture:
  `r7d2_direct_seed_authority_is_lane_indexed_for_multiofe_profiles`.
- The fixture builds a `DirectPublicationDayInputBuilder` with two synthetic
  lane seed surfaces carrying intentionally different direct profile operands.

## Evidence

- Ran:
  `cargo test -p openwepp-runner r7d2_direct_seed_authority_is_lane_indexed_for_multiofe_profiles`
  passed.
- Static: the test asserts lane 1 and lane 2 differ for
  `profile_depth_m`, `profile_porosity_cap_m`, `profile_field_capacity_m`, and
  `profile_wilting_point_m`.
- Static: `DirectPublicationDayInputBuilder::profile_inputs(lane_index)` and
  `seed_surface_authority(lane_index)` now choose lane-indexed seed authority
  when more than one seed surface exists and fail closed on out-of-range lane
  access.
- Static: H2637 samples after the change show lane-varying direct ET/profile
  operands rather than one cloned aggregate profile, so the remaining public
  output residual is not the original seed/profile alias.
