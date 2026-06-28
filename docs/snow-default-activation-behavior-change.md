# Behavior Change — Default Snow Melt/Density Model Activated

- Date: 2026-06-27
- Scope: `openwepp-cli-hill` direct-production snow physics (default behavior)
- Authority: `SC-SNOWFREEZE-001` v101 `INV-SNOWFREEZE-072`; work-package
  `docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001/`
- Rollback: available (see below)

## What changed

The **default** direct-production snow model changed from the legacy CoE
melt + legacy WEPP density path to the validated opt-in bundle:

- `snow_melt_model = coe_liquid_holding_capacity_v1`
  (liquid-water holding-capacity drainage, replacing the `350 kg m^-3`
  density-gate liquid-export proxy)
- `snow_density_model = physics_bulk_density_compaction_v1`
  (Anderson/SNOBAL-lineage bulk density compaction)

This is now the behavior when the package-bound selector environment variables
are **absent**. The active snow density cap remains `522 kg m^-3`. No parser,
runfile, public output schema, or user CLI surface changed.

## Why (and why it is not a regression)

The bundle is the best validated snow-depth path to date: paired observed
snow-depth control failures dropped from `1147` to `498` over `1415`
Sleepers/Harvard rows, with no paired surface worse than the prior
holding-capacity-only step. The change is **mass-conserving** (composite
snow-state closure `runtime_swe = runtime_depth_m * runtime_density_kg_m3 / 1000`
holds at machine epsilon), so no water is created or destroyed.

## What this means for your outputs

**Snow-affected downstream outputs will differ from prior defaults.** Because the
melt timing/magnitude and the snowpack density/depth now track observations more
closely, the meltwater that the legacy path retained is released and routed
downstream. Total water is **conserved** ("the water has to go somewhere"), but
the **partition and timing** of:

- surface runoff vs infiltration,
- water-balance terms,
- erosion / sediment,
- watershed routing

**change** on any run that carries snow. This is improved-input propagation, not
a regression: the snow signal driving these consumers is closer to observed than
before. If you have a project calibrated against the prior default snow behavior,
expect snow-affected results to move.

## Evidence scope (stated honestly)

Activation was gated on the Policy-B **workspace-suite** no-regression check (the
existing test suite passes under the bundle selectors) plus composite snow-state
conservation closure, **not** on a separate bundle-vs-legacy diff of
snow-affected erosion / water-balance / watershed outputs. The activation commit
changed no downstream output goldens. The activation basis is therefore
**conservation + improved snow input + reversibility**, not a demonstrated
downstream output-diff. A future package may characterize the downstream deltas
explicitly if a calibration-grade comparison is needed.

## Known residual / not yet closed

- `498/1415` paired snow-depth rows still fail the snow-control gate.
- **Frost attribution remains blocked** by `SNOW-CONTROL-RESIDUALS-REMAIN`.
- The residual is now roughly two-sided (`264` over- / `234` under-persistence);
  most under-persistence is a known mechanism cost of the bulk-compaction arm.
- The `550 kg m^-3` SNOBAL cap re-anchor is follow-up only (projection mixed; a
  dynamic implementation and rerun would be required).

## Rollback

The prior default behavior is preserved as explicit rollback/test selectors
(diagnostic environment variables, not user CLI/runfile config):

```
OPENWEPP_SNOWDENSITY1038_MELT_MODEL=legacy_coe
OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=legacy_wepp
```

Set both to restore the legacy CoE melt + legacy WEPP density default. Unknown or
unsupported selector values fail closed; empty values select the activated
defaults.
