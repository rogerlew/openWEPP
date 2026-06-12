# disposition

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Outcome

FDHP01 is executed but held. The D2 storage/publication defect is closed by the
layered-state continuation. D3 Increment A landed the non-driving fine-sublayer
shadow state and `frwatc` handoff proof surface. D3 Increment B landed
fine-state-derived depth and the freeze arms. D3 Increment C1b has now landed
the water-side capacity/overflow infrastructure required before retaining thaw
arms. D3 depth/duration parity remains open for C2 under
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`.

Static:

- The retired freeze-index frost-depth proxy and `0.20 m` model cap were
  removed from production frost-depth progression.
- Frost depth advances/thaws from an hourly signed heat-flow balance with
  separate surface heat loss (`Qsrf`) and lower-front unfrozen-soil heat input
  (`Quf`), bounded by physical profile depth.
- Active frost now carries per-layer `wb18_perc_frozen_depth_####` and
  `wb18_perc_frzw_####` state. WAT `frozwt` is bound to the legacy
  `Σ soilf(i)` lineage:
  `Σ(wb18_perc_frzw_#### + thetdr_#### * wb18_perc_frozen_depth_####)`.
- Freezing transfers active liquid from the affected layer into `frzw`; thawing
  returns active ice to liquid storage. Layer capacity/domain violations fail
  closed.
- WAT `SoilWaterTotal` remains the hydout-equivalent unfrozen `Total-Soil`
  alias; frost-active storage audits use `Total-Soil + frozwt` under
  `SC-WATBAL-001` v152.
- `SC-SNOWFREEZE-001` v57 rejects scalar `frdp * theta` frozen-water stores,
  prohibits post-hoc scalar depth projection into layer stores, and records
  that full depth parity still requires the fine-sublayer frost port.
- `SC-SNOWFREEZE-001` v58 corrects `INV-SNOWFREEZE-012` so `frwatc(1)` runs at
  active-day hour-1 ingress (`frostn.for:335-337`), and authorizes Increment A
  shadow aliases for `fgfrst`/`slfsd`/`slsic`/`slsw`/`sltime`/`yst`/`nwfrzz`.
- Increment A shadow state is written as diagnostics only. It does not drive
  active depth, conductivity, WAT publication, or freeze/thaw behavior.
- `SC-SNOWFREEZE-001` v59 promotes the fine-layer state as active freeze-depth
  authority, adds `frost.hourly.frzflg_####`, derives runtime
  `frdp`/`thdp`/`tfrdp`/`tthawd` from `fgfrst`/`slfsd`, retires scalar
  target-depth projection as production authority, and ratifies
  threshold-bounded exchange-debit limiting at the available-liquid handoff
  boundary.
- `SC-SNOWFREEZE-001` v60/v61 authorizes Increment C1b fine-layer
  capacity/overflow semantics, `watpdg`/`watbtm` publication surfaces,
  `watbtm` as WB13 `Dp` lineage, and bounded WB18/WB13 roundoff handling.
- Increment B mutates `slfsd`/`slsic`/`slsw`/`nwfrzz` for freeze-active hours
  (`frzng`/`frznw` lineage) and aggregates per-layer frozen depth/water from
  the same fine state.
- Increment C1b keeps fine-layer ice and liquid inside total pore capacity,
  routes unretained valid liquid through named overflow surfaces, publishes
  `watbtm` into `Dp`, preserves scalar/layer storage across bounded roundoff,
  and keeps the C1a capacity boundary from reappearing on the cohort.
- WAT parquet physical bytes are now deterministic while preserving readable
  Arrow field metadata, so the Increment A bit-identical cohort gate can remain
  literal.

Ran:

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` passed.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract --
  --nocapture` passed, `22` tests.
- `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract --
  --nocapture` passed after `SC-SNOWFREEZE-001` v58 /
  `SC-WATBAL-001` v152 updates.
- `cargo test --test hphys0320_stmtim_start_time_source_line_contract --
  --nocapture` passed after the same version updates.
- `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` passed.
- `cargo test -p openwepp-hillslope-output hillslope_wat -- --nocapture`
  passed, `4` tests.
- `cargo clippy -p openwepp-hillslope-output --all-targets -- -D warnings`
  passed.
- Latest-source `cargo fmt --check`, full workspace clippy, full workspace
  tests, and `cargo deny check` passed after final WAT footer minimization.
- Fresh 43-prefix `algebraic-radium` frost-on cohort run:
  `/tmp/fdhp01_layered_store_20260611T080722Z`, `43/43` clean exits.
- Increment B 43-prefix cohort execution was clean (`43/43`); copied wrappers
  wrote WAT/HBP/loss outputs under
  `/tmp/frostval01_rerun_20260611T020951Z/outputs`, with CLI manifests under
  `/tmp/fdhp01_increment_b_final_20260611T193423Z/outputs`.
- Increment C1b 43-prefix cohort execution was clean (`43/43`) at
  `/tmp/fdhp01_increment_c1b_cohort_final14_20260612T035618Z`; the parent ran
  comparisons locally without the comparator subagent per user quota direction.

## Closure Evidence

- Persisted compact reports:
  - `fdhp01_layered_closure_summary_20260611.json`
  - `fdhp01_layered_execution_summary_20260611.json`
  - `fdhp01_layered_run_status_20260611.tsv`
  - `fdhp01_layered_activation_summary_20260611.csv`
  - `fdhp01_layered_annual_closure_residuals_20260611.csv`
  - `fdhp01_layered_depth_metrics_20260611.csv`
  - `fdhp01_layered_frozwt_frdp_ratio_20260611.csv`
- Clean frost-on prefixes: `43/43`.
- `p2` no longer reproduces the prior `HKERNEL-WB11-PERC-E-003` fail-closed
  event.
- Annual `Total-Soil + frozwt` identity max abs residual:
  `1.2683574368566042e-07 mm`.
- Mean abs annual `Total-Soil + frozwt` residual:
  `2.1277404919798806e-09 mm`.
- Soil-only annual identity now fails as expected under the ratified additive
  storage term: max abs residual `119.04111532237937 mm`.
- `frozwt/frdp` is no longer an exact scalar audit: `36064` active rows,
  minimum per-prefix correlation `0.8210678396408895`, median correlation
  `0.963536279373424`, maximum ratio standard deviation
  `0.0700106996666242`.

## Hold Evidence

- OpenWEPP maximum frost depth remains near the physical profile bound:
  mean `1782.0379909380451 mm`, range
  `1780.3226093850215..1783.3684719591117 mm`.
- Matched legacy mean maximum frost depth is `414.22093023255815 mm`; mean
  open-minus-legacy max-depth delta is `1367.8170607054872 mm`.
- Median depth correlation is `-0.27756218032931956`.
- All `43/43` prefixes exceed `200 mm`; this proves the retired proxy cap is
  gone, but the depth progression is still not legacy-envelope faithful.
- OpenWEPP frozen-day count is `518.5348837209302` days lower than legacy on
  average.

## Increment A Output Gate Evidence

- Clean pre baseline root: `/tmp/fdhp01_increment_a_pre_20260611T164115Z`.
- Latest current root:
  `/tmp/fdhp01_increment_a_current_pre_like_pre_1_20260611T181018Z`,
  `43/43` clean exits.
- Pre vs current `H.hbp` and `H.loss.json` physical bytes are `43/43`
  identical.
- Pre vs current WAT decoded rows/columns are `43/43` identical.
- Pre vs current WAT physical bytes are `0/43` identical because the clean pre
  baseline has `43` unique nondeterministic `ARROW:schema` footer hashes.
- Latest current-vs-current physical bytes are `43/43` identical for
  `H.hbp`, `H.loss.json`, and `H.wat.parquet`; decoded WAT is also `43/43`.

## Increment B Gate Evidence

- Persisted compact reports:
  - `fdhp01_increment_b_execution_summary_20260611.json`
  - `fdhp01_increment_b_run_status_20260611.tsv`
  - `fdhp01_increment_b_annual_closure_residuals_20260611.csv`
  - `fdhp01_increment_b_depth_metrics_20260611.csv`
  - `fdhp01_increment_b_frozwt_frdp_ratio_20260611.csv`
- Years 2-6 `Total-Soil + frozwt` identity remains at numerical noise:
  max abs residual `3.0880187296133954e-11 mm`, mean abs residual
  `1.2662284657486707e-11 mm`.
- Year 7 remains a tiny boundary watch item: max abs residual
  `1.2683569483584733e-07 mm`. Year 1 initialization residual is recorded
  separately outside the staged gate: max abs `1.0505061950707386 mm`.
- Profile-bound pinning directional gate passes: `0/43` prefixes pin at
  `ProfileDepth`; minimum margin to the profile bound is
  `16.63152804088827 mm`.
- `frozwt/frdp` no longer carries the scalar signature: maximum per-prefix
  correlation `0.9861968090242198`, median `0.9635362793734238`, with
  `36064` frost-active rows.
- Depth magnitude is still a hold item: mean maximum depth
  `1782.265765656973 mm`, median maximum depth `1782.454753408546 mm`.

## Increment C Attempt Evidence

- Persisted compact reports:
  - `d3-increment-c-thaw-arms-20260611.md`
  - `fdhp01_increment_c_execution_summary_20260611.json`
  - `fdhp01_increment_c_run_status_20260611.tsv`
  - `fdhp01_increment_c_annual_closure_residuals_20260611.csv`
  - `fdhp01_increment_c_depth_metrics_20260611.csv`
  - `fdhp01_increment_c_frozwt_frdp_ratio_20260611.csv`
  - `fdhp01_increment_c_activation_summary_20260611.csv`
- The first thaw-arm attempt failed on `p1` at 1990 day 45 with
  `wb18_perc_frzw_0001=0.06135293352005228` exceeding
  `wb18_perc_ul_0001=0.05875247947169813`.
- A source cap made the cohort execute cleanly (`43/43`), but the D2 hard
  stop failed: years 2-6 `Total-Soil + frozwt` max abs residual was
  `2325832826960980.0 mm`.
- The attempted tree produced catastrophic `Total-Soil` amplification during
  freeze/thaw cycles; for `p1`, year 4 reached `1.558719e+35 mm`.
- The attempted production, contract, and test edits were backed out. The
  package remains at the Increment B boundary (`SC-SNOWFREEZE-001` v59).

## Increment C1 Attempt Evidence

- Persisted compact reports:
  - `d3-increment-c1-capacity-redistribution-20260611.md`
  - `fdhp01_increment_c1_execution_summary_20260611.json`
  - `fdhp01_increment_c1_run_summary_20260611.json`
  - `fdhp01_increment_c1_run_status_20260611.tsv`
  - `fdhp01_increment_c1_annual_closure_residuals_20260611.csv`
  - `fdhp01_increment_c1_depth_metrics_20260611.csv`
  - `fdhp01_increment_c1_frozwt_frdp_ratio_20260611.csv`
  - `fdhp01_increment_c1_activation_summary_20260611.csv`
  - `fdhp01_increment_c1_p43_aggregate_cap_smoke_20260611.json`
- The parent model ran C1 locally without the comparator subagent per user
  direction. The full cohort ran `43/43` clean but failed the D2 hard stop:
  years 2-6 max abs residual `16628.157022818832 mm`.
- A p43 aggregate-cap smoke collapsed storage overfill to
  `ProfilePorosityCap`, but annual closure still failed by up to
  `200.39845415539014 mm`.
- The attempted production, contract, and test edits were backed out. The
  package remains at the Increment B boundary (`SC-SNOWFREEZE-001` v59).

## Increment C1a Diagnostic Evidence

- Persisted compact reports:
  - `d3-increment-c1a-seam-accounting-20260611.md`
  - `fdhp01_increment_c1a_seam_accounting_summary_20260611.json`
  - `fdhp01_increment_c1a_seam_ledger_excerpt_20260611.csv`
- C1a ran without the comparator subagent per user quota direction. A
  temporary env-gated ledger patch was removed before commit.
- Both p43 and p1 failed at `HKERNEL-WB14-RUNOFF-E-003` on simulation day 94;
  the day-93 write already exceeded aggregate `ul` by `50.049070656902806 m`
  on p43 and `50.644102740198335 m` on p1.
- The first fine-layer capacity excess was present on day 1, and shadow
  `frwatc(1)` residuals reached about `33 m` before the day-93 re-freeze.
- C1b is bound to the appended C1a seam accounting specification in
  `d3-fine-sublayer-port-scope.md`: single daily ingress, fine-state
  ownership until egress, capacity-bound freezing, explicit `watpdg`/`watbtm`
  identity routing, and wholesale coarse-state recomputation at egress.

## Increment C1b Gate Evidence

- Persisted compact reports:
  - `d3-increment-c1b-capacity-overflow-20260612.md`
  - `fdhp01_increment_c1b_execution_summary_20260612.json`
  - `fdhp01_increment_c1b_run_status_20260612.tsv`
  - `fdhp01_increment_c1b_annual_closure_residuals_20260612.csv`
  - `fdhp01_increment_c1b_depth_metrics_20260612.csv`
  - `fdhp01_increment_c1b_frozwt_frdp_ratio_20260612.csv`
  - `fdhp01_increment_c1b_starter_capacity_20260612.json`
- C1b ran without the comparator subagent per user quota direction. The full
  local cohort ran `43/43` clean at
  `/tmp/fdhp01_increment_c1b_cohort_final14_20260612T035618Z`.
- Years 2-6 `Total-Soil + frozwt` closure remains at numerical noise: max abs
  residual `1.5347723092418164e-12 mm`, mean abs
  `1.0758525853139703e-13 mm`.
- Year 7 no longer carries the prior B-boundary `1.268e-7 mm` watch magnitude:
  max abs residual `6.963318810448982e-13 mm`.
- p1/p43 starter traces scanned the first 100 simulation days with zero
  `frzw > ul` rows. Minimum margins were `0.020482917898791884 m` on p1 and
  `0.020378509421531917 m` on p43.
- Valid-input capacity guards did not trip on the cohort. Profile-bound
  pinning remains removed (`0/43` pinned), and `frozwt/frdp` max correlation
  is `0.9860178382757524`, below Increment B's `0.9861968090242198`.
- The depth watch remains open and sharper: mean maximum depth is
  `1791.9747961835646 mm`, worse than Increment B's `1782.265765656973 mm`.
  C2 must handle thaw arms and explain the remaining freeze-side
  energy/resistance depth magnitude.

## D3 Attempt Evidence

- A coarse continuous per-layer energy-front attempt was run from dirty commit
  `8d13ba898a111aeed97375cc997d0ca65d6b85b7` into
  `/tmp/fdhp01_d3_layered_energy_20260611T085142Z`.
- The attempt ran `43/43` clean and improved median maximum depth to
  `490.774886655666 mm`, but did not meet the package phase boundary:
  mean max depth remained `643.2973898432339 mm`, max depth remained
  `1789.9130899451595 mm`, median correlation was `-0.1876255663636445`,
  and median frozen-duration delta was `-428` days.
- Static legacy inspection localized the missing behavior to the fine-sublayer
  `frostn` state machine: `frostn.for:360-683`, `frzng.for:235-560`, and
  `frwatc.for:89-137`.
- The coarse-front production/test edit was backed out. `SC-SNOWFREEZE-001`
  v57 remains as the D3 hold amendment, v58 supersedes it for Increment A
  shadow-state handoff authority, v59 supersedes it for Increment B active
  freeze-arm authority, and v61 supersedes it for Increment C1b capacity/
  overflow authority.

## Review Closure

Claude's post-closeout review findings are accepted. The diagnostic conclusion
from addendum 2e was correct: the scalar model could not satisfy the v150/v151
audit identity by construction. The package still cannot be marked complete
because D3 depth/duration parity is open. The first Increment C attempt shows
the next pass must not reintroduce top/bottom thaw arms without first porting
capacity-aware `watdst` redistribution and `watpdg`/`watbtm` overflow handling.
The C1 attempt adds that capacity enforcement alone is insufficient unless the
overflow surfaces are reconciled with the WAT balance identity. C1a supplied
that accounting specification, and C1b now lands the water-side infrastructure
with conservation intact. The years 2-6 conservation gate remains the hard
stop before any depth/duration acceptance claim; C2 is the next required D3
increment.
