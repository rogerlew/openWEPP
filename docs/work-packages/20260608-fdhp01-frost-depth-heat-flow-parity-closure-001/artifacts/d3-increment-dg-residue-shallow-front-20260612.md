# D3 Increment Dg — Residue Path + Shallow-Front Minimum Resistance

Status: landed; FDHP01 remains executed-hold

Evidence mode: Ran

Date: 2026-06-12

## Objective

Execute Increment Dg from `d3-staged-increment-plan.md`: restore the two
legacy frost surface-resistance terms localized by Df.

1. Publish and propagate winter/frost residue depth (`resdep`) into
   `frost.runtime_residue_depth_m`.
2. Apply the legacy shallow-front minimum top-frozen conduction distance
   (`dpfsfl = dg(1) / nfine(1) / 2`) whenever the active below-freezing frost
   surface heat path is thinner than that minimum.

The comparator subagent was not used per user quota direction. Cohort
execution and comparisons ran locally with the release CLI plus
Pandas/PyArrow.

## Implementation

- `SC-SNOWFREEZE-001` is amended to v66 with
  `REF-SNOWFREEZE-LEGACY-WINTER-RESDEP` and Dg authority for `resdep/kres`
  surface resistance and the shallow-front `dpfsfl` floor.
- Management runtime projection now derives initial legacy cropland
  `resdep` from the same initial residue mass lineage used by
  `init1.for`/`res_dp.for` and publishes it to
  `frost.runtime_residue_depth_m`.
- The runner preserves that management-derived frost residue depth when
  static snow/frost sidecar surfaces are merged, preventing the frost default
  zero from overwriting the management lineage.
- The frost surface heat path includes residue resistance and floors the
  below-freezing shallow frozen-soil conduction path to the first fine-layer
  midpoint. Positive-temperature thaw paths are unchanged.
- Added Dg contract tests proving residue resistance suppresses the surface
  flux and the `dpfsfl` floor limits shallow-front flux even with zero
  residue.

## Validation

Ran:

- `cargo fmt --check` — pass.
- `git diff --check` — pass.
- `cargo test -p openwepp-hillslope-orchestrator management_runtime_surfaces_project_required_pl_controls_and_seeds` — pass.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` — pass, `42/42`.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass after the
  mechanical `.cloned()` to `.copied()` fix.
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract --test hphys0320_stmtim_start_time_source_line_contract` — pass after updating the expected `SC-SNOWFREEZE-001` version from `65` to `66`.
- `cargo test --workspace` — pass.
- `cargo deny check` — pass.
- `bash tools/release/check_authority_suite_antievasion.sh` — pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract` — pass.
- `wctl doc-lint --path docs` — pass, `1220` files, `0` errors, `0` warnings.
- Release build: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` — pass; production binary SHA
  `3275db431339402596a27a28d7976062eb4655771e9e159fdf929fa1410883ad`.

## Cohort Results

Native production cohort:

- Root: `/tmp/fdhp01_increment_dg_native_cohort_20260612T184601Z`.
- Execution: `43/43` clean exits, `43/43` WAT outputs.
- Years 2-6 independent `Total-Soil + frozwt` closure:
  `6.261351281899863e-07 mm` max abs residual (`p11`, year 6).
- Depth/duration improved materially from De native but remains open:
  mean max depth `498.08123930883653 mm`, median max
  `488.08620069478803 mm`, range
  `416.1483027612484..616.5385754860384 mm`, `30/43` prefixes inside the
  `240..503.2 mm` envelope, median depth correlation
  `0.7551022199950611`, and median frozen-duration residual `+84` days.

Forced legacy-snow diagnostic cohort:

- Root: `/tmp/fdhp01_increment_dg_forced_snow_cohort_20260612T185203Z`.
- Forcing CSV:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/legacy_snow_forcing_daily_hour24.csv`.
- Execution: `43/43` clean exits, `43/43` WAT outputs.
- Years 2-6 independent `Total-Soil + frozwt` closure:
  `5.835723933533821e-07 mm` max abs residual (`p11`, year 4).
- Dg passes its directional forced-snow gate: mean max depth drops from De's
  `655.9890274782282 mm` to `490.0923199552928 mm`; median max drops from
  `652.3375464029963 mm` to `479.356967770298 mm`; prefixes inside the
  envelope improve from `0/43` to `30/43`; and median frozen-duration residual
  improves from `+186` days to `+73` days.
- Full D3 acceptance remains open: `13` prefixes still exceed the
  `503.2 mm` upper envelope bound under forced legacy snow, with outliers
  concentrated in `p1`, `p2`, `p3`, `p8`, `p11`, `p13`, `p20`, `p21`, `p22`,
  `p23`, `p26`, `p28`, and `p32`.

The forced-snow diagnostic used a temporary env-gated hook at the frost
snow-depth/density read seam. The hook was removed before the final production
rebuild; the production binary SHA after removal is
`3275db431339402596a27a28d7976062eb4655771e9e159fdf929fa1410883ad`.

## Artifacts

- `fdhp01_increment_dg_native_run_status_20260612.tsv`
- `fdhp01_increment_dg_native_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dg_native_depth_metrics_20260612.csv`
- `fdhp01_increment_dg_native_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dg_native_activation_summary_20260612.csv`
- `fdhp01_increment_dg_native_execution_summary_20260612.json`
- `fdhp01_increment_dg_forced_snow_run_status_20260612.tsv`
- `fdhp01_increment_dg_forced_snow_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dg_forced_snow_depth_metrics_20260612.csv`
- `fdhp01_increment_dg_forced_snow_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dg_forced_snow_activation_summary_20260612.csv`
- `fdhp01_increment_dg_forced_snow_execution_summary_20260612.json`

## Disposition

Dg lands because it restores the Df-localized legacy surface-resistance
terms, preserves D2 closure at WAT-publication texture, and passes the
directional forced-snow gate without duration regression.

FDHP01 remains `executed-hold`: the package D3 acceptance boundary is not
closed because the controlled cohort still has `13/43` prefixes outside the
legacy maximum-depth envelope and median frozen-duration residual remains
`+73` days. The next increment should localize the residual on the forced-snow
outlier set rather than retuning snow, WAT publication, D2 storage, or the
Dg residue/`dpfsfl` resistance terms.
