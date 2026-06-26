# Review Agent A

Evidence mode: Static + Ran.

## Findings

1. Accepted/fixed: the diagnostic replay initially reconstructed daily snow
   input with a hard-coded `0.1` factor. Current fixtures have `newsnw=100`,
   but the bridge should not depend on that. The replay now carries
   `snow_water_m` from `precip_mass_mm * snow_precip_fraction / 1000`.

2. Accepted/fixed: the opt-in replay exposed an albedo cold-start boundary for
   same-day snow after snow-free morning hours. The typed helper now preserves
   opt-in albedo state when future same-day snowfall is present; 05D missing
   active-state fail-closed coverage remains green.

3. No issue: the replay is diagnostic-only. `CoeShortwaveAlbedoV1` is not wired
   into production default selection, parser surfaces, or output schemas by this
   package.

## Residual Risk

The diagnostic replay is a snowbench tool, not the production runfile selector.
05F must decide whether the opt-in API and cold-start state policy are ready for
broader runtime exposure.
