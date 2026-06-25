# Pre-Implementation Evidence

Evidence mode: Static + Ran.

## Starting State

- SNOWFROST-FIDELITY-D published WAT `Snow-Depth` from
  `snow.runtime_depth_m` without changing snow/frost physics.
- D reran all five observed sites and produced `0` defect-attribution eligible
  sites.
- Sites 1, 2, and 4 fail paired snow-depth control under `TOL-SNOWFREEZE-009`;
  sites 3 and 5 lack paired observed snow-depth rows.
- D classified the residual family as snow-confounded rather than frost-model
  defective.

## Static Correspondence Clues

- `tests/fixtures/snowfreeze_observed/observations/manifest.json` records
  Sleepers snow-depth values as centimeters and GGD498 station 10 snow as
  `snow_cm`.
- `tests/fixtures/snowfreeze_observed/README.md` states that snow depth and
  density, not SWE, govern insulation.
- WAT `Snow-Water` is SWE and cannot satisfy snow-depth control.

## Execution Boundary

This package may add correspondence authority and diagnostic/audit tooling. It
must not change production snow/frost physics or tune residuals.
