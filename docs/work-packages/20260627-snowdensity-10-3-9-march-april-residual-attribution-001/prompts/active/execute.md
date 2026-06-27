# Execute SNOWDENSITY-10.3.9

Execute
`docs/work-packages/20260627-snowdensity-10-3-9-march-april-residual-attribution-001/`
autonomously.

Constraints:

- Diagnostic-only. Do not change production snow/frost physics, defaults,
  selectors, parser/runfile/user surfaces, public output schemas, fixtures,
  coefficients, radiation, canopy, phase partition, density, melt, rain heat,
  longwave, or frost code.
- Use the SNOWDENSITY-10.3.8 opt-in coupled WAT report as the residual baseline.
- Do not count observation-blocked surfaces as verdict-bearing residual surfaces.
- Attribute March/April residuals before recommending the next one-lever package.
- Close with JSON/Markdown artifacts, focused guard tests, strategy-doc
  disposition, gate results, and explicit remaining blockers.
