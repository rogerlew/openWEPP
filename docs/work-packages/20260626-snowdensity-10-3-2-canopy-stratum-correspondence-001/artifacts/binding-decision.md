# Binding Decision

Status: complete.

Evidence class: Static.

Decision: `CURRENT-SINGLE-MIXED-HILLSLOPE-NOT-STRATUM-BOUND`.

The current Harvard and Marcell fixtures do not satisfy the 10.3.2 closure
condition for canopy-stratified verdicts. They are single mixed-forest
hillslopes with current runtime `cancov = 0.55`, while the observations are
stratified into open and under-canopy classes.

## Authorized Uses After 10.3.2

Allowed:

- Use `harvard_mixed_ma` and `marcell_mixed_mn` as single mixed-hillslope
  diagnostics.
- Use them to prove runtime/export plumbing and no-regression gates.
- Use them as planning anchors for future canopy-variant work.

Not allowed:

- Treat the current Harvard/Marcell mixed-hillslope outputs as observed
  `open`, `hardwood`, `hemlock`, `deciduous`, or `conifer` strata.
- Use Harvard/Marcell current mixed-hillslope comparisons as decisive
  canopy-attenuation verdicts.
- Tune melt, albedo, density, radiation, or canopy parameters against a
  stratum-resolved target without a stratum-bound model surface.

## Required Route Before Canopy Verdicts

One of these must happen before Harvard or Marcell carry canopy verdicts:

1. Generate paired model variants for each observed stratum:
   `open`, `deciduous/hardwood`, and `conifer/hemlock`.
2. Author a site-specific aggregation rule with explicit observed stratum
   weights and bind the current mixed model only to that aggregate.

Preferred route for 10.3.3 gradient melt adjudication: generate paired
open/deciduous-or-hardwood/conifer-or-hemlock variants, because the forcing
robust signal of these sites is the within-site canopy ordering.

## Carry-Forward To 10.3.3

10.3.3 may proceed on the broader canopy gradient only if it:

- excludes Harvard/Marcell from stratum-verdict cells until variants or
  aggregation exist; or
- scopes Harvard/Marcell to static mixed-hillslope diagnostic context; or
- first adds the paired model variants required above.
