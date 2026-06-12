# Gate Results

Status: executed-hold

Evidence mode: Ran

Date: 2026-06-12

## Required Gates

| Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |

## Focused Regression Gates

| Command | Result |
|---|---|
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 22 tests after Increment A shadow-state diagnostics |
| `cargo test --test cli04_runner_wat_parquet_contract_derived_tests -- --nocapture` | Pass, 2 tests |
| `cargo test --test sim_contract_boundary_unit_registry -- --nocapture` | Pass, 14 tests |
| `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture` | Pass, 8 tests |
| `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` | Pass, 3 tests |
| `cargo test -p openwepp-runner --lib hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture` | Pass, 1 test |
| `cargo test -p openwepp-hillslope-output schema_includes_required_dataset_metadata_keys -- --nocapture` | Pass, 1 test |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v58 / `SC-WATBAL-001` v152 updates |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v58 / `SC-WATBAL-001` v152 updates |

## Dependency / Authority Guards

`cargo deny check` reported:

```text
advisories ok, bans ok, licenses ok, sources ok
```

No source-level anti-evasion guard was run for this package because FDHP01 did
not edit external-authority suite posture, cohort fixture bindings, or
required-case bindings. The auth11 guard test was included in the successful
workspace test run.

## Post-Review Cohort Gate

| Command / Gate | Result |
|---|---|
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass |
| D1 focused gates: `cargo fmt --check`; WB13 runner unit; summary accumulator lib; HPHYS0203; HPHYS0208; CLIM06 frost suite | Pass |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_closure_after_d1_restored_20260611T053545Z` | Fail, `42/43` clean exits; `p2` failed before WAT publication at `HKERNEL-WB11-PERC-E-003` |
| Annual closure on emitted prefixes | Fail, max abs residual `2.4798612273409617 mm` after D1; pre-D1 post-review residual was `75.43917280313423 mm` |
| FDMC01 depth/duration movement on emitted prefixes | Fail, max-depth mean `1782.2670980346527 mm`, median correlation `-0.10301692862035305`; duration delta diagnostic only because closure failed |

## D2 Exchange Diagnostic Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt` | Pass, formatting applied |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 17 tests; freeze-onset and warm-thaw diagnostics reconcile liquid/frozen exchange |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-WATBAL-001` v150 update |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-WATBAL-001` v150 update |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |

## D2 Frozwt Publication Source Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` | Pass, 3 tests; WAT `frozwt` requires `frost.runtime_frwatc_frozen_water_after_m` and rejects a missing exchange-store symbol |
| `cargo test -p openwepp-runner --lib hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture` | Pass, proves WAT `frozwt` follows the exchange diagnostic instead of `runtime_ws_frz` |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-WATBAL-001` v151 update |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-WATBAL-001` v151 update |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 17 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_frozwt_publication_20260611T070334Z` | Fail, `42/43` clean exits; `p2` failed before WAT publication at `HKERNEL-WB11-PERC-E-003` |
| Annual closure on emitted prefixes | Fail, max abs residual `2.4798612273409617 mm`; unchanged from the post-D1 floor |
| `frozwt/frdp` cohort ratio audit | Fail, `35297` frost-active days; min correlation `0.9999999999999994`, median ratio median `0.15199999999999997`, max ratio standard deviation `3.2273877788806054e-17` |

## Layered Frozen-Store Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 19 tests; layered store rejects scalar `frdp * theta` equivalence and updates per-layer `frzw` |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v57 / `SC-WATBAL-001` v152 |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v57 / `SC-WATBAL-001` v152 |
| `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` | Pass, 3 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `b4cb18a728d1556c2ba50f28d3fe7671f306735a82aa5153682a40ad72c69c8e` |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_layered_store_20260611T080722Z` | Pass for execution, `43/43` clean exits |
| Annual `Total-Soil + frozwt` closure | Pass, max abs residual `1.2683574368566042e-07 mm`, mean abs residual `2.1277404919798806e-09 mm` |
| Soil-only annual identity | Expected fail under v152 additive storage term, max abs residual `119.04111532237937 mm` |
| `frozwt/frdp` cohort ratio audit | Pass, no exact scalar ratio: `36064` frost-active days, min correlation `0.8210678396408895`, median correlation `0.963536279373424`, max ratio standard deviation `0.0700106996666242` |
| FDMC01 depth/duration movement | Fail, max-depth mean `1782.0379909380451 mm` versus legacy `414.22093023255815 mm`; median depth correlation `-0.27756218032931956`; frozen-day delta `-518.5348837209302` |

## D3 Coarse-Front Attempt Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass before cohort attempt |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 20 tests in the temporary D3 attempt |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `86a82da1f31780c27aacc4a7694193fba53363e0107f56360c10aa146412d160` |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_d3_layered_energy_20260611T085142Z` | Pass for execution, `43/43` clean exits |
| FDMC01 depth/duration movement | Fail, median max depth improved to `490.774886655666 mm` but mean max depth remained `643.2973898432339 mm`, max depth `1789.9130899451595 mm`, median depth correlation `-0.1876255663636445`, and median frozen-day delta `-428` |
| Annual closure reconstruction | Excluded from package evidence; the committed layered-store report's `outputs` field was not reproducible from exposed WAT columns alone during this pass |
| Production/test landing | Backed out; at that attempt boundary only `SC-SNOWFREEZE-001` v57 and D3 attempt artifacts remained |

## D3 Fine-Sublayer Attempt Gates

| Command / Gate | Result |
|---|---|
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass before cohort attempt; release binary SHA `e81e65713378f082fc5a49f6596a6809f756b65fc328d9bb90cabef10e78c9e7` |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_d3_fine_sublayer_20260611T160601Z` | Pass for execution, `43/43` clean exits |
| Annual `Total-Soil + frozwt` closure | Fail, max abs residual `70.27250390582333 mm`; years 2-6 max residual also `70.27250390582333 mm` |
| FDMC01 depth/duration movement | Fail, max-depth mean `1782.0386969356455 mm` versus legacy `414.22093023255815 mm`; median depth correlation `-0.275982943044058`; mean frozen-day delta `-520.953488372093` |
| p22 mass-coupled helper probe, `/tmp/fdhp01_d3_mass_probe_20260611T162003Z` | Fail, max abs years 2-6 residual `86.995866585106 mm` |
| p22 aggregate exchange probe, `/tmp/fdhp01_d3_exchange_probe_20260611T162312Z` | Fail, max abs years 2-6 residual `86.995866585106 mm` |
| Direct fine-liquid egress probe | Failed focused thaw diagnostic: expected `8.109464696602291`, observed `0.011923545603720697` |
| Post-backout `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, `19/19` |
| Production/test landing | Backed out; no D3 fine-sublayer production behavior landed |

## D3 Increment A Shadow-State Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo test -p openwepp-hillslope-output hillslope_wat -- --nocapture` | Pass, 4 tests; deterministic WAT bytes and file field metadata both preserved |
| `cargo clippy -p openwepp-hillslope-output --all-targets -- -D warnings` | Pass |
| `cargo test --test cli04_runner_wat_parquet_contract_derived_tests -- --nocapture` | Pass, 2 tests |
| `cargo test --test sim_contract_boundary_unit_registry -- --nocapture` | Pass, 14 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 22 tests; includes Increment A shadow fine-state vectors |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v58 |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v58 |
| Corrected 43-prefix latest current cohort | Pass, `43/43` clean exits at `/tmp/fdhp01_increment_a_current_pre_like_pre_1_20260611T181018Z`; release binary SHA `cd3a2318550e641c94d3c54a8dc7bf5dacf42cc80d57178e521b4215ae75c12b` |
| Pre vs current `H.hbp` physical byte equality | Pass, `43/43` |
| Pre vs current `H.loss.json` physical byte equality | Pass, `43/43` |
| Pre vs current WAT decoded row/column equality | Pass, `43/43` |
| Pre vs current fallback warning counts | Pass, `43` vs `43`, mismatch runs `0` |
| Pre vs current `H.wat.parquet` physical byte equality | Fail, `0/43`; isolated to preexisting nondeterministic `ARROW:schema` footer bytes in the clean pre baseline |
| Current-vs-current `H.hbp`, `H.loss.json`, and `H.wat.parquet` physical byte equality | Pass, `43/43` for all three surfaces across `/tmp/fdhp01_increment_a_current_pre_like_pre_1_20260611T181018Z` and `/tmp/fdhp01_increment_a_current_pre_like_pre_2_20260611T181018Z` |
| Current-vs-current decoded WAT equality and fallback warning counts | Pass, decoded `43/43`; warnings `43` vs `43`, mismatch runs `0` |
| Full workspace gates after final WAT footer minimization | Pass: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`; compact summary `fdhp01_increment_a_gates_latest_20260611.json` |

Increment A disposition: local contract and metadata gates pass, non-WAT
outputs are byte-identical to the clean pre baseline, and WAT decoded payloads
are identical. The literal pre-vs-current physical WAT byte gate is invalidated
by the old baseline's nondeterministic parquet footer, now fixed and proven by
the current-vs-current `43/43` physical parity run. Full latest-source Rust
gates pass.

## D3 Increment B Freeze-Arm Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 26 tests; includes Increment B dispatch, fine-front energy, `frznw`, and `watdst` vectors |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v59 |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v59 |
| `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture` | Pass, 8 tests after the threshold-bounded exchange-debit boundary fix |
| 43-prefix `algebraic-radium` frost-on cohort | Pass, `43/43` clean exits; wrappers wrote outputs under `/tmp/frostval01_rerun_20260611T020951Z/outputs`, with CLI manifests under `/tmp/fdhp01_increment_b_final_20260611T193423Z/outputs` |
| Annual `Total-Soil + frozwt` closure, years 2-6 | Pass, max abs residual `3.0880187296133954e-11 mm`, mean abs residual `1.2662284657486707e-11 mm` |
| Year-7 boundary watch item | Still present at tiny magnitude, max abs residual `1.2683569483584733e-07 mm` |
| Year-1 initialization residual | Recorded outside the staged gate, max abs residual `1.0505061950707386 mm` |
| Profile-bound pinning directional gate | Pass, `0/43` prefixes pinned at `ProfileDepth`; minimum margin `16.63152804088827 mm` |
| `frozwt/frdp` scalar-signature gate | Pass, max per-prefix correlation `0.9861968090242198`, median `0.9635362793734238`, below the rejected `0.9987` scalar signature |

Increment B disposition: staged gates pass, but D3 remains `executed-hold`.
Depth no longer pins exactly at the profile bound, but remains near it
(mean max `1782.265765656973 mm`). The first Increment C thaw-arm attempt is
recorded below; the next D3 pass must add capacity-aware `watdst`
redistribution plus `watpdg`/`watbtm` overflow handling before retaining
top/bottom thaw arms.

Disposition: Rust gates and D2 storage closure pass, but package acceptance
still fails on D3 frost-depth parity. FDHP01 is executed-hold.

## D3 Increment C Thaw-Arm Attempt Gates

| Command / Gate | Result |
|---|---|
| Required comparator-suite runner | Unavailable; spawned subagent errored due GPT-5.3-Codex-Spark usage limit, so heavy gates were run locally |
| `cargo fmt --check` / `git diff --check` | Pass before cohort failure disposition |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass after test-helper cleanup |
| `cargo test --workspace` | Pass before cohort failure disposition |
| `cargo deny check` | Pass |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 30 tests in the attempted tree after adding a pore-cap regression |
| First cohort attempt | Failed on `p1`, 1990 day 45: `wb18_perc_frzw_0001=0.06135293352005228` exceeded `wb18_perc_ul_0001=0.05875247947169813` |
| Post-cap 43-prefix cohort | Pass for process execution, `43/43` clean exits |
| Annual `Total-Soil + frozwt` closure, years 2-6 | Fail, max abs residual `2325832826960980.0 mm`, mean abs residual `863664411656061.6 mm` |
| Year-7 boundary watch item | Fails as a real storage blow-up, max abs residual `2203549546983243.5 mm` |
| Profile-bound pinning directional gate | Directionally unpinned, `0/43` pinned, but still near the profile bound: mean max `1794.0628184427708 mm`, minimum margin `5.350358610292005 mm` |
| Depth correlation | Fail, median `-0.4265170275507577` |
| Frozen duration | Fail for acceptance shape: median open-minus-legacy delta `+382` days |

Increment C disposition: failed and backed out. The attempt proves thaw arms
need capacity-aware `watdst` redistribution plus `watpdg`/`watbtm` overflow
handling before `mlttp`/`mltbtm` can be retained. FDHP01 remains
`executed-hold` at the Increment B boundary.

## D3 Increment C1 Capacity/Redistribution Attempt Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user reported GPT-5.3-Codex-Spark weekly quota exhausted, so parent ran local cohort and metrics |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_increment_c1_ -- --nocapture` | Pass during attempted tree, 3 C1-focused tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass before cohort, 29 tests in attempted tree |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass before cohort |
| 43-prefix `algebraic-radium` frost-on cohort | Pass for process execution, `43/43` clean exits; run root `/tmp/fdhp01_increment_c1_capacity_fix_20260611T224555Z` |
| Annual `Total-Soil + frozwt` closure, years 2-6 | Fail, max abs residual `16628.157022818832 mm`, mean abs residual `6238.80817440851 mm` |
| Profile-bound pinning directional gate | Pass, `0/43` pinned prefixes |
| `frozwt/frdp` not regressed vs Increment B | Fail, max correlation `0.9919091097937477` versus Increment B max `0.9861968090242198` |
| p43 aggregate-cap localization smoke | Partial localization only: max storage collapsed to `809.0776779996984 mm` against `ProfilePorosityCap=809.0776779996982 mm`, but annual closure still missed by up to `200.39845415539014 mm` |
| Production/contract/test landing | Backed out; C1 did not meet the D2 hard stop |
| Post-backout `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, `26/26`, confirming return to the Increment B test boundary |
| Post-backout `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, `5/5`, confirming `SC-SNOWFREEZE-001` v59 expectation restored |
| Post-backout `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, `3/3`, confirming `SC-SNOWFREEZE-001` v59 expectation restored |

Increment C1 disposition: failed and backed out. The attempt proves capacity
enforcement is necessary but insufficient unless `watpdg`/`watbtm`
redistribution is reconciled with the WAT balance identity. FDHP01 remains
`executed-hold` at the Increment B boundary.

## D3 Increment C1b Capacity/Overflow Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local DuckDB/CLI cohort comparisons. |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass, `advisories ok, bans ok, licenses ok, sources ok` |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `e7135eda1a751d8acb16dbca2096c12d3c10f6589ae84f55ff57efe45ed1f27e` |
| `git diff --check` | Pass |
| Debug-marker search | Pass for C1b markers; only pre-existing CLI `eprintln!` paths and an unrelated historical doc mention were found |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |
| C1b focused CLIM06 capacity/overflow vectors | Pass in `cargo test --workspace`; rejects persisted over-capacity ice, bounds freeze-path ice formation, uses active `ul` capacity, and routes overflow to `watbtm` while closing the shadow identity |
| C1b focused WB13/WB17/WB18/trace unit vectors | Pass in `cargo test --workspace`; covers WB13 `Dp` overflow publication, WB18 dust canonicalization, scalar/layer roundoff rebalance, positive deep loss, zero root uptake, no WB14 replay, and preferred WB19 geometry in WB18 guard diagnostics |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 30 tests |
| `cargo test --test hphys0283_snowmelt_infiltration_partition_contract -- --nocapture` | Pass, 1 test |
| `cargo test --test hphys0285_spring_soil_storage_retention_contract -- --nocapture` | Pass, 3 tests |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v61 |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v61 |
| p1/p43 starter capacity trace | Pass; zero `frzw > ul` rows across `1700` trace rows and `15192` layer checks per prefix |
| 43-prefix `algebraic-radium` frost-on cohort | Pass, `43/43` clean exits; run root `/tmp/fdhp01_increment_c1b_cohort_final14_20260612T035618Z` |
| Annual `Total-Soil + frozwt` closure, years 2-6 | Pass, max abs residual `1.5347723092418164e-12 mm`, mean abs residual `1.0758525853139703e-13 mm` |
| Year-7 boundary watch item | Pass at noise for C1b, max abs residual `6.963318810448982e-13 mm` |
| Year-1 initialization residual | Recorded outside the staged gate, max abs residual `1.0505061950725292 mm` |
| Valid-input capacity guard | Pass, no valid cohort capacity guard trips |
| Profile-bound pinning directional gate | Pass, `0/43` prefixes pinned; minimum margin `5.824859208653152 mm` |
| `frozwt/frdp` scalar-signature gate | Pass, max per-prefix correlation `0.9860178382757524`, below Increment B max `0.9861968090242198`; median correlation `0.943746050619941` |
| D3 depth/duration acceptance | Still open; mean max depth `1791.9747961835646 mm` is worse than Increment B mean `1782.265765656973 mm`, with `45782` frozen days and `42556` days above `200 mm` |

Increment C1b disposition: landed at `executed-hold`. The water-side
capacity/overflow gate passes and D2 conservation remains at noise, but D3
depth/duration acceptance remains open for C2 thaw-arm and state-machine work.

## D3 Increment C2 Thaw-Arm State-Machine Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local DuckDB/CLI cohort comparisons. |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass, `advisories ok, bans ok, licenses ok, sources ok` |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `0b3fed8561232d0de371a195e3f5a5609121ddd2253713b3fb0139add9ec8a4f`; sidecar SHA `5bfe72bec363900838255730a8cd5ee60144294c1a6640282e774f9e3c2f94a7` |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_c2 -- --nocapture` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_d2_contract_frwatc_freeze_exchange_diagnostics_reconcile_liquid_and_frozen_storage -- --nocapture` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_c1b_overflow_routes_to_watbtm_and_closes_shadow_identity -- --nocapture` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 33 tests |
| `git diff --check` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |
| Debug-marker search | Pass for temporary C2/debug-guard marker names; no C2 debug marker remains |
| 43-prefix `algebraic-radium` hourly frost-on cohort | Pass, `43/43` clean exits; run root `/tmp/fdhp01_increment_c2_cohort_hourly_fix_20260612T035740Z` |
| Cohort lane check | Pass; retained run used `/wc1/runs/al/algebraic-radium/wepp/runs`, `selected_lane=hourly`, `mode_divergence=false` |
| Annual `Total-Soil + frozwt` closure, years 2-6 | Pass at the package C1b additive-storage ledger surface, max abs residual `0.0 mm` |
| Year-7 boundary watch item | Pass at the retained ledger surface, max abs residual `0.0 mm` |
| Profile-bound pinning directional gate | Pass, `0/43` prefixes pinned; minimum margin `5.54557792097421 mm` |
| `frozwt/frdp` scalar-signature gate | Pass, max per-prefix correlation `0.9441102161636825`, median `0.8831449770567324` |
| D3 depth envelope acceptance | Fail; mean max depth `1793.52198510966 mm`, median `1793.649969637327 mm`, max `1794.4544220790258 mm` |
| D3 depth-correlation acceptance | Fail; median correlation `-0.16722397856345997`, range `-0.30843972585040713..0.19389645706206324` |
| D3 frozen-duration acceptance | Fail; open-minus-legacy median `111` days, mean `74.48837209302326` days, range `-293..171` days |
| Days above `200 mm` watch | Fail/watch; median `815` days |

Increment C2 disposition: landed at `executed-hold`. Thaw-arm, sandwich
geometry, thaw-through, and overflow routing now run cleanly without reopening
D2/p2 conservation, but D3 acceptance remains open. The C2 discriminating
experiment establishes the remaining defect as freeze-side
energy/resistance/front-advance behavior rather than missing thaw-storage
plumbing.

## D3 Increment Da Energy Characterization Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/DuckDB comparisons. |
| Temporary p1 hourly trace | Pass; trace captured at `/tmp/fdhp01_increment_da_trace_20260612T043800Z`, then instrumentation was removed before production rebuild. |
| Trace-marker source search | Pass; no Da trace marker remains under `crates/`. |
| `cargo fmt --check` | Pass |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` after trace removal | Pass, release binary SHA `0b3fed8561232d0de371a195e3f5a5609121ddd2253713b3fb0139add9ec8a4f`. |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 33 tests |
| 43-prefix `algebraic-radium` hourly frost-on cohort | Pass, `43/43` clean exits; run root `/tmp/fdhp01_increment_da_cohort_20260612T044217Z`. |
| WAT outputs | Pass, `43/43`. |
| WAT row equality versus C2 | Pass, `43/43`; Da lands no production physics. |
| Independent WAT closure ledger repair | Pass; Da table recomputes `RM + Irr - Interception - Q - Ep - Es - Er - Dp - latqcc - Tile` against annual `delta(Total-Soil + frozwt)` instead of reusing a tautological identity. |
| Independent annual `Total-Soil + frozwt` closure, years 2-6 | Pass at repaired WAT-surface numerical floor, max abs residual `1.3813070645629644e-07 mm` (`p11`, year 5). |
| p43 year-2 watch item | Cleared as WAT-surface numerical texture, residual `-1.912025027195341e-08 mm`. |
| p1/p20 closure spot checks, years 2-6 | p1 max abs `1.1812772982011666e-13 mm`; p20 max abs `6.650235917504688e-14 mm`. |
| Profile-bound pinning | Still unpinned, `0/43` prefixes; minimum margin `5.54557792097421 mm`. |
| `frozwt/frdp` scalar-signature gate | Still pass, max per-prefix correlation `0.9441102161636825`, median `0.8831449770567324`. |
| D3 depth envelope acceptance | Still fail; mean max depth `1793.52198510966 mm`, median `1793.649969637327 mm`, max `1794.4544220790258 mm`. |
| D3 depth-correlation acceptance | Still fail; median correlation `-0.16722397856345997`. |
| D3 frozen-duration acceptance | Still fail; open-minus-legacy median `111` days. |
| Days above `200 mm` watch | Still fail/watch; median `815` days. |
| `git diff --check` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |

Increment Da disposition: landed as diagnostic evidence at `executed-hold`. Da
localizes the remaining D3 implementation target to the legacy `frzng` in-hour
front-advance/resistance feedback: resistance must grow and `qhtout` must be
recomputed after fine-layer front advance inside the hour. It does not support
a W/m2-to-hour unit conversion defect, missing thaw storage, or low-latent
energy as the primary root cause.

## D3 Increment Db Freeze-Resistance Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/DuckDB comparisons. |
| Pre-fix within-hour red test | Failed as intended: one cold hour advanced `0.1996 m` on a thin-front profile by spending stale start-hour resistance. |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_db_freeze_front_recomputes_resistance_within_hour -- --nocapture` | Pass after Db implementation |
| `cargo fmt && cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 34 tests |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |
| Release CLI build/help | Pass, produced `target/release/openwepp-cli-hill`; release binary SHA `b97d1d1112c0d65470fd5f349d5cf96090c708fa34b243f64fdfb18b77530c37`. |
| 43-prefix `algebraic-radium` hourly frost-on cohort | Pass, `43/43` clean exits; run root `/tmp/fdhp01_increment_db_cohort_20260612T051524Z`. |
| WAT outputs | Pass, `43/43`. |
| Independent annual `Total-Soil + frozwt` closure, years 2-6 | Pass at WAT-publication numerical texture, max abs residual `1.9976620946327017e-07 mm` (`p11`, year 6). |
| p1/p20/p43 closure spot checks | p1 max abs `1.7719159473017498e-13 mm`; p20 max abs `1.3145040611561853e-13 mm`; p43 year 2 `-5.3290705182007514e-14 mm`. |
| Profile-bound pinning | Pass, `0/43` prefixes pinned; minimum margin `1356.009607259958 mm`. |
| D3 depth envelope acceptance | Pass, `43/43` maximum depths inside the legacy `240..503.2 mm` envelope; mean max `409.16220799389805 mm`, median max `407.3294069097544 mm`. |
| `frozwt/frdp` scalar-signature gate | Pass, max per-prefix correlation `0.8677993973935473`, median `0.7716116121810137`. |
| D3 depth-correlation acceptance | Fail; median correlation `-0.05296014769462692`, range `-0.07638011292463864..0.18461818718568115`. |
| D3 frozen-duration acceptance | Fail; open-minus-legacy median `-452` days, mean `-455.3953488372093` days, range `-505..-408` days. |
| Days above `200 mm` watch | Improved but not closing evidence; median `92` days. |
| `git diff --check` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |

Increment Db disposition: landed at `executed-hold`. Db closes the stale
within-hour freeze-resistance defect and fixes the maximum-depth envelope, but
D3 remains open because correlation and frozen-duration acceptance still fail.

## D3 Increment Dc Seasonal Heat/Thaw Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/DuckDB/PyArrow comparisons. |
| Pre-fix Dc red tests | Failed as intended: synthetic lower-front heat produced `14.7 W/m2` where seasonal `tmpbl` should zero-gate; one-hour thaw advanced too far. |
| `cargo fmt --check` after Dc implementation | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_dc_ -- --nocapture` after Dc implementation | Pass, 3 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` after Dc implementation | Pass, 37 tests |
| Release CLI build for failed attempt | Pass, failed-attempt binary SHA `772b5778f710619c7e1a99da82c69417d67c8ee15a3ff0a9bb44afe2886e3e21`. |
| p35 focused guard rerun | Pass after bounded fine-theta boundary canonicalization in the failed attempt. |
| 43-prefix `algebraic-radium` hourly frost-on cohort | Pass execution, `43/43` clean exits; run root `/tmp/fdhp01_increment_dc_cohort_20260612T062840Z`. |
| WAT outputs | Pass execution, `43/43`. |
| Independent annual `Total-Soil + frozwt` closure, years 2-6 | **Fail**, max abs residual `0.2706094484356498 mm` (`p34`, year 2), above the accepted Db `~2e-7 mm` WAT-publication texture. |
| p1/p20/p43 closure spot checks | Fail/regressed: p1 max abs `0.022261272887243777 mm`; p20 max abs `0.023365382872134077 mm`; p43 year 2 `-0.24479853886504088 mm`. |
| Profile-bound pinning | Fail/regressed, `1/43` prefixes pinned; minimum margin `2.2737367544323206e-13 mm`. |
| D3 depth envelope acceptance | Fail/regressed, `0/43` maximum depths inside the legacy `240..503.2 mm` envelope; mean max `1062.5086535449198 mm`, median max `1044.4140627263175 mm`, range `763.4002205550781..1799.9999999999998 mm`. |
| `frozwt/frdp` scalar-signature gate | Pass, max per-prefix correlation `0.7044204014819017`, median `0.6225055509763039`. |
| D3 depth-correlation acceptance | Directionally improved, median `0.6595441080376979`, but not accepted because D2 and depth-envelope gates fail. |
| D3 frozen-duration acceptance | Fail/regressed, open-minus-legacy median `+751` days, mean `+749.3720930232558` days, range `+724..+794` days. |
| Days above `200 mm` watch | Fail/regressed, full-WAT median `1306` days. |

Increment Dc disposition: failed and backed out. Dc proves the F1/F2 one-pass
change can improve timing correlation, but it reopens additive storage and
depth/duration defects. At the post-Dc backout boundary, production returned
to Db and the next increment had to split seasonal lower-front heat from
thaw-timing dynamics while preserving the Db independent WAT ledger floor
before any D3 acceptance claim.

## D3 Increment Dc1 Accounting Repair Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/PyArrow comparisons. |
| Pre-fix Dc1 red tests | Failed as intended: seasonal lower-front heat still used the `14.7 W/m2` surrogate, one-hour thaw spent stale start-hour resistance, and p35 lower-bound theta roundoff tripped a material guard. |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_dc1_ -- --nocapture` | Pass, 4 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 38 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `95491b24f36065c28f90ca7e55bfceb39cf14ac2c270ddfd207eb750a2e4a536` |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass, `advisories ok, bans ok, licenses ok, sources ok` |
| `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract` | Pass after `SC-SNOWFREEZE-001` v64 |
| `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract` | Pass after `SC-SNOWFREEZE-001` v64 |
| 43-prefix `algebraic-radium` hourly frost-on cohort | Pass, `43/43` clean exits; run root `/tmp/fdhp01_increment_dc1_cohort_20260612T101238Z`. |
| WAT outputs | Pass, `43/43`. |
| Independent annual `Total-Soil + frozwt` closure, years 2-6 | Pass at WAT-publication texture, max abs residual `6.471338602487275e-07 mm` (`p11`, year 4). |
| p1/p20/p43 closure spot checks | Pass: p1 max abs `9.41296818268711e-10 mm`; p20 max abs `1.0458300891968975e-13 mm`; p43 year 2 `-1.1013412404281553e-13 mm`. |
| Profile-bound pinning | Recorded only for Dc1; still red for D3, `1/43` prefixes pinned and minimum margin `2.2737367544323206e-13 mm`. |
| D3 depth envelope acceptance | Recorded only for Dc1; fail, `0/43` maximum depths inside the legacy `240..503.2 mm` envelope; mean max `1146.5109665924424 mm`, median max `1110.3558249519133 mm`. |
| `frozwt/frdp` scalar-signature gate | Pass, max per-prefix correlation `0.7889919205846698`, median `0.7316780606012344`. |
| D3 depth-correlation acceptance | Recorded only for Dc1; directionally improved to median `0.6415921721982907` but not accepted while depth/duration fail. |
| D3 frozen-duration acceptance | Recorded only for Dc1; fail/regressed, open-minus-legacy median `+567` days. |
| Days above `200 mm` watch | Fail/watch, full-WAT median `1126` days. |

Increment Dc1 disposition: landed at `executed-hold`. Dc1 repairs the Dc
additive-storage leak and preserves D2/p2 closure at WAT-publication numerical
texture, but D3 remains open. Depth/duration evidence is now held for the F4
snow-insulation/depth-duration discriminator before MOFE closure.

## D3 Increment Dd Legacy-Snow-Forced Diagnostic Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/PyArrow/Pandas comparisons. |
| Legacy `H*.winter.dat` generation | Pass, `43/43` pinned-baseline runs exited clean with winter outputs under `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/output`. |
| Legacy snow forcing extraction | Pass, generated `109951` daily rows keyed by prefix, simulation year, and Julian day from hour-24 `snodpt`/`densgt`; maximum legacy snow depth `0.7258 m`, maximum density `350 kg/m3`. |
| Temporary forced-snow hook | Pass for diagnostic use; env-gated hook replaced only the snow depth/density consumed by frost heat-flow resistance, then was removed before production rebuild. |
| Diagnostic release build | Pass, diagnostic binary SHA `8cab0ac419cdfbb90bd3506913de166d7d85d1f4697a2023e2d24d29a0113181`. |
| 43-prefix `algebraic-radium` forced-snow frost-on cohort | Pass execution, `43/43` clean exits; run root `/tmp/fdhp01_increment_dd_forced_snow_cohort_20260612T121500Z`. |
| WAT outputs | Pass, `43/43`. |
| Independent annual `Total-Soil + frozwt` closure, years 2-6 | Pass at WAT-publication texture, max abs residual `6.726058817130287e-07 mm` (`p11`, year 4). |
| p1/p20/p43 closure spot checks | Pass: p1 max abs `2.5357493882438575e-13 mm`; p20 max abs `1.3500311979441904e-13 mm`; p43 year 2 `-1.2079226507921703e-13 mm`. |
| Profile-bound pinning | Pass under forced snow, `0/43` prefixes pinned; minimum margin `372.60609939367146 mm`. |
| D3 depth envelope certification | Fail, `0/43` maximum depths inside the legacy `240..503.2 mm` envelope; mean max `856.817674502367 mm`, median max `844.2352603016866 mm`, range `654.0796339074789..1427.3939006063285 mm`. |
| D3 depth-correlation certification | Directionally improved but not sufficient alone; median correlation `0.7118806632341061`, range `0.4357068574368132..0.8709499689951673`. |
| D3 frozen-duration certification | Fail, open-minus-legacy median `+502` days, mean `+499.74418604651163` days, range `+462..+558` days. |
| Days above `200 mm` watch | Fail/watch, full-WAT median `937` days. |
| Clean-source release rebuild after hook removal | Pass, production binary SHA `95491b24f36065c28f90ca7e55bfceb39cf14ac2c270ddfd207eb750a2e4a536`. |
| Production source diff after hook removal | Pass; no source diff remains from the temporary forced-snow hook. |
| `git diff --check` | Pass |
| `cargo fmt --check` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |

Increment Dd disposition: diagnostic executed at `executed-hold`. Legacy snow
depth/density is a material contributor, but forced legacy snow does not close
D3. The remaining depth/duration residual is frost-side under controlled snow
forcing and needs the next scoped hourly flux/front localization increment.

## D3 Increment De Content-Dependent `qdry` Conductivity Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/DuckDB/Pandas comparisons. |
| Contract amendment | Pass, `SC-SNOWFREEZE-001` v65 adds legacy `frostn.for:430-458` `Qdry` conductivity authority. |
| `cargo fmt --check` | Pass |
| `git diff --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, `40/40` |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass, `advisories ok, bans ok, licenses ok, sources ok` |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | Pass |
| Native production cohort | Pass execution, `43/43` clean exits and `43/43` WAT outputs; root `/tmp/fdhp01_increment_de_native_cohort_final_20260612T171358Z`. |
| Native years 2-6 independent `Total-Soil + frozwt` closure | Pass at WAT-publication texture, max abs residual `5.474257917248426e-07 mm` (`p11`, year 6). |
| Native D3 depth/duration | Fail, mean max depth `705.505148615878 mm`, `0/43` prefixes inside the `240..503.2 mm` envelope, median frozen-duration residual `+288` days. |
| Forced-snow diagnostic hook | Pass for diagnostic use; env-gated hook replaced only snow depth/density consumed by frost heat-flow resistance, then was removed before production rebuild. |
| Forced-snow diagnostic cohort | Pass execution, `43/43` clean exits and `43/43` WAT outputs; root `/tmp/fdhp01_increment_de_forced_snow_cohort_20260612T171017Z_proper`. |
| Forced-snow years 2-6 independent `Total-Soil + frozwt` closure | Pass at WAT-publication texture, max abs residual `4.355148297552347e-07 mm` (`p11`, year 4). |
| Forced-snow D3 depth envelope certification | Fail, `0/43` maximum depths inside the legacy `240..503.2 mm` envelope; mean max `655.9890274782282 mm`, median max `652.3375464029963 mm`, range `558.1869128158116..741.4969698215496 mm`. |
| Forced-snow D3 frozen-duration certification | Fail, open-minus-legacy median `+186` days, mean `+183.5581395348837` days, range `+116..+300` days. |
| Forced-snow depth-correlation certification | Directionally improved but not sufficient alone; median correlation `0.770042438411068`. |
| Clean-source release rebuild after hook removal | Pass, production binary SHA `981da203d9ced9b1d73f049fa3a4b227710862a3dbecaad9d4619f03ae7dd2d5`. |

Increment De disposition: landed at `executed-hold`. The F5 conductivity
correction is real and improves the controlled-snow residual, but it does not
certify D3. The next increment remains frost-side under De forced-snow forcing
and must localize the first remaining hourly front/flux divergence.

## D3 Increment Df Paired Hourly Localization Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/Pandas/PyArrow comparisons. |
| Legacy winter column source check | Pass; `winter.dat` `ground` is `gdrft` ground-drift snow from `winter.for`, not a surface-temperature column. |
| Temporary p1/p2 paired hourly trace | Pass; trace root `/tmp/fdhp01_increment_df_trace2_20260612T175406Z`, p1/p2 clean exits, WAT outputs present. |
| Paired hourly join | Pass; p1 joined `30240` legacy hourly rows, p2 joined `29928` legacy hourly rows. |
| First material divergence localization | Pass; both p1 and p2 diverge at year 1 day 1 hour 2 while snow-free on both sides. Legacy frost depth `5.0 mm`; openWEPP `42.057866709 mm` on p1 and `41.417581693 mm` on p2. |
| Term attribution | Pass; dominant seam is surface resistance: openWEPP `residue_depth_m = 0.0` while legacy residue is `23.0 mm`, and openWEPP omits the legacy `dpfsfl` shallow-front minimum conduction distance. |
| Trace-marker source search after removal | Pass; no `OPENWEPP_FDHP01_DF` or `fdhp01_df` markers remain under `crates/`. |
| Clean-source release rebuild after hook removal | Pass, production binary SHA `981da203d9ced9b1d73f049fa3a4b227710862a3dbecaad9d4619f03ae7dd2d5`. |

Increment Df disposition: diagnostic executed at `executed-hold`. Df leaves no
production edits and identifies Dg as the next bounded implementation:
restore the legacy residue-depth frost resistance path and the shallow-front
minimum conduction distance, then re-run the De forced-snow certification and
native cohort.

## D3 Increment Dg Residue/Shallow-Front Resistance Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/Pandas/PyArrow comparisons. |
| Contract amendment | Pass, `SC-SNOWFREEZE-001` v66 binds the legacy `resdep/kres` frost surface-residue path and below-freezing shallow-front `dpfsfl` minimum conduction distance. |
| Production implementation | Pass, openWEPP consumes legacy residue depth in frost surface resistance and floors below-freezing shallow-front conduction distance to `dg(1)/nfine(1)/2`. |
| Native production cohort | Pass execution, `43/43` clean exits and `43/43` WAT outputs; root `/tmp/fdhp01_increment_dg_native_cohort_20260612T184601Z`. |
| Native years 2-6 independent `Total-Soil + frozwt` closure | Pass at WAT-publication texture, max abs residual `6.261351281899863e-07 mm`. |
| Native D3 depth/duration | Directionally improved but not accepted: mean max `498.08123930883653 mm`, `30/43` prefixes inside the legacy `240..503.2 mm` envelope, median duration residual `+84` days. |
| Forced-snow diagnostic cohort | Pass execution, `43/43` clean exits and `43/43` WAT outputs; root `/tmp/fdhp01_increment_dg_forced_snow_cohort_20260612T185203Z`. |
| Forced-snow years 2-6 independent `Total-Soil + frozwt` closure | Pass at WAT-publication texture, max abs residual `5.835723933533821e-07 mm`. |
| Forced-snow D3 directional gate | Pass directionally: mean max depth `655.9890274782282 -> 490.0923199552928 mm`, envelope membership `0/43 -> 30/43`, median duration residual `+186 -> +73` days. |
| D3 acceptance | Fail/hold: `13/43` forced-snow prefixes still exceed the `503.2 mm` upper envelope bound. |
| Clean-source release rebuild after diagnostic hook removal | Pass, production binary SHA `3275db431339402596a27a28d7976062eb4655771e9e159fdf929fa1410883ad`. |

Increment Dg disposition: landed at `executed-hold`. Dg closed the Df
surface-resistance cut point and improved both native and forced-snow depth,
but the remaining forced-snow plateau outliers required a new localization.

## D3 Increment Dh Frozen-Path Conductivity Refutation Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local source inspection and artifact comparisons. |
| Pinned-source inspection | Pass/refutation: `frostn.for:188-193` assigns fixed `kftill = 1.75` and `kfutil = 2.1`; `frostn`/`frzng`/`frznw` consume those fixed constants in the frozen surface path. |
| Lower-front conductivity attribution | Pass/refutation: the soil-property-dependent `bdcons`/`slsw`/`ksoilf` expression belongs to the lower-front unfrozen `kufzfl` path already handled by De. |
| Contract amendment | Pass, `SC-SNOWFREEZE-001` v67 binds fixed legacy `kftill`/`kfutil` constants and rejects per-soil frozen-path replacement absent superseding authority. |
| Production physics edit | None; Dh stopped before production edits because the premise was false. |
| Focused and full gates | Pass per Dh artifact: `cargo fmt --check`, focused contract tests, `git diff --check`, `wctl doc-lint --path docs`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check`, anti-evasion guards, and `cargo test --workspace`. |

Increment Dh disposition: refutation executed at `executed-hold`. The
per-soil frozen-path conductivity mechanism is eliminated as source-line-owned
scope; Dg outliers remain open.

## D3 Increment Di Post-Dg Paired Re-localization Gates

| Command / Gate | Result |
|---|---|
| Comparator-suite runner | Not used; user explicitly requested no comparator subagent because GPT-5.3-Codex-Spark weekly quota was exhausted. Parent ran local CLI/Pandas/PyArrow comparisons. |
| Temporary p8/p20/p2 paired hourly trace | Pass; trace root `/tmp/fdhp01_increment_di_trace_20260612T1302Z`, all three prefixes exited clean, WAT outputs present. |
| Diagnostic release build with temporary hook | Pass, `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`. |
| Paired hourly join | Pass; p8 joined `29472` legacy hourly rows, p20 joined `30336`, p2 joined `29928`. |
| Static legacy source check | Pass; `hr_tmp.for:38-48` calls `tmpadj`, `tmpadj.for:349-364` computes `surtmp(hour)` and caps positive snow-active surface temperature, and `frostn.for:467-480` consumes `surtmp(hour)` for top heat flux. |
| Term attribution | Pass; dominant residual is missing legacy `hr_tmp`/`tmpadj` surface-temperature synthesis. Deep divergent advance with forced snow present and negative open surface temperature is `0.997852`, `0.999063`, and `1.000000` on p8/p20/p2; median surface-flux share is `1.000000`, `1.000000`, and `0.994355`. |
| Secondary discriminators | Pass; topology, hourly snow-depth mismatch, lower-front heat, and deep-layer latent cost are not the cut point. |
| Trace-marker source search after removal | Pass; no `OPENWEPP_FDHP01_DI`, `fdhp01_di`, or `FDHP01_DI` markers remain under `crates/`. |
| Clean-source release rebuild after hook removal | Pass, production binary SHA `3275db431339402596a27a28d7976062eb4655771e9e159fdf929fa1410883ad`. |
| `cargo fmt --check` | Pass |
| `git diff --check` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings`. |

Increment Di disposition: diagnostic executed at `executed-hold`. Di leaves no
production edits and scopes Dj to port/expose legacy `hr_tmp`/`tmpadj`
surface-temperature synthesis into the frost surface heat path without
retuning snow, `kfactor`, latent heat, WAT/D2, residue, `dpfsfl`,
`kftill`/`kfutil`, or `Qdry`.
