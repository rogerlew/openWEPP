# Worker Handoff

Status: complete.
Evidence mode: Static.

Next recommended package: `SNOWDENSITY-05C Albedo State Core`.

First actions for 05C:

- Ratify the opt-in albedo state and model id before code.
- Choose temperature/age albedo constants from cited authority, not from
  SNOTEL-site fitting.
- Define fresh-snow reset, age/temperature decay, domain bounds, missing-state
  fail-closed behavior, and rollback interaction with `legacy_coe`.

Constraints to carry forward:

- Do not implement `coe_shortwave_albedo_v1` production melt until 05C is complete.
- Preserve the 05B radiation-source binding.
- Use the daily climate `rad`/`radly` authority and the single
  `radmj = radly * 0.04184` conversion path.
- Do not tune, rescale, clip, or reinterpret shared radiation forcing for snowmelt.
- Do not add a snow-only radiation scalar.
- Preserve the signed `melt_bmelt_in` convention.
- Do not promote `dense_slow_melt_v1`.
- Keep `legacy_coe` default and rollback.
