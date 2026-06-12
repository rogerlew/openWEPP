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
