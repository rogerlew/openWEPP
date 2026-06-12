# D3 Increment Dj - Legacy `tmpadj` Surface Temperature

Status: landed; FDHP01 remains executed-hold

Evidence mode: Static + Ran

Date: 2026-06-12

## Objective

Execute Increment Dj from `d3-staged-increment-plan.md` without the comparator
subagent. Di localized the remaining Dg forced-snow plateau residual to missing
legacy `hr_tmp`/`tmpadj` surface-temperature synthesis in the frost top
heat-flow path. Dj ports that source-line-owned seam and reruns the native and
forced-snow cohorts.

The comparator subagent was not used per user quota direction. Cohort
execution and comparisons ran locally with the release CLI plus local
Pandas/PyArrow reductions.

## Implementation

- `SC-SNOWFREEZE-001` is amended to v68 with
  `REF-SNOWFREEZE-LEGACY-TMPADJ`.
- Active frost now synthesizes `frost.hourly.surface_temp_c_####` from the
  pinned `tmpadj.for` path before computing top heat flow:
  hourly winter air temperature, radiation, cloud fraction, wind, albedo,
  canopy/roughness, snow depth/density, residue depth/conductivity, and current
  frost/thaw geometry feed the legacy net-radiation/turbulent/conductive
  balance.
- The legacy positive-under-snow cap is preserved: computed surface
  temperature above `0 degC` is capped to `0 degC` when snow depth exceeds
  `0.001 m`.
- Runtime projection now emits `winter.hourly.air_temp_c_####`,
  `winter.hourly.rad_mj_m2_####`, and
  `winter.hourly.cloud_fraction_####` whenever frost processing is enabled by
  `frost.options.wintRed` or runtime frost state, even on warm/no-snow days.
- `frost.hourly.surface_temp_c_####` is registered in the unit catalog and
  published through the hourly frost writeback seam.

Temporary forced-snow diagnostic hooks were used only for the controlled cohort
and were removed before final source validation.

## Validation

Ran:

- `cargo fmt --check` - pass.
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_uses_frost_option_trigger_on_warm_days` - pass.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_dj_ -- --nocapture` - pass, `3/3`.
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract --test hphys0320_stmtim_start_time_source_line_contract` - pass, `8/8` after the `SC-SNOWFREEZE-001` v68 version bump.

Full workspace and authority gates are recorded in `gate-results.md`.

## Cohort Results

Native production cohort:

- Root: `/tmp/fdhp01_increment_dj_native_cohort_20260612T205827Z`.
- Execution: `43/43` clean exits, `43/43` WAT outputs.
- Years 2-6 independent `Total-Soil + frozwt` closure:
  `6.17207992173463e-07 mm` max abs residual (`p11`, year 6).
- Depth/duration remains open: mean max depth `506.7933035417255 mm`, median
  max `497.8858706468047 mm`, range
  `426.97573085329185..615.1578103266359 mm`, `30/43` prefixes inside the
  `240..503.2 mm` envelope, median depth correlation
  `0.7630792145889135`, and median frozen-duration residual `+72` days.

Forced legacy-snow diagnostic cohort:

- Root: `/tmp/fdhp01_increment_dj_forced_snow_cohort_20260612T205827Z`.
- Forcing CSV:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/legacy_snow_forcing_daily_hour24.csv`.
- Execution: `43/43` clean exits, `43/43` WAT outputs.
- Years 2-6 independent `Total-Soil + frozwt` closure:
  `5.09157033201646e-07 mm` max abs residual (`p11`, year 4).
- Dj does not pass the forced-snow material-improvement gate. Relative to Dg
  forced snow, mean max depth regresses
  `490.0923199552928 -> 501.3624240499244 mm`, median max regresses
  `479.356967770298 -> 492.3588252690888 mm`, and the outlier set above
  `503.2 mm` is unchanged at `13/43`: `p1`, `p2`, `p3`, `p8`, `p11`, `p13`,
  `p20`, `p21`, `p22`, `p23`, `p26`, `p28`, `p32`.
- Duration improves but does not close acceptance: median open-minus-legacy
  frozen-duration residual moves `+73 -> +61` days.

## Artifacts

- `fdhp01_increment_dj_native_run_status_20260612.tsv`
- `fdhp01_increment_dj_native_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dj_native_depth_metrics_20260612.csv`
- `fdhp01_increment_dj_native_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dj_native_activation_summary_20260612.csv`
- `fdhp01_increment_dj_native_execution_summary_20260612.json`
- `fdhp01_increment_dj_forced_snow_run_status_20260612.tsv`
- `fdhp01_increment_dj_forced_snow_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dj_forced_snow_depth_metrics_20260612.csv`
- `fdhp01_increment_dj_forced_snow_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dj_forced_snow_activation_summary_20260612.csv`
- `fdhp01_increment_dj_forced_snow_execution_summary_20260612.json`

## Disposition

Dj lands the source-line-owned `tmpadj` surface-temperature seam and preserves
D2 storage closure at WAT-publication texture, but FDHP01 remains
`executed-hold` because maximum-depth acceptance did not improve under forced
legacy snow.

The next bounded increment should localize the post-Dj maximum-depth residual
with direct legacy `tmpadj`/`frostn` evidence for surface temperature, top heat
flux, and front advance on the unchanged forced-snow outlier set. Do not retune
snow depth/density, `kfactor`, latent heat, WAT publication, D2 storage,
residue depth, `dpfsfl`, fixed `kftill`/`kfutil`, or lower-front `Qdry`.
