# Worker Handoff

Evidence mode: Static plus Ran.

## Current State

SNOWDENSITY-10.3.4 is complete as a diagnostic-only maritime
over-accumulation package. The report disposition is
`PARTITION-THAW-FIRST`.

Paired Sleepers and Harvard surfaces all over-accumulate modeled snow depth.
HJ Andrews and Hubbard Brook have useful forcing diagnostics but are
observation-blocked until paired snow-depth observations are installed.

## Next Recommended Package

SNOWDENSITY-10.3.5 should focus on partition/thaw-window decomposition:

- reconstruct event windows where near-zero precipitation entered as modeled
  snow;
- compare modeled snowpack ablation during observed thaw windows;
- prove whether the existing CoE `dmelt` rain term is active or inactive before
  touching rain-on-snow heat;
- keep production physics opt-in or diagnostic-only until the event-window
  evidence is contract-bound.

## Do Not Do Next

- Do not tune precipitation to reduce snow depth.
- Do not promote `coe_shortwave_albedo_v1`.
- Do not add a rain-heat, longwave, density, partition, or frost production
  change without a new contract-first work-package.
- Do not label HJ Andrews or Hubbard Brook defective until paired snow
  observations are installed.
