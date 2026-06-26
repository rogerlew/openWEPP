# Implementation Evidence

Static:

- Added `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`.
- Exported `SnowMeltModel`, `SnowAlbedoModel`, `SnowAlbedoState`,
  `SnowAlbedoUpdateInputs`, `SnowAlbedoUpdateOutcome`, `SnowAlbedoError`, and
  `update_snow_albedo_state`.
- `SnowMeltModel::LegacyCoe` returns inactive output and does not require
  albedo state.
- `SnowMeltModel::CoeShortwaveAlbedoV1` requires the accepted albedo model id
  and a valid prior state unless fresh snowfall resets the state.
- Inputs fail closed on non-finite values, negative snow/SWE/age increments,
  out-of-range underlying albedo, model mismatch, or missing required state.

No production routed-melt call site was changed.
