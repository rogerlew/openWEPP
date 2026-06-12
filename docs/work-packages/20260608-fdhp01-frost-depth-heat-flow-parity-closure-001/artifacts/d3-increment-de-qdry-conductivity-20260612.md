# D3 Increment De — Content-Dependent `qdry` Conductivity

Status: executed-hold

Evidence mode: Ran

Date: 2026-06-12

## Objective

Execute Increment De from `d3-staged-increment-plan.md`: replace the
unconditional `0.2 W/m/K` lower-front conductivity fallback with the legacy
content-dependent harmonic-mean path from `frostn.for:430-458`, while keeping
the same conductivity authority in the `mltbtm` bottom-thaw energy path.

## Implementation

- `SC-SNOWFREEZE-001` is amended to v65 with
  `REF-SNOWFREEZE-LEGACY-FROSTN-QDRY` and De authority for
  `tmpbl`/`Qdry`: per-fine-layer
  `k = (0.5096 + 7.4493*slsw - 8.7484*slsw^2) *
  (0.0014139*bdcons - 1.0588) * ksoilf`, harmonic aggregation over the
  metre below the lower front, and `0.2 W/m/K` only when no positive
  conductivity term exists.
- Published parser-derived bulk density into the WB19 runtime layer surface
  as `wb19_bulk_density_kg_m3_####` and `bulk_density_kg_m3` for the first
  layer, with a strict `(0, 2650] kg/m3` frost-seam guard.
- Rebuilt `lower_front_heat_w_m2` and `thaw_fine_bottom_with_resistance_feedback`
  so both use the same content-dependent lower-front conductivity.
- Added contract-derived tests for moist/dry conductivity behavior and
  autumn freeze-onset suppression.

The comparator subagent was not used per user quota direction. Cohort and
comparison work ran locally with the release CLI plus DuckDB/Pandas.

## Validation

Ran:

- `cargo fmt --check` — pass.
- `git diff --check` — pass.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
  — pass, `40/40`.
- `cargo test --workspace` — pass.
- `cargo deny check` — pass.
- `bash tools/release/check_authority_suite_antievasion.sh` — pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract` — pass.
- Release build: `cargo build --release -p openwepp-runner --bin
  openwepp-cli-hill` — pass; production binary SHA
  `981da203d9ced9b1d73f049fa3a4b227710862a3dbecaad9d4619f03ae7dd2d5`.

## Cohort Results

Native production cohort:

- Root: `/tmp/fdhp01_increment_de_native_cohort_final_20260612T171358Z`.
- Execution: `43/43` clean exits, `43/43` WAT outputs.
- Years 2-6 independent `Total-Soil + frozwt` closure:
  `5.474257917248426e-07 mm` max abs residual (`p11`, year 6), within the
  Dd creep-watch texture.
- Depth/duration remain red under native snow: mean max depth
  `705.505148615878 mm`, median max `688.4317537049452 mm`, `0/43`
  prefixes inside the legacy `240..503.2 mm` envelope, median depth
  correlation `0.746163306608445`, and median frozen-duration residual
  `+288` days.

Forced legacy-snow diagnostic cohort:

- Root: `/tmp/fdhp01_increment_de_forced_snow_cohort_20260612T171017Z_proper`.
- Forcing CSV:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/legacy_snow_forcing_daily_hour24.csv`.
- Execution: `43/43` clean exits, `43/43` WAT outputs.
- Years 2-6 independent `Total-Soil + frozwt` closure:
  `4.355148297552347e-07 mm` max abs residual (`p11`, year 4).
- Forced legacy snow plus De improves but does not certify D3: mean max depth
  `655.9890274782282 mm`, median max `652.3375464029963 mm`, range
  `558.1869128158116..741.4969698215496 mm`, `0/43` prefixes inside the
  `240..503.2 mm` envelope, median depth correlation
  `0.770042438411068`, and median frozen-duration residual `+186` days.

The forced-snow diagnostic used a temporary env-gated hook at the frost
snow-depth/density read seam. The hook was removed before the production
rebuild. One earlier diagnostic invocation reused runfiles whose explicit
output paths still pointed at the native root; it produced no WAT files under
its own forced root and was superseded by the `_proper` forced run above plus
the fresh native production rerun.

## Disposition

De lands the F5 `qdry` conductivity correction and preserves D2 at the accepted
WAT-publication texture, but FDHP01 remains `executed-hold`. The controlled
forced-snow certification still fails the D3 envelope and duration gate.

The next increment remains frost-side under the De forced-snow setup. It should
use paired hourly trajectory evidence to locate the first remaining material
front/flux divergence after content-dependent `qdry`: openWEPP hourly
`frzflg`, `Qsrf`, `Quf`, lower-front heat, surface/frozen/residue/snow
resistance, front advance/retreat, and fine-layer ice/liquid motions against
legacy `H*.winter.dat` plus source-line state. Do not tune snow density/depth,
kfactor, latent heat, WAT publication, or D2 storage surfaces.
