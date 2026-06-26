# Worker Handoff

Next recommended package: `SNOWDENSITY-05D Opt-In CoE Melt Implementation`

05D should wire `coe_shortwave_albedo_v1` into the existing production CoE
melt-term path only after consuming the 05A sign convention, 05B shortwave
source binding, and 05C albedo-state core.

Required constraints:

- Consume `SnowAlbedoState` only when `snow_melt_model = coe_shortwave_albedo_v1`.
- Preserve `legacy_coe` default behavior.
- Do not alter the 05B radiation-source binding.
- Preserve signed `melt_bmelt_in` semantics.
- Do not tune, rescale, clip, or reinterpret shared radiation forcing for
  snowmelt.
- Do not promote `dense_slow_melt_v1`.
- Prove independent reconstruction of raw melt, redistributed melt, `wmelt`,
  SWE loss, and WB12/WB13 liquid forcing before accepting routed melt.

05C residual risks:

- `Ta` increment source and update timing must be made explicit at the 05D
  call site.
- Active-snow missing-state errors must surface as typed opt-in failures, not
  silent fallback to legacy melt.
