# D3 Increment Dd — Legacy-Snow-Forced Frost Certification

Status: executed-hold

Evidence mode: Ran

Date: 2026-06-12

## Objective

Run the Dd discriminator from `d3-staged-increment-plan.md`: force only the
snow depth/density inputs consumed by openWEPP frost heat-flow resistance from
legacy `H*.winter.dat` while leaving openWEPP frost physics, publication, and
water-balance accounting live.

## Method

- Generated legacy daily winter output for all `43` algebraic-radium prefixes
  with `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`.
- Parsed hour-24 `snodpt` and `densgt` rows from `H*.winter.dat` into a
  diagnostic CSV keyed by `(prefix, simulation_year, julian_day)`, filling
  dates absent from winter output as zero snow depth/density.
- Added a temporary env-gated hook at the frost snow-depth/density read seam in
  `compute_active_frost_coupling`; ran the cohort with:
  - `OPENWEPP_FDHP01_DD_FORCED_SNOW_CSV=/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/legacy_snow_forcing_daily_hour24.csv`
  - `OPENWEPP_FDHP01_DD_PREFIX=pN`
- Removed the temporary hook after the forced cohort and rebuilt the production
  release CLI from clean source. The production binary hash after hook removal
  is `95491b24f36065c28f90ca7e55bfceb39cf14ac2c270ddfd207eb750a2e4a536`.

The comparator subagent was not used per user quota direction; all comparisons
were run locally with the release CLI plus PyArrow/Pandas.

## Evidence

- Legacy winter generation root:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd`
- Forced openWEPP cohort root:
  `/tmp/fdhp01_increment_dd_forced_snow_cohort_20260612T121500Z`
- Compact artifacts:
  - `fdhp01_increment_dd_legacy_winter_generation_20260612.json`
  - `fdhp01_increment_dd_legacy_snow_forcing_summary_20260612.json`
  - `fdhp01_increment_dd_run_status_20260612.tsv`
  - `fdhp01_increment_dd_annual_closure_residuals_20260612.csv`
  - `fdhp01_increment_dd_depth_metrics_20260612.csv`
  - `fdhp01_increment_dd_frozwt_frdp_ratio_20260612.csv`
  - `fdhp01_increment_dd_activation_summary_20260612.csv`
  - `fdhp01_increment_dd_execution_summary_20260612.json`

## Result

Dd does **not** certify F4 as the whole remaining D3 story.

- Execution: `43/43` clean, `43/43` WAT outputs.
- D2 preservation: years 2-6 independent `Total-Soil + frozwt` closure stayed
  at WAT-publication texture, max abs
  `6.726058817130287e-07 mm`; p43 year 2 is
  `-1.2079226507921703e-13 mm`.
- Snow forcing helped depth magnitude, but did not close the D3 envelope:
  mean max depth `856.817674502367 mm`, median `844.2352603016866 mm`,
  range `654.0796339074789..1427.3939006063285 mm`, and `0/43` prefixes
  inside the `240..503.2 mm` legacy envelope.
- Profile-bound pinning was removed under forced snow: `0/43` pinned,
  minimum margin `372.60609939367146 mm`.
- Timing/correlation improved but remains incomplete: median depth correlation
  `0.7118806632341061` versus Dc1 `0.6415921721982907`.
- Frozen duration remains too persistent: median open-minus-legacy delta
  `+502` days, versus Dc1 `+567` days.
- Days above `200 mm` remain high: full-WAT median `937` days.

## Disposition

Legacy snow depth/density is a material contributor, but the controlled forced
run leaves a frost-side residual. The next increment should keep the Dd forced
snow harness as the diagnostic setup and localize the remaining frost-side
depth/duration error with in-process hourly evidence: compare openWEPP
`frzflg`, `Qsrf`, `Quf`, surface temperature, snow/residue/frozen resistance,
front advance/retreat, and layer ice/liquid motion against legacy
`H*.winter.dat`/source-line state for the first material divergence. Do not
tune snow density/depth, kfactor, latent heat, or publication surfaces.
