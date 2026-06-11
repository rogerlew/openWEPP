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
- Contract-version expectation tests now target `SC-SNOWFREEZE-001` v57 and
  `SC-WATBAL-001` v152 front matter.
- D2 exchange diagnostics now publish liquid soil water before/after, frozen
  water before/after, freeze debit, thaw credit, and signed liquid delta at
  the active frost exchange seam. The focused CLIM06 suite now includes
  freeze-onset and warm-thaw vectors proving the in-process exchange algebra
  reconciles on both signs.
- Active frost now persists per-layer frozen depth and active frozen water in
  `wb18_perc_frozen_depth_####` and `wb18_perc_frzw_####`.
- WAT `frozwt` publication now resolves the legacy `Σ soilf(i)` store from
  layer state, not from scalar `frdp * theta`; focused CLIM06 vectors reject
  scalar-store equivalence and prove layer `frzw` updates with freeze/thaw.

## Layered-State Cohort Validation

Ran, 2026-06-11:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: pass.
- Fresh frost-on 43-prefix `algebraic-radium` population after the layered
  frozen-store continuation, using generated runfile wrappers and the release
  binary: `/tmp/fdhp01_layered_store_20260611T080722Z`.
- Compact persisted reports:
  - `fdhp01_layered_run_status_20260611.tsv`
  - `fdhp01_layered_activation_summary_20260611.csv`
  - `fdhp01_layered_annual_closure_residuals_20260611.csv`
  - `fdhp01_layered_depth_metrics_20260611.csv`
  - `fdhp01_layered_frozwt_frdp_ratio_20260611.csv`
  - `fdhp01_layered_closure_summary_20260611.json`
  - `fdhp01_layered_execution_summary_20260611.json`

Results:

- Clean frost-on prefixes: `43/43`.
- The prior `p2` `HKERNEL-WB11-PERC-E-003` fail-closed event does not
  reproduce.
- Activation: `43/43 frsoil.active=true`, `43/43` nonzero
  `frozwt`.
- Annual closure rows: `258`.
- Max absolute annual `Total-Soil + frozwt` closure residual:
  `1.2683574368566042e-07 mm` (baseline FROSTVAL01 rerun max was
  `3.2173375075217336e-11 mm`; pre-layered post-review residual was
  `2.4798612273409617 mm`).
- Mean absolute annual `Total-Soil + frozwt` closure residual:
  `2.1277404919798806e-09 mm`.
- Soil-only identity now fails as expected under the v152 additive storage
  term: max abs residual `119.04111532237937 mm`, mean abs residual
  `52.31253307756673 mm`.
- Frost depth max range:
  `1780.3226093850215..1783.3684719591117 mm`.
- Mean max depth: `1782.0379909380451 mm` versus matched legacy mean max depth
  `414.22093023255815 mm`.
- Median depth correlation: `-0.27756218032931956`
  versus FDMC01 pre-fix proxy median `0.13332765680932177`.
- Frozen-duration delta is `-518.5348837209302` days mean open-minus-legacy,
  versus FDMC01 `+257.6279069767442`.
- `frozwt/frdp` audit no longer shows exact scalar publication: `36064`
  active rows, minimum per-prefix correlation `0.8210678396408895`, median
  correlation `0.963536279373424`, median-of-medians ratio
  `0.27690505830652684`, maximum ratio standard deviation
  `0.0700106996666242`.

## Disposition

FDHP01 does not close the executable single-OFE model-depth implementation
boundary. The WAT `frdp` publication surface exists, and D1 removed the dominant
frozen-storage double count from `SoilWaterTotal`. D2 added the seam
diagnostics needed to judge freeze/thaw exchange wiring and `SC-WATBAL-001`
v152 ratifies that `Total-Soil + frozwt` is the frost-active storage audit
term and binds WAT `frozwt` to the layered legacy `Σ soilf(i)` store. The fresh
cohort shows D2 closure: the additive identity returns to numerical noise, the
prior `p2` failure no longer reproduces, and `frozwt` is no longer an exact
scalar function of `frdp`.

The package remains in defect closure because D3 depth/duration parity still
fails materially. The next actionable work is to complete the layered
thermal-resistance/depth-progression port so the frost front is bounded by the
layered frost state rather than only by the physical profile, then rerun the
same additive identity and depth/duration cohort gates without comparator
tuning.

## D3 Coarse-Front Attempt

Ran, 2026-06-11:

- A coarse continuous per-layer energy-front experiment was built locally from
  commit `8d13ba898a111aeed97375cc997d0ca65d6b85b7` and run against a fresh
  `43/43` `algebraic-radium` frost-on population:
  `/tmp/fdhp01_d3_layered_energy_20260611T085142Z`.
- Trusted package reports:
  - `fdhp01_d3_attempt_summary_20260611.json`
  - `fdhp01_d3_attempt_run_status_20260611.tsv`
  - `fdhp01_d3_attempt_activation_summary_20260611.csv`
  - `fdhp01_d3_attempt_depth_metrics_20260611.csv`
  - `fdhp01_d3_attempt_frozwt_frdp_ratio_20260611.csv`
  - `fdhp01_d3_attempt_execution_summary_20260611.json`

Results:

- Clean prefixes: `43/43`.
- The median maximum frost depth moved into the legacy envelope:
  `490.774886655666 mm` versus legacy median `420.0 mm`.
- The phase boundary still failed: mean maximum frost depth
  `643.2973898432339 mm` versus legacy mean `414.22093023255815 mm`, max
  openWEPP depth `1789.9130899451595 mm`, median depth correlation
  `-0.1876255663636445`, and mean frozen-duration delta
  `-415.2093023255814` days open-minus-legacy.
- Dry-profile outliers remained severe: p23 `1789.9130899451595 mm`, p2
  `1756.6037758327177 mm`, p3 `1639.9350755585738 mm`.
- The provisional annual closure reconstruction generated during this pass is
  not package evidence: the committed layered-store report's `outputs` field
  could not be reproduced from exposed WAT columns alone. D2 closure remains
  governed by the committed layered-store reports above.

Static:

- `/workdir/wepp-forest_260430_baseline/src/frostn.for:360-683` selects
  `frzflg` from `qhtout`/`qdry`, dispatches `frzng`, `mltbtm`, and `mlttp`,
  and closes active hours with `frwatc(0)`.
- `/workdir/wepp-forest_260430_baseline/src/frzng.for:235-560` advances the
  frost front fine-layer by fine-layer with a remaining-hour budget, migration
  water (`qwet`/`qwater`), and `slfsd`/`slsic` updates.
- `/workdir/wepp-forest_260430_baseline/src/frwatc.for:89-137` recomputes
  `frozen`, `frzw`, `st`, and `soilw` from fine-layer `slfsd`/`slsic`/`slsw`
  state.

Disposition:

- The coarse-front production/test edit did not meet the package's legitimate
  phase boundary and was backed out. It should not be repeated or promoted.
- `SC-SNOWFREEZE-001` v57 remains the D3 hold amendment: D3 closure requires
  the same fine-layer state that `frwatc` publishes, with front advance and
  thaw retreat coupled to that state. v58 supersedes it for Increment A by
  pinning hour-1 `frwatc(1)` ingress and authorizing the non-driving shadow
  state.
