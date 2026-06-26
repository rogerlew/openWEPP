# SNOWDENSITY-05B Kickoff Prompt

Read the root and local agent instructions, then execute
SNOWDENSITY-05B as a contract/source-binding package.

Required decisions:

- Identify the canonical radiation source openWEPP owns for future
  `coe_shortwave_albedo_v1` melt work.
- Bind units, slope/aspect transformation, hourly distribution, and ET shared
  authority.
- Amend `SC-SNOWFREEZE-001` before any implementation claim.
- Add a focused contract guard.

Hard boundaries:

- Do not implement production melt.
- Do not add a gridded-provider selector inside openWEPP.
- Do not add a snow-only radiation scalar.
- Do not tune, rescale, clip, or reinterpret shared radiation forcing for snow.
- Preserve `legacy_coe` default and rollback.
