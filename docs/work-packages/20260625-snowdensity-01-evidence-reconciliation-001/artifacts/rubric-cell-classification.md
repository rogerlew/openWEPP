# Rubric Cell Classification

Evidence mode: Static.

Authority: `SC-SNOWFREEZE-001#INV-SNOWFREEZE-050`,
`TOL-SNOWFREEZE-011`.

This classification defines what SNOWDENSITY-02 should treat as in-scope for an
opt-in snow physics candidate. It does not change the rubric or thresholds.

## Actionable Snow-Physics Candidate Cells

These cells can plausibly improve from a bulk snowpack physics model without
site-specific constants:

- **Mean cold-season bulk density (`R`)**: direct target for state-evolved
  density.
- **Seasonal densification trajectory `rho(t)` (`R`)**: direct target for
  metamorphism, overburden compaction, and wet-snow compaction.
- **Depth-SWE seasonal slope (`R`)**: direct diagnostic of whether density/depth
  relation is physically shaped.
- **Bias-sign consistency (`R`)**: useful cross-cutting diagnostic for persistent
  density/depth bias.
- **Rain-on-snow response (`R`, currently unavailable in H/I0)**: future direct
  target for retained liquid/release/refreeze behavior.
- **Post-storm settling rate / mid-winter melt response (`mixed`)**: future
  target once event-paired data is curated.
- **Conservation (`R`, currently external-observation-unavailable)**: hard
  internal gate for the opt-in solver.

## Mixed / Conditional Cells

These cells are partly snow physics and partly forcing or representation:

- **Snow-cover duration and inter-annual variability (`R`)**: affected by melt,
  cold content, liquid release, and phase, but also by precipitation timing and
  gridded forcing.
- **Accumulation onset and build-up rate (`R`)**: affected by fresh-snow density
  and phase partition, but highly sensitive to precipitation/temperature forcing.
- **Peak SWE/depth date (`R` date; `L` magnitude)**: timing may improve through
  snow thermal/melt physics; absolute magnitude remains forcing-limited.
- **Ablation/melt-out date and rate (`R`)**: affected by melt physics, cold
  content, albedo, and forcing.
- **Regime ordering across snow climates (`R`)**: a useful summary after the
  candidate exists, not a formula-selection input by itself.

## Forcing-Limited Cells

These cells should be reported but not used as standalone production-defect or
calibration targets:

- **Mean peak SWE bias (`L`)**.
- **Mean peak depth bias (`L`)**.
- Absolute depth/SWE magnitude residuals where DAYMET/CLIGEN station mismatch,
  wind redistribution, lapse, precipitation undercatch, and point-vs-hillslope
  representativeness dominate.

## Future-Data / Currently Unavailable Cells

- **New-snow density per storm (`R`)**: not scored by daily SNOTEL corpus; useful
  for fresh-snow-density formula validation if event data is acquired.
- **Rain-on-snow event response (`R`)**: requires event windows and confident
  phase/forcing pairing.
- **External conservation (`R`)**: external observations do not reconstruct model
  mass/energy closure; the solver must provide internal closure evidence.

## SNOWDENSITY-02 Scope Decision

SNOWDENSITY-02 should contract the following first-candidate scope:

- state-evolved SWE/depth/density;
- bulk cold content or bulk snow temperature;
- temperature-dependent fresh-snow density candidate family;
- densification candidate family, with Anderson-1976/SNOBAL as leading
  candidate pending ratification;
- liquid retention/release/refreeze obligations;
- internal mass/energy closure gates;
- no per-site constants.

It should explicitly defer wind redistribution, precipitation undercatch
correction, watershed spatial snow distribution, and PySnobal hardening.
