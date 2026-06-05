# Contract Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `SC-WATBAL-001#INV-WATBAL-078` as the HPHYS0305 paired
  fixed-baseline/openWEPP melt-term/state gate.
- Ratified HPHYS0305 trace aliases for `snow_hourly_rain_m`,
  `snow_hourly_snowfall_depth_m`, snow hourly depth/density maps, melt-term
  maps, and winter hourly forcing maps.
- `SC-SNOWFREEZE-001` already contained melt-term/state authority through
  `INV-SNOWFREEZE-033`; no snow-physics equation change was needed.

Ran:

- Static contract/test guard confirmed by
  `cargo test --test hphys0305_paired_melt_term_state_contract -- --nocapture`.
