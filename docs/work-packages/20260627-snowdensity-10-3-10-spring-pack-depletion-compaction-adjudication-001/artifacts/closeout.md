# Closeout

Status: complete

Evidence class: Static + Ran

## Result

SNOWDENSITY-10.3.10 generated a diagnostic-only cap-feasibility report from the
SNOWDENSITY-10.3.8 opt-in coupled direct-production WAT artifact and the
March/April paired snow-depth observations used by SNOWDENSITY-10.3.9. No
production physics, defaults, selectors, fixtures, public schemas, coefficients,
radiation, canopy, phase partition, density, melt, rain heat, longwave, frost, or
density cap changed.

## Findings

- Density cap used: existing `SC-SNOWFREEZE-001` cap `522 kg m^-3`.
- March/April paired rows: `463`.
- March/April failures: `282`.
- Compaction-only feasible failures: `190/282`.
- Depletion-required failures: `49/282`.
  - `CAP_LIMITED_DEPLETION_REQUIRED`: `33`
  - `PATCHY_MELTOUT_OR_DEPLETION_REQUIRED`: `16`
- Under-persistence failures: `43/282`.
- Diagnostic row-summed SWE depletion required at cap: `1.2300507182249083 m`.
- Depletion-required rows by cover: hardwood `5`, open `23`, open field `21`.

## Disposition

The leading March/April lever is compaction/densification, not spring mass
depletion. A majority of failures can fit within the observed depth tolerance if
modeled SWE is compacted under the existing `522 kg m^-3` cap. The depletion tail
is real and should be preserved as a later one-lever package if compaction does
not clear the gate, but it should not be the first correction.

Recommended next process:
`SPRING-COMPACTION-DENSIFICATION-CANDIDATE`.
