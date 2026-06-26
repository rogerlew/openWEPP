# Worker Handoff

Status: complete.
Evidence mode: Static.

Next recommended package: `SNOWDENSITY-05B Shortwave Source Binding`.

First actions for 05B:

- Identify the canonical gridded daily shortwave/radiation source openWEPP can
  consume without duplicating wepppy orchestration.
- Prove units, slope/aspect transformation, hourly distribution, and
  provenance into `winter.hourly.rad_mj_m2_####`.
- Bind the result to `SC-CLIMATE-001#INV-CLIMATE-013`.
- Reject raw Langleys/day, already-MJ double conversion, clipping, fitted
  radiation scalars, and site-specific radiation tuning.

Constraints to carry forward:

- Do not implement `coe_shortwave_albedo_v1` production melt until 05B and 05C are complete.
- Preserve the signed `melt_bmelt_in` convention.
- Do not promote `dense_slow_melt_v1`.
- Keep `legacy_coe` default and rollback.
