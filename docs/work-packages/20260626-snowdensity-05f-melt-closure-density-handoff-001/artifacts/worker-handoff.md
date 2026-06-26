# SNOWDENSITY-05F Worker Handoff

Status: complete.

Next recommended package: `SNOWDENSITY-06 Density Compaction`.

## Accepted Boundary

- `legacy_coe` remains the default and rollback path.
- Consume `coe_shortwave_albedo_v1` only as a fixed opt-in melt boundary.
- Required operands: `snow_melt_model`, `winter.hourly.rad_mj_m2_####`,
  `snow_albedo`, `snow_albedo_model_id`,
  `snow_albedo_accumulated_positive_temperature_c_day`,
  `snow_albedo_fresh_snow_reset_water_equiv_m`,
  `snow_melt_shortwave_absorbed_fraction`, raw melt, redistributed melt, routed
  `wmelt`, SWE loss, WB12 `S`, WB13 liquid forcing, and runtime
  SWE/depth/density after-state.

## Guardrails

- Do not retune melt, albedo, coefficients, or shared radiation for density.
- Do not use `coe_shortwave_albedo_v1` as compensation for missing overburden or
  metamorphism compaction.
- Preserve the same-day future snowfall cold-start albedo policy: fresh-snow
  reset, valid previous opt-in carry, or typed fail-closed.
- Report both 05E diagnostic replay and H as-built context before any default-candidate claim.
- Do not add parser/runfile/CLI selectors, output-schema changes, compatibility
  deletion, default activation, or snow-influenced parity re-baselines in the
  density package.

## Residual Risks

- 05E improved diagnostic legacy but did not beat H as-built context.
- Non-SNOTEL frost attribution remains blocked by snow-control failures.
- Density compaction must prove forcing-robust improvement without melt
  retuning before any activation route can be considered.
