# D3 Increment Di - Post-Dg Paired Re-localization

Status: executed-hold; no production physics edit

Evidence mode: Static + Ran

Date: 2026-06-12

## Objective

Execute Increment Di from `d3-staged-increment-plan.md` without the comparator
subagent. Di re-ran the paired hourly trace on post-Dg forced-snow plateau
representatives (`p8`, `p20`, and `p2`) to localize the remaining forced-snow
overshoot before scoping another implementation increment.

## Method

- Added a temporary env-gated hook inside `compute_active_frost_coupling` that:
  - forced only frost-consumed snow depth/density from the Dg legacy hour-24
    daily snow CSV;
  - wrote hourly front depth, branch, surface flux, lower-front heat, surface
    resistance, snow/residue inputs, fine-cell location, and local fine-layer
    liquid/ice terms.
- Ran `p8`, `p20`, and `p2` through the Dg forced-snow runfiles after
  rewriting output paths into a new temp root.
- Joined the trace to pinned legacy `H*.winter.dat` rows by
  `(simulation_year, julian, hour)`.
- Removed the temporary hook before updating package evidence.

The comparator subagent was not used per user quota direction. All comparisons
were run locally with the release CLI plus Pandas/PyArrow.

## Evidence

- Temporary trace root:
  `/tmp/fdhp01_increment_di_trace_20260612T1302Z`
- Dg forced-snow source cohort:
  `/tmp/fdhp01_increment_dg_forced_snow_cohort_20260612T185203Z`
- Forced snow CSV:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/legacy_snow_forcing_daily_hour24.csv`
- Legacy winter root:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/output`
- Compact artifacts:
  - `fdhp01_increment_di_localization_summary_20260612.json`
  - `fdhp01_increment_di_localization_metrics_20260612.csv`
  - `fdhp01_increment_di_term_attribution_20260612.csv`
  - `fdhp01_increment_di_max_depth_context_20260612.csv`

The raw hourly trace CSVs were not committed; they remain under `/tmp`.

## Static Source Finding

The legacy winter output header is produced by `winthd.for`, but it does not
publish `surtmp`; it publishes snow, frost/thaw depth, frost thickness, and
residue depth. Therefore the surface-temperature question cannot be settled
from `H*.winter.dat` alone.

Pinned baseline source does provide the controlling provenance:

- `hr_tmp.for:38-48` says `tmpadj` calculates surface temperature hourly and
  calls `tmpadj(hour, halfdy)` before frost processing.
- `tmpadj.for:1-7` defines the routine as the hourly surface-temperature
  adjustment; `tmpadj.for:34-39` reads `cwint` winter state including `surtmp`,
  snow depth, frost depth, thaw depth, and density.
- `tmpadj.for:349-364` computes `surtmp(hour)` from net radiation, wind, hourly
  air temperature, effective layered conductance, and system depth, then caps
  positive `surtmp` to `0.0` when snow is present.
- `frostn.for:467-480` consumes `surtmp(hour)` for top heat flux and applies
  the same positive-under-snow cap before computing the harmonic surface path.

The current openWEPP frost loop does not port that surface-temperature
synthesis. It sets `surface_temp_c = hourly_air_temp_c` whenever hourly air is
below freezing, and only applies the positive-under-snow cap.

## Result

Di localizes the remaining post-Dg forced-snow overshoot to missing legacy
`tmpadj`/`hr_tmp` surface-temperature synthesis feeding the frost surface heat
path.

| prefix | open max mm | legacy max mm | deep divergent advance rows | divergent advance mm | share with forced snow + negative open surface temp | median surface-flux share |
|---|---:|---:|---:|---:|---:|---:|
| `p8` | `530.023647` | `440.0` | `350` | `44.023354` | `0.997852` | `1.000000` |
| `p20` | `574.592448` | `374.3` | `408` | `50.820335` | `0.999063` | `1.000000` |
| `p2` | `609.852334` | `280.0` | `976` | `222.906902` | `1.000000` | `0.994355` |

The discriminators are stable across the three target prefixes:

- In all deep divergent advance rows, `surface_temp_c` equals
  `hourly_air_temp_c`; the open frost path is not using a distinct
  surface-temperature state.
- The deep divergent rows are surface-path dominated. Median freezing surface
  flux is `1.663142`, `1.725178`, and `2.734764 W/m2` for `p8`, `p20`, and
  `p2`; median lower-front heat is `0.0`, `0.0`, and `0.023829 W/m2`.
- Snow-depth timing mismatch is secondary under the Dg forced-snow setup:
  rows with `>10 mm` open/legacy hourly snow-depth mismatch account for only
  `0.085`, `0.140`, and `0.082` of deep divergent advance for `p8`, `p20`,
  and `p2`.
- The plateau depths are not exact topology boundaries or the `0.2 m` tillage
  seam. Max depths are `9.976`, `5.408`, and `9.852 mm` from the nearest fine
  boundary and `330.024`, `374.592`, and `409.852 mm` from `tilld`.
- Deep-layer water content modulates the magnitude but is not the first-order
  discriminator: front-layer `slsw_theta` at max is `0.157450`, `0.165000`,
  and `0.106426`, while the divergent advance remains surface-flux dominated.

`p2` remains larger in magnitude, but it is not a distinct mechanism in this
trace. Its outlier behavior is the same missing surface-temperature path with
a larger accumulation window and lower legacy maximum depth.

## Validation

Ran:

- `cargo fmt --check` - pass.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` with the
  temporary trace hook - pass.
- p8/p20/p2 diagnostic release CLI runs - pass, WAT outputs and trace CSVs
  present under `/tmp/fdhp01_increment_di_trace_20260612T1302Z`.
- Paired hourly joins - pass: p8 `29472` rows, p20 `30336` rows, p2 `29928`
  rows.
- `rg -n "OPENWEPP_FDHP01_DI|fdhp01_di|FDHP01_DI" crates` after hook removal -
  pass, no markers remain.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` after hook
  removal - pass, clean-source release binary SHA
  `3275db431339402596a27a28d7976062eb4655771e9e159fdf929fa1410883ad`.
- `git diff --check` - pass.
- `wctl doc-lint --path docs` - pass, `1220` files, `0` errors, `0` warnings.

## Disposition

Di is diagnostic-only and leaves production at the Dg/Dh boundary. FDHP01
remains `executed-hold`.

The next bounded implementation increment should be Dj: port or expose the
legacy `hr_tmp`/`tmpadj` winter surface-temperature synthesis into the frost
surface-heat path, then re-run the Dg forced-snow representatives, the
forced-snow cohort, the native cohort, and the independent years 2-6
`Total-Soil + frozwt` closure gate.

Do not retune snow depth/density, kfactor, latent heat, WAT publication, D2
storage, residue depth, `dpfsfl`, fixed `kftill`/`kfutil`, or lower-front
`Qdry` for this residual.
