# Execute SNOWDENSITY-10.3.10

Execute
`docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/`
autonomously.

Constraints:

- Diagnostic-only. Do not change production snow/frost physics, defaults,
  selectors, parser/runfile/user surfaces, public output schemas, fixtures,
  coefficients, radiation, canopy, phase partition, density, melt, rain heat,
  longwave, or frost code.
- Use the existing `SC-SNOWFREEZE-001` `522 kg m^-3` density cap; do not invent a
  new cap or fit a cap to observations.
- Base residuals on the SNOWDENSITY-10.3.8 opt-in coupled WAT candidate
  `coe_liquid_holding_capacity_v1`.
- Do not count observation-blocked surfaces as verdict-bearing residual surfaces.
- Close with JSON/Markdown artifacts, focused guard tests, strategy-doc
  disposition, gate results, and explicit remaining blockers.
