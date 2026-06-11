# FDHP01 Frost Depth Validation Ledger

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Baseline Defect

Static:

- FDMC01 artifacts sized the pre-fix defect: openWEPP depth was capped at
  `200 mm`; legacy heat-flow output ranged `240.0..503.2 mm` across the
  characterized prefixes; depth-series median correlation was `0.13`; frozen
  duration was over-persistent by roughly `+258 days`.
- The exact FDMC01 owcmp suite could not be rerun for this package because no
  declared FDMC01 owcmp manifest exists in the repository.

## Landed Validation

Ran:

- `fdhp01_contract_heat_flow_depth_can_exceed_retired_proxy_cap` passes and
  demonstrates active frost depth can progress beyond the retired `0.20 m`
  model cap while remaining bounded by `solthk`.
- `fdhp01_contract_heat_flow_publishes_separate_surface_and_unfrozen_fluxes`
  passes and demonstrates active frost publication now carries both surface
  heat loss (`Qsrf`) and lower unfrozen-soil heat flow (`Quf`).
- `fdhp01_contract_warm_heat_flow_thaws_prior_deep_frost` passes and
  demonstrates warm signed heat-flow can thaw a prior physical frost depth
  above `0.20 m` instead of rejecting or ratcheting it; the paired assertion
  proves prior frozen storage is credited back to liquid `wb11_soil_water`.
- `fdhp01_contract_frozen_water_exchange_hard_fails_on_liquid_overdraw` passes
  and demonstrates the frozen-water exchange does not silently create storage
  beyond available liquid soil water; it halts in `RunoffReconciliation` with
  `BoundaryClass::DomainViolation`.
- The full CLIM06 frost contract integration suite passes.
- WAT parquet metadata now requires and observes `frdp` in `mm` under dataset
  version `1.4`.
- Runner unit tests pass for nonzero runtime `frdp` to WAT `mm` conversion and
  profile-depth bound rejection.
- Unit registry tests now require `hillslope_wat.frdp`.
- `pl14s_tier_a_candidate_emission_and_replay_contract` passes after replacing
  runner provenance bounds for `Dfrost`/`Dthaw` with physical `solthk` bounds.
- `cargo test --workspace` passes, preserving the broader rung-1/frost
  non-regression surface covered by the workspace suite.
- Contract-version expectation tests now target `SC-SNOWFREEZE-001` v55 and
  `SC-WATBAL-001` v150 front matter.
- D2 exchange diagnostics now publish liquid soil water before/after, frozen
  water before/after, freeze debit, thaw credit, and signed liquid delta at
  the active frost exchange seam. The focused CLIM06 suite now includes
  freeze-onset and warm-thaw vectors proving the in-process exchange algebra
  reconciles on both signs.

## Post-Review Cohort Validation

Ran, 2026-06-11:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: pass.
- Fresh frost-on 43-prefix `algebraic-radium` population after the D1
  `SoilWaterTotal` publication fix, using generated runfile wrappers and the
  release binary:
  `/tmp/fdhp01_closure_after_d1_restored_20260611T053545Z`.
- Compact persisted reports:
  - `fdhp01_run_status_20260611.tsv`
  - `fdhp01_activation_summary_20260611.csv`
  - `fdhp01_annual_closure_residuals_20260611.csv`
  - `fdhp01_depth_metrics_20260611.csv`
  - `fdhp01_closure_summary_20260611.json`

Results:

- Clean frost-on prefixes: `42/43`.
- Failure: `p2` failed before WAT publication at
  `HKERNEL-WB11-PERC-E-003`, `sim_day_index=308`, calendar `1990-308`.
- Emitted-prefix activation: `42/42 frsoil.active=true`, `42/42` nonzero
  `frozwt`.
- Emitted-prefix annual closure rows: `252`.
- Max absolute annual closure residual after D1: `2.4798612273409617 mm`
  (baseline FROSTVAL01 rerun max was `3.2173375075217336e-11 mm`; pre-D1
  post-review run max was `75.43917280313423 mm`).
- Mean absolute annual closure residual after D1:
  `0.9738853177643827 mm`.
- Emitted-prefix frost depth max range:
  `1780.5852693307895..1783.3684719591115 mm`.
- Emitted-prefix mean max depth: `1782.2670980346527 mm` versus matched
  legacy mean max depth `417.4166666666667 mm`.
- Emitted-prefix median depth correlation: `-0.10301692862035305`
  versus FDMC01 pre-fix proxy median `0.13332765680932177`.
- Emitted-prefix frozen-duration delta improved in sign/magnitude
  (`-27.61904761904762` days mean open-minus-legacy, versus FDMC01
  `+257.6279069767442`), but this is diagnostic only because closure and the
  full 43-prefix run failed.

## Disposition

FDHP01 does not close the executable single-OFE model-depth implementation
boundary. The WAT `frdp` publication surface exists, and D1 removed the dominant
frozen-storage double count from `SoilWaterTotal`. D2 added the seam
diagnostics needed to judge freeze/thaw exchange wiring and `SC-WATBAL-001`
v150 ratifies that `Total-Soil + frozwt` is the frost-active storage audit
term. Addendum 2c then localized the remaining WAT contradiction to `frozwt`
publication: emitted `frozwt` is `0.149 * frdp` over measured frost-active
days, so the current additive audit uses a depth-derived quantity rather than
the exchanged frozen store. The cohort validation still failed required
closure criteria and reopened `GAP-SNOWFREEZE-002` in `SC-SNOWFREEZE-001` v55.

The package remains in defect closure. The next actionable work is to fix
FDHP01 by publishing the true exchanged frozen store as `frozwt`, rerunning the
cohort additive identity, keeping the independent `p2` fail-closed defect
tracked separately, and then closing the D3 depth/duration gap without
comparator tuning.
