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
- D3 Increment A now publishes non-driving shadow fine-state diagnostics for
  `fgfrst`/`slfsd`/`slsic`/`slsw`/`sltime`/`yst`/`nwfrzz`. The shadow state is
  a handoff/conservation proof surface only and must not be treated as active
  depth authority until Increment B.
- D3 Increment B now derives active frost depth from the fine-state
  `fgfrst`/`slfsd` scan and mutates `slfsd`/`slsic`/`slsw`/`nwfrzz` in
  freeze-active hours through `frzng`/`frznw` lineage.
- `SC-SNOWFREEZE-001` is now v59. `frwatc(1)` is pinned to active-day hour-1
  ingress, `frost.hourly.frzflg_####` is a required freeze/thaw branch
  diagnostic, scalar target-depth projection is retired as production
  authority, and threshold-bounded exchange-debit limiting is authorized only at
  the available-liquid handoff boundary.

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
- `SC-SNOWFREEZE-001` v57 keeps `GAP-SNOWFREEZE-002` open for the D3
  depth/duration defect.

D3 attempt result:

- Codex attempted a coarse continuous per-layer energy-front implementation and
  ran `/tmp/fdhp01_d3_layered_energy_20260611T085142Z` (`43/43` clean).
- It improved median maximum frost depth to `490.774886655666 mm`, but failed
  D3 acceptance: mean max depth `643.2973898432339 mm`, max depth
  `1789.9130899451595 mm`, median correlation `-0.1876255663636445`, and
  median frozen-duration delta `-428` days.
- The attempted production/test edit was backed out. Do not restart from the
  coarse-front approach; the next implementation needs the legacy fine-layer
  `fgfrst`/`slfsd`/`slsic`/`slsw`/`nwfrzz` state machine and the
  `frostn`/`frzng`/`mltbtm`/`frwatc` bidirectional coupling.
- `SC-SNOWFREEZE-001` v57 explicitly prohibits post-hoc scalar depth projection
  into layer stores; v58 adds the Increment A shadow handoff authority.

Increment B result:

- Clean `43/43` cohort execution. The copied wrappers wrote outputs under
  `/tmp/frostval01_rerun_20260611T020951Z/outputs`; the CLI manifest root was
  `/tmp/fdhp01_increment_b_final_20260611T193423Z/outputs`.
- Years 2-6 `Total-Soil + frozwt` closure remains at numerical noise: max abs
  `3.0880187296133954e-11 mm`. The year-7 boundary watch item remains tiny:
  `1.2683569483584733e-07 mm`.
- Profile-bound pinning directional gate passes: `0/43` prefixes pinned at
  `ProfileDepth`, minimum margin `16.63152804088827 mm`.
- `frozwt/frdp` scalar-signature gate passes: max per-prefix correlation
  `0.9861968090242198`, below the rejected `0.9987` signature.
- Depth magnitude remains near the profile bound: mean maximum depth
  `1782.265765656973 mm`.

Increment C attempt result:

- Codex attempted top/bottom thaw arms, sandwich geometry, and thaw-through
  behavior. The production/contract/test edits were backed out.
- First cohort failure: `p1` at 1990 day 45 rejected
  `wb18_perc_frzw_0001=0.06135293352005228` against
  `wb18_perc_ul_0001=0.05875247947169813`.
- A source cap made the attempted tree run `43/43`, but years 2-6
  `Total-Soil + frozwt` closure failed catastrophically: max abs residual
  `2325832826960980.0 mm`; `p1` year-4 `Total-Soil` reached
  `1.558719e+35 mm`.
- The failed attempt is recorded in
  `d3-increment-c-thaw-arms-20260611.md` and the
  `fdhp01_increment_c_*_20260611.*` reports.

First actionable item: re-execute Increment C only after adding the missing
capacity-aware `watdst` redistribution and `watpdg`/`watbtm` overflow paths.
Do not retain `mlttp`/`mltbtm` thaw arms or advance to MOFE until years 2-6
conservation remains at noise, the cohort is `43/43`, and the full
depth/duration acceptance gate passes without comparator tuning.
