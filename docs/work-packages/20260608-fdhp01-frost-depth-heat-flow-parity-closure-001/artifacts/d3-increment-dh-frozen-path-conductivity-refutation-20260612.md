# D3 Increment Dh - Frozen-Path Conductivity Refutation

Status: executed-hold; no production physics edit

Evidence mode: Static + Ran

Date: 2026-06-12

## Objective

Execute Increment Dh from `d3-staged-increment-plan.md` without the comparator
subagent. Dh proposed that the remaining Dg forced-snow plateaus might be
caused by openWEPP using fixed frozen-path conductivities where legacy computed
soil-dependent `kftill`/`kfutil`.

## Static Finding

Pinned-source inspection refutes the implementation premise.

- `frostn.for:188-193` assigns fixed `kftill = 1.75`,
  `kfutil = 2.1`, and `kres = 0.05 * kresf`.
- `frostn.for:530-534`, `frzng.for:135`, `frzng.for:295-304`, and
  `frznw.for:106-108` consume those constants in the frozen tilled/untilled
  surface path.
- The soil-property-dependent conductivity expression involving `bdcons`,
  `slsw`, and `ksoilf` is the lower-front unfrozen `kufzfl` path in
  `frostn.for:430-458` and companion thaw/freeze routines, already handled by
  Increment De.
- Both `/workdir/wepp-forest_260430_baseline/src` and exploratory
  `/workdir/wepp-forest/src` show the same fixed `kftill`/`kfutil`
  assignments.

Therefore a per-soil frozen-path conductivity port would contradict the pinned
baseline rather than close a migration gap.

## Local Comparison

Ran local Pandas comparisons over the Dg forced-snow evidence artifact
`fdhp01_increment_dg_forced_snow_depth_metrics_20260612.csv`; the comparator
subagent was not used.

Representative plateau rows:

| prefix | group | open max depth mm | legacy max depth mm | delta mm | frozen-days delta |
|---|---|---:|---:|---:|---:|
| `p8` | A | `530.023647` | `426.7` | `103.323647` | `+77` |
| `p20` | B | `574.363914` | `374.3` | `200.063914` | `+45` |
| `p2` | C | `609.561911` | `280.0` | `329.561911` | `+116` |

Remaining Dg forced-snow outliers above `503.2 mm`: `p1`, `p2`, `p3`, `p8`,
`p11`, `p13`, `p20`, `p21`, `p22`, `p23`, `p26`, `p28`, `p32`.

The plateau structure remains real, but Dh's named frozen-path conductivity
mechanism is not the source-line-owned explanation.

## Implementation

- `SC-SNOWFREEZE-001` is amended to v67 with
  `REF-SNOWFREEZE-LEGACY-FROZEN-PATH-KF`.
- `INV-SNOWFREEZE-006` now binds fixed legacy `kftill`/`kfutil` constants for
  the frozen surface path and explicitly rejects replacing them with per-soil
  frozen-path conductivity absent superseding authority.
- Added a contract regression test proving published runtime
  `frost.runtime_kftill_w_m_k` and `frost.runtime_kfutil_w_m_k` remain
  `1.75` and `2.1` even when `ksoilf` and layer conductivity inputs vary.
- No production code was changed.

## Validation

Ran:

- `cargo fmt --check` - pass.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_dh_frozen_path_conductivity_uses_pinned_legacy_constants -- --nocapture` - pass.
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract --test hphys0320_stmtim_start_time_source_line_contract` - pass.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` - pass, `43/43`.
- `git diff --check` - pass.
- `wctl doc-lint --path docs` - pass, `1220` files, `0` errors, `0` warnings.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo deny check` - pass.
- `bash tools/release/check_authority_suite_antievasion.sh` - pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract` - pass.
- `cargo test --workspace` - pass.

## Disposition

Dh is executed as a refutation, not a landed physics change. FDHP01 remains
`executed-hold`; the next increment must localize the residual Dg forced-snow
outliers without retuning snow, WAT publication, D2 storage, residue depth,
`dpfsfl`, or fixed `kftill`/`kfutil`.
