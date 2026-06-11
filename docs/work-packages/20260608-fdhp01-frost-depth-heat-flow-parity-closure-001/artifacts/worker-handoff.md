# worker handoff

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Handoff

FDHP01 is not complete. No branch was created for this closure pass.

Primary landed behavior:

- Frost depth now uses hourly signed heat flow and latent-heat increments
  instead of the retired freeze-index proxy.
- Depth state is bounded by `solthk`/physical profile depth, while the
  remaining `0.20 m` constant is only the CLIM06 tilled-layer conductivity
  scale.
- Active frost now carries per-layer frozen-depth and frozen-water state in
  `wb18_perc_frozen_depth_####` and `wb18_perc_frzw_####`.
- Frozen-water exchange is bidirectional within layer capacity: freezing
  withdraws liquid water into `frzw`, and thawing returns active ice to liquid
  `wb11_soil_water`.
- WAT output now publishes `frdp` in `mm` from runtime
  `frost.runtime_frdp_m`; dataset version is `1.4`.
- WAT `SoilWaterTotal` is now the hydout-equivalent `Total-Soil` alias again;
  `frozwt` remains separately published to avoid frozen-storage double
  counting.
- `SC-WATBAL-001` v152 pins the legacy `frwatc.for`/`watbalprint.for`
  definition: `Total-Soil`/`SoilWaterTotal` exclude frozen water, and
  frost-active storage audits use `Total-Soil + frozwt`.
- Active frost exchange now publishes `frost.runtime_frwatc_*` diagnostics
  proving liquid/frozen before/after state, freeze debit, thaw credit, and
  signed liquid delta.
- WAT `frozwt` publication now resolves the layer-state legacy store
  `Σ soilf(i) = Σ(wb18_perc_frzw_#### + thetdr_#### *
  wb18_perc_frozen_depth_####)` instead of scalar `frdp * theta`.

Validation status before post-review cohort validation:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.

Layered-store cohort result:

- Post-review cohort run root:
  `/tmp/fdhp01_layered_store_20260611T080722Z`.
- `43/43` frost-on prefixes exited clean; the prior `p2`
  `HKERNEL-WB11-PERC-E-003` failure no longer reproduces.
- Annual `Total-Soil + frozwt` closure is restored to numerical noise: max abs
  residual `1.2683574368566042e-07 mm`.
- `frozwt/frdp` no longer has exact per-prefix scalar ratios (`36064`
  frost-active rows; minimum correlation `0.8210678396408895`; maximum ratio
  standard deviation `0.0700106996666242`).
- Depth metrics still fail: openWEPP max depth mean
  `1782.0379909380451 mm` versus legacy `414.22093023255815 mm`, median depth
  correlation `-0.27756218032931956`, and open frozen-day count
  `518.5348837209302` days lower than legacy on average.
- `SC-SNOWFREEZE-001` v56 keeps `GAP-SNOWFREEZE-002` open for the D3
  depth/duration defect.

First actionable item: close defect `FDHP01-FROST-DEPTH-HEATFLOW-001` by
completing the layered thermal-resistance/depth-progression port. D2 additive
storage and the prior `p2` fail-closed event are no longer the lead blockers.
Do not advance to MOFE until the full 43-prefix cohort still runs clean,
frost-active annual closure remains at numerical noise under the ratified
storage term, and depth/duration evidence materially closes the FDMC01 gap
without comparator tuning.
