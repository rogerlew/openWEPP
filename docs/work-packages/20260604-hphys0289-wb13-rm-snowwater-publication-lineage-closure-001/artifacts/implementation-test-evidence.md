# Implementation/Test Evidence

Status: complete
Evidence mode: Static/Ran

## Implementation

Static:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` now publishes `snow.routed_melt_m` from the same `runoff_snow_term` used by WB12/WB14 runoff/infiltration reconciliation.
- `crates/openwepp-runner/src/hillslope/mod.rs` now requires `snow.routed_melt_m`, guards it non-negative, and computes WB13 `RM` from post-winter rain branch + routed melt + irrigation instead of raw precipitation plus SWE-delta proxy.
- WB13 `Snow-Water` remains runtime-SWE based and fail-closed for negative SWE.

## Focused Tests

Ran:

- `cargo fmt --check` after `cargo fmt`: pass.
- `cargo test -p openwepp-runner hphys0289_wb13_rm_publication -- --nocapture`: pass, `5 passed; 0 failed`.
- `cargo test --test hphys0289_wb13_rm_snowwater_publication_contract -- --nocapture`: pass, `2 passed; 0 failed`.

## Adjacent Tests

Ran:

- `cargo test --test hphys0288_winter_rain_snowmelt_partition_contract -- --nocapture`: pass.
- `cargo test --test wb13_daily_water_balance_output_surface_contract -- --nocapture`: pass.
- `cargo test -p openwepp-runner hphys0239_wb13_hydrology_publication_prefers_flux_surface_over_stale_state_surface -- --nocapture`: pass.

## Full H1..H39 Runtime/Semantic Suite

Ran:

- `python docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/hphys0289_diagnostics.py --run-root /tmp/hphys0289_full_release_current_20260605T000159Z`
  - Runtime status: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/hillslope_batch_status.tsv`
  - Semantic status: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/semantic_status.tsv`
  - Summary: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/hillslope_semantic_summary.md`
  - Semantic pass: `0/39`

| Symbol | Fail Count | Mean Abs Diff | Max Abs Diff |
| --- | ---: | ---: | ---: |
| Ep | 45401 | 0.727061 | 7.242659 |
| Es | 500 | 0.010422 | 1.825681 |
| Er | 0 | 0.000000 | 0.000000 |
| Total-Soil | 52521 | 57.069194 | 348.886998 |
| SoilWaterTotal | 52521 | 57.069194 | 348.886998 |
| Dp | 9220 | 0.042845 | 0.244800 |
| latqcc | 36003 | 0.373461 | 11.865076 |
| Q | 2108 | 0.552220 | 38.472185 |
| RM | 5868 | 0.258409 | 27.960000 |
| Snow-Water | 10391 | 2.899431 | 65.506840 |
| P | 0 | 0.000000 | 0.000000 |

## Delta vs HPHYS0288

Ran:

| Symbol | HPHYS0288 Fail | HPHYS0289 Fail | Δ Fail | HPHYS0288 Mean Abs | HPHYS0289 Mean Abs | Δ Mean Abs |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Ep | 45401 | 45401 | 0 | 0.727061 | 0.727061 | +0.000000 |
| Es | 500 | 500 | 0 | 0.010422 | 0.010422 | +0.000000 |
| Er | 0 | 0 | 0 | 0.000000 | 0.000000 | +0.000000 |
| Total-Soil | 52521 | 52521 | 0 | 57.069194 | 57.069194 | +0.000000 |
| SoilWaterTotal | 52521 | 52521 | 0 | 57.069194 | 57.069194 | +0.000000 |
| Dp | 9220 | 9220 | 0 | 0.042845 | 0.042845 | +0.000000 |
| latqcc | 36003 | 36003 | 0 | 0.373461 | 0.373461 | +0.000000 |
| Q | 2108 | 2108 | 0 | 0.552220 | 0.552220 | +0.000000 |
| RM | 6633 | 5868 | -765 | 0.248018 | 0.258409 | +0.010391 |
| Snow-Water | 10391 | 10391 | 0 | 2.899431 | 2.899431 | +0.000000 |
| P | 0 | 0 | 0 | 0.000000 | 0.000000 | +0.000000 |

Interpretation: HPHYS0289 moved only WB13 `RM`. The fail-count improvement confirms publication lineage changed; unchanged `Q`/`Snow-Water` and remaining `RM` residuals keep the package in HOLD.

## Surprise During First Full Run

Ran:

- First full suite root `/tmp/hphys0289_full_release_20260604T235528Z` failed all hillslopes with `flux:snow.routed_melt_m` lower-bound violation at `-1.6263032587282567e-19`.
- Fix: normalize only after explicit non-negative range check using the existing within-tolerance nonnegative normalizer. Material negatives still fail closed.
