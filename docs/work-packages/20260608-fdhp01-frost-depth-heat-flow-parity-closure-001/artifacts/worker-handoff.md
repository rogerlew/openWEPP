# worker handoff

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-12

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
- `SC-SNOWFREEZE-001` is now v61 after C1b. Fine-layer ice/liquid capacity is
  bounded by total pore capacity, valid overflow is routed through
  `watpdg`/`watbtm`, `watbtm` contributes to WB13 `Dp`, and bounded WB18/WB13
  roundoff canonicalization is contract-authorized.
- `SC-SNOWFREEZE-001` is now v62 after C2. Top/bottom thaw, sandwich geometry,
  `fgthwd`, `nwfrzz` release, `watpdg`/`watbtm` capacity-routed overflow, and
  non-amplifying repeated freeze/thaw conservation are contract-authorized.
- `SC-SNOWFREEZE-001` is now v64 after Dc1. Seasonal `tmpbl`/`Qdry`
  lower-front heat, in-hour thaw resistance feedback, and bounded fine-theta
  lower-bound roundoff canonicalization are contract-authorized. The stable
  lower-front heat surrogate remains retired.

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

Increment C1 attempt result:

- Codex attempted capacity guards, downward fine-layer redistribution, and
  `watpdg`/`watbtm` surfaces without the comparator subagent per user
  direction.
- The full local cohort ran `43/43` clean at
  `/tmp/fdhp01_increment_c1_capacity_fix_20260611T224555Z`, but years 2-6
  `Total-Soil + frozwt` closure failed with max abs residual
  `16628.157022818832 mm`.
- A p43 aggregate-cap smoke proved the profile-overfill part is localizable:
  storage capped at `809.0776779996984 mm` against
  `ProfilePorosityCap=809.0776779996982 mm`, but annual closure still missed
  by up to `200.39845415539014 mm`.
- The attempted production/contract/test edits were backed out.

Increment C1a diagnostic result:

- Codex ran the seam-accounting diagnostic without the comparator subagent.
  The temporary ledger patch was removed before commit.
- p43 and p1 both failed at `HKERNEL-WB14-RUNOFF-E-003` on simulation day 94;
  the day-93 write exceeded aggregate `ul` by about `50 m`, and the first
  fine-layer capacity excess was already present on day 1.
- The appended C1a spec in `d3-fine-sublayer-port-scope.md` is now binding
  for the next implementation increment: single daily ingress, fine-state
  ownership until egress, capacity-bound freezing, explicit
  `watpdg`/`watbtm` identity routing, and wholesale coarse-state recomputation
  at egress.

Increment C1b result:

- Codex landed the capacity/overflow infrastructure without the comparator
  subagent per user quota direction.
- p1/p43 starter traces over the first 100 simulation days had zero
  `frzw > ul` rows and minimum margins of `0.020482917898791884 m` and
  `0.020378509421531917 m`.
- The local `43/43` cohort at
  `/tmp/fdhp01_increment_c1b_cohort_final14_20260612T035618Z` ran clean with
  no valid-input capacity guard trips.
- Years 2-6 `Total-Soil + frozwt` closure stayed at noise: max abs residual
  `1.5347723092418164e-12 mm`.
- Year 7 closure also stayed at noise: max abs residual
  `6.963318810448982e-13 mm`.
- Profile-bound pinning did not regress (`0/43` pinned) and `frozwt/frdp` max
  correlation stayed below Increment B (`0.9860178382757524`).
- Depth magnitude did regress as a watch item: mean maximum depth is
  `1791.9747961835646 mm`, still near the profile bound.

Increment C2 result:

- Codex landed the thaw-arm state machine without the comparator subagent per
  user quota direction.
- The authoritative local hourly cohort at
  `/tmp/fdhp01_increment_c2_cohort_hourly_fix_20260612T035740Z` ran `43/43`
  clean with `43/43` WAT outputs and `selected_lane=hourly`.
- Years 2-6 and year-7 `Total-Soil + frozwt` closure remained at the package
  C1b additive-storage ledger noise floor (`0.0 mm` in the retained annual
  table).
- D2/p2 did not reopen, profile-bound pinning remains removed (`0/43` pinned),
  and `frozwt/frdp` max correlation is `0.9441102161636825`.
- D3 acceptance still fails: mean maximum depth is `1793.52198510966 mm`,
  median depth correlation is `-0.16722397856345997`, open-minus-legacy frozen
  duration median is `111` days, and median days above `200 mm` is `815`.

Increment Da result:

- Codex executed the diagnostic p1 hourly energy characterization without the
  comparator subagent per user quota direction.
- The temporary trace ran at
  `/tmp/fdhp01_increment_da_trace_20260612T043800Z` and was removed before the
  production rebuild. No Da trace marker remains under `crates/`.
- Legacy `frzng.for` updates `qoutdm` and recomputes `qhtout` inside the
  3600-second freeze loop whenever the front advances through fine layers.
- The p1 trace proves openWEPP is missing that in-hour feedback. Year 1 day 1
  hour 2 advances `0.000397484 -> 1.162927773 m` while retaining resistance
  `0.000227134 m2 C/W`; applying the hour-end frozen path gives
  `0.572822749 m2 C/W` and drops `|qhtout|` from `35602.871` to
  `14.117 W/m2`.
- The repaired independent annual WAT ledger closes years 2-6 to
  `1.3813070645629644e-07 mm`; p43 year 2 is
  `-1.912025027195341e-08 mm`, cleared as WAT-surface numerical texture.
- The fresh Da cohort at `/tmp/fdhp01_increment_da_cohort_20260612T044217Z`
  ran `43/43` clean and is WAT-row-identical to C2.

Increment Db result:

- Codex executed the `frzng` in-hour resistance feedback implementation
  without the comparator subagent per user quota direction.
- The new within-hour red test failed before the runtime fix with one cold
  hour advancing `0.1996 m`, then passed after Db recomputed resistance/`Qsrf`
  after each fine-layer front advance.
- The fresh Db cohort at `/tmp/fdhp01_increment_db_cohort_20260612T051524Z`
  ran `43/43` clean.
- The independent years 2-6 WAT ledger remains at WAT-publication numerical
  texture: max abs residual `1.9976620946327017e-07 mm`; p43 year 2 is
  `-5.3290705182007514e-14 mm`.
- Db fixes maximum-depth runaway: `0/43` profile pins, all 43 maximum depths
  inside the legacy `240..503.2 mm` envelope, mean max
  `409.16220799389805 mm`, median max `407.3294069097544 mm`.
- D3 remains open: median depth correlation is `-0.05296014769462692`, and
  frozen duration under-persists with median open-minus-legacy `-452` days.

Increment Dc result:

- Codex attempted the combined seasonal lower-front heat plus thaw-resistance
  pass without the comparator subagent per user quota direction.
- The fresh Dc cohort at `/tmp/fdhp01_increment_dc_cohort_20260612T062840Z`
  ran `43/43` clean, but failed the D2 hard stop and D3 envelope.
- Years 2-6 independent WAT ledger regressed to max abs
  `0.2706094484356498 mm`; p43 year 2 regressed to
  `-0.24479853886504088 mm`.
- Depth correlation improved to median `0.6595441080376979`, but depth and
  duration regressed: mean max depth `1062.5086535449198 mm`, one profile pin,
  `0/43` prefixes inside the `240..503.2 mm` envelope, and median frozen
  duration `+751` days versus legacy.
- The Dc production, contract, and test edits were backed out. At the post-Dc
  backout boundary, production returned to Db / `SC-SNOWFREEZE-001` v63 until
  the split Dc1 pass.

Increment Dc1 result:

- Codex landed the split accounting-preserving Dc1 pass without the comparator
  subagent per user quota direction.
- The fresh Dc1 cohort at
  `/tmp/fdhp01_increment_dc1_cohort_20260612T101238Z` ran `43/43` clean with
  `43/43` WAT outputs.
- Years 2-6 independent `Total-Soil + frozwt` closure returned to
  WAT-publication texture: max abs residual
  `6.471338602487275e-07 mm`; p43 year 2 is
  `-1.1013412404281553e-13 mm`.
- D3 remains open: mean maximum depth is `1146.5109665924424 mm`, one prefix
  pins at the profile bound, no prefix is inside the legacy `240..503.2 mm`
  envelope, median depth correlation is `0.6415921721982907`, and frozen
  duration over-persists with median open-minus-legacy `+567` days.
- `SC-SNOWFREEZE-001` v64 is the accepted production boundary.

Increment Dd result:

- Codex executed the legacy-snow-forced diagnostic without the comparator
  subagent per user quota direction. The temporary forced-snow hook was removed
  before the production rebuild.
- Legacy winter output generation ran `43/43` clean under
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd`; the forced openWEPP
  cohort ran `43/43` clean at
  `/tmp/fdhp01_increment_dd_forced_snow_cohort_20260612T121500Z`.
- Years 2-6 independent `Total-Soil + frozwt` closure stayed at
  WAT-publication texture: max abs residual
  `6.726058817130287e-07 mm`; p43 year 2 is
  `-1.2079226507921703e-13 mm`.
- Forced legacy snow removes profile-bound pinning (`0/43` pinned) and improves
  median depth correlation to `0.7118806632341061`, but it does not close D3:
  mean max depth is `856.817674502367 mm`, `0/43` prefixes are inside the
  legacy `240..503.2 mm` envelope, and frozen duration over-persists with
  median open-minus-legacy `+502` days.

First actionable item: scope and execute the next frost-side localization
increment under the Dd controlled-snow setup. Compare openWEPP hourly `frzflg`,
`Qsrf`, `Quf`, surface temperature, snow/residue/frozen resistance, front
advance/retreat, and fine-layer ice/liquid motion against legacy
`H*.winter.dat`/source-line state for the first material forced-snow
divergence. Do not tune snow density/depth, kfactor, latent heat, publication,
or D2 storage surfaces. Do not advance to MOFE until the cohort is `43/43`, the
year-7 boundary item stays explained or eliminated, and the full
depth/duration acceptance gate passes or is assigned to a documented upstream
snow handoff with evidence.

Increment De result:

- Codex landed the legacy content-dependent `Qdry` conductivity correction
  without the comparator subagent per user quota direction.
- `SC-SNOWFREEZE-001` is v65; the lower-front heat path now uses the
  `frostn.for:430-458` fine-layer polynomial/harmonic conductivity with `0.2`
  only as the dry fallback, and bottom thaw uses the same lower-front
  conductivity.
- Full Rust closure and authority guards passed: `cargo fmt --check`,
  `git diff --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`,
  `bash tools/release/check_authority_suite_antievasion.sh`, and
  `cargo test --test auth11_required_suite_obligation_guards_contract`.
- Native production cohort:
  `/tmp/fdhp01_increment_de_native_cohort_final_20260612T171358Z`, `43/43`
  clean, years 2-6 closure `5.474257917248426e-07 mm`, mean max depth
  `705.505148615878 mm`, median duration residual `+288` days.
- Forced legacy-snow diagnostic cohort:
  `/tmp/fdhp01_increment_de_forced_snow_cohort_20260612T171017Z_proper`,
  `43/43` clean, years 2-6 closure `4.355148297552347e-07 mm`, mean max
  depth `655.9890274782282 mm`, median depth correlation
  `0.770042438411068`, median duration residual `+186` days.
- D3 remains open: forced snow plus De still has `0/43` prefixes inside the
  legacy `240..503.2 mm` maximum-depth envelope.

De-to-Df historical first actionable item, completed by Increment Df: scope and
execute the next frost-side
localization increment under the De forced-snow setup. Use paired hourly
trajectory evidence to locate the first remaining material front/flux
divergence after content-dependent `Qdry`: openWEPP hourly `frzflg`, `Qsrf`,
`Quf`, lower-front heat, surface/frozen/residue/snow resistance, front
advance/retreat, and fine-layer ice/liquid motions against legacy
`H*.winter.dat` plus source-line state. Do not tune snow density/depth,
kfactor, latent heat, WAT publication, or D2 storage surfaces.

Increment Df result:

- Codex executed the p1/p2 paired hourly localization without the comparator
  subagent per user quota direction.
- A temporary env-gated forced-snow + hourly trace hook was run at
  `/tmp/fdhp01_increment_df_trace2_20260612T175406Z` and removed before the
  clean-source production rebuild. No `OPENWEPP_FDHP01_DF`/`fdhp01_df` source
  marker remains under `crates/`.
- The legacy winter `ground` column is ground-drift snow, not temperature.
- First material divergence is year 1 day 1 hour 2 on both p1 and p2, before
  snow is involved: legacy frost depth `5.0 mm`; openWEPP `42.057866709 mm`
  on p1 and `41.417581693 mm` on p2.
- The localized seam is surface resistance. openWEPP passes
  `residue_depth_m = 0.0` to frost while legacy uses `23.0 mm`, and openWEPP
  omits the legacy shallow-front `dpfsfl` minimum top-frozen conduction
  distance. At the first divergence, the conservative legacy resistance
  estimate is roughly `899x` (p1) and `951x` (p2) larger than openWEPP.
- Df is diagnostic-only; production remains at the De boundary and clean-source
  release SHA
  `981da203d9ced9b1d73f049fa3a4b227710862a3dbecaad9d4619f03ae7dd2d5`.

Df-to-Dg historical first actionable item, completed by Increment Dg: execute
Increment Dg from
`d3-staged-increment-plan.md`. Port the legacy residue-depth frost resistance
surface and the shallow-front `dpfsfl` minimum conduction distance, add focused
red tests from the Df p1/p2 fixtures, then run the De forced-snow certification
and native cohort. Do not tune snow density/depth, kfactor, latent heat, WAT
publication, or D2 storage surfaces.

Increment Dg result:

- Codex landed the residue-depth frost resistance surface and below-freezing
  shallow-front `dpfsfl` conduction floor without the comparator subagent per
  user quota direction.
- `SC-SNOWFREEZE-001` is now v66. It binds the legacy `resdep/kres` lineage
  and the shallow-front `dpfsfl = dg(1) / nfine(1) / 2` conduction-distance
  floor for active below-freezing frost heat paths.
- The native production cohort at
  `/tmp/fdhp01_increment_dg_native_cohort_20260612T184601Z` ran `43/43` clean
  with `43/43` WAT outputs. Years 2-6 independent
  `Total-Soil + frozwt` closure stayed at WAT-publication texture: max abs
  residual `6.261351281899863e-07 mm`.
- Native depth/duration improved materially from De: mean max depth
  `498.08123930883653 mm`, median max `488.08620069478803 mm`, `30/43`
  prefixes inside the legacy `240..503.2 mm` envelope, median depth
  correlation `0.7551022199950611`, and median frozen-duration residual
  `+84` days.
- The forced legacy-snow diagnostic cohort at
  `/tmp/fdhp01_increment_dg_forced_snow_cohort_20260612T185203Z` ran `43/43`
  clean with `43/43` WAT outputs. Years 2-6 independent closure stayed at
  `5.835723933533821e-07 mm`.
- Dg passes its directional forced-snow gate: mean max depth improved
  `655.9890274782282 -> 490.0923199552928 mm`, median max improved
  `652.3375464029963 -> 479.356967770298 mm`, envelope membership improved
  `0/43 -> 30/43`, and median frozen-duration residual improved
  `+186 -> +73` days.
- FDHP01 remains `executed-hold`: `13/43` forced-snow prefixes still exceed
  the `503.2 mm` upper envelope bound (`p1`, `p2`, `p3`, `p8`, `p11`, `p13`,
  `p20`, `p21`, `p22`, `p23`, `p26`, `p28`, `p32`), so the package D3
  acceptance boundary is not closed.
- The temporary forced-snow diagnostic hook was removed before the final
  production rebuild. Clean-source release binary SHA:
  `3275db431339402596a27a28d7976062eb4655771e9e159fdf929fa1410883ad`.

Dh result:

- Codex executed the proposed per-soil frozen-path conductivity increment
  without the comparator subagent per user quota direction.
- Static pinned-source inspection refuted the implementation premise before
  production edits: `frostn.for:188-193` sets fixed `kftill = 1.75` and
  `kfutil = 2.1`; `frostn`/`frzng`/`frznw` consume those constants in the
  frozen surface path.
- The soil-property-dependent `bdcons`/`slsw`/`ksoilf` expression is the
  lower-front unfrozen `kufzfl` path already handled by Increment De, not a
  per-soil replacement for frozen-path `kftill`/`kfutil`.
- `SC-SNOWFREEZE-001` is now v67 with
  `REF-SNOWFREEZE-LEGACY-FROZEN-PATH-KF`, and a contract regression test
  prevents replacing fixed `kftill`/`kfutil` with per-soil frozen-path
  conductivity absent superseding authority.
- No production physics was changed. FDHP01 remains `executed-hold`; the Dg
  forced-snow outlier set remains the active residual.

Di result:

- Codex executed the post-Dg paired hourly re-localization without the
  comparator subagent per user quota direction.
- A temporary env-gated trace/forced-snow hook was run for plateau
  representatives `p8`, `p20`, and `p2`, then removed before package updates.
- The paired traces localize the residual to missing legacy winter
  surface-temperature synthesis: openWEPP currently feeds below-freezing
  hourly air temperature directly into frost surface heat flow, while pinned
  legacy computes `surtmp(hour)` through `hr_tmp`/`tmpadj` before `frostn`
  consumes it.
- Deep divergent advance is surface-path dominated. Forced snow is present and
  open `surface_temp_c` is negative for `0.997852`, `0.999063`, and
  `1.000000` of the advance on `p8`, `p20`, and `p2`; median surface-flux
  share is `1.000000`, `1.000000`, and `0.994355`.
- Topology, hourly snow-depth mismatch, lower-front heat, and deep-layer
  latent cost are secondary discriminators. `p2` is larger in magnitude but
  not a distinct mechanism in the Di trace.
- No production physics was changed. FDHP01 remains `executed-hold`.

Updated first actionable item: execute Increment Dj from
`artifacts/d3-staged-increment-plan.md`: port or expose the pinned legacy
`hr_tmp`/`tmpadj` `surtmp(hour)` synthesis into the frost surface heat path,
then rerun the Di representatives, the forced-snow cohort, the native cohort,
and the independent years 2-6 `Total-Soil + frozwt` closure gate. Do not
retune snow density/depth, `kfactor`, latent heat, WAT publication, D2
storage, residue depth, `dpfsfl`, fixed `kftill`/`kfutil`, or lower-front
`Qdry`. If required `tmpadj` inputs are missing at the frost seam, hold with a
named missing-input boundary rather than substituting a proxy surface
temperature.
