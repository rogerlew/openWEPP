# D3 Increment Dc1 Accounting Repair

Status: executed-hold; Dc1 landed

Evidence mode: Static + Ran

Date: 2026-06-12

## Scope

Increment Dc1 split the failed Dc pass and kept the acceptance boundary
conservation-first. The landed runtime change:

- Retires the stable lower-front heat surrogate and uses the legacy seasonal
  `tmpbl` wave for `Qdry`, with a named guarded `kufz = 0.2 W m^-1 degC^-1`
  fallback until the full `bdcons` conductivity path is available.
- Recomputes thaw resistance/front geometry inside top and bottom thaw hours
  instead of spending a start-hour thaw flux across multiple fine layers.
- Canonicalizes only finite fine-layer liquid-theta lower-bound roundoff within
  `1e-10` volumetric theta; material sub-residual values remain typed domain
  violations.
- Leaves depth/duration as recorded evidence only for Dc1 because the known F4
  snow-insulation seam still controls acceptance interpretation.

## Red Evidence

The focused Dc1 red tests failed on the Db boundary before implementation:

- Cold seasonal `tmpbl` expected the lower-front heat gate to emit `0 W/m2`,
  but the old surrogate emitted `14.7 W/m2`.
- Warm constant monthly temperatures expected the legacy fallback heat of
  `3.0 W/m2`, but the old surrogate still emitted `14.7 W/m2`.
- The top-thaw one-hour vector spent too much thaw energy with stale
  start-hour resistance.
- The p35 fine-theta boundary vector tripped the lower-bound guard on
  roundoff-scale valid input.

The same focused test set passed after the Dc1 implementation.

## Ran

| Gate | Result |
|---|---|
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_dc1_ -- --nocapture` | Pass, 4 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 38 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `95491b24f36065c28f90ca7e55bfceb39cf14ac2c270ddfd207eb750a2e4a536` |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract` | Pass after `SC-SNOWFREEZE-001` v64 |
| `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract` | Pass after `SC-SNOWFREEZE-001` v64 |
| 43-prefix local `algebraic-radium` frost-on cohort | Pass, `43/43` clean exits at `/tmp/fdhp01_increment_dc1_cohort_20260612T101238Z` |

Comparator subagent was not used; the parent ran local CLI/PyArrow comparisons
per user quota direction.

## Cohort Result

Persisted reports:

- `fdhp01_increment_dc1_execution_summary_20260612.json`
- `fdhp01_increment_dc1_run_status_20260612.tsv`
- `fdhp01_increment_dc1_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_dc1_depth_metrics_20260612.csv`
- `fdhp01_increment_dc1_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_dc1_activation_summary_20260612.csv`

Accounting gate:

- Clean exits: `43/43`, WAT outputs: `43/43`.
- Independent annual ledger:
  `RM + Irr - Interception - Q - Ep - Es - Er - Dp - latqcc - Tile -
  delta(Total-Soil + frozwt)`.
- Years 2-6 max abs residual:
  `6.471338602487275e-07 mm` at `p11`, year 4.
- p1 years 2-6 max abs residual:
  `9.41296818268711e-10 mm`.
- p20 years 2-6 max abs residual:
  `1.0458300891968975e-13 mm`.
- p43 year-2 residual:
  `-1.1013412404281553e-13 mm`.
- Year 1 remains outside the staged hard gate; all-years max abs residual is
  `79.52424388720456 mm` at `p40`, year 1.

Depth/duration evidence, recorded but not accepted for Dc1:

- Mean maximum frost depth:
  `1146.5109665924424 mm`; median `1110.3558249519133 mm`; range
  `877.0024741427947..1799.9999999999998 mm`.
- Profile-bound pinning regressed to `1/43` prefixes, with minimum margin
  `2.2737367544323206e-13 mm`.
- `0/43` prefixes are inside the legacy `240..503.2 mm` maximum-depth
  envelope.
- Median depth correlation improved to `0.6415921721982907`.
- Frozen duration over-persists: median open-minus-legacy `+567` days.
- Median full-WAT days above `200 mm`: `1126`.
- `frozwt/frdp` remains non-scalar: median correlation
  `0.7316780606012344` across `57061` frost-active days.

## Disposition

Dc1 lands. It repairs the Dc additive-storage leak and preserves D2/p2 closure
at WAT-publication numerical texture, but FDHP01 remains `executed-hold`.
Depth/duration evidence still fails D3 acceptance and is now assigned to the
F4 snow-insulation/depth-duration coupling discriminator before MOFE closure.
